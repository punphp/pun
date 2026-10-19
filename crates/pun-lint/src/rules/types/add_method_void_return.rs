//! `lint/types/add-method-void-return`, ported from Rector 2.6.1's `AddVoidReturnTypeWhereNoReturnRector`
//! (rules/TypeDeclaration/Rector/ClassMethod).
//!
//! A method without a return type that returns no value and does not yield gets `: void`, unless a child class may
//! return something (an empty or throw-only method of a class that is not final, a protected method of one), or a
//! parent declares the method's return type (`ClassMethodReturnVendorLockResolver`).

use mago_syntax::cst::{MethodBody, Modifier, Node, NodeKind};

use super::facts::facts;
use super::{
    MAGIC, VOID_TYPE, add_return_type, class_scope, has_exclusive_void, is_return_vendor_locked,
    is_throw_statement, statement_count,
};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddMethodVoidReturn;

impl Rule for AddMethodVoidReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-method-void-return"
    }

    fn min_php_version(&self) -> u32 {
        VOID_TYPE
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
        let Node::Method(method) = node else {
            return;
        };
        // abstract and interface methods have no body
        let (None, MethodBody::Concrete(body)) = (&method.return_type_hint, &method.body) else {
            return;
        };
        let Some(scope) = class_scope(ancestors) else {
            return;
        };
        // shouldSkipClassMethod
        let count = statement_count(ctx.source, body);
        if !scope.is_final {
            let only_throw = count == 1 && body.statements.first().is_some_and(is_throw_statement);
            if only_throw || count == 0 {
                return;
            }
        }
        let modifiers = &method.modifiers;
        if modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Protected(_)))
        {
            if !scope.is_final {
                return;
            }
        } else if scope.is_abstract && count == 0 {
            return;
        }
        if scope.is_interface || !has_exclusive_void(body) {
            return;
        }
        // ClassMethodReturnVendorLockResolver::isVendorLocked
        let name = method.name.value.to_ascii_lowercase();
        if name != b"__invoke" && MAGIC.iter().any(|m| m.as_bytes() == name) {
            return;
        }
        let private = modifiers.iter().any(|m| matches!(m, Modifier::Private(_)));
        if !private && is_return_vendor_locked(&facts(ctx), &scope, method.name.value) {
            return;
        }
        out.push(add_return_type(
            node_span(&method.name),
            node_span(&method.parameter_list).end,
            "void",
        ));
    }
}
