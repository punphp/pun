//! The analyser hook behind `Facts::implicit_array_writes`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use mago_analyzer::context::block::BreakContext;
use mago_analyzer::plugin::{
    ExpressionHook, ExpressionHookResult, HookContext, HookResult, Provider, ProviderMeta,
};
use mago_span::HasSpan;
use mago_syntax::cst::{AssignmentOperator, Expression, Variable};

pub(super) const YES: u8 = 0;
pub(super) const MAYBE: u8 = 1;
pub(super) const NO: u8 = 2;

const SUPERGLOBALS: [&[u8]; 9] = [
    b"$GLOBALS",
    b"$_SERVER",
    b"$_GET",
    b"$_POST",
    b"$_FILES",
    b"$_COOKIE",
    b"$_SESSION",
    b"$_REQUEST",
    b"$_ENV",
];

/// The state of each watched variable by assignment span; a loop body analysed twice with different answers is a maybe.
pub(super) struct Observer(pub Arc<Mutex<HashMap<(u32, u32), (String, u8)>>>);

impl Provider for Observer {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "pun-implicit-array",
            "implicit array",
            "variable certainty at array writes",
        );
        &META
    }
}

impl ExpressionHook for Observer {
    fn is_observer(&self) -> bool {
        true
    }

    fn before_expression(
        &self,
        expr: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        let Expression::Assignment(assignment) = expr else {
            return Ok(ExpressionHookResult::Continue);
        };
        if !matches!(assignment.operator, AssignmentOperator::Assign(_)) {
            return Ok(ExpressionHookResult::Continue);
        }
        let mut root = assignment.lhs;
        let mut nested = false;
        loop {
            match root {
                Expression::ArrayAccess(a) => root = a.array,
                Expression::ArrayAppend(a) => root = a.array,
                _ => break,
            }
            nested = true;
        }
        let Expression::Variable(Variable::Direct(variable)) = root else {
            return Ok(ExpressionHookResult::Continue);
        };
        if !nested || SUPERGLOBALS.contains(&variable.name) {
            return Ok(ExpressionHookResult::Continue);
        }
        let block = context.block_context();
        let certainty = block.variable_certainty(mago_word::word(variable.name));
        // in a loop, this write defines the variable for the next iteration
        let in_loop = (block.break_types.iter()).any(|b| matches!(b, BreakContext::Loop(_)));
        let state = if certainty.is_defined() {
            YES
        } else if certainty.is_absent() && !in_loop {
            NO
        } else {
            MAYBE
        };
        let span = expr.span();
        let key = (span.start.offset, span.end.offset);
        let name = String::from_utf8_lossy(variable.name).into_owned();
        let mut found = self.0.lock().unwrap_or_else(|e| e.into_inner());
        let entry = found.entry(key).or_insert((name, state));
        if entry.1 != state {
            entry.1 = MAYBE;
        }
        Ok(ExpressionHookResult::Continue)
    }
}
