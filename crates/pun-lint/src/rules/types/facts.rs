//! What Rector reads from PHPStan, for the rules that need more than the syntax tree: the classes, methods and
//! functions a name stands for, and the native type of an expression (`NodeTypeResolver::getNativeType`). Pun takes
//! them from Mago: its codebase (the prelude of built-in symbols with the project's symbols) and its analyser's type of
//! each expression of the file. Docblocks are blanked out before Mago reads a file, so that both see only native types,
//! as PHPStan's native reflection and native scope do.
//!
//! The project's codebase is set once by the caller with [`set_project_codebase`]; without one, a file is looked at
//! with the prelude and its own symbols (and, in tests, the sources given to [`with_sources`]). The codebase of a file
//! is kept per thread while rules ask about the same source (the fix rounds of one file).

use std::borrow::Cow;
use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, LazyLock, RwLock};

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::artifacts::AnalysisArtifacts;
use mago_analyzer::plugin::{PluginRegistry, create_registry_with_plugins};
use mago_analyzer::settings::Settings;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{File, FileType};
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_prelude::Prelude;
use mago_span::HasSpan;
use mago_syntax::cst::{
    Access, Call, ClassLikeConstantSelector, Expression, FunctionCall, Literal, Node, TriviaKind,
    Variable,
};
use mago_syntax::parser::parse_file;
use pun_problem::Span;

use crate::Context;

/// Mago's prelude: the built-in classes, functions and constants.
static PRELUDE: LazyLock<(CodebaseMetadata, SymbolReferences)> = LazyLock::new(|| {
    let prelude = Prelude::build();
    (prelude.metadata, prelude.symbol_references)
});

/// The analyser's plugins: Mago's defaults.
static PLUGINS: LazyLock<PluginRegistry> =
    LazyLock::new(|| create_registry_with_plugins(&[], false));

/// The project's codebase, populated, when the caller has scanned the project.
static PROJECT: RwLock<Option<Arc<CodebaseMetadata>>> = RwLock::new(None);

/// Use `codebase` (the project's symbols with the prelude's, populated) for every file from now on, or go back to
/// analysing each file on its own with `None`.
pub fn set_project_codebase(codebase: Option<Arc<CodebaseMetadata>>) {
    *PROJECT.write().unwrap_or_else(|e| e.into_inner()) = codebase;
}

/// The codebase of a project: the prelude with the symbols of `files` (path, code), populated, for
/// [`set_project_codebase`]. `php_version` is a `PHP_VERSION_ID`.
pub fn project_codebase(files: &[(String, String)], php_version: u32) -> Arc<CodebaseMetadata> {
    let version = php_version_of(php_version);
    let (prelude, references) = &*PRELUDE;
    let mut codebase = prelude.clone();
    let mut references = references.clone();
    for (path, code) in files {
        scan_into(
            &mut codebase,
            &mago_file(path, &without_docblocks(code), FileType::Host),
            version,
        );
    }
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    Arc::new(codebase)
}

thread_local! {
    /// Other files' sources whose symbols a file analysed on its own sees (tests: Rector's fixture `Source/` classes).
    static SOURCES: RefCell<Vec<(String, String)>> = const { RefCell::new(Vec::new()) };
    /// The facts of the last file analysed on this thread, by its source and PHP version.
    static LAST: RefCell<Option<(String, u32, Rc<Facts>)>> = const { RefCell::new(None) };
}

/// Run `f` with the symbols of `sources` (path, code) visible to the files analysed on their own on this thread.
pub fn with_sources<R>(sources: Vec<(String, String)>, f: impl FnOnce() -> R) -> R {
    SOURCES.with(|s| *s.borrow_mut() = sources);
    LAST.with(|l| *l.borrow_mut() = None);
    let result = f();
    SOURCES.with(|s| s.borrow_mut().clear());
    LAST.with(|l| *l.borrow_mut() = None);
    result
}

/// What the analysis of a file gives: Mago's types, the spans of the values PHPStan leaves untyped, and the types
/// PHPStan gives in place of Mago's.
struct Analysis {
    artifacts: Option<AnalysisArtifacts>,
    untyped: HashSet<(u32, u32)>,
    declared: HashMap<(u32, u32), TUnion>,
}

/// The codebase a file's rules look symbols up in.
pub(crate) struct Facts {
    pub codebase: Arc<CodebaseMetadata>,
    path: String,
    /// The file's source with its docblocks blanked out.
    native_source: String,
    version: PHPVersion,
    /// The analysis, run the first time a rule asks for a type, and the spans of the values inside traits that come
    /// from a variable.
    artifacts: OnceCell<Analysis>,
}

