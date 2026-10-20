//! The constant condition rules: If, ElseIf, TernaryOperator, WhileLoopAlwaysFalse, WhileLoopAlwaysTrue, DoWhileLoop,
//! BooleanNot, BooleanAnd, BooleanOr and LogicalXor ConstantConditionRule.
//!
//! A call's condition reports through FunctionCallConstantConditionRule's collector, unless an ImpossibleCheckType rule
//! owns the call. Not ported: ConstantConditionInTraitRule (nothing reports in a trait) and the
//! `treatPhpDocTypesAsCertain` and possibly impure tips.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_database::file::File;

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
use crate::codebase::parse;
use super::super::declarations::doc_before;
use super::super::members;
use mago_codex::symbol::SymbolKind;
use pun_phpstan::ignore::Error;
use pun_phpstan::types::{Level, NeverKind, Tri, Type, is_super_type_of, recommended_level, union};
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
    /// TryCatchTypeVisitor's `tryCatchTypes` of a match: the class names its enclosing `try` statements catch.
    caught: HashMap<Span, Vec<Span>>,
    /// The matches in a function whose `@throws` tag covers UnhandledMatchError.
    throws: HashSet<Span>,
}

pub(in super::super) fn facts(c: &Checker, program: &Program) -> Facts {
    let mut pass = Pass {
        contents: &c.file.contents,
        functions: Vec::new(),
        tries: Vec::new(),
        facts: Facts::default(),
    };
    if c.level >= 4 {
        pass.walk_program(program, &mut ());
    }
    pass.facts
}

