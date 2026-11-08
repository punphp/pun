//! `lint/phpunit/add-property-type-create-mock`, ported from Rector 2.8.4's `TypedPropertyFromCreateMockAssignRector`
//! (rules/TypeDeclaration/Rector/Class_): in a PHPUnit test case, a property with one item whose assigned types
//! (`AssignToPropertyTypeInferer`) declare as `MockObject`, or a class or interface that extends it, gets that
//! declaration, replacing any other type it has. A property already typed `MockObject` (or a subtype) or with an
//! intersection type is left alone.
//!
//! When the constructor (or `setUp()`) does not always assign the property, only a nullable declaration is written,
//! and the property's default becomes `null`.

use mago_syntax::cst::{ClassLikeMember, Hint, Node, NodeKind, Property, PropertyItem};
use pun_problem::{Edit, Safety, Span};

use super::add_property_type_constructor::is_test_case;
use super::add_property_type_setup::MOCK_OBJECT;
use super::facts::{Facts, facts};
use super::property_type::{
    Assigned, ClassView, Prop, TYPED_PROPERTIES, Ty, assigned_type, default_span, hint_ty,
    property_declaration, variable_span,
};
use crate::rules::no_redundant_property_default::is_property_assigned;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddPropertyTypeCreateMock;

impl Rule for AddPropertyTypeCreateMock {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-property-type-create-mock"
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
        if !is_test_case(&facts, &view) {
            return;
        }
        let mut edits: Vec<Edit> = Vec::new();
        for member in view.members.iter() {
            let ClassLikeMember::Property(property) = member else {
                continue;
            };
            let (hint, item): (Option<&Hint<'_>>, &PropertyItem<'_>) = match property {
                Property::Plain(p) => {
                    let items: Vec<&PropertyItem<'_>> = p.items.iter().collect();
                    let [item] = items[..] else { continue };
                    (p.hint.as_ref(), item)
                }
                Property::Hooked(p) => (p.hint.as_ref(), &p.item),
            };
            // isAlreadyTypedWithMockObject
            if let Some(hint) = hint {
                if matches!(hint, Hint::Intersection(_))
                    || is_mock_object(&facts, &hint_ty(hint, &view.path, &view))
                {
                    continue;
                }
            }
            let prop = Prop {
                property,
                item: Some(item),
            };
            let name = prop.name();
            let assigned = is_property_assigned(ctx, &view, true, name);
            let Assigned::Type(t) = assigned_type(&facts, &view, &prop, assigned) else {
                continue;
            };
            let Some(declaration) = property_declaration(&t, &facts.codebase, ctx.php_version)
            else {
                continue;
            };
            if !is_mock_object(&facts, &t) {
                continue;
            }
            if !assigned {
                if !declaration.starts_with('?') {
                    continue;
                }
                match default_span(&prop) {
                    Some(span) => edits.push(Edit::new(span, " = null")),
                    None => {
                        let at = variable_span(&prop).end;
                        edits.push(Edit::new(Span::new(at, at), " = null"));
                    }
                }
            }
            match hint {
                Some(hint) => edits.push(Edit::new(node_span(hint), declaration)),
                None => {
                    let at = variable_span(&prop).start;
                    edits.push(Edit::new(Span::new(at, at), format!("{declaration} ")));
                }
            }
        }
        let Some(first) = edits.first() else { return };
        out.push(Report::fixed(
            first.span,
            "Type the properties assigned a mock as `MockObject`.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// `isObjectType($type, new ObjectType(MockObject))`: without `null`, the type is `MockObject` or a class-like that
/// extends it (Rector's `isObjectType()` of an intersection declaration is false).
fn is_mock_object(facts: &Facts, t: &Ty) -> bool {
    let is_mock = |name: &str| {
        name.eq_ignore_ascii_case(MOCK_OBJECT)
            || facts
                .codebase
                .get_class_like(name.as_bytes())
                .is_some_and(|c| {
                    c.all_parent_interfaces
                        .iter()
                        .chain(c.all_parent_classes.iter())
                        .any(|p| p.as_bytes().eq_ignore_ascii_case(MOCK_OBJECT.as_bytes()))
                })
    };
    let members: Vec<&Ty> = t
        .members()
        .into_iter()
        .filter(|m| **m != Ty::Null)
        .collect();
    !members.is_empty()
        && members.iter().all(|m| match m {
            Ty::Object(name) => is_mock(name),
            _ => false,
        })
}