impl Facts {
    /// The native type of the expression at `span`, if the analyser reached it. Inside a trait, PHPStan (as Rector
    /// runs it) knows no variable, `$this` included, so a value that comes from one has no type.
    pub fn native_type(&self, span: Span) -> Option<&TUnion> {
        let analysis = self.artifacts.get_or_init(|| self.analyse());
        let (start, end) = (span.start as u32, span.end as u32);
        if analysis.untyped.contains(&(start, end)) {
            return None;
        }
        if let Some(t) = analysis.declared.get(&(start, end)) {
            return Some(t);
        }
        analysis
            .artifacts
            .as_ref()?
            .expression_types
            .get(&(start, end))
            .map(|t| t.as_ref())
    }

    /// The declared return type of the built-in function `call` calls (PHP's own signature, not the stub's
    /// docblock), when it calls one.
    fn builtin_return<'s>(
        &'s self,
        names: &ResolvedNames<'_>,
        call: &FunctionCall<'_>,
    ) -> Option<&'s TUnion> {
        let Expression::Identifier(id) = call.function else {
            return None;
        };
        let resolved = names.get(id);
        let name = if self.codebase.get_function(resolved).is_some() {
            resolved
        } else {
            let written = id.value();
            if written.contains(&b'\\') && !written.starts_with(b"\\") {
                return None;
            }
            written.rsplit(|b| *b == b'\\').next()?
        };
        PRELUDE.0.get_function(name)?;
        self.codebase
            .get_function(name)?
            .return_type_declaration_metadata
            .as_ref()
            .map(|t| &t.type_union)
    }

    /// The text at `span` when it is in this file.
    pub fn text(&self, span: mago_span::Span) -> Option<&str> {
        let file = mago_file(&self.path, "", FileType::Host);
        (span.file_id == file.id)
            .then(|| {
                self.native_source
                    .get(span.start.offset as usize..span.end.offset as usize)
            })
            .flatten()
    }

    /// The name Mago gives the anonymous class at `span` (`{anonymous-class:<path>:<line>:<column>}`).
    pub fn anonymous_class_name(&self, span: mago_span::Span) -> String {
        let file = mago_file(&self.path, &self.native_source, FileType::Host);
        mago_codex::get_anonymous_class_name(&file, span).to_string()
    }

    /// Analyse the file. Mago trips debug assertions on some code; an analysis that panics gives no types.
    fn analyse(&self) -> Analysis {
        let file = mago_file(&self.path, &self.native_source, FileType::Host);
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        let mut untyped = HashSet::new();
        let names = NameResolver::new(&arena).resolve(program);
        let mut matches: Vec<(&[u8], u32)> = Vec::new();
        let mut functions: Vec<(u32, u32)> = Vec::new();
        // each `$variable = ...;`: the variable, where the assignment ends, and the declared return type of the
        // built-in function it calls, if it does
        let mut assigned: Vec<(&[u8], u32, Option<&TUnion>)> = Vec::new();
        // `get_class($x)` calls: their span and the argument's
        let mut get_class: Vec<(mago_span::Span, mago_span::Span)> = Vec::new();

        any_node(Node::Program(program), &mut |n| match n {
            Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_) => {
                let span = n.span();
                functions.push((span.start.offset, span.end.offset));
            }
            Node::Assignment(a) if a.operator.is_assign() => {
                if let Expression::Variable(Variable::Direct(v)) = a.lhs {
                    let declared = match a.rhs {
                        Expression::Call(Call::Function(call)) => self.builtin_return(&names, call),
                        _ => None,
                    };
                    assigned.push((v.name, a.span().end.offset, declared));
                }
            }
            Node::Trait(_) => any_node(n, &mut |e| {
                if let Node::Expression(e) = e
                    && from_variable(e)
                {
                    let span = e.span();
                    untyped.insert((span.start.offset, span.end.offset));
                }
            }),
            Node::FunctionCall(call) => {
                matches.extend(dynamic_matches(call));
                if let Expression::Identifier(id) = call.function
                    && id
                        .value()
                        .strip_prefix(b"\\")
                        .unwrap_or(id.value())
                        .eq_ignore_ascii_case(b"get_class")
                    && let Some(argument) = call.argument_list.arguments.first()
                {
                    get_class.push((call.span(), argument.value().span()));
                }
            }
            _ => {}
        });
        // the variable a `preg_match()` with a pattern PHPStan does not know fills, after the call
        if !matches.is_empty() {
            // until the end of the innermost function around the call
            let matches: Vec<(&[u8], u32, u32)> = matches
                .into_iter()
                .map(|(name, after)| {
                    let end = functions
                        .iter()
                        .filter(|(s, e)| *s <= after && after <= *e)
                        .map(|(_, e)| *e)
                        .min()
                        .unwrap_or(u32::MAX);
                    (name, after, end)
                })
                .collect();
            any_node(Node::Program(program), &mut |n| {
                let at = n.span().start.offset;
                if let Node::DirectVariable(v) = n
                    && matches
                        .iter()
                        .any(|(name, after, end)| *name == v.name && *after <= at && at < *end)
                {
                    untyped.insert((v.span.start.offset, v.span.end.offset));
                }
            });
        }
        // a variable set from a built-in function has the function's declared return type in PHPStan's native
        // scope, not the one its stub or an extension refines from the arguments (`str_replace()` gives
        // `array|string`), until the next assignment
        let mut declared: HashMap<(u32, u32), TUnion> = HashMap::new();
        if assigned.iter().any(|(_, _, t)| t.is_some()) {
            any_node(Node::Program(program), &mut |n| {
                let Node::DirectVariable(v) = n else {
                    return;
                };
                let at = v.span.start.offset;
                let function = functions
                    .iter()
                    .filter(|(s, e)| *s <= at && at <= *e)
                    .min_by_key(|(s, e)| e - s)
                    .copied()
                    .unwrap_or((0, u32::MAX));
                let last = assigned
                    .iter()
                    .filter(|(name, after, _)| {
                        *name == v.name && function.0 <= *after && *after <= at
                    })
                    .max_by_key(|(_, after, _)| *after);
                if let Some((_, _, Some(t))) = last {
                    declared.insert((at, v.span.end.offset), (*t).clone());
                }
            });
        }
        let analyzer = Analyzer::new(
            &arena,
            &file,
            &names,
            &self.codebase,
            &PLUGINS,
            Settings::new(self.version),
        );
        let mut result = AnalysisResult::new(SymbolReferences::new());
        let artifacts = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            analyzer.analyze_with_artifacts(program, &mut result)
        }))
        .ok()
        .and_then(Result::ok);
        if let Some(artifacts) = &artifacts {
            // a member of a class the codebase lacks (vendor/ is not scanned; Rector's PHPStan reads it): Mago types
            // it from the rest of the object's union (`?Unknown` gives `null`), PHPStan from the class
            let unknown = |t: &TUnion| {
                t.types.iter().any(|t| match t {
                    // a class name Mago could not resolve
                    TAtomic::Reference(_) => true,
                    TAtomic::Object(TObject::Named(c)) => {
                        self.codebase.get_class_like(c.name.as_bytes()).is_none()
                    }
                    _ => false,
                })
            };
            any_node(Node::Program(program), &mut |n| {
                let (object, span) = match n {
                    Node::PropertyAccess(a) => (a.object, a.span()),
                    Node::NullSafePropertyAccess(a) => (a.object, a.span()),
                    Node::MethodCall(c) => (c.object, c.span()),
                    Node::NullSafeMethodCall(c) => (c.object, c.span()),
                    _ => return,
                };
                let o = object.span();
                if (artifacts.expression_types)
                    .get(&(o.start.offset, o.end.offset))
                    .is_some_and(|t| unknown(t))
                {
                    untyped.insert((span.start.offset, span.end.offset));
                }
            });
            // `get_class()` of a value PHPStan cannot tell is an object may be `false` (`toGetClassResultType`)
            for (call, argument) in get_class {
                let key = (argument.start.offset, argument.end.offset);
                if artifacts
                    .expression_types
                    .get(&key)
                    .is_none_or(|t| t.types.iter().any(|t| matches!(t, TAtomic::Mixed(_))))
                {
                    untyped.insert((call.start.offset, call.end.offset));
                }
            }
        }
        Analysis {
            artifacts,
            untyped,
            declared,
        }
    }
}

