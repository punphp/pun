//! Partial, passive PHPStan-native state. These transfers never evaluate an expression or choose a path.

use std::cmp::Ordering;
use std::rc::Rc;

use mago_codex::metadata::function_like::{FunctionLikeKind, FunctionLikeMetadata};
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::string::TStringCasing;
use mago_codex::ttype::atomic::scalar::{TScalar, float::TFloat, int::TInteger, string::TString};
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    get_bool, get_false, get_float, get_int, get_int_or_float, get_int_range, get_literal_float,
    get_literal_int, get_literal_string, get_mixed, get_string_with_props, get_true,
};
use mago_syntax::cst::{
    AssignmentOperator, Binary, BinaryOperator, Call, Expression, Literal, Statement, UnaryPrefix,
    UnaryPrefixOperator, Variable,
};
use mago_word::WordMap;
use mago_word::word;

use crate::artifacts::AnalysisArtifacts;
use crate::context::block::BlockContext;

pub(crate) fn scalar_type(ty: &TUnion) -> bool {
    ty.types.iter().all(|atomic| {
        matches!(
            atomic,
            TAtomic::Scalar(_) | TAtomic::Null | TAtomic::Mixed(_)
        )
    })
}

fn supported_function(function: &FunctionLikeMetadata) -> bool {
    matches!(function.kind, FunctionLikeKind::Function)
        && function.flags.is_user_defined()
        && !function.flags.is_by_reference()
        && !function.flags.has_yield()
        && function.assertions.is_empty()
        && function.if_true_assertions.is_empty()
        && function.if_false_assertions.is_empty()
        && function.globals_accessed.is_empty()
        && function.parameters.iter().all(|parameter| {
            !parameter.flags.is_by_reference()
                && !parameter.flags.is_variadic()
                && parameter.out_type.is_none()
                && parameter
                    .type_declaration_metadata
                    .as_ref()
                    .is_none_or(|ty| scalar_type(&ty.type_union))
        })
}

pub(crate) fn parameter_types(function: &FunctionLikeMetadata) -> Option<WordMap<Rc<TUnion>>> {
    supported_function(function).then(|| {
        function
            .parameters
            .iter()
            .map(|parameter| {
                let ty = parameter
                    .type_declaration_metadata
                    .as_ref()
                    .map_or_else(get_mixed, |ty| ty.type_union.clone());
                (parameter.name.0, Rc::new(ty))
            })
            .collect()
    })
}

pub(crate) fn function_return_type(function: &FunctionLikeMetadata) -> Option<TUnion> {
    if !supported_function(function) {
        return None;
    }
    let ty = function
        .return_type_declaration_metadata
        .as_ref()
        .map_or_else(get_mixed, |ty| ty.type_union.clone());
    scalar_type(&ty).then_some(ty)
}

pub(crate) fn before_statement(statement: &Statement<'_>, block: &mut BlockContext<'_>) {
    if block.native_locals.is_none() {
        return;
    }
    if !matches!(
        statement,
        Statement::OpeningTag(_)
            | Statement::ClosingTag(_)
            | Statement::Noop(_)
            | Statement::Use(_)
            | Statement::Namespace(_)
            | Statement::Function(_)
            | Statement::Block(_)
            | Statement::Expression(_)
            | Statement::Return(_)
    ) {
        // Branch, loop, reference and exceptional transfers are not implemented. Invalidation
        // happens BEFORE their children, so neither inner nor subsequent queries use stale state.
        block.native_locals = None;
    }
}

