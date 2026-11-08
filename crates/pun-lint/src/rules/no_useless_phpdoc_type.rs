//! `lint/dead-code/no-useless-phpdoc-type`, ported from Rector 2.8.4's `RemoveUselessUnionReturnDocblockRector`
//! (rules/DeadCode/Rector/ClassMethod), from PHP 8.0 (union types).
//!
//! The first `@return` of a function's or method's doc comment goes when it is a union that takes in the whole native
//! return type (`$doc->isSuperTypeOf($native)` is yes), so it says nothing more. It stays with a description, and with
//! `static` or `$this` in the union, and when the native type may be an array (the doc may say its items). The removal
//! takes `@phan-return` along, not the `@phpstan-` and `@psalm-` tags; a doc comment left empty goes.

use mago_syntax::cst::{Node, NodeKind};
use pun_phpstan::phpdoc::Node as DocNode;
use pun_problem::{Fix, Safety};

use super::lint_b_doc::Doc;
use super::no_useless_param_tag::function_like;
use super::types::compare::classes::with_classes;
use super::types::compare::is_super;
use super::types::compare::mapper::TypeScope;
use super::types::compare::tags::{Kind, first_named, removed_by_type};
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUselessPhpdocType;

impl Rule for NoUselessPhpdocType {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-phpdoc-type"
    }

    fn min_php_version(&self) -> u32 {
        80000
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method, NodeKind::Function])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some((_, Some(return_type))) = function_like(&node) else {
            return;
        };
        let start = node_span(&node).start;
        let Some(doc) = Doc::last_of(ctx, ancestors, start) else {
            return;
        };
        let Some((_, tag)) = first_named(&doc, "@return") else {
            return;
        };
        // keep description, it carries info the native type cannot; only union docblock types are handled here
        let DocNode::Union(members) = &tag.ty else {
            return;
        };
        if tag.has_description {
            return;
        }
        // keep static/$this, they denote the runtime subclass the native type cannot express
        if members
            .iter()
            .any(|m| matches!(m, DocNode::This) || matches!(m, DocNode::Ident(n) if n == "static"))
        {
            return;
        }
        let facts = facts(ctx);
        let mut chain = ancestors.to_vec();
        chain.push(node);
        let scope = TypeScope::new(&chain, facts.documented());
        let useless = with_classes(facts.documented(), || {
            let native = scope.map_hint(&return_type.hint);
            // keep array-involving native type (array, ?array, union with array), the docblock may carry element types
            if !native.to_super().is_array().no() {
                return false;
            }
            // the docblock adds nothing when it is broader than the native type
            is_super(&scope.map_doc(&tag.ty), &native).yes()
        });
        if !useless {
            return;
        }
        let removed = removed_by_type(&doc, Kind::Return, "");
        out.push(Report {
            span: doc.span,
            message: "Remove the `@return` union, which takes in the whole native return type."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![doc.remove_last(ctx.source, &removed)],
            }),
        });
    }
}
