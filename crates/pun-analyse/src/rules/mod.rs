//! PHPStan's level-0 rules. One walk per analysed file; each `walk_in_*` hook is the `processNode()` of the rules
//! registered for that node type, in PHPStan's source order.

mod calls;
mod classes;
mod members;
mod names;
mod params;
mod scope;

use std::collections::{HashMap, HashSet};
use std::path::Path;

use mago_allocator::{Arena, LocalArena};
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::PluginRegistry;
use mago_analyzer::settings::Settings;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::reference::SymbolReferences;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{File, FileType};
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::parser::parse_file;
use mago_syntax::walker::MutWalker;

use pun_phpstan::comments::lines_to_ignore;
use pun_phpstan::ignore::{
    Error, ignores_without_comments, local_ignores, parse_errors, unmatched_line_errors,
};

use crate::Project;
use crate::codebase::{Sources, read};
use mago_codex::symbol::SymbolKind;
use mago_database::file::FileId;
use scope::{Guard, falsy, statements_terminate, terminates, truthy};
use std::path::PathBuf;
use std::sync::Arc;

/// What the rules read of the project (its `ignoreErrors` stay on the calling thread).
#[derive(Clone, Copy)]
pub struct Checks {
    php_version: u32,
    report_ignores_without_comments: bool,
    report_unmatched_ignores: bool,
}

impl Checks {
    pub fn of(project: &Project) -> Checks {
        Checks {
            php_version: project.php_version,
            report_ignores_without_comments: project.report_ignores_without_comments,
            report_unmatched_ignores: project.report_unmatched_ignores,
        }
    }
}

pub const IDENTIFIERS: &[&str] = &[
    "function.notFound",
    "class.notFound",
    "new.static",
    "new.trait",
    "new.enum",
    "new.interface",
    "new.abstract",
    "new.noConstructor",
    "new.privateConstructor",
    "new.protectedConstructor",
    "arguments.count",
    "function.void",
    "method.void",
    "staticMethod.void",
    "method.notFound",
    "method.private",
    "method.protected",
    "staticMethod.notFound",
    "method.staticCall",
    "staticMethod.private",
    "staticMethod.protected",
    "method.callToAbstract",
    "staticMethod.callToAbstract",
    "outOfClass.self",
    "outOfClass.static",
    "outOfClass.parent",
    "class.noParent",
    "classConstant.notFound",
    "classConstant.onTrait",
    "classConstant.private",
    "classConstant.protected",
    "catch.notThrowable",
    "parameter.trait",
    "return.trait",
    "interface.notFound",
    "class.extendsInterface",
    "class.extendsTrait",
    "class.extendsEnum",
    "interfaceExtends.class",
    "interfaceExtends.trait",
    "interfaceExtends.enum",
    "classImplements.class",
    "classImplements.trait",
    "classImplements.enum",
    "enumImplements.class",
    "enumImplements.trait",
    "enumImplements.enum",
    "ignore.unmatchedLine",
    "ignore.unmatchedIdentifier",
    "ignore.parseError",
    "ignore.noComment",
    "ignore.unmatched",
    "ignore.count",
    "ignore.allLineErrors",
];

pub const DISCOVERING_SYMBOLS_TIP: &str =
    "Learn more at https://phpstan.org/user-guide/discovering-symbols";

/// The class-like a node is in (`$scope->getClassReflection()`).
struct ClassScope {
    name: Option<String>,
    /// Guards the constructor ends with, which PHPStan remembers in the other methods.
    constructor_guards: Vec<Guard>,
}

/// `$scope->getFunction()`: the function or method a node is in (closures don't change it).
pub struct FunctionScope {
    pub name: String,
    /// `Some(is_static)` for a method.
    pub method_static: Option<bool>,
}

/// What a function-like body saved of its parent's scope.
struct Saved {
    guards: Vec<Guard>,
    marks: Vec<usize>,
    vars: HashMap<Vec<u8>, Vec<Guard>>,
}

