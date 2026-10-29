//! `lint/phpunit/add-property-type-setup`, ported from Rector 2.6.1's `TypedPropertyFromStrictSetUpRector`
//! (rules/TypeDeclaration/Rector/Property) with `TrustedClassMethodPropertyTypeInferer`: in a class with a `setUp()`
//! method, an untyped private property gets the type of what `setUp()` assigns it, as
//! `lint/types/add-property-type-constructor` infers it from the constructor.
//!
//! A property of several items gets the type of its first one. When the type is `MockObject` and the `@var` names
//! another class, the `@var` becomes `(\PHPUnit\Framework\MockObject\MockObject & \That\Class)`.
//!
//! Unlike Rector, Pun leaves a property alone whose default is `null` when the type does not allow `null`: Rector
//! writes `private int $a = null;`, which PHP rejects.

use mago_syntax::cst::{Node, NodeKind, Property, PropertyItem};
use pun_problem::{Edit, Fix, Safety, Span};

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
        let Some(view) = ClassView::of(&facts, node, ancestors) else {
            return;
        };
        let Some(set_up) = view.method(b"setUp") else {
            return;
        };
        let test_case = is_test_case(&facts, &view);
        let mut edits: Vec<Edit> = Vec::new();
        for prop in view.properties() {
            if !prop.is_private() {
                continue;
            }
            // `getName($property)` is the name of the first item
            let prop = match prop.item {
                Some(_) => prop,
                None => Prop {
                    property: prop.property,
                    item: first_item(&prop),
                },
            };
            let t = infer(&facts, &view, &prop, set_up, ctx, ancestors, test_case);
            let Some(declaration) = property_declaration(&t, &facts.codebase, ctx.php_version)
            else {
                continue;
            };
            if prop.default().is_some_and(is_null_literal) && !t.members().contains(&&Ty::Null) {
                continue;
            }
            if let Ty::Object(name) = &t
                && name.eq_ignore_ascii_case(MOCK_OBJECT)
                && let Some(var) = VarTag::of(ctx.source, Node::Property(prop.property), &view.path)
                && let Ok(Some(tag)) = &var.tag
                && let Some(written) = object_name(&tag.ty)
            {
                let class = doc_class(&view.path, written);
                if !class.eq_ignore_ascii_case(MOCK_OBJECT)
                    && let Some(e) = var.set(ctx.source, &format!("(\\{MOCK_OBJECT} & \\{class})"))
                {
                    edits.extend(e);
                }
            }
            let at = variable_span(&prop).start;
            edits.push(Edit::new(Span::new(at, at), format!("{declaration} ")));
        }
        let Some(first) = edits.first() else { return };
        out.push(Report {
            span: first.span,
            message: "Type the properties `setUp()` assigns.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}

/// The first item of a plain property with several.
fn first_item<'ast, 'arena>(prop: &Prop<'ast, 'arena>) -> Option<&'ast PropertyItem<'arena>> {
    let Property::Plain(p) = prop.property else {
        return None;
    };
    p.items.iter().next()
}
