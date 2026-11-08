//! `lint/types/add-param-phpdoc-from-reference-assign`, ported from Rector 2.8.4's
//! `AddParamArrayDocblockFromAssignsParamToParamReferenceRector` (rules/TypeDeclarationDocblocks/Rector/ClassMethod):
//! a by-reference parameter declared `array` that a private method assigns one item to gets that item's type as `T[]`.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_phpstan::types::{Type, mixed};

use super::docblock::{FunctionLike, assignments, doc_report, is_array_hint, variable_name};
use crate::rules::phpdoc_writer::type_mapper::is_useful_array_tag;
use crate::rules::phpdoc_writer::{decorate_param, expr_type};
use crate::{Context, Report, Rule};

pub struct AddParamPhpdocFromReferenceAssign;

impl Rule for AddParamPhpdocFromReferenceAssign {
    fn id(&self) -> &'static str {
        "lint/types/add-param-phpdoc-from-reference-assign"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = FunctionLike::of(node).filter(|f| f.method.is_some()) else {
            return;
        };
        // a by-ref param type is invariant in PHPStan; narrowing it below array breaks callers passing a plain array,
        // which are invisible for a non-private method
        if !matches!(node, Node::Method(m) if m.modifiers.iter().any(|m| matches!(m, mago_syntax::cst::Modifier::Private(_))))
        {
            return;
        }
        let target = f.target(ctx);
        let mut info = target.info(ctx.source);
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let mut changed = false;
        for param in &f.parameters.parameters {
            if param.ampersand.is_none() || !param.hint.as_ref().is_some_and(is_array_hint) {
                continue;
            }
            let name = String::from_utf8_lossy(&param.variable.name[1..]).into_owned();
            if is_useful_array_tag(info.param_tag(&name).map(|t| &t.ty)) {
                continue;
            }
            // `ArrayDimFetchFinder::findDimFetchAssignToVariableName()`
            let exprs: Vec<&Expression<'_>> = assignments(node)
                .into_iter()
                .filter(|a| match a.lhs {
                    Expression::ArrayAccess(d) => variable_name(d.array) == Some(name.as_str()),
                    Expression::ArrayAppend(d) => variable_name(d.array) == Some(name.as_str()),
                    _ => false,
                })
                .map(|a| a.rhs)
                .collect();
            // to kick off with one
            let [expr] = exprs.as_slice() else {
                continue;
            };
            // (a type Pun does not map, such as a closure's, adds nothing)
            if let Some(t) = expr_type(ctx, &scope, expr)
                && decorate_param(
                    &mut info,
                    &Type::array(mixed(), t),
                    &name,
                    param.ellipsis.is_some(),
                )
            {
                changed = true;
            }
        }
        if changed {
            out.extend(doc_report(
                f.name,
                "Add the `@param` array type from the value assigned to the reference parameter.",
                ctx.source,
                &target,
                &info,
            ));
        }
    }
}
