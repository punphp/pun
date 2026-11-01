//! The symbols every rule queries: Mago's prelude (built-ins) extended with every analysed and scanned file.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, OnceLock};

use mago_allocator::{Arena, LocalArena};
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::PluginRegistry;
use mago_analyzer::settings::Settings;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::class_like_constant::ClassLikeConstantMetadata;
use mago_codex::metadata::constant::ConstantMetadata;
use mago_codex::metadata::flags::MetadataFlags;
use mago_codex::metadata::function_like::ReflectedSideEffects;
use mago_codex::metadata::ttype::TypeMetadata;
use mago_codex::populator::populate_codebase_parallel;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::inference::get_literal_constant_type;
use mago_codex::scanner::inference::infer_initializer_with_codebase;
use mago_codex::scanner::inference::initializer_class_names;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::get_empty;
use mago_codex::ttype::get_error;
use mago_codex::ttype::get_int_or_float;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::get_named_object;
use mago_codex::ttype::get_object;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{File, FileId, FileType};
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_names::scope::NamespaceScope;
use mago_php_version::PHPVersion;
use mago_prelude::Prelude;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::{self, ConstantAccess, Expression, Program};
use mago_syntax::parser::parse_file_with_settings;
use mago_syntax::settings::{LexerSettings, ParserSettings};
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
    let program = parse_file_with_settings(
        arena,
        file,
        ParserSettings {
            lexer: LexerSettings {
                retain_invalid_legacy_octal: true,
                ..Default::default()
            },
            ..Default::default()
        },
    );
    (program, NameResolver::new(arena).resolve(program))
}

/// Scans `file`, then resets `arena`, so one arena serves many files.
fn scan(
    arena: &mut LocalArena,
    file: &File,
    version: PHPVersion,
    reachable: Option<(&autoload::Index, &mut Vec<usize>, &mut Vec<usize>)>,
    trusted_aliases: bool,
) -> CodebaseMetadata {
    let mut scanned = 'scan: {
        let arena = &*arena;
        let (program, names) = parse(arena, file);
        // FileNodesFetcher rejects the whole file before exporting any reflection or autoload roots.
        if !crate::parser::errors(file, program, version).is_empty() {
            break 'scan CodebaseMetadata::default();
        }
        if let Some((index, wanted, declarations)) = reachable {
            index.wanted(&file.contents, program, &names, wanted, declarations);
        }
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            scan_program(arena, file, program, &names, version)
        }))
        .unwrap_or_default()
    };
    arena.reset();
    // PHPStan's source locators do not execute class_alias(). Admit only registrations already established by
    // the Laravel loader below, before population can expose source-only aliases to inference or any rule.
    if !trusted_aliases {
        let keys = mago_codex::metadata::CodebaseEntryKeys {
            class_like_names: Vec::new(),
            class_like_aliases: scanned.class_like_alias_declarations().collect(),
            function_like_keys: Vec::new(),
            constant_names: Vec::new(),
            file_ids: Vec::new(),
        };
        scanned.remove_entries_by_keys(&keys);
    }
    if file.file_type == FileType::Patch {
        scanned.convert_partial_to_patch();
    }
    scanned
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
    // NativeMethodReflection's effect is a reflection fact, not a stub PHPDoc annotation.
    for ((class, name), method) in &mut metadata.function_likes {
        if let Some(reflected) = builtins
            .classes
            .get(&String::from_utf8_lossy(class.as_bytes()).to_ascii_lowercase())
            .and_then(|class| class.methods.get(&String::from_utf8_lossy(name.as_bytes()).to_ascii_lowercase()))
        {
            method.reflected_side_effects = Some(match reflected.side_effects.as_str() {
                "Yes" => ReflectedSideEffects::Yes,
                "No" => ReflectedSideEffects::No,
                _ => ReflectedSideEffects::Maybe,
            });
        }
    }
    // Runtime reflection supplies empty-array constructor defaults even when an API stub
    // writes null (ArrayObject and ArrayIterator). Generic inference must see the real value.
    for ((class, name), constructor) in &mut metadata.function_likes {
        if !name.as_bytes().eq_ignore_ascii_case(b"__construct") {
            continue;
        }
        let Some(variant) = builtins
            .classes
            .get(&String::from_utf8_lossy(class.as_bytes()).to_ascii_lowercase())
            .and_then(|class| class.methods.get("__construct"))
            .and_then(|constructor| constructor.variants.first())
        else {
            continue;
        };
        for (parameter, reflected) in constructor.parameters.iter_mut().zip(&variant.params) {
            if reflected.default.as_deref() == Some("array{}") {
                parameter.default_type = Some(TypeMetadata::new(
                    mago_codex::ttype::get_empty_keyed_array(),
                    parameter.span,
                ));
                parameter.flags.insert(MetadataFlags::HAS_DEFAULT);
            }
        }
    }
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
    scan(&mut LocalArena::new(), &stub, version, None, false)
}

/// Files scanned by one task: small enough that the threads finish together.
const CHUNK: usize = 32;

/// Analysed files and loaded global-constant declaration files, as the scan read them.
pub(crate) type Sources = HashMap<PathBuf, Arc<File>>;

