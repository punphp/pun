//! `lint/phpunit/add-property-type-setup`, ported from Rector 2.8.4's `TypedPropertyFromStrictSetUpRector`
//! (rules/TypeDeclaration/Rector/Property) with `TrustedClassMethodPropertyTypeInferer`: in a class with a `setUp()`
//! method, an untyped private property gets the type of what `setUp()` assigns it, as
//! `lint/types/add-property-type-constructor` infers it from the constructor.
//!
//! A property of several items gets the type of its first one. When the type is `MockObject` and the `@var` names
//! another class, the `@var` becomes `\PHPUnit\Framework\MockObject\MockObject&\That\Class`.
//!
//! Unlike Rector, Pun leaves a property alone whose default is `null` when the type does not allow `null`: Rector
//! writes `private int $a = null;`, which PHP rejects.

use mago_syntax::cst::{ClassLikeMember, Node, NodeKind, Property};
use pun_problem::{Edit, Safety, Span};

use super::add_property_type_constructor::{infer, is_test_case};
use super::doctrine_repository::{doc_class, object_name};
use super::facts::facts;
use super::property_type::{
    ClassView, Prop, TYPED_PROPERTIES, Ty, VarTag, is_null_literal, property_declaration,
    variable_span,
};
use crate::{Context, Report, Rule};

pub struct AddPropertyTypeSetup;

/// `ClassName::MOCK_OBJECT`.
pub(crate) const MOCK_OBJECT: &str = "PHPUnit\\Framework\\MockObject\\MockObject";

impl Rule for AddPropertyTypeSetup {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-property-type-setup"
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
        let Some(set_up) = view.method(b"setUp") else {
            return;
        };
        let test_case = is_test_case(&facts, &view);
        let mut edits: Vec<Edit> = Vec::new();
        for prop in untyped_properties(&view) {
            if !prop.is_private() {
                continue;
            }
            // `getName($property)` is the name of the first item
            let prop = Prop {
                property: prop.property,
                item: match prop.property {
                    Property::Plain(p) => p.items.iter().next(),
                    Property::Hooked(h) => Some(&h.item),
                },
            };
            let t = infer(&facts, &view, &prop, set_up, ctx, test_case);
            let Some(declaration) = property_declaration(&t, &facts.codebase, ctx.php_version)
            else {
                continue;
            };
            if prop.default().is_some_and(is_null_literal) && !t.members().contains(&&Ty::Null) {
                continue;
            }
            if let Ty::Object(name) = &t
                && name.eq_ignore_ascii_case(MOCK_OBJECT)
            {
                let var = VarTag::of(ctx, prop.property, &view.path);
                if let Ok(Some(tag)) = &var.tag
                    && let Some(written) = object_name(&tag.ty)
                {
                    let class = doc_class(&view.path, written);
                    if !class.eq_ignore_ascii_case(MOCK_OBJECT)
                        && let Some(e) =
                            var.set(ctx.source, &format!("\\{MOCK_OBJECT}&\\{class}"))
                    {
                        edits.extend(e);
                    }
                }
            }
            let at = variable_span(&prop).start;
            edits.push(Edit::new(Span::new(at, at), format!("{declaration} ")));
        }
        let Some(first) = edits.first() else { return };
        out.push(Report::fixed(
            first.span,
            "Type the properties `setUp()` assigns.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// The class's untyped properties in order (`getProperties()` with no `$type`): [`ClassView::properties`] and the
/// hooked ones, which Rector types too.
pub(crate) fn untyped_properties<'ast, 'arena>(
    view: &ClassView<'ast, 'arena>,
) -> Vec<Prop<'ast, 'arena>> {
    let mut plain = view.properties().into_iter();
    view.members
        .iter()
        .filter_map(|m| match m {
            ClassLikeMember::Property(p @ Property::Hooked(h)) if h.hint.is_none() => Some(Prop {
                property: p,
                item: Some(&h.item),
            }),
            ClassLikeMember::Property(Property::Plain(p)) if p.hint.is_none() => plain.next(),
            _ => None,
        })
        .collect()
}
