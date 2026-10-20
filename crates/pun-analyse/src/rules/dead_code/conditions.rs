//! The constant condition rules: If, ElseIf, TernaryOperator, WhileLoopAlwaysFalse, WhileLoopAlwaysTrue, DoWhileLoop,
//! BooleanNot, BooleanAnd, BooleanOr and LogicalXor ConstantConditionRule.
//!
//! A call's condition reports through FunctionCallConstantConditionRule's collector, unless an ImpossibleCheckType rule
//! owns the call. Not ported: ConstantConditionInTraitRule (nothing reports in a trait) and the
//! `treatPhpDocTypesAsCertain` and possibly impure tips.

use std::collections::{HashMap, HashSet};

use mago_codex::assertion::Assertion;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::super::Checker;
use super::super::declarations::doc_before;
use super::super::members;
use pun_phpstan::types::{Level, Type, recommended_level};
use super::finally;
use super::no_discard::unparen;
use super::noop::{self, asserts, function_of};

const REMOVE_TIP: &str = "Remove remaining cases below this one and this error will disappear too.";

/// The functions with a TypeSpecifier extension or a `($value is T ? true : false)` stub return. The stubs with
/// `@phpstan-assert-if-true =T` (is_resource(), ctype_alpha(), file functions...) set the call as the root
/// expression, so ImpossibleCheckTypeHelper never decides them.
const TYPE_CHECKS: &[&str] = &[
    "array_is_list",
    "array_key_exists",
    "assert",
    "class_exists",
    "ctype_digit",
    "enum_exists",
    "fnmatch",
    "in_array",
    "interface_exists",
    "is_a",
    "is_array",
    "is_bool",
    "is_callable",
    "is_countable",
    "is_double",
    "is_float",
    "is_int",
    "is_integer",
    "is_iterable",
    "is_long",
    "is_null",
    "is_numeric",
    "is_object",
    "is_real",
    "is_scalar",
    "is_string",
    "is_subclass_of",
    "key_exists",
    "mb_stripos",
    "mb_strlen",
    "mb_strpos",
    "mb_strripos",
    "mb_strrpos",
    "mb_strstr",
    "method_exists",
    "preg_match",
    "preg_match_all",
    "property_exists",
    "settype",
    "str_contains",
    "str_ends_with",
    "str_starts_with",
    "stripos",
    "strlen",
    "strpos",
    "strripos",
    "strrpos",
    "strstr",
    "trait_exists",
];

const NEVER: &[&str] = &[
    "never",
    "never-return",
    "never-returns",
    "no-return",
    "noreturn",
];

/// LastConditionVisitor's `isLastCondition` attribute, by the condition's span, and the `while` loops in a function
/// (or else closure) that never returns.
#[derive(Default)]
pub(in super::super) struct Facts {
    last: HashMap<Span, bool>,
    never_loops: HashSet<Span>,
    /// LastConditionVisitor's `isMatch`: a match arm's last condition.
    match_arms: HashSet<Span>,
}

pub(in super::super) fn facts(c: &Checker, program: &Program) -> Facts {
    let mut pass = Pass {
        contents: &c.file.contents,
        functions: Vec::new(),
        facts: Facts::default(),
    };
    if c.level >= 4 {
        pass.walk_program(program, &mut ());
    }
    pass.facts
}

struct Pass<'c> {
    contents: &'c [u8],
    /// (named, returns never) of the enclosing function-likes.
    functions: Vec<(bool, bool)>,
    facts: Facts,
}

/// A body's statements, as PHP-Parser's `stmts` (without the braces' block).
fn body<'s, 'arena>(s: &'s [Statement<'arena>]) -> &'s [Statement<'arena>] {
    match s {
        [Statement::Block(b)] => b.statements.as_slice(),
        s => s,
    }
}

fn is_throw(s: &Statement) -> bool {
    matches!(s, Statement::Expression(e) if matches!(e.expression, Expression::Throw(_)))
}

