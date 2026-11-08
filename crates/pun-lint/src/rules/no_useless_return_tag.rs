//! `lint/phpdoc/no-useless-return`, ported from Rector 2.8.4's `RemoveUselessReturnTagRector`
//! (rules/DeadCode/Rector/ClassMethod) with `ReturnTagRemover` and `DeadReturnTagValueNodeAnalyzer`
//! (rules/DeadCode/PhpDoc).
//!
//! The first `@return` of a function's or method's doc comment goes when the native return type already says what
//! it says, or when it says nothing useful next to it (`@return void`, `@return mixed`, any type over a native
//! `void` or `never` but `never` over `void`, a nullable doc type over its native type without `null`). It stays
//! with a description, a generic or other type the native one cannot say, a template type, `$this` in a trait,
//! `true` or `false` over a native `bool`, or a union with `true`, `false`, a `mixed[]` or a generic member. The
//! removal takes `@phan-return` along, not the `@phpstan-` and `@psalm-` tags; a doc comment left empty goes.

use mago_syntax::cst::{Hint, Node, NodeKind};
use pun_phpstan::phpdoc::Node as DocNode;
use pun_problem::{Fix, Safety};

use super::lint_b_doc::Doc;
use super::no_useless_param_tag::{function_like, is_bool_identifier};
use super::types::compare::classes::with_classes;
use super::types::compare::mapper::TypeScope;
use super::types::compare::tags::{
    Kind, TypeTag, first_named, has_generic, has_mixed_array, is_allowed, is_standalone_legal,
    removed_by_type,
};
use super::types::compare::{
    are_native_and_doc_types_equal, are_types_equal, is_legal_without_template, remove_null,
};
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUselessReturnTag;

impl Rule for NoUselessReturnTag {
    fn id(&self) -> &'static str {
        "lint/phpdoc/no-useless-return"
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
        let facts = facts(ctx);
        let mut chain = ancestors.to_vec();
        chain.push(node);
        let scope = TypeScope::new(&chain, facts.documented());
        if !with_classes(facts.documented(), || {
            is_dead(&tag, &return_type.hint, &scope)
        }) {
            return;
        }
        let removed = removed_by_type(&doc, Kind::Return, "");
        out.push(Report {
            span: doc.span,
            message: "Remove the `@return` tag, which says what the native type says.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![doc.remove_last(ctx.source, &removed)],
            }),
        });
    }
}

/// `DeadReturnTagValueNodeAnalyzer::isDead()`.
fn is_dead(tag: &TypeTag, return_type: &Hint<'_>, scope: &TypeScope<'_, '_, '_, '_>) -> bool {
    if tag.has_description || matches!(tag.ty, DocNode::Generic { .. }) {
        return false;
    }
    let doc_type = scope.map_doc(&tag.ty);
    if !is_legal_without_template(&doc_type) {
        return false;
    }
    if scope.in_trait() && matches!(tag.ty, DocNode::This) {
        return false;
    }
    let native = scope.map_hint(return_type);
    if !are_native_and_doc_types_equal(&native, &doc_type, scope.class_info()) {
        return is_dead_not_equal(tag, return_type, &native, &doc_type);
    }
    if is_allowed(&tag.ty) {
        return false;
    }
    let DocNode::Union(members) = &tag.ty else {
        return is_standalone_legal(&tag.ty, is_bool_identifier(return_type));
    };
    if has_generic(members) || has_mixed_array(members) {
        return false;
    }
    !members
        .iter()
        .any(|m| matches!(m, DocNode::Ident(n) if n.eq_ignore_ascii_case("true") || n.eq_ignore_ascii_case("false")))
}

fn is_dead_not_equal(
    tag: &TypeTag,
    return_type: &Hint<'_>,
    native: &super::types::compare::RType,
    doc_type: &super::types::compare::RType,
) -> bool {
    if matches!(&tag.ty, DocNode::Ident(n) if n == "void") {
        return true;
    }
    if !has_useful_phpdoc_type(tag, return_type) {
        return true;
    }
    doc_type.is_union() && are_types_equal(&remove_null(doc_type), native)
}

/// Whether the `@return` says something next to the native type: not `mixed`, and over a native `void` only
/// `never`; nothing over a native `never`.
fn has_useful_phpdoc_type(tag: &TypeTag, return_type: &Hint<'_>) -> bool {
    if matches!(&tag.ty, DocNode::Ident(n) if n == "mixed") {
        return false;
    }
    let is_never = matches!(return_type, Hint::Never(_));
    if !matches!(return_type, Hint::Void(_)) {
        return !is_never;
    }
    matches!(&tag.ty, DocNode::Ident(n) if n == "never")
}