/// The codebase, and the analysed files' [`Sources`].
pub(crate) fn build(
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
                            let symbols = scan(
                                arena,
                                &file,
                                version,
                                Some((&index, &mut wanted, &mut declarations)),
                                false,
                            );
                            let has_constants = !symbols.constants.is_empty();
                            metadata.extend(symbols);
                            wanted.retain(|&w| !loaded[w]);
                            if file_type == FileType::Host {
                                if names_phpstan(&file.contents) {
                                    api.get_or_init(|| {
                                        std::thread::spawn(move || scan_api(version))
                                    });
                                }
                            }
                            if file_type == FileType::Host || has_constants {
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
        metadata.extend(scan(&mut LocalArena::new(), &file, version, None, true));
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
    resolve_initializers(&mut metadata, &sources, project);
    Ok((metadata, sources))
}

/// Resolve referenced globals and constants of already-loaded user classes/interfaces. Autoload remains
/// lazy, and evaluating one initializer never analyzes that file's unrelated executable bodies.
fn resolve_initializers(metadata: &mut CodebaseMetadata, sources: &Sources, project: &Project) {
    let mut roots = Vec::new();
    let mut arena = LocalArena::new();
    for path in &project.analysed {
        if let Some(file) = sources.get(path) {
            let (program, names) = parse(&arena, file);
            if !crate::parser::errors(file, program, php_version(project.php_version)).is_empty() {
                arena.reset();
                continue;
            }
            let mut inputs = ConstantInputs {
                metadata,
                names: &names,
                names_found: Vec::new(),
                class: None,
            };
            inputs.walk_program(program, &mut ());
            roots.extend(inputs.names_found);
            arena.reset();
        }
    }
    // BetterReflection selects a non-private inherited constant before an explicitly reused trait.
    // Normalize these copies before evaluating their source, without rebinding the selected owner.
    let inherited: Vec<_> = metadata
        .class_likes
        .iter()
        .flat_map(|(name, class)| {
            class
                .constants
                .keys()
                .filter_map(|member| {
                    let mut seen = HashSet::new();
                    let ancestors: Vec<_> = std::iter::successors(Some(class.as_ref()), |k| {
                        k.direct_parent_class
                            .and_then(|p| metadata.get_class_like_by_name(p))
                    })
                    .take_while(|k| seen.insert(k.name))
                    .collect();
                    let selected = ancestors.into_iter().rev().fold(
                        None::<&ClassLikeConstantMetadata>,
                        |inherited, k| {
                            let current = k.constants.get(member);
                            let own = current.is_some_and(|m| {
                                k.span.file_id == m.span.file_id
                                    && k.span.start.offset <= m.span.start.offset
                                    && m.span.end.offset <= k.span.end.offset
                            });
                            if !own
                                && let Some(inherited) = inherited.filter(|m| {
                                    m.visibility != mago_codex::visibility::Visibility::Private
                                })
                            {
                                Some(inherited)
                            } else {
                                current
                            }
                        },
                    );
                    selected
                        .filter(|k| k.span != class.constants[member].span)
                        .map(|k| (*name, *member, k.clone()))
                })
                .collect::<Vec<_>>()
        })
        .collect();
    for (class, member, constant) in inherited {
        metadata
            .class_likes
            .get_mut(&class)
            .expect("selected class exists")
            .constants
            .insert(member, constant);
    }
    let trait_sources: Vec<_> = metadata
        .class_likes
        .iter()
        .filter(|(_, class)| !class.kind.is_trait())
        .flat_map(|(name, class)| {
            class
                .used_traits
                .iter()
                .filter_map(|t| metadata.get_class_like_by_name(*t))
                .flat_map(|t| t.constants.values().map(|constant| (*name, constant.span)))
                .collect::<Vec<_>>()
        })
        .collect();
    // Loaded declarations must expose one canonical value to every consumer, including wildcard
    // PHPDoc and existing class-string accesses whose member is not a static signature reference.
    roots.extend(metadata.class_likes.iter().flat_map(|(name, class)| {
        class
            .constants
            .iter()
            .filter_map(move |(member, constant)| {
                (!constant.flags.is_built_in()).then_some(Initializer::Class(*name, *member))
            })
    }));
    let arguments: Vec<_> = metadata
        .class_likes
        .iter()
        .flat_map(|(name, class)| {
            class
                .attributes
                .iter()
                .enumerate()
                .flat_map(move |(attribute, value)| {
                    value
                        .arguments
                        .iter()
                        .enumerate()
                        .filter_map(move |(argument, value)| {
                            value.value_span.map(|at| (*name, attribute, argument, at))
                        })
                })
        })
        .collect();
    let mut initializer_sources: HashMap<_, _> = sources
        .values()
        .map(|file| (file.id, file.as_ref()))
        .collect();
    // Symbol-only files without global declarations are not retained in Sources. Reopen only files
    // of already-loaded declarations, never arbitrary autoload files or their executable bodies.
    let missing: HashSet<_> = arguments
        .iter()
        .map(|(_, _, _, at)| at.file_id)
        .chain(
            metadata
                .class_likes
                .values()
                .filter(|class| !class.flags.is_built_in())
                .flat_map(|class| {
                    class
                        .constants
                        .values()
                        .map(|constant| constant.span.file_id)
                        .chain(std::iter::once(class.span.file_id))
                }),
        )
        .filter(|id| !initializer_sources.contains_key(id))
        .collect();
    let attribute_sources: Vec<_> = project
        .scanned
        .iter()
        .chain(&project.stubs)
        .chain(project.autoloaded.iter().map(|file| &file.path))
        .filter(|path| missing.contains(&FileId::new(path.to_string_lossy().as_bytes())))
        .filter_map(|path| read(path, FileType::Vendored).ok().flatten())
        .collect();
    initializer_sources.extend(attribute_sources.iter().map(|file| (file.id, file)));
    let mut copies: HashMap<_, Vec<_>> = HashMap::new();
    let mut classes: HashMap<_, Vec<_>> = HashMap::new();
    for (class, k) in &metadata.class_likes {
        classes.entry(k.span.file_id).or_default().push(*class);
        for (name, constant) in &k.constants {
            copies
                .entry((constant.span, *name))
                .or_default()
                .push(*class);
        }
    }
    let arena = LocalArena::new();
    let mut resolver = Initializers {
        metadata,
        sources: initializer_sources,
        arena: &arena,
        parsed: HashMap::new(),
        version: php_version(project.php_version),
        visited: HashSet::new(),
        requested: HashSet::new(),
        copies,
        classes,
        path_constants: project.use_path_constants_as_constant_string,
    };
    for name in roots {
        resolver.resolve(name);
    }
    // Shadowed trait declarations are still checked independently. Resolve their source dependencies
    // too; reading Consumer::VALUE would otherwise silently evaluate the overriding declaration.
    for (class, at) in trait_sources {
        resolver.evaluate(at, false, Some(class));
    }
    for (name, attribute, argument, at) in arguments {
        if let Some(value) = resolver.evaluate(at, true, Some(name)) {
            resolver
                .metadata
                .class_likes
                .get_mut(&name)
                .expect("selected class exists")
                .attributes[attribute]
                .arguments[argument]
                .value_type = Some(value);
        }
    }
}

struct ConstantInputs<'a, 'arena> {
    metadata: &'a CodebaseMetadata,
    names: &'a ResolvedNames<'arena>,
    names_found: Vec<Initializer>,
    class: Option<mago_word::Word>,
}

fn selected_global<'a>(
    metadata: &'a CodebaseMetadata,
    names: &ResolvedNames<'_>,
    access: &ConstantAccess<'_>,
) -> Option<&'a ConstantMetadata> {
    metadata.get_constant(names.get(access)).or_else(|| {
        (!access.name.value().contains(&b'\\') && !names.is_imported(&access.name))
            .then(|| metadata.get_constant(access.name.value()))
            .flatten()
    })
}

