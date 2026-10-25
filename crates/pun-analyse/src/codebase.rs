//! The symbols every rule queries: Mago's prelude (built-ins) extended with every analysed and scanned file.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, OnceLock};

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::constant::ConstantMetadata;
use mago_codex::metadata::flags::MetadataFlags;
use mago_codex::metadata::ttype::TypeMetadata;
use mago_codex::populator::populate_codebase_parallel;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{File, FileType};
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_prelude::Prelude;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::{Hint, IntersectionHint, Program};
use mago_syntax::parser::parse_file;
use mago_syntax::walker::MutWalker;
use rayon::prelude::*;

use crate::{Project, autoload};

/// PHPStan's unprefixed classes and functions (scripts/export-phpstan-api-stub.php): PHPStan always has them loaded.
pub(crate) const PHPSTAN_API: &[u8] = include_bytes!("../data/phpstan-api.php");

/// Whether `code` names PHPStan's own namespaces, so the (large) API stub is worth scanning.
fn names_phpstan(code: &[u8]) -> bool {
    code.windows(8)
        .any(|w| w.eq_ignore_ascii_case(b"phpstan\\") || w.eq_ignore_ascii_case(b"phpparse"))
}

pub(crate) fn php_version(id: u32) -> PHPVersion {
    PHPVersion::new(id / 10000, id / 100 % 100, id % 100)
}

/// A file to analyse or scan that exists but can't be read. PHPStan stops the run on one (`FileReader`'s
/// CouldNotReadFileException); a file that is gone is skipped instead.
#[derive(Debug)]
pub struct CouldNotReadFile {
    pub path: PathBuf,
    pub source: std::io::Error,
}

impl std::fmt::Display for CouldNotReadFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not read file: {}", self.path.display())
    }
}

impl std::error::Error for CouldNotReadFile {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// `path` as a Mago file, `None` when it does not exist.
pub(crate) fn read(path: &Path, file_type: FileType) -> Result<Option<File>, CouldNotReadFile> {
    let contents = match std::fs::read(path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(CouldNotReadFile {
                path: path.to_path_buf(),
                source,
            });
        }
    };
    let name = path.to_string_lossy().into_owned().into_bytes();
    Ok(Some(File::new(
        Cow::Owned(name),
        file_type,
        Some(path.to_path_buf()),
        Cow::Owned(contents),
    )))
}

/// `file` parsed into `arena`, with its names resolved.
pub(crate) fn parse<'arena>(
    arena: &'arena LocalArena,
    file: &File,
) -> (&'arena Program<'arena>, ResolvedNames<'arena>) {
    let program = parse_file(arena, file);
    (program, NameResolver::new(arena).resolve(program))
}

/// Scans `file`, then resets `arena`, so one arena serves many files.
fn scan(
    arena: &mut LocalArena,
    file: &File,
    version: PHPVersion,
    reachable: Option<(&autoload::Index, &mut Vec<usize>, &mut Vec<usize>)>,
) -> CodebaseMetadata {
    let mut scanned = {
        let arena = &*arena;
        let (program, names) = parse(arena, file);
        if let Some((index, wanted, declarations)) = reachable {
            index.wanted(&file.contents, program, &names, wanted, declarations);
        }
        // Mago's scanner trips a debug assertion (an empty TUnion::from_vec, debug builds only) on an intersection
        // type naming a non-class type, which PHP rejects; scanning again with those types read as `mixed` keeps
        // the file's symbols
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            scan_program(arena, file, program, &names, version)
        }))
        .or_else(|_| {
            let masked = File::new(
                file.name.clone(),
                file.file_type,
                file.path.clone(),
                Cow::Owned(mask_intersections(program, &file.contents)),
            );
            let (program, names) = parse(arena, &masked);
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                scan_program(arena, &masked, program, &names, version)
            }))
        })
        .unwrap_or_default()
    };
    arena.reset();
    if file.file_type == FileType::Patch {
        scanned.convert_partial_to_patch();
    }
    scanned
}