pub struct Checker<'a, 'arena> {
    file: &'a File,
    /// Mago's inferred types of the expressions rules ask about (see [`typed_expressions()`]).
    types: HashMap<Span, TUnion>,
    names: &'a ResolvedNames<'arena>,
    codebase: &'a CodebaseMetadata,
    classes: Vec<ClassScope>,
    /// Guards in effect; `marks` are where enclosing blocks started (guards an early exit leaves last to the
    /// block's end).
    guards: Vec<Guard>,
    marks: Vec<usize>,
    vars: HashMap<Vec<u8>, Vec<Guard>>,
    saved: Vec<Saved>,
    functions: Vec<FunctionScope>,
    /// Inside the arguments of class_exists() & co. (`inFunctionCallsStack`).
    in_exists_call: usize,
    /// Spans of expressions PHPStan processes in a top-level ExpressionContext (`isInFirstLevelStatement()`):
    /// expression statements, `for` initialisations and increments, arrow function and match arm bodies.
    top_level: HashSet<(u32, u32)>,
    /// `f(...)` on the right of `|>`: analysed as the call `f($x)`, not as a first-class callable.
    piped: HashSet<Span>,
    /// Closures whose `$this` is rebound (`->call()`, `->bind()`, `->bindTo()`, `Closure::bind()`).
    rebound: HashSet<Span>,
    /// `Error::file`: the path, or `T.php (in context of class C)` inside a trait analysed for class C.
    label: String,
    /// The analysed class's file and name when walking a trait in its context (`$scope->isInTrait()`); the trait
    /// walked is `context_trait`.
    context: Option<(String, String)>,
    context_trait: Option<String>,
    /// Inside the trait `context_trait` (in a context walk, nothing else reports).
    in_target: bool,
    /// (class, trait) for each `use T;` in a class body, to analyse T in the class's context.
    trait_uses: Vec<(String, String)>,
    /// Line ranges of the traits declared in this file (their ignores are never unmatched here).
    trait_lines: Vec<(i64, i64)>,
    errors: Vec<Error>,
}

/// A checker for one walk of `file`.
fn checker<'a, 'arena>(
    arena: &'arena LocalArena,
    file: &'a File,
    program: &'arena Program<'arena>,
    names: &'a ResolvedNames<'arena>,
    codebase: &'a CodebaseMetadata,
    php_version: u32,
) -> Checker<'a, 'arena> {
    Checker {
        file,
        types: infer(arena, file, program, names, codebase, php_version),
        names,
        codebase,
        classes: Vec::new(),
        guards: Vec::new(),
        marks: Vec::new(),
        vars: HashMap::new(),
        saved: Vec::new(),
        functions: Vec::new(),
        in_exists_call: 0,
        top_level: HashSet::new(),
        piped: HashSet::new(),
        rebound: HashSet::new(),
        label: String::from_utf8_lossy(&file.name).into_owned(),
        context: None,
        context_trait: None,
        in_target: false,
        trait_uses: Vec::new(),
        trait_lines: Vec::new(),
        errors: Vec::new(),
    }
}

/// The file's errors after its `@phpstan-ignore*` comments (LocalIgnoresProcessor), with the unmatched ones, and
/// the errors of the traits its classes use, analysed in each class's context. `files` finds a symbol's file.
pub fn check_file(
    path: &Path,
    codebase: &CodebaseMetadata,
    project: &Checks,
    files: &HashMap<FileId, PathBuf>,
    sources: &Sources,
) -> Vec<Error> {
    // the scan read every analysed file it could
    let Some(file) = sources.get(path) else {
        return Vec::new();
    };
    let arena = LocalArena::new();
    let program = parse_file(&arena, file);
    let names = NameResolver::new(&arena).resolve(program);
    let mut checker = checker(&arena, file, program, &names, codebase, project.php_version);
    checker.walk_program(program, &mut ());
    let name = checker.file_name();
    let (mut lines, parse) = lines_to_ignore(&file.contents);
    let mut unmatched = lines.clone();
    unmatched.retain(|line, _| {
        !checker
            .trait_lines
            .iter()
            .any(|(a, b)| a <= line && line <= b)
    });
    let mut errors = std::mem::take(&mut checker.errors);
    errors.extend(parse_errors(&name, &parse));
    let mut out = Vec::new();
    if project.report_ignores_without_comments {
        out.extend(ignores_without_comments(&name, &mut lines, &mut unmatched));
    }
    let (kept, _) = local_ignores(&name, errors, &mut lines, &mut unmatched);
    out.extend(kept);
    if project.report_unmatched_ignores {
        out.extend(unmatched_line_errors(&name, &unmatched));
    }
    let mut pending = std::mem::take(&mut checker.trait_uses);
    let mut seen = HashSet::new();
    while let Some((class, trait_name)) = pending.pop() {
        if !seen.insert((class.to_ascii_lowercase(), trait_name.to_ascii_lowercase())) {
            continue;
        }
        let (errors, uses) = in_trait_context(
            &name,
            &class,
            &trait_name,
            codebase,
            project,
            files,
            sources,
        );
        out.extend(errors);
        pending.extend(uses);
    }
    out
}