impl Pass<'_> {
    /// A list ending with `if (...) {...} throw ...;`: the `if`'s last condition is the last.
    fn list(&mut self, statements: &[Statement]) {
        if let [.., Statement::If(i), last] = statements
            && is_throw(last)
            && i.body.else_statements().is_none()
        {
            let cond = i
                .body
                .else_if_clauses()
                .last()
                .map_or(i.condition, |(e, _)| *e);
            self.facts.last.insert(unparen(cond).span(), true);
        }
    }

    fn enter(&mut self, named: bool, start: Span, hint: Option<&FunctionLikeReturnTypeHint>) {
        let native = hint.is_some_and(|h| matches!(h.hint, Hint::Never(_)));
        let doc = doc_before(self.contents, start.start.offset as usize).is_some_and(|doc| {
            String::from_utf8_lossy(doc).lines().any(|line| {
                let mut words = line
                    .trim_start_matches([' ', '\t', '*', '/'])
                    .split_whitespace();
                matches!(
                    words.next(),
                    Some("@return" | "@phpstan-return" | "@psalm-return")
                ) && words
                    .next()
                    .is_some_and(|t| NEVER.contains(&t.to_ascii_lowercase().as_str()))
            })
        });
        self.functions.push((named, native || doc));
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Pass<'_> {
    fn walk_in_function(&mut self, f: &'ast Function<'arena>, _: &mut ()) {
        self.enter(true, f.span(), f.return_type_hint.as_ref());
        self.list(f.body.statements.as_slice());
    }
    fn walk_out_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {
        self.functions.pop();
    }
    fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
        self.enter(true, m.span(), m.return_type_hint.as_ref());
        if let MethodBody::Concrete(b) = &m.body {
            self.list(b.statements.as_slice());
        }
    }
    fn walk_out_method(&mut self, _: &'ast Method<'arena>, _: &mut ()) {
        self.functions.pop();
    }
    fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        self.enter(false, f.span(), f.return_type_hint.as_ref());
    }
    fn walk_out_closure(&mut self, _: &'ast Closure<'arena>, _: &mut ()) {
        self.functions.pop();
    }
    fn walk_in_arrow_function(&mut self, f: &'ast ArrowFunction<'arena>, _: &mut ()) {
        self.enter(false, f.span(), f.return_type_hint.as_ref());
    }
    fn walk_out_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {
        self.functions.pop();
    }
    fn walk_in_namespace(&mut self, n: &'ast Namespace<'arena>, _: &mut ()) {
        self.list(n.statements().as_slice());
    }
    fn walk_in_match(&mut self, m: &'ast Match<'arena>, _: &mut ()) {
        let last = m.arms.len().saturating_sub(1);
        for (n, arm) in m.arms.iter().enumerate() {
            if let MatchArm::Expression(a) = arm
                && let Some(cond) = a.conditions.iter().last()
            {
                let span = unparen(cond).span();
                self.facts.last.insert(span, n == last);
                self.facts.match_arms.insert(span);
            }
        }
    }
    fn walk_in_if(&mut self, i: &'ast If<'arena>, _: &mut ()) {
        let elseifs = i.body.else_if_clauses();
        if let Some(last) = elseifs.len().checked_sub(1) {
            let missing_or_throwing = i
                .body
                .else_statements()
                .is_none_or(|s| matches!(body(s), [s] if is_throw(s)));
            for (n, (cond, _)) in elseifs.iter().enumerate() {
                self.facts
                    .last
                    .insert(unparen(cond).span(), n == last && missing_or_throwing);
            }
        }
        self.list(body(i.body.statements()));
        for (_, s) in &elseifs {
            self.list(body(s));
        }
        if let Some(s) = i.body.else_statements() {
            self.list(body(s));
        }
    }
    fn walk_in_switch_case(&mut self, case: &'ast SwitchCase<'arena>, _: &mut ()) {
        self.list(case.statements());
    }
    fn walk_in_try(&mut self, t: &'ast Try<'arena>, _: &mut ()) {
        self.list(t.block.statements.as_slice());
        if let Some(f) = &t.finally_clause {
            self.list(f.block.statements.as_slice());
        }
    }
    fn walk_in_try_catch_clause(&mut self, clause: &'ast TryCatchClause<'arena>, _: &mut ()) {
        self.list(clause.block.statements.as_slice());
    }
    fn walk_in_do_while(&mut self, d: &'ast DoWhile<'arena>, _: &mut ()) {
        self.list(body(std::slice::from_ref(d.statement)));
    }
    fn walk_in_for(&mut self, f: &'ast For<'arena>, _: &mut ()) {
        self.list(body(f.body.statements()));
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        self.list(body(f.body.statements()));
    }
    fn walk_in_while(&mut self, w: &'ast While<'arena>, _: &mut ()) {
        self.list(body(w.body.statements()));
        // `$scope->getFunction() ?? $scope->getAnonymousFunctionReflection()`
        let function = self
            .functions
            .iter()
            .rev()
            .find(|(named, _)| *named)
            .or(self.functions.last());
        if function.is_some_and(|(_, never)| *never) {
            self.facts.never_loops.insert(w.span());
        }
    }
}