#[derive(Clone, Copy)]
enum Initializer {
    Global(mago_word::Word),
    Class(mago_word::Word, mago_word::Word),
}

/// Ordinary inherited copies share a source declaration. Trait rebinding intentionally does not use
/// this span identity: the same trait initializer can mean different things in different consumers.
pub(crate) fn initializer_class(
    metadata: &CodebaseMetadata,
    at: Span,
) -> Option<&ClassLikeMetadata> {
    smallest_containing(metadata.class_likes.values().map(Box::as_ref), at)
}

fn smallest_containing<'m>(
    classes: impl Iterator<Item = &'m ClassLikeMetadata>,
    at: Span,
) -> Option<&'m ClassLikeMetadata> {
    classes
        .filter(|class| {
            class.span.file_id == at.file_id
                && class.span.start.offset <= at.start.offset
                && at.end.offset <= class.span.end.offset
        })
        .min_by_key(|class| class.span.end.offset - class.span.start.offset)
}

/// A trait copy is implemented by its consuming class, unless a non-private parent constant was
/// already selected. BetterReflection merges parents before traits, even after an explicit reuse.
pub(crate) fn initializer_owner(
    metadata: &CodebaseMetadata,
    class: mago_word::Word,
    name: mago_word::Word,
) -> Option<mago_word::Word> {
    let constant = metadata.get_class_constant(class.as_bytes(), name.as_bytes())?;
    copy_owner(
        metadata,
        class,
        name,
        constant.span,
        initializer_class(metadata, constant.span)?,
    )
}

