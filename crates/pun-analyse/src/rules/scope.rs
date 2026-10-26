//! The part of PHPStan's scope the level-0 rules read: whether `class_exists('X')` / `function_exists('f')` is
//! known to be true here (`isInClassExists()` / `isInFunctionExists()`). PHPStan gets it from TypeSpecifier
//! narrowing; this follows the same constructs: `if`/`elseif`/`else` branches and what an always-terminating
//! branch leaves after the `if`, `&&`/`||`/`!`, `? :`, `match (true)`, variables holding such a check, closures
//! inheriting their parent's scope, and checks a constructor ends with (remembered in the class's other methods).

use std::collections::HashMap;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Type, is_super_type_of};

use super::Checker;
use super::members::Method;

/// A `*_exists()` call known to be true; names as the string argument has them, without a leading `\`.
#[derive(Clone, Debug, Eq)]
pub enum Guard {
    Class(String),
    Function(String),
    /// `defined('Foo::BAR')`: the class part lowercase, as `foo::BAR`.
    ClassConstant(String),
    /// `defined('FOO')`: the global constant `FOO` (its fully qualified name).
    Constant(String),
    /// `method_exists(Foo::class, 'bar')`: receiver key and the method's original spelling.
    Method(String),
    /// The precondition's source: method_exists's receiver, or is_callable's literal array.
    MethodSource(String, Span),
    /// `!($this instanceof Foo)`: in a class that is a Foo, the code here is unreachable.
    ThisNotInstanceOf(String),
    /// `PHP_VERSION_ID` compared with an integer: its range here, bounds inclusive (`None` unbounded).
    PhpVersion(Option<i64>, Option<i64>),
    /// An expression with a type in the scope (`hasExpressionType()`), by its key (`$this->name`).
    Expr(String),
    /// A class-constant expression holder. Unlike a readonly property holder, a containing fetch is invalidated
    /// when an impure call mutates its receiver, even if that receiver occurs below a readonly property.
    ConstantFetch(String),
    /// A selector's finite string type in a branch, used by conditional expression holders at joins.
    ExprStrings(String, Vec<String>),
    /// A holder missing on these paths. Selecting a constant name can rule every missing path out.
    ExprWhen(String, Vec<Vec<(String, Vec<String>)>>),
    /// A remembered true file-existence call, read by RequireFileExistsRule.
    File(super::include_file::FileGuard),
}

impl PartialEq for Guard {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Method(a), Self::Method(b)) | (Self::MethodSource(a, _), Self::MethodSource(b, _)) => {
                let (a, am) = a.split_once("::").expect("method receiver key");
                let (b, bm) = b.split_once("::").expect("method receiver key");
                a == b && am.eq_ignore_ascii_case(bm)
            }
            (Self::Class(a), Self::Class(b))
            | (Self::Function(a), Self::Function(b))
            | (Self::ClassConstant(a), Self::ClassConstant(b))
            | (Self::Constant(a), Self::Constant(b))
            | (Self::ThisNotInstanceOf(a), Self::ThisNotInstanceOf(b))
            | (Self::Expr(a), Self::Expr(b))
            | (Self::ConstantFetch(a), Self::ConstantFetch(b)) => a == b,
            (Self::ExprStrings(a, av), Self::ExprStrings(b, bv)) => a == b && av == bv,
            (Self::ExprWhen(a, av), Self::ExprWhen(b, bv)) => a == b && av == bv,
            (Self::PhpVersion(a, b), Self::PhpVersion(c, d)) => (a, b) == (c, d),
            (Self::File(a), Self::File(b)) => a == b,
            _ => false,
        }
    }
}

fn expression_guard(g: &Guard) -> bool {
    matches!(g, Guard::Expr(_) | Guard::ConstantFetch(..) | Guard::ExprStrings(..) | Guard::ExprWhen(..))
}

pub(super) fn definite_expression(g: &Guard) -> bool {
    matches!(g, Guard::Expr(_) | Guard::ConstantFetch(..))
}

/// Remember existence predicates in a boolean variable, not the source expression's type holder. Merely
/// assigning `$value = $receiver::NAME` does not give the fetch an expression type when `$value` is tested.
pub(super) fn remembered_guard(g: &Guard) -> bool {
    !expression_guard(g) && !matches!(g, Guard::File(_))
}

/// Guards from `expr` being true. `var` looks a variable's remembered guards up.
pub fn truthy(
    expr: &Expression,
    var: &dyn Fn(&[u8]) -> Vec<Guard>,
    resolve: &dyn Fn(&Identifier) -> String,
    function: &dyn Fn(&Identifier) -> String,
) -> Vec<Guard> {
    let mut guards = truthy_of(expr, var, resolve, function);
    if !compound(expr) {
        super::property_access::narrowed(expr, true, resolve, &mut guards);
        constant_narrowed(expr, true, resolve, function, &mut guards);
    }
    guards
}

/// `!`, `&&`, `||` and parentheses, whose operands narrow on their own.
fn compound(expr: &Expression) -> bool {
    match expr {
        Expression::Parenthesized(_) => true,
        Expression::UnaryPrefix(u) => matches!(u.operator, UnaryPrefixOperator::Not(_)),
        Expression::Binary(b) => matches!(
            b.operator,
            BinaryOperator::And(_)
                | BinaryOperator::LowAnd(_)
                | BinaryOperator::Or(_)
                | BinaryOperator::LowOr(_)
        ),
        _ => false,
    }
}

