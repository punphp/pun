//! Level 2's InvalidBinaryOperationRule (`binaryOp.invalid`, `assignOp.invalid`): an arithmetic, bitwise or concat
//! operation whose result type (InitializerExprTypeResolver's `getPlusType()` and co.) is an ErrorType.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{Cast, Level, StrictMixed, Type, cast_is_error, is_super_type_of, mixed};

use super::super::{Checker, declared};

pub const IDENTIFIERS: &[&str] = &["binaryOp.invalid", "assignOp.invalid"];

#[derive(Clone, Copy, PartialEq)]
enum Op {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Pow,
    Concat,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
    /// `|>`, a call of its right side
    Pipe,
}

fn binary_op(op: &BinaryOperator) -> Option<Op> {
    Some(match op {
        BinaryOperator::Addition(_) => Op::Plus,
        BinaryOperator::Subtraction(_) => Op::Minus,
        BinaryOperator::Multiplication(_) => Op::Mul,
        BinaryOperator::Division(_) => Op::Div,
        BinaryOperator::Modulo(_) => Op::Mod,
        BinaryOperator::Exponentiation(_) => Op::Pow,
        BinaryOperator::StringConcat(_) => Op::Concat,
        BinaryOperator::BitwiseAnd(_) => Op::BitAnd,
        BinaryOperator::BitwiseOr(_) => Op::BitOr,
        BinaryOperator::BitwiseXor(_) => Op::BitXor,
        BinaryOperator::LeftShift(_) => Op::ShiftLeft,
        BinaryOperator::RightShift(_) => Op::ShiftRight,
        _ => return None,
    })
}

fn assign_op(op: &AssignmentOperator) -> Option<Op> {
    Some(match op {
        AssignmentOperator::Addition(_) => Op::Plus,
        AssignmentOperator::Subtraction(_) => Op::Minus,
        AssignmentOperator::Multiplication(_) => Op::Mul,
        AssignmentOperator::Division(_) => Op::Div,
        AssignmentOperator::Modulo(_) => Op::Mod,
        AssignmentOperator::Exponentiation(_) => Op::Pow,
        AssignmentOperator::Concat(_) => Op::Concat,
        AssignmentOperator::BitwiseAnd(_) => Op::BitAnd,
        AssignmentOperator::BitwiseOr(_) => Op::BitOr,
        AssignmentOperator::BitwiseXor(_) => Op::BitXor,
        AssignmentOperator::LeftShift(_) => Op::ShiftLeft,
        AssignmentOperator::RightShift(_) => Op::ShiftRight,
        _ => return None,
    })
}

/// The operands the rule reads.
pub(super) fn typed(e: &Expression, spans: &mut Vec<Span>) {
    match e {
        Expression::Binary(b) if binary_op(&b.operator).is_some() => spans.extend([b.lhs.span(), b.rhs.span()]),
        Expression::Assignment(a) if assign_op(&a.operator).is_some() => spans.extend([a.lhs.span(), a.rhs.span()]),
        Expression::Pipe(p) => spans.extend([p.input.span(), p.callable.span()]),
        _ => {}
    }
}

const TRUE: Type = Type::ConstBool(true);
const FALSE: Type = Type::ConstBool(false);

/// `getConstantScalarTypes()`: the constant scalars a type is a union of (`bool` is `true|false`).
fn constant_scalars(t: &Type) -> Option<Vec<&Type>> {
    match t {
        Type::ConstInt(_) | Type::ConstFloat(_) | Type::ConstString(_) | Type::ConstBool(_) | Type::Null => Some(vec![t]),
        Type::Bool => Some(vec![&TRUE, &FALSE]),
        Type::Union(ts) => ts.iter().map(constant_scalars).try_fold(Vec::new(), |mut all, one| {
            all.extend(one?);
            Some(all)
        }),
        _ => None,
    }
}

/// The value of a constant scalar's `toNumber()` (a numeric string's, a bool's, null's).
fn number(t: &Type) -> f64 {
    match t {
        Type::ConstInt(i) => *i as f64,
        Type::ConstFloat(f) => *f,
        Type::ConstString(s) => std::str::from_utf8(s).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0.0),
        Type::ConstBool(b) => f64::from(u8::from(*b)),
        _ => 0.0,
    }
}

/// `$base->exponentiate($exponent)` is an ErrorType.
fn pow_fails(base: &Type, exponent: &Type) -> bool {
    match base {
        Type::Union(ts) | Type::Benevolent(ts) => ts.iter().any(|t| pow_fails(t, exponent)),
        Type::Intersection(ts) => ts.iter().all(|t| pow_fails(t, exponent)),
        Type::Template(t) => pow_fails(&t.bound, exponent),
        Type::Null => false,
        Type::Resource | Type::Closure(_) | Type::Callable(_) | Type::Iterable(..) => true,
        _ if base.is_array().yes() => true,
        _ if !is_super_type_of(&Type::ObjectWithoutClass, base).no() => {
            matches!(exponent, Type::Never(_)) || is_super_type_of(&Type::ObjectWithoutClass, exponent).no()
        }
        // ExponentiateHelper
        _ => match exponent {
            Type::Union(ts) => ts.iter().any(|t| pow_fails(base, t)),
            _ => !is_super_type_of(&Type::Union(vec![Type::Int, Type::Float, Type::String, Type::Bool, Type::Null]), exponent).yes(),
        },
    }
}

