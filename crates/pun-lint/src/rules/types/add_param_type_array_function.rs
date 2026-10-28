//! `lint/types/add-param-type-array-function`, ported from Rector 2.6.1's `AddArrayFunctionClosureParamTypeRector`
//! (rules/TypeDeclaration/Rector/FuncCall) with `NativeFuncCallPositions`: in a call of `array_walk()`,
//! `array_map()`, `usort()` or `array_filter()` with two arguments, a closure or arrow function with one untyped
//! parameter gets the array's item type.
//!
//! Known differences on laravel-framework, where Mago infers an item type PHPStan does not: Pun adds one over a list
//! that `empty()` has narrowed to non-empty (ScheduleTestCommand), over an array literal of destructured values
//! (ParsesSqlServerConfigurationUrls) and over `array_slice()` of `explode()` (ExceptionReportingTest), and writes
//! `int|string|null` where PHPStan keeps the literal keys' order `string|int|null` (Cloud/Queue.php).

use mago_syntax::cst::{Node, NodeKind};

use super::add_param_type_array_any_all::closure_params;
use super::facts::facts;
use super::param::{add_hint, array_shape_of, is_mixed, item_type, param_declaration, report};
use crate::rules::laravel::get_arg;
use crate::rules::lint_b_helpers::value;
use crate::rules::rector_code_quality::named;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// `NativeFuncCallPositions::ARRAY_AND_CALLBACK_POSITIONS`: each function, its array's position and its callback's.
pub(crate) const ARRAY_AND_CALLBACK_POSITIONS: [(&str, usize, usize); 4] = [
    ("array_walk", 0, 1),
    ("array_map", 1, 0),
    ("usort", 0, 1),
    ("array_filter", 0, 1),
];

/// `PhpVersionFeature::SCALAR_TYPES`.
const SCALAR_TYPES: u32 = 70000;

pub struct AddParamTypeArrayFunction;

impl Rule for AddParamTypeArrayFunction {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-array-function"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if call.argument_list.arguments.len() != 2 {
            return;
        }
        let Some(&(_, array_at, callback_at)) = ARRAY_AND_CALLBACK_POSITIONS
            .iter()
            .find(|(name, _, _)| named(call, name))
        else {
            return;
        };
        let Some(callback) = get_arg(&call.argument_list, "callback", callback_at) else {
            return;
        };
        let Some(params) = closure_params(value(callback)) else {
            return;
        };
        let [param] = params.parameters.as_slice() else {
            return;
        };
        if param.hint.is_some() {
            return;
        }
        let Some(array) = get_arg(&call.argument_list, "array", array_at) else {
            return;
        };
        let facts = facts(ctx);
        let Some(item) = array_shape_of(&facts, value(array))
            .as_ref()
            .and_then(item_type)
        else {
            return;
        };
        if is_mixed(&item) {
            return;
        }
        let Some(declaration) = param_declaration(&item, &facts.codebase, ctx.php_version) else {
            return;
        };
        out.push(report(
            node_span(call),
            "Add the array's item type to the callback's parameter.",
            vec![add_hint(param, &declaration)],
        ));
    }
}
