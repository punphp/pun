use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};

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
use mago_codex::ttype::get_false;
use mago_codex::ttype::get_int;
use mago_codex::ttype::get_int_or_float;
use mago_codex::ttype::get_int_range;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::get_named_object;
use mago_codex::ttype::get_non_empty_string;
use mago_codex::ttype::get_null;
use mago_codex::ttype::get_object;
use mago_codex::ttype::get_string;
use mago_codex::ttype::get_truthy_string;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{File, FileId, FileType};
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_names::scope::NamespaceScope;
use mago_php_version::PHPVersion;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::{self, ConstantAccess, Expression, Program};
use mago_syntax::parser::parse_file_content_with_settings;
use mago_syntax::settings::{LexerSettings, ParserSettings};
use mago_syntax::walker::MutWalker;
use rayon::prelude::*;

use crate::bitset::BitSet;
use crate::rules::Facts;
use crate::{Project, autoload};

/// PHPStan's unprefixed classes and functions (scripts/export-phpstan-api-stub.php): PHPStan always has them loaded.
pub(crate) const PHPSTAN_API: &[u8] = pun_phpstan::phpstan_api::PHPSTAN_API;

fn names_phpstan(code: &[u8]) -> bool {
    code.windows(8)
        .any(|w| w.eq_ignore_ascii_case(b"phpstan\\") || w.eq_ignore_ascii_case(b"phpparse"))
}

pub(crate) fn php_version(id: u32) -> PHPVersion {
    pun_tokens::PhpVersion::from_id(id).to_mago()
}

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
    Ok(Some(from_contents(path, file_type, Cow::Owned(contents))))
}

fn from_contents(path: &Path, file_type: FileType, contents: Cow<'static, [u8]>) -> File {
    let name = path.to_string_lossy().into_owned().into_bytes();
    File::new(
        Cow::Owned(name),
        file_type,
        Some(path.to_path_buf()),
        contents,
    )
}

pub(crate) fn parse<'arena>(
    arena: &'arena LocalArena,
    file: &File,
) -> (&'arena Program<'arena>, ResolvedNames<'arena>) {
    parse_contents(arena, file.id, &file.contents)
}

