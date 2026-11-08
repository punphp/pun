//! `lint/phpunit/add-test-void-return`, ported from Rector 2.8.4's `AddTestsVoidReturnTypeWhereNoReturnRector`
//! (rules/TypeDeclaration/Rector/Class_).
//!
//! In a test class, a test method (`TestsNodeAnalyzer::isTestClassMethod()`) without a return type that is not
//! abstract and returns no value and does not yield (`SilentVoidResolver::hasExclusiveVoid()`) gets `: void`.

use mago_syntax::cst::{ClassLikeMember, MethodBody, Node, NodeKind};

use super::annotations::named_tags;
use super::{class_body, is_in_test_class, is_public};
use crate::rules::lint_b_doc::Doc;
use crate::rules::rector_batch5::has_attribute;
use crate::rules::types::{VOID_TYPE, add_return_type, has_exclusive_void};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddTestVoidReturn;

impl Rule for AddTestVoidReturn {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-test-void-return"
    }

    fn min_php_version(&self) -> u32 {
        VOID_TYPE
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (members, _, _) = class_body(node).unwrap();
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        for member in members {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            // has type already; an abstract method has no body
            let (None, MethodBody::Concrete(body)) = (&method.return_type_hint, &method.body)
            else {
                continue;
            };
            if !is_public(method) {
                continue;
            }
            let is_test = method.name.value.starts_with(b"test")
                || has_attribute(
                    &scope,
                    &method.attribute_lists,
                    "PHPUnit\\Framework\\Attributes\\Test",
                )
                || Doc::of(ctx, &scope, node_span(method).start).is_some_and(|doc| {
                    named_tags(ctx, &scope, &doc, "test").is_some_and(|t| !t.is_empty())
                });
            if !is_test || !has_exclusive_void(body) {
                continue;
            }
            out.push(add_return_type(
                node_span(&method.name),
                node_span(&method.parameter_list).end,
                "void",
            ));
        }
    }
}