/// The condition expressions whose types the rules read.
pub(super) fn typed(program: &Program) -> Vec<Span> {
    struct Wanted {
        spans: Vec<Span>,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_conditional(&mut self, t: &'ast Conditional<'arena>, _: &mut ()) {
            self.spans.push(t.condition.span());
        }
        fn walk_in_binary(&mut self, b: &'ast Binary<'arena>, _: &mut ()) {
            if logical(&b.operator).is_some() || comparison(&b.operator).is_some() {
                self.spans.extend([b.lhs.span(), b.rhs.span(), b.span()]);
            }
        }
        fn walk_in_argument_list(&mut self, a: &'ast ArgumentList<'arena>, _: &mut ()) {
            self.spans.extend(a.arguments.iter().map(|a| a.value().span()));
        }
        fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
            if matches!(u.operator, UnaryPrefixOperator::Not(_)) {
                self.spans.push(u.operand.span());
            }
        }
    }
    let mut wanted = Wanted { spans: Vec::new() };
    wanted.walk_program(program, &mut ());
    wanted.spans
}

/// The identifier prefix and sigil of `&&`, `||`, `and`, `or` and `xor`.
fn logical(op: &BinaryOperator) -> Option<(&'static str, &'static str)> {
    match op {
        BinaryOperator::And(_) => Some(("booleanAnd", "&&")),
        BinaryOperator::Or(_) => Some(("booleanOr", "||")),
        BinaryOperator::LowAnd(_) => Some(("logicalAnd", "and")),
        BinaryOperator::LowOr(_) => Some(("logicalOr", "or")),
        BinaryOperator::LowXor(_) => Some(("logicalXor", "xor")),
        _ => None,
    }
}

/// ConstantConditionRuleHelper::shouldSkip(): the conditions other rules report.
fn skipped(e: &Expression) -> bool {
    match e {
        Expression::Binary(b) => matches!(
            b.operator,
            BinaryOperator::Equal(_)
                | BinaryOperator::NotEqual(_)
                | BinaryOperator::AngledNotEqual(_)
                | BinaryOperator::Identical(_)
                | BinaryOperator::NotIdentical(_)
                | BinaryOperator::Instanceof(_)
                | BinaryOperator::And(_)
                | BinaryOperator::Or(_)
                | BinaryOperator::LessThan(_)
                | BinaryOperator::LessThanOrEqual(_)
                | BinaryOperator::GreaterThan(_)
                | BinaryOperator::GreaterThanOrEqual(_)
        ),
        Expression::UnaryPrefix(u) => matches!(u.operator, UnaryPrefixOperator::Not(_)),
        Expression::Conditional(_) => true,
        Expression::Construct(Construct::Isset(_) | Construct::Empty(_)) => true,
        _ => false,
    }
}

impl Checker<'_, '_> {
    /// `getBooleanType()` when it's a constant: the condition is always true or always false.
    fn constant(&self, e: &Expression) -> Option<bool> {
        if skipped(unparen(e)) {
            return None;
        }
        self.truthiness(e)
    }

    /// The rules run at level 4, and not in a trait (ConstantConditionInTraitRule isn't ported).
    fn conditions_checked(&self) -> bool {
        self.level >= 4 && self.in_trait().is_none()
    }

    /// The error goes through FunctionCallConstantConditionRule's collector: a call an ImpossibleCheckType rule owns.
    fn deferred(&self, e: &Expression) -> bool {
        match unparen(e) {
            Expression::Call(Call::Function(f)) => self.owned_function(f),
            Expression::Call(Call::Method(m)) => self.asserting(m.object, &m.method),
            Expression::Call(Call::StaticMethod(m)) => self.asserting(m.class, &m.method),
            _ => false,
        }
    }

