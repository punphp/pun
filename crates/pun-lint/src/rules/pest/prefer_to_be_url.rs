//! `lint/pest/prefer-to-be-url`, ported from pest-plugin-rector's `UseToBeUrlRector`: `expect(filter_var($url,
//! FILTER_VALIDATE_URL))->not->toBeFalse()` and `expect(filter_var($url, FILTER_VALIDATE_URL) !== false)->toBeTrue()`
//! become `expect($url)->toBeUrl()` when `$url` may be a string.

use mago_syntax::cst::{BinaryOperator, Expression, FunctionCall, MethodCall, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{
    Is, Trinary, first_value, func_call, is_bool_constant, is_name, property_fetch, rename, report,
    subject_holder, type_is,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToBeUrl;

impl Rule for PreferToBeUrl {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-url"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        let Some(holder) = subject_holder(call) else {
            return;
        };
        let subject = first_value(holder.arguments);
        let mut edits = Vec::new();
        let url = if is_name(&call.method, "toBeFalse") && is_not_chain(call) {
            let Some(url) = subject
                .and_then(filter_var_validate_url)
                .and_then(|f| first_value(&f.argument_list))
            else {
                return;
            };
            // `removeNotFromChain`.
            if let Some(not) = property_fetch(call.object).filter(|p| is_name(&p.property, "not")) {
                edits.push(Edit::new(
                    Span::new(
                        node_span(unparenthesize(not.object)).end,
                        node_span(unparenthesize(call.object)).end,
                    ),
                    "",
                ));
            }
            url
        } else if is_name(&call.method, "toBeTrue") {
            let Some(Expression::Binary(comparison)) = subject.map(unparenthesize) else {
                return;
            };
            if !matches!(comparison.operator, BinaryOperator::NotIdentical(_))
                || !is_bool_constant(comparison.rhs, "false")
            {
                return;
            }
            let Some(url) =
                filter_var_validate_url(comparison.lhs).and_then(|f| first_value(&f.argument_list))
            else {
                return;
            };
            url
        } else {
            return;
        };
        if type_is(ctx, url, Is::String) == Trinary::No {
            return;
        }
        edits.extend(holder.set_arguments(ctx, url));
        edits.push(rename(call, "toBeUrl"));
        out.push(report(
            node_span(call),
            "Use `toBeUrl()` instead of `filter_var()` with `FILTER_VALIDATE_URL`.".to_string(),
            Safety::Unsafe,
            edits,
        ));
    }
}

/// `isFilterVarValidateUrl`: a call of `filter_var` whose second argument is the constant `FILTER_VALIDATE_URL`.
fn filter_var_validate_url<'a>(expr: &'a Expression<'a>) -> Option<&'a FunctionCall<'a>> {
    let call = func_call(expr).filter(|f| named(f, "filter_var"))?;
    let filter = call.argument_list.arguments.get(1)?;
    match unparenthesize(super::argument_value(filter)) {
        Expression::ConstantAccess(c) => {
            let name = c.name.value();
            (name.strip_prefix(b"\\").unwrap_or(name))
                .eq_ignore_ascii_case(b"FILTER_VALIDATE_URL")
                .then_some(call)
        }
        _ => None,
    }
}

/// `isNotChain`: a `->not` among the property fetches right under the call.
fn is_not_chain(call: &MethodCall<'_>) -> bool {
    let mut var = call.object;
    while let Some(fetch) = property_fetch(var) {
        if is_name(&fetch.property, "not") {
            return true;
        }
        var = fetch.object;
    }
    false
}
