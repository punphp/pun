//! Partial, passive PHPStan-native state. These transfers never evaluate an expression or choose a path.

use std::rc::Rc;

use mago_codex::metadata::function_like::{FunctionLikeKind, FunctionLikeMetadata};
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{AssignmentOperator, Call, Expression, Literal, Statement, Variable};
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
        _ => artifacts.get_rc_native_expression_type(expression).cloned(),
    };
    if let Some(ty) = ty {
        artifacts.set_rc_native_expression_type(expression, ty);
    } else {
        block.native_locals = None;
    }
}
