//! The symbols every rule queries: Mago's prelude (built-ins) extended with every analysed and scanned file.

use std::borrow::Cow;
use std::collections::HashSet;
use std::path::Path;
use std::sync::LazyLock;

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::populator::populate_codebase;
use mago_codex::scanner::scan_program;
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_prelude::Prelude;
use mago_syntax::parser::parse_file;

use crate::Project;

static PRELUDE: LazyLock<Prelude> = LazyLock::new(Prelude::build);

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

fn scan(metadata: &mut CodebaseMetadata, file: &File, version: PHPVersion) {
    let arena = LocalArena::new();
    let program = parse_file(&arena, file);
    let names = NameResolver::new(&arena).resolve(program);
    // ponytail: Mago's scanner trips a debug assertion on some hints (an empty TUnion::from_vec, debug builds
    // only); the file's symbols are then unknown
    let scanned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        scan_program(&arena, file, program, &names, version)
    }));
    if let Ok(scanned) = scanned {
        metadata.extend(scanned);
    }
}

pub fn build(project: &Project) -> CodebaseMetadata {
    let mut metadata = PRELUDE.metadata.clone();
    // PHPStan's reflection knows only its own built-ins (symbols.json): the prelude's others (newer PHP, other
    // extensions) go, so a polyfill in vendor declares them as user code.
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
    let mut references = PRELUDE.symbol_references.clone();
    let version = php_version(project.php_version);
    let mut seen = HashSet::new();
    let files: Vec<_> = project
        .analysed
        .iter()
        .map(|p| (p, FileType::Host))
        .chain(project.scanned.iter().map(|p| (p, FileType::Vendored)))
        .filter(|(p, _)| seen.insert(*p))
        .collect();
    // Scanning is most of the time (every vendored file); in contiguous chunks, merged in order.
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    let chunks: Vec<(CodebaseMetadata, bool)> = std::thread::scope(|s| {
        let handles: Vec<_> = files
            .chunks(files.len().div_ceil(threads).max(1))
            .map(|chunk| {
                s.spawn(move || {
                    let mut metadata = CodebaseMetadata::default();
                    let mut api = false;
                    for &(path, file_type) in chunk {
                        let Some(file) = read(path, file_type) else {
                            continue;
                        };
                        api |= file_type == FileType::Host && names_phpstan(&file.contents);
                        scan(&mut metadata, &file, version);
                    }
                    (metadata, api)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("scan thread"))
            .collect()
    });
    let mut api = false;
    for (chunk, uses_api) in chunks {
        metadata.extend(chunk);
        api |= uses_api;
    }
    if api {
        let stub = File::new(
            Cow::Borrowed(b"phpstan-api.php"),
            FileType::Vendored,
            None,
            Cow::Borrowed(PHPSTAN_API),
        );
        scan(&mut metadata, &stub, version);
    }
    populate_codebase(
        &mut metadata,
        &mut references,
        Default::default(),
        Default::default(),
    );
    metadata
}
