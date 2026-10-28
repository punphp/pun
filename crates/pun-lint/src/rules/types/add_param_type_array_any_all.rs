//! `lint/types/add-param-type-array-any-all` and `lint/types/prefer-narrow-param-type`, ported from Rector 2.6.1's
//! `AddArrayAnyAllClosureParamTypeRector` and `NarrowArrayAnyAllNullableParamTypeRector` (rules/TypeDeclaration/
//! Rector/FuncCall): the first parameter of the closure or arrow function passed to `array_any()`/`array_all()` gets
//! the array's item type when it has none; with `array_find()`/`array_find_key()` too, a nullable one (`?T`) is
//! narrowed to an item type that cannot be `null`.
//!
//! Known differences on laravel-framework: Pun adds `?UploadedFile` over `$files` that `is_array()` and a wrap made
//! an array, where PHPStan's item type is `mixed` (InteractsWithInput), and writes a property docblock's union in
//! Mago's sorted order, `string|\Closure`, where PHPStan keeps the written `\Closure|string` (Events/Dispatcher.php).

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{Expression, FunctionLikeParameterList, Hint, Node, NodeKind};

use super::facts::facts;
use super::param::{add_hint, array_shape_of, is_mixed, item_type, param_declaration, report};
use crate::rules::laravel::get_arg;
use crate::rules::lint_b_helpers::value;
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PhpVersionFeature::ARRAY_ANY`.
const ARRAY_ANY: u32 = 80400;

pub struct AddParamTypeArrayAnyAll;

pub struct PreferNarrowParamType;

impl Rule for AddParamTypeArrayAnyAll {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-array-any-all"
    }

    fn min_php_version(&self) -> u32 {
        ARRAY_ANY
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
        visit(node, ctx, out, &["array_any", "array_all"], false);
    }
}

impl Rule for PreferNarrowParamType {
    fn id(&self) -> &'static str {
        "lint/types/prefer-narrow-param-type"
    }

    fn min_php_version(&self) -> u32 {
        ARRAY_ANY
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
        visit(
            node,
            ctx,
            out,
            &["array_any", "array_all", "array_find", "array_find_key"],
            true,
        );
    }
}

/// The shared `refactor()`: `narrow` for the nullable parameter of `NarrowArrayAnyAllNullableParamTypeRector`.
fn visit(node: Node<'_, '_>, ctx: &Context<'_>, out: &mut Vec<Report>, names: &[&str], narrow: bool) {
    let Node::FunctionCall(call) = node else { return };
    if !names.iter().any(|n| named(call, n)) {
        return;
    }
    let (Some(array), Some(callback)) = (
        get_arg(&call.argument_list, "array", 0),
        get_arg(&call.argument_list, "callback", 1),
    ) else {
        return;
    };
    let Some(params) = closure_params(value(callback)) else {
        return;
    };
    let Some(param) = params.parameters.first() else {
        return;
    };
    let hint_span = match (&param.hint, narrow) {
        (None, false) => None,
        (Some(hint @ Hint::Nullable(_)), true) => Some(node_span(hint)),
        _ => return,
    };
    let facts = facts(ctx);
    let Some(item) = array_shape_of(&facts, value(array))
        .as_ref()
        .and_then(item_type)
    else {
        return;
    };
    if is_mixed(&item) || (narrow && item.types.iter().any(|t| matches!(t, TAtomic::Null))) {
        return;
    }
    let Some(declaration) = param_declaration(&item, &facts.codebase, ctx.php_version) else {
        return;
    };
    let edit = match hint_span {
        Some(span) => pun_problem::Edit::new(span, declaration),
        None => add_hint(param, &declaration),
    };
    out.push(report(
        node_span(call),
        "Add the array's item type to the callback's parameter.",
        vec![edit],
    ));
}

/// The parameters of a closure or an arrow function.
pub(crate) fn closure_params<'a>(expr: &'a Expression<'a>) -> Option<&'a FunctionLikeParameterList<'a>> {
    match unparenthesize(expr) {
        Expression::Closure(c) => Some(&c.parameter_list),
        Expression::ArrowFunction(a) => Some(&a.parameter_list),
        _ => None,
    }
}
