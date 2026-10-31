//! PHPStan's synthetic FuncCall(TypeExpr...) pricing, using the ordinary invocation engine.

use std::rc::Rc;

use mago_allocator::{Arena, vec_in};
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::{Argument, ArgumentList, FunctionCall, PositionalArgument, sequence::TokenSeparatedSequence};
use mago_word::word;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::common::synthetic::new_synthetic_variable;
use crate::context::{Context, block::BlockContext};
use crate::error::AnalysisError;
use crate::plugin::provider::function::CallbackInvocation;

pub(super) fn evaluate<'ctx, 'arena, A: Arena>(
    context: &mut Context<'ctx, 'arena, A>,
    before: &BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
    request: &CallbackInvocation<'_, 'arena>,
) -> Result<TUnion, AnalysisError> {
    let mut scope = before.clone();
    scope.if_body_context = None;
    scope.finally_scope = None;
    // These synthetic full-only values are not native observations of source operands.
    scope.native_locals = None;
    let mut scratch = AnalysisArtifacts::new();
    let callback = artifacts
        .get_expression_type(request.callback)
        .expect("a callback request refers to an evaluated operand")
        .clone();
    let arena = context.arena;
    let mut variable = |index: usize, value: TUnion| {
        // NUL cannot occur in a PHP source variable name; no captured local can collide.
        let name = format!("$\0callback_type_expr_{index}");
        let mut span = Span::zero();
        span.start.offset = index as u32 * 2;
        span.end.offset = span.start.offset + 1;
        scope.locals.insert(word(name.as_bytes()), Rc::new(value));
        new_synthetic_variable(arena, name.as_bytes(), span)
    };
    let callee = arena.alloc(variable(0, callback));
    let mut arguments = vec_in![arena];
    for (index, value) in request.arguments.iter().enumerate() {
        arguments.push(Argument::Positional(PositionalArgument {
            ellipsis: None,
            value: arena.alloc(variable(index + 1, value.clone())),
        }));
    }
    let call = FunctionCall {
        function: callee,
        argument_list: ArgumentList {
            left_parenthesis: Span::zero(),
            arguments: TokenSeparatedSequence::new(arguments, vec_in![arena]),
            right_parenthesis: Span::zero(),
        },
    };
    context.gather_callback(|context| {
        call.analyze(context, &mut scope, &mut scratch)?;
        Ok(scratch
            .get_expression_type(&call.span())
            .expect("analyzed invocation has a return")
            .clone())
    })
}
