//! What a by-reference parameter is left holding. Level 3's ParameterOutAssignedTypeRule (every write to it) and
//! ParameterOutExecutionEndTypeRule (every execution end, when it has a `@param-out`) over ParameterOutTypeCheck,
//! level 4's TooWideFunctionParameterOutTypeRule and TooWideMethodParameterOutTypeRule over
//! TooWideParameterOutTypeCheck (the merged scope of the execution ends and returns), and level 2's
//! IncompatibleSelfOutTypeRule.
//!
//! The variable's types come from Mago's analyser through [`Observer`]: before and after every statement of a
//! function-like with a by-reference parameter, and after every write expression there.
//!
//! ponytail: `ParameterVariableOriginalValueExpr` (an execution end skipped once every path to it wrote the
//! parameter) is read from the syntax: a statement list's earlier statements that always write it, and an `if` all of
//! whose branches do. Destructuring targets are not write sites.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use mago_analyzer::plugin::{
    ExpressionHook, HookAction, HookContext, HookResult, Provider, ProviderMeta, StatementHook,
};
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::symbols;
use pun_phpstan::types::{
    Level, NeverKind, StrictMixed, Type, accepts_reasons, ancestor_args, is_super_type_of, recommended_level, union,
};

use super::super::assign::{Callee, by_ref_arguments, by_ref_call_arguments};
use super::super::declarations::has_tag;
use super::super::missing_types::declaring;
use super::super::{Checker, declared, generics};
use super::flow::{self, list};
use super::missing_return::{late, method_name};

pub const IDENTIFIERS: &[&str] = &[
    "paramOut.type",
    "parameterByRef.type",
    "paramOut.unusedType",
    "paramOut.nestedUnusedType",
    "paramOut.tooWideBool",
    "parameterByRef.unusedType",
    "parameterByRef.nestedUnusedType",
    "parameterByRef.tooWideBool",
    "selfOut.type",
    "selfOut.static",
    "selfOut.unresolvableType",
];

/// The by-reference parameters' types, by the span of the statement or expression they were read at: (name with
/// `$`, Mago's type, `None` when undefined).
pub(in super::super) type Read = Vec<(Vec<u8>, Option<TUnion>)>;

/// What [`Observer`] read.
#[derive(Default)]
pub(in super::super) struct Observed {
    pub before: HashMap<Span, Read>,
    pub after: HashMap<Span, Read>,
    pub expressions: HashMap<Span, Read>,
}

/// The function-like bodies with by-reference parameters: (body, parameter names with `$`).
pub(in super::super) fn watched(program: &Program) -> Vec<(Span, Vec<Vec<u8>>)> {
    struct W(Vec<(Span, Vec<Vec<u8>>)>);
    fn names(l: &FunctionLikeParameterList) -> Vec<Vec<u8>> {
        l.parameters.iter().filter(|p| p.ampersand.is_some()).map(|p| p.variable.name.to_vec()).collect()
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for W {
        fn walk_in_function(&mut self, f: &'ast Function<'arena>, _: &mut ()) {
            let n = names(&f.parameter_list);
            if !n.is_empty() {
                self.0.push((f.body.span(), n));
            }
        }
        fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
            if let MethodBody::Concrete(b) = &m.body {
                let n = names(&m.parameter_list);
                if !n.is_empty() {
                    self.0.push((b.span(), n));
                }
            }
        }
    }
    let mut w = W(Vec::new());
    w.walk_program(program, &mut ());
    w.0
}

/// Reads the watched parameters around the statements and write expressions of their bodies.
#[derive(Clone)]
pub(in super::super) struct Observer {
    pub bodies: Arc<Vec<(Span, Vec<Vec<u8>>)>>,
    pub found: Arc<Mutex<Observed>>,
}

impl Observer {
    fn read(&self, span: Span, context: &HookContext<'_, '_>) -> Option<Read> {
        let (_, names) = self
            .bodies
            .iter()
            .filter(|(b, _)| b.start.offset <= span.start.offset && span.end.offset <= b.end.offset)
            .min_by_key(|(b, _)| b.end.offset - b.start.offset)?;
        Some(names.iter().map(|n| (n.clone(), context.get_variable_type(n).map(|t| (**t).clone()))).collect())
    }
}

impl Provider for Observer {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new("pun-param-out", "param-out", "by-reference parameter types");
        &META
    }
}

impl StatementHook for Observer {
    fn before_statement(&self, s: &Statement<'_>, context: &mut HookContext<'_, '_>) -> HookResult<HookAction> {
        if let Some(r) = self.read(s.span(), context) {
            self.found.lock().expect("no hook panics while holding the lock").before.insert(s.span(), r);
        }
        Ok(HookAction::Continue)
    }
    fn after_statement(&self, s: &Statement<'_>, context: &mut HookContext<'_, '_>) -> HookResult<()> {
        if let Some(r) = self.read(s.span(), context) {
            self.found.lock().expect("no hook panics while holding the lock").after.insert(s.span(), r);
        }
        Ok(())
    }
}