/// The owner of `class`'s copy of the constant declared at `span` in `source`.
fn copy_owner(
    metadata: &CodebaseMetadata,
    class: mago_word::Word,
    name: mago_word::Word,
    span: Span,
    source: &ClassLikeMetadata,
) -> Option<mago_word::Word> {
    if !source.kind.is_trait() {
        return Some(source.name);
    }
    let mut owner = metadata.get_class_like_by_name(class)?;
    let mut seen = HashSet::new();
    while seen.insert(owner.name)
        && let Some(parent) = owner
            .direct_parent_class
            .and_then(|p| metadata.get_class_like_by_name(p))
    {
        if !parent.constants.get(&name).is_some_and(|k| {
            k.span == span && k.visibility != mago_codex::visibility::Visibility::Private
        }) {
            break;
        }
        owner = parent;
    }
    (!owner.kind.is_trait()).then_some(owner.name)
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for ConstantInputs<'_, 'arena> {
    fn walk_in_constant_access(&mut self, access: &'ast ConstantAccess<'arena>, _: &mut ()) {
        // The same selection as ConstantAccess::analyze: existence stops fallback, and imported or
        // qualified names can never accidentally select an unrelated global constant.
        let selected = selected_global(self.metadata, self.names, access);
        if let Some(selected) = selected.filter(|k| !k.flags.is_built_in()) {
            self.names_found.push(Initializer::Global(selected.name));
        }
    }

    fn walk_in_class_constant_access(
        &mut self,
        access: &'ast cst::ClassConstantAccess<'arena>,
        _: &mut (),
    ) {
        let cst::ClassLikeConstantSelector::Identifier(name) = &access.constant else {
            return;
        };
        let class = match access.class.unparenthesized() {
            Expression::Identifier(name) => mago_word::word(self.names.get(name)),
            Expression::Self_(_) | Expression::Static(_) => {
                let Some(class) = self
                    .class
                    .or_else(|| initializer_class(self.metadata, access.span()).map(|k| k.name))
                else {
                    return;
                };
                class
            }
            Expression::Parent(_) => {
                let Some(parent) = self
                    .class
                    .and_then(|k| self.metadata.get_class_like_by_name(k))
                    .or_else(|| initializer_class(self.metadata, access.span()))
                    .and_then(|class| class.direct_parent_class)
                else {
                    return;
                };
                parent
            }
            _ => return,
        };
        if self
            .metadata
            .get_class_constant(class.as_bytes(), name.value)
            .is_some_and(|constant| !constant.flags.is_built_in())
        {
            self.names_found
                .push(Initializer::Class(class, mago_word::word(name.value)));
        }
    }
}

struct Initializers<'a> {
    metadata: &'a mut CodebaseMetadata,
    sources: HashMap<FileId, &'a File>,
    /// Each source is parsed once into `arena`; a file declares many constants.
    arena: &'a LocalArena,
    parsed: HashMap<FileId, (&'a Program<'a>, Rc<ResolvedNames<'a>>)>,
    version: PHPVersion,
    visited: HashSet<(Span, Option<mago_word::Word>)>,
    /// Class constants already requested: a repeat selects the same declaration and owner.
    requested: HashSet<(mago_word::Word, mago_word::Word)>,
    /// The classes holding each class constant declaration, by its span and name.
    copies: HashMap<(Span, mago_word::Word), Vec<mago_word::Word>>,
    /// The classes declared in each file.
    classes: HashMap<FileId, Vec<mago_word::Word>>,
    path_constants: bool,
}

impl Initializers<'_> {
    fn resolve(&mut self, input: Initializer) {
        match input {
            Initializer::Global(name) => self.resolve_global(name),
            Initializer::Class(class, name) => self.resolve_class(class, name),
        }
    }

    fn resolve_class(&mut self, class: mago_word::Word, name: mago_word::Word) {
        if !self.requested.insert((class, name)) {
            return;
        }
        let at = self
            .metadata
            .get_class_constant(class.as_bytes(), name.as_bytes())
            .expect("selected class constant exists")
            .span;
        // Synthetic API stubs and unavailable sources retain their scanned values. They cannot be
        // re-evaluated, and must not trigger repeated declaring-owner searches or inherited copies.
        if !self.sources.contains_key(&at.file_id) {
            return;
        }
        let in_file = (self.classes.get(&at.file_id).into_iter().flatten())
            .filter_map(|class| self.metadata.class_likes.get(class).map(Box::as_ref));
        let Some(source) = smallest_containing(in_file, at) else {
            return;
        };
        let Some(owner) = copy_owner(self.metadata, class, name, at, source) else {
            return;
        };
        // Population copies ordinary class/interface constants, keeping the declaration's span.
        // Every copy with that declaring source identity gets the one raw initializer below.
        let copies: Vec<_> = (self.copies.get(&(at, name)).into_iter().flatten())
            .copied()
            .filter(|class| copy_owner(self.metadata, *class, name, at, source) == Some(owner))
            .collect();
        if !self.visited.insert((at, Some(owner))) {
            return;
        }
        let declaration = self
            .metadata
            .class_likes
            .get_mut(&owner)
            .expect("declaring class exists")
            .constants
            .get_mut(&name)
            .expect("declaring constant exists");
        // Preserve the scanner's direct-self sentinel and existing expansion cycle guards.
        if declaration
            .inferred_type
            .as_ref()
            .is_some_and(TUnion::is_never)
        {
            return;
        }
        let previous = declaration.inferred_type.take();
        let value = self.evaluate(at, false, Some(owner)).or(previous);
        // Native/PHPDoc declarations of the copies stay unchanged.
        for class in copies {
            self.metadata
                .class_likes
                .get_mut(&class)
                .expect("selected class exists")
                .constants
                .get_mut(&name)
                .expect("selected constant exists")
                .inferred_type
                .clone_from(&value);
        }
    }

    fn resolve_global(&mut self, name: mago_word::Word) {
        let declaration = self
            .metadata
            .constants
            .get_mut(&name)
            .expect("selected constant exists");
        let at = declaration.span;
        if !self.visited.insert((at, None)) {
            return;
        }
        // ConstantResolver's currentlyResolving entry is mixed, not a copied scanner approximation.
        // A global constant reflection uses its value, not its PHPDoc @var annotation.
        let previous = declaration.inferred_type.take();
        let declared = declaration.type_metadata.take();
        let mut value = self.evaluate(at, false, None);
        if let Some(value) = value.as_mut()
            && value.is_mixed()
            && let Some(previous) = previous.as_ref()
            && previous == &get_int_or_float()
        {
            // Mago's full analyzer loses the scanner's numeric result when a cycle supplies mixed.
            // Retain that existing bound, with PHPStan's benevolent-union flags, not an invented value.
            *value = previous.clone();
            value.set_ignore_nullable_issues(true);
            value.set_ignore_falsable_issues(true);
        }
        let declaration = self
            .metadata
            .constants
            .get_mut(&name)
            .expect("selected constant exists");
        declaration.type_metadata = value
            .clone()
            .map(|value| TypeMetadata::new(value, at))
            .or(declared);
        declaration.inferred_type = value.or(previous);
    }

    fn evaluate(
        &mut self,
        at: Span,
        require_global: bool,
        class: Option<mago_word::Word>,
    ) -> Option<TUnion> {
        let file = *self.sources.get(&at.file_id)?;
        let arena = self.arena;
        let (program, names) = self
            .parsed
            .entry(at.file_id)
            .or_insert_with(|| {
                let (program, names) = parse(arena, file);
                (program, Rc::new(names))
            })
            .clone();
        let mut find = FindInitializer {
            at,
            argument: require_global,
            expression: None,
            namespace: None,
        };
        find.walk_program(program, &mut ());
        let expression = find.expression?;
        let mut inputs = ConstantInputs {
            metadata: self.metadata,
            names: &names,
            names_found: Vec::new(),
            class,
        };
        inputs.walk_expression(expression, &mut ());
        // Expression holders must discover their declarations after their own dependencies have
        // canonical values. Postorder also handles a class constant whose value is another holder.
        struct Holders<'ast, 'arena>(Vec<(&'ast Expression<'arena>, mago_word::Word)>);
        impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Holders<'ast, 'arena> {
            fn walk_out_class_constant_access(
                &mut self,
                access: &'ast cst::ClassConstantAccess<'arena>,
                _: &mut (),
            ) {
                if !matches!(
                    access.class.unparenthesized(),
                    Expression::Identifier(_)
                        | Expression::Self_(_)
                        | Expression::Parent(_)
                        | Expression::Static(_)
                ) && let cst::ClassLikeConstantSelector::Identifier(name) = &access.constant
                    && !name.value.eq_ignore_ascii_case(b"class")
                {
                    self.0.push((access.class, mago_word::word(name.value)));
                }
            }
        }
        let mut holders = Holders(Vec::new());
        holders.walk_expression(expression, &mut ());
        // Literal/unsupported arguments without a constant dependency retain their scanner result.
        if require_global && inputs.names_found.is_empty() && holders.0.is_empty() {
            return None;
        }
        let mut inherited_benevolence = false;
        for input in inputs.names_found {
            self.resolve(input);
            if let Initializer::Global(name) = input
                && let Some(value) = self
                    .metadata
                    .get_constant(name.as_bytes())
                    .and_then(|k| k.type_metadata.as_ref())
            {
                inherited_benevolence |= value.type_union.ignore_nullable_issues()
                    && value.type_union.ignore_falsable_issues();
            }
        }
        for (expression, name) in holders.0 {
            let holder = self.inference(class).initializer_type(
                &arena,
                file,
                program,
                &names,
                expression,
                find.namespace.clone(),
            );
            for target in initializer_class_names(self.metadata, &holder) {
                if self
                    .metadata
                    .get_class_constant(target.as_bytes(), name.as_bytes())
                    .is_some_and(|constant| !constant.flags.is_built_in())
                {
                    self.resolve_class(target, name);
                }
            }
        }
        let mut value = self.inference(class).initializer_type(
            &arena,
            file,
            program,
            &names,
            expression,
            find.namespace,
        );
        if inherited_benevolence && value == get_int_or_float() {
            // Keep the numeric union's existing marker across the full evaluator's arithmetic join.
            value.set_ignore_nullable_issues(true);
            value.set_ignore_falsable_issues(true);
        }
        Some(value)
    }

    fn inference(&self, class: Option<mago_word::Word>) -> InitializerInference<'_> {
        InitializerInference {
            metadata: self.metadata,
            version: self.version,
            class,
            path_constants: self.path_constants,
            path: class
                .and_then(|k| self.metadata.get_class_like_by_name(k))
                .and_then(|k| self.sources.get(&k.span.file_id))
                .and_then(|f| f.path.clone()),
        }
    }
}

