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
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    get_float, get_int, get_int_or_float, get_int_or_string, get_mixed_keyed_array, get_string,
};
use mago_database::file::{File, FileId, FileType};
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_prelude::Prelude;
use mago_span::HasSpan;
use mago_syntax::cst::{
    Access, BinaryOperator, Call, ClassLikeConstantSelector, ClassLikeMemberSelector, Expression,
    FunctionCall, Node, TriviaKind, UnaryPrefixOperator, Variable,
};
use mago_syntax::parser::parse_file;
use pun_problem::Span;

use crate::Context;

/// PHP's built-in methods whose return type is only tentative, `class::method` lowercase
/// (scripts/export-tentative-methods.php).
static TENTATIVE: LazyLock<HashSet<&'static [u8]>> = LazyLock::new(|| {
    (include_str!("../../../data/tentative-methods.txt").lines())
        .map(str::as_bytes)
        .collect()
});

/// Mago's prelude: the built-in classes, functions and constants.
static PRELUDE: LazyLock<(CodebaseMetadata, SymbolReferences)> = LazyLock::new(|| {
    let prelude = Prelude::build();
    (prelude.metadata, prelude.symbol_references)
});

/// The analyser's plugins: Mago's defaults.
static PLUGINS: LazyLock<PluginRegistry> =
    LazyLock::new(|| create_registry_with_plugins(&[], false));

/// A project's symbols with the prelude's, populated, scanned twice: with the docblocks blanked out, for PHPStan's
/// native types that Rector's type rules read, and as written, for the types Rector reads of a call of a built-in
/// function and of class constants.
pub struct ProjectCodebase {
    native: Arc<CodebaseMetadata>,
    documented: Arc<CodebaseMetadata>,
}

/// The project's codebase, when the caller has scanned the project.
static PROJECT: RwLock<Option<Arc<ProjectCodebase>>> = RwLock::new(None);

/// Use `codebase` for every file from now on, or go back to analysing each file on its own with `None`.
pub fn set_project_codebase(codebase: Option<Arc<ProjectCodebase>>) {
    *PROJECT.write().unwrap_or_else(|e| e.into_inner()) = codebase;
}

/// The codebase of a project: the prelude with the symbols of `files` (path, code), populated, for
/// [`set_project_codebase`]. `php_version` is a `PHP_VERSION_ID`.
pub fn project_codebase(files: &[(String, String)], php_version: u32) -> Arc<ProjectCodebase> {
    project_codebase_autoloaded(files, php_version, &|_| None)
}

/// [`project_codebase`] with the classes Composer autoloads, as PHPStan's PSR locators find them on demand: each
/// class a scanned file names that the codebase lacks is looked up with `autoload` (an ASCII-lowercased class name
/// to its file's path and code), and that file is scanned too. Files whose path has a `/vendor/` directory count as
/// vendor code for [`is_vendor_file`].
pub fn project_codebase_autoloaded(
    files: &[(String, String)],
    php_version: u32,
    autoload: &dyn Fn(&[u8]) -> Option<(String, String)>,
) -> Arc<ProjectCodebase> {
    let version = php_version_of(php_version);
    let (prelude, references) = &*PRELUDE;
    let (mut codebase, mut documented) = (prelude.clone(), prelude.clone());
    let mut named = Vec::new();
    for (path, code) in files {
        let file = mago_file(path, &without_docblocks(code), FileType::Host);
        remember_source(file.id, code);
        named.extend(scan_into(&mut codebase, &file, version));
        scan_into(
            &mut documented,
            &mago_file(path, code, FileType::Host),
            version,
        );
    }
    let mut asked: HashSet<Vec<u8>> = HashSet::new();
    while let Some(name) = named.pop() {
        if !asked.insert(name.clone()) || codebase.get_class_like(&name).is_some() {
            continue;
        }
        if let Some((path, code)) = autoload(&name) {
            let file = mago_file(&path, &without_docblocks(&code), FileType::Vendored);
            remember_source(file.id, &code);
            if path.contains("/vendor/") {
                (VENDOR.write().unwrap_or_else(|e| e.into_inner())).insert(file.id);
            }
            named.extend(scan_into(&mut codebase, &file, version));
            let file = mago_file(&path, &code, FileType::Vendored);
            scan_into(&mut documented, &file, version);
        }
    }
    let populated = |mut codebase: CodebaseMetadata| {
        let mut references = references.clone();
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        Arc::new(codebase)
    };
    Arc::new(ProjectCodebase {
        native: populated(codebase),
        documented: populated(documented),
    })
}