fn parse_contents<'arena>(
    arena: &'arena LocalArena,
    id: FileId,
    contents: &[u8],
) -> (&'arena Program<'arena>, ResolvedNames<'arena>) {
    let program = parse_file_content_with_settings(
        arena,
        id,
        contents,
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

fn scan(
    arena: &mut LocalArena,
    file: &File,
    version: PHPVersion,
    reachable: Option<(&autoload::Index, &mut Vec<usize>, &mut Vec<usize>)>,
    trusted_aliases: bool,
) -> (CodebaseMetadata, Facts) {
    let (mut scanned, facts) = {
        let arena = &*arena;
        let (program, names) = parse(arena, file);
        let facts = Facts::of(&file.contents, program, file.file_type == FileType::Host);
        let scanned = if !crate::parser::errors(file, program, version).is_empty() {
            CodebaseMetadata::default()
        } else {
            if let Some((index, wanted, declarations)) = reachable {
                index.wanted(&file.contents, program, &names, wanted, declarations);
            }
            crate::mago::contained("scanner", file, || {
                scan_program(arena, file, program, &names, version)
            })
            .unwrap_or_default()
        };
        (scanned, facts)
    };
    arena.reset();
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
    (scanned, facts)
}

static FIRST: AtomicBool = AtomicBool::new(true);
static PHP84: LazyLock<(CodebaseMetadata, SymbolReferences)> =
    LazyLock::new(|| build_prelude(80400));
static PHP85: LazyLock<(CodebaseMetadata, SymbolReferences)> =
    LazyLock::new(|| build_prelude(80500));

fn prelude(php_version: u32) -> (CodebaseMetadata, SymbolReferences) {
    if FIRST.swap(false, Ordering::Relaxed) {
        build_prelude(php_version)
    } else if php_version >= 80500 {
        PHP85.clone()
    } else {
        PHP84.clone()
    }
}

/// Mago's prelude, encoded by build.rs.
static PRELUDE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/prelude.bin"));

fn build_prelude(php_version: u32) -> (CodebaseMetadata, SymbolReferences) {
    let (mut metadata, symbol_references): (CodebaseMetadata, SymbolReferences) =
        bincode::serde::decode_from_slice(PRELUDE, bincode::config::standard())
            .expect("build.rs encoded the prelude")
            .0;
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
    for ((class, name), method) in &mut metadata.function_likes {
        let name = String::from_utf8_lossy(name.as_bytes()).to_ascii_lowercase();
        let effects = if class.is_empty() {
            builtins
                .functions
                .get(&name)
                .map(|function| &function.side_effects)
        } else {
            builtins
                .classes
                .get(&String::from_utf8_lossy(class.as_bytes()).to_ascii_lowercase())
                .and_then(|class| class.methods.get(&name))
                .map(|method| &method.side_effects)
        };
        if let Some(effects) = effects {
            method.reflected_side_effects = Some(match effects.as_str() {
                "Yes" => ReflectedSideEffects::Yes,
                "No" => ReflectedSideEffects::No,
                _ => ReflectedSideEffects::Maybe,
            });
        }
    }
    for ((class, name), function) in &mut metadata.function_likes {
        if !class.is_empty() {
            continue;
        }
        let Some([variant]) = builtins
            .functions
            .get(&String::from_utf8_lossy(name.as_bytes()).to_ascii_lowercase())
            .map(|function| function.variants.as_slice())
        else {
            continue;
        };
        let Some(ret) = function
            .return_type_metadata
            .as_ref()
            .or(function.return_type_declaration_metadata.as_ref())
        else {
            continue;
        };
        let bound = |b: &str| match b {
            "min" | "max" => Some(None),
            b => b.parse::<i64>().ok().map(Some),
        };
        let mut narrowed = false;
        let reflected: Option<Vec<TAtomic>> = variant
            .ret
            .strip_prefix('(')
            .and_then(|r| r.strip_suffix(')'))
            .unwrap_or(&variant.ret)
            .split(" | ")
            .map(|part| {
                let (general, union) = match part {
                    "null" => (get_null(), get_null()),
                    "false" => (get_false(), get_false()),
                    "non-empty-string" => (get_string(), get_non_empty_string()),
                    "non-falsy-string" => (get_string(), get_truthy_string()),
                    part => {
                        let (from, to) = part
                            .strip_prefix("int<")?
                            .strip_suffix('>')?
                            .split_once(", ")?;
                        (get_int(), get_int_range(bound(from)?, bound(to)?))
                    }
                };
                narrowed |= general != union;
                ret.type_union
                    .types
                    .contains(general.types.first()?)
                    .then(|| union.types.first().cloned())?
            })
            .collect();
        let Some(reflected) = reflected else { continue };
        if narrowed && reflected.len() == ret.type_union.types.len() {
            let span = ret.span;
            function.return_type_metadata =
                Some(TypeMetadata::new(TUnion::from_vec(reflected), span));
        }
    }
    for ((class, name), method) in &mut metadata.function_likes {
        let Some([variant]) = builtins
            .classes
            .get(&String::from_utf8_lossy(class.as_bytes()).to_ascii_lowercase())
            .and_then(|class| {
                class
                    .methods
                    .get(&String::from_utf8_lossy(name.as_bytes()).to_ascii_lowercase())
            })
            .map(|method| method.variants.as_slice())
        else {
            continue;
        };
        let Some(ret) = method
            .return_type_metadata
            .as_ref()
            .or(method.return_type_declaration_metadata.as_ref())
        else {
            continue;
        };
        let plain = |a: &TAtomic| match a {
            TAtomic::Null => true,
            TAtomic::Object(mago_codex::ttype::atomic::object::TObject::Named(n)) => {
                n.type_parameters.is_none() && !n.is_static && n.intersection_types.is_none()
            }
            _ => false,
        };
        if !ret.type_union.types.iter().all(plain) {
            continue;
        }
        let is_name = |p: &str| {
            !p.is_empty()
                && p.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'\\')
        };
        let reflected: Option<Vec<TAtomic>> = variant
            .ret
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(" | ")
            .map(|part| match part {
                "null" => Some(TAtomic::Null),
                p if is_name(p) => get_named_object(mago_word::word(p), None)
                    .types
                    .first()
                    .cloned(),
                p => {
                    let (class, argument) = p.strip_suffix('>')?.split_once('<')?;
                    if !is_name(class) || !is_name(argument) {
                        return None;
                    }
                    let argument = get_named_object(mago_word::word(argument), None);
                    Some(TAtomic::Object(
                        mago_codex::ttype::atomic::object::TObject::Named(
                            mago_codex::ttype::atomic::object::named::TNamedObject::new_with_type_parameters(
                                mago_word::word(class),
                                Some(vec![argument]),
                            ),
                        ),
                    ))
                }
            })
            .collect();
        let Some(reflected) = reflected else { continue };
        let lower = |types: &[TAtomic]| {
            let mut names: Vec<String> = types
                .iter()
                .map(|a| {
                    String::from_utf8_lossy(mago_codex::ttype::TType::get_id(a).as_bytes())
                        .to_ascii_lowercase()
                })
                .collect();
            names.sort();
            names
        };
        if lower(&reflected) != lower(&ret.type_union.types) {
            let span = ret.span;
            method.return_type_metadata =
                Some(TypeMetadata::new(TUnion::from_vec(reflected), span));
        }
    }
    for ((class, name), method) in &mut metadata.function_likes {
        let Some([variant]) = builtins
            .classes
            .get(&String::from_utf8_lossy(class.as_bytes()).to_ascii_lowercase())
            .and_then(|class| {
                class
                    .methods
                    .get(&String::from_utf8_lossy(name.as_bytes()).to_ascii_lowercase())
            })
            .map(|method| method.variants.as_slice())
        else {
            continue;
        };
        if method.return_type_metadata.is_none() && variant.ret.starts_with("__benevolent<") {
            method
                .return_type_metadata
                .clone_from(&method.return_type_declaration_metadata);
        }
        let Some(ret) = method.return_type_metadata.as_mut() else {
            continue;
        };
        if variant.ret.starts_with("__template<") && !variant.ret.contains(" | ") {
            if let [TAtomic::Null, template @ TAtomic::GenericParameter(_)]
            | [template @ TAtomic::GenericParameter(_), TAtomic::Null] =
                ret.type_union.types.as_ref()
            {
                ret.type_union = TUnion::from_vec(vec![template.clone()]);
            }
        } else if variant.ret.starts_with("__benevolent<") {
            ret.type_union.set_ignore_nullable_issues(true);
            ret.type_union.set_ignore_falsable_issues(true);
        }
    }
    for ((scope, name), function) in &mut metadata.function_likes {
        if !scope.is_empty() || !function.flags.is_built_in() {
            continue;
        }
        let Some(declared) = function
            .return_type_metadata
            .as_ref()
            .or(function.return_type_declaration_metadata.as_ref())
        else {
            continue;
        };
        let [TAtomic::Array(mago_codex::ttype::atomic::array::TArray::Keyed(array))] =
            declared.type_union.types.as_ref()
        else {
            continue;
        };
        if array.known_items.is_some()
            || array.non_empty
            || !array
                .parameters
                .as_ref()
                .is_some_and(|(k, v)| k.is_array_key() && v.is_mixed())
        {
            continue;
        }
        let Some([variant]) = builtins
            .functions
            .get(&String::from_utf8_lossy(name.as_bytes()).to_ascii_lowercase())
            .map(|function| function.variants.as_slice())
        else {
            continue;
        };
        if let Some(reflected) = crate::larastan::config::type_from_text(variant.ret.as_bytes())
            .filter(|t| t.types.iter().all(TAtomic::is_array))
        {
            let span = declared.span;
            function.return_type_metadata = Some(TypeMetadata::new(reflected, span));
        }
    }
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

/// The declarations of [`PHPSTAN_API`], one per `namespace` block, with the lowercase names of the class-likes each
/// declares. A block that declares a function has no class name and is loaded with the API, as PHPStan's bootstrap
/// loads its functions; the class-likes are loaded as PHPStan's autoloader loads them, when a name reaches them.
fn api_blocks() -> Vec<(std::ops::Range<usize>, Vec<Vec<u8>>, bool)> {
    let mut starts: Vec<usize> = memchr::memmem::find_iter(PHPSTAN_API, b"\nnamespace ")
        .map(|at| at + 1)
        .collect();
    starts.push(PHPSTAN_API.len());
    (starts.windows(2))
        .filter_map(|w| {
            let block = &PHPSTAN_API[w[0]..w[1]];
            let namespace = block[b"namespace ".len()..].split(|&b| b == b';').next()?;
            let mut classes = Vec::new();
            let mut function = false;
            for line in block.split(|&b| b == b'\n') {
                let mut words = line
                    .split(|&b| b == b' ')
                    .skip_while(|w| matches!(*w, b"final" | b"abstract" | b"readonly"));
                match words.next() {
                    Some(b"class" | b"interface" | b"trait" | b"enum") => {
                        let name = words.next()?;
                        let name = name.split(|&b| b == b':').next()?;
                        let mut class = namespace.to_ascii_lowercase();
                        class.push(b'\\');
                        class.extend(name.iter().map(u8::to_ascii_lowercase));
                        classes.push(class);
                    }
                    Some(b"function") => function = true,
                    _ => {}
                }
            }
            (function || !classes.is_empty()).then_some((w[0]..w[1], classes, function))
        })
        .collect()
}

const CHUNK: usize = 32;

pub(crate) struct Scanned {
    pub(crate) file: Arc<File>,
    pub(crate) facts: Facts,
}

#[derive(Default)]
pub(crate) struct Sources {
    files: HashMap<PathBuf, Arc<File>>,
    scanned: HashMap<FileId, Scanned>,
    api: HashMap<FileId, Scanned>,
}

impl Sources {
    pub(crate) fn scanned(&self, id: FileId) -> Option<&Scanned> {
        self.scanned.get(&id).or_else(|| self.api.get(&id))
    }
}

impl std::ops::Deref for Sources {
    type Target = HashMap<PathBuf, Arc<File>>;

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

impl std::ops::DerefMut for Sources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.files
    }
}

pub(crate) fn build(
    project: &Project,
    laravel_root: Option<&Path>,
) -> Result<(CodebaseMetadata, Sources), CouldNotReadFile> {
    let _ = mago_codex::ttype::expander::COLLECTION_OF_RESULT.set(crate::laravel::collection_of);
    let _ = mago_codex::ttype::expander::BUILDER_OF_RESULT.set(crate::rules::late_builder);
    let version = project.php_version.to_mago();
    let aliases = match laravel_root {
        Some(root) => crate::larastan::aliases::load(root),
        None => (project.extensions())
            .flat_map(|e| e.class_aliases())
            .collect(),
    };
    let mut seen = HashSet::new();
    let mut files: Vec<_> = project
        .analysed
        .iter()
        .map(|p| (p, FileType::Host))
        .chain(project.scanned.iter().map(|p| (p, FileType::Vendored)))
        .chain(project.stubs.iter().map(|p| (p, FileType::Patch)))
        .filter(|(p, _)| seen.insert(*p))
        .collect();
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
    // PHPStan's API blocks come last. Their names reach only other blocks: before, the whole API was scanned
    // without following its names into the vendor directory.
    let blocks = api_blocks();
    let api_start = files.len();
    let api_paths: Vec<PathBuf> = (0..blocks.len())
        .map(|i| PathBuf::from(format!("phpstan-api-{i}.php")))
        .collect();
    files.extend(api_paths.iter().map(|p| (p, FileType::Vendored)));
    let api_classes: Vec<_> = (blocks.iter().enumerate())
        .flat_map(|(i, (_, classes, _))| {
            classes.iter().map(move |class| {
                (
                    api_start + i,
                    crate::project::Autoloaded {
                        path: PathBuf::new(),
                        class: class.clone(),
                        psr0: false,
                    },
                )
            })
        })
        .collect();
    let api_index = autoload::Index::new(api_classes.iter().map(|(i, a)| (*i, a)), &[]);
    let index = autoload::Index::new(
        classes
            .into_iter()
            .chain(api_classes.iter().map(|(i, a)| (*i, a))),
        &aliases,
    );
    let php_version = project.php_version.id();
    let builtins = move || {
        crate::rules::builtins(php_version);
        pun_phpstan::symbols::symbols(pun_phpstan::symbols::PhpVersion::new(8, 4))
    };
    let names_phpstan = AtomicBool::new(false);
    let (((mut metadata, mut references), _), scanned) = rayon::join(
        || rayon::join(move || prelude(php_version), builtins),
        || {
            let mut loaded = BitSet::new(files.len());
            for i in 0..eager {
                loaded.set(i, true);
            }
            let mut round: Vec<usize> = (0..eager).collect();
            let mut scanned = Vec::new();
            let mut api = Vec::new();
            let mut sources = Sources::default();
            let mut used_aliases = Vec::new();
            let mut first = true;
            while !round.is_empty() {
                let size = (round.len() / (rayon::current_num_threads() * 4)).clamp(1, CHUNK);
                let chunks = round
                    .par_chunks(size)
                    .map_init(LocalArena::new, |arena, chunk| {
                        let mut metadata = CodebaseMetadata::default();
                        let mut sources = Vec::new();
                        let mut wanted = Vec::new();
                        let mut declarations = Vec::new();
                        let mut api = Vec::new();
                        for &i in chunk {
                            let (path, file_type) = files[i];
                            if let Some((block, ..)) = i.checked_sub(api_start).map(|b| &blocks[b])
                            {
                                let mut contents = b"<?php\n".to_vec();
                                contents.extend_from_slice(&PHPSTAN_API[block.clone()]);
                                let stub = File::new(
                                    Cow::Owned(path.to_string_lossy().into_owned().into_bytes()),
                                    FileType::Vendored,
                                    None,
                                    Cow::Owned(contents),
                                );
                                let (symbols, facts) = scan(
                                    arena,
                                    &stub,
                                    version,
                                    Some((&api_index, &mut wanted, &mut declarations)),
                                    false,
                                );
                                wanted.retain(|&w| !loaded.get(w));
                                let file = Arc::new(stub);
                                api.push((i, symbols, Scanned { file, facts }));
                                continue;
                            }
                            let file = match read(path, file_type)? {
                                Some(file) => file,
                                None => {
                                    match project.extensions().find_map(|e| e.bundled_stub(path)) {
                                        Some(contents) => {
                                            from_contents(path, file_type, Cow::Borrowed(contents))
                                        }
                                        None => continue,
                                    }
                                }
                            };
                            let (symbols, facts) = scan(
                                arena,
                                &file,
                                version,
                                Some((&index, &mut wanted, &mut declarations)),
                                false,
                            );
                            let has_constants = !symbols.constants.is_empty();
                            metadata.extend(symbols);
                            wanted.retain(|&w| !loaded.get(w));
                            if file_type == FileType::Host && self::names_phpstan(&file.contents) {
                                names_phpstan.store(true, Ordering::Relaxed);
                            }
                            let by_path = file_type == FileType::Host || has_constants;
                            sources.push((
                                path.clone(),
                                Scanned {
                                    file: Arc::new(file),
                                    facts,
                                },
                                by_path,
                            ));
                        }
                        Ok((metadata, sources, wanted, declarations, api))
                    })
                    .collect::<Result<Vec<_>, CouldNotReadFile>>()?;
                round.clear();
                for (chunk, files, wanted, declarations, blocks) in chunks {
                    scanned.push(chunk);
                    for (path, file, by_path) in files {
                        if by_path {
                            sources.files.insert(path, file.file.clone());
                        }
                        sources.scanned.insert(file.file.id, file);
                    }
                    round.extend(wanted);
                    used_aliases.extend(declarations);
                    api.extend(blocks);
                }
                // The analysed files are the first round: the API is loaded only if one of them names PHPStan.
                let with_api = names_phpstan.load(Ordering::Relaxed);
                if std::mem::take(&mut first) && with_api {
                    round.extend(
                        (blocks.iter().enumerate())
                            .filter(|(_, b)| b.2)
                            .map(|(b, _)| api_start + b),
                    );
                }
                if !with_api {
                    round.retain(|&i| i < api_start);
                }
                round.sort_unstable();
                round.dedup();
                for &i in &round {
                    loaded.set(i, true);
                }
            }
            Ok((scanned, sources, used_aliases, api))
        },
    );
    let (scanned, mut sources, mut used_aliases, mut api) = scanned?;
    for chunk in scanned {
        metadata.extend(chunk);
    }
    if !used_aliases.is_empty() {
        used_aliases.sort_unstable();
        used_aliases.dedup();
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
        metadata.extend(scan(&mut LocalArena::new(), &file, version, None, true).0);
    }
    api.sort_unstable_by_key(|(i, ..)| *i);
    for (_, symbols, stub) in api {
        metadata.extend(symbols);
        sources.api.insert(stub.file.id, stub);
    }
    metadata.apply_patches_pass();
    let pseudo: Vec<_> = (metadata.patch_function_likes.iter())
        .filter(|(key, _)| !metadata.function_likes.contains_key(key))
        .filter(|((class, name), _)| {
            (metadata.class_likes.get(class)).is_some_and(|k| {
                k.pseudo_methods.contains(name) || k.static_pseudo_methods.contains(name)
            })
        })
        .map(|(key, f)| (*key, f.clone()))
        .collect();
    metadata.function_likes.extend(pseudo);
    for extension in project.extensions() {
        extension.prepare_codebase(&mut metadata);
    }
    populate_codebase_parallel(
        &mut metadata,
        &mut references,
        Default::default(),
        Default::default(),
    );
    ancestor_stub_docs(&mut metadata);
    resolve_initializers(&mut metadata, &sources, project);
    Ok((metadata, sources))
}

fn ancestor_stub_docs(metadata: &mut CodebaseMetadata) {
    use mago_codex::misc::GenericParent;
    use mago_codex::ttype::template::{TemplateResult, inferred_type_replacer};
    fn visit(metadata: &CodebaseMetadata, name: mago_word::Word, order: &mut Vec<mago_word::Word>) {
        let Some(k) = metadata.class_likes.get(&name) else {
            return;
        };
        for n in k
            .direct_parent_interfaces
            .iter()
            .chain(k.used_traits.iter())
            .chain(k.direct_parent_class.iter())
        {
            let Some(a) = metadata.get_class_like(n.as_bytes()) else {
                continue;
            };
            if order.len() < 256 && !order.contains(&a.name) {
                order.push(a.name);
                visit(metadata, a.name, order);
            }
        }
    }
    let mut updates = Vec::new();
    for class_key in metadata.patch_class_likes.keys() {
        let Some(class) = metadata.class_likes.get(class_key) else {
            continue;
        };
        if class.flags.is_user_defined() {
            continue;
        }
        let mut order = vec![class.name];
        visit(metadata, class.name, &mut order);
        for method in &class.methods {
            if metadata
                .patch_function_likes
                .get(&(class.name, *method))
                .is_some_and(|p| p.has_docblock)
            {
                continue;
            }
            let Some(ancestor) = order.iter().skip(1).find(|a| {
                metadata
                    .patch_function_likes
                    .get(&(**a, *method))
                    .is_some_and(|p| p.has_docblock)
                    && metadata.function_likes.contains_key(&(**a, *method))
            }) else {
                continue;
            };
            let parent = &metadata.function_likes[&(*ancestor, *method)];
            let mut templates = TemplateResult::default();
            for (name, ty) in class
                .template_extended_parameters
                .get(ancestor)
                .into_iter()
                .flatten()
            {
                templates.add_lower_bound(*name, GenericParent::ClassLike(*ancestor), ty.clone());
            }
            let substitute = |t: &mago_codex::metadata::ttype::TypeMetadata| {
                let mut t = t.clone();
                t.type_union = inferred_type_replacer::replace(&t.type_union, &templates, metadata);
                t
            };
            let returned = parent
                .return_type_metadata
                .as_ref()
                .filter(|t| t.from_docblock)
                .map(substitute);
            let params: Vec<_> = parent
                .parameters
                .iter()
                .map(|p| {
                    p.type_metadata
                        .as_ref()
                        .filter(|t| t.from_docblock)
                        .map(substitute)
                })
                .collect();
            updates.push((
                (class.name, *method),
                returned,
                params,
                parent.template_types.clone(),
            ));
        }
    }
    for (key, returned, params, templates) in updates {
        let Some(f) = metadata.function_likes.get_mut(&key) else {
            continue;
        };
        if returned.is_some() {
            f.return_type_metadata = returned;
        }
        for (p, ty) in f.parameters.iter_mut().zip(params) {
            if ty.is_some() {
                p.type_metadata = ty;
            }
        }
        if !templates.is_empty() {
            f.template_types = templates;
        }
    }
}

fn resolve_initializers(metadata: &mut CodebaseMetadata, sources: &Sources, project: &Project) {
    let version = project.php_version.to_mago();
    let loaded: &CodebaseMetadata = metadata;
    let mut roots: Vec<Initializer> = project
        .analysed
        .par_iter()
        .map_init(LocalArena::new, |arena, path| {
            let Some(file) = sources.get(path) else {
                return Vec::new();
            };
            let found = {
                let (program, names) = parse(arena, file);
                if crate::parser::errors(file, program, version).is_empty() {
                    let mut inputs = ConstantInputs {
                        metadata: loaded,
                        names: &names,
                        names_found: Vec::new(),
                        class: None,
                    };
                    inputs.walk_program(program, &mut ());
                    inputs.names_found
                } else {
                    Vec::new()
                }
            };
            arena.reset();
            found
        })
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect();
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
    let initializer_sources: HashMap<_, _> = sources
        .scanned
        .values()
        .map(|scanned| (scanned.file.id, scanned.file.as_ref()))
        .collect();
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
    // Every file an initializer can be evaluated in, parsed up front in parallel: the resolver below is serial
    // and would otherwise parse them one at a time. A parse depends only on the file and the initializers in it, so
    // the functions, methods and doc comments that hold none are blanked first: they are most of a file and are
    // never read.
    let mut wanted: HashMap<FileId, HashSet<Span>> = HashMap::new();
    for at in (metadata.class_likes.values())
        .flat_map(|class| class.constants.values().map(|constant| constant.span))
        .chain(metadata.constants.values().map(|constant| constant.span))
        .chain(trait_sources.iter().map(|(_, at)| *at))
        .chain(arguments.iter().map(|(.., at)| *at))
        .filter(|at| initializer_sources.contains_key(&at.file_id))
    {
        wanted.entry(at.file_id).or_default().insert(at);
    }
    let wanted: Vec<(FileId, HashSet<Span>)> = wanted.into_iter().collect();
    let mut arenas: Vec<LocalArena> = wanted.iter().map(|_| LocalArena::new()).collect();
    let parsed: HashMap<FileId, ParsedSource<'_>> = arenas
        .par_iter_mut()
        .zip(&wanted)
        .map(|(arena, (id, spans))| {
            let arena: &LocalArena = arena;
            let file = initializer_sources[id];
            let mut contents = file.contents.to_vec();
            for &(start, end) in
                (sources.scanned(*id).map(|s| &s.facts.blankable[..])).unwrap_or_default()
            {
                if spans
                    .iter()
                    .all(|at| at.end.offset <= start || end <= at.start.offset)
                {
                    for b in &mut contents[start as usize..end as usize] {
                        if !matches!(b, b'\n' | b'\r') {
                            *b = b' ';
                        }
                    }
                }
            }
            let (program, names) = parse_contents(arena, *id, &contents);
            let mut index = InitializerIndex::default();
            index.walk_program(program, &mut ());
            index.at.retain(|at, _| spans.contains(at));
            index.at.shrink_to_fit();
            (*id, (program, names, index))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(id, (program, names, index))| (id, (program, Rc::new(names), Rc::new(index))))
        .collect();
    let arena = LocalArena::new();
    let mut resolver = Initializers {
        metadata,
        sources: initializer_sources,
        arena: &arena,
        parsed,
        version: project.php_version.to_mago(),
        visited: HashSet::new(),
        requested: HashSet::new(),
        copies,
        classes,
        path_constants: project
            .options
            .contains(crate::ProjectOptions::USE_PATH_CONSTANTS_AS_CONSTANT_STRING),
    };
    for name in roots {
        resolver.resolve(name);
    }
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
    fn walk_in_constant_access(&mut self, access: &'ast ConstantAccess<'arena>, (): &mut ()) {
        let selected = selected_global(self.metadata, self.names, access);
        if let Some(selected) = selected.filter(|k| !k.flags.is_built_in()) {
            self.names_found.push(Initializer::Global(selected.name));
        }
    }

    fn walk_in_class_constant_access(
        &mut self,
        access: &'ast cst::ClassConstantAccess<'arena>,
        (): &mut (),
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
    arena: &'a LocalArena,
    parsed: HashMap<FileId, ParsedSource<'a>>,
    version: PHPVersion,
    visited: HashSet<(Span, Option<mago_word::Word>)>,
    requested: HashSet<(mago_word::Word, mago_word::Word)>,
    copies: HashMap<(Span, mago_word::Word), Vec<mago_word::Word>>,
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
        if declaration
            .inferred_type
            .as_ref()
            .is_some_and(TUnion::is_never)
        {
            return;
        }
        let previous = declaration.inferred_type.take();
        let value = self.evaluate(at, false, Some(owner)).or(previous);
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
        let previous = declaration.inferred_type.take();
        let declared = declaration.type_metadata.take();
        let mut value = self.evaluate(at, false, None);
        if let Some(value) = value.as_mut()
            && value.is_mixed()
            && let Some(previous) = previous.as_ref()
            && previous == &get_int_or_float()
        {
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
        let (program, names, index) = self
            .parsed
            .entry(at.file_id)
            .or_insert_with(|| {
                let (program, names) = parse(arena, file);
                let mut index = InitializerIndex::default();
                index.walk_program(program, &mut ());
                (program, Rc::new(names), Rc::new(index))
            })
            .clone();
        let (expression, namespace) = index.find(at, require_global);
        let expression = expression?;
        let mut inputs = ConstantInputs {
            metadata: self.metadata,
            names: &names,
            names_found: Vec::new(),
            class,
        };
        inputs.walk_expression(expression, &mut ());
        struct Holders<'ast, 'arena>(Vec<(&'ast Expression<'arena>, mago_word::Word)>);
        impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Holders<'ast, 'arena> {
            fn walk_out_class_constant_access(
                &mut self,
                access: &'ast cst::ClassConstantAccess<'arena>,
                (): &mut (),
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
                arena,
                file,
                program,
                &names,
                expression,
                namespace.clone(),
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
        let mut value = self
            .inference(class)
            .initializer_type(arena, file, program, &names, expression, namespace);
        if inherited_benevolence && value == get_int_or_float() {
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

type ParsedSource<'a> = (
    &'a Program<'a>,
    Rc<ResolvedNames<'a>>,
    Rc<InitializerIndex<'a, 'a>>,
);

#[derive(Default)]
struct InitializerIndex<'ast, 'arena> {
    at: HashMap<Span, Vec<Candidate<'ast, 'arena>>>,
    namespaces: Vec<&'ast cst::Namespace<'arena>>,
}

#[derive(Clone, Copy)]
enum Candidate<'ast, 'arena> {
    Value(&'ast Expression<'arena>),
    Call(Option<&'ast Expression<'arena>>),
}

impl<'ast, 'arena> InitializerIndex<'ast, 'arena> {
    fn find(
        &self,
        at: Span,
        argument: bool,
    ) -> (
        Option<&'ast Expression<'arena>>,
        Option<cst::Namespace<'arena>>,
    ) {
        let mut expression = None;
        for candidate in self.at.get(&at).into_iter().flatten() {
            match *candidate {
                Candidate::Value(e) => expression = Some(e),
                Candidate::Call(value) if !argument => expression = value,
                Candidate::Call(_) => {}
            }
        }
        let namespace = (self.namespaces.iter().rev())
            .find(|n| {
                n.span().start.offset <= at.start.offset && at.end.offset <= n.span().end.offset
            })
            .map(|n| (*n).clone());
        (expression, namespace)
    }

    fn push(&mut self, at: Span, candidate: Candidate<'ast, 'arena>) {
        self.at.entry(at).or_default().push(candidate);
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for InitializerIndex<'ast, 'arena> {
    fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, (): &mut ()) {
        self.push(expression.span(), Candidate::Value(expression));
    }
    fn walk_in_namespace(&mut self, namespace: &'ast cst::Namespace<'arena>, (): &mut ()) {
        self.namespaces.push(namespace);
    }
    fn walk_in_constant_item(&mut self, item: &'ast cst::ConstantItem<'arena>, (): &mut ()) {
        self.push(item.span(), Candidate::Value(item.value));
    }
    fn walk_in_class_like_constant_item(
        &mut self,
        item: &'ast cst::ClassLikeConstantItem<'arena>,
        (): &mut (),
    ) {
        self.push(item.span(), Candidate::Value(item.value));
    }
    fn walk_in_function_call(&mut self, call: &'ast cst::FunctionCall<'arena>, (): &mut ()) {
        let value = call.argument_list.arguments.get(1).map(|arg| arg.value());
        self.push(call.span(), Candidate::Call(value));
    }
}

struct FindInitializer<'ast, 'arena> {
    at: Span,
    argument: bool,
    expression: Option<&'ast Expression<'arena>>,
    namespace: Option<cst::Namespace<'arena>>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for FindInitializer<'ast, 'arena> {
    fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, (): &mut ()) {
        if expression.span() == self.at {
            self.expression = Some(expression);
        }
    }
    fn walk_in_namespace(&mut self, namespace: &'ast cst::Namespace<'arena>, (): &mut ()) {
        if namespace.span().start.offset <= self.at.start.offset
            && self.at.end.offset <= namespace.span().end.offset
        {
            self.namespace = Some(namespace.clone());
        }
    }
    fn walk_in_constant_item(&mut self, item: &'ast cst::ConstantItem<'arena>, (): &mut ()) {
        if item.span() == self.at {
            self.expression = Some(item.value);
        }
    }
    fn walk_in_class_like_constant_item(
        &mut self,
        item: &'ast cst::ClassLikeConstantItem<'arena>,
        (): &mut (),
    ) {
        if item.span() == self.at {
            self.expression = Some(item.value);
        }
    }
    fn walk_in_function_call(&mut self, call: &'ast cst::FunctionCall<'arena>, (): &mut ()) {
        if !self.argument && call.span() == self.at {
            self.expression = call.argument_list.arguments.get(1).map(|arg| arg.value());
        }
    }
}

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
            self.at.start.offset <= at.start.offset
                && at.end.offset <= self.at.end.offset
                && initializer_class(self.metadata, at).is_some_and(|k| k.span == self.at)
        }
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Inputs<'_> {
        fn walk_in_class_like_constant_item(
            &mut self,
            item: &'ast cst::ClassLikeConstantItem<'arena>,
            (): &mut (),
        ) {
            if self.owns(item.span()) {
                self.spans.push(item.value.span());
            }
        }
        fn walk_in_attribute(&mut self, attribute: &'ast cst::Attribute<'arena>, (): &mut ()) {
            if self.owns(attribute.span())
                && let Some(arguments) = &attribute.argument_list
            {
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
        struct Islands<'ast, 'arena> {
            stack: Vec<(Span, bool)>,
            expressions: Vec<(&'ast Expression<'arena>, bool, Option<Span>)>,
        }
        impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Islands<'ast, 'arena> {
            fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, (): &mut ()) {
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
            fn walk_out_expression(&mut self, expression: &'ast Expression<'arena>, (): &mut ()) {
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
                    !owned
                        && parent.is_none_or(|parent| scanner_owned.contains(&parent))
                        && !matches!(
                            expression,
                            Expression::Identifier(_)
                                | Expression::Self_(_)
                                | Expression::Parent(_)
                                | Expression::Static(_)
                        )
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
        for (expression, _, _) in islands
            .expressions
            .into_iter()
            .filter(|(_, owned, _)| *owned)
        {
            let mut value = match expression {
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
                        None => {
                            get_literal_constant_type(access.name.value()).unwrap_or_else(get_error)
                        }
                    }
                }
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
        let statements = cst::Sequence::from_slice(arena.alloc_slice_fill_iter([statement]));
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
            statements: cst::Sequence::from_slice(arena.alloc_slice_fill_iter([statement])),
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
        crate::mago::contained("analyser of a constant initializer", file, || {
            analyzer.analyze_with_artifacts(isolated, &mut result)
        })?
        .ok()
    }
}

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
