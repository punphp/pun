//! `lint/phpdoc/no-useless-param`, ported from Rector 2.8.4's `RemoveUselessParamTagRector`
//! (rules/DeadCode/Rector/ClassMethod) with `ParamTagRemover` and `DeadParamTagValueNodeAnalyzer`
//! (rules/DeadCode/PhpDoc).
//!
//! Each `@param` of a function's or method's doc comment goes when its parameter's native type already says what it
//! says (`TypeComparator::arePhpParserAndPhpStanPhpDocTypesEqual()`). It stays with a description, a generic or other
//! type the native one cannot say, a template type, `true` or `false` over a native `bool`, or a union with a
//! `mixed[]` or generic member. Only the tags named exactly `@param` count; a doc comment left empty goes.

use mago_syntax::cst::{
    FunctionLikeParameterList, FunctionLikeReturnTypeHint, Hint, Node, NodeKind,
};
use pun_phpstan::phpdoc::Node as DocNode;
use pun_problem::{Fix, Safety};

use super::lint_b_doc::Doc;
use super::types::compare::classes::with_classes;
use super::types::compare::mapper::TypeScope;
use super::types::compare::tags::{
    Kind, TypeTag, has_generic, has_mixed_array, is_allowed, is_standalone_legal, tag_name,
    type_tag,
};
use super::types::compare::{are_native_and_doc_types_equal, is_legal_without_template};
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUselessParamTag;

impl Rule for NoUselessParamTag {
    fn id(&self) -> &'static str {
        "lint/phpdoc/no-useless-param"
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
        let Some((params, _)) = function_like(&node) else {
            return;
        };
        let start = node_span(&node).start;
        let Some(doc) = Doc::last_of(ctx, ancestors, start) else {
            return;
        };
        let facts = facts(ctx);
        let mut chain = ancestors.to_vec();
        chain.push(node);
        let scope = TypeScope::new(&chain, facts.documented());
        let removed: Vec<bool> = with_classes(facts.documented(), || {
            (0..doc.len())
                .map(|i| {
                    // handle only basic types, keep phpstan/psalm helper ones
                    tag_name(doc.text(i)) == "@param"
                        && type_tag(&doc, i).is_some_and(|tag| {
                            tag.kind == Kind::Param && is_dead(&tag, params, &scope)
                        })
                })
                .collect()
        });
        if !removed.contains(&true) {
            return;
        }
        out.push(Report {
            span: doc.span,
            message: "Remove the `@param` tag, which says what the native type says.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![doc.remove_last(ctx.source, &removed)],
            }),
        });
    }
}

/// The parameters and the return type of a function or method.
pub(crate) fn function_like<'n, 'arena>(
    node: &Node<'n, 'arena>,
) -> Option<(
    &'n FunctionLikeParameterList<'arena>,
    Option<&'n FunctionLikeReturnTypeHint<'arena>>,
)> {
    match node {
        Node::Method(m) => Some((&m.parameter_list, m.return_type_hint.as_ref())),
        Node::Function(f) => Some((&f.parameter_list, f.return_type_hint.as_ref())),
        _ => None,
    }
}

/// Whether a native type is the identifier `bool` (`$node->toString() === 'bool'`; php-parser lowercases built-in types).
pub(crate) fn is_bool_identifier(hint: &Hint<'_>) -> bool {
    matches!(hint, Hint::Bool(_))
}

/// `DeadParamTagValueNodeAnalyzer::isDead()`.
fn is_dead(
    tag: &TypeTag,
    params: &FunctionLikeParameterList<'_>,
    scope: &TypeScope<'_, '_, '_, '_>,
) -> bool {
    let Some(param) = params
        .parameters
        .iter()
        .find(|p| p.variable.name == tag.variable.as_bytes())
    else {
        return false;
    };
    let Some(hint) = &param.hint else {
        return false;
    };
    if tag.has_description || matches!(tag.ty, DocNode::Generic { .. }) {
        return false;
    }
    let doc_type = scope.map_doc(&tag.ty);
    if !is_legal_without_template(&doc_type) {
        return false;
    }
    if !are_native_and_doc_types_equal(&scope.map_hint(hint), &doc_type, scope.class_info()) {
        return false;
    }
    if is_allowed(&tag.ty) {
        return false;
    }
    match &tag.ty {
        DocNode::Union(members) => !has_mixed_array(members) && !has_generic(members),
        ty => is_standalone_legal(ty, is_bool_identifier(hint)),
    }
}
