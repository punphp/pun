//! `lint/types/add-return-type-from-native-call`, ported from Rector 2.8.4's `ReturnTypeFromStrictNativeCallRector`
//! (rules/TypeDeclaration/Rector/ClassMethod, with `AddReturnTypeFromStrictNativeCall`,
//! `StrictNativeFunctionReturnTypeAnalyzer` and `ExclusiveNativeCallLikeReturnMatcher`): a function or method whose
//! every `return` returns a call of one of PHP's own functions or methods gets the union of the calls' types.

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{Call, Expression, Node, NodeKind};

use super::declare::{return_declaration, union_declaration, union_order};
use super::facts::facts;
use super::silent_void::Place;
use super::type_factory::{Created, create_keep_constant, merged_arrays};
use super::{
    SCALAR_TYPES, add_return_type, class_scope, has_only_return_with_expr, returns_scoped, untyped,
};
use crate::rules::require_named_argument_order::{Target, callee};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromNativeCall;

impl Rule for AddReturnTypeFromNativeCall {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-native-call"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = untyped(node, ancestors, ctx) else {
            return;
        };
        let scope = class_scope(ancestors);
        let facts = facts(ctx);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let returns = returns_scoped(f.body);
        if !has_only_return_with_expr(&place, f.body, &returns) {
            return;
        }
        let mut types: Vec<TAtomic> = Vec::new();
        for value in returns.iter().filter_map(|r| r.value) {
            // `ExclusiveNativeCallLikeReturnMatcher`: a function, method or static method call of a built-in
            let target = match value {
                Expression::Call(Call::Function(c)) => Target::Function(c.function),
                Expression::Call(Call::Method(c)) => Target::Method(c.object, &c.method),
                Expression::Call(Call::StaticMethod(c)) => Target::Static(c.class, &c.method),
                _ => return,
            };
            if !callee(&facts, target, ancestors).is_some_and(|m| m.flags.is_built_in()) {
                return;
            }
            // `NodeTypeResolver::getType`: the call's type with the docblocks read
            let Some(t) = facts.full_type(node_span(value)) else {
                return;
            };
            types.extend(merged_arrays(
                union_order(t.types.as_ref()).into_iter().cloned().collect(),
            ));
        }
        let r#type = match create_keep_constant(types) {
            Created::Mixed => return,
            Created::Single(t) => {
                return_declaration([&t], &facts.codebase, ctx.php_version, &[], true)
            }
            Created::Union(types) => union_declaration(
                &types.iter().collect::<Vec<_>>(),
                &facts.codebase,
                ctx.php_version,
            ),
        };
        if let Some(r#type) = r#type {
            out.push(add_return_type(f.span, f.at, &r#type));
        }
    }
}
