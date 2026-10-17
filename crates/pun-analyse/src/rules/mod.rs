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

use mago_allocator::LocalArena;
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
use crate::codebase::read;
use scope::{Guard, falsy, statements_terminate, terminates, truthy};

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
    errors: Vec<Error>,
}

/// The file's errors after its `@phpstan-ignore*` comments (LocalIgnoresProcessor), with the unmatched ones.
pub fn check_file(path: &Path, codebase: &CodebaseMetadata, project: &Project) -> Vec<Error> {
    let Some(file) = read(path, FileType::Host) else {
        return Vec::new();
    };
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let mut checker = Checker {
        file: &file,
        types: infer(
            &arena,
            &file,
            program,
            &names,
            codebase,
            project.php_version,
        ),
        names: &names,
        codebase,
        classes: Vec::new(),
        guards: Vec::new(),
        marks: Vec::new(),
        vars: HashMap::new(),
        saved: Vec::new(),
        functions: Vec::new(),
        in_exists_call: 0,
        top_level: HashSet::new(),
        errors: Vec::new(),
    };
    checker.walk_program(program, &mut ());
    let name = checker.file_name();
    let (mut lines, parse) = lines_to_ignore(&file.contents);
    let mut unmatched = lines.clone();
    let mut errors = checker.errors;
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
    out
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

impl<'a, 'arena> Checker<'a, 'arena> {
    /// `$scope->getType($expr)`, as Mago infers it, for the expressions [`typed_expressions()`] lists.
    fn type_of(&self, expr: &Expression) -> Option<&TUnion> {
        self.types.get(&expr.span())
    }

    fn file_name(&self) -> String {
        String::from_utf8_lossy(&self.file.name).into_owned()
    }

    fn report(&mut self, at: impl HasSpan, identifier: &str, message: String, tip: Option<&str>) {
        let line = i64::from(self.file.line_number(at.span().start.offset)) + 1;
        self.errors.push(Error {
            identifier: Some(identifier.to_string()),
            tip: tip.map(str::to_string),
            ..Error::new(message, self.file_name(), Some(line))
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
        self.enter_class(Some(e.name.span), e.members.as_slice());
    }
    fn walk_out_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {
        self.classes.pop();
    }
    // ponytail: trait bodies have no class scope; PHPStan analyses them once per using class (add with traits).
    fn walk_in_trait(&mut self, t: &'ast Trait<'arena>, _: &mut ()) {
        self.enter_class(None, t.members.as_slice());
    }
    fn walk_out_trait(&mut self, _: &'ast Trait<'arena>, _: &mut ()) {
        self.classes.pop();
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
        calls::method_call(self, call);
    }
    fn walk_in_pipe(&mut self, p: &'ast Pipe<'arena>, _: &mut ()) {
        calls::pipe(self, p);
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        calls::static_method_call(self, call);
    }

    fn walk_in_statement_expression(&mut self, s: &'ast ExpressionStatement<'arena>, _: &mut ()) {
        self.mark_top_level(s.expression);
    }
    fn walk_in_for(&mut self, f: &'ast For<'arena>, _: &mut ()) {
        for e in f.initializations.iter().chain(f.increments.iter()) {
            self.mark_top_level(e);
        }
    }
    fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
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