struct FindInitializer<'ast, 'arena> {
    at: Span,
    argument: bool,
    expression: Option<&'ast Expression<'arena>>,
    namespace: Option<cst::Namespace<'arena>>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for FindInitializer<'ast, 'arena> {
    fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, _: &mut ()) {
        if expression.span() == self.at {
            self.expression = Some(expression);
        }
    }
    fn walk_in_namespace(&mut self, namespace: &'ast cst::Namespace<'arena>, _: &mut ()) {
        if namespace.span().start.offset <= self.at.start.offset
            && self.at.end.offset <= namespace.span().end.offset
        {
            self.namespace = Some(namespace.clone());
        }
    }
    fn walk_in_constant_item(&mut self, item: &'ast cst::ConstantItem<'arena>, _: &mut ()) {
        if item.span() == self.at {
            self.expression = Some(item.value);
        }
    }
    fn walk_in_class_like_constant_item(
        &mut self,
        item: &'ast cst::ClassLikeConstantItem<'arena>,
        _: &mut (),
    ) {
        if item.span() == self.at {
            self.expression = Some(item.value);
        }
    }
    fn walk_in_function_call(&mut self, call: &'ast cst::FunctionCall<'arena>, _: &mut ()) {
        if !self.argument && call.span() == self.at {
            self.expression = call.argument_list.arguments.get(1).map(|arg| arg.value());
        }
    }
}