    fn owned_function(&self, f: &FunctionCall) -> bool {
        function_of(self, f.function).is_some_and(|function| match function {
            noop::Function::User(f) => asserts(f),
            noop::Function::Builtin(b) => {
                let name = b.name.to_ascii_lowercase();
                TYPE_CHECKS.contains(&name.as_str())
            }
        })
    }

    /// A type that is exactly `true` or `false`.
    fn exact(&self, span: Span) -> Option<bool> {
        self.types.get(&span).and_then(|t| {
            if t.is_true() {
                Some(true)
            } else {
                t.is_false().then_some(false)
            }
        })
    }

    /// `toBoolean()` as a constant: the expression is always truthy or always falsy.
    fn truthiness(&self, e: &Expression) -> Option<bool> {
        let t = self
            .type_of(e)
            .filter(|t| !t.types.is_empty() && !t.is_never())?;
        if t.is_always_truthy() {
            Some(true)
        } else {
            t.is_always_falsy().then_some(false)
        }
    }

    /// The sure types against the arguments: true when every argument already is the type, false when one can't be.
    /// determineContext() asks a void function for its `@phpstan-assert` tags, anything else in a truthy context
    /// for its `@phpstan-assert-if-true` tags. An unresolved template makes the call the root expression.
    fn asserted(&self, f: &FunctionLikeMetadata, args: &ArgumentList) -> Option<bool> {
        let void = f.return_type_metadata.as_ref().is_some_and(|r| r.type_union.is_void());
        let assertions = if void { &f.assertions } else { &f.if_true_assertions };
        if !void && f.assertions_inferred
            || assertions.is_empty()
            || assertions.values().flatten().any(|a| {
                matches!(a, Assertion::IsType(TAtomic::GenericParameter(_)) | Assertion::IsNotType(TAtomic::GenericParameter(_)))
            })
        {
            return None;
        }
        let mut all = Some(true);
        for (name, assertions) in assertions {
            let i = f.parameters.iter().position(|p| p.get_name().0 == *name)?;
            let arg = args.arguments.iter().enumerate().find_map(|(j, a)| match a {
                Argument::Positional(a) if j == i => Some(a.value),
                Argument::Named(a) if name.as_bytes()[1..] == *a.name.value => Some(a.value),
                _ => None,
            })?;
            let given = self.type_of(arg)?;
            for assertion in assertions {
                let (Assertion::IsType(a) | Assertion::IsNotType(a)) = assertion else {
                    return None;
                };
                let wanted = TUnion::from_atomic(a.clone());
                let is = if union_comparator::is_contained_by(
                    self.codebase,
                    given,
                    &wanted,
                    false,
                    false,
                    false,
                    &mut ComparisonResult::new(),
                ) {
                    Some(true)
                } else if union_comparator::can_expression_types_be_identical(self.codebase, given, &wanted, false, false) {
                    None
                } else {
                    Some(false)
                };
                let is = is.map(|b| b == matches!(assertion, Assertion::IsType(_)));
                all = match (all, is) {
                    (Some(false), _) | (_, Some(false)) => Some(false),
                    (Some(true), Some(true)) => Some(true),
                    _ => None,
                };
            }
        }
        all
    }

    fn asserting(&self, on: &Expression, method: &ClassLikeMemberSelector) -> bool {
        matches!(self.method_target(on, method), Some(members::Method::User { f, .. }) if asserts(f))
    }

    fn constant_condition(
        &mut self,
        e: &Expression,
        line_of: Span,
        message: &str,
        identifier: &str,
        tip: Option<&str>,
    ) -> Option<bool> {
        let value = self.constant(e)?;
        if !self.deferred(e) {
            let line = self.line_of(line_of);
            let (text, suffix) = if value {
                ("true", "True")
            } else {
                ("false", "False")
            };
            self.report_at_line(
                line,
                &format!("{identifier}{suffix}"),
                format!("{message} is always {text}."),
                tip,
            );
        }
        Some(value)
    }

    fn last_condition(&self, span: Span) -> Option<bool> {
        self.conditions.last.get(&span).copied()
    }
}