pub(crate) fn before_expression(expression: &Expression<'_>, block: &mut BlockContext<'_>) {
    if block.native_locals.is_none() {
        return;
    }
    let supported = match expression {
        Expression::Literal(Literal::String(value)) => value.value.is_some(),
        Expression::Literal(Literal::Integer(value)) => value.value.is_some(),
        Expression::Literal(_)
        | Expression::Parenthesized(_)
        | Expression::Variable(Variable::Direct(_)) => true,
        Expression::UnaryPrefix(unary) => matches!(
            unary.operator,
            UnaryPrefixOperator::Plus(_)
                | UnaryPrefixOperator::Negation(_)
                | UnaryPrefixOperator::Not(_)
        ),
        Expression::Binary(binary) => {
            binary.operator.is_additive()
                || binary.operator.is_multiplicative()
                || binary.operator.is_comparison()
                || binary.operator.is_concatenation()
                || matches!(binary.operator, BinaryOperator::LowXor(_))
        }
        Expression::Assignment(assignment) => {
            matches!(assignment.operator, AssignmentOperator::Assign(_))
                && matches!(assignment.lhs, Expression::Variable(Variable::Direct(variable))
                    if variable.name != b"$this" && variable.name != b"$GLOBALS" && !variable.name.starts_with(b"$_"))
        }
        // The resolved target is checked before arguments in function_call.rs.
        Expression::Call(Call::Function(call)) => {
            matches!(call.function, Expression::Identifier(_))
        }
        _ => false,
    };
    if !supported {
        block.native_locals = None;
    }
}

pub(crate) fn after_expression(
    expression: &Expression<'_>,
    block: &mut BlockContext<'_>,
    artifacts: &mut AnalysisArtifacts,
) {
    let Some(locals) = &block.native_locals else {
        return;
    };
    let ty = match expression {
        // Literals have no PHPDoc or contextual type input. Share the value computed on this walk.
        Expression::Literal(_) => artifacts.get_rc_expression_type(expression).cloned(),
        Expression::Variable(Variable::Direct(variable)) => {
            locals.get(&word(variable.name)).cloned()
        }
        Expression::Parenthesized(parenthesized) => artifacts
            .get_rc_native_expression_type(parenthesized.expression)
            .cloned(),
        // An assignment expression yields its RHS, even when @var changes the target's type.
        Expression::Assignment(assignment) => artifacts
            .get_rc_native_expression_type(assignment.rhs)
            .cloned(),
        Expression::UnaryPrefix(unary) => unary_type(unary, artifacts).map(Rc::new),
        Expression::Binary(binary) => binary_type(binary, artifacts).map(Rc::new),
        _ => artifacts.get_rc_native_expression_type(expression).cloned(),
    };
    if let Some(ty) = ty {
        artifacts.set_rc_native_expression_type(expression, ty);
    } else {
        block.native_locals = None;
    }
}

// Bounded scalar transfers from PHPStan's InitializerExprTypeResolver. Unlike Mago's expression
// analyzers these helpers only consume native artifacts: no diagnostics, hooks or operand walks.
#[derive(Clone, Copy)]
enum Number {
    Int(Option<i64>),
    Float(Option<f64>),
}

impl Number {
    fn from_type(ty: &TUnion) -> Option<Self> {
        if !ty.is_single() {
            return None;
        }
        match ty.get_single() {
            TAtomic::Scalar(TScalar::Integer(TInteger::Literal(value))) => {
                Some(Self::Int(Some(*value)))
            }
            TAtomic::Scalar(TScalar::Integer(TInteger::Unspecified)) => Some(Self::Int(None)),
            TAtomic::Scalar(TScalar::Float(TFloat::Literal(value))) if value.is_finite() => {
                Some(Self::Float(Some(**value)))
            }
            TAtomic::Scalar(TScalar::Float(TFloat::Float)) => Some(Self::Float(None)),
            _ => None,
        }
    }

    fn is_float(self) -> bool {
        matches!(self, Self::Float(_))
    }

    fn value(self) -> Option<f64> {
        match self {
            Self::Int(value) => value.map(|value| value as f64),
            Self::Float(value) => value,
        }
    }
}

fn float_literal(value: f64) -> Option<TUnion> {
    value.is_finite().then(|| get_literal_float(value))
}

fn truth(ty: &TUnion) -> Option<bool> {
    if ty.is_always_truthy() {
        Some(true)
    } else if ty.is_always_falsy() {
        Some(false)
    } else {
        None
    }
}

fn boolean(value: Option<bool>) -> TUnion {
    match value {
        Some(true) => get_true(),
        Some(false) => get_false(),
        None => get_bool(),
    }
}