impl ExpressionHook for Observer {
    fn is_observer(&self) -> bool {
        true
    }
    fn after_expression(&self, e: &Expression<'_>, context: &mut HookContext<'_, '_>) -> HookResult<()> {
        let write = match e {
            Expression::Assignment(_) | Expression::Call(_) | Expression::Instantiation(_) | Expression::UnaryPostfix(_) => true,
            Expression::UnaryPrefix(u) => u.operator.is_increment_or_decrement(),
            _ => false,
        };
        if write && let Some(r) = self.read(e.span(), context) {
            self.found.lock().expect("no hook panics while holding the lock").expressions.insert(e.span(), r);
        }
        Ok(())
    }
}

/// Where a write's new type is read.
#[derive(Clone)]
enum At {
    Expression(Span),
    After(Span),
    Before(Span),
    /// a caught exception's
    Type(Type),
}

/// A VariableAssignNode of a by-reference parameter: its name with `$`, the variable written (its line is the
/// error's) and where its type is read.
struct Site {
    name: Vec<u8>,
    variable: Span,
    at: At,
}

/// The direct variable an assignment target writes through (`$p`, `$p[..]`, `$p[]`).
fn root<'a, 'arena>(mut e: &'a Expression<'arena>) -> Option<&'a DirectVariable<'arena>> {
    loop {
        match e {
            Expression::Parenthesized(p) => e = p.expression,
            Expression::ArrayAccess(a) => e = a.array,
            Expression::ArrayAppend(a) => e = a.array,
            Expression::Variable(Variable::Direct(v)) => return Some(v),
            _ => return None,
        }
    }
}

/// The write sites in a body, not in its closures, arrow functions or declarations.
struct Sites<'c, 'a, 'arena> {
    c: &'c Checker<'a, 'arena>,
    names: &'c [Vec<u8>],
    out: Vec<Site>,
}

impl Sites<'_, '_, '_> {
    fn push(&mut self, v: &DirectVariable, at: At) {
        if self.names.iter().any(|n| n.as_slice() == v.name) {
            self.out.push(Site { name: v.name.to_vec(), variable: v.span(), at });
        }
    }
    fn arguments(&mut self, refs: crate::bitset::BitSet, args: &ArgumentList, call: Span) {
        for (a, r) in args.arguments.iter().zip(refs) {
            if r && let Expression::Variable(Variable::Direct(v)) = a.value() {
                self.push(v, At::Expression(call));
            }
        }
    }
    fn first(&mut self, v: &DirectVariable, stmts: &[Statement]) {
        if let Some(s) = list(stmts).first() {
            self.push(v, At::Before(s.span()));
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Sites<'_, '_, 'arena> {
    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        if let Some(v) = root(a.lhs) {
            self.push(v, At::Expression(a.span()));
        }
    }
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        if u.operator.is_increment_or_decrement()
            && let Expression::Variable(Variable::Direct(v)) = u.operand
        {
            self.push(v, At::Expression(u.span()));
        }
    }
    fn walk_in_unary_postfix(&mut self, u: &'ast UnaryPostfix<'arena>, _: &mut ()) {
        if let Expression::Variable(Variable::Direct(v)) = u.operand {
            self.push(v, At::Expression(u.span()));
        }
    }
    fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, _: &mut ()) {
        for value in u.values.iter() {
            if !matches!(value, Expression::Variable(_))
                && let Some(v) = root(value)
            {
                self.push(v, At::After(u.span()));
            }
        }
    }
    fn walk_in_function_call(&mut self, f: &'ast FunctionCall<'arena>, _: &mut ()) {
        let refs = by_ref_arguments(self.c, f);
        self.arguments(refs, &f.argument_list, f.span());
    }
    fn walk_in_method_call(&mut self, m: &'ast MethodCall<'arena>, _: &mut ()) {
        let refs = by_ref_call_arguments(self.c, Callee::Method(m), &m.argument_list);
        self.arguments(refs, &m.argument_list, m.span());
    }
    fn walk_in_static_method_call(&mut self, m: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        let refs = by_ref_call_arguments(self.c, Callee::StaticMethod(m), &m.argument_list);
        self.arguments(refs, &m.argument_list, m.span());
    }
    fn walk_in_instantiation(&mut self, n: &'ast Instantiation<'arena>, _: &mut ()) {
        if let Some(args) = &n.argument_list {
            let refs = by_ref_call_arguments(self.c, Callee::New(n), args);
            self.arguments(refs, args, n.span());
        }
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        let targets = match &f.target {
            ForeachTarget::Value(v) => vec![v.value],
            ForeachTarget::KeyValue(kv) => vec![kv.key, kv.value],
        };
        for t in targets {
            let t = match t {
                Expression::UnaryPrefix(u) if u.operator.is_reference() => u.operand,
                t => t,
            };
            if let Expression::Variable(Variable::Direct(v)) = t {
                self.first(v, f.body.statements());
            }
        }
    }
    fn walk_in_try_catch_clause(&mut self, k: &'ast TryCatchClause<'arena>, _: &mut ()) {
        if let Some(v) = &k.variable {
            let class = self.c.class_scope();
            let t = declared::with_classes(self.c, || late(self.c, &declared::native(self.c, &k.hint, class), class, false));
            self.push(v, At::Type(t));
        }
    }
    fn walk_closure(&mut self, _: &'ast Closure<'arena>, _: &mut ()) {}
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {}
    fn walk_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {}
    fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {}
    fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
    fn walk_interface(&mut self, _: &'ast Interface<'arena>, _: &mut ()) {}
    fn walk_trait(&mut self, _: &'ast Trait<'arena>, _: &mut ()) {}
    fn walk_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {}
}

