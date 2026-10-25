//! The symbols every rule queries: Mago's prelude (built-ins) extended with every analysed and scanned file.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, OnceLock};

use mago_allocator::{Arena, LocalArena};
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::PluginRegistry;
use mago_analyzer::settings::Settings;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::constant::ConstantMetadata;
use mago_codex::metadata::flags::MetadataFlags;
use mago_codex::metadata::ttype::TypeMetadata;
use mago_codex::populator::populate_codebase_parallel;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::get_int_or_float;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{File, FileId, FileType};
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_prelude::Prelude;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::{self, ConstantAccess, Expression, Hint, IntersectionHint, Program};
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
    scan(&mut LocalArena::new(), &stub, version, None)
}

/// Files scanned by one task: small enough that the threads finish together.
const CHUNK: usize = 32;

/// Analysed files and loaded global-constant declaration files, as the scan read them.
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
                            let symbols = scan(
                                arena,
                                &file,
                                version,
                                Some((&index, &mut wanted, &mut declarations)),
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
    resolve_initializers(&mut metadata, &sources, project);
    Ok((metadata, sources))
}

/// Resolve only global constants referenced by analysed code and their initializer dependencies. Scanning an
/// autoload file remains lazy, and evaluating one initializer never walks that file's unrelated declarations.
fn resolve_initializers(metadata: &mut CodebaseMetadata, sources: &Sources, project: &Project) {
    let mut roots = Vec::new();
    let mut arena = LocalArena::new();
    for path in &project.analysed {
        if let Some(file) = sources.get(path) {
            let (program, names) = parse(&arena, file);
            let mut inputs = ConstantInputs {
                metadata,
                names: &names,
                names_found: Vec::new(),
            };
            inputs.walk_program(program, &mut ());
            roots.extend(inputs.names_found);
            arena.reset();
        }
    }
    let mut resolver = Initializers {
        metadata,
        sources: sources
            .values()
            .map(|file| (file.id, file.as_ref()))
            .collect(),
        version: php_version(project.php_version),
        visited: HashSet::new(),
    };
    for name in roots {
        resolver.resolve(name);
    }
}

struct ConstantInputs<'a, 'arena> {
    metadata: &'a CodebaseMetadata,
    names: &'a ResolvedNames<'arena>,
    names_found: Vec<mago_word::Word>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for ConstantInputs<'_, 'arena> {
    fn walk_in_constant_access(&mut self, access: &'ast ConstantAccess<'arena>, _: &mut ()) {
        // The same selection as ConstantAccess::analyze: existence stops fallback, and imported or
        // qualified names can never accidentally select an unrelated global constant.
        let selected = self
            .metadata
            .get_constant(self.names.get(access))
            .or_else(|| {
                (!access.name.value().contains(&b'\\') && !self.names.is_imported(&access.name))
                    .then(|| self.metadata.get_constant(access.name.value()))
                    .flatten()
            });
        if let Some(selected) = selected.filter(|k| !k.flags.is_built_in()) {
            self.names_found.push(selected.name);
        }
    }
}

struct Initializers<'a> {
    metadata: &'a mut CodebaseMetadata,
    sources: HashMap<FileId, &'a File>,
    version: PHPVersion,
    visited: HashSet<Span>,
}

impl Initializers<'_> {
    fn resolve(&mut self, name: mago_word::Word) {
        let declaration = self
            .metadata
            .constants
            .get_mut(&name)
            .expect("selected constant exists");
        let at = declaration.span;
        if !self.visited.insert(at) {
            return;
        }
        // ConstantResolver's currentlyResolving entry is mixed, not a copied scanner approximation.
        // A global constant reflection uses its value, not its PHPDoc @var annotation.
        let previous = declaration.inferred_type.take();
        let declared = declaration.type_metadata.take();
        let mut value = self.evaluate(at);
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

    fn evaluate(&mut self, at: Span) -> Option<TUnion> {
        struct Find<'ast, 'arena> {
            at: Span,
            expression: Option<&'ast Expression<'arena>>,
            namespace: Option<cst::Namespace<'arena>>,
        }
        impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Find<'ast, 'arena> {
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
            fn walk_in_function_call(&mut self, call: &'ast cst::FunctionCall<'arena>, _: &mut ()) {
                // A scanned define() has the invocation span rather than a ConstantItem span.
                if call.span() == self.at {
                    self.expression = call.argument_list.arguments.get(1).map(|arg| arg.value());
                }
            }
        }
        let file = *self.sources.get(&at.file_id)?;
        let arena = LocalArena::new();
        let (program, names) = parse(&arena, file);
        let mut find = Find {
            at,
            expression: None,
            namespace: None,
        };
        find.walk_program(program, &mut ());
        let expression = find.expression?;
        let mut inputs = ConstantInputs {
            metadata: self.metadata,
            names: &names,
            names_found: Vec::new(),
        };
        inputs.walk_expression(expression, &mut ());
        let mut inherited_benevolence = false;
        for name in inputs.names_found {
            self.resolve(name);
            if let Some(value) = self
                .metadata
                .get_constant(name.as_bytes())
                .and_then(|k| k.type_metadata.as_ref())
            {
                inherited_benevolence |= value.type_union.ignore_nullable_issues()
                    && value.type_union.ignore_falsable_issues();
            }
        }
        let statement = cst::Statement::Expression(cst::ExpressionStatement {
            expression,
            terminator: cst::Terminator::Semicolon(expression.span()),
        });
        let statements =
            cst::Sequence::from_slice(arena.alloc_slice_fill_iter([statement].into_iter()));
        let statement = if let Some(mut namespace) = find.namespace {
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
        let registry = PluginRegistry::with_library_providers();
        let analyzer = Analyzer::new(
            &arena,
            file,
            &names,
            self.metadata,
            &registry,
            Settings::new(self.version),
        );
        let mut result = AnalysisResult::new(SymbolReferences::new());
        let artifacts = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            analyzer.analyze_with_artifacts(isolated, &mut result)
        }))
        .ok()?
        .ok()?;
        let mut value = artifacts.get_expression_type(expression)?.clone();
        if inherited_benevolence && value == get_int_or_float() {
            // Keep the numeric union's existing marker across the full evaluator's arithmetic join.
            value.set_ignore_nullable_issues(true);
            value.set_ignore_falsable_issues(true);
        }
        Some(value)
    }
}