pub(in super::super) fn if_statement(c: &mut Checker, i: &If) {
    if !c.conditions_checked() {
        return;
    }
    c.constant_condition(
        i.condition,
        unparen(i.condition).span(),
        "If condition",
        "if.always",
        None,
    );
    for (cond, _) in i.body.else_if_clauses() {
        let last = c.last_condition(unparen(cond).span());
        if c.constant(cond) == Some(true) && last == Some(true) {
            continue;
        }
        let tip = (last == Some(false)).then_some(REMOVE_TIP);
        let tip = tip.filter(|_| c.constant(cond) == Some(true));
        c.constant_condition(
            cond,
            unparen(cond).span(),
            "Elseif condition",
            "elseif.always",
            tip,
        );
    }
}

pub(in super::super) fn ternary(c: &mut Checker, t: &Conditional) {
    if !c.conditions_checked() {
        return;
    }
    c.constant_condition(
        t.condition,
        t.span(),
        "Ternary operator condition",
        "ternary.always",
        None,
    );
}

pub(in super::super) fn while_loop(c: &mut Checker, w: &While) {
    if !c.conditions_checked() {
        return;
    }
    match c.constant(w.condition) {
        Some(false) => {
            c.constant_condition(
                w.condition,
                w.span(),
                "While loop condition",
                "while.always",
                None,
            );
        }
        Some(true) => {
            let body = finally::statements(c, w.body.statements());
            let exits = !body.exits.is_empty()
                || body
                    .jumps
                    .iter()
                    .any(|(_, kind, n)| *kind == "break" || *n > 1);
            if !exits
                && !has_yield(w.body.statements())
                && !c.conditions.never_loops.contains(&w.span())
            {
                c.constant_condition(
                    w.condition,
                    w.span(),
                    "While loop condition",
                    "while.always",
                    None,
                );
            }
        }
        None => {}
    }
}

pub(in super::super) fn do_while(c: &mut Checker, d: &DoWhile) {
    if !c.conditions_checked() {
        return;
    }
    let body = std::slice::from_ref(d.statement);
    let exits = finally::statements(c, body);
    let reported = match c.constant(d.condition) {
        Some(true) => {
            exits.exits.is_empty()
                && !has_yield(body)
                && exits
                    .jumps
                    .iter()
                    .all(|(_, kind, n)| *kind == "continue" && *n <= 1)
        }
        Some(false) => !exits.jumps.iter().any(|(_, kind, _)| *kind == "break"),
        None => false,
    };
    if reported {
        c.constant_condition(
            d.condition,
            unparen(d.condition).span(),
            "Do-while loop condition",
            "doWhile.always",
            None,
        );
    }
}

pub(in super::super) fn boolean_not(c: &mut Checker, u: &UnaryPrefix) {
    if !c.conditions_checked() || !matches!(u.operator, UnaryPrefixOperator::Not(_)) {
        return;
    }
    let Some(value) = c.constant(u.operand) else {
        return;
    };
    let last = c.last_condition(u.span());
    if !value && last == Some(true) {
        return;
    }
    let tip = (!value && last == Some(false)).then_some(REMOVE_TIP);
    if !c.deferred(u.operand) {
        let line = c.line_of(unparen(u.operand).span());
        let (text, suffix) = if value {
            ("false", "False")
        } else {
            ("true", "True")
        };
        c.report_at_line(
            line,
            &format!("booleanNot.always{suffix}"),
            format!("Negated boolean expression is always {text}."),
            tip,
        );
    }
}

/// The identifier prefix, sigil and rule kind of a comparison: StrictComparisonOfDifferentTypesRule,
/// ConstantLooseComparisonRule or NumberComparisonOperatorsConstantConditionRule.
fn comparison(op: &BinaryOperator) -> Option<(&'static str, &'static str, &'static str)> {
    Some(match op {
        BinaryOperator::Identical(_) => ("identical", "===", "Strict"),
        BinaryOperator::NotIdentical(_) => ("notIdentical", "!==", "Strict"),
        BinaryOperator::Equal(_) => ("equal", "==", "Loose"),
        BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => ("notEqual", "!=", "Loose"),
        BinaryOperator::GreaterThan(_) => ("greater", ">", ""),
        BinaryOperator::GreaterThanOrEqual(_) => ("greaterOrEqual", ">=", ""),
        BinaryOperator::LessThan(_) => ("smaller", "<", ""),
        BinaryOperator::LessThanOrEqual(_) => ("smallerOrEqual", "<=", ""),
        BinaryOperator::Instanceof(_) => ("instanceof", "instanceof", "Instanceof"),
        _ => return None,
    })
}