/// A by-reference parameter as the function reflection gives it.
struct Param {
    name: String,
    /// `getType()`
    ty: Type,
    /// `getOutType()`, resolved
    out: Option<Type>,
    variadic: bool,
}

impl Param {
    /// The variable's type on entry (a variadic one packs the arguments).
    fn entry(&self) -> Type {
        if self.variadic { Type::array(pun_phpstan::types::mixed(), self.ty.clone()) } else { self.ty.clone() }
    }
}

/// A function-like with by-reference parameters.
struct Body<'s, 'arena> {
    block: &'s Block<'arena>,
    span: Span,
    /// `function foo()` / `method Foo::bar()`, lower-case first
    of: String,
    params: Vec<Param>,
    /// TooWide*ParameterOutTypeRule checks it
    too_wide: bool,
}

fn inside(outer: Span, inner: Span) -> bool {
    outer.start.offset <= inner.start.offset && inner.end.offset <= outer.end.offset
}

/// The statements (not in closures or declarations) and whether the earlier statements of every path to each
/// always wrote the parameter (`ParameterVariableOriginalValueExpr` gone).
struct Paths<'s, 'arena> {
    statements: Vec<&'s Statement<'arena>>,
    written_before: HashMap<Span, bool>,
}

impl<'s, 'arena> Paths<'s, 'arena> {
    /// The statement lists of a statement and the condition-free part they inherit from it.
    fn children(s: &'s Statement<'arena>) -> Vec<Vec<&'s Statement<'arena>>> {
        match s {
            Statement::Block(b) => vec![list(b.statements.as_slice())],
            Statement::If(i) => match &i.body {
                IfBody::Statement(b) => {
                    let mut v = vec![flow::body(b.statement)];
                    v.extend(b.else_if_clauses.iter().map(|e| flow::body(e.statement)));
                    v.extend(b.else_clause.iter().map(|e| flow::body(e.statement)));
                    v
                }
                IfBody::ColonDelimited(b) => {
                    let mut v = vec![list(b.statements.as_slice())];
                    v.extend(b.else_if_clauses.iter().map(|e| list(e.statements.as_slice())));
                    v.extend(b.else_clause.iter().map(|e| list(e.statements.as_slice())));
                    v
                }
            },
            Statement::Foreach(f) => vec![list(f.body.statements())],
            Statement::While(w) => vec![list(w.body.statements())],
            Statement::For(f) => vec![list(f.body.statements())],
            Statement::DoWhile(d) => vec![flow::body(d.statement)],
            Statement::Switch(sw) => sw.body.cases().iter().map(|c| list(c.statements())).collect(),
            Statement::Try(t) => {
                let mut v = vec![list(t.block.statements.as_slice())];
                v.extend(t.catch_clauses.iter().map(|k| list(k.block.statements.as_slice())));
                v.extend(t.finally_clause.iter().map(|f| list(f.block.statements.as_slice())));
                v
            }
            Statement::Declare(d) => match &d.body {
                DeclareBody::Statement(s) => vec![flow::body(s)],
                DeclareBody::ColonDelimited(b) => vec![list(b.statements.as_slice())],
            },
            _ => Vec::new(),
        }
    }