fn truthy_of(
    expr: &Expression,
    var: &dyn Fn(&[u8]) -> Vec<Guard>,
    resolve: &dyn Fn(&Identifier) -> String,
    function: &dyn Fn(&Identifier) -> String,
) -> Vec<Guard> {
    match expr {
        Expression::Parenthesized(p) => truthy(p.expression, var, resolve, function),
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
            falsy(u.operand, var, resolve, function)
        }
        Expression::Binary(b) => match b.operator {
            BinaryOperator::And(_) | BinaryOperator::LowAnd(_) => {
                let mut g = truthy(b.lhs, var, resolve, function);
                g.extend(truthy(b.rhs, var, resolve, function));
                g
            }
            BinaryOperator::Or(_) | BinaryOperator::LowOr(_) => {
                both(truthy(b.lhs, var, resolve, function), truthy(b.rhs, var, resolve, function))
            }
            BinaryOperator::Identical(_) | BinaryOperator::Equal(_) => {
                match (bool_literal(b.lhs), bool_literal(b.rhs)) {
                    (_, Some(true)) => truthy(b.lhs, var, resolve, function),
                    (Some(true), _) => truthy(b.rhs, var, resolve, function),
                    (_, Some(false)) => falsy(b.lhs, var, resolve, function),
                    (Some(false), _) => falsy(b.rhs, var, resolve, function),
                    _ => Vec::new(),
                }
            }
            BinaryOperator::NotIdentical(_) | BinaryOperator::NotEqual(_) => {
                match (bool_literal(b.lhs), bool_literal(b.rhs)) {
                    (_, Some(false)) => truthy(b.lhs, var, resolve, function),
                    (Some(false), _) => truthy(b.rhs, var, resolve, function),
                    (_, Some(true)) => falsy(b.lhs, var, resolve, function),
                    (Some(true), _) => falsy(b.rhs, var, resolve, function),
                    _ => Vec::new(),
                }
            }
            _ => php_version(b, true).into_iter().collect(),
        },
        Expression::Variable(Variable::Direct(v)) => var(v.name),
        Expression::Assignment(a) => truthy(a.rhs, var, resolve, function),
        Expression::Call(Call::Function(call)) => {
            let Some(guard) = exists_call(call, resolve) else { return Vec::new(); };
            let mut guards = Vec::new();
            if let Guard::Method(key) = &guard {
                guards.push(Guard::MethodSource(key.clone(), call.argument_list.arguments.first().expect("exists receiver").value().span()));
            }
            guards.push(guard);
            guards
        }
        _ => Vec::new(),
    }
}

/// Guards from `expr` being false.
pub fn falsy(
    expr: &Expression,
    var: &dyn Fn(&[u8]) -> Vec<Guard>,
    resolve: &dyn Fn(&Identifier) -> String,
    function: &dyn Fn(&Identifier) -> String,
) -> Vec<Guard> {
    let mut guards = falsy_of(expr, var, resolve, function);
    if !compound(expr) {
        super::property_access::narrowed(expr, false, resolve, &mut guards);
        constant_narrowed(expr, false, resolve, function, &mut guards);
    }
    guards
}

fn contains_call(e: &Expression) -> bool {
    struct Calls(bool);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Calls {
        fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, _: &mut ()) {
            self.0 |= matches!(e, Expression::Call(_));
        }
    }
    let mut calls = Calls(false);
    calls.walk_expression(e, &mut ());
    calls.0
}

/// ExprPrinter identities for the value expressions used as class-constant receivers/selectors. Unlike
/// inferred types, these identities distinguish two variables holding the same class and a dynamic selector
/// from its current finite values. Parentheses and imported names do not change the identity.
/// Named calls keep ExprPrinter's resolved names and argument syntax, not their return types or acceptors.
/// Dynamic callees, indirect variables and dynamic property selectors remain unavailable identities.
fn expression_key(e: &Expression, resolve: &dyn Fn(&Identifier) -> String, function: &dyn Fn(&Identifier) -> String) -> Option<String> {
    let key = |e| expression_key(e, resolve, function);
    Some(match super::types::flow::unparen(e) {
        Expression::Variable(Variable::Direct(v)) => String::from_utf8_lossy(v.name).into_owned(),
        Expression::Identifier(id) => format!("\\{}", resolve(id)),
        Expression::Self_(_) => "self".into(),
        Expression::Static(_) => "static".into(),
        Expression::Parent(_) => "parent".into(),
        Expression::Literal(Literal::Integer(i)) => format!("int({})", i.value?),
        // Escape bytes, not PHP variable spellings inside string literals: invalidating `$receiver` must
        // not invalidate a call whose argument merely contains the text '$receiver'.
        Expression::Literal(Literal::String(s)) => format!("string({})", s.value?.iter().map(|b| format!("{b:02x}")).collect::<String>()),
        Expression::Literal(Literal::True(_)) => "true".into(),
        Expression::Literal(Literal::False(_)) => "false".into(),
        Expression::Literal(Literal::Null(_)) => "null".into(),
        Expression::Call(call) => {
            let (callee, args) = match call {
                Call::Function(f) => {
                    let Expression::Identifier(id) = f.function else { return None };
                    (function(id), &f.argument_list)
                }
                Call::Method(m) => {
                    let ClassLikeMemberSelector::Identifier(id) = &m.method else { return None };
                    (format!("{}->{}", key(m.object)?, String::from_utf8_lossy(id.value)), &m.argument_list)
                }
                Call::StaticMethod(m) => {
                    let ClassLikeMemberSelector::Identifier(id) = &m.method else { return None };
                    (format!("{}::{}", key(m.class)?, String::from_utf8_lossy(id.value)), &m.argument_list)
                }
                _ => return None,
            };
            let args: Option<Vec<_>> = args.arguments.iter().map(|arg| {
                let value = key(arg.value())?;
                Some(match arg {
                    Argument::Named(n) => format!("{}: {value}", String::from_utf8_lossy(n.name.value)),
                    Argument::Positional(p) => format!("{}{value}", if p.ellipsis.is_some() { "..." } else { "" }),
                })
            }).collect();
            format!("{callee}({})", args?.join(", "))
        }
        Expression::ArrayAccess(a) => format!("{}[{}]", key(a.array)?, key(a.index)?),
        Expression::Access(Access::Property(p)) => {
            let ClassLikeMemberSelector::Identifier(id) = &p.property else { return None };
            format!("{}->{}", key(p.object)?, String::from_utf8_lossy(id.value))
        }
        Expression::Access(Access::StaticProperty(p)) => {
            let Variable::Direct(v) = &p.property else { return None };
            format!("{}::{}", key(p.class)?, String::from_utf8_lossy(v.name))
        }
        Expression::Access(Access::ClassConstant(a)) => constant_key(a, resolve, function)?,
        _ => return None,
    })
}

