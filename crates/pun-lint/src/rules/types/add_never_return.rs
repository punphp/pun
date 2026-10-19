//! `lint/types/add-never-return-type`, ported from Rector 2.6.1's `ReturnNeverTypeRector` (rules/TypeDeclaration/
//! Rector/ClassMethod): a function or method that never returns gets the `never` return type (see [`super::never`]).
//! A public method of a PHPUnit test class that already has a return type is left alone.

use mago_syntax::cst::{MethodBody, Modifier, Node, NodeKind};

use super::class_scope;
use super::facts::facts;
use super::never::{FunctionLike, NEVER_TYPE, add_never, is_test_class};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddNeverReturn;

impl Rule for AddNeverReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-never-return-type"
    }

    fn min_php_version(&self) -> u32 {
        NEVER_TYPE
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
        let f = match node {
            Node::Function(function) => FunctionLike {
                span: node_span(&function.name),
                at: node_span(&function.parameter_list).end,
                return_type: function.return_type_hint.as_ref(),
                body: &function.body,
                method: None,
            },
            Node::Method(method) => {
                let MethodBody::Concrete(body) = &method.body else {
                    return;
                };
                // isTestClassMethodWithFilledReturnType: public is no visibility modifier or `public`
                let public = !method
                    .modifiers
                    .iter()
                    .any(|m| matches!(m, Modifier::Private(_) | Modifier::Protected(_)));
                if public
                    && method.return_type_hint.is_some()
                    && class_scope(ancestors).is_some_and(|s| is_test_class(&facts(ctx), &s))
                {
                    return;
                }
                FunctionLike {
                    span: node_span(&method.name),
                    at: node_span(&method.parameter_list).end,
                    return_type: method.return_type_hint.as_ref(),
                    body,
                    method: Some(method),
                }
            }
            _ => return,
        };
        out.extend(add_never(f, ancestors, ctx));
    }
}