    /// The statement always writes it (`sites` are the writes' variables).
    fn always(s: &'s Statement<'arena>, sites: &[Span]) -> bool {
        let any = |at: Span| sites.iter().any(|w| inside(at, *w));
        match s {
            Statement::Expression(_) | Statement::Return(_) | Statement::Unset(_) | Statement::Echo(_) => any(s.span()),
            Statement::Block(b) => list(b.statements.as_slice()).iter().any(|s| Self::always(s, sites)),
            Statement::If(i) => {
                let has_else = match &i.body {
                    IfBody::Statement(b) => b.else_clause.is_some(),
                    IfBody::ColonDelimited(b) => b.else_clause.is_some(),
                };
                any(i.condition.span())
                    || (has_else && Self::children(s).iter().all(|l| l.iter().any(|s| Self::always(s, sites))))
            }
            // (the catches write it when each catches into it or its body does)
            Statement::Try(t) => {
                list(t.block.statements.as_slice()).iter().any(|s| Self::always(s, sites))
                    || (!t.catch_clauses.is_empty() && t.catch_clauses.iter().all(|k| any(k.span())))
            }
            _ => false,
        }
    }

    fn walk(&mut self, stmts: Vec<&'s Statement<'arena>>, mut written: bool, sites: &[Span]) {
        for s in stmts {
            self.statements.push(s);
            self.written_before.insert(s.span(), written);
            for l in Self::children(s) {
                self.walk(l, written, sites);
            }
            written |= Self::always(s, sites);
        }
    }

    fn of(b: &'s Block<'arena>, sites: &[Span]) -> Self {
        let mut p = Paths { statements: Vec::new(), written_before: HashMap::new() };
        p.walk(list(b.statements.as_slice()), false, sites);
        p
    }

    fn get(&self, span: Span) -> Option<&'s Statement<'arena>> {
        self.statements.iter().copied().find(|s| s.span() == span)
    }

    /// The innermost statement around `at`.
    fn around(&self, at: Span) -> Option<&'s Statement<'arena>> {
        self.statements
            .iter()
            .copied()
            .filter(|s| s.span() != at && inside(s.span(), at))
            .min_by_key(|s| s.span().end.offset - s.span().start.offset)
    }
}

/// Where an execution end's scope is read.
enum EndAt {
    After(Span),
    Before(Span),
    Entry,
}

/// Which statement tells whether every path to an execution end wrote the parameter.
#[derive(Clone, Copy)]
enum Written {
    /// the statements before it or it always did
    Through(Span),
    /// the statements before it did
    Before(Span),
    Never,
}

/// An ExecutionEndNode at `at`: where its scope is read (the end statement's, an empty branch's (its `if`'s before),
/// a trailing comment's (the statement before it), an empty body's (the function's entry)), and whether it ends with
/// an expression statement typed explicit `never`.
fn end_at(c: &Checker, paths: &Paths, body: Span, at: Span) -> Option<(EndAt, Written, bool)> {
    if at == body || !inside(body, at) {
        return Some((EndAt::Entry, Written::Never, false));
    }
    if let Some(s) = paths.get(at) {
        let never = matches!(s, Statement::Expression(_)) && flow_always(c, s);
        return Some((EndAt::After(at), Written::Through(at), never));
    }
    let around = paths.around(at);
    let text = &c.file.contents[at.start.offset as usize..];
    let keyword = text.starts_with(b"if") || text.starts_with(b"elseif") || text.starts_with(b"else");
    if keyword && let Some(i @ Statement::If(_)) = around {
        return Some((EndAt::Before(i.span()), Written::Before(i.span()), false));
    }
    // a `Nop`: the last statement before it in its list
    let scope = around.map_or(body, HasSpan::span);
    let last = paths
        .statements
        .iter()
        .copied()
        .filter(|s| inside(scope, s.span()) && s.span() != scope && s.span().end.offset <= at.start.offset)
        .max_by_key(|s| (s.span().end.offset, u32::MAX - s.span().start.offset));
    match (last, around) {
        (Some(s), _) => Some((EndAt::After(s.span()), Written::Through(s.span()), false)),
        (None, Some(i @ Statement::If(_))) => Some((EndAt::Before(i.span()), Written::Before(i.span()), false)),
        (None, None) => Some((EndAt::Entry, Written::Never, false)),
        (None, Some(_)) => None,
    }
}

fn flow_always(c: &Checker, s: &Statement) -> bool {
    let results = flow::Results {
        types: &c.types,
        call_terminations: Some(&c.call_terminations),
        contents: &c.file.contents,
        codebase: Some(c.codebase),
    };
    results.statements(&[s]).always
}

impl Checker<'_, '_> {
    fn param_out_read(&self, at: &EndAt, name: &str, entry: &Type) -> Option<Type> {
        let read = match at {
            EndAt::After(s) => self.variables.param_out.after.get(s),
            EndAt::Before(s) => self.variables.param_out.before.get(s),
            EndAt::Entry => return Some(entry.clone()),
        }?;
        self.read_param(read, name)
    }

