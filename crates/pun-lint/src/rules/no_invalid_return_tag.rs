//! `lint/dead-code/no-invalid-return-tag`, ported from Rector 2.8.4's `RemoveReturnTagIncompatibleWithNativeTypeRector`
//! (rules/DeadCode/Rector/ClassMethod).
//!
//! The first `@return` of a function's or method's doc comment goes when its type contradicts the native return type
//! (`$native->isSuperTypeOf($doc)` is no); a narrower type stays. It also stays with a description, a generic type, a
//! type alias of the class anywhere in it (`@phpstan-type`, `@psalm-type`, `@phan-type` or an imported one), a template
//! of the method, or `$this` over a class type in a trait. The removal takes `@phan-return` along, not the `@phpstan-`
//! and `@psalm-` tags; a doc comment left empty goes.

use mago_syntax::cst::{Node, NodeKind};
use pun_phpstan::phpdoc::{Node as DocNode, TagValue, tags};
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

pub struct NoInvalidReturnTag;

impl Rule for NoInvalidReturnTag {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-invalid-return-tag"
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
        // no native return type to compare against
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
        // keep any documented explanation, and generic return types, which carry template info the native type
        // cannot express
        if tag.has_description || matches!(tag.ty, DocNode::Generic { .. }) {
            return;
        }
        if let DocNode::Ident(name) = &tag.ty
            && template_names(doc.text_all()).contains(name)
        {
            return;
        }
        // the alias can be nested in a composed type as well, e.g. "ConfigArray|CustomConfig" or "?ConfigArray"
        let aliases = class_type_aliases(ctx, ancestors);
        if !aliases.is_empty() && contains_name(&tag.ty, &aliases) {
            return;
        }
        let facts = facts(ctx);
        let mut chain = ancestors.to_vec();
        chain.push(node);
        let scope = TypeScope::new(&chain, facts.documented());
        let contradicts = with_classes(facts.documented(), || {
            let native = scope.map_hint(&return_type.hint);
            if scope.in_trait() && matches!(tag.ty, DocNode::This) && native.is_object_type() {
                return false;
            }
            // a subtype/narrowing is legitimate; only a contradiction is dead
            is_super(&native, &scope.map_doc(&tag.ty)).no()
        });
        if !contradicts {
            return;
        }
        let removed = removed_by_type(&doc, Kind::Return, "");
        out.push(Report {
            span: doc.span,
            message: "Remove the `@return` tag, which contradicts the native return type."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![doc.remove_last(ctx.source, &removed)],
            }),
        });
    }
}

/// The names of the `@template` tags of a doc comment (`PhpDocInfo::getTemplateNames()`).
pub(crate) fn template_names(doc: &str) -> Vec<String> {
    (tags(doc).unwrap_or_default().into_iter())
        .filter_map(|t| match t.value {
            TagValue::Template(tpl) if t.name == "@template" => Some(tpl.name),
            _ => None,
        })
        .collect()
}

/// `containsTypeAliasName()`: an identifier anywhere in the type node is one of `names`.
fn contains_name(node: &DocNode, names: &[String]) -> bool {
    let any = |nodes: &[DocNode]| nodes.iter().any(|n| contains_name(n, names));
    match node {
        DocNode::Ident(name) => names.contains(name),
        DocNode::Nullable(inner) | DocNode::Array(inner) | DocNode::Projection(_, inner) => {
            contains_name(inner, names)
        }
        DocNode::Union(nodes) | DocNode::Inter(nodes) => any(nodes),
        DocNode::Generic { args, .. } => any(args),
        DocNode::OffsetAccess(a, b) => contains_name(a, names) || contains_name(b, names),
        DocNode::Conditional {
            subject,
            target,
            then,
            otherwise,
            ..
        } => [subject, target, then, otherwise]
            .iter()
            .any(|n| contains_name(n, names)),
        DocNode::ConditionalForParameter {
            target,
            then,
            otherwise,
            ..
        } => [target, then, otherwise]
            .iter()
            .any(|n| contains_name(n, names)),
        _ => false,
    }
}

/// The type aliases the doc comment of the class-like around the node defines or imports
/// (`ResolvedPhpDocBlock::getTypeAliasTags()` and `getTypeAliasImportTags()`).
fn class_type_aliases(ctx: &crate::Context<'_>, ancestors: &[Node<'_, '_>]) -> Vec<String> {
    let Some(i) = ancestors.iter().rposition(|n| {
        matches!(
            n,
            Node::Class(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
                | Node::AnonymousClass(_)
        )
    }) else {
        return Vec::new();
    };
    let Some(doc) = Doc::last_of(ctx, &ancestors[..i], node_span(&ancestors[i]).start) else {
        return Vec::new();
    };
    (tags(doc.text_all()).unwrap_or_default().into_iter())
        .filter_map(|t| match t.value {
            TagValue::TypeAlias(name, _)
                if matches!(
                    t.name.as_str(),
                    "@phan-type" | "@psalm-type" | "@phpstan-type"
                ) =>
            {
                Some(name)
            }
            TagValue::TypeAliasImport(name, _, alias)
                if matches!(
                    t.name.as_str(),
                    "@psalm-import-type" | "@phpstan-import-type"
                ) =>
            {
                Some(alias.unwrap_or(name))
            }
            _ => None,
        })
        .collect()
}
