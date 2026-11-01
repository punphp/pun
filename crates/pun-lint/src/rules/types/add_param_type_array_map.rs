//! `lint/types/add-param-type-array-map`, ported from Rector 2.6.1's `AddClosureParamTypeForArrayMapRector`
//! (rules/TypeDeclaration/Rector/FunctionLike): the closure passed first to `array_map()` gets, for each untyped
//! parameter, the item type of the array passed at its position, when that array is an `ArrayType` (not a constant
//! array, nor a non-empty one, whose item type Rector's type corrector makes `mixed`).
//!
//! `AccessoryNonEmptyArrayTypeCorrector` makes `explode()` of a lowercase or uppercase string `array<mixed, string>`,
//! except where `str_contains()` (or `str_starts_with()`, `str_ends_with()`) of the haystack proves the delimiter
//! there, which PHPStan's extension finds by the fully qualified name only. An item type that is a list or a
//! non-empty array is PHPStan's intersection, which Rector maps to nothing; of `A&B` with `A` a subclass of `B`,
//! PHPStan keeps `A`.

use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Argument, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::facts::facts;
use super::param::{
    ArrayShape, add_hint, array_shape_of, doc_order, is_mixed, own_static, param_declaration_in,
};
use super::class_scope;
use crate::rules::lint_b_helpers::value;
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddParamTypeArrayMap;

impl Rule for AddParamTypeArrayMap {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-array-map"
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
        let Node::FunctionCall(call) = node else { return };
        if !named(call, "array_map") {
            return;
        }
        let args: Vec<&Argument<'_>> = call.argument_list.arguments.iter().collect();
        let Some(Expression::Closure(closure)) = args.first().map(|a| unparenthesize(value(a)))
        else {
            return;
        };
        let facts = facts(ctx);
        let order = doc_order(ctx.source, ancestors);
        let params: Vec<_> = closure.parameter_list.parameters.iter().collect();
        let mut edits = Vec::new();
        for (position, arg) in args[1..].iter().enumerate() {
            // spread (...$arrays) cannot be mapped positionally
            if matches!(arg, Argument::Positional(p) if p.ellipsis.is_some()) {
                return;
            }
            let Some(param) = params.get(position) else {
                continue;
            };
            let Some(ArrayShape::General(Some(item))) = array_shape_of(&facts, ancestors, value(arg))
            else {
                continue;
            };
            if is_mixed(&item) || param.hint.is_some() {
                continue;
            }
            let class = class_scope(ancestors).and_then(|scope| scope.name);
            let item = TUnion::from_vec(item.types.iter().map(|t| own_static(t, class.as_deref())).collect());
            if let Some(declaration) = param_declaration_in(&item, &facts.codebase, ctx.php_version, &order) {
                edits.push(add_hint(param, &declaration));
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report::fixed(
            node_span(call),
            "Add the arrays' item types to the array_map() closure's parameters.",
            Safety::Unsafe,
            edits,
        ));
    }
}