fn unary_type(unary: &UnaryPrefix<'_>, artifacts: &AnalysisArtifacts) -> Option<TUnion> {
    let ty = artifacts.get_native_expression_type(unary.operand)?;
    if unary.operator.is_not() {
        return Some(boolean(truth(ty).map(|value| !value)));
    }
    let number = Number::from_type(ty)?;
    if matches!(unary.operator, UnaryPrefixOperator::Plus(_)) {
        return Some(ty.clone());
    }
    match number {
        Number::Int(Some(value)) => value
            .checked_neg()
            .map(get_literal_int)
            .or_else(|| float_literal(-(value as f64))),
        Number::Float(Some(value)) => float_literal(-value),
        Number::Int(None) => Some(get_int()),
        Number::Float(None) => Some(get_float()),
    }
}

fn binary_type(binary: &Binary<'_>, artifacts: &AnalysisArtifacts) -> Option<TUnion> {
    let operator = &binary.operator;
    if operator.is_concatenation() {
        return concat_type(binary, artifacts);
    }
    let left = artifacts.get_native_expression_type(binary.lhs)?;
    let right = artifacts.get_native_expression_type(binary.rhs)?;
    if matches!(operator, BinaryOperator::LowXor(_)) {
        return Some(boolean(
            truth(left)
                .zip(truth(right))
                .map(|(left, right)| left ^ right),
        ));
    }
    if operator.is_comparison() {
        return comparison_type(binary, left, right);
    }
    arithmetic_type(
        operator,
        Number::from_type(left)?,
        Number::from_type(right)?,
    )
}

fn arithmetic_type(operator: &BinaryOperator<'_>, left: Number, right: Number) -> Option<TUnion> {
    if matches!(operator, BinaryOperator::Modulo(_)) {
        // Float/string coercion and range/union arithmetic are not part of this slice.
        let (Number::Int(left), Number::Int(Some(right))) = (left, right) else {
            return None;
        };
        if right == 0 {
            return None;
        }
        return Some(if let Some(left) = left {
            get_literal_int(left.checked_rem(right).unwrap_or(0))
        } else {
            let bound = (right.unsigned_abs() - 1) as i64;
            if bound == 0 {
                get_literal_int(0)
            } else {
                get_int_range(Some(-bound), Some(bound))
            }
        });
    }
    if matches!(operator, BinaryOperator::Division(_)) && right.value() == Some(0.0) {
        return None;
    }
    if let (Number::Int(Some(left)), Number::Int(Some(right))) = (left, right) {
        let integer = match operator {
            BinaryOperator::Addition(_) => left.checked_add(right),
            BinaryOperator::Subtraction(_) => left.checked_sub(right),
            BinaryOperator::Multiplication(_) => left.checked_mul(right),
            _ => (left.checked_rem(right) == Some(0))
                .then(|| left.checked_div(right))
                .flatten(),
        };
        if let Some(integer) = integer {
            return Some(get_literal_int(integer));
        }
    }
    if let (Some(left), Some(right)) = (left.value(), right.value()) {
        return float_literal(match operator {
            BinaryOperator::Addition(_) => left + right,
            BinaryOperator::Subtraction(_) => left - right,
            BinaryOperator::Multiplication(_) => left * right,
            _ => left / right,
        });
    }
    let float = left.is_float() || right.is_float();
    // PHPStan folds an integer zero multiplier even with a general float, but not float zero.
    if matches!(operator, BinaryOperator::Multiplication(_))
        && (matches!(left, Number::Int(Some(0))) || matches!(right, Number::Int(Some(0))))
    {
        return Some(if float {
            get_literal_float(0.0)
        } else {
            get_literal_int(0)
        });
    }
    Some(if float {
        get_float()
    } else if matches!(operator, BinaryOperator::Division(_))
        && !matches!(right, Number::Int(Some(1 | -1)))
        && !matches!(left, Number::Int(Some(0)))
    {
        get_int_or_float()
    } else {
        get_int()
    })
}

// Category plus constant-ness, restricted to scalar shapes whose identity transfer is implemented.
fn identity_shape(ty: &TUnion) -> Option<(u8, bool)> {
    if let Some(number) = Number::from_type(ty) {
        return Some((u8::from(number.is_float()), number.value().is_some()));
    }
    if !ty.is_single() {
        return None;
    }
    match ty.get_single() {
        TAtomic::Scalar(TScalar::String(string))
            if string.get_known_literal_value().is_some() || *string == TString::general() =>
        {
            Some((2, string.get_known_literal_value().is_some()))
        }
        TAtomic::Scalar(TScalar::Bool(value)) => Some((3, value.value.is_some())),
        TAtomic::Null => Some((4, true)),
        _ => None,
    }
}