fn constant_key(a: &ClassConstantAccess, resolve: &dyn Fn(&Identifier) -> String, function: &dyn Fn(&Identifier) -> String) -> Option<String> {
    let name = match &a.constant {
        ClassLikeConstantSelector::Identifier(id) => String::from_utf8_lossy(id.value).into_owned(),
        ClassLikeConstantSelector::Expression(e) => {
            // PHPStan's Printer::pObjectProperty() normalizes only identifier-shaped literal strings, not a
            // variable or a class constant which happens to have that same inferred value.
            static IDENTIFIER: std::sync::LazyLock<pcre2::bytes::Regex> = std::sync::LazyLock::new(|| {
                pcre2::bytes::Regex::new(r"^[a-zA-Z_\x80-\xff][a-zA-Z0-9_\x80-\xff]*$").expect("identifier regexp")
            });
            match super::types::flow::unparen(e.expression) {
                Expression::Literal(Literal::String(s)) if IDENTIFIER.is_match(s.value?).expect("byte regexp") => {
                    String::from_utf8_lossy(s.value?).into_owned()
                }
                _ => format!("{{{}}}", expression_key(e.expression, resolve, function)?),
            }
        }
        ClassLikeConstantSelector::Missing(_) => return None,
    };
    Some(format!("{}::{name}", expression_key(a.class, resolve, function)?))
}

/// TypeSpecifier's subject entries, not every fetch evaluated by the condition. In particular a loose
/// comparison with a nonzero number does not specify the subject in its unequal branch.
fn constant_narrowed(e: &Expression, truthy: bool, resolve: &dyn Fn(&Identifier) -> String, function: &dyn Fn(&Identifier) -> String, out: &mut Vec<Guard>) {
    match super::types::flow::unparen(e) {
        Expression::Access(Access::ClassConstant(a)) => {
            if let Some(key) = constant_key(a, resolve, function) {
                out.push(Guard::ConstantFetch(key));
            }
        }
        Expression::Binary(b) if b.operator.is_comparison() && !matches!(b.operator, BinaryOperator::Spaceship(_)) => {
            // Boolean-literal comparisons delegate to truthy/falsy above. Other call-containing comparisons
            // need the general TypeSpecifier subject/purity engine, not merely a printable expression key.
            if contains_call(e) {
                return;
            }
            if matches!(b.operator, BinaryOperator::Equal(_) | BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_)) {
                let equal = truthy == matches!(b.operator, BinaryOperator::Equal(_));
                let sentinel = |e: &Expression| matches!(e, Expression::Literal(Literal::Null(_) | Literal::True(_) | Literal::False(_)))
                    || matches!(e, Expression::Literal(Literal::Integer(i)) if i.value == Some(0))
                    || matches!(e, Expression::Literal(Literal::String(s)) if s.value == Some(b""));
                if !equal && !sentinel(b.lhs) && !sentinel(b.rhs) {
                    return;
                }
            }
            constant_narrowed(b.lhs, truthy, resolve, function, out);
            constant_narrowed(b.rhs, truthy, resolve, function, out);
        }
        _ => {}
    }
}

fn falsy_of(
    expr: &Expression,
    var: &dyn Fn(&[u8]) -> Vec<Guard>,
    resolve: &dyn Fn(&Identifier) -> String,
    function: &dyn Fn(&Identifier) -> String,
) -> Vec<Guard> {
    match expr {
        Expression::Parenthesized(p) => falsy(p.expression, var, resolve, function),
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
            truthy(u.operand, var, resolve, function)
        }
        Expression::Binary(b) => match b.operator {
            BinaryOperator::Or(_) | BinaryOperator::LowOr(_) => {
                let mut g = falsy(b.lhs, var, resolve, function);
                g.extend(falsy(b.rhs, var, resolve, function));
                g
            }
            BinaryOperator::And(_) | BinaryOperator::LowAnd(_) => {
                both(falsy(b.lhs, var, resolve, function), falsy(b.rhs, var, resolve, function))
            }
            BinaryOperator::Instanceof(_) => match (b.lhs, b.rhs) {
                (Expression::Variable(Variable::Direct(v)), Expression::Identifier(id))
                    if v.name == b"$this" =>
                {
                    vec![Guard::ThisNotInstanceOf(resolve(id))]
                }
                _ => Vec::new(),
            },
            BinaryOperator::Identical(_)
            | BinaryOperator::Equal(_)
            | BinaryOperator::NotIdentical(_)
            | BinaryOperator::NotEqual(_) => {
                let flipped = matches!(
                    b.operator,
                    BinaryOperator::Identical(_) | BinaryOperator::Equal(_)
                );
                match (bool_literal(b.lhs), bool_literal(b.rhs)) {
                    (_, Some(v)) if v != flipped => truthy(b.lhs, var, resolve, function),
                    (Some(v), _) if v != flipped => truthy(b.rhs, var, resolve, function),
                    _ => Vec::new(),
                }
            }
            _ => php_version(b, false).into_iter().collect(),
        },
        _ => Vec::new(),
    }
}