/// Compare a source initializer even when reflection selected a shadowing declaration. Dependencies
/// were resolved at population time; this read-only entry point uses the same canonical engine.
pub(crate) fn source_initializer(
    metadata: &CodebaseMetadata,
    file: &File,
    at: Span,
    class: mago_word::Word,
    version: u32,
    path_constants: bool,
    path: Option<PathBuf>,
) -> Option<TUnion> {
    let arena = LocalArena::new();
    let (program, names) = parse(&arena, file);
    let mut find = FindInitializer {
        at,
        argument: false,
        expression: None,
        namespace: None,
    };
    find.walk_program(program, &mut ());
    Some(
        InitializerInference {
            metadata,
            version: php_version(version),
            class: Some(class),
            path_constants,
            path,
        }
        .initializer_type(
            &arena,
            file,
            program,
            &names,
            find.expression?,
            find.namespace,
        ),
    )
}

/// Declaration/attribute checks use ordinary expression flow in each consuming class, independently
/// of the raw reflection union and of the original trait file's first inference pass.
pub(crate) fn trait_initializer_types(
    metadata: &CodebaseMetadata,
    file: &File,
    at: Span,
    class: mago_word::Word,
    version: u32,
) -> HashMap<Span, TUnion> {
    struct Inputs<'a> {
        metadata: &'a CodebaseMetadata,
        at: Span,
        spans: Vec<Span>,
    }
    impl Inputs<'_> {
        fn owns(&self, at: Span) -> bool {
            // Nested anonymous/named classes keep their own lexical scope, not the trait consumer's.
            self.at.start.offset <= at.start.offset
                && at.end.offset <= self.at.end.offset
                && initializer_class(self.metadata, at).is_some_and(|k| k.span == self.at)
        }
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Inputs<'_> {
        fn walk_in_class_like_constant_item(
            &mut self,
            item: &'ast cst::ClassLikeConstantItem<'arena>,
            _: &mut (),
        ) {
            if self.owns(item.span()) {
                self.spans.push(item.value.span());
            }
        }
        fn walk_in_attribute(&mut self, attribute: &'ast cst::Attribute<'arena>, _: &mut ()) {
            if self.owns(attribute.span()) {
                if let Some(arguments) = &attribute.argument_list {
                    self.spans.extend(
                        arguments
                            .arguments
                            .iter()
                            .filter_map(|arg| arg.value())
                            .map(HasSpan::span),
                    );
                }
            }
        }
    }
    let arena = LocalArena::new();
    let (program, names) = parse(&arena, file);
    let mut inputs = Inputs {
        metadata,
        at,
        spans: Vec::new(),
    };
    inputs.walk_program(program, &mut ());
    let inference = InitializerInference {
        metadata,
        version: php_version(version),
        class: Some(class),
        path_constants: true,
        path: None,
    };
    let mut types = HashMap::new();
    for at in inputs.spans {
        let mut find = FindInitializer {
            at,
            argument: true,
            expression: None,
            namespace: None,
        };
        find.walk_program(program, &mut ());
        let expression = find.expression.expect("collected expression exists");
        let artifacts = inference.expression_artifacts(
            &arena,
            file,
            program,
            &names,
            expression,
            find.namespace,
            false,
            None,
        );
        types.extend(
            artifacts
                .into_iter()
                .flat_map(|artifacts| artifacts.expression_types)
                .map(|((start, end), value)| {
                    (
                        Span::new(
                            file.id,
                            mago_span::Position::new(start),
                            mago_span::Position::new(end),
                        ),
                        (*value).clone(),
                    )
                }),
        );
    }
    types
}

struct InitializerInference<'a> {
    metadata: &'a CodebaseMetadata,
    version: PHPVersion,
    class: Option<mago_word::Word>,
    path_constants: bool,
    path: Option<PathBuf>,
}