fn compare(c: &mut Checker, b: &Binary) {
    let Some((identifier, sigil, kind)) = comparison(&b.operator) else {
        return;
    };
    let value = c.exact(b.span());
    let right = match (kind, b.rhs) {
        // ImpossibleInstanceOfRule: a class name, as ObjectType
        ("Instanceof", Expression::Identifier(id)) => Some(Type::Object(c.resolved(id), vec![])),
        _ => c.phpstan_type(b.rhs),
    };
    let (Some(value), Some(left), Some(right)) = (value, c.phpstan_type(b.lhs), right) else {
        return;
    };
    let last = c.last_condition(b.span());
    let (text, suffix) = if value { ("true", "True") } else { ("false", "False") };
    let (l, r) = super::super::declared::with_classes(c, || {
        if kind == "Instanceof" {
            (left.describe(Level::TypeOnly), right.describe(recommended_level(&right, None)))
        } else {
            (left.describe(Level::Value), right.describe(Level::Value))
        }
    });
    let message = if kind == "Instanceof" {
        if value && last == Some(true) {
            return;
        }
        format!("Instanceof between {l} and {r} will always evaluate to {text}.")
    } else if kind.is_empty() {
        format!("Comparison operation \"{sigil}\" between {l} and {r} is always {text}.")
    } else {
        // the always-true one is left to the last condition
        if value && last == Some(true) {
            return;
        }
        format!("{kind} comparison using {sigil} between {l} and {r} will always evaluate to {text}.")
    };
    let mut tips = Vec::new();
    if value && last == Some(false) && !kind.is_empty() {
        tips.push(REMOVE_TIP);
    }
    if value
        && kind == "Strict"
        && !c.conditions.match_arms.contains(&b.span())
        && [&left, &right].iter().all(|t| matches!(t, Type::EnumCase(..)))
    {
        tips.push("Use match expression instead. PHPStan will report unhandled enum cases.");
    }
    let tip = match tips.as_slice() {
        [] => None,
        [tip] => Some(tip.to_string()),
        tips => Some(tips.iter().map(|t| format!("• {t}")).collect::<Vec<_>>().join("\n")),
    };
    c.report(b, &format!("{identifier}.always{suffix}"), message, tip.as_deref());
}

pub(in super::super) fn binary(c: &mut Checker, b: &Binary) {
    let Some((identifier, sigil)) = logical(&b.operator) else {
        if c.conditions_checked() {
            compare(c, b);
        }
        return;
    };
    if !c.conditions_checked() {
        return;
    }
    let last = c.last_condition(b.span());
    let xor = matches!(b.operator, BinaryOperator::LowXor(_));
    let top_level = c.is_top_level(b.span());
    let sigil = if xor { "xor" } else { sigil };
    let side = |c: &mut Checker, e: &Expression, which: &str| -> bool {
        let Some(value) = c.constant(e) else {
            return false;
        };
        if value && last == Some(true) {
            return false;
        }
        let tip = (value && last == Some(false)).then_some(REMOVE_TIP);
        let side = if which == "left" { "Left" } else { "Right" };
        c.constant_condition(
            e,
            unparen(e).span(),
            &format!("{side} side of {sigil}"),
            &format!("{identifier}.{which}Always"),
            tip,
        );
        true
    };
    let left = side(c, b.lhs, "left");
    // the right side of a short-circuited operator is never evaluated: its type is never
    let skipped = c
        .types
        .get(&b.lhs.span())
        .is_some_and(|t| match b.operator {
            BinaryOperator::And(_) | BinaryOperator::LowAnd(_) => t.is_always_falsy(),
            BinaryOperator::Or(_) | BinaryOperator::LowOr(_) => t.is_always_truthy(),
            _ => false,
        });
    let right = (xor || !top_level) && !skipped && side(c, b.rhs, "right");
    if xor || left || right || top_level {
        return;
    }
    let Some(value) = c.types.get(&b.span()).and_then(|t| {
        if t.is_true() {
            Some(true)
        } else if t.is_false() {
            Some(false)
        } else {
            None
        }
    }) else {
        return;
    };
    if value && last == Some(true) {
        return;
    }
    let tip = (value && last == Some(false)).then_some(REMOVE_TIP);
    let (text, suffix) = if value {
        ("true", "True")
    } else {
        ("false", "False")
    };
    c.report(
        b,
        &format!("{identifier}.always{suffix}"),
        format!("Result of {sigil} is always {text}."),
        tip,
    );
}