/// What holds when `a` or `b` does: the guards both have, and the hull of their `PHP_VERSION_ID` ranges (exact for
/// the one question rules ask of it, whether the range lies within or outside `[v, max]`).
fn both(a: Vec<Guard>, b: Vec<Guard>) -> Vec<Guard> {
    let mut out: Vec<Guard> = a.iter().filter(|g| b.contains(g)).cloned().collect();
    if let (Some((alo, ahi)), Some((blo, bhi))) = (version_range(&a), version_range(&b)) {
        out.retain(|g| !matches!(g, Guard::PhpVersion(..)));
        out.push(Guard::PhpVersion(
            alo.zip(blo).map(|(x, y)| x.min(y)),
            ahi.zip(bhi).map(|(x, y)| x.max(y)),
        ));
    }
    out
}

/// The intersection of the `PHP_VERSION_ID` ranges among `guards`, `None` when none narrows it.
pub fn version_range(guards: &[Guard]) -> Option<(Option<i64>, Option<i64>)> {
    guards.iter().fold(None, |acc, g| match g {
        Guard::PhpVersion(lo, hi) => {
            let (alo, ahi) = acc.unwrap_or((None, None));
            let hi = match (ahi, *hi) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, b) => a.or(b),
            };
            Some((alo.max(*lo), hi))
        }
        _ => acc,
    })
}

/// `PHP_VERSION_ID <op> N` (either side) being `holds`: the range it leaves.
fn php_version(b: &Binary, holds: bool) -> Option<Guard> {
    let int = |e: &Expression| match e {
        Expression::Literal(Literal::Integer(i)) => i.value.and_then(|v| i64::try_from(v).ok()),
        _ => None,
    };
    let is_version = |e: &Expression| {
        matches!(e, Expression::ConstantAccess(c)
            if c.name.value().strip_prefix(b"\\").unwrap_or(c.name.value()) == b"PHP_VERSION_ID")
    };
    // normalised to `PHP_VERSION_ID <op> n`
    let (n, flip) = if is_version(b.lhs) {
        (int(b.rhs)?, false)
    } else if is_version(b.rhs) {
        (int(b.lhs)?, true)
    } else {
        return None;
    };
    // (strictly less, or-equal) of `PHP_VERSION_ID` against n
    let (less, equal) = match (b.operator, flip) {
        (BinaryOperator::LessThan(_), false) | (BinaryOperator::GreaterThan(_), true) => {
            (true, false)
        }
        (BinaryOperator::LessThanOrEqual(_), false)
        | (BinaryOperator::GreaterThanOrEqual(_), true) => (true, true),
        (BinaryOperator::GreaterThan(_), false) | (BinaryOperator::LessThan(_), true) => {
            (false, false)
        }
        (BinaryOperator::GreaterThanOrEqual(_), false)
        | (BinaryOperator::LessThanOrEqual(_), true) => (false, true),
        _ => return None,
    };
    // when false, `<` becomes `>=` and `<=` becomes `>`
    let (less, equal) = if holds {
        (less, equal)
    } else {
        (!less, !equal)
    };
    Some(match (less, equal) {
        (true, true) => Guard::PhpVersion(None, Some(n)),
        (true, false) => Guard::PhpVersion(None, Some(n - 1)),
        (false, true) => Guard::PhpVersion(Some(n), None),
        (false, false) => Guard::PhpVersion(Some(n + 1), None),
    })
}

fn bool_literal(e: &Expression) -> Option<bool> {
    match e {
        Expression::Literal(Literal::True(_)) => Some(true),
        Expression::Literal(Literal::False(_)) => Some(false),
        _ => None,
    }
}

/// `class_exists('X')` (and interface/trait/enum_exists, which imply it) or `function_exists('f')` with a constant
/// string (or `X::class`) argument.
fn exists_call(call: &FunctionCall, resolve: &dyn Fn(&Identifier) -> String) -> Option<Guard> {
    let Expression::Identifier(id) = call.function else {
        return None;
    };
    // an unqualified call falls back to the global function
    let name = match id {
        Identifier::Local(l) => String::from_utf8_lossy(l.value).into_owned(),
        _ => resolve(id),
    }
    .to_ascii_lowercase();
    let arg = call.argument_list.arguments.first()?.value();
    if name.trim_start_matches('\\') == "method_exists" {
        let class = method_receiver(arg, resolve)?;
        let method = call.argument_list.arguments.get(1)?.value();
        let method = constant_string(method, resolve)?;
        return Some(Guard::Method(format!("{class}::{method}")));
    }
    if name.trim_start_matches('\\') == "is_callable" {
        return super::method_callable::literal_guard(arg, resolve).map(Guard::Method);
    }
    let value = constant_string(arg, resolve)?;
    let value = value.trim_start_matches('\\').to_string();
    match name.trim_start_matches('\\') {
        "class_exists" | "interface_exists" | "trait_exists" | "enum_exists" => {
            Some(Guard::Class(value))
        }
        "function_exists" => Some(Guard::Function(value)),
        // a `define()` statement defines the constant for the rest of the scope
        "define" => Some(Guard::Constant(value)),
        "defined" => Some(match value.split_once("::") {
            Some((class, constant)) => Guard::ClassConstant(format!("{}::{constant}", class.to_ascii_lowercase())),
            None => Guard::Constant(value),
        }),
        _ => None,
    }
}