fn direct_local<'a>(mut expression: &'a Expression<'_>) -> Option<&'a [u8]> {
    while let Expression::Parenthesized(parenthesized) = expression {
        expression = parenthesized.expression;
    }
    if let Expression::Variable(Variable::Direct(variable)) = expression {
        Some(variable.name)
    } else {
        None
    }
}

fn comparison_type(binary: &Binary<'_>, left: &TUnion, right: &TUnion) -> Option<TUnion> {
    let operator = &binary.operator;
    let same_local =
        direct_local(binary.lhs).is_some_and(|name| direct_local(binary.rhs) == Some(name));
    if operator.is_identity() {
        let (left_kind, left_constant) = identity_shape(left)?;
        let (right_kind, right_constant) = identity_shape(right)?;
        let identical = if left_kind != right_kind {
            Some(false)
        } else if left_constant && right_constant {
            Some(left.get_single() == right.get_single())
        } else {
            same_local.then_some(true)
        };
        return Some(boolean(
            identical.map(|value| value ^ operator.is_negated_equality()),
        ));
    }
    let left = Number::from_type(left)?;
    let right = Number::from_type(right)?;
    let ordering = if let (Number::Int(Some(left)), Number::Int(Some(right))) = (left, right) {
        Some(left.cmp(&right))
    } else {
        left.value()
            .zip(right.value())
            .and_then(|(left, right)| left.partial_cmp(&right))
    };
    if matches!(operator, BinaryOperator::Spaceship(_)) {
        return Some(ordering.map_or_else(
            || get_int_range(Some(-1), Some(1)),
            |order| get_literal_int(order as i64),
        ));
    }
    let equal = matches!(
        operator,
        BinaryOperator::Equal(_) | BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_)
    );
    let value = if equal && same_local {
        Some(!operator.is_negated_equality())
    } else {
        ordering.map(|order| match operator {
            BinaryOperator::Equal(_) => order == Ordering::Equal,
            BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => {
                order != Ordering::Equal
            }
            BinaryOperator::LessThan(_) => order == Ordering::Less,
            BinaryOperator::LessThanOrEqual(_) => order != Ordering::Greater,
            BinaryOperator::GreaterThan(_) => order == Ordering::Greater,
            _ => order != Ordering::Less,
        })
    };
    Some(boolean(value))
}

fn concat_type(binary: &Binary<'_>, artifacts: &AnalysisArtifacts) -> Option<TUnion> {
    // Mago flattens concat trees and never visits their intermediate binary nodes. Fold the SAME
    // visited leaves without evaluating them again or publishing synthetic observer events.
    let mut stack = vec![binary.rhs, binary.lhs];
    let mut result = None;
    while let Some(expression) = stack.pop() {
        if let Expression::Binary(inner) = expression
            && inner.operator.is_concatenation()
        {
            stack.push(inner.rhs);
            stack.push(inner.lhs);
            continue;
        }
        let operand = artifacts.get_native_expression_type(expression)?;
        result = Some(match result {
            None => operand.clone(),
            Some(previous) => concat_strings(&previous, operand)?,
        });
    }
    result
}

fn concat_strings(left: &TUnion, right: &TUnion) -> Option<TUnion> {
    let left = left.get_single_string()?;
    let right = right.get_single_string()?;
    match (
        left.get_known_literal_value(),
        right.get_known_literal_value(),
    ) {
        (Some(left), Some(right)) => Some(get_literal_string(word([left, right].concat()))),
        (Some(b""), _) => Some(TUnion::from_atomic(TAtomic::Scalar(TScalar::String(
            *right,
        )))),
        (_, Some(b"")) => Some(TUnion::from_atomic(TAtomic::Scalar(TScalar::String(*left)))),
        _ => Some(get_string_with_props(
            false,
            (left.is_non_empty && right.is_non_empty) || left.is_truthy || right.is_truthy,
            left.is_non_empty || right.is_non_empty,
            false,
            TStringCasing::Unspecified,
        )),
    }
}
