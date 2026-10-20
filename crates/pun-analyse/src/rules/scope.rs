//! The part of PHPStan's scope the level-0 rules read: whether `class_exists('X')` / `function_exists('f')` is
//! known to be true here (`isInClassExists()` / `isInFunctionExists()`). PHPStan gets it from TypeSpecifier
//! narrowing; this follows the same constructs: `if`/`elseif`/`else` branches and what an always-terminating
//! branch leaves after the `if`, `&&`/`||`/`!`, `? :`, `match (true)`, variables holding such a check, closures
//! inheriting their parent's scope, and checks a constructor ends with (remembered in the class's other methods).

use std::collections::HashMap;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::Checker;

/// A `*_exists()` call known to be true; names as the string argument has them, without a leading `\`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Guard {
    Class(String),
    Function(String),
    /// `defined('Foo::BAR')`: the class part lowercase, as `foo::BAR`.
    ClassConstant(String),
    /// `defined('FOO')`: the global constant `FOO` (its fully qualified name).
    Constant(String),
    /// `method_exists(Foo::class, 'bar')`, lowercase `foo::bar` (`static`/`self` for those `::class`).
    Method(String),
    /// `!($this instanceof Foo)`: in a class that is a Foo, the code here is unreachable.
    ThisNotInstanceOf(String),
    /// `PHP_VERSION_ID` compared with an integer: its range here, bounds inclusive (`None` unbounded).
    PhpVersion(Option<i64>, Option<i64>),
    /// An expression with a type in the scope (`hasExpressionType()`), by its key (`$this->name`).
    Expr(String),
}

/// Guards from `expr` being true. `var` looks a variable's remembered guards up.
pub fn truthy(
    expr: &Expression,
    var: &dyn Fn(&[u8]) -> Vec<Guard>,
    resolve: &dyn Fn(&Identifier) -> String,
) -> Vec<Guard> {
    let mut guards = truthy_of(expr, var, resolve);
    if !compound(expr) {
        super::property_access::narrowed(expr, true, resolve, &mut guards);
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
) -> Vec<Guard> {
    match expr {
        Expression::Parenthesized(p) => truthy(p.expression, var, resolve),
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
            falsy(u.operand, var, resolve)
        }
        Expression::Binary(b) => match b.operator {
            BinaryOperator::And(_) | BinaryOperator::LowAnd(_) => {
                let mut g = truthy(b.lhs, var, resolve);
                g.extend(truthy(b.rhs, var, resolve));
                g
            }
            BinaryOperator::Or(_) | BinaryOperator::LowOr(_) => {
                both(truthy(b.lhs, var, resolve), truthy(b.rhs, var, resolve))
            }
            BinaryOperator::Identical(_) | BinaryOperator::Equal(_) => {
                match (bool_literal(b.lhs), bool_literal(b.rhs)) {
                    (_, Some(true)) => truthy(b.lhs, var, resolve),
                    (Some(true), _) => truthy(b.rhs, var, resolve),
                    (_, Some(false)) => falsy(b.lhs, var, resolve),
                    (Some(false), _) => falsy(b.rhs, var, resolve),
                    _ => Vec::new(),
                }
            }
            BinaryOperator::NotIdentical(_) | BinaryOperator::NotEqual(_) => {
                match (bool_literal(b.lhs), bool_literal(b.rhs)) {
                    (_, Some(false)) => truthy(b.lhs, var, resolve),
                    (Some(false), _) => truthy(b.rhs, var, resolve),
                    (_, Some(true)) => falsy(b.lhs, var, resolve),
                    (Some(true), _) => falsy(b.rhs, var, resolve),
                    _ => Vec::new(),
                }
            }
            _ => php_version(b, true).into_iter().collect(),
        },
        Expression::Variable(Variable::Direct(v)) => var(v.name),
        Expression::Assignment(a) => truthy(a.rhs, var, resolve),
        Expression::Call(Call::Function(call)) => exists_call(call, resolve).into_iter().collect(),
        _ => Vec::new(),
    }
}

/// Guards from `expr` being false.
pub fn falsy(
    expr: &Expression,
    var: &dyn Fn(&[u8]) -> Vec<Guard>,
    resolve: &dyn Fn(&Identifier) -> String,
) -> Vec<Guard> {
    let mut guards = falsy_of(expr, var, resolve);
    if !compound(expr) {
        super::property_access::narrowed(expr, false, resolve, &mut guards);
    }
    guards
}

fn falsy_of(
    expr: &Expression,
    var: &dyn Fn(&[u8]) -> Vec<Guard>,
    resolve: &dyn Fn(&Identifier) -> String,
) -> Vec<Guard> {
    match expr {
        Expression::Parenthesized(p) => falsy(p.expression, var, resolve),
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
            truthy(u.operand, var, resolve)
        }
        Expression::Binary(b) => match b.operator {
            BinaryOperator::Or(_) | BinaryOperator::LowOr(_) => {
                let mut g = falsy(b.lhs, var, resolve);
                g.extend(falsy(b.rhs, var, resolve));
                g
            }
            BinaryOperator::And(_) | BinaryOperator::LowAnd(_) => {
                both(falsy(b.lhs, var, resolve), falsy(b.rhs, var, resolve))
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
                    (_, Some(v)) if v != flipped => truthy(b.lhs, var, resolve),
                    (Some(v), _) if v != flipped => truthy(b.rhs, var, resolve),
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
        let class = match arg {
            Expression::Access(Access::ClassConstant(ClassConstantAccess {
                class: Expression::Static(_) | Expression::Self_(_),
                constant: ClassLikeConstantSelector::Identifier(c),
                ..
            })) if c.value.eq_ignore_ascii_case(b"class") => {
                if matches!(arg, Expression::Access(Access::ClassConstant(a)) if matches!(a.class, Expression::Static(_)))
                {
                    "static".to_string()
                } else {
                    "self".to_string()
                }
            }
            // a variable's object gets the method (`$this::name`, `$foo::name`)
            Expression::Variable(Variable::Direct(v)) => String::from_utf8_lossy(v.name).into_owned(),
            _ => constant_string(arg, resolve)?,
        };
        let method = call.argument_list.arguments.get(1)?.value();
        let method = constant_string(method, resolve)?;
        return Some(Guard::Method(
            format!("{}::{}", class.trim_start_matches('\\'), method).to_ascii_lowercase(),
        ));
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
                // a `$this` check holds in the constructor alone
                guards.extend(
                    self.falsy(i.condition)
                        .into_iter()
                        .filter(|g| !matches!(g, Guard::ThisNotInstanceOf(_))),
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
        truthy(
            e,
            &|v| self.vars.get(v).cloned().unwrap_or_default(),
            &|id| self.resolved(id),
        )
    }

    pub(super) fn falsy(&self, e: &Expression<'arena>) -> Vec<Guard> {
        falsy(
            e,
            &|v| self.vars.get(v).cloned().unwrap_or_default(),
            &|id| self.resolved(id),
        )
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
        let mark = self.guards.len();
        self.guards.extend(extra);
        body(self);
        self.guards.truncate(mark);
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
    ) -> bool {
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