/// The bounded receivers shared by literal `method_exists()` guards and their type refinement.
pub(super) fn method_receiver(e: &Expression, resolve: &dyn Fn(&Identifier) -> String) -> Option<String> {
    let receiver = match e {
        Expression::Parenthesized(p) => return method_receiver(p.expression, resolve),
        Expression::Access(Access::ClassConstant(ClassConstantAccess {
            class: Expression::Static(_),
            constant: ClassLikeConstantSelector::Identifier(c),
            ..
        })) if c.value.eq_ignore_ascii_case(b"class") => "static".to_string(),
        Expression::Access(Access::ClassConstant(ClassConstantAccess {
            class: Expression::Self_(_),
            constant: ClassLikeConstantSelector::Identifier(c),
            ..
        })) if c.value.eq_ignore_ascii_case(b"class") => "self".to_string(),
        Expression::Variable(Variable::Direct(v)) => return Some(String::from_utf8_lossy(v.name).into_owned()),
        _ => constant_string(e, resolve)?,
    };
    Some(receiver.trim_start_matches('\\').to_ascii_lowercase())
}

/// Collect the pre-guard types in the same inference run as the expressions the rules check.
pub(super) fn typed_method_receivers(program: &Program) -> Vec<Span> {
    struct Wanted(Vec<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
            if matches!(exists_call(call, &|id| String::from_utf8_lossy(id.value()).into_owned()), Some(Guard::Method(_))) {
                self.0.push(call.argument_list.arguments.first().expect("exists receiver").value().span());
            }
        }
    }
    let mut wanted = Wanted(Vec::new());
    wanted.walk_program(program, &mut ());
    wanted.0
}

fn constant_string(e: &Expression, resolve: &dyn Fn(&Identifier) -> String) -> Option<String> {
    match e {
        Expression::Literal(Literal::String(s)) => {
            s.value.map(|v| String::from_utf8_lossy(v).into_owned())
        }
        Expression::Access(Access::ClassConstant(ClassConstantAccess {
            class: Expression::Identifier(id),
            constant: ClassLikeConstantSelector::Identifier(c),
            ..
        })) if c.value.eq_ignore_ascii_case(b"class") => Some(resolve(id)),
        _ => None,
    }
}

/// Whether control never reaches the statement after `s` (PHPStan's always-terminating statements, without
/// never-returning calls).
pub fn terminates(s: &Statement) -> bool {
    match s {
        Statement::Return(_) | Statement::Continue(_) | Statement::Break(_) => true,
        Statement::Expression(e) => matches!(
            e.expression,
            Expression::Throw(_) | Expression::Construct(Construct::Exit(_) | Construct::Die(_))
        ),
        Statement::Block(b) => b.statements.iter().last().is_some_and(terminates),
        _ => false,
    }
}

pub fn statements_terminate<'a>(mut s: impl Iterator<Item = &'a Statement<'a>>) -> bool {
    s.any(terminates)
}
/// What a function-like body saved of its parent's scope.
pub(super) struct Saved {
    guards: Vec<Guard>,
    marks: Vec<usize>,
    vars: HashMap<Vec<u8>, Vec<Guard>>,
}

impl<'a, 'arena> Checker<'a, 'arena> {
    /// `$this` is `never` here: after `if ($this instanceof Foo) { return; }` in a class that is a Foo (a trait is
    /// never an instanceof's match).
    pub(super) fn this_is_never(&self, class: &ClassLikeMetadata) -> bool {
        self.guards.iter().any(|g| match g {
            Guard::ThisNotInstanceOf(x) => {
                self.codebase.get_class_like(x.as_bytes()).is_some_and(|x| {
                    x.kind != SymbolKind::Trait
                        && self
                            .codebase
                            .is_instance_of(class.name.as_bytes(), x.name.as_bytes())
                })
            }
            _ => false,
        })
    }