/// The files [`project_codebase_autoloaded`] read from a `vendor/` directory. A file's id is a hash of its path, so
/// the set only grows, and codebases built at the same time share it.
static VENDOR: LazyLock<RwLock<HashSet<FileId>>> = LazyLock::new(Default::default);

/// Whether `file` is vendor code of the project's codebase (Rector: its path contains `/vendor/`).
pub(crate) fn is_vendor_file(file: FileId) -> bool {
    (VENDOR.read().unwrap_or_else(|e| e.into_inner())).contains(&file)
}

/// The sources of the files the codebases were built from, docblocks kept, by file id, for the rules that read
/// another file's syntax (Rector's `AstResolver`). A file's id is a hash of its path, so a file read again replaces
/// its old source.
static SOURCES_BY_ID: LazyLock<RwLock<HashMap<FileId, Arc<str>>>> = LazyLock::new(Default::default);

fn remember_source(file: FileId, code: &str) {
    (SOURCES_BY_ID.write().unwrap_or_else(|e| e.into_inner())).insert(file, Arc::from(code));
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
    /// The codebase as written, docblocks read (see [`ProjectCodebase`]).
    documented: Arc<CodebaseMetadata>,
    path: String,
    /// The file's source with its docblocks blanked out.
    native_source: String,
    /// The file's source as written.
    source: String,
    version: PHPVersion,
    /// The analysis, run the first time a rule asks for a type, and the spans of the values inside traits that come
    /// from a variable.
    artifacts: OnceCell<Analysis>,
    /// The analysis of the file as written, docblocks read, run the first time a type of it is needed.
    documented_artifacts: OnceCell<Option<AnalysisArtifacts>>,
}

