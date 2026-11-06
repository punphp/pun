//! `lint/phpunit/add-property-type-setup-docblock`, ported from Rector 2.6.1's
//! `TypedPropertyFromDocblockSetUpDefinedRector` (rules/TypeDeclaration/Rector/Class_): in a PHPUnit test case with a
//! `setUp()` method, an untyped private non-static property with one item that the constructor or `setUp()` always
//! assigns (`ConstructorAssignDetector`) gets the declaration of its `@var` type. The `@var` goes when it says no more
//! than the new declaration (`DeadVarTagValueNodeAnalyzer::isDead`).

use mago_syntax::cst::{Modifier, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::add_property_type_constructor::is_test_case;
use super::add_property_type_setup::untyped_properties;
use super::compare::classes::with_classes;
use super::compare::mapper::TypeScope;
use super::compare::tags::{TypeTag, first_named};
use super::facts::facts;
use super::property_type::{
    ClassView, TYPED_PROPERTIES, Ty, VarTag, doc_type, property_declaration, ty, variable_span,
};
use crate::rules::lint_b_doc::Doc;
use crate::rules::no_redundant_property_default::is_property_assigned;
use crate::rules::no_useless_var_tag::{Target, is_dead};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};
use pun_phpstan::phpdoc::Node as PhpDocNode;

pub struct AddPropertyTypeSetupDocblock;

impl Rule for AddPropertyTypeSetupDocblock {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-property-type-setup-docblock"
    }

    fn min_php_version(&self) -> u32 {
        TYPED_PROPERTIES
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
        let facts = facts(ctx);
        let view = ClassView::of(&facts, node, ancestors).expect("the rule targets classes");
        if !is_test_case(&facts, &view) || view.method(b"setUp").is_none() {
            return;
        }
        let mut edits: Vec<Edit> = Vec::new();
        for prop in &untyped_properties(&view) {
            if prop.item.is_none()
                || !prop.is_private()
                || prop
                    .modifiers()
                    .iter()
                    .any(|m| matches!(m, Modifier::Static(_)))
                || !is_property_assigned(ctx, &view, true, prop.name())
            {
                continue;
            }
            // getVarType(): phpdoc-parser's reading of the first `@var` (which takes `$this`, unlike `VarTag`), and
            // the type PHPStan reads there
            let Some(((_, tag), doc)) =
                Doc::last_of(ctx, &view.path, node_span(prop.property).start)
                    .and_then(|doc| first_named(&doc, "@var"))
                    .zip(doc_type(&facts, &view, prop.name()))
            else {
                continue;
            };
            // `self` in the tag is `SelfObjectType`, which declares as `self` (Mago reads the class)
            let says_self = names_self(&tag.ty);
            let own = |m: Ty| match m {
                Ty::Object(n) if says_self && n.eq_ignore_ascii_case(&view.name) => Ty::Static(n),
                other => other,
            };
            let t = match ty(&doc) {
                Ty::Union(members) => Ty::Union(members.into_iter().map(own).collect()),
                other => own(other),
            };
            if matches!(t, Ty::Mixed) {
                continue;
            }
            let Some(declaration) = property_declaration(&t, &facts.codebase, ctx.php_version)
            else {
                continue;
            };
            let at = variable_span(prop).start;
            edits.push(Edit::new(Span::new(at, at), format!("{declaration} ")));
            if is_dead_after(&facts, &view, &tag, &declaration)
                && let Some(e) =
                    VarTag::of(ctx, prop.property, &view.path).remove(ctx.source)
            {
                edits.extend(e);
            }
        }
        let Some(first) = edits.first() else { return };
        out.push(Report::fixed(
            first.span,
            "Type the properties `setUp()` assigns from their `@var`.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// `DeadVarTagValueNodeAnalyzer::isDead()` of the property's `@var` once it is declared `declaration`.
fn is_dead_after(
    facts: &super::facts::Facts,
    view: &ClassView<'_, '_>,
    tag: &TypeTag,
    declaration: &str,
) -> bool {
    // Rector keeps the `@var` (`self`, `static`, `$this`, `?self`, `self|null`) of a property it declares `self` or
    // `?self`: its `SelfObjectType` never equals the doc's type
    if declaration.trim_start_matches('?') == "self" {
        return false;
    }
    // the declaration Pun writes reads the same as a PHPDoc type
    let (node, _) = pun_phpstan::phpdoc::parse_prefix(declaration)
        .expect("a type declaration parses as a PHPDoc type");
    let scope = TypeScope::new(&view.path, facts.documented());
    with_classes(facts.documented(), || {
        let native = scope.map_doc(&node);
        is_dead(tag, &Target::Mapped(native), &scope, facts)
    })
}

/// Whether a PHPDoc type names `self`, alone or in a union.
fn names_self(node: &PhpDocNode) -> bool {
    match node {
        PhpDocNode::Ident(n) => n.eq_ignore_ascii_case("self"),
        PhpDocNode::Nullable(n) => names_self(n),
        PhpDocNode::Union(m) => m.iter().any(names_self),
        _ => false,
    }
}