    /// Guards the statements' early exits leave at their end: `if (!cond) { return; }` leaves `cond`.
    pub(super) fn exit_guards<'s>(
        &self,
        statements: impl Iterator<Item = &'s Statement<'arena>>,
    ) -> Vec<Guard>
    where
        'arena: 's,
    {
        let mut guards = Vec::new();
        for s in statements {
            if let Statement::Expression(e) = s
                && let Expression::Call(Call::Function(call)) = e.expression
            {
                guards.extend(self.define_guard(call));
            }
            if let Statement::If(i) = s
                && let IfBody::Statement(b) = &i.body
                && b.else_if_clauses.is_empty()
                && b.else_clause.is_none()
                && terminates(b.statement)
            {
                // Receiver identities and file-existence expression types hold in the constructor alone.
                guards.extend(
                    self.falsy(i.condition)
                        .into_iter()
                        .filter(|g| !matches!(g, Guard::ThisNotInstanceOf(_) | Guard::File(_)) && !expression_guard(g)),
                );
            }
        }
        guards
    }

    /// A `define('NAME', ...)` call's constant.
    pub(super) fn define_guard(&self, call: &FunctionCall<'arena>) -> Option<Guard> {
        let Expression::Identifier(id) = call.function else {
            return None;
        };
        let name = id.value().rsplit(|b| *b == b'\\').next().expect("a name");
        if !name.eq_ignore_ascii_case(b"define") {
            return None;
        }
        exists_call(call, &|id| self.resolved(id))
    }

    pub(super) fn truthy(&self, e: &Expression<'arena>) -> Vec<Guard> {
        let mut guards = truthy(
            e,
            &|v| self.vars.get(v).cloned().unwrap_or_default(),
            &|id| self.resolved(id),
            &|id| self.guard_function_name(id),
        );
        guards.extend(super::include_file::guards(self, e, true));
        guards
    }

    pub(super) fn falsy(&self, e: &Expression<'arena>) -> Vec<Guard> {
        let mut guards = falsy(
            e,
            &|v| self.vars.get(v).cloned().unwrap_or_default(),
            &|id| self.resolved(id),
            &|id| self.guard_function_name(id),
        );
        guards.extend(super::include_file::guards(self, e, false));
        guards
    }

    /// PHP-Parser leaves an unqualified function's namespace-fallback spelling intact; imported and
    /// qualified function calls instead print the resolved fully qualified name.
    fn guard_function_name(&self, id: &Identifier) -> String {
        match id {
            Identifier::Local(l) if !self.names.is_imported(&l.span) => String::from_utf8_lossy(l.value).into_owned(),
            _ => format!("\\{}", self.resolved(id)),
        }
    }

    /// `isInClassExists($name)`.
    pub(super) fn in_class_exists(&self, name: &str) -> bool {
        self.in_exists_call > 0
            || self
                .guards
                .contains(&Guard::Class(name.trim_start_matches('\\').to_string()))
    }

    /// `isInFunctionExists($name)`.
    pub(super) fn in_function_exists(&self, name: &str) -> bool {
        self.guards
            .contains(&Guard::Function(name.trim_start_matches('\\').to_string()))
    }

    /// Walks `body` with `extra` guards on top of the current ones.
    pub(super) fn with_guards(&mut self, extra: Vec<Guard>, body: impl FnOnce(&mut Self)) {
        let saved = self.guards.clone();
        let marks = self.marks.clone();
        self.guards.extend(extra);
        body(self);
        self.guards = saved;
        self.marks = marks;
    }

    /// Removing a holder must also translate the enclosing blocks' guard boundaries.
    fn retain_guards(&mut self, mut keep: impl FnMut(&Guard) -> bool) {
        let mut removed = Vec::new();
        let mut index = 0;
        self.guards.retain(|guard| {
            let retained = keep(guard);
            if !retained {
                removed.push(index);
            }
            index += 1;
            retained
        });
        for mark in &mut self.marks {
            *mark -= removed.partition_point(|index| *index < *mark);
        }
    }

    /// Braces do not end an expression holder's lifetime. Conditional walks merge their outgoing holders.
    pub(super) fn end_guard_block(&mut self) {
        let mark = self.marks.pop().expect("pushed");
        let outgoing = self.guards.split_off(mark);
        self.guards.extend(outgoing.into_iter().filter(expression_guard));
    }

    pub(super) fn has_class_constant_type(&self, a: &ClassConstantAccess, name: &str) -> bool {
        let Some(key) = constant_key(a, &|id| self.resolved(id), &|id| self.guard_function_name(id)) else { return false };
        self.guards.iter().any(|g| match g {
            Guard::ConstantFetch(k) => *k == key,
            Guard::ExprWhen(k, missing) if *k == key => {
                let ClassLikeConstantSelector::Expression(selector) = &a.constant else { return false };
                let selector = expression_key(selector.expression, &|id| self.resolved(id), &|id| self.guard_function_name(id));
                missing.iter().all(|path| path.iter().any(|(k, values)| selector.as_ref() == Some(k) && !values.iter().any(|v| v == name)))
            }
            _ => false,
        })
    }

    /// Reflection for the bounded, single named receivers this slice can invalidate without selecting a
    /// representative from a union. Unresolved/dynamic methods are invalidating, never assumed pure.
    fn guard_method(&self, on: &Expression, selector: &ClassLikeMemberSelector) -> Option<Method<'_>> {
        let ClassLikeMemberSelector::Identifier(id) = selector else { return None };
        let name = match super::types::flow::unparen(on) {
            Expression::Identifier(id) => self.resolved(id),
            Expression::Self_(_) | Expression::Static(_) => self.class_scope()?.name.to_string(),
            Expression::Parent(_) => self.class_scope()?.direct_parent_class?.to_string(),
            e => match self.phpstan_type(e)? {
                Type::Object(n, _) | Type::Static(n, _) | Type::This(n) | Type::EnumCase(n, _) => n,
                _ => return None,
            },
        };
        match super::reflection::known(self, &name)? {
            super::reflection::Known::User(_) => self.method_of(&name, id.value).ok().flatten(),
            super::reflection::Known::Builtin(k) => std::iter::once(k)
                .chain(k.ancestors.iter().filter_map(|n| self.builtins.classes.get(&n.to_ascii_lowercase())))
                .find_map(|declaring| declaring.methods.get(&String::from_utf8_lossy(id.value).to_ascii_lowercase())
                    .map(|f| Method::Builtin { declaring, f })),
        }
    }

    /// PhpMethodReflection::hasSideEffects(): void first, then resolved purity, then a fluent `$this` result.
    /// A maybe effect is remembered by PHPStan's default rememberPossiblyImpureFunctionValues=true.
    fn invalidating_method(&self, method: &Method) -> bool {
        match method {
            Method::Builtin { f, .. } => f.side_effects == "Yes",
            Method::User { declaring, f, .. } => {
                if f.name.as_bytes().eq_ignore_ascii_case(b"__construct") {
                    return true;
                }
                if method.variants().all.iter().all(|v| v.void) {
                    return true;
                }
                if let Some(pure) = super::declared::method_purity(self, declaring, f.name.as_bytes()).0 {
                    return !pure;
                }
                super::declared::with_classes(self, || {
                    let ret = super::declared::codex_return(self, f, Some(declaring))
                        .or_else(|| f.return_type_metadata.as_ref().map(|t| self.to_phpstan(&t.type_union)));
                    ret.is_some_and(|ret| is_super_type_of(&Type::This(declaring.original_name.to_string()), &ret).yes())
                })
            }
        }
    }

    pub(super) fn invalidate_called_receiver(&mut self, on: &Expression, selector: &ClassLikeMemberSelector) {
        if self.guard_method(on, selector).is_none_or(|m| self.invalidating_method(&m)) {
            if let Some(key) = expression_key(on, &|id| self.resolved(id), &|id| self.guard_function_name(id)) {
                self.invalidate_constant_fetches(&key);
            }
        }
    }

    pub(super) fn invalidate_call_references(&mut self, args: &ArgumentList, references: Vec<bool>) {
        for (arg, reference) in args.arguments.iter().zip(references) {
            if reference {
                self.invalidate_expression(arg.value());
            }
        }
    }

    pub(super) fn invalidate_static_call(&mut self, call: &StaticMethodCall) {
        if !matches!(call.class, Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) {
            return;
        }
        let invalidate = self.guard_method(call.class, &call.method).is_some_and(|m| {
            self.class_scope().is_some_and(|k| k.original_name.as_bytes().eq_ignore_ascii_case(m.declaring_name().as_bytes()))
                && self.invalidating_method(&m)
        });
        if invalidate {
            self.invalidate_constant_fetches("$this");
        }
    }

    fn invalidate_constant_fetches(&mut self, receiver: &str) {
        let class = self.class_scope().map(|k| {
            let parents: Vec<_> = k.all_parent_classes.iter().chain(k.all_parent_interfaces.iter())
                .map(|n| format!("\\{n}").to_ascii_lowercase()).collect();
            (format!("\\{}", k.original_name), parents)
        });
        self.retain_guards(|guard| {
            let key = match guard {
                Guard::ConstantFetch(key) | Guard::ExprWhen(key, _) => key,
                _ => return true,
            };
            !key_contains(key, receiver)
                && !(receiver == "$this" && ["self::", "static::", "parent::"].iter().any(|n| key_contains(key, n)))
                // ClassReflection::is() is exact for the current class name, but resolves ancestor names.
                && !(receiver == "$this" && class.as_ref().is_some_and(|(own, parents)| {
                    key_contains(key, own) || parents.iter().any(|n| key_contains(&key.to_ascii_lowercase(), n))
                }))
        });
    }

    pub(super) fn assigned_expression(&mut self, a: &Assignment) {
        self.invalidate_expression(a.lhs);
        if matches!(a.operator, AssignmentOperator::Assign(_))
            && let Some(key) = expression_key(a.lhs, &|id| self.resolved(id), &|id| self.guard_function_name(id))
            && let Some(ty) = self.phpstan_type(a.rhs)
        {
            let values = super::names::constant_strings(&ty);
            if !values.is_empty() {
                self.guards.push(Guard::ExprStrings(key, values));
            }
        }
    }

    pub(super) fn guarded_expression(&mut self, extra: Vec<Guard>, e: &Expression<'arena>) -> Vec<Guard> {
        let mut outgoing = Vec::new();
        self.with_guards(extra, |c| {
            c.walk_expression(e, &mut ());
            outgoing = c.expression_guards();
        });
        outgoing
    }

    pub(super) fn expression_guards(&self) -> Vec<Guard> {
        self.guards.iter().filter(|g| expression_guard(g)).cloned().collect()
    }

    /// ScopeOps::mergeWith: an expression holder is definite only if every surviving path has it.
    pub(super) fn join_expression_guards(&mut self, paths: &[Vec<Guard>]) {
        self.retain_guards(|guard| !expression_guard(guard) || paths.iter().all(|p| p.contains(guard)));
        if let Some((first, rest)) = paths.split_first() {
            for guard in first.iter().filter(|g| rest.iter().all(|p| p.contains(g))) {
                if !self.guards.contains(guard) {
                    self.guards.push(guard.clone());
                }
            }
        }
        // ConditionalExpressionHolder: preserve the branch constraints where a holder is absent. A later
        // filterByTruthyValue($selector === 'NAME') may make that holder definite again.
        for guard in paths.iter().flatten() {
            let key = match guard {
                Guard::Expr(key) | Guard::ConstantFetch(key) => key,
                _ => continue,
            };
            let missing: Vec<_> = paths.iter().filter(|p| !p.contains(guard)).map(|path| {
                path.iter().filter_map(|g| match g {
                    Guard::ExprStrings(key, values) => Some((key.clone(), values.clone())),
                    _ => None,
                }).collect::<Vec<_>>()
            }).collect();
            if !missing.is_empty() && missing.iter().all(|p| !p.is_empty()) {
                let guard = Guard::ExprWhen(key.clone(), missing);
                if !self.guards.contains(&guard) {
                    self.guards.push(guard);
                }
            }
        }
    }

    /// MutatingScope::invalidateExpression(): invalidate a holder containing the written receiver/selector,
    /// not another expression with an equal inferred type. Array writes invalidate the base array as well.
    pub(super) fn invalidate_expression(&mut self, e: &Expression) {
        let e = super::types::flow::unparen(e);
        match e {
            Expression::ArrayAccess(a) => return self.invalidate_expression(a.array),
            Expression::ArrayAppend(a) => return self.invalidate_expression(a.array),
            Expression::List(l) => {
                for item in l.elements.iter() {
                    self.invalidate_element(item);
                }
                return;
            }
            Expression::Array(a) => {
                for item in a.elements.iter() {
                    self.invalidate_element(item);
                }
                return;
            }
            _ => {}
        }
        let Some(key) = expression_key(e, &|id| self.resolved(id), &|id| self.guard_function_name(id)) else { return };
        let contains_key = |expr: &str| key_contains(expr, &key);
        let contains = |guard: &Guard| match guard {
            Guard::Expr(expr) | Guard::ConstantFetch(expr) | Guard::ExprStrings(expr, _) => contains_key(expr),
            Guard::ExprWhen(expr, missing) => contains_key(expr) || missing.iter().flatten().any(|(key, _)| contains_key(key)),
            _ => false,
        };
        self.retain_guards(|guard| !contains(guard));
    }

    fn invalidate_element(&mut self, item: &ArrayElement) {
        match item {
            ArrayElement::KeyValue(kv) => self.invalidate_expression(kv.value),
            ArrayElement::Value(v) => self.invalidate_expression(v.value),
            _ => {}
        }
    }

    /// A function or method body: a fresh scope (the constructor's remembered guards in a method).
    pub(super) fn enter_function(&mut self, mut guards: Vec<Guard>) {
        // the narrowed `PHP_VERSION_ID` carries into a function or class declared in the branch
        guards.extend(self.guards.iter().filter(|g| matches!(g, Guard::PhpVersion(..))).cloned());
        self.saved.push(Saved {
            guards: std::mem::replace(&mut self.guards, guards),
            marks: std::mem::take(&mut self.marks),
            vars: std::mem::take(&mut self.vars),
        });
    }

    /// `MutatingScope::getPhpVersion()`: the range a `PHP_VERSION_ID` guard leaves (bounds inclusive), `None` for
    /// the configured version when none narrows it below ConstantResolver::PHP_MIN_ANALYZABLE_VERSION_ID or bounds it.
    fn php_versions(&self) -> Option<(i64, Option<i64>)> {
        const MIN: i64 = 50207;
        match version_range(&self.guards) {
            Some((lo, hi)) if lo.is_some_and(|l| l > MIN) || hi.is_some() => Some((lo.unwrap_or(MIN).max(MIN), hi)),
            _ => None,
        }
    }

    /// `$scope->getPhpVersion()->...->yes()` for a feature of PHP `version` and later (an empty range is never, which
    /// every version range contains).
    pub(super) fn surely_at_least(&self, version: i64) -> bool {
        match self.php_versions() {
            Some((lo, hi)) => lo >= version || hi.is_some_and(|h| h < lo),
            None => i64::from(self.php_version) >= version,
        }
    }

    /// `$scope->getPhpVersion()->...->yes()` for a feature PHP `version` removed.
    pub(super) fn surely_below(&self, version: i64) -> bool {
        match self.php_versions() {
            Some((lo, hi)) => hi.is_some_and(|h| h < version || h < lo),
            None => i64::from(self.php_version) < version,
        }
    }

    /// `$scope->getPhpVersion()->...()->no()` for a feature of PHP `version` and later: false only when every
    /// version the narrowed `PHP_VERSION_ID` allows is older.
    pub(super) fn may_be_at_least(&self, version: i64) -> bool {
        match self.php_versions() {
            Some((lo, hi)) => hi.is_none_or(|h| h >= version || h < lo),
            None => i64::from(self.php_version) >= version,
        }
    }

    pub(super) fn leave_function(&mut self) {
        let s = self.saved.pop().expect("entered");
        self.guards = s.guards;
        self.marks = s.marks;
        self.vars = s.vars;
    }

    pub(super) fn walk_branch(
        &mut self,
        extra: Vec<Guard>,
        statements: &[&Statement<'arena>],
    ) -> (bool, Vec<Guard>) {
        let mut outgoing = Vec::new();
        self.with_guards(extra, |c| {
            c.marks.push(c.guards.len());
            for s in statements {
                c.walk_statement(s, &mut ());
            }
            outgoing = c.expression_guards();
            let mark = c.marks.pop().expect("pushed");
            c.guards.truncate(mark);
        });
        (statements_terminate(statements.iter().copied()), outgoing)
    }
}

fn key_contains(expr: &str, key: &str) -> bool {
    expr.match_indices(key).any(|(start, _)| {
        // `$receiverLong` and `$this->receiverLong` are not subexpressions of `$receiver`/`$this->receiver`.
        (start == 0 || expr.as_bytes().get(start - 1).is_some_and(|b| !b.is_ascii_alphanumeric() && !matches!(*b, b'_' | b'\\') && *b < 0x80))
            && (key.ends_with("::") || expr.as_bytes().get(start + key.len()).is_none_or(|b| !b.is_ascii_alphanumeric() && *b != b'_' && *b < 0x80))
    })
}