/// For `preg_match($pattern, $subject, $matches)` (or `preg_match_all`) with a pattern that is not a string literal:
/// `$matches`, and where the call ends. PHPStan types the matches only for a pattern it knows; Mago's stub always
/// makes them an array.
fn dynamic_matches<'a>(call: &FunctionCall<'a>) -> Option<(&'a [u8], u32)> {
    let Expression::Identifier(name) = call.function else {
        return None;
    };
    let name = name.value().strip_prefix(b"\\").unwrap_or(name.value());
    if !name.eq_ignore_ascii_case(b"preg_match") && !name.eq_ignore_ascii_case(b"preg_match_all") {
        return None;
    }
    let mut arguments = call.argument_list.arguments.iter();
    let pattern = arguments.next()?.value();
    if matches!(pattern, Expression::Literal(Literal::String(_))) {
        return None;
    }
    let Expression::Variable(Variable::Direct(matches)) = arguments.nth(1)?.value() else {
        return None;
    };
    Some((matches.name, call.span().end.offset))
}

/// Whether PHPStan, inside a trait, leaves the value of `expr` untyped: a variable, or a property, method call or
/// offset on one; a constant of `self` or `static`.
fn from_variable(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Variable(_) => true,
        Expression::Parenthesized(p) => from_variable(p.expression),
        Expression::Access(Access::Property(p)) => from_variable(p.object),
        Expression::Access(Access::NullSafeProperty(p)) => from_variable(p.object),
        Expression::Call(Call::Method(c)) => from_variable(c.object),
        Expression::Call(Call::NullSafeMethod(c)) => from_variable(c.object),
        Expression::ArrayAccess(a) => from_variable(a.array),
        // the constants of `self` and `static`, which are not the trait's
        Expression::Access(Access::ClassConstant(c)) => {
            matches!(c.class, Expression::Self_(_) | Expression::Static(_))
                && !matches!(&c.constant, ClassLikeConstantSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"class"))
        }
        _ => false,
    }
}

