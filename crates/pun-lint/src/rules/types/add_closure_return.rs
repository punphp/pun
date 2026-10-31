//! `lint/types/add-closure-return-type` and `lint/types/add-arrow-function-return-type`, ported from Rector 2.6.1's
//! `ClosureReturnTypeRector` (rules/TypeDeclaration/Rector/Closure) and `AddArrowFunctionReturnTypeRector`
//! (rules/TypeDeclaration/Rector/ArrowFunction): a closure gets the union of its returns' native types, an arrow
//! function the native type of its expression.
//!
//! Mago does not tell a declared `mixed` from an unknown type, so neither gets a type here (Rector writes `mixed`
//! for the declared one); and an arrow function returning an offset (`fn () => $a['k']`), for which Rector reads
//! the docblock type, gets its native type.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Expression, Node, NodeKind};

use super::declare::{expression_order, return_declaration};
use super::facts::{Facts, facts};
use super::native::native_type;
use super::silent_void::{Place, has_silent_void};
use super::{add_return_type, class_scope, returns_scoped};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddClosureReturn;

pub struct AddArrowFunctionReturn;

/// `PhpVersionFeature::SCALAR_TYPES`.
const SCALAR_TYPES: u32 = 70000;
/// `PhpVersionFeature::ARROW_FUNCTION`.
const ARROW_FUNCTION: u32 = 70400;

impl Rule for AddClosureReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-closure-return-type"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        closure_return(node, ancestors, ctx, out, false);
    }
}

/// `ClosureReturnTypeRector::refactor()` over the native types of the returns, or, for
/// `ClosureReturnTypeFromAssertInstanceOfRector`, over the types PHPStan knows there (`full`).
pub(crate) fn closure_return<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    ancestors: &[Node<'ast, 'arena>],
    ctx: &Context<'_>,
    out: &mut Vec<Report>,
    full: bool,
) {
    let Node::Closure(closure) = node else {
        return;
    };
    if closure.return_type_hint.is_some() {
        return;
    }
    let facts = facts(ctx);
    let scope = class_scope(ancestors);
    let place = Place {
        facts: &facts,
        ancestors,
        scope: scope.as_ref(),
    };
    // ReturnedNodesReturnTypeInfererTypeInferer::inferFunctionLike
    let returns = returns_scoped(&closure.body);
    if returns.is_empty() {
        return;
    }
    let mut types: Vec<&TAtomic> = Vec::new();
    let mut has_void = false;
    for r in &returns {
        match r.value {
            None => has_void = true,
            Some(value) => match expression_order(value, |expr| type_of(&facts, expr, full)) {
                Some(t) => types.extend(t),
                None => return,
            },
        }
    }
    if !has_void && has_silent_void(&place, closure.body.statements.iter()) {
        has_void = true;
    }
    // only `void` is mixed; `void` with others has no declaration; `never` is another rule's
    if has_void
        || types
            .iter()
            .any(|t| matches!(t, TAtomic::Never | TAtomic::Mixed(_)))
    {
        return;
    }
    let Some(r#type) = return_declaration(types, &facts.codebase, ctx.php_version, &[], true)
    else {
        return;
    };
    let at = match &closure.use_clause {
        Some(uses) => node_span(uses).end,
        None => node_span(&closure.parameter_list).end,
    };
    out.push(add_return_type(node_span(&closure.function), at, &r#type));
}

/// The native type of `expr`, or the type PHPStan knows there.
fn type_of<'f>(facts: &'f Facts, expr: &Expression<'_>, full: bool) -> Option<&'f TUnion> {
    if full {
        facts
            .full_type(node_span(expr))
            .filter(|t| !t.types.is_empty())
    } else {
        native_type(facts, expr)
    }
}

impl Rule for AddArrowFunctionReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-arrow-function-return-type"
    }

    fn min_php_version(&self) -> u32 {
        ARROW_FUNCTION
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ArrowFunction])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ArrowFunction(arrow) = node else {
            return;
        };
        if arrow.return_type_hint.is_some() {
            return;
        }
        let facts = facts(ctx);
        let Some(types) = expression_order(arrow.expression, |expr| native_type(&facts, expr))
        else {
            return;
        };
        if types
            .iter()
            .any(|t| matches!(t, TAtomic::Void | TAtomic::Never | TAtomic::Mixed(_)))
        {
            return;
        }
        if let Some(r#type) = return_declaration(types, &facts.codebase, ctx.php_version, &[], true)
        {
            out.push(add_return_type(
                node_span(&arrow.r#fn),
                node_span(&arrow.parameter_list).end,
                &r#type,
            ));
        }
    }
}