impl InitializerInference<'_> {
    fn initializer_type<'arena>(
        &self,
        arena: &'arena LocalArena,
        file: &File,
        program: &'arena Program<'arena>,
        names: &ResolvedNames<'arena>,
        expression: &'arena Expression<'arena>,
        namespace: Option<cst::Namespace<'arena>>,
    ) -> TUnion {
        // Full inference supplies exact arithmetic only for independent subtrees without canonical
        // control flow. Never evaluate such an operand inside its parent's condition: narrowing there
        // would replace reflection unions with flow-selected values again. Unsupported reflection
        // leaves are mixed; function extensions and constructor inference belong to ordinary flow.
        struct Islands<'ast, 'arena> {
            stack: Vec<(Span, bool)>,
            expressions: Vec<(&'ast Expression<'arena>, bool, Option<Span>)>,
        }
        impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Islands<'ast, 'arena> {
            fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, _: &mut ()) {
                self.stack.push((
                    expression.span(),
                    matches!(
                        expression,
                        Expression::Conditional(_)
                            | Expression::Call(_)
                            | Expression::Instantiation(_)
                            | Expression::ConstantAccess(_)
                            | Expression::Access(cst::Access::ClassConstant(_))
                            | Expression::Binary(cst::Binary {
                                operator: cst::BinaryOperator::NullCoalesce(_),
                                ..
                            })
                    ),
                ));
            }
            fn walk_out_expression(&mut self, expression: &'ast Expression<'arena>, _: &mut ()) {
                let (_, scanner_owned) = self.stack.pop().expect("entered expression");
                let parent = self.stack.last_mut().map(|parent| {
                    parent.1 |= scanner_owned;
                    parent.0
                });
                self.expressions.push((expression, scanner_owned, parent));
            }
        }
        let mut islands = Islands {
            stack: Vec::new(),
            expressions: Vec::new(),
        };
        islands.walk_expression(expression, &mut ());
        let scanner_owned: HashSet<_> = islands
            .expressions
            .iter()
            .filter(|(_, owned, _)| *owned)
            .map(|(expression, _, _)| expression.span())
            .collect();
        let scope = NamespaceScope::new(
            namespace
                .as_ref()
                .and_then(|n| n.name.as_ref())
                .map(|n| n.value().to_vec()),
        );
        let mut types = HashMap::new();
        for (expression, _, _) in
            islands
                .expressions
                .iter()
                .copied()
                .filter(|(expression, owned, parent)| {
                    !owned && parent.is_none_or(|parent| scanner_owned.contains(&parent))
                        // Class holder names are syntax, not standalone value expressions.
                        && !matches!(expression, Expression::Identifier(_) | Expression::Self_(_) | Expression::Parent(_) | Expression::Static(_))
                })
        {
            if let Some(value) = self
                .expression_artifacts(
                    arena,
                    file,
                    program,
                    names,
                    expression,
                    namespace.clone(),
                    true,
                    None,
                )
                .and_then(|artifacts| artifacts.get_expression_type(expression).cloned())
            {
                types.insert(expression.span(), value);
            }
        }
        // Replay only the canonical nodes, in postorder, through the same scanner query. The type
        // map supplies reflection-owned leaves, not a second arithmetic/control-flow evaluator.
        for (expression, _, _) in islands
            .expressions
            .into_iter()
            .filter(|(_, owned, _)| *owned)
        {
            let mut value = match expression {
                // InitializerExprTypeResolver::getType(New_): constructor inference belongs only
                // to declaration flow. Even self/parent/static are literal ObjectType identities.
                Expression::Instantiation(new) => match new.class {
                    Expression::Identifier(name) => {
                        get_named_object(mago_word::word(names.get(name)), None)
                    }
                    Expression::Self_(_) | Expression::Parent(_) | Expression::Static(_) => {
                        get_named_object(
                            mago_word::word(&file.contents[new.class.span().to_range_usize()]),
                            None,
                        )
                    }
                    _ => get_object(),
                },
                Expression::ConstantAccess(access) => {
                    match selected_global(self.metadata, names, access) {
                        Some(constant) if !constant.flags.is_built_in() => {
                            constant.inferred_type.clone().unwrap_or_else(get_mixed)
                        }
                        Some(_) => self
                            .expression_artifacts(
                                arena,
                                file,
                                program,
                                names,
                                expression,
                                namespace.clone(),
                                true,
                                None,
                            )
                            .and_then(|artifacts| {
                                artifacts.get_expression_type(expression).cloned()
                            })
                            .unwrap_or_else(get_mixed),
                        // InitializerExprTypeResolver::getType(ConstFetch) returns ErrorType when
                        // reflection cannot find the constant. Ordinary expression flow stays separate.
                        None => {
                            get_literal_constant_type(access.name.value()).unwrap_or_else(get_error)
                        }
                    }
                }
                // The existing analyzer owns arithmetic, array keys and other enclosing syntax.
                // Supply canonical children instead of reselecting their initializer branches.
                _ if !matches!(
                    expression,
                    Expression::Conditional(_)
                        | Expression::Parenthesized(_)
                        | Expression::Call(_)
                        | Expression::Access(cst::Access::ClassConstant(_))
                        | Expression::Binary(cst::Binary {
                            operator: cst::BinaryOperator::NullCoalesce(_),
                            ..
                        })
                ) && !matches!(expression, Expression::Binary(binary) if binary.operator.is_bitwise()) =>
                {
                    self.expression_artifacts(
                        arena,
                        file,
                        program,
                        names,
                        expression,
                        namespace.clone(),
                        true,
                        Some(&types),
                    )
                    .and_then(|artifacts| artifacts.get_expression_type(expression).cloned())
                    .unwrap_or_else(get_mixed)
                }
                _ => infer_initializer_with_codebase(
                    file,
                    names,
                    &scope,
                    expression,
                    self.metadata,
                    self.class,
                    &types,
                )
                .unwrap_or_else(get_mixed),
            };
            // Materialize PHPStan's removeFalsey result using the existing excluded-set
            // representation. Mago's legacy truthiness flags alone lose this reflection identity.
            for atomic in value.types.to_mut() {
                if let TAtomic::Mixed(mixed) = atomic
                    && mixed.is_truthy()
                {
                    *atomic = mixed
                        .subtract(&get_empty(), self.metadata)
                        .get_single_owned();
                }
            }
            types.insert(expression.span(), value);
        }
        infer_initializer_with_codebase(
            file,
            names,
            &scope,
            expression,
            self.metadata,
            self.class,
            &types,
        )
        .unwrap_or_else(get_mixed)
    }

    fn expression_artifacts<'arena>(
        &self,
        arena: &'arena LocalArena,
        file: &File,
        program: &'arena Program<'arena>,
        names: &ResolvedNames<'arena>,
        expression: &'arena Expression<'arena>,
        namespace: Option<cst::Namespace<'arena>>,
        canonical: bool,
        canonical_values: Option<&HashMap<Span, TUnion>>,
    ) -> Option<mago_analyzer::artifacts::AnalysisArtifacts> {
        let statement = cst::Statement::Expression(cst::ExpressionStatement {
            expression,
            terminator: cst::Terminator::Semicolon(expression.span()),
        });
        let statements =
            cst::Sequence::from_slice(arena.alloc_slice_fill_iter([statement].into_iter()));
        let statement = if let Some(mut namespace) = namespace {
            match &mut namespace.body {
                cst::NamespaceBody::Implicit(body) => body.statements = statements,
                cst::NamespaceBody::BraceDelimited(body) => body.statements = statements,
            }
            cst::Statement::Namespace(namespace)
        } else {
            statements[0].clone()
        };
        let isolated = arena.alloc(Program {
            statements: cst::Sequence::from_slice(
                arena.alloc_slice_fill_iter([statement].into_iter()),
            ),
            ..program.clone()
        });
        let mut registry = PluginRegistry::with_library_providers();
        if let Some(values) = canonical_values {
            registry.register_expression_hook(InitializerValues(values.clone()));
        }
        if let Some(class) = self.class {
            registry.register_expression_hook(InitializerScope {
                class,
                canonical,
                path_constants: self.path_constants,
                path: self.path.clone(),
            });
        }
        let analyzer = Analyzer::new(
            arena,
            file,
            names,
            self.metadata,
            &registry,
            Settings::new(self.version),
        );
        let mut result = AnalysisResult::new(SymbolReferences::new());
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            analyzer.analyze_with_artifacts(isolated, &mut result)
        }))
        .ok()?
        .ok()
    }
}