impl Facts {
    /// The native type of the expression at `span`, if the analyser reached it. Inside a trait, PHPStan 2.2.7 (as
    /// Rector runs it, at the version Pinkary pins) does not know `$this`, so a value that comes from it has no type.
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
        self.codebase
            .get_function(&self.builtin_name(names, call)?)?
            .return_type_declaration_metadata
            .as_ref()
            .map(|t| &t.type_union)
    }

    /// The name of the built-in function `call` calls, when it calls one (PHP falls back to the global function).
    fn builtin_name(&self, names: &ResolvedNames<'_>, call: &FunctionCall<'_>) -> Option<Vec<u8>> {
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
        Some(name.to_vec())
    }

    /// What arithmetic on `operand` gives in PHPStan's native scope when `operand` calls a built-in function, whose
    /// declared return type it takes there: `Some(true)` an error, for a type that may be a string, an object or an
    /// array (which only `+` takes); `Some(false)` `float|int`, for `mixed` or a type that may be a float or an
    /// integer (`abs()`'s `int|float`); `None` the type Mago gives.
    fn builtin_operand(
        &self,
        names: &ResolvedNames<'_>,
        operand: &Expression<'_>,
        plus: bool,
    ) -> Option<bool> {
        let Expression::Call(Call::Function(call)) = crate::syntax::unparenthesize(operand) else {
            return None;
        };
        let t = self.builtin_return(names, call)?;
        let error = (t.types.iter()).any(|a| match a {
            TAtomic::Scalar(s) => s.is_any_string(),
            TAtomic::Array(_) => !plus,
            other => matches!(other, TAtomic::Object(_) | TAtomic::Reference(_)),
        });
        let float = t
            .types
            .iter()
            .any(|a| matches!(a, TAtomic::Scalar(s) if s.is_float()));
        let wide = t.is_mixed()
            || (float
                && t.types
                    .iter()
                    .any(|a| !matches!(a, TAtomic::Scalar(s) if s.is_float())));
        (error || wide).then_some(error)
    }

    /// Whether PHPStan's native type of calling `method` on a value of type `receiver` is `mixed` where Mago's is not:
    /// the method is a built-in one whose return type is only tentative (`DateTime::getTimestamp()`), and the receiver
    /// is not one built-in class, for which Rector's `getNativeType` takes the full type instead.
    fn tentative_call(&self, receiver: &TUnion, method: &[u8]) -> bool {
        let classes: Vec<&[u8]> = (receiver.types.iter())
            .filter_map(|t| match t {
                TAtomic::Object(TObject::Named(n)) => Some(n.name.as_bytes()),
                _ => None,
            })
            .collect();
        if let [class] = classes[..]
            && receiver.types.len() == 1
            && is_builtin_class(class)
        {
            return false;
        }
        let method = method.to_ascii_lowercase();
        classes.iter().any(|class| {
            (self.codebase.get_class_like(class))
                .and_then(|c| {
                    c.declaring_method_ids
                        .iter()
                        .find(|(m, _)| m.as_bytes() == method)
                })
                .is_some_and(|(_, id)| {
                    let declaring = id.get_class_name().as_bytes().to_ascii_lowercase();
                    let key = [declaring.as_slice(), b"::", method.as_slice()].concat();
                    TENTATIVE.contains(key.as_slice())
                })
        })
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

    /// The codebase with the docblocks read (PHPStan's reflection with PHPDoc types).
    pub fn documented(&self) -> &CodebaseMetadata {
        &self.documented
    }

    /// The source of the file `file` of the codebase, docblocks kept, as the codebase read it (its spans point into
    /// it).
    pub fn source_of(&self, file: FileId) -> Option<Arc<str>> {
        (SOURCES_BY_ID.read().unwrap_or_else(|e| e.into_inner()))
            .get(&file)
            .cloned()
    }

    /// The name Mago gives the anonymous class at `span` (`{anonymous-class:<path>:<line>:<column>}`).
    pub fn anonymous_class_name(&self, span: mago_span::Span) -> String {
        let file = mago_file(&self.path, &self.native_source, FileType::Host);
        mago_codex::get_anonymous_class_name(&file, span).to_string()
    }

    /// The type of the expression at `span` with the docblocks read (PHPStan's `getType()`), if the analyser reached
    /// it. A docblock never widens a native type (`TypehintHelper::decideType`), so where Mago loses the documented
    /// type to `mixed` (a variable a closure captured by reference, once the closure is called) the native type
    /// stands.
    pub fn full_type(&self, span: Span) -> Option<&TUnion> {
        let full = (self.documented_artifacts()?.expression_types)
            .get(&(span.start as u32, span.end as u32))?;
        match self.native_type(span) {
            Some(native) if full.is_mixed() && full.by_reference() && !native.is_mixed() => {
                Some(native)
            }
            _ => Some(full.as_ref()),
        }
    }

    fn documented_artifacts(&self) -> Option<&AnalysisArtifacts> {
        (self.documented_artifacts)
            .get_or_init(|| self.analyse_documented())
            .as_ref()
    }

    /// Mago's analysis of the file as written, docblocks read, in the documented codebase.
    fn analyse_documented(&self) -> Option<AnalysisArtifacts> {
        let file = mago_file(&self.path, &self.source, FileType::Host);
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        let names = NameResolver::new(&arena).resolve(program);
        let settings = Settings::new(self.version);
        let analyzer = Analyzer::new(&arena, &file, &names, &self.documented, &PLUGINS, settings);
        let mut result = AnalysisResult::new(SymbolReferences::new());
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            analyzer.analyze_with_artifacts(program, &mut result)
        }))
        .ok()
        .and_then(Result::ok)
    }

    /// Analyse the file. Mago trips debug assertions on some code; an analysis that panics gives no types.
    fn analyse(&self) -> Analysis {
        let file = mago_file(&self.path, &self.native_source, FileType::Host);
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        let mut untyped = HashSet::new();
        let names = NameResolver::new(&arena).resolve(program);
        let mut functions: Vec<(u32, u32)> = Vec::new();
        // each `$variable = ...;`: the variable, where the assignment ends, and the declared return type of the
        // built-in function it calls, if it does
        let mut assigned: Vec<(&[u8], u32, Option<&TUnion>)> = Vec::new();
        // the calls of built-in functions, and the argument of each `get_class()`
        let mut builtin_calls: Vec<(mago_span::Span, Option<mago_span::Span>)> = Vec::new();
        // arithmetic PHPStan types `float|int` where Mago's type is narrower
        let mut widened: Vec<(u32, u32)> = Vec::new();

        any_node(Node::Program(program), &mut |n| match n {
            Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_) => {
                let span = n.span();
                functions.push((span.start.offset, span.end.offset));
            }
            // a compound assignment (`$s .= 'x'`) ends the declared type too
            Node::Assignment(a) => {
                if let Expression::Variable(Variable::Direct(v)) = a.lhs {
                    let declared = match a.rhs {
                        Expression::Call(Call::Function(call)) if a.operator.is_assign() => {
                            self.builtin_return(&names, call)
                        }
                        _ => None,
                    };
                    assigned.push((v.name, a.span().end.offset, declared));
                }
            }
            // Rector takes a built-in function call's full type only for the call itself (`getNativeType`); inside
            // arithmetic PHPStan's native scope types it by its declaration (see `builtin_operand`)
            Node::Binary(b) if b.operator.is_arithmetic() => {
                let plus = matches!(b.operator, BinaryOperator::Addition(_));
                let operands = [b.lhs, b.rhs].map(|o| self.builtin_operand(&names, o, plus));
                let span = (b.span().start.offset, b.span().end.offset);
                if operands.contains(&Some(true)) {
                    untyped.insert(span);
                } else if operands.contains(&Some(false)) {
                    widened.push(span);
                }
            }
            Node::UnaryPrefix(u) if u.operator.is_arithmetic() => {
                let span = (u.span().start.offset, u.span().end.offset);
                match self.builtin_operand(&names, u.operand, false) {
                    Some(true) => {
                        untyped.insert(span);
                    }
                    Some(false) => widened.push(span),
                    None => {}
                }
            }
            Node::Trait(_) => {
                // an anonymous class in the trait has its own `$this`, which PHPStan types
                let mut anonymous = Vec::new();
                any_node(n, &mut |c| {
                    if let Node::AnonymousClass(c) = c {
                        anonymous.push((c.span().start.offset, c.span().end.offset));
                    }
                });
                let own = |s: mago_span::Span| {
                    !(anonymous.iter()).any(|(a, b)| *a <= s.start.offset && s.end.offset <= *b)
                };
                any_node(n, &mut |e| {
                    if let Node::Expression(e) = e
                        && own(e.span())
                        && from_this(e)
                    {
                        let span = e.span();
                        untyped.insert((span.start.offset, span.end.offset));
                    }
                });
                // a variable whose latest assignment in the method takes such a value has none either
                any_node(n, &mut |m| {
                    if let Node::Method(_) = m
                        && own(m.span())
                    {
                        let mut writes: Vec<(&[u8], u32, bool)> = Vec::new();
                        any_node(m, &mut |a| {
                            if let Node::Assignment(a) = a
                                && let Expression::Variable(Variable::Direct(v)) = a.lhs
                            {
                                let this =
                                    a.operator.is_assign() && own(a.span()) && from_this(a.rhs);
                                writes.push((v.name, a.span().end.offset, this));
                            }
                        });
                        if !writes.iter().any(|w| w.2) {
                            return;
                        }
                        any_node(m, &mut |r| {
                            let Node::DirectVariable(v) = r else { return };
                            let at = v.span.start.offset;
                            let last = (writes.iter())
                                .filter(|(name, end, _)| *name == v.name && *end <= at)
                                .max_by_key(|(_, end, _)| *end);
                            if let Some((_, _, true)) = last {
                                untyped.insert((at, v.span.end.offset));
                            }
                        });
                    }
                });
            }
            Node::FunctionCall(call) => {
                if let Some(name) = self.builtin_name(&names, call) {
                    let argument = (name.eq_ignore_ascii_case(b"get_class"))
                        .then(|| {
                            call.argument_list
                                .arguments
                                .first()
                                .map(|a| a.value().span())
                        })
                        .flatten();
                    builtin_calls.push((call.span(), argument));
                }
            }
            _ => {}
        });
        // a variable set from a built-in function has the function's declared return type in PHPStan's native
        // scope, not the one its stub or an extension refines from the arguments (`str_replace()` gives
        // `array|string`), until the next assignment
        let mut declared: HashMap<(u32, u32), TUnion> = HashMap::new();
        for span in widened {
            declared.insert(span, get_int_or_float());
        }
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
        // `.` on a value whose native type may be an array is an error in PHPStan's native scope: a call of a built-in
        // function declared to return one (`str_replace()`'s `array|string`), or a variable set from such a call. The
        // error stays with that concatenation; one around it is a string again.
        any_node(Node::Program(program), &mut |n| {
            let Node::Binary(b) = n else { return };
            if !b.operator.is_concatenation() {
                return;
            }
            let array = |o: &Expression<'_>| {
                let t = match crate::syntax::unparenthesize(o) {
                    Expression::Call(Call::Function(call)) => self.builtin_return(&names, call),
                    Expression::Variable(Variable::Direct(v)) => {
                        declared.get(&(v.span.start.offset, v.span.end.offset))
                    }
                    _ => None,
                };
                t.is_some_and(|t| t.types.iter().any(|a| matches!(a, TAtomic::Array(_))))
            };
            if array(b.lhs) || array(b.rhs) {
                untyped.insert((b.span().start.offset, b.span().end.offset));
            }
        });
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
            // (a class name Mago could not resolve is a reference; `new` of one is `never`)
            let unknown = |t: &TUnion| {
                (t.types.iter()).any(|t| matches!(t, TAtomic::Reference(_) | TAtomic::Never))
            };
            any_node(Node::Program(program), &mut |n| {
                let (object, span, method) = match n {
                    Node::PropertyAccess(a) => (a.object, a.span(), None),
                    Node::NullSafePropertyAccess(a) => (a.object, a.span(), None),
                    Node::MethodCall(c) => (c.object, c.span(), Some(&c.method)),
                    Node::NullSafeMethodCall(c) => (c.object, c.span(), Some(&c.method)),
                    _ => return,
                };
                // Mago types `(new X)` under the span inside the parentheses
                let o = crate::syntax::unparenthesize(object).span();
                let Some(t) = (artifacts.expression_types).get(&(o.start.offset, o.end.offset))
                else {
                    return;
                };
                let tentative = || match method {
                    Some(ClassLikeMemberSelector::Identifier(m)) => self.tentative_call(t, m.value),
                    _ => false,
                };
                if unknown(t) || tentative() {
                    untyped.insert((span.start.offset, span.end.offset));
                }
            });
            // `getClassConstFetchTypeByReflection`: a constant fetched from `static` or from an object (`$this::A`) is
            // `mixed` when neither its class nor the constant is final and the constant has no type
            let open = |class: &[u8], constant: &[u8]| {
                (self.documented.get_class_like(class)).is_some_and(|c| !c.flags.is_final())
                    && (self.documented.get_class_constant(class, constant))
                        .is_some_and(|k| !k.flags.is_final() && k.type_metadata.is_none())
            };
            // the class-likes `static` can be, by span: the name of each class and interface, `None` for the
            // anonymous classes (final to PHPStan)
            let mut scopes: Vec<(mago_span::Span, Option<&[u8]>)> = Vec::new();
            any_node(Node::Program(program), &mut |n| match n {
                Node::Class(c) => scopes.push((c.span(), Some(names.get(&c.name)))),
                Node::Interface(i) => scopes.push((i.span(), Some(names.get(&i.name)))),
                Node::AnonymousClass(c) => scopes.push((c.span(), None)),
                Node::ClassConstantAccess(a) => {
                    let ClassLikeConstantSelector::Identifier(constant) = &a.constant else {
                        return;
                    };
                    if constant.value.eq_ignore_ascii_case(b"class") {
                        return;
                    }
                    let span = a.span();
                    let mixed = match a.class {
                        Expression::Static(_) => (scopes.iter().rev())
                            .find(|(s, _)| {
                                s.start.offset <= span.start.offset
                                    && span.end.offset <= s.end.offset
                            })
                            .and_then(|(_, class)| *class)
                            .is_some_and(|class| open(class, constant.value)),
                        Expression::Self_(_)
                        | Expression::Parent(_)
                        | Expression::Identifier(_) => false,
                        object => {
                            let o = object.span();
                            (artifacts
                                .expression_types
                                .get(&(o.start.offset, o.end.offset)))
                            .is_some_and(|t| {
                                t.types.iter().any(|t| match t {
                                    TAtomic::Object(TObject::Named(n)) => {
                                        open(n.name.as_bytes(), constant.value)
                                    }
                                    _ => false,
                                })
                            })
                        }
                    };
                    if mixed {
                        untyped.insert((span.start.offset, span.end.offset));
                    }
                }
                _ => {}
            });
            // a `(string)` cast is a string, also of an object whose `__toString()` declares no type (PHP declares it
            // `string`), where Mago takes that method's missing type
            any_node(Node::Program(program), &mut |n| {
                if let Node::UnaryPrefix(u) = n
                    && matches!(
                        u.operator,
                        UnaryPrefixOperator::StringCast(..) | UnaryPrefixOperator::BinaryCast(..)
                    )
                {
                    let key = (u.span().start.offset, u.span().end.offset);
                    if (artifacts.expression_types.get(&key)).is_none_or(|t| {
                        !t.types
                            .iter()
                            .all(|a| matches!(a, TAtomic::Scalar(s) if s.is_any_string()))
                    }) {
                        declared.insert(key, get_string());
                    }
                }
            });
            // arithmetic and bitwise operators on a `mixed` operand, where Mago keeps `mixed` and PHPStan's
            // `InitializerExprTypeResolver` gives a number type, innermost first
            let mut operations = Vec::new();
            any_node(Node::Program(program), &mut |n| {
                if let Node::Binary(b) = n
                    && (b.operator.is_arithmetic() || b.operator.is_bitwise())
                {
                    operations.push(b);
                }
            });
            for b in operations.into_iter().rev() {
                let key = |e: &Expression<'_>| {
                    let s = crate::syntax::unparenthesize(e).span();
                    (s.start.offset, s.end.offset)
                };
                let kind = |e: &Expression<'_>| -> Option<Operand> {
                    if untyped.contains(&key(e)) {
                        return None;
                    }
                    let t = (declared.get(&key(e)))
                        .or_else(|| artifacts.expression_types.get(&key(e)).map(|t| t.as_ref()))?;
                    operand(t)
                };
                let (Some(left), Some(right)) = (kind(b.lhs), kind(b.rhs)) else {
                    continue;
                };
                if let Some(t) = mixed_math(&b.operator, left, right) {
                    declared.insert((b.span().start.offset, b.span().end.offset), t);
                }
            }
        }
        // `getNativeType` takes the full type, docblocks read, of a call of a built-in function
        if let Some(documented) = (!builtin_calls.is_empty())
            .then(|| self.documented_artifacts())
            .flatten()
        {
            let key = |s: mago_span::Span| (s.start.offset, s.end.offset);
            let t = |s: mago_span::Span| documented.expression_types.get(&key(s));
            for (call, argument) in builtin_calls {
                // `get_class()` of a value PHPStan cannot tell is an object may be `false` (`toGetClassResultType`)
                if argument.is_some_and(|a| {
                    t(a).is_none_or(|t| t.types.iter().any(|t| matches!(t, TAtomic::Mixed(_))))
                }) {
                    untyped.insert(key(call));
                } else if let Some(t) = t(call) {
                    declared.insert(key(call), TUnion::clone(t));
                }
            }
        }
        // an untyped value (PHPStan's `mixed` or error, where Mago's analysis still had a type) spreads, innermost
        // first: the parentheses and ternaries around it are untyped too, and arithmetic on it is `float|int`
        if !untyped.is_empty() {
            let mut around = Vec::new();
            any_node(Node::Program(program), &mut |n| match n {
                Node::Parenthesized(p) => around.push((p.span(), false, vec![p.expression])),
                Node::Conditional(c) => around.push((
                    c.span(),
                    false,
                    vec![c.then.unwrap_or(c.condition), c.r#else],
                )),
                Node::Binary(b) if b.operator.is_arithmetic() => {
                    around.push((b.span(), true, vec![b.lhs, b.rhs]))
                }
                Node::UnaryPrefix(u) if u.operator.is_arithmetic() => {
                    around.push((u.span(), true, vec![u.operand]))
                }
                _ => {}
            });
            for (span, arithmetic, parts) in around.into_iter().rev() {
                let key = |s: mago_span::Span| (s.start.offset, s.end.offset);
                if untyped.contains(&key(span)) {
                    continue;
                }
                if !parts.iter().any(|p| untyped.contains(&key(p.span()))) {
                    // `(1 + $mixed)` is the type of what it encloses
                    if let (false, [inner]) = (arithmetic, &parts[..])
                        && let Some(t) = declared.get(&key(inner.span())).cloned()
                    {
                        declared.insert(key(span), t);
                    }
                    continue;
                }
                if arithmetic {
                    declared.insert(key(span), get_int_or_float());
                } else {
                    untyped.insert(key(span));
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

/// What [`mixed_math`] needs of an operand's native type.
#[derive(Clone, Copy, PartialEq)]
enum Operand {
    Mixed,
    /// `int|float`, which only [`mixed_math`] gives (Mago types the operation `mixed`)
    Number,
    Int,
    Float,
}

fn operand(t: &TUnion) -> Option<Operand> {
    let all =
        |f: fn(&TScalar) -> bool| (t.types.iter()).all(|a| matches!(a, TAtomic::Scalar(s) if f(s)));
    if t.is_mixed() {
        Some(Operand::Mixed)
    } else if all(TScalar::is_int) {
        Some(Operand::Int)
    } else if all(TScalar::is_float) {
        Some(Operand::Float)
    } else if all(|s| s.is_int() || s.is_float()) {
        Some(Operand::Number)
    } else {
        None
    }
}

/// PHPStan's native type of `left <operator> right` when an operand is `mixed` or a number that only this gave
/// (`getPlusType`, `getPowType`, `resolveCommonMath` and the bitwise types), each union in the order PHPStan builds
/// it; `None` otherwise, or when the result is not a plain number type.
fn mixed_math(operator: &BinaryOperator<'_>, left: Operand, right: Operand) -> Option<TUnion> {
    use Operand::*;
    if ![left, right].iter().any(|o| matches!(o, Mixed | Number)) {
        return None;
    }
    let float = left == Float || right == Float;
    // in this order, unsorted (`TUnion::from_vec` sorts)
    let union = |types: Vec<TUnion>| {
        TUnion::new(Cow::Owned(
            types
                .into_iter()
                .flat_map(|t| t.types.into_owned())
                .collect(),
        ))
    };
    Some(match operator {
        BinaryOperator::Addition(_) if left == Mixed && right == Mixed => {
            union(vec![get_float(), get_int(), get_mixed_keyed_array()])
        }
        BinaryOperator::Exponentiation(_) if left == Mixed => union(vec![get_float(), get_int()]),
        BinaryOperator::Exponentiation(_) => return None,
        BinaryOperator::Modulo(_) => get_int(),
        BinaryOperator::LeftShift(_) | BinaryOperator::RightShift(_) if float => get_int(),
        BinaryOperator::BitwiseAnd(_)
        | BinaryOperator::BitwiseOr(_)
        | BinaryOperator::BitwiseXor(_) => match (left, right) {
            (Mixed, Mixed) => get_int_or_string(),
            _ => get_int(),
        },
        _ if float => get_float(),
        _ => get_int_or_float(),
    })
}

/// Whether PHPStan, inside a trait, leaves the value of `expr` untyped: `$this`, or a property, method call, offset or clone
/// of it; a constant of `self` or `static`.
fn from_this(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Variable(Variable::Direct(v)) => v.name == b"$this",
        Expression::Parenthesized(p) => from_this(p.expression),
        Expression::Access(Access::Property(p)) => from_this(p.object),
        Expression::Access(Access::NullSafeProperty(p)) => from_this(p.object),
        Expression::Call(Call::Method(c)) => from_this(c.object),
        Expression::Call(Call::NullSafeMethod(c)) => from_this(c.object),
        Expression::ArrayAccess(a) => from_this(a.array),
        Expression::Clone(c) => from_this(c.object),
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

/// The symbols of `file`, added to `codebase`, and the names the file resolves, ASCII-lowercased. Mago trips debug
/// assertions on some code; a scan that panics adds none.
fn scan_into(codebase: &mut CodebaseMetadata, file: &File, version: PHPVersion) -> Vec<Vec<u8>> {
    let arena = LocalArena::new();
    let program = parse_file(&arena, file);
    let names = NameResolver::new(&arena).resolve(program);
    if let Ok(scanned) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        scan_program(&arena, file, program, &names, version)
    })) {
        codebase.extend(scanned);
    }
    names
        .iter()
        .map(|(_, _, name, _)| name.to_ascii_lowercase())
        .collect()
}

/// The facts of a file: the project's codebase, or the prelude with the file's own symbols.
fn scan(ctx: &Context<'_>) -> Facts {
    let path = ctx.path.to_string_lossy().into_owned();
    let project = PROJECT
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .as_ref()
        .map(Arc::clone);
    let project = project.unwrap_or_else(|| {
        let mut files: Vec<(String, String)> = SOURCES.with(|s| s.borrow().clone());
        files.push((path.clone(), ctx.source.to_string()));
        project_codebase(&files, ctx.php_version)
    });
    Facts {
        codebase: Arc::clone(&project.native),
        documented: Arc::clone(&project.documented),
        path,
        native_source: without_docblocks(ctx.source),
        source: ctx.source.to_string(),
        version: php_version_of(ctx.php_version),
        artifacts: OnceCell::new(),
        documented_artifacts: OnceCell::new(),
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