/// Call `f` on `node` and every node below it.
fn any_node<'a>(node: Node<'a, 'a>, f: &mut impl FnMut(Node<'a, 'a>)) {
    // parents first, in source order, on an explicit stack: a recursive walk of a whole file is as deep as its tree
    let (mut stack, mut children) = (vec![node], Vec::new());
    while let Some(node) = stack.pop() {
        f(node);
        node.visit_children(|child| children.push(child));
        stack.extend(children.drain(..).rev());
    }
}

/// `code` with the text of each docblock blanked out (`/** ... */` becomes `/*  ...  */` with spaces, lines kept),
/// so that every offset stays where it was.
pub(crate) fn without_docblocks(code: &str) -> String {
    let arena = LocalArena::new();
    let file = mago_file("", code, FileType::Host);
    let program = parse_file(&arena, &file);
    let mut bytes = code.as_bytes().to_vec();
    for trivia in program
        .trivia
        .iter()
        .filter(|t| t.kind == TriviaKind::DocBlockComment)
    {
        let (start, end) = (
            trivia.span.start.offset as usize,
            trivia.span.end.offset as usize,
        );
        for b in &mut bytes[start + 2..end - 2] {
            if *b != b'\n' {
                *b = b' ';
            }
        }
    }
    String::from_utf8(bytes).unwrap_or_else(|_| code.to_string())
}

/// The facts of the file `ctx` is about.
pub(crate) fn facts(ctx: &Context<'_>) -> Rc<Facts> {
    let cached = LAST.with(|l| {
        l.borrow()
            .as_ref()
            .filter(|(source, version, _)| *version == ctx.php_version && source == ctx.source)
            .map(|(_, _, facts)| Rc::clone(facts))
    });
    if let Some(facts) = cached {
        return facts;
    }
    let facts = Rc::new(scan(ctx));
    LAST.with(|l| {
        *l.borrow_mut() = Some((ctx.source.to_string(), ctx.php_version, Rc::clone(&facts)))
    });
    facts
}

fn php_version_of(id: u32) -> PHPVersion {
    PHPVersion::new(id / 10000, id / 100 % 100, id % 100)
}

fn mago_file(path: &str, code: &str, file_type: FileType) -> File {
    File::new(
        Cow::Owned(path.as_bytes().to_vec()),
        file_type,
        None,
        Cow::Owned(code.as_bytes().to_vec()),
    )
}

/// The symbols of `file`, added to `codebase`. Mago trips debug assertions on some code; a scan that panics adds
/// none.
fn scan_into(codebase: &mut CodebaseMetadata, file: &File, version: PHPVersion) {
    let arena = LocalArena::new();
    let program = parse_file(&arena, file);
    let names = NameResolver::new(&arena).resolve(program);
    if let Ok(scanned) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        scan_program(&arena, file, program, &names, version)
    })) {
        codebase.extend(scanned);
    }
}

/// The facts of a file: the project's codebase, or the prelude with the file's own symbols.
fn scan(ctx: &Context<'_>) -> Facts {
    let path = ctx.path.to_string_lossy().into_owned();
    let project = PROJECT
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .as_ref()
        .map(Arc::clone);
    let codebase = project.unwrap_or_else(|| {
        let mut files: Vec<(String, String)> = SOURCES.with(|s| s.borrow().clone());
        files.push((path.clone(), ctx.source.to_string()));
        project_codebase(&files, ctx.php_version)
    });
    Facts {
        codebase,
        path,
        native_source: without_docblocks(ctx.source),
        version: php_version_of(ctx.php_version),
        artifacts: OnceCell::new(),
    }
}

/// Whether `class` is one of PHP's own classes (PHPStan: a class reflection without a file name).
pub(crate) fn is_builtin_class(class: &[u8]) -> bool {
    PRELUDE.0.get_class_like(class).is_some()
}

/// Whether `function` is one of PHP's own functions (PHPStan: `FunctionReflection::isBuiltin()`).
pub(crate) fn is_builtin_function(function: &[u8]) -> bool {
    PRELUDE.0.get_function(function).is_some()
}