/// InTraitNode: trait `trait_name` walked in the context of `class` (declared in `class_file`), its errors labelled
/// `T.php (in context of class C)` and matched against the trait's own `@phpstan-ignore` comments. Also returns the
/// traits it uses.
fn in_trait_context(
    class_file: &str,
    class: &str,
    trait_name: &str,
    codebase: &CodebaseMetadata,
    project: &Checks,
    files: &HashMap<FileId, PathBuf>,
    sources: &Sources,
) -> (Vec<Error>, Vec<(String, String)>) {
    let Some(t) = codebase
        .get_class_like(trait_name.as_bytes())
        .filter(|t| t.kind == SymbolKind::Trait)
    else {
        return (Vec::new(), Vec::new());
    };
    // an analysed trait as the scan read it, a scanned one from disk
    let Some(file) = files.get(&t.span.file_id).and_then(|p| {
        sources
            .get(p)
            .cloned()
            .or_else(|| read(p, FileType::Host).map(Arc::new))
    }) else {
        return (Vec::new(), Vec::new());
    };
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let mut checker = checker(
        &arena,
        &file,
        program,
        &names,
        codebase,
        project.php_version,
    );
    let display = codebase.get_class_like(class.as_bytes()).map_or_else(
        || class.to_string(),
        |k| String::from_utf8_lossy(k.original_name.as_bytes()).into_owned(),
    );
    checker.label = format!("{} (in context of class {display})", checker.file_name());
    checker.context = Some((class_file.to_string(), class.to_string()));
    checker.context_trait = Some(String::from_utf8_lossy(t.name.as_bytes()).into_owned());
    checker.walk_program(program, &mut ());
    let label = checker.label.clone();
    let (mut lines, _) = lines_to_ignore(&file.contents);
    lines.retain(|line, _| {
        checker
            .trait_lines
            .iter()
            .any(|(a, b)| a <= line && line <= b)
    });
    let mut unmatched = lines.clone();
    let (kept, _) = local_ignores(
        &label,
        std::mem::take(&mut checker.errors),
        &mut lines,
        &mut unmatched,
    );
    (kept, checker.trait_uses)
}

/// The expressions whose type a rule reads: the class of `new $expr`.
#[derive(Default)]
struct TypedExpressions(Vec<Span>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for TypedExpressions {
    fn walk_in_instantiation(&mut self, new: &'ast Instantiation<'arena>, _: &mut ()) {
        if !matches!(
            new.class,
            Expression::Identifier(_)
                | Expression::Static(_)
                | Expression::Self_(_)
                | Expression::Parent(_)
                | Expression::AnonymousClass(_)
        ) {
            self.0.push(new.class.span());
        }
    }
}

/// Runs Mago's analyzer over the file when a rule needs an expression's type (most files need none).
fn infer<'arena>(
    arena: &'arena LocalArena,
    file: &File,
    program: &'arena Program<'arena>,
    names: &ResolvedNames<'arena>,
    codebase: &CodebaseMetadata,
    php_version: u32,
) -> HashMap<Span, TUnion> {
    let mut wanted = TypedExpressions::default();
    wanted.walk_program(program, &mut ());
    if wanted.0.is_empty() {
        return HashMap::new();
    }
    let registry = PluginRegistry::new();
    let settings = Settings::new(crate::codebase::php_version(php_version));
    let analyzer = Analyzer::new(arena, file, names, codebase, &registry, settings);
    let mut result = AnalysisResult::new(SymbolReferences::new());
    // Only the declarations holding a wanted expression: a method's types come from its own body and the codebase,
    // so the file's other functions and methods would only cost time (most of the rules' time on large classes).
    let program = arena.alloc(Program {
        statements: Prune { arena, wanted: &wanted.0 }.statements(&program.statements),
        ..program.clone()
    });
    // ponytail: Mago's analyzer trips debug assertions on some code (an empty TUnion::from_vec); no types then
    let analysed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        analyzer.analyze_with_artifacts(program, &mut result)
    }));
    let Ok(Ok(artifacts)) = analysed else {
        return HashMap::new();
    };
    wanted
        .0
        .into_iter()
        .filter_map(|span| Some((span, artifacts.get_expression_type(&span)?.clone())))
        .collect()
}

/// A copy of a program's statements without the functions, class-likes and methods that hold none of `wanted`. Every
/// other statement and member stays, so the kept code's scope is unchanged.
struct Prune<'w, 'arena> {
    arena: &'arena LocalArena,
    wanted: &'w [Span],
}