    fn read_param(&self, read: &Read, name: &str) -> Option<Type> {
        let (_, t) = read.iter().find(|(n, _)| &n[1..] == name.as_bytes())?;
        Some(self.to_phpstan(t.as_ref()?))
    }
}

/// `ParameterOutTypeCheck::check()`: the error of a parameter left holding `ty`.
fn out_check(c: &Checker, b: &Body, p: &Param, ty: Type, out: &Type, param_out: bool) -> Option<(String, &'static str)> {
    // findTypeToCheck() only decides whether there is a type to check
    c.find_type_to_check_of(Some(ty.clone()), &|t| is_super_type_of(out, t).yes())?;
    let assigned = if p.variadic {
        if !ty.is_array().yes() {
            return None;
        }
        ty.iterable_value_type()
    } else {
        ty
    };
    if is_super_type_of(out, &assigned).yes() {
        return None;
    }
    let level = recommended_level(out, Some(&assigned));
    let message = format!(
        "Parameter &${} {} of {} expects {}, {} given.",
        p.name,
        if param_out { "@param-out type" } else { "by-ref type" },
        b.of,
        out.describe(level),
        assigned.describe(level)
    );
    Some((message, if param_out { "paramOut.type" } else { "parameterByRef.type" }))
}

const CHANGE_TIP: &str = "You can change the parameter out type with @param-out PHPDoc tag.";
const NARROW_TIP: &str = "You can narrow the parameter out type with @param-out PHPDoc tag.";

fn sites(c: &Checker, b: &Body) -> Vec<Site> {
    let names: Vec<Vec<u8>> = b.params.iter().map(|p| format!("${}", p.name).into_bytes()).collect();
    let mut s = Sites { c, names: &names, out: Vec::new() };
    for st in b.block.statements.iter() {
        s.walk_statement(st, &mut ());
    }
    s.out
}