/// ImpossibleCheckType{Function,Method,StaticMethod}CallRule: a type-checking call (one FunctionCallConstantCondition-
/// Rule leaves to these rules) whose result is constant.
pub(in super::super) fn function_check(c: &mut Checker, call: &FunctionCall) {
    if !c.conditions_checked() || !c.owned_function(call) {
        return;
    }
    let name = noop::source(c, call.function.span());
    let subject = format!("function {}", name.trim_start_matches('\\'));
    // ImpossibleCheckTypeHelper: assert() is decided by its argument
    let value = if name.trim_start_matches('\\').eq_ignore_ascii_case("assert") {
        call.argument_list.arguments.first().and_then(|a| c.truthiness(a.value()))
    } else {
        let asserted = match function_of(c, call.function) {
            Some(noop::Function::User(f)) => c.asserted(f, &call.argument_list),
            _ => None,
        };
        c.exact(call.span()).or(asserted)
    };
    impossible_check(c, call.span(), value, "function", subject, &call.argument_list);
}

pub(in super::super) fn method_check(c: &mut Checker, call: &MethodCall) {
    method_report(c, call.span(), call.object, &call.method, "method", "method", &call.argument_list);
}

pub(in super::super) fn static_method_check(c: &mut Checker, call: &StaticMethodCall) {
    method_report(c, call.span(), call.class, &call.method, "staticMethod", "static method", &call.argument_list);
}

fn method_report(
    c: &mut Checker,
    span: Span,
    on: &Expression,
    method: &ClassLikeMemberSelector,
    identifier: &str,
    what: &str,
    args: &ArgumentList,
) {
    if !c.conditions_checked() {
        return;
    }
    let (subject, asserted) = match c.method_target(on, method) {
        Some(m @ members::Method::User { f, .. }) if asserts(f) => (
            format!("{what} {}::{}", m.declaring_display(c), m.name()),
            c.asserted(f, args),
        ),
        _ => return,
    };
    let value = c.exact(span).or(asserted);
    impossible_check(c, span, value, identifier, subject, args);
}

fn impossible_check(
    c: &mut Checker,
    span: Span,
    value: Option<bool>,
    identifier: &str,
    subject: String,
    args: &ArgumentList,
) {
    let Some(value) = value else {
        return;
    };
    let last = c.last_condition(span);
    if value && last == Some(true) {
        return;
    }
    let types: Option<Vec<_>> = args.arguments.iter().map(|a| c.phpstan_type(a.value())).collect();
    let Some(types) = types else { return };
    let mut described: Vec<String> =
        super::super::declared::with_classes(c, || types.iter().map(|t| t.describe(Level::Value)).collect());
    // getArgumentsDescription(): nothing without arguments
    let with = match described.len() {
        0..=2 => described.first().map(|_| format!(" with {}", described.join(" and "))).unwrap_or_default(),
        _ => {
            let last = described.pop().unwrap_or_default();
            format!(" with arguments {} and {last}", described.join(", "))
        }
    };
    let (text, suffix) = if value { ("true", "alreadyNarrowedType") } else { ("false", "impossibleType") };
    let tip = (value && last == Some(false)).then_some(REMOVE_TIP);
    let line = c.line_of(span);
    c.report_at_line(
        line,
        &format!("{identifier}.{suffix}"),
        format!("Call to {subject}(){with} will always evaluate to {text}."),
        tip,
    );
}

/// `hasYield()` of a loop body's result: a `yield` outside nested closures and anonymous classes.
fn has_yield(statements: &[Statement]) -> bool {
    struct Finder(bool);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Finder {
        fn walk_in_yield(&mut self, _: &'ast Yield<'arena>, _: &mut ()) {
            self.0 = true;
        }
        fn walk_closure(&mut self, _: &'ast Closure<'arena>, _: &mut ()) {}
        fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {}
        fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {}
    }
    let mut finder = Finder(false);
    for s in statements {
        finder.walk_statement(s, &mut ());
    }
    finder.0
}
