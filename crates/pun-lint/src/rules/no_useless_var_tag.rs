//! `lint/phpdoc/no-useless-var`, ported from Rector 2.6.1's `RemoveUselessVarTagRector`
//! (rules/DeadCode/Rector/Property) with `VarTagRemover::removeVarTagIfUseless()` and
//! `DeadVarTagValueNodeAnalyzer` (rules/DeadCode/PhpDoc).
//!
//! The first `@var` of a property's, a class constant's or an expression statement's doc comment goes when the native
//! type already says it: the property's or constant's declared type, or, for `$x = expr;`, the native type of `expr`
//! when the tag says exactly that. It stays with a description, a generic or other type the native one cannot say
//! (`PhpDocTypeChanger::isAllowed()`), or a template type. The removal takes every `@var` (and `@phan-var`) of the
//! same variable along, not the `@phpstan-` and `@psalm-` ones; a doc comment left empty goes.

use mago_syntax::cst::{ClassLikeMember, Expression, Hint, Node, NodeKind, Statement};
use pun_problem::{Fix, Safety};

use super::lint_b_doc::Doc;
use super::types::compare::classes::with_classes;
use super::types::compare::mapper::{TypeScope, expr_type};
use super::types::compare::tags::{Kind, TypeTag, first_named, is_allowed, removed_by_type};
use super::types::compare::{
    RType, are_native_and_doc_types_equal, are_types_equal, equals, is_legal_without_template,
    is_super, remove_null,
};
use super::types::facts::{Facts, facts};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUselessVarTag;

impl Rule for NoUselessVarTag {
    fn id(&self) -> &'static str {
        "lint/phpdoc/no-useless-var"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ClassLikeMember, NodeKind::Statement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let target = match node {
            Node::ClassLikeMember(ClassLikeMember::Property(p)) => Target::Hint(p.hint()),
            Node::ClassLikeMember(ClassLikeMember::Constant(c)) => Target::Hint(c.hint.as_ref()),
            Node::Statement(Statement::Expression(e)) => match e.expression {
                Expression::Assignment(a) if a.operator.is_assign() => Target::Expr(a.rhs),
                _ => Target::Other,
            },
            _ => return,
        };
        let start = node_span(&node).start;
        let Some(doc) = Doc::last_of(ctx.source, ancestors, start) else {
            return;
        };
        let Some((_, tag)) = first_named(&doc, "@var") else {
            return;
        };
        let facts = facts(ctx);
        let scope = TypeScope::new(ancestors, facts.documented());
        let dead = with_classes(facts.documented(), || {
            is_dead(&tag, &target, &scope, &facts)
        });
        if !dead || is_allowed(&tag.ty) {
            return;
        }
        let removed = removed_by_type(&doc, Kind::Var, &tag.variable);
        out.push(Report {
            span: doc.span,
            message: "Remove the `@var` tag, which says what the native type says.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![doc.remove_last(ctx.source, &removed)],
            }),
        });
    }
}

/// What the tag is compared with: a declared type (`Property::$type`, `ClassConst::$type`), the assigned value of an
/// expression statement, or nothing to compare with.
pub(crate) enum Target<'n, 'arena> {
    Hint(Option<&'n Hint<'arena>>),
    Expr(&'n Expression<'arena>),
    Other,
}

/// The span of an expression, for the facts' types.
/// `DeadVarTagValueNodeAnalyzer::isDead()`.
fn is_dead(
    tag: &TypeTag,
    target: &Target<'_, '_>,
    scope: &TypeScope<'_, '_, '_, '_>,
    facts: &Facts,
) -> bool {
    if matches!(target, Target::Hint(None)) || tag.has_description {
        return false;
    }
    // allow Identifier, ComplexType, and Name on Property and ClassConst
    let property_type = match target {
        Target::Hint(Some(hint)) => scope.map_hint(hint),
        // `ExprNodeMapper`: the scope's type of the expression
        Target::Expr(e) => RType::T(expr_type(facts, scope.ancestors(), e, false)),
        _ => return false,
    };
    if matches!(tag.ty, pun_phpstan::phpdoc::Node::Generic { .. }) {
        return false;
    }
    // is strict type superior to doc type? keep strict type only
    let doc_type = scope.map_doc(&tag.ty);
    if let Target::Expr(e) = target {
        let native = RType::T(expr_type(facts, scope.ancestors(), e, true));
        if !equals(&doc_type, &native) {
            return false;
        }
    }
    if !is_legal_without_template(&doc_type) {
        return false;
    }
    if property_type.is_union() && !doc_type.is_union() {
        return !doc_type.is_intersection();
    }
    if property_type.is_object_type() && doc_type.is_object_type() {
        if equals(&doc_type, &property_type) {
            return true;
        }
        // more specific type is already in the property
        return is_super(&doc_type, &property_type).yes();
    }
    if are_native_and_doc_types_equal(&property_type, &doc_type, scope.class_info()) {
        return true;
    }
    doc_type.is_union() && are_types_equal(&remove_null(&doc_type), &property_type)
}

/// `VarTagRemover::removeVarPhpTagValueNodeIfNotComment()`: the children of `doc` (the doc comment of a node at
/// `ancestors`) to remove so that its `@var` goes, given the type `t` the node has; `None` when it stays (a
/// description, a type the native one cannot say, or a narrower type than `t`).
pub(crate) fn var_removal_if_not_comment(
    doc: &Doc<'_>,
    t: &RType,
    scope: &TypeScope<'_, '_, '_, '_>,
) -> Option<Vec<bool>> {
    let (_, tag) = first_named(doc, "@var")?;
    // has description? keep it; keep string[] etc.
    if tag.has_description || is_allowed(&tag.ty) {
        return None;
    }
    // keep subtypes like positive-int
    let var_type = scope.map_doc(&tag.ty);
    if !are_types_equal(t, &var_type) && super::types::compare::is_subtype(&var_type, t) {
        return None;
    }
    Some(removed_by_type(doc, Kind::Var, ""))
}
