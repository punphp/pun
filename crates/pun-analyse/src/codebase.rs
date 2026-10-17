//! The symbols every rule queries: Mago's prelude (built-ins) extended with every analysed and scanned file.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::populator::populate_codebase_parallel;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_prelude::Prelude;
use mago_syntax::parser::parse_file;
use rayon::prelude::*;

use crate::{Project, autoload};

/// PHPStan's unprefixed classes and functions (scripts/export-phpstan-api-stub.php): PHPStan always has them loaded.
const PHPSTAN_API: &[u8] = include_bytes!("../data/phpstan-api.php");

/// Whether `code` names PHPStan's own namespaces, so the (large) API stub is worth scanning.
fn names_phpstan(code: &[u8]) -> bool {
    code.windows(8)
        .any(|w| w.eq_ignore_ascii_case(b"phpstan\\") || w.eq_ignore_ascii_case(b"phpparse"))
}

pub fn php_version(id: u32) -> PHPVersion {
    PHPVersion::new(id / 10000, id / 100 % 100, id % 100)
}

pub fn read(path: &Path, file_type: FileType) -> Option<File> {
    let contents = std::fs::read(path).ok()?;
    let name = path.to_string_lossy().into_owned().into_bytes();
    Some(File::new(
        Cow::Owned(name),
        file_type,
        Some(path.to_path_buf()),
        Cow::Owned(contents),
    ))
}

/// Scans `file`, then resets `arena`, so one arena serves many files.
fn scan(arena: &mut LocalArena, file: &File, version: PHPVersion) -> CodebaseMetadata {
    let scanned = {
        let arena = &*arena;
        let program = parse_file(arena, file);
        let names = NameResolver::new(arena).resolve(program);
        // ponytail: Mago's scanner trips a debug assertion on some hints (an empty TUnion::from_vec, debug builds
        // only); the file's symbols are then unknown
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            scan_program(arena, file, program, &names, version)
        }))
        .unwrap_or_default()
    };
    arena.reset();
    scanned
}

/// The prelude for one build: the first build of the process owns a fresh one (a single run keeps one copy, not a
/// cached one and its clone); later builds clone one built once.
fn prelude() -> (CodebaseMetadata, SymbolReferences) {
    static FIRST: AtomicBool = AtomicBool::new(true);
    static CACHED: LazyLock<(CodebaseMetadata, SymbolReferences)> = LazyLock::new(build_prelude);
    if FIRST.swap(false, Ordering::Relaxed) {
        build_prelude()
    } else {
        CACHED.clone()
    }
}

/// Mago's prelude, without the built-ins PHPStan's reflection doesn't know (symbols.json: newer PHP, other
/// extensions), so a polyfill in vendor declares them as user code.
fn build_prelude() -> (CodebaseMetadata, SymbolReferences) {
    let Prelude {
        mut metadata,
        symbol_references,
        ..
    } = Prelude::build();
    let builtins = pun_phpstan::symbols::symbols(pun_phpstan::symbols::PhpVersion::Php85);
    metadata.class_likes.retain(|name, k| {
        !k.flags.is_built_in()
            || builtins
                .classes
                .contains_key(&String::from_utf8_lossy(name.as_bytes()).to_ascii_lowercase())
    });
    metadata.function_likes.retain(|(scope, name), f| {
        !scope.is_empty()
            || !f.flags.is_built_in()
            || builtins
                .functions
                .contains_key(&String::from_utf8_lossy(name.as_bytes()).to_ascii_lowercase())
    });
    (metadata, symbol_references)
}

/// Files scanned by one task: small enough that the threads finish together.
const CHUNK: usize = 32;

/// The analysed files as the scan read them, by path, for the rules to walk without reading them again.
pub type Sources = HashMap<PathBuf, Arc<File>>;

pub fn build(project: &Project) -> CodebaseMetadata {
    build_with_sources(project).0
}

/// The codebase, and the analysed files' [`Sources`].
pub fn build_with_sources(project: &Project) -> (CodebaseMetadata, Sources) {
    let version = php_version(project.php_version);
    let mut seen = HashSet::new();
    let mut files: Vec<_> = project
        .analysed
        .iter()
        .map(|p| (p, FileType::Host))
        .chain(project.scanned.iter().map(|p| (p, FileType::Vendored)))
        .filter(|(p, _)| seen.insert(*p))
        .collect();
    // psr-4 and psr-0 files after them, scanned once a scanned file names their class (a file below two directories
    // is one file with two classes)
    let eager = files.len();
    let mut at: HashMap<&PathBuf, usize> = HashMap::new();
    let mut classes = Vec::new();
    for a in &project.autoloaded {
        if seen.contains(&a.path) && !at.contains_key(&a.path) {
            continue;
        }
        let i = *at.entry(&a.path).or_insert_with(|| {
            files.push((&a.path, FileType::Vendored));
            files.len() - 1
        });
        classes.push((i, a));
    }
    let index = autoload::Index::new(classes.into_iter());
    // Scanning is most of the time (the vendored files): in rounds, each the psr-4 and psr-0 files the previous one
    // named, in chunks across the threads, merged in file order, while the prelude is built and PHPStan's 8.4
    // reflection of built-in classes (symbols::class(), which the rules read) is loaded, rather than by the first file
    // that asks while the others wait.
    let php84 = || pun_phpstan::symbols::symbols(pun_phpstan::symbols::PhpVersion::Php84);
    let (((mut metadata, mut references), _), (scanned, sources, api)) = rayon::join(
        || rayon::join(prelude, php84),
        || {
            let mut loaded = vec![false; files.len()];
            loaded[..eager].fill(true);
            let mut round: Vec<usize> = (0..eager).collect();
            let mut scanned = Vec::new();
            let mut sources = Sources::new();
            let mut api = false;
            while !round.is_empty() {
                let size = (round.len() / (rayon::current_num_threads() * 4)).clamp(1, CHUNK);
                let chunks: Vec<_> = round
                    .par_chunks(size)
                    .map_init(LocalArena::new, |arena, chunk| {
                        let mut metadata = CodebaseMetadata::default();
                        let mut sources = Vec::new();
                        let mut wanted = Vec::new();
                        for &i in chunk {
                            let (path, file_type) = files[i];
                            let Some(file) = read(path, file_type) else {
                                continue;
                            };
                            metadata.extend(scan(arena, &file, version));
                            index.wanted(&file.contents, &mut wanted);
                            wanted.retain(|&w| !loaded[w]);
                            if file_type == FileType::Host {
                                sources.push((path.clone(), Arc::new(file)));
                            }
                        }
                        (metadata, sources, wanted)
                    })
                    .collect();
                round.clear();
                for (chunk, files, wanted) in chunks {
                    scanned.push(chunk);
                    for (path, file) in files {
                        api |= names_phpstan(&file.contents);
                        sources.insert(path, file);
                    }
                    round.extend(wanted);
                }
                round.sort_unstable();
                round.dedup();
                for &i in &round {
                    loaded[i] = true;
                }
            }
            (scanned, sources, api)
        },
    );
    for chunk in scanned {
        metadata.extend(chunk);
    }
    if api {
        let stub = File::new(
            Cow::Borrowed(b"phpstan-api.php"),
            FileType::Vendored,
            None,
            Cow::Borrowed(PHPSTAN_API),
        );
        metadata.extend(scan(&mut LocalArena::new(), &stub, version));
    }
    populate_codebase_parallel(
        &mut metadata,
        &mut references,
        Default::default(),
        Default::default(),
    );
    (metadata, sources)
}
