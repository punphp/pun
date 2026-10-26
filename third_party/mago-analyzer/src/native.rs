//! Partial, passive PHPStan-native state. These transfers never evaluate an expression or choose a path.

use std::cmp::Ordering;
use std::rc::Rc;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::function_like::{FunctionLikeKind, FunctionLikeMetadata};
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::string::TStringCasing;
use mago_codex::ttype::atomic::scalar::{TScalar, float::TFloat, int::TInteger, string::TString};
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    get_bool, get_false, get_float, get_int, get_int_or_float, get_int_range, get_literal_float,
    get_literal_int, get_literal_string, get_mixed, get_never, get_string, get_string_with_props,
    get_true, wrap_atomic,
};
use mago_syntax::cst::{
    AssignmentOperator, Binary, BinaryOperator, Call, Expression, Literal, Statement, UnaryPrefix,
    UnaryPrefixOperator, Variable,
};
use mago_word::WordMap;
use mago_word::word;

use crate::artifacts::AnalysisArtifacts;
use crate::context::block::BlockContext;
use crate::utils::php_emulation::string_to_int;

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
    if let Statement::If(statement) = statement {
        if !condition_shape(statement.condition, true) {
            block.native_locals = None;
        }
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
        // Loop, reference and exceptional transfers are not implemented. Invalidation
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
        Expression::UnaryPrefix(unary) => {
            scalar_cast(&unary.operator).is_some()
                || matches!(
                    unary.operator,
                    UnaryPrefixOperator::Plus(_)
                        | UnaryPrefixOperator::Negation(_)
                        | UnaryPrefixOperator::Not(_)
                )
        }
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
        Expression::Conditional(conditional) => condition_shape(conditional.condition, true),
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
    // A ternary value can be unavailable while its full-selected continuation is supported.
    // The conditional analyzer owns both transfers independently.
    if matches!(expression, Expression::Conditional(_)) {
        return;
    }
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

pub(crate) type Locals = WordMap<Rc<TUnion>>;

// Do not enter conditional::analyze's re-evaluation path with an effectful wrapped condition.
// Short circuit, calls, and general expression constraints remain outside this bounded transfer.
pub(crate) fn condition_shape(expression: &Expression<'_>, assignment: bool) -> bool {
    match expression {
        Expression::Literal(_) | Expression::Variable(Variable::Direct(_)) => true,
        Expression::Parenthesized(value) => condition_shape(value.expression, false),
        Expression::UnaryPrefix(value) if value.operator.is_not() => {
            condition_shape(value.operand, false)
        }
        Expression::Assignment(value) => {
            assignment && value.operator.is_assign() && direct_local(value.lhs).is_some()
        }
        Expression::Binary(value) => {
            (value.operator.is_identity() || ordered(&value.operator))
                && ((direct_local(value.lhs).is_some() && literal_shape(value.rhs))
                    || (direct_local(value.rhs).is_some() && literal_shape(value.lhs)))
        }
        _ => false,
    }
}

fn literal_shape(expression: &Expression<'_>) -> bool {
    match expression {
        Expression::Literal(_) => true,
        Expression::UnaryPrefix(value) => {
            matches!(value.operand, Expression::Literal(_))
                && matches!(value.operator, UnaryPrefixOperator::Negation(_))
        }
        _ => false,
    }
}

fn ordered(operator: &BinaryOperator<'_>) -> bool {
    matches!(
        operator,
        BinaryOperator::LessThan(_)
            | BinaryOperator::LessThanOrEqual(_)
            | BinaryOperator::GreaterThan(_)
            | BinaryOperator::GreaterThanOrEqual(_)
    )
}

/// PHPStan applies the ordinary condition's specifications to both holder maps. In particular,
/// full-type decided scalar comparisons can impose never on a native-reachable branch.
/// Callers validate condition_shape before evaluation; an unsupported condition has no live map.
pub(crate) fn branch(
    condition: &Expression<'_>,
    locals: &Option<Locals>,
    artifacts: &AnalysisArtifacts,
    positive: bool,
    codebase: &CodebaseMetadata,
) -> Option<Locals> {
    let mut locals = locals.clone()?;
    match condition {
        Expression::Parenthesized(value) => {
            return branch(
                value.expression,
                &Some(locals),
                artifacts,
                positive,
                codebase,
            );
        }
        Expression::UnaryPrefix(value) if value.operator.is_not() => {
            return branch(value.operand, &Some(locals), artifacts, !positive, codebase);
        }
        Expression::Variable(Variable::Direct(variable)) => {
            let name = word(variable.name);
            let narrowed = truthy_type(locals.get(&name)?, positive, codebase)?;
            locals.insert(name, Rc::new(narrowed));
        }
        Expression::Assignment(value) => {
            let name = word(direct_local(value.lhs)?);
            let narrowed = truthy_type(locals.get(&name)?, positive, codebase)?;
            locals.insert(name, Rc::new(narrowed));
        }
        Expression::Binary(value) => {
            let (name, literal, reversed) = if let Some(name) = direct_local(value.lhs) {
                (word(name), value.rhs, false)
            } else {
                (word(direct_local(value.rhs)?), value.lhs, true)
            };
            // The shape gate allows only scalar literal operands here.
            let literal = artifacts.get_native_expression_type(literal)?;
            let current = locals.get(&name)?;
            let narrowed = if value.operator.is_identity() {
                let equal = positive ^ value.operator.is_negated_equality();
                let decided_scalar = matches!(
                    literal.get_single(),
                    TAtomic::Scalar(TScalar::Integer(_) | TScalar::String(_))
                ) && !equal;
                let decided = artifacts.get_expression_type(value).and_then(truth);
                if let Some(known) = decided.filter(|_| decided_scalar) {
                    if known != positive {
                        get_never()
                    } else {
                        (**current).clone()
                    }
                } else {
                    literal_type(current, literal.get_single(), equal, codebase)?
                }
            } else {
                let TAtomic::Scalar(TScalar::Integer(TInteger::Literal(limit))) =
                    literal.get_single()
                else {
                    return None;
                };
                let greater = matches!(
                    value.operator,
                    BinaryOperator::GreaterThan(_) | BinaryOperator::GreaterThanOrEqual(_)
                ) ^ reversed;
                let inclusive = matches!(
                    value.operator,
                    BinaryOperator::LessThanOrEqual(_) | BinaryOperator::GreaterThanOrEqual(_)
                );
                let above = greater == positive;
                let include_limit = inclusive == positive;
                let boundary = if include_limit {
                    Some(*limit)
                } else if above {
                    limit.checked_add(1)
                } else {
                    limit.checked_sub(1)
                };
                match boundary {
                    Some(boundary) => range_type(
                        current,
                        if above { boundary } else { i64::MIN },
                        if above { i64::MAX } else { boundary },
                        codebase,
                    )?,
                    None => get_never(),
                }
            };
            locals.insert(name, Rc::new(narrowed));
        }
        _ => {} // Literal conditions do not constrain a local.
    }
    Some(locals)
}

/// Only continuing, full-selected edges contribute. An unavailable contributing edge poisons the
/// result; an absent variable on an edge does not replace its when-defined type with mixed.
pub(crate) fn join(branches: &[Option<Locals>], codebase: &CodebaseMetadata) -> Option<Locals> {
    let mut merged = branches.first()?.clone()?;
    for branch in &branches[1..] {
        for (name, ty) in branch.as_ref()? {
            let value = merged
                .get(name)
                .map_or_else(|| ty.clone(), |old| Rc::new(union(old, ty, codebase)));
            merged.insert(*name, value);
        }
    }
    Some(merged)
}

fn union(left: &TUnion, right: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
    let mut result = combine_union_types(left, right, codebase, CombinerOptions::default());
    // Mago retains falsy literals beside a truthy string. PHPStan collapses the complete
    // truthy/falsy partition back to string; keep this normalization out of the full lane.
    let has_literal = |want: &[u8]| {
        result.types.iter().any(|atomic| {
            matches!(atomic,
        TAtomic::Scalar(TScalar::String(value)) if value.get_known_literal_value() == Some(want))
        })
    };
    let has_empty = has_literal(b"");
    let has_zero = has_literal(b"0");
    if (has_empty || has_zero)
        && result.types.iter().any(|atomic| {
            let TAtomic::Scalar(TScalar::String(value)) = atomic else {
                return false;
            };
            let mut general = value.clone();
            general.is_truthy = false;
            general.is_non_empty = false;
            general == TString::general() && value.is_non_empty && (!value.is_truthy || has_zero)
        })
    {
        result
            .types
            .to_mut()
            .retain(|atomic| !matches!(atomic, TAtomic::Scalar(TScalar::String(_))));
        let mut string = TString::general();
        string.is_non_empty = !has_empty;
        result
            .types
            .to_mut()
            .push(TAtomic::Scalar(TScalar::String(string)));
    }
    result
}

fn interval(min: i64, max: i64) -> TUnion {
    if min > max {
        get_never()
    } else if min == max {
        get_literal_int(min)
    } else {
        get_int_range(
            (min != i64::MIN).then_some(min),
            (max != i64::MAX).then_some(max),
        )
    }
}

fn range_type(ty: &TUnion, min: i64, max: i64, codebase: &CodebaseMetadata) -> Option<TUnion> {
    let mut result = get_never();
    for atomic in ty.types.iter() {
        let part = match atomic {
            TAtomic::Scalar(TScalar::Integer(value)) => interval(
                min.max(value.get_minimum_value().unwrap_or(i64::MIN)),
                max.min(value.get_maximum_value().unwrap_or(i64::MAX)),
            ),
            _ => return None,
        };
        result = union(&result, &part, codebase);
    }
    Some(result)
}

fn integer_literal(
    value: &TInteger,
    want: i64,
    equal: bool,
    codebase: &CodebaseMetadata,
) -> TUnion {
    let min = value.get_minimum_value().unwrap_or(i64::MIN);
    let max = value.get_maximum_value().unwrap_or(i64::MAX);
    if equal {
        interval(min.max(want), max.min(want))
    } else if want < min || want > max {
        wrap_atomic(TAtomic::Scalar(TScalar::Integer(*value)))
    } else {
        let left = want
            .checked_sub(1)
            .map_or_else(get_never, |end| interval(min, end));
        let right = want
            .checked_add(1)
            .map_or_else(get_never, |start| interval(start, max));
        union(&left, &right, codebase)
    }
}

fn literal_type(
    ty: &TUnion,
    literal: &TAtomic,
    equal: bool,
    codebase: &CodebaseMetadata,
) -> Option<TUnion> {
    let mut result = get_never();
    for atomic in ty.types.iter() {
        let part = match (atomic, literal) {
            (
                TAtomic::Scalar(TScalar::Integer(value)),
                TAtomic::Scalar(TScalar::Integer(TInteger::Literal(want))),
            ) => integer_literal(value, *want, equal, codebase),
            (TAtomic::Scalar(TScalar::Bool(value)), TAtomic::Scalar(TScalar::Bool(want))) => {
                let want = want.value?;
                match value.value {
                    Some(value) if (value == want) != equal => get_never(),
                    Some(_) => wrap_atomic(atomic.clone()),
                    None => boolean(Some(want == equal)),
                }
            }
            (TAtomic::Scalar(TScalar::String(value)), TAtomic::Scalar(TScalar::String(want))) => {
                let want_literal = want.get_known_literal_value()?;
                if let Some(value) = value.get_known_literal_value() {
                    if (value == want_literal) == equal {
                        wrap_atomic(atomic.clone())
                    } else {
                        get_never()
                    }
                } else {
                    // comparison_type admitted only a general or known-literal string.
                    if equal {
                        wrap_atomic(literal.clone())
                    } else {
                        wrap_atomic(atomic.clone())
                    }
                }
            }
            (TAtomic::Null, TAtomic::Null) => {
                if equal {
                    wrap_atomic(TAtomic::Null)
                } else {
                    get_never()
                }
            }
            (
                TAtomic::Null
                | TAtomic::Scalar(TScalar::Bool(_) | TScalar::Integer(_) | TScalar::String(_)),
                TAtomic::Null
                | TAtomic::Scalar(TScalar::Bool(_) | TScalar::Integer(_) | TScalar::String(_)),
            ) => {
                if equal {
                    get_never()
                } else {
                    wrap_atomic(atomic.clone())
                }
            }
            _ => return None,
        };
        result = union(&result, &part, codebase);
    }
    Some(result)
}

fn truthy_type(ty: &TUnion, positive: bool, codebase: &CodebaseMetadata) -> Option<TUnion> {
    let mut result = get_never();
    for atomic in ty.types.iter() {
        let part = match atomic {
            TAtomic::Never => get_never(),
            TAtomic::Null => {
                if positive {
                    get_never()
                } else {
                    wrap_atomic(atomic.clone())
                }
            }
            TAtomic::Scalar(TScalar::Bool(value)) => match value.value {
                Some(value) if value != positive => get_never(),
                _ => boolean(Some(positive)),
            },
            TAtomic::Scalar(TScalar::Integer(value)) => {
                integer_literal(value, 0, !positive, codebase)
            }
            TAtomic::Scalar(TScalar::String(value)) => {
                if let Some(literal) = value.get_known_literal_value() {
                    if (literal != b"" && literal != b"0") == positive {
                        wrap_atomic(atomic.clone())
                    } else {
                        get_never()
                    }
                } else {
                    let mut general = value.clone();
                    general.is_truthy = false;
                    general.is_non_empty = false;
                    if general != TString::general() {
                        return None;
                    }
                    if positive {
                        let mut value = value.clone();
                        value.is_truthy = true;
                        value.is_non_empty = true;
                        wrap_atomic(TAtomic::Scalar(TScalar::String(value)))
                    } else {
                        let mut values = get_never();
                        if !value.is_non_empty {
                            values = get_literal_string(word(""));
                        }
                        if !value.is_truthy {
                            values = union(&values, &get_literal_string(word("0")), codebase);
                        }
                        values
                    }
                }
            }
            _ => return None,
        };
        result = union(&result, &part, codebase);
    }
    Some(result)
}

pub(crate) fn conditional_value(
    condition: &Expression<'_>,
    then: Option<&Expression<'_>>,
    otherwise: &Expression<'_>,
    artifacts: &AnalysisArtifacts,
    codebase: &CodebaseMetadata,
) -> Option<TUnion> {
    let condition_type = artifacts.get_native_expression_type(condition)?;
    let positive = || match then {
        Some(then) => artifacts.get_native_expression_type(then).cloned(),
        None => truthy_type(condition_type, true, codebase),
    };
    let negative = || artifacts.get_native_expression_type(otherwise).cloned();
    match truth(condition_type) {
        Some(true) => positive(),
        Some(false) => negative(),
        None => Some(union(&positive()?, &negative()?, codebase)),
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
    if let Some(cast) = scalar_cast(&unary.operator) {
        return cast_type(cast, ty);
    }
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

#[derive(Clone, Copy)]
enum ScalarCast {
    Integer,
    Float,
    Boolean,
    String,
}

fn scalar_cast(operator: &UnaryPrefixOperator<'_>) -> Option<ScalarCast> {
    Some(match operator {
        UnaryPrefixOperator::IntCast(_, _) | UnaryPrefixOperator::IntegerCast(_, _) => {
            ScalarCast::Integer
        }
        UnaryPrefixOperator::FloatCast(_, _) | UnaryPrefixOperator::DoubleCast(_, _) => {
            ScalarCast::Float
        }
        UnaryPrefixOperator::BoolCast(_, _) | UnaryPrefixOperator::BooleanCast(_, _) => {
            ScalarCast::Boolean
        }
        UnaryPrefixOperator::StringCast(_, _) | UnaryPrefixOperator::BinaryCast(_, _) => {
            ScalarCast::String
        }
        _ => return None,
    })
}

// InitializerExprTypeResolver::getCastType delegates to the operand type's scalar conversion.
// Keep unsupported unions and non-finite operands unavailable, rather than consulting full types.
fn cast_type(cast: ScalarCast, ty: &TUnion) -> Option<TUnion> {
    use ScalarCast::{Boolean, Float, Integer, String};
    use TAtomic::{Mixed, Null, Scalar};
    use TScalar::{Bool, Float as Decimal, Integer as Int, String as Text};

    if !ty.is_single() {
        return None;
    }
    let atomic = ty.get_single();
    if matches!(atomic, Scalar(Decimal(TFloat::Literal(value))) if !value.is_finite()) {
        return None;
    }
    Some(match (cast, atomic) {
        (_, Mixed(mixed)) if mixed.is_vanilla() => match cast {
            Integer => get_int(),
            Float => get_float(),
            Boolean => get_bool(),
            String => get_string(),
        },
        (
            Boolean,
            Scalar(Int(_) | Decimal(TFloat::Float | TFloat::Literal(_)) | Bool(_) | Text(_)) | Null,
        ) => boolean(truth(ty)),
        (Integer, Scalar(Int(_)))
        | (Float, Scalar(Decimal(TFloat::Float | TFloat::Literal(_))))
        | (String, Scalar(Text(_))) => ty.clone(),
        (Integer, Null) => get_literal_int(0),
        (Float, Null) => get_literal_float(0.0),
        (String, Null) => get_literal_string(word("")),
        (Integer, Scalar(Bool(value))) => {
            boolean_cast_values(value.value, |value| TScalar::literal_int(i64::from(value)))
        }
        (Float, Scalar(Bool(value))) => boolean_cast_values(value.value, |value| {
            TScalar::literal_float(f64::from(value))
        }),
        (String, Scalar(Bool(value))) => boolean_cast_values(value.value, |value| {
            TScalar::literal_string(word(if value { "1" } else { "" }))
        }),
        (Float, Scalar(Int(integer))) => integer
            .get_literal_value()
            .map_or_else(get_float, |value| get_literal_float(value as f64)),
        (Integer, Scalar(Decimal(TFloat::Literal(value)))) => {
            get_literal_int(float_to_integer(**value))
        }
        (Integer, Scalar(Decimal(TFloat::Float))) => get_int(),
        (Integer, Scalar(Text(text))) => {
            if let Some(value) = text.get_known_literal_value() {
                let prefix = numeric_prefix(value);
                let number = prefix.parse::<f64>().unwrap_or(0.0);
                // Unlike float casts, PHP saturates overflowing numeric strings, but infinity casts to zero.
                get_literal_int(if number.is_finite() {
                    string_to_int(prefix.as_bytes())
                } else {
                    0
                })
            } else {
                get_int()
            }
        }
        (Float, Scalar(Text(text))) => {
            if let Some(value) = text.get_known_literal_value() {
                return float_literal(numeric_prefix(value).parse::<f64>().unwrap_or(0.0));
            }
            get_float()
        }
        (String, Scalar(Int(TInteger::Literal(value)))) => {
            get_literal_string(mago_word::i64_word(*value))
        }
        (String, Scalar(Decimal(TFloat::Literal(value)))) => {
            if **value == 0.0 {
                // ConstantFloatType::toString intentionally represents both zero signs.
                TUnion::from_vec(vec![
                    Scalar(TScalar::literal_string(word("-0"))),
                    Scalar(TScalar::literal_string(word("0"))),
                ])
            } else {
                get_literal_string(word(finite_float_string(**value)))
            }
        }
        (String, Scalar(Decimal(TFloat::Float))) => {
            // The shared union constructor drops casing when is_numeric is true.
            TUnion::from_atomic(Scalar(Text(TString::general_with_props(
                true,
                false,
                true,
                false,
                TStringCasing::Uppercase,
            ))))
        }
        // General/ranged integer strings need PHPStan's decimal-integer-string accessory, not numeric-string.
        _ => return None,
    })
}

fn boolean_cast_values(value: Option<bool>, cast: impl Fn(bool) -> TScalar) -> TUnion {
    match value {
        Some(value) => TUnion::from_atomic(TAtomic::Scalar(cast(value))),
        None => TUnion::from_vec(vec![
            TAtomic::Scalar(cast(false)),
            TAtomic::Scalar(cast(true)),
        ]),
    }
}

// PHP's finite float-to-int cast wraps modulo 2^64; Rust's `as i64` alone saturates.
fn float_to_integer(value: f64) -> i64 {
    let value = value.trunc() % 18446744073709551616.0;
    if value >= 9223372036854775808.0 {
        (value - 18446744073709551616.0) as i64
    } else if value < -9223372036854775808.0 {
        (value + 18446744073709551616.0) as i64
    } else {
        value as i64
    }
}

// ConstantStringType::{toInteger,toFloat} use PHP's leading numeric prefix, not is_numeric().
// PHP whitespace is six ASCII bytes, including vertical tab, and never Unicode whitespace.
fn numeric_prefix(input: &[u8]) -> &str {
    let start = input
        .iter()
        .position(|byte| !matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'\x0b' | b'\x0c'))
        .unwrap_or(input.len());
    let input = &input[start..];
    let mut offset = usize::from(matches!(input.first(), Some(b'+' | b'-')));
    let mut digits = false;
    while input.get(offset).is_some_and(u8::is_ascii_digit) {
        digits = true;
        offset += 1;
    }
    if input.get(offset) == Some(&b'.') {
        offset += 1;
        while input.get(offset).is_some_and(u8::is_ascii_digit) {
            digits = true;
            offset += 1;
        }
    }
    if !digits {
        return "";
    }
    if matches!(input.get(offset), Some(b'e' | b'E')) {
        let exponent = offset;
        offset += 1;
        if matches!(input.get(offset), Some(b'+' | b'-')) {
            offset += 1;
        }
        let start = offset;
        while input.get(offset).is_some_and(u8::is_ascii_digit) {
            offset += 1;
        }
        if offset == start {
            offset = exponent;
        }
    }
    // The scanned prefix contains ASCII digits and punctuation only, even for arbitrary PHP bytes.
    std::str::from_utf8(&input[..offset]).unwrap()
}

// ConstantFloatType::toString uses PHP's default precision=14, unlike Mago's shortest Ryu spelling.
// The caller handles zero (whose native type includes both signs) and excludes non-finite values.
fn finite_float_string(value: f64) -> String {
    let scientific = format!("{:.13e}", value.abs());
    let (mantissa, exponent) = scientific.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let digits = digits.trim_end_matches('0');
    let sign = if value < 0.0 { "-" } else { "" };
    if !(-4..14).contains(&exponent) {
        let (head, tail) = digits.split_at(1);
        let tail = if tail.is_empty() { "0" } else { tail };
        let exponent_sign = if exponent < 0 { '-' } else { '+' };
        return format!("{sign}{head}.{tail}E{exponent_sign}{}", exponent.abs());
    }
    if exponent < 0 {
        return format!("{sign}0.{}{digits}", "0".repeat((-exponent - 1) as usize));
    }
    let length = exponent as usize + 1;
    if digits.len() <= length {
        format!("{sign}{digits}{}", "0".repeat(length - digits.len()))
    } else {
        format!("{sign}{}.{}", &digits[..length], &digits[length..])
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
        if let Some(value) = literal_identity(left, right).or_else(|| literal_identity(right, left))
        {
            return Some(boolean(
                value.map(|value| value ^ operator.is_negated_equality()),
            ));
        }
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
    if ordered(operator)
        && let (Some((left_min, left_max)), Some((right_min, right_max))) =
            (integer_bounds(left), integer_bounds(right))
    {
        let value = match operator {
            BinaryOperator::LessThan(_) => {
                if left_max < right_min {
                    Some(true)
                } else if left_min >= right_max {
                    Some(false)
                } else {
                    None
                }
            }
            BinaryOperator::LessThanOrEqual(_) => {
                if left_max <= right_min {
                    Some(true)
                } else if left_min > right_max {
                    Some(false)
                } else {
                    None
                }
            }
            BinaryOperator::GreaterThan(_) => {
                if left_min > right_max {
                    Some(true)
                } else if left_max <= right_min {
                    Some(false)
                } else {
                    None
                }
            }
            _ => {
                if left_min >= right_max {
                    Some(true)
                } else if left_max < right_min {
                    Some(false)
                } else {
                    None
                }
            }
        };
        return Some(boolean(value));
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

fn integer_bounds(ty: &TUnion) -> Option<(i64, i64)> {
    let mut min = i64::MAX;
    let mut max = i64::MIN;
    for atomic in ty.types.iter() {
        let TAtomic::Scalar(TScalar::Integer(value)) = atomic else {
            return None;
        };
        min = min.min(value.get_minimum_value().unwrap_or(i64::MIN));
        max = max.max(value.get_maximum_value().unwrap_or(i64::MAX));
    }
    Some((min, max))
}

fn literal_identity(ty: &TUnion, literal: &TUnion) -> Option<Option<bool>> {
    let (kind, true) = identity_shape(literal)? else {
        return None;
    };
    let mut possible = false;
    let mut definite = true;
    for atomic in ty.types.iter() {
        let value = if let (
            TAtomic::Scalar(TScalar::Integer(value)),
            TAtomic::Scalar(TScalar::Integer(TInteger::Literal(want))),
        ) = (atomic, literal.get_single())
        {
            let min = value.get_minimum_value().unwrap_or(i64::MIN);
            let max = value.get_maximum_value().unwrap_or(i64::MAX);
            if *want < min || *want > max {
                Some(false)
            } else if min == max {
                Some(true)
            } else {
                None
            }
        } else {
            let (atomic_kind, constant) = identity_shape(&wrap_atomic(atomic.clone()))?;
            if atomic_kind != kind {
                Some(false)
            } else if constant {
                Some(atomic == literal.get_single())
            } else {
                None
            }
        };
        possible |= value != Some(false);
        definite &= value == Some(true);
    }
    Some(if definite {
        Some(true)
    } else if !possible {
        Some(false)
    } else {
        None
    })
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