impl<'arena> Prune<'_, 'arena> {
    fn holds(&self, span: Span) -> bool {
        self.wanted
            .iter()
            .any(|w| span.start.offset <= w.start.offset && w.end.offset <= span.end.offset)
    }

    fn statements(
        &self,
        statements: &Sequence<'arena, Statement<'arena>>,
    ) -> Sequence<'arena, Statement<'arena>> {
        let kept = statements.iter().filter_map(|s| match s {
            Statement::Function(_)
            | Statement::Class(_)
            | Statement::Interface(_)
            | Statement::Trait(_)
            | Statement::Enum(_)
                if !self.holds(s.span()) =>
            {
                None
            }
            Statement::Namespace(n) => Some(Statement::Namespace(Namespace {
                body: match &n.body {
                    NamespaceBody::Implicit(b) => NamespaceBody::Implicit(NamespaceImplicitBody {
                        statements: self.statements(&b.statements),
                        ..b.clone()
                    }),
                    NamespaceBody::BraceDelimited(b) => NamespaceBody::BraceDelimited(Block {
                        statements: self.statements(&b.statements),
                        ..b.clone()
                    }),
                },
                ..n.clone()
            })),
            Statement::Class(c) => Some(Statement::Class(Class {
                members: self.members(&c.members),
                ..c.clone()
            })),
            Statement::Trait(t) => Some(Statement::Trait(Trait {
                members: self.members(&t.members),
                ..t.clone()
            })),
            Statement::Enum(e) => Some(Statement::Enum(Enum {
                members: self.members(&e.members),
                ..e.clone()
            })),
            s => Some(s.clone()),
        });
        Sequence::from_slice(self.arena.alloc_slice_fill_iter(kept))
    }

    fn members(
        &self,
        members: &Sequence<'arena, ClassLikeMember<'arena>>,
    ) -> Sequence<'arena, ClassLikeMember<'arena>> {
        let kept = members
            .iter()
            .filter(|m| !matches!(m, ClassLikeMember::Method(_)) || self.holds(m.span()))
            .cloned();
        Sequence::from_slice(self.arena.alloc_slice_fill_iter(kept))
    }
}

impl<'a, 'arena> Checker<'a, 'arena> {
    /// `$scope->getType($expr)`, as Mago infers it, for the expressions [`typed_expressions()`] lists.
    fn type_of(&self, expr: &Expression) -> Option<&TUnion> {
        self.types.get(&expr.span())
    }

    fn file_name(&self) -> String {
        String::from_utf8_lossy(&self.file.name).into_owned()
    }

    fn report(&mut self, at: impl HasSpan, identifier: &str, message: String, tip: Option<&str>) {
        if self.context.is_some() && !self.in_target {
            return;
        }
        let line = i64::from(self.file.line_number(at.span().start.offset)) + 1;
        self.errors.push(Error {
            identifier: Some(identifier.to_string()),
            tip: tip.map(str::to_string),
            file_path: self
                .context
                .as_ref()
                .map_or_else(|| self.file_name(), |(f, _)| f.clone()),
            trait_file_path: self.context.as_ref().map(|_| self.file_name()),
            ..Error::new(message, self.label.clone(), Some(line))
        });
    }

    /// A `nonIgnorable()` error: no `@phpstan-ignore` or `ignoreErrors` entry removes it.
    fn report_non_ignorable(
        &mut self,
        at: impl HasSpan,
        identifier: &str,
        message: String,
        tip: Option<&str>,
    ) {
        self.report(at, identifier, message, tip);
        self.errors.last_mut().expect("reported").can_be_ignored = false;
    }

    /// `(string) $node->name` after php-parser's NameResolver: an unqualified function or constant name stays as
    /// written (it falls back to the global one at runtime), everything else is the resolved name.
    fn written_or_resolved(&self, id: &Identifier<'arena>) -> String {
        let text = match id {
            Identifier::Local(l) if !self.names.is_imported(&l.span) => l.value,
            _ => self.names.get(&id.span()),
        };
        String::from_utf8_lossy(text).into_owned()
    }

    fn rebind(&mut self, mut e: &Expression) {
        while let Expression::Parenthesized(p) = e {
            e = p.expression;
        }
        if matches!(e, Expression::Closure(_) | Expression::ArrowFunction(_)) {
            self.rebound.insert(e.span());
        }
    }

    fn trait_uses_of(&mut self, class: &str, members: &[ClassLikeMember<'arena>]) {
        for m in members {
            if let ClassLikeMember::TraitUse(u) = m {
                for id in u.trait_names.iter() {
                    let t = self.resolved(id);
                    self.trait_uses.push((class.to_string(), t));
                }
            }
        }
    }

    fn resolved_span(&self, span: Span) -> String {
        String::from_utf8_lossy(self.names.get(&span)).into_owned()
    }

    fn resolved(&self, id: &Identifier<'arena>) -> String {
        String::from_utf8_lossy(self.names.get(&id.span())).into_owned()
    }

    fn class_scope(&self) -> Option<&'a ClassLikeMetadata> {
        let name = self.classes.last()?.name.as_deref()?;
        self.codebase.get_class_like(name.as_bytes())
    }

    fn enter_class(&mut self, name: Option<Span>, members: &[ClassLikeMember<'arena>]) {
        let name = name.map(|s| String::from_utf8_lossy(self.names.get(&s)).into_owned());
        let constructor_guards = members
            .iter()
            .find_map(|m| match m {
                ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
                    match &m.body {
                        MethodBody::Concrete(b) => Some(self.exit_guards(b.statements.iter())),
                        MethodBody::Abstract(_) => None,
                    }
                }
                _ => None,
            })
            .unwrap_or_default();
        self.classes.push(ClassScope {
            name,
            constructor_guards,
        });
    }