/// The operation's type is an ErrorType, `sm` saying which `mixed` is a StrictMixedType.
fn fails(op: Op, l: &Type, r: &Type, sm: StrictMixed) -> bool {
    if op == Op::Pow {
        return pow_fails(l, r);
    }
    // PipeHandler's FuncCall of the right side: an ErrorType when it is no callable
    if op == Op::Pipe {
        return r.is_callable().no();
    }
    let num = |t: &Type| cast_is_error(t, Cast::Number, sm);
    if op == Op::Concat {
        return cast_is_error(l, Cast::String, sm) || cast_is_error(r, Cast::String, sm);
    }
    // (a never operand, whose result is never, is no type to check)
    // ponytail: more than CALCULATE_SCALARS_LIMIT (128) pairs of constant scalars are evaluated all the same
    if let (Some(ls), Some(rs)) = (constant_scalars(l), constant_scalars(r))
    {
        let bitwise = matches!(op, Op::BitAnd | Op::BitOr | Op::BitXor);
        return ls.iter().any(|a| {
            rs.iter().any(|b| {
                if bitwise && matches!((a, b), (Type::ConstString(_), Type::ConstString(_))) {
                    return false;
                }
                let divisor = number(b);
                num(a)
                    || num(b)
                    || (op == Op::Div && divisor == 0.0)
                    || (op == Op::Mod && divisor.trunc() == 0.0)
                    || (matches!(op, Op::ShiftLeft | Op::ShiftRight) && divisor < 0.0)
            })
        });
    }
    let zero = |t: &Type| constant_scalars(t).is_some_and(|ts| ts.iter().any(|t| !num(t) && number(t) == 0.0));
    let common = |l: &Type, r: &Type| l.is_array().yes() || r.is_array().yes() || num(l) || num(r);
    match op {
        // (a `mixed` operand, which the level checks only as a StrictMixedType, is no MixedType here)
        Op::BitAnd | Op::BitOr | Op::BitXor => {
            let (ls, rs) = (l.is_string(), r.is_string());
            !(ls.yes() && rs.yes()) && ((ls.maybe() && rs.maybe()) || num(l) || num(r))
        }
        Op::Mod => num(l) || num(r) || zero(r),
        Op::Div => zero(r) || common(l, r),
        Op::Plus => {
            let (la, ra) = (l.is_array(), r.is_array());
            if la.yes() && ra.yes() {
                return false;
            }
            if (la.yes() && ra.no()) || (la.no() && ra.yes()) {
                return true;
            }
            if (la.yes() && ra.maybe()) || (la.maybe() && ra.yes()) {
                return false;
            }
            let plusable = Type::Union(vec![Type::String, Type::Float, Type::Int, Type::array(mixed(), mixed()), Type::Bool]);
            let (lp, rp) = (is_super_type_of(&plusable, l).yes(), is_super_type_of(&plusable, r).yes());
            !(la.maybe() && ra.maybe() && lp && rp) && common(l, r)
        }
        _ => common(l, r),
    }
}

/// `$scope->getType($e)` filtered as `findTypeToCheck()` with the operation's callback.
fn checked(c: &Checker, e: &Expression, op: Op, sm: StrictMixed) -> Option<Type> {
    let callback = |t: &Type| match op {
        Op::Concat => !cast_is_error(t, Cast::String, sm),
        Op::Plus => !cast_is_error(t, Cast::Number, sm) || t.is_array().yes(),
        _ => !cast_is_error(t, Cast::Number, sm),
    };
    c.find_type_to_check(e, &callback).ty
}

fn check(c: &mut Checker, op: Op, left: &Expression, right: &Expression, sigil: Span, identifier: &str) {
    if c.level < 2 {
        return;
    }
    let error = declared::with_classes(c, || {
        let sm = StrictMixed::default();
        let (Some(l), Some(r)) = (checked(c, left, op, sm), checked(c, right, op, sm)) else { return false };
        // the GMP and BcMath\Number operator extensions type the operations of those objects
        let extension = |t: &Type| {
            ["GMP", "BcMath\\Number"].iter().any(|k| is_super_type_of(&Type::Object((*k).into(), vec![]), t).yes())
        };
        !extension(&l) && !extension(&r) && fails(op, &l, &r, sm)
    });
    let (Some(l), Some(r)) = (c.phpstan_type(left), c.phpstan_type(right)) else { return };
    if !error {
        return;
    }
    let sigil = String::from_utf8_lossy(&c.file.contents[sigil.start.offset as usize..sigil.end.offset as usize]).into_owned();
    let message = format!(
        "Binary operation \"{sigil}\" between {} and {} results in an error.",
        l.describe(Level::Value),
        r.describe(Level::Value)
    );
    c.report(left, identifier, message, None);
}

pub(in super::super) fn binary(c: &mut Checker, b: &Binary) {
    if let Some(op) = binary_op(&b.operator) {
        check(c, op, b.lhs, b.rhs, b.operator.span(), "binaryOp.invalid");
    }
}

pub(in super::super) fn assignment(c: &mut Checker, a: &Assignment) {
    if let Some(op) = assign_op(&a.operator) {
        check(c, op, a.lhs, a.rhs, a.operator.span(), "assignOp.invalid");
    }
}


pub(in super::super) fn pipe(c: &mut Checker, p: &Pipe) {
    check(c, Op::Pipe, p.input, p.callable, p.operator, "binaryOp.invalid");
}