struct Pass<'c> {
    contents: &'c [u8],
    /// (named, returns never, `@throws` an UnhandledMatchError) of the enclosing function-likes.
    functions: Vec<(bool, bool, bool)>,
    /// TryCatchTypeVisitor: the enclosing `try` statements' depth in `functions` and caught class names.
    tries: Vec<(usize, Vec<Span>)>,
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
        // hasUnhandledMatchErrorThrowsTag(); ponytail: the tag's class names by their last segment
        let throws = doc_before(self.contents, start.start.offset as usize).is_some_and(|doc| {
            String::from_utf8_lossy(doc).lines().any(|line| {
                let mut words = line.trim_start_matches([' ', '\t', '*', '/']).split_whitespace();
                matches!(words.next(), Some("@throws" | "@phpstan-throws"))
                    && words.next().is_some_and(|t| {
                        t.split('|').any(|class| {
                            let class = class.rsplit('\\').next().unwrap_or_default().to_ascii_lowercase();
                            ["unhandledmatcherror", "error", "throwable"].contains(&class.as_str())
                        })
                    })
            })
        });
        self.functions.push((named, native || doc, throws));
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
    fn walk_in_try(&mut self, t: &'ast Try<'arena>, _: &mut ()) {
        fn names(h: &Hint, out: &mut Vec<Span>) {
            match h {
                Hint::Union(u) => {
                    names(u.left, out);
                    names(u.right, out);
                }
                h => out.push(h.span()),
            }
        }
        let mut caught = Vec::new();
        for clause in t.catch_clauses.iter() {
            names(&clause.hint, &mut caught);
        }
        self.tries.push((self.functions.len(), caught));
        self.list(t.block.statements.as_slice());
        if let Some(f) = &t.finally_clause {
            self.list(f.block.statements.as_slice());
        }
    }
    fn walk_out_try(&mut self, _: &'ast Try<'arena>, _: &mut ()) {
        self.tries.pop();
    }
    fn walk_in_property_hook(&mut self, h: &'ast PropertyHook<'arena>, _: &mut ()) {
        self.enter(true, h.span(), None);
    }
    fn walk_out_property_hook(&mut self, _: &'ast PropertyHook<'arena>, _: &mut ()) {
        self.functions.pop();
    }
    fn walk_in_match(&mut self, m: &'ast Match<'arena>, _: &mut ()) {
        if self.functions.iter().rev().find(|(named, ..)| *named).is_some_and(|(.., throws)| *throws) {
            self.facts.throws.insert(m.span());
        }
        let depth = self.functions.len();
        let caught: Vec<Span> = self.tries.iter().filter(|(d, _)| *d == depth).flat_map(|(_, c)| c.clone()).collect();
        if !caught.is_empty() {
            self.facts.caught.insert(m.span(), caught);
        }
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
            .find(|(named, ..)| *named)
            .or(self.functions.last());
        if function.is_some_and(|(_, never, _)| *never) {
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
        fn walk_in_match(&mut self, m: &'ast Match<'arena>, _: &mut ()) {
            for arm in m.arms.iter() {
                if let MatchArm::Expression(a) = arm {
                    self.spans.extend(a.conditions.iter().map(|c| c.span()));
                }
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

    /// The rules run at level 4.
    fn conditions_checked(&self) -> bool {
        self.level >= 4
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
        let Some(value) = self.constant(e) else {
            self.no_condition(unparen(e).span(), &format!("{identifier}True"));
            return None;
        };
        if !self.deferred(e) {
            let line = self.line_of(line_of);
            let (text, suffix) = if value {
                ("true", "True")
            } else {
                ("false", "False")
            };
            self.condition_error(
                unparen(e).span(),
                line,
                &format!("{identifier}{suffix}"),
                value,
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
        c.no_condition(u.span(), "booleanNot.alwaysTrue");
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
        c.condition_error(
            u.span(),
            line,
            &format!("booleanNot.always{suffix}"),
            !value,
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
    if kind == "Instanceof" && !matches!(b.rhs, Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) {
        // ImpossibleInstanceOfRule: a class expression that is neither a string nor an object
        let allowed = Type::Union(vec![Type::String, Type::ObjectWithoutClass]);
        let found = c.find_type_to_check(b.rhs, &|t| !is_super_type_of(&allowed, t).yes());
        if found.ty.is_some_and(|t| !is_super_type_of(&allowed, &t).yes())
            && let (Some(left), Some(class)) = (c.phpstan_type(b.lhs), c.phpstan_type(b.rhs))
        {
            let (l, r) = super::super::declared::with_classes(c, || (left.describe(Level::TypeOnly), class.describe(Level::TypeOnly)));
            let line = c.line_of(b.span());
            c.report_at_line(line, "instanceof.invalidExprType", format!("Instanceof between {l} and {r} results in an error."), None);
            return;
        }
    }
    let value = c.exact(b.span());
    let right = match (kind, b.rhs) {
        // ImpossibleInstanceOfRule: a class name, as ObjectType
        ("Instanceof", Expression::Identifier(id)) => Some(Type::Object(c.resolved(id), vec![])),
        ("Instanceof", Expression::Self_(_) | Expression::Static(_)) => c.class_scope().map(|k| Type::Object(c.display(k), vec![])),
        _ => c.phpstan_type(b.rhs),
    };
    let (Some(value), Some(left), Some(right)) = (value, c.phpstan_type(b.lhs), right) else {
        c.no_condition(b.span(), &format!("{identifier}.alwaysTrue"));
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
    let line = c.line_of(b.span());
    c.condition_error(b.span(), line, &format!("{identifier}.always{suffix}"), value, message, tip.as_deref());
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
    let line = c.line_of(b.span());
    c.condition_error(
        b.span(),
        line,
        &format!("{identifier}.always{suffix}"),
        value,
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
            Some(noop::Function::Builtin(b)) if let Some(value) = c.special(&b.name.to_ascii_lowercase(), &call.argument_list) => {
                return impossible_check(c, call.span(), value, "function", subject, &call.argument_list);
            }
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
    let last = c.last_condition(span);
    let Some(value) = value.filter(|v| !(*v && last == Some(true))) else {
        c.no_condition(span, &format!("{identifier}.impossibleType"));
        return;
    };
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
    c.condition_error(
        span,
        line,
        &format!("{identifier}.{suffix}"),
        value,
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

/// A void call's value is null.
fn void_null(t: Type) -> Type {
    if t == Type::Void { Type::Null } else { t }
}

/// A type of one value: what a match arm condition removes from the subject.
fn single(t: &Type) -> bool {
    matches!(
        t,
        Type::ConstInt(_) | Type::ConstString(_) | Type::ConstBool(_) | Type::Null | Type::EnumCase(..)
    )
}

/// `$subject === $arm` as a constant: the same single value, or types without a common value.
fn identical(subject: &Type, arm: &Type) -> Option<bool> {
    if single(subject) && subject == arm {
        return Some(true);
    }
    (is_super_type_of(subject, arm) == Tri::No).then_some(false)
}

impl Checker<'_, '_> {
    /// An enum's cases, or None when the class isn't an enum.
    fn enum_cases(&self, class: &str) -> Option<Vec<Type>> {
        let k = self.codebase.get_class_like(class.as_bytes())?;
        (k.kind == SymbolKind::Enum).then(|| {
            k.enum_cases
                .keys()
                .map(|case| Type::EnumCase(class.to_string(), String::from_utf8_lossy(case.as_bytes()).into_owned()))
                .collect()
        })
    }

    /// A type `remove()` takes values from: scalars, null, enums and their cases.
    fn subtractable(&self, t: &Type) -> bool {
        match t {
            Type::Union(ts) => ts.iter().all(|t| self.subtractable(t)),
            Type::Object(class, _) | Type::Static(class, _) => self.enum_cases(class).is_some(),
            t => matches!(
                t,
                Type::Int
                    | Type::IntRange(..)
                    | Type::ConstInt(_)
                    | Type::String
                    | Type::ConstString(_)
                    | Type::Bool
                    | Type::ConstBool(_)
                    | Type::Null
                    | Type::EnumCase(..)
            ),
        }
    }

    /// `getEnumCases()` spelled out: an enum object becomes the union of its cases.
    fn expand_enums(&self, t: Type) -> Type {
        match t {
            Type::Union(ts) => union(ts.into_iter().map(|t| self.expand_enums(t)).collect()),
            Type::Object(ref class, _) | Type::Static(ref class, _) => {
                let cases = self.enum_cases(class);
                cases.map_or(t, union)
            }
            t => t,
        }
    }

    /// The subject's type after an arm of one value didn't match (TypeCombinator::remove()).
    fn remove(&self, t: Type, value: &Type) -> Type {
        match t {
            Type::Union(ts) => union(ts.into_iter().map(|t| self.remove(t, value)).collect()),
            t if t == *value => Type::Never(NeverKind::Implicit),
            Type::Bool if matches!(value, Type::ConstBool(_)) => Type::ConstBool(*value == Type::ConstBool(false)),
            Type::Int | Type::IntRange(..) if let Type::ConstInt(n) = value => {
                let (min, max) = match t {
                    Type::IntRange(min, max) => (min, max),
                    _ => (None, None),
                };
                if min.is_some_and(|m| *n < m) || max.is_some_and(|m| *n > m) {
                    return t;
                }
                union(vec![
                    Type::int_range(min, n.checked_sub(1)),
                    Type::int_range(n.checked_add(1), max),
                ])
            }
            Type::Object(ref class, _) | Type::Static(ref class, _)
                if matches!(value, Type::EnumCase(c, _) if c.eq_ignore_ascii_case(class)) =>
            {
                let cases = self.enum_cases(class);
                cases.map_or(t, |cases| union(cases.into_iter().filter(|case| case != value).collect()))
            }
            t => t,
        }
    }
}

/// `getFiniteTypes()`: the values a type has, none when it has infinitely many.
fn finite(c: &Checker, t: &Type) -> Vec<Type> {
    match c.expand_enums(t.clone()) {
        Type::Union(ts) => {
            let all: Vec<Vec<Type>> = ts.iter().map(|t| finite(c, t)).collect();
            if all.iter().any(Vec::is_empty) { Vec::new() } else { all.concat() }
        }
        Type::Bool => vec![Type::ConstBool(true), Type::ConstBool(false)],
        t if single(&t) || matches!(t, Type::ConstFloat(_)) => vec![t],
        _ => Vec::new(),
    }
}

/// `getConstantStrings()`
fn constant_strings(t: &Type) -> Vec<String> {
    match t {
        Type::ConstString(s) => vec![s.clone()],
        Type::Union(ts) => ts.iter().map(constant_strings).try_fold(Vec::new(), |mut all, s| {
            (!s.is_empty()).then(|| {
                all.extend(s);
                all
            })
        }).unwrap_or_default(),
        _ => Vec::new(),
    }
}

/// `isConstantArray()` is not no: a constant array is among the type's arrays.
fn has_constant_array(t: &Type) -> bool {
    match t {
        Type::Union(ts) | Type::Intersection(ts) => ts.iter().any(has_constant_array),
        t => matches!(t, Type::ConstArray(_)),
    }
}

/// `TrinaryLogic::extremeIdentity()`: yes or no when all agree, else maybe.
fn extreme(results: impl IntoIterator<Item = Tri>) -> Tri {
    results.into_iter().reduce(|a, b| if a == b { a } else { Tri::Maybe }).unwrap_or(Tri::Maybe)
}

/// The constant result: true for yes, false for no.
fn decided(t: Tri) -> Option<bool> {
    (!t.maybe()).then(|| t.yes())
}

impl Checker<'_, '_> {
    /// ImpossibleCheckTypeHelper's own answers for in_array(), method_exists(), property_exists() and the
    /// class_exists() family, with what their TypeSpecifying extensions narrow; `None` for the other functions.
    fn special(&self, name: &str, args: &ArgumentList) -> Option<Option<bool>> {
        let arg = |i: usize| args.arguments.get(i).and_then(|a| self.phpstan_type(a.value()));
        Some(match name {
            "in_array" => (|| self.in_array(arg(0)?, arg(1)?, arg(2)))(),
            "method_exists" => (|| self.method_exists(arg(0)?, arg(1)?))(),
            "property_exists" => (|| self.property_exists(arg(0)?, arg(1)?))(),
            // the always true result is suppressed: autoloading can fail
            "class_exists" | "interface_exists" | "trait_exists" | "enum_exists" => {
                let strings = arg(0).map(|t| constant_strings(&t)).unwrap_or_default();
                let wrong_kind = |s: &String| {
                    self.codebase.get_class_like(s.trim_start_matches('\\').as_bytes()).is_some_and(|k| match name {
                        "class_exists" => matches!(k.kind, SymbolKind::Interface | SymbolKind::Trait),
                        "interface_exists" => k.kind != SymbolKind::Interface,
                        "trait_exists" => k.kind != SymbolKind::Trait,
                        _ => k.kind != SymbolKind::Enum,
                    })
                };
                (!strings.is_empty() && strings.iter().all(wrong_kind)).then_some(false)
            }
            _ => return None,
        })
    }

    fn in_array(&self, needle: Type, haystack: Type, strict: Option<Type>) -> Option<bool> {
        if !haystack.is_array().yes() {
            return None;
        }
        let value = haystack.iterable_value_type();
        let is_enum = |t: &Type| {
            let values = finite(self, t);
            !values.is_empty() && values.iter().all(|t| matches!(t, Type::EnumCase(..)))
        };
        if strict != Some(Type::ConstBool(true)) && !is_enum(&needle) && !is_enum(&value) {
            return None;
        }
        let needles = finite(self, &needle);
        let values = finite(self, &value);
        let needle_super = is_super_type_of(&needle, &value);
        if !has_constant_array(&haystack) {
            if haystack.is_iterable_at_least_once().yes() {
                return (needles.len() == 1 && values.len() == 1).then(|| decided(needle_super)).flatten();
            }
            if !needle_super.no() {
                // the array might be empty
                return None;
            }
        }
        if !matches!(&haystack, Type::ConstArray(a) if a.values.is_empty()) {
            let arrays = match &haystack {
                Type::Union(ts) => ts.clone(),
                t => vec![t.clone()],
            };
            (arrays.len() != 1 || !matches!(value, Type::Never(_))).then_some(())?;
            if !needle_super.no() {
                if needles.is_empty() {
                    return None;
                }
                for array in &arrays {
                    // a general array cannot guarantee any specific value is present
                    let Type::ConstArray(a) = array else { return None };
                    let guaranteed: Vec<Type> = a
                        .values
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| !a.optional.contains(i))
                        .filter_map(|(_, v)| <[Type; 1]>::try_from(finite(self, v)).ok())
                        .map(|[v]| v)
                        .collect();
                    if !needles.iter().all(|n| guaranteed.iter().any(|g| is_super_type_of(g, n).yes())) {
                        return None;
                    }
                }
            }
            // (a needle that is a supertype of the values has finite types, so the values have them too)
        }
        // the narrowed needle and the non-empty haystack
        decided(is_super_type_of(&value, &needle).and(haystack.is_iterable_at_least_once()))
    }

    /// `ObjectType::hasMethod()`, with whether the class declares the method natively.
    fn object_has_method(&self, class: &str, method: &str) -> (Tri, bool) {
        let Some(k) = self.codebase.get_class_like(class.as_bytes()) else {
            return (Tri::Maybe, false);
        };
        if members::method(self, k, method.as_bytes()).is_some() {
            (Tri::Yes, true)
        } else if members::annotation_method(self, k, method.as_bytes()).is_some() {
            (Tri::Yes, false)
        } else {
            (if k.flags.is_final() || k.kind == SymbolKind::Enum { Tri::No } else { Tri::Maybe }, false)
        }
    }

    fn method_exists(&self, object: Type, method: Type) -> Option<bool> {
        if let Type::ConstString(class) = &object
            && self.codebase.get_class_like(class.as_bytes()).is_none()
        {
            return Some(false);
        }
        let Type::ConstString(method) = method else { return None };
        let object = match object {
            Type::ConstString(class) => Type::Object(class, Vec::new()),
            t => t,
        };
        // ponytail: a generic class-string's object (`class-string<T>&literal-string`) is not decided, Mago's
        // class-string types lose it
        let found: Vec<_> =
            object.object_class_names().iter().map(|class| self.object_has_method(class, &method)).collect();
        match extreme(found.iter().map(|f| f.0)) {
            Tri::Yes => found.iter().all(|f| f.1).then_some(true),
            Tri::No => Some(false),
            Tri::Maybe => None,
        }
    }

    /// PropertyExistsTypeSpecifyingExtension: the object narrowed to `object&hasProperty(name)`; a property that
    /// isn't native (an object shape's, an `@property` tag's) narrows the call instead.
    fn property_exists(&self, object: Type, property: Type) -> Option<bool> {
        let Type::ConstString(property) = property else { return None };
        if property.is_empty() {
            return None;
        }
        let objects = match &object {
            Type::Union(ts) => ts.clone(),
            t => vec![t.clone()],
        };
        let dollar = format!("${property}");
        let mut found = Vec::new();
        for object in &objects {
            found.push(match object {
                Type::ObjectShape(properties) => {
                    if properties.iter().any(|(name, _, _)| *name == property) {
                        return None;
                    }
                    Tri::No
                }
                Type::Object(class, _) | Type::Static(class, _) | Type::This(class) => {
                    let Some(k) = self.codebase.get_class_like(class.as_bytes()) else { return None };
                    if self.declared_property(k, dollar.as_bytes()).is_some() {
                        Tri::Yes
                    } else if k.magic_property_ids.contains_key(&mago_word::word(dollar.as_bytes())) {
                        return None;
                    } else {
                        let open = self.allows_dynamic_properties(k) || !(k.flags.is_final() || k.kind == SymbolKind::Enum);
                        if open { Tri::Maybe } else { Tri::No }
                    }
                }
                _ => return None,
            });
        }
        decided(extreme(found))
    }
}

/// MatchExpressionRule: arms always true or false against the subject narrowed by the arms above, and the values no
/// arm handles.
pub(in super::super) fn match_expression(c: &mut Checker, m: &Match) {
    if !c.conditions_checked() {
        return;
    }
    // ponytail: subjects of the types `remove()` handles; arrays, class strings, templates... wait for TypeCombinator
    let Some(mut remaining) = c.phpstan_type(m.expression).map(void_null).filter(|t| c.subtractable(t)) else {
        return;
    };
    // an enum of one case is that case
    if let expanded @ Type::EnumCase(..) = c.expand_enums(remaining.clone()) {
        remaining = expanded;
    }
    let constant_subject = matches!(remaining, Type::ConstBool(_));
    let last_arm = m.arms.len().saturating_sub(1);
    let mut dead = false;
    let mut default = false;
    for (i, arm) in m.arms.iter().enumerate() {
        if dead {
            break;
        }
        let MatchArm::Expression(arm) = arm else {
            default = true;
            continue;
        };
        for cond in arm.conditions.iter() {
            let Some(value) = c.phpstan_type(cond).map(void_null) else {
                continue;
            };
            let subject = remaining.clone();
            if single(&value) {
                remaining = c.remove(remaining, &value);
            }
            let Some(result) = identical(&subject, &value) else {
                c.no_condition(cond.span(), "match.alwaysTrue");
                continue;
            };
            dead |= result;
            if constant_subject && c.constant(cond).is_none() || c.deferred(cond) || result && i == last_arm {
                c.no_condition(cond.span(), "match.alwaysTrue");
                continue;
            }
            let (mut s, v) =
                super::super::declared::with_classes(c, || (subject.describe(Level::Value), value.describe(Level::Value)));
            // `$this` narrowed to a case of its enum
            if let (Expression::Variable(Variable::Direct(this)), Type::EnumCase(class, _)) = (unparen(m.expression), &subject)
                && this.name == b"$this"
            {
                s = format!("$this({class})&{s}");
            }
            let (text, tip) = if result { ("true", Some(REMOVE_TIP)) } else { ("false", None) };
            let line = c.line_of(cond.span());
            c.condition_error(
                cond.span(),
                line,
                if result { "match.alwaysTrue" } else { "match.alwaysFalse" },
                result,
                format!("Match arm comparison between {s} and {v} is always {text}."),
                tip,
            );
        }
    }
    if default || dead {
        return;
    }
    let remaining = c.expand_enums(remaining);
    // isUnhandledMatchErrorCaught()
    let caught = c.conditions.caught.get(&m.span()).is_some_and(|names| {
        names.iter().any(|n| {
            let name = c.names.get(n);
            ["UnhandledMatchError", "Error", "Throwable"].iter().any(|s| name.eq_ignore_ascii_case(s.as_bytes()))
        })
    });
    if matches!(remaining, Type::Never(_)) || caught || c.conditions.throws.contains(&m.span()) {
        return;
    }
    let values = if matches!(remaining, Type::Union(_)) { "values" } else { "value" };
    let described = super::super::declared::with_classes(c, || remaining.describe(Level::Value));
    c.report(
        m.span(),
        "match.unhandled",
        format!("Match expression does not handle remaining {values}: {described}"),
        None,
    );
}

/// The message prefix of a ConstantConditionInTraitCollector entry: `rule`, trait, expression, value (`-` for no
/// error), then the message.
const IN_TRAIT: &str = "\u{1e}in-trait\u{1f}";

impl Checker<'_, '_> {
    /// A constant-condition error, collected per class context in a trait (ConstantConditionInTraitHelper::emitError()).
    fn condition_error(&mut self, key: Span, line: i64, identifier: &str, value: bool, message: String, tip: Option<&str>) {
        let message = match self.in_trait() {
            Some(t) => format!("{IN_TRAIT}{}\u{1f}{t}\u{1f}{}\u{1f}{value}\u{1f}{message}", rule(identifier), key.start.offset),
            None => message,
        };
        self.report_at_line(line, identifier, message, tip);
    }

    /// emitNoError(): the rule found nothing at `key` in this class context.
    fn no_condition(&mut self, key: Span, identifier: &str) {
        if let Some(t) = self.in_trait() {
            let message = format!("{IN_TRAIT}{}\u{1f}{t}\u{1f}{}\u{1f}-\u{1f}", rule(identifier), key.start.offset);
            let line = self.line_of(key);
            self.report_at_line(line, identifier, message, None);
        }
    }
}

/// The rule of an identifier: its first segment.
fn rule(identifier: &str) -> &str {
    identifier.split('.').next().unwrap_or_default()
}

/// ConstantConditionInTraitRule: a trait's condition reports when every class context agrees on it, once without the
/// context when they report the same message.
pub(crate) fn conditions_in_traits(errors: Vec<Error>) -> Vec<Error> {
    let (collected, mut errors): (Vec<Error>, Vec<Error>) =
        errors.into_iter().partition(|e| e.message.starts_with(IN_TRAIT));
    // rule, trait, expression -> class context -> the errors, or none
    let mut groups: BTreeMap<(String, String, String), BTreeMap<String, Vec<(String, Error)>>> = BTreeMap::new();
    for mut e in collected {
        let fields: Vec<String> = e.message[IN_TRAIT.len()..].splitn(5, '\u{1f}').map(str::to_string).collect();
        let [rule, t, key, value, message] = <[String; 5]>::try_from(fields).expect("an entry has five fields");
        let context = groups.entry((rule, t, key)).or_default().entry(e.file.clone()).or_default();
        if value != "-" {
            e.message = message;
            context.push((value, e));
        }
    }
    for contexts in groups.into_values() {
        let values: BTreeSet<&str> = contexts
            .values()
            .map(|es| es.first().map_or("-", |(v, _)| v.as_str()))
            .collect();
        if values.len() > 1 {
            continue;
        }
        let mut unique: Vec<Error> = Vec::new();
        for (_, e) in contexts.into_values().flatten() {
            if !unique.iter().any(|u| u.message == e.message) {
                unique.push(e);
            }
        }
        if let [one] = unique.as_mut_slice() {
            // removeTraitContext()
            let file = one.trait_file_path.take().unwrap_or_default();
            one.file = file.clone();
            one.file_path = file;
        }
        errors.extend(unique);
    }
    errors
}

/// The match subjects' types before any arm. Mago analyses a subject again in each arm's scope, so the type it keeps
/// is the last arm's: these come from a copy of the file whose matches have only a `default` arm, at the same offsets.
/// Also the subjects whose arms are too short for that: their types are unknown.
pub(in super::super) fn match_subject_types(
    file: &File,
    program: &Program,
    codebase: &CodebaseMetadata,
    php_version: u32,
) -> (HashMap<Span, TUnion>, Vec<Span>) {
    /// The matches with arms: subject, the start and end between their braces, nesting depth.
    struct Matches(Vec<(Span, usize, usize, usize)>, usize);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Matches {
        fn walk_in_match(&mut self, m: &'ast Match<'arena>, _: &mut ()) {
            if !m.arms.is_empty() {
                let (start, end) = (m.left_brace.end.offset as usize, m.right_brace.start.offset as usize);
                self.0.push((m.expression.span(), start, end, self.1));
            }
            self.1 += 1;
        }
        fn walk_out_match(&mut self, _: &'ast Match<'arena>, _: &mut ()) {
            self.1 -= 1;
        }
    }
    let mut matches = Matches(Vec::new(), 0);
    matches.walk_program(program, &mut ());
    let mut types = HashMap::new();
    let mut unknown = Vec::new();
    if matches.0.is_empty() {
        return (types, unknown);
    }
    // a nested match is gone from a copy without its outer match's arms: one copy per depth
    let deepest = matches.0.iter().map(|m| m.3).max().unwrap_or_default();
    for depth in 0..=deepest {
        let mut contents = file.contents.to_vec();
        let mut subjects = Vec::new();
        for &(subject, s, e, _) in matches.0.iter().filter(|m| m.3 == depth) {
            let mut arms = file.contents[s..e].iter().map(|b| if *b == b'\n' { b'\n' } else { b' ' }).collect::<Vec<_>>();
            // `default=>1`, its tokens between the arms' line breaks; ponytail: shorter arms keep Mago's type
            let mut at = 0;
            let placed = [b"default".as_slice(), b"=>", b"1"].iter().all(|token| {
                while at + token.len() <= arms.len() && arms[at..at + token.len()].contains(&b'\n') {
                    at += 1;
                }
                let room = at + token.len() <= arms.len();
                if room {
                    arms[at..at + token.len()].copy_from_slice(token);
                    at += token.len();
                }
                room
            });
            if placed {
                contents[s..e].copy_from_slice(&arms);
                subjects.push(subject);
            } else {
                unknown.push(subject);
            }
        }
        let copy = File::new(file.name.clone(), file.file_type, file.path.clone(), Cow::Owned(contents));
        let arena = LocalArena::new();
        let (program, names) = parse(&arena, &copy);
        types.extend(super::super::infer::analyse(&arena, &copy, program, &names, codebase, php_version, subjects));
    }
    (types, unknown)
}