    /// Guards the statements' early exits leave at their end: `if (!cond) { return; }` leaves `cond`.
    fn exit_guards<'s>(&self, statements: impl Iterator<Item = &'s Statement<'arena>>) -> Vec<Guard>
    where
        'arena: 's,
    {
        let mut guards = Vec::new();
        for s in statements {
            if let Statement::If(i) = s
                && let IfBody::Statement(b) = &i.body
                && b.else_if_clauses.is_empty()
                && b.else_clause.is_none()
                && terminates(b.statement)
            {
                guards.extend(self.falsy(i.condition));
            }
        }
        guards
    }

    fn truthy(&self, e: &Expression<'arena>) -> Vec<Guard> {
        truthy(
            e,
            &|v| self.vars.get(v).cloned().unwrap_or_default(),
            &|id| self.resolved(id),
        )
    }

    fn falsy(&self, e: &Expression<'arena>) -> Vec<Guard> {
        falsy(
            e,
            &|v| self.vars.get(v).cloned().unwrap_or_default(),
            &|id| self.resolved(id),
        )
    }

    fn mark_top_level(&mut self, mut e: &Expression) {
        while let Expression::Parenthesized(p) = e {
            e = p.expression;
        }
        let span = e.span();
        self.top_level.insert((span.start.offset, span.end.offset));
        // PHPStan passes the statement's context on to a ternary's branches, the right side of `&&` / `||` and
        // the operand of `@`
        match e {
            Expression::Conditional(c) => {
                if let Some(then) = c.then {
                    self.mark_top_level(then);
                }
                self.mark_top_level(c.r#else);
            }
            Expression::Binary(b)
                if matches!(
                    b.operator,
                    BinaryOperator::And(_)
                        | BinaryOperator::LowAnd(_)
                        | BinaryOperator::Or(_)
                        | BinaryOperator::LowOr(_)
                ) =>
            {
                self.mark_top_level(b.rhs);
            }
            Expression::UnaryPrefix(u)
                if matches!(u.operator, UnaryPrefixOperator::ErrorControl(_)) =>
            {
                self.mark_top_level(u.operand);
            }
            _ => {}
        }
    }

    fn is_top_level(&self, span: Span) -> bool {
        self.top_level
            .contains(&(span.start.offset, span.end.offset))
    }

    /// `isInClassExists($name)`.
    fn in_class_exists(&self, name: &str) -> bool {
        self.in_exists_call > 0
            || self
                .guards
                .contains(&Guard::Class(name.trim_start_matches('\\').to_string()))
    }

    /// `isInFunctionExists($name)`.
    fn in_function_exists(&self, name: &str) -> bool {
        self.guards
            .contains(&Guard::Function(name.trim_start_matches('\\').to_string()))
    }

    /// Walks `body` with `extra` guards on top of the current ones.
    fn with_guards(&mut self, extra: Vec<Guard>, body: impl FnOnce(&mut Self)) {
        let mark = self.guards.len();
        self.guards.extend(extra);
        body(self);
        self.guards.truncate(mark);
    }

    /// A function or method body: a fresh scope (the constructor's remembered guards in a method).
    fn enter_function(&mut self, guards: Vec<Guard>) {
        self.saved.push(Saved {
            guards: std::mem::replace(&mut self.guards, guards),
            marks: std::mem::take(&mut self.marks),
            vars: std::mem::take(&mut self.vars),
        });
    }

    fn leave_function(&mut self) {
        let s = self.saved.pop().expect("entered");
        self.guards = s.guards;
        self.marks = s.marks;
        self.vars = s.vars;
    }

    fn walk_branch(&mut self, extra: Vec<Guard>, statements: &[&Statement<'arena>]) -> bool {
        self.with_guards(extra, |c| {
            c.marks.push(c.guards.len());
            for s in statements {
                c.walk_statement(s, &mut ());
            }
            let mark = c.marks.pop().expect("pushed");
            c.guards.truncate(mark);
        });
        statements_terminate(statements.iter().copied())
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Checker<'_, 'arena> {
    fn walk_in_class(&mut self, class: &'ast Class<'arena>, _: &mut ()) {
        let current = format!("Class {}", self.resolved_span(class.name.span));
        names::class_hierarchy(
            self,
            class.span(),
            &current,
            class.extends.as_ref(),
            class.implements.as_ref(),
            false,
        );
        if self.context.is_none() {
            let name = self.resolved_span(class.name.span);
            self.trait_uses_of(&name, class.members.as_slice());
        }
        self.enter_class(Some(class.name.span), class.members.as_slice());
    }
    fn walk_out_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {
        self.classes.pop();
    }
    fn walk_in_interface(&mut self, i: &'ast Interface<'arena>, _: &mut ()) {
        let current = format!("Interface {}", self.resolved_span(i.name.span));
        names::class_hierarchy(self, i.span(), &current, i.extends.as_ref(), None, true);
        self.enter_class(Some(i.name.span), i.members.as_slice());
    }
    fn walk_out_interface(&mut self, _: &'ast Interface<'arena>, _: &mut ()) {
        self.classes.pop();
    }
    fn walk_in_enum(&mut self, e: &'ast Enum<'arena>, _: &mut ()) {
        let current = format!("Enum {}", self.resolved_span(e.name.span));
        names::class_hierarchy(self, e.span(), &current, None, e.implements.as_ref(), false);
        if self.context.is_none() {
            let name = self.resolved_span(e.name.span);
            self.trait_uses_of(&name, e.members.as_slice());
        }
        self.enter_class(Some(e.name.span), e.members.as_slice());
    }
    fn walk_out_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {
        self.classes.pop();
    }
    // ponytail: trait bodies have no class scope; PHPStan analyses them once per using class (add with traits).
    /// A trait is analysed only in the context of each class using it (see [`in_trait_context()`]).
    fn walk_trait(&mut self, t: &'ast Trait<'arena>, c: &mut ()) {
        let start = i64::from(self.file.line_number(t.span().start.offset)) + 1;
        let end = i64::from(self.file.line_number(t.span().end.offset)) + 1;
        self.trait_lines.push((start, end));
        let Some((_, class)) = self.context.clone() else {
            return;
        };
        let name = self.resolved_span(t.name.span);
        if !self
            .context_trait
            .as_deref()
            .is_some_and(|n| n.eq_ignore_ascii_case(&name))
        {
            return;
        }
        self.trait_uses_of(&class, t.members.as_slice());
        let saved = self.classes.len();
        self.enter_class(None, t.members.as_slice());
        self.classes[saved].name = Some(class);
        self.in_target = true;
        for member in t.members.iter() {
            self.walk_class_like_member(member, c);
        }
        self.in_target = false;
        self.classes.truncate(saved);
    }
    fn walk_in_anonymous_class(&mut self, a: &'ast AnonymousClass<'arena>, _: &mut ()) {
        names::class_hierarchy(
            self,
            a.span(),
            "Anonymous class",
            a.extends.as_ref(),
            a.implements.as_ref(),
            false,
        );
        self.enter_class(None, a.members.as_slice());
    }

    fn walk_in_function(&mut self, f: &'ast Function<'arena>, _: &mut ()) {
        let name = self.resolved_span(f.name.span);
        names::typehints(
            self,
            names::FunctionKind::Function(&name),
            &f.parameter_list,
            f.return_type_hint.as_ref(),
        );
        self.enter_function(Vec::new());
        self.functions.push(FunctionScope {
            name: String::from_utf8_lossy(f.name.value).into_owned(),
            method_static: None,
        });
    }
    fn walk_out_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {
        self.leave_function();
        self.functions.pop();
    }
    fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
        // ponytail: methods of traits and anonymous classes (no class scope yet) unchecked
        for p in m
            .parameter_list
            .parameters
            .iter()
            .filter(|p| p.is_promoted_property())
        {
            names::property_type(self, p.span(), p.hint.as_ref(), &[p.variable.name]);
        }
        if let Some(class) = self.class_scope() {
            let class = String::from_utf8_lossy(class.original_name.as_bytes()).into_owned();
            let method = String::from_utf8_lossy(m.name.value).into_owned();
            names::typehints(
                self,
                names::FunctionKind::Method(&class, &method),
                &m.parameter_list,
                m.return_type_hint.as_ref(),
            );
        }
        let guards = if m.name.value.eq_ignore_ascii_case(b"__construct") {
            Vec::new()
        } else {
            self.classes
                .last()
                .map(|c| c.constructor_guards.clone())
                .unwrap_or_default()
        };
        self.enter_function(guards);
        self.functions.push(FunctionScope {
            name: String::from_utf8_lossy(m.name.value).into_owned(),
            method_static: Some(m.modifiers.iter().any(|m| matches!(m, Modifier::Static(_)))),
        });
    }
    fn walk_out_method(&mut self, _: &'ast Method<'arena>, _: &mut ()) {
        self.leave_function();
        self.functions.pop();
    }

    fn walk_in_block(&mut self, _: &'ast Block<'arena>, _: &mut ()) {
        self.marks.push(self.guards.len());
    }
    fn walk_out_block(&mut self, _: &'ast Block<'arena>, _: &mut ()) {
        let mark = self.marks.pop().expect("pushed");
        self.guards.truncate(mark);
    }

    fn walk_if(&mut self, r#if: &'ast If<'arena>, c: &mut ()) {
        self.walk_expression(r#if.condition, c);
        // (condition, statements) per branch; `None` is the `else`
        let mut branches: Vec<(Option<&Expression<'arena>>, Vec<&Statement<'arena>>)> = Vec::new();
        let has_else = match &r#if.body {
            IfBody::Statement(b) => {
                branches.push((Some(r#if.condition), vec![b.statement]));
                branches.extend(
                    b.else_if_clauses
                        .iter()
                        .map(|e| (Some(e.condition), vec![e.statement])),
                );
                if let Some(e) = &b.else_clause {
                    branches.push((None, vec![e.statement]));
                }
                b.else_clause.is_some()
            }
            IfBody::ColonDelimited(b) => {
                branches.push((Some(r#if.condition), b.statements.iter().collect()));
                branches.extend(
                    b.else_if_clauses
                        .iter()
                        .map(|e| (Some(e.condition), e.statements.iter().collect())),
                );
                if let Some(e) = &b.else_clause {
                    branches.push((None, e.statements.iter().collect()));
                }
                b.else_clause.is_some()
            }
        };
        let mut previous_false = Vec::new();
        let mut survivors: Vec<Vec<Guard>> = Vec::new();
        for (i, (cond, statements)) in branches.into_iter().enumerate() {
            let mut entry = previous_false.clone();
            if let Some(cond) = cond {
                if i > 0 {
                    self.with_guards(previous_false.clone(), |s| s.walk_expression(cond, &mut ()));
                }
                entry.extend(self.truthy(cond));
                previous_false.extend(self.falsy(cond));
            }
            if !self.walk_branch(entry.clone(), &statements) {
                survivors.push(entry);
            }
        }
        if !has_else {
            survivors.push(previous_false);
        }
        // after the `if`: what holds in every branch that falls through
        if let Some((first, rest)) = survivors.split_first() {
            let after: Vec<Guard> = first
                .iter()
                .filter(|g| rest.iter().all(|r| r.contains(g)))
                .cloned()
                .collect();
            self.guards.extend(after);
        }
    }

    fn walk_conditional(&mut self, cond: &'ast Conditional<'arena>, c: &mut ()) {
        self.walk_expression(cond.condition, c);
        if let Some(then) = cond.then {
            let t = self.truthy(cond.condition);
            self.with_guards(t, |s| s.walk_expression(then, &mut ()));
        }
        let f = self.falsy(cond.condition);
        self.with_guards(f, |s| s.walk_expression(cond.r#else, &mut ()));
    }

    fn walk_binary(&mut self, b: &'ast Binary<'arena>, c: &mut ()) {
        if matches!(b.operator, BinaryOperator::Instanceof(_)) {
            names::instanceof(self, b.rhs);
        }
        self.walk_expression(b.lhs, c);
        let extra = match b.operator {
            BinaryOperator::And(_) | BinaryOperator::LowAnd(_) => self.truthy(b.lhs),
            BinaryOperator::Or(_) | BinaryOperator::LowOr(_) => self.falsy(b.lhs),
            _ => Vec::new(),
        };
        self.with_guards(extra, |s| s.walk_expression(b.rhs, &mut ()));
    }

    fn walk_match(&mut self, m: &'ast Match<'arena>, c: &mut ()) {
        self.walk_expression(m.expression, c);
        let on_true = matches!(m.expression, Expression::Literal(Literal::True(_)));
        for arm in m.arms.iter() {
            match arm {
                MatchArm::Expression(arm) => {
                    for cond in arm.conditions.iter() {
                        self.walk_expression(cond, c);
                    }
                    let extra = match (
                        on_true,
                        arm.conditions.iter().collect::<Vec<_>>().as_slice(),
                    ) {
                        (true, [one]) => self.truthy(one),
                        _ => Vec::new(),
                    };
                    self.mark_top_level(arm.expression);
                    self.with_guards(extra, |s| s.walk_expression(arm.expression, &mut ()));
                }
                MatchArm::Default(arm) => {
                    self.mark_top_level(arm.expression);
                    self.walk_expression(arm.expression, c);
                }
            }
        }
    }

    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        if let Expression::Variable(Variable::Direct(v)) = a.lhs {
            let guards = self.truthy(a.rhs);
            if guards.is_empty() {
                self.vars.remove(v.name);
            } else {
                self.vars.insert(v.name.to_vec(), guards);
            }
        }
    }
    fn walk_out_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {
        self.classes.pop();
    }

    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        calls::function_call(self, call);
        self.in_exists_call += usize::from(calls::is_class_exists_family(self, call));
    }
    fn walk_out_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        self.in_exists_call -= usize::from(calls::is_class_exists_family(self, call));
    }

    fn walk_in_instantiation(&mut self, new: &'ast Instantiation<'arena>, _: &mut ()) {
        classes::instantiation(self, new);
    }

    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        if let ClassLikeMemberSelector::Identifier(m) = &call.method
            && [&b"call"[..], b"bind", b"bindto"].contains(&m.value.to_ascii_lowercase().as_slice())
        {
            self.rebind(call.object);
        }
        calls::method_call(self, call);
    }
    fn walk_in_pipe(&mut self, p: &'ast Pipe<'arena>, _: &mut ()) {
        self.piped.insert(p.callable.span());
        calls::pipe(self, p);
    }
    fn walk_in_function_partial_application(
        &mut self,
        f: &'ast FunctionPartialApplication<'arena>,
        _: &mut (),
    ) {
        calls::function_callable(self, f);
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        if let (Expression::Identifier(id), ClassLikeMemberSelector::Identifier(m)) =
            (call.class, &call.method)
            && self.resolved(id).eq_ignore_ascii_case("Closure")
            && m.value.eq_ignore_ascii_case(b"bind")
            && let Some(Argument::Positional(a)) = call.argument_list.arguments.first()
        {
            self.rebind(a.value);
        }
        calls::static_method_call(self, call);
    }
    // ponytail: a rebound closure gets no class scope (PHPStan checks it against the bound object's class)
    fn walk_out_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        if self.rebound.contains(&f.span()) {
            self.classes.pop();
        }
    }
    fn walk_out_arrow_function(&mut self, f: &'ast ArrowFunction<'arena>, _: &mut ()) {
        if self.rebound.contains(&f.span()) {
            self.classes.pop();
        }
    }

    fn walk_in_statement_expression(&mut self, s: &'ast ExpressionStatement<'arena>, _: &mut ()) {
        self.mark_top_level(s.expression);
    }
    fn walk_in_for(&mut self, f: &'ast For<'arena>, _: &mut ()) {
        for e in f.initializations.iter().chain(f.increments.iter()) {
            self.mark_top_level(e);
        }
    }
    fn walk_in_use(&mut self, u: &'ast Use<'arena>, _: &mut ()) {
        names::use_functions(self, u);
    }
    fn walk_in_property(&mut self, p: &'ast Property<'arena>, _: &mut ()) {
        let names: Vec<&[u8]> = p.variables().iter().map(|v| v.name).collect();
        names::property_type(self, p.span(), p.hint(), &names);
    }
    fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        if self.rebound.contains(&f.span()) {
            self.classes.push(ClassScope {
                name: None,
                constructor_guards: Vec::new(),
            });
        }
        names::typehints(
            self,
            names::FunctionKind::Anonymous,
            &f.parameter_list,
            f.return_type_hint.as_ref(),
        );
    }
    fn walk_in_class_constant_access(&mut self, a: &'ast ClassConstantAccess<'arena>, _: &mut ()) {
        names::class_constant(self, a);
    }
    fn walk_in_try_catch_clause(&mut self, clause: &'ast TryCatchClause<'arena>, _: &mut ()) {
        names::catch(self, clause);
    }
    fn walk_in_arrow_function(&mut self, f: &'ast ArrowFunction<'arena>, _: &mut ()) {
        if self.rebound.contains(&f.span()) {
            self.classes.push(ClassScope {
                name: None,
                constructor_guards: Vec::new(),
            });
        }
        names::typehints(
            self,
            names::FunctionKind::Anonymous,
            &f.parameter_list,
            f.return_type_hint.as_ref(),
        );
        self.mark_top_level(f.expression);
    }
    fn walk_in_match_expression_arm(&mut self, a: &'ast MatchExpressionArm<'arena>, _: &mut ()) {
        self.mark_top_level(a.expression);
    }
    fn walk_in_match_default_arm(&mut self, a: &'ast MatchDefaultArm<'arena>, _: &mut ()) {
        self.mark_top_level(a.expression);
    }
}

impl<'a> Checker<'a, '_> {
    /// `ReflectionProvider::getFunction()`: the display name and the variants.
    fn function(&self, name: &[u8]) -> Option<(String, Vec<params::Acceptor>)> {
        let lower = String::from_utf8_lossy(name).to_ascii_lowercase();
        if let Some(f) = self
            .codebase
            .get_function(name)
            .filter(|f| !f.flags.is_built_in())
        {
            return Some((
                String::from_utf8_lossy(f.original_name.as_bytes()).into_owned(),
                vec![params::Acceptor::of(f)],
            ));
        }
        let f = builtins().functions.get(&lower)?;
        Some((
            f.name.clone(),
            f.variants
                .iter()
                .map(params::Acceptor::of_builtin)
                .collect(),
        ))
    }

    /// `ReflectionProvider::getClass()`, or `None` when PHPStan has no such class.
    fn class_like(&self, name: &str) -> Option<&'a ClassLikeMetadata> {
        let class = self.codebase.get_class_like(name.as_bytes())?;
        (!class.flags.is_built_in() || builtins().classes.contains_key(&name.to_ascii_lowercase()))
            .then_some(class)
    }
}

fn builtins() -> &'static pun_phpstan::symbols::Symbols {
    pun_phpstan::symbols::symbols(pun_phpstan::symbols::PhpVersion::Php85)
}