/// Both level-3 rules and both level-4 ones on a body.
fn check(c: &mut Checker, b: Body) {
    let sites = sites(c, &b);
    let found = declared::with_classes(c, || {
        let mut errors: Vec<(Span, String, &'static str, Option<String>)> = Vec::new();
        if c.level >= 3 {
            // ParameterOutAssignedTypeRule
            for s in &sites {
                let Some(p) = b.params.iter().find(|p| p.name.as_bytes() == &s.name[1..]) else { continue };
                let read = match &s.at {
                    At::Expression(e) => c.variables.param_out.expressions.get(e),
                    At::After(e) => c.variables.param_out.after.get(e),
                    At::Before(e) => c.variables.param_out.before.get(e),
                    At::Type(t) => {
                        if let Some((m, id)) = out_check(c, &b, p, t.clone(), &p.out.clone().unwrap_or_else(|| p.ty.clone()), p.out.is_some()) {
                            errors.push((s.variable, m, id, p.out.is_none().then(|| CHANGE_TIP.to_string())));
                        }
                        continue;
                    }
                };
                let Some(ty) = read.and_then(|r| c.read_param(r, &p.name)) else { continue };
                let (out, param_out) = p.out.clone().map_or_else(|| (p.ty.clone(), false), |o| (o, true));
                if let Some((m, id)) = out_check(c, &b, p, ty, &out, param_out) {
                    errors.push((s.variable, m, id, (!param_out).then(|| CHANGE_TIP.to_string())));
                }
            }
        }
        let paths = Paths::of(b.block, &[]);
        let results = flow::Results {
            types: &c.types,
            call_terminations: Some(&c.call_terminations),
            contents: &c.file.contents,
            codebase: Some(c.codebase),
        };
        let (_, ends) = results.execution_ends(&list(b.block.statements.as_slice()), b.span, Some(b.block.right_brace));
        let ends: Vec<(Span, Option<(EndAt, Written, bool)>)> = ends.iter().map(|e| (e.at, end_at(c, &paths, b.span, e.at))).collect();
        if c.level >= 3 {
            // ParameterOutExecutionEndTypeRule
            for (at, end) in &ends {
                let Some((read, w, never)) = end else { continue };
                if *never {
                    continue;
                }
                for p in &b.params {
                    let Some(out) = &p.out else { continue };
                    if written(&b, &sites, *w, p) {
                        continue;
                    }
                    let Some(ty) = c.param_out_read(read, &p.name, &p.entry()) else { continue };
                    if let Some((m, id)) = out_check(c, &b, p, ty, out, true) {
                        let line = c.line_of(*at);
                        if !errors.iter().any(|(s, em, ..)| c.line_of(*s) == line && *em == m) {
                            errors.push((*at, m, id, None));
                        }
                    }
                }
            }
        }
        if c.level >= 4 && b.too_wide {
            errors.extend(too_wide(c, &b, &ends));
        }
        errors
    });
    for (at, m, id, tip) in found {
        c.report(at, id, m, tip.as_deref());
    }
}

/// Whether every path to an end wrote parameter `p` (`ParameterVariableOriginalValueExpr` is gone).
fn written(b: &Body, sites: &[Site], w: Written, p: &Param) -> bool {
    let own: Vec<Span> = sites.iter().filter(|s| s.name[1..] == *p.name.as_bytes()).map(|s| s.variable).collect();
    let paths = Paths::of(b.block, &own);
    let before = |s: Span| paths.written_before.get(&s).copied().unwrap_or(false);
    match w {
        Written::Never => false,
        Written::Before(s) => before(s),
        Written::Through(s) => before(s) || paths.get(s).is_some_and(|st| Paths::always(st, &own)),
    }
}

/// `TooWideParameterOutTypeCheck::check()`.
fn too_wide(c: &Checker, b: &Body, ends: &[(Span, Option<(EndAt, Written, bool)>)]) -> Vec<(Span, String, &'static str, Option<String>)> {
    let of = format!("{}{}", b.of[..1].to_ascii_uppercase(), &b.of[1..]);
    struct Returns(Vec<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Returns {
        fn walk_in_return(&mut self, r: &'ast Return<'arena>, _: &mut ()) {
            self.0.push(r.span());
        }
        fn walk_closure(&mut self, _: &'ast Closure<'arena>, _: &mut ()) {}
        fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {}
        fn walk_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {}
        fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {}
        fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
    }
    let mut r = Returns(Vec::new());
    for s in b.block.statements.iter() {
        r.walk_statement(s, &mut ());
    }
    let mut reads: Vec<EndAt> = Vec::new();
    for (_, end) in ends {
        let Some((at, ..)) = end else { return Vec::new() };
        reads.push(match at {
            EndAt::After(s) => EndAt::After(*s),
            EndAt::Before(s) => EndAt::Before(*s),
            EndAt::Entry => EndAt::Entry,
        });
    }
    reads.extend(r.0.into_iter().map(EndAt::After));
    if reads.is_empty() {
        return Vec::new();
    }
    let mut errors = Vec::new();
    for p in &b.params {
        let mut types = Vec::new();
        for at in &reads {
            let Some(t) = c.param_out_read(at, &p.name, &p.entry()) else { return errors };
            types.push(t);
        }
        let mut actual = union(types);
        if p.variadic {
            if !actual.is_array().yes() {
                continue;
            }
            actual = actual.iterable_value_type();
        }
        let (out, param_out) = p.out.clone().map_or_else(|| (p.ty.clone(), false), |o| (o, true));
        let kind = if param_out { "@param-out type" } else { "by-ref type" };
        let tip = (!param_out).then(|| NARROW_TIP.to_string());
        let narrowed = narrow(c, &out, &actual);
        if narrowed == out {
            if !c.too_wide.contains(crate::TooWide::REPORT_NESTED) {
                continue;
            }
            let nested = traverse(&out, &actual, &|d, a| narrow(c, d, a));
            if nested == out {
                continue;
            }
            let described = |t: &Type| t.describe(recommended_level(t, None));
            let message = format!(
                "{} {} of {} can be narrowed to {}.",
                if param_out { "PHPDoc tag @param-out type" } else { "By-ref type" },
                described(&out),
                b.of,
                described(&nested)
            );
            let reasons = accepts_reasons(&nested, &out, true, StrictMixed::default());
            let tip = match reasons.as_slice() {
                [] => None,
                [one] => Some(one.clone()),
                all => Some(all.iter().map(|r| format!("• {r}")).collect::<Vec<_>>().join("\n")),
            };
            let id = if param_out { "paramOut.nestedUnusedType" } else { "parameterByRef.nestedUnusedType" };
            errors.push((b.span, message, id, tip));
            continue;
        }
        // createErrors()
        if out.is_boolean().yes() {
            let never = if matches!(narrowed, Type::ConstBool(true)) { "false" } else { "true" };
            let message = format!(
                "{of} never assigns {never} to &${} so the {kind} can be changed to {}.",
                p.name,
                narrowed.describe(recommended_level(&narrowed, None))
            );
            let id = if param_out { "paramOut.tooWideBool" } else { "parameterByRef.tooWideBool" };
            errors.push((b.span, message, id, tip.clone()));
            continue;
        }
        let Type::Union(members) = &out else { continue };
        for m in members {
            if is_super_type_of(&narrowed, m).no() {
                let message = format!(
                    "{of} never assigns {} to &${} so it can be removed from the {kind}.",
                    m.describe(recommended_level(m, None)),
                    p.name
                );
                let id = if param_out { "paramOut.unusedType" } else { "parameterByRef.unusedType" };
                errors.push((b.span, message, id, tip.clone()));
            }
        }
    }
    errors
}

/// `isConstantScalarValue()->yes()`.
fn constant_scalar(t: &Type) -> bool {
    match t {
        Type::ConstInt(_) | Type::ConstFloat(_) | Type::ConstString(_) | Type::ConstBool(_) | Type::Null => true,
        Type::Union(ts) => ts.iter().all(constant_scalar),
        _ => false,
    }
}

/// `TooWideTypeCheck::narrowType()` with `$checkDescendantClass` and `$native` false.
fn narrow(c: &Checker, declared: &Type, actual: &Type) -> Type {
    if let Type::Union(members) = declared {
        if constant_scalar(declared) && constant_scalar(actual) {
            return declared.clone();
        }
        return union(members.iter().filter(|m| !is_super_type_of(m, actual).no()).cloned().collect());
    }
    if !c.too_wide.contains(crate::TooWide::REPORT_BOOL) || !declared.is_boolean().yes() || matches!(declared, Type::ConstBool(_)) {
        return declared.clone();
    }
    union(
        [Type::ConstBool(true), Type::ConstBool(false)]
            .into_iter()
            .filter(|b| !is_super_type_of(b, actual).no())
            .collect(),
    )
}

/// `getOffsetValueType()` of a constant key.
fn offset_value(t: &Type, key: &Type) -> Type {
    match t {
        Type::ConstArray(a) => a
            .keys
            .iter()
            .position(|k| k == key)
            .map_or(Type::Never(NeverKind::Implicit), |i| a.values[i].clone()),
        Type::Union(ts) => union(ts.iter().map(|t| offset_value(t, key)).collect()),
        t => t.iterable_value_type(),
    }
}

/// `SimultaneousTypeTraverser::map()` with TooWideTypeCheck's callback (`narrow` a declared type by the actual one,
/// else descend).
fn traverse(declared: &Type, actual: &Type, narrow: &dyn Fn(&Type, &Type) -> Type) -> Type {
    let narrowed = narrow(declared, actual);
    if narrowed != *declared {
        return narrowed;
    }
    let go = |d: &Type, a: &Type| traverse(d, a, narrow);
    match declared {
        Type::Array(k, v) => {
            let (nk, nv) = (go(k, &actual.iterable_key_type()), go(v, &actual.iterable_value_type()));
            if nk == **k && nv == **v { declared.clone() } else { Type::Array(Box::new(nk), Box::new(nv)) }
        }
        Type::ConstArray(a) if actual.is_array().yes() => {
            let mut a = (**a).clone();
            for (k, v) in a.keys.iter().zip(a.values.iter_mut()) {
                *v = go(v, &offset_value(actual, k));
            }
            Type::ConstArray(Box::new(a))
        }
        Type::Union(members) => {
            let mut right: Vec<Type> = match actual {
                Type::Union(ts) => ts.clone(),
                t => vec![t.clone()],
            };
            let mut out = Vec::new();
            for m in members {
                let (candidates, rest): (Vec<Type>, Vec<Type>) =
                    right.into_iter().partition(|r| is_super_type_of(m, r).yes());
                right = rest;
                out.push(if candidates.is_empty() { m.clone() } else { go(m, &union(candidates)) });
            }
            if out == *members { declared.clone() } else { union(out) }
        }
        Type::Object(name, args, exact) if !args.is_empty() => {
            let Type::Object(other, other_args, _) = actual else { return declared.clone() };
            let theirs = if other.eq_ignore_ascii_case(name) {
                Some(other_args.clone())
            } else {
                symbols::lookup(other).and_then(|k| ancestor_args(&k, name, other_args))
            };
            match theirs {
                Some(theirs) if theirs.len() == args.len() => {
                    Type::Object(name.clone(), args.iter().zip(&theirs).map(|(a, o)| go(a, o)).collect(), *exact)
                }
                _ => declared.clone(),
            }
        }
        Type::Intersection(parts) if declared.is_array().yes() && actual.is_array().yes() => {
            let (k, v) = (actual.iterable_key_type(), actual.iterable_value_type());
            Type::Intersection(
                parts
                    .iter()
                    .map(|p| match p {
                        Type::Array(pk, pv) => Type::Array(Box::new(go(pk, &k)), Box::new(go(pv, &v))),
                        p => p.clone(),
                    })
                    .collect(),
            )
        }
        _ => declared.clone(),
    }
}

fn params(c: &Checker, list: &FunctionLikeParameterList, sig: &declared::Signature, class: Option<&ClassLikeMetadata>) -> Vec<Param> {
    list.parameters
        .iter()
        .zip(&sig.params)
        .filter(|(p, _)| p.ampersand.is_some())
        .map(|(p, (name, ty, out, _))| Param {
            name: name.clone(),
            ty: late(c, ty, class, true),
            out: out.as_ref().map(|o| late(c, o, class, true)),
            variadic: p.ellipsis.is_some(),
        })
        .collect()
}

pub(in super::super) fn function(c: &mut Checker, f: &Function) {
    if c.level < 3 {
        return;
    }
    let params = declared::with_classes(c, || params(c, &f.parameter_list, &declared::function(c, f), None));
    if params.is_empty() {
        return;
    }
    let of = format!("function {}()", c.resolved_span(f.name.span()));
    check(c, Body { block: &f.body, span: f.span(), of, params, too_wide: true });
}

pub(in super::super) fn method(c: &mut Checker, m: &Method) {
    if c.level < 3 || c.in_trait().is_some() {
        return;
    }
    let MethodBody::Concrete(block) = &m.body else { return };
    let (Some(context), Some(declaring)) = (c.class_scope(), declaring(c, m.span())) else { return };
    let params =
        declared::with_classes(c, || params(c, &m.parameter_list, &declared::method(c, m, declaring, context), Some(context)));
    if params.is_empty() {
        return;
    }
    let of = format!("method {}::{}()", c.display(context), method_name(c, m));
    let checked = m.modifiers.contains_private()
        || context.flags.is_final()
        || context.original_name.as_bytes().starts_with(b"{anonymous-class:")
        || m.modifiers.contains_final()
        || has_tag(&c.tags_before(m), &["@final"])
        || c.too_wide.contains(crate::TooWide::PARAM_OUT_IN_PROTECTED_AND_PUBLIC_METHODS);
    check(c, Body { block, span: m.span(), of, params, too_wide: checked });
}

/// `UnresolvableTypeHelper::getUnresolvableType() !== null`.
fn unresolvable(t: &Type) -> bool {
    let mut found = false;
    super::super::type_aliases::each(t, &mut |t| {
        found |= matches!(t, Type::Error | Type::Never(NeverKind::Implicit));
    });
    found
}

/// An OffsetAccessType of a constant array resolved to the offset's value (`OffsetAccessType::getResult()`).
fn offsets_resolved(t: &Type) -> Type {
    match t {
        Type::OffsetAccess(on, offset) => offset_value(&offsets_resolved(on), &offsets_resolved(offset)),
        t => t.traverse(&offsets_resolved),
    }
}

fn has_offset_access(t: &Type) -> bool {
    let found = std::cell::Cell::new(matches!(t, Type::OffsetAccess(..)));
    t.traverse(&|i| {
        found.set(found.get() || has_offset_access(i));
        i.clone()
    });
    found.get()
}

/// IncompatibleSelfOutTypeRule.
pub(in super::super) fn self_out(c: &mut Checker, m: &Method) {
    if c.level < 2 || c.in_trait().is_some() {
        return;
    }
    let (Some(context), Some(declaring)) = (c.class_scope(), declaring(c, m.span())) else { return };
    let Some(self_out) = declared::with_classes(c, || declared::method(c, m, declaring, context).self_out) else { return };
    let name = method_name(c, m);
    let display = c.display(context);
    let errors = declared::with_classes(c, || {
        // (an OffsetAccessType stays as written: only isSuperTypeOf() resolves it, and it is resolvable)
        let written = self_out.clone();
        let self_out = late(c, &offsets_resolved(&self_out), Some(context), false);
        let class = Type::Object(c.reflected_name(context.name.as_bytes()), Vec::new(), false);
        let mut errors: Vec<(&'static str, String)> = Vec::new();
        if !is_super_type_of(&class, &self_out).yes() {
            let described = if has_offset_access(&written) { written.describe(Level::Precise) } else { self_out.describe(Level::Precise) };
            errors.push((
                "selfOut.type",
                format!(
                    "Self-out type {described} of method {display}::{name} is not subtype of {}.",
                    class.describe(Level::Precise)
                ),
            ));
        }
        if m.modifiers.contains_static() {
            errors.push((
                "selfOut.static",
                format!(
                    "PHPDoc tag @phpstan-self-out is not supported above static method {}::{name}().",
                    declared::original_name(context)
                ),
            ));
        }
        if unresolvable(&self_out) {
            errors.push((
                "selfOut.unresolvableType",
                format!("PHPDoc tag @phpstan-self-out for method {display}::{name}() contains unresolvable type."),
            ));
        }
        let messages = generics::GenericMessages::at(&format!("PHPDoc tag {}", generics::escape("@phpstan-self-out")));
        let mut found = generics::Found::default();
        generics::generic_object_check(&self_out, &messages, &mut found);
        errors.extend(found.errors.into_iter().map(|(id, m, _)| (id, m)));
        errors
    });
    for (id, message) in errors {
        c.report(m.span(), id, message, None);
    }
}