/// Array inference consumes the same already-resolved initializer values as the scanner query.
struct InitializerValues(HashMap<Span, TUnion>);

impl mago_analyzer::plugin::ExpressionHook for InitializerValues {
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut mago_analyzer::plugin::HookContext<'_, '_>,
    ) -> mago_analyzer::plugin::HookResult<mago_analyzer::plugin::ExpressionHookResult> {
        use mago_analyzer::plugin::ExpressionHookResult;
        Ok(self
            .0
            .get(&expression.span())
            .map_or(ExpressionHookResult::Continue, |value| {
                ExpressionHookResult::SkipWithType(value.clone())
            }))
    }
}

/// Isolated arithmetic islands need their lexical self/parent scope, not a synthetic global scope.
struct InitializerScope {
    class: mago_word::Word,
    canonical: bool,
    path_constants: bool,
    path: Option<PathBuf>,
}

impl mago_analyzer::plugin::ExpressionHook for InitializerScope {
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut mago_analyzer::plugin::HookContext<'_, '_>,
    ) -> mago_analyzer::plugin::HookResult<mago_analyzer::plugin::ExpressionHookResult> {
        let class = context.codebase().get_class_like_by_name(self.class);
        context.block_context_mut().scope.set_class_like(class);
        if self.canonical
            && let Expression::MagicConstant(magic) = expression
        {
            use mago_codex::ttype::get_literal_string;
            let value = match magic {
                cst::MagicConstant::Namespace(_) => {
                    let name = class.expect("initializer class exists").original_name;
                    get_literal_string(mago_word::word(
                        name.as_bytes()
                            .rsplitn(2, |b| *b == b'\\')
                            .nth(1)
                            .unwrap_or(b""),
                    ))
                }
                cst::MagicConstant::File(_) | cst::MagicConstant::Directory(_) => {
                    let path = self.path.as_deref().unwrap_or_else(|| Path::new(""));
                    let path = if matches!(magic, cst::MagicConstant::Directory(_)) {
                        path.parent().unwrap_or(path)
                    } else {
                        path
                    };
                    let string = mago_codex::ttype::atomic::scalar::string::TString::known_literal(
                        mago_word::word(path.to_string_lossy().as_bytes()),
                    );
                    TUnion::from_atomic(TAtomic::Scalar(TScalar::String(if self.path_constants {
                        string
                    } else {
                        string.with_unspecified_literal()
                    })))
                }
                _ => return Ok(mago_analyzer::plugin::ExpressionHookResult::Continue),
            };
            return Ok(mago_analyzer::plugin::ExpressionHookResult::SkipWithType(
                value,
            ));
        }
        Ok(mago_analyzer::plugin::ExpressionHookResult::Continue)
    }
}