/// `contents` with each intersection type that names a non-class type overwritten by `mixed` (padded to its length).
fn mask_intersections(program: &Program, contents: &[u8]) -> Vec<u8> {
    struct Masks(Vec<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Masks {
        fn walk_in_intersection_hint(&mut self, hint: &'ast IntersectionHint<'arena>, _: &mut ()) {
            fn classes(h: &Hint) -> bool {
                match h {
                    Hint::Intersection(i) => classes(i.left) && classes(i.right),
                    h => matches!(
                        h,
                        Hint::Identifier(_) | Hint::Self_(_) | Hint::Static(_) | Hint::Parent(_)
                    ),
                }
            }
            let span = hint.span();
            // an inner intersection of one already masked (the walk visits the outer one first)
            let inside = self.0.last().is_some_and(|outer: &Span| {
                outer.start.offset <= span.start.offset && span.end.offset <= outer.end.offset
            });
            if !inside && !(classes(hint.left) && classes(hint.right)) {
                self.0.push(span);
            }
        }
    }
    let mut masks = Masks(Vec::new());
    masks.walk_program(program, &mut ());
    let mut contents = contents.to_vec();
    for span in masks.0 {
        let range = span.to_range_usize();
        let mut replacement = b"mixed".to_vec();
        replacement.resize(range.len(), b' ');
        contents[range].copy_from_slice(&replacement);
    }
    contents
}

/// The prelude for one build at `php_version`: the first build of the process owns a fresh one (a single run keeps
/// one copy, not a cached one and its clone); later builds clone one built once per PHP version.
fn prelude(php_version: u32) -> (CodebaseMetadata, SymbolReferences) {
    static FIRST: AtomicBool = AtomicBool::new(true);
    static PHP84: LazyLock<(CodebaseMetadata, SymbolReferences)> =
        LazyLock::new(|| build_prelude(80400));
    static PHP85: LazyLock<(CodebaseMetadata, SymbolReferences)> =
        LazyLock::new(|| build_prelude(80500));
    if FIRST.swap(false, Ordering::Relaxed) {
        build_prelude(php_version)
    } else if php_version >= 80500 {
        PHP85.clone()
    } else {
        PHP84.clone()
    }
}

/// Mago's prelude, without the built-ins PHPStan's reflection at `php_version` doesn't know (symbols.json: newer
/// PHP, other extensions), so a polyfill in vendor declares them as user code.
fn build_prelude(php_version: u32) -> (CodebaseMetadata, SymbolReferences) {
    let Prelude {
        mut metadata,
        symbol_references,
        ..
    } = Prelude::build();
    let builtins = crate::rules::builtins(php_version);
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
    // ConstantResolver::resolvePredefinedConstant keeps both 32- and 64-bit values, not the host's
    // stub initializer. Namespace fallback in Mago reads this metadata instead of its platform helper.
    // Also retain numeric constants absent from the prelude (PHP 8.5's FILTER_THROW_ON_FAILURE)
    // and non-finite float literals, which the scanner's PHPDoc types otherwise generalize.
    for (name, value) in &builtins.constants {
        use pun_phpstan::types::Type;
        let values = match pun_phpstan::resolve::resolve_str(value, None) {
            Type::ConstInt(i) => vec![Type::ConstInt(i)],
            Type::ConstFloat(f) => vec![Type::ConstFloat(f)],
            Type::Union(parts) => parts,
            _ => continue,
        };
        let Some(values) = values
            .into_iter()
            .map(|t| match t {
                Type::ConstInt(i) => Some(TAtomic::Scalar(TScalar::literal_int(i))),
                Type::ConstFloat(f) => Some(TAtomic::Scalar(TScalar::literal_float(f))),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()
        else {
            continue;
        };
        let key = mago_word::ascii_lowercase_constant_name_word(name.as_bytes());
        let constant = metadata.constants.entry(key).or_insert_with(|| {
            ConstantMetadata::new(mago_word::word(name), Span::zero(), MetadataFlags::BUILTIN)
        });
        let value = TUnion::new(Cow::Owned(values));
        constant.type_metadata = Some(TypeMetadata::new(value.clone(), constant.span));
        constant.inferred_type = Some(value);
    }
    (metadata, symbol_references)
}

/// PHPStan's API stub's symbols.
fn scan_api(version: PHPVersion) -> CodebaseMetadata {
    let stub = File::new(
        Cow::Borrowed(b"phpstan-api.php"),
        FileType::Vendored,
        None,
        Cow::Borrowed(PHPSTAN_API),
    );
    scan(&mut LocalArena::new(), &stub, version, None)
}

/// Files scanned by one task: small enough that the threads finish together.
const CHUNK: usize = 32;

/// The analysed files as the scan read them, by path, for the rules to walk without reading them again.
pub(crate) type Sources = HashMap<PathBuf, Arc<File>>;

pub(crate) fn build(
    project: &Project,
    laravel_root: Option<&Path>,
) -> Result<CodebaseMetadata, CouldNotReadFile> {
    Ok(build_with_sources_at(project, laravel_root)?.0)
}

/// The codebase, and the analysed files' [`Sources`].
pub(crate) fn build_with_sources(
    project: &Project,
) -> Result<(CodebaseMetadata, Sources), CouldNotReadFile> {
    build_with_sources_at(project, None)
}

fn build_with_sources_at(
    project: &Project,
    laravel_root: Option<&Path>,
) -> Result<(CodebaseMetadata, Sources), CouldNotReadFile> {
    let version = php_version(project.php_version);
    let aliases = laravel_root
        .or_else(|| project.larastan.as_ref().map(|l| l.root.as_path()))
        .map(crate::larastan::aliases::load)
        .unwrap_or_default();
    let mut seen = HashSet::new();
    let mut files: Vec<_> = project
        .analysed
        .iter()
        .map(|p| (p, FileType::Host))
        .chain(project.scanned.iter().map(|p| (p, FileType::Vendored)))
        .chain(project.stubs.iter().map(|p| (p, FileType::Patch)))
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
    let index = autoload::Index::new(classes.into_iter(), &aliases);
    // Scanning is most of the time (the vendored files): in rounds, each the psr-4 and psr-0 files the previous one
    // named, in chunks across the threads, merged in file order, while the prelude is built and PHPStan's reflection
    // of built-ins is loaded (at the project's PHP version, which the rules read, and 8.4's, which pun-phpstan's type
    // system reads), rather than by the first file that asks while the others wait.
    let php_version = project.php_version;
    let builtins = move || {
        crate::rules::builtins(php_version);
        pun_phpstan::symbols::symbols(pun_phpstan::symbols::PhpVersion::Php84)
    };
    // PHPStan's API stub (one large file, about a quarter of the run on larastan and pest when scanned after the
    // rounds) is scanned on its own thread as soon as an analysed file names PHPStan, beside the rounds; it is merged
    // after them, as before
    let api = OnceLock::new();
    let (((mut metadata, mut references), _), scanned) = rayon::join(
        || rayon::join(move || prelude(php_version), builtins),
        || {
            let mut loaded = vec![false; files.len()];
            loaded[..eager].fill(true);
            let mut round: Vec<usize> = (0..eager).collect();
            let mut scanned = Vec::new();
            let mut sources = Sources::new();
            let mut used_aliases = Vec::new();
            while !round.is_empty() {
                let size = (round.len() / (rayon::current_num_threads() * 4)).clamp(1, CHUNK);
                let chunks = round
                    .par_chunks(size)
                    .map_init(LocalArena::new, |arena, chunk| {
                        let mut metadata = CodebaseMetadata::default();
                        let mut sources = Vec::new();
                        let mut wanted = Vec::new();
                        let mut declarations = Vec::new();
                        for &i in chunk {
                            let (path, file_type) = files[i];
                            let Some(file) = read(path, file_type)? else {
                                continue;
                            };
                            metadata.extend(scan(
                                arena,
                                &file,
                                version,
                                Some((&index, &mut wanted, &mut declarations)),
                            ));
                            wanted.retain(|&w| !loaded[w]);
                            if file_type == FileType::Host {
                                if names_phpstan(&file.contents) {
                                    api.get_or_init(|| {
                                        std::thread::spawn(move || scan_api(version))
                                    });
                                }
                                sources.push((path.clone(), Arc::new(file)));
                            }
                        }
                        Ok((metadata, sources, wanted, declarations))
                    })
                    .collect::<Result<Vec<_>, CouldNotReadFile>>()?;
                round.clear();
                for (chunk, files, wanted, declarations) in chunks {
                    scanned.push(chunk);
                    sources.extend(files);
                    round.extend(wanted);
                    used_aliases.extend(declarations);
                }
                round.sort_unstable();
                round.dedup();
                for &i in &round {
                    loaded[i] = true;
                }
            }
            Ok((scanned, sources, used_aliases))
        },
    );
    let (scanned, sources, mut used_aliases) = scanned?;
    for chunk in scanned {
        metadata.extend(chunk);
    }
    if !used_aliases.is_empty() {
        used_aliases.sort_unstable();
        used_aliases.dedup();
        // Feed declarations through Mago's class_alias scanner, before population resolves aliases, inheritance
        // and signatures. Real declarations win and missing/cyclic targets stay unknown in its normal registry.
        let quote = |s: &str| s.replace('\\', "\\\\").replace('\'', "\\'");
        let mut declarations = String::from("<?php\n");
        for i in used_aliases {
            let (alias, target) = &aliases[i];
            declarations.push_str(&format!(
                "\\class_alias('{}', '{}');\n",
                quote(target),
                quote(alias)
            ));
        }
        let file = File::new(
            Cow::Borrowed(b"pun://laravel-aliases"),
            FileType::Vendored,
            None,
            Cow::Owned(declarations.into_bytes()),
        );
        metadata.extend(scan(&mut LocalArena::new(), &file, version, None));
    }
    if let Some(api) = api.into_inner() {
        metadata.extend(api.join().expect("the API stub scan never panics"));
    }
    // ponytail: Mago leaves a user-defined (analysed) class unpatched, where PHPStan's stub wins; stubs of vendor
    // classes, the usual kind, apply alike
    metadata.apply_patches_pass();
    // Mago materialises a patch's method over an existing one only; a stub's `@method` adds one, as PHPStan's does
    let pseudo: Vec<_> = (metadata.patch_function_likes.iter())
        .filter(|(key, _)| !metadata.function_likes.contains_key(*key))
        .filter(|((class, name), _)| {
            (metadata.class_likes.get(class)).is_some_and(|k| {
                k.pseudo_methods.contains(name) || k.static_pseudo_methods.contains(name)
            })
        })
        .map(|(key, f)| (*key, f.clone()))
        .collect();
    metadata.function_likes.extend(pseudo);
    populate_codebase_parallel(
        &mut metadata,
        &mut references,
        Default::default(),
        Default::default(),
    );
    Ok((metadata, sources))
}
