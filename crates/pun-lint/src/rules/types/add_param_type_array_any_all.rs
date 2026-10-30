//! `lint/types/add-param-type-array-any-all` and `lint/types/prefer-narrow-param-type`, ported from Rector 2.6.1's
//! `AddArrayAnyAllClosureParamTypeRector` and `NarrowArrayAnyAllNullableParamTypeRector` (rules/TypeDeclaration/
//! Rector/FuncCall): the first parameter of the closure or arrow function passed to `array_any()`/`array_all()` gets
//! the array's item type when it has none; with `array_find()`/`array_find_key()` too, a nullable one (`?T`) is
//! narrowed to an item type that cannot be `null`.
//!
//! A property's docblock union keeps its written order (`@var array<string, \Closure|string>`), and a constant
//! array's values the order of its keys, as PHPStan has them.

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{Expression, FunctionLikeParameterList, Hint, Node, NodeKind};

use super::facts::facts;
use super::param::{
    ArrayShape, add_hint, array_shape_of, constant_values, constant_values_union, is_mixed, item_declaration, item_type,
    param_declaration_unsorted, report,
};
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
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        visit(node, ancestors, ctx, out, &["array_any", "array_all"], false);
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
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        visit(
            node,
            ancestors,
            ctx,
            out,
            &["array_any", "array_all", "array_find", "array_find_key"],
            true,
        );
    }
}

/// The shared `refactor()`: `narrow` for the nullable parameter of `NarrowArrayAnyAllNullableParamTypeRector`.
fn visit(
    node: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    ctx: &Context<'_>,
    out: &mut Vec<Report>,
    names: &[&str],
    narrow: bool,
) {
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
    let Some(shape) = array_shape_of(&facts, ancestors, value(array)) else { return };
    let Some(item) = item_type(&shape) else {
        return;
    };
    if is_mixed(&item) || (narrow && item.types.iter().any(|t| matches!(t, TAtomic::Null))) {
        return;
    }
    let declaration = match shape {
        // `ConstantArrayType::getItemType()`, PHPStan's union of the values in the order their keys were set
        ArrayShape::Constant(values) => {
            let Some(types) = constant_values(&facts, ancestors, value(array), &values) else { return };
            param_declaration_unsorted(&constant_values_union(&types), &facts.codebase, ctx.php_version)
        }
        ArrayShape::General(_) => item_declaration(&facts, ancestors, value(array), &item, ctx.php_version),
    };
    let Some(declaration) = declaration else {
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
