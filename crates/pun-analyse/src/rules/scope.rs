//! The part of PHPStan's scope the level-0 rules read: whether `class_exists('X')` / `function_exists('f')` is
//! known to be true here (`isInClassExists()` / `isInFunctionExists()`). PHPStan gets it from TypeSpecifier
//! narrowing; this follows the same constructs: `if`/`elseif`/`else` branches and what an always-terminating
//! branch leaves after the `if`, `&&`/`||`/`!`, `? :`, `match (true)`, variables holding such a check, closures
//! inheriting their parent's scope, and checks a constructor ends with (remembered in the class's other methods).

use mago_syntax::cst::*;

/// A `*_exists()` call known to be true; names as the string argument has them, without a leading `\`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Guard {
    Class(String),
    Function(String),
    /// `defined('Foo::BAR')`: the class part lowercase, as `foo::BAR`.
    ClassConstant(String),
}

/// Guards from `expr` being true. `var` looks a variable's remembered guards up.
pub fn truthy(
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
            _ => Vec::new(),
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
            BinaryOperator::Identical(_)
            | BinaryOperator::Equal(_)
            | BinaryOperator::NotIdentical(_)
            | BinaryOperator::NotEqual(_) => {
                let flipped = match b.operator {
                    BinaryOperator::Identical(_) | BinaryOperator::Equal(_) => true,
                    _ => false,
                };
                match (bool_literal(b.lhs), bool_literal(b.rhs)) {
                    (_, Some(v)) if v != flipped => truthy(b.lhs, var, resolve),
                    (Some(v), _) if v != flipped => truthy(b.rhs, var, resolve),
                    _ => Vec::new(),
                }
            }
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

fn both(a: Vec<Guard>, b: Vec<Guard>) -> Vec<Guard> {
    a.into_iter().filter(|g| b.contains(g)).collect()
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
    let arg = match call.argument_list.arguments.first()? {
        Argument::Positional(a) => a.value,
        Argument::Named(a) => a.value,
    };
    let value = constant_string(arg, resolve)?;
    let value = value.trim_start_matches('\\').to_string();
    match name.trim_start_matches('\\') {
        "class_exists" | "interface_exists" | "trait_exists" | "enum_exists" => {
            Some(Guard::Class(value))
        }
        "function_exists" => Some(Guard::Function(value)),
        "defined" => {
            let (class, constant) = value.split_once("::")?;
            Some(Guard::ClassConstant(format!(
                "{}::{constant}",
                class.to_ascii_lowercase()
            )))
        }
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
