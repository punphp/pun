//! `lint/types/add-property-type-assigns`, ported from Rector 2.6.1's `TypedPropertyFromAssignsRector`
//! (rules/TypeDeclaration/Rector/Property) with `AllAssignNodePropertyTypeInferer` and `PropertyTypeDecorator`: an
//! untyped private property gets the union of the native types of every value the class assigns to it.
//!
//! The values are the class's assignments to the property or to an offset of it (`AssignToPropertyTypeInferer`);
//! `null` joins them unless the constructor always assigns the property or it has a default, and a `null` default adds
//! `null` too. A property the class writes dynamically, one assigned a parameter without a native type, and one whose
//! default the assigned types do not allow are left alone. A nullable union gets `= null` when it has no default. The
//! `@var` goes when it says no more than the new type.
//!
//! The option `inline-public` (Rector's `inline_public`) types protected and public properties too. Properties a used
//! trait declares or writes, whose declared type is `null`, `callable` or a class that does not exist, protected ones
//! of a class that is not final or whose parent declares them, and those Doctrine maps (`Column`, `OneToMany`,
//! `ManyToMany`, `ManyToOne`, `OneToOne`, and ODM's `ReferenceMany`, `EmbedMany`, `ReferenceOne`, `EmbedOne`, as an
//! attribute or an annotation) are left alone, as is a class with a Doctrine `loadMetadata()` method.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::add_property_type_constructor::is_test_case;
use super::facts::facts;
use super::property_type::{
    Assigned, ClassView, Prop, TYPED_PROPERTIES, Ty, VarTag, assigned_type, declared_ty, doc_type,
    is_legal, is_null_literal, is_var_tag_useless, mixed_passed_or_union, property_declaration,
    variable_span,
};
use crate::rules::mago_helpers::{boolean, configure};
use crate::rules::no_redundant_property_default::is_property_assigned;
use crate::rules::phpunit::annotations::Annotated;
use crate::rules::rector_batch5::{has_attribute, imported};
use crate::{Context, Report, Rule};

#[derive(Default)]
pub struct AddPropertyTypeAssigns {
    inline_public: bool,
}

/// `CollectionMapping::TO_MANY_CLASSES`, `TO_ONE_CLASSES` and `MappingClass::COLUMN`.
const DOCTRINE_MAPPING: [&str; 9] = [
    "Doctrine\\ORM\\Mapping\\OneToMany",
    "Doctrine\\ORM\\Mapping\\ManyToMany",
    "Doctrine\\ODM\\MongoDB\\Mapping\\Annotations\\ReferenceMany",
    "Doctrine\\ODM\\MongoDB\\Mapping\\Annotations\\EmbedMany",
    "Doctrine\\ORM\\Mapping\\ManyToOne",
    "Doctrine\\ORM\\Mapping\\OneToOne",
    "Doctrine\\ODM\\MongoDB\\Mapping\\Annotations\\ReferenceOne",
    "Doctrine\\ODM\\MongoDB\\Mapping\\Annotations\\EmbedOne",
    "Doctrine\\ORM\\Mapping\\Column",
];

impl Rule for AddPropertyTypeAssigns {
    fn id(&self) -> &'static str {
        "lint/types/add-property-type-assigns"
    }

    fn min_php_version(&self) -> u32 {
        TYPED_PROPERTIES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "inline-public" => self.inline_public = boolean(value)?,
                _ => return None,
            }
            Some(())
        })
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
        if view.method(b"loadMetadata").is_some() {
            return;
        }
        let test_case = is_test_case(&facts, &view);
        let mut edits: Vec<Edit> = Vec::new();
        for prop in &view.properties() {
            if !prop.is_private() && !self.inline_public {
                continue;
            }
            if is_doctrine_mapped(ctx.source, prop, &view.path) {
                continue;
            }
            if !is_legal(&facts, &view, prop, self.inline_public) {
                continue;
            }
            let assigned = is_property_assigned(ctx, &view, test_case, prop.name());
            let Assigned::Type(t) = assigned_type(&facts, &view, prop, assigned) else {
                continue;
            };
            if matches!(t, Ty::Mixed) {
                continue;
            }
            // decorateTypeWithNullableIfDefaultPropertyNull
            let t = match prop.default() {
                Some(default) if is_null_literal(default) && !t.members().contains(&&Ty::Null) => {
                    mixed_passed_or_union(vec![t, Ty::Null], true)
                }
                _ => t,
            };
            let Some(declaration) = property_declaration(&t, &facts.codebase, ctx.php_version)
            else {
                continue;
            };
            let Some(var) = VarTag::of(ctx.source, Node::Property(prop.property), &view.path)
            else {
                continue;
            };
            let at = variable_span(prop);
            edits.push(Edit::new(
                Span::new(at.start, at.start),
                format!("{declaration} "),
            ));
            // PropertyTypeDecorator::decoratePropertyUnionType: a nullable union gets a `null` default
            if t.is_union() && t.members().contains(&&Ty::Null) && prop.default().is_none() {
                edits.push(Edit::new(Span::new(at.end, at.end), " = null"));
            }
            if let Ok(Some(tag)) = &var.tag
                && is_var_tag_useless(
                    tag,
                    doc_type(&facts, &view, prop.name()).as_ref(),
                    &declared_ty(&t),
                )
                && let Some(e) = var.remove(ctx.source)
            {
                edits.extend(e);
            }
        }
        let Some(first) = edits.first() else { return };
        out.push(Report {
            span: first.span,
            message: "Type the properties from what the class assigns to them.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}

/// `isDoctrineMappedProperty` (`AttrinationFinder::hasByMany`): a Doctrine mapping attribute, or a doc annotation
/// naming one.
fn is_doctrine_mapped(source: &str, prop: &Prop<'_, '_>, path: &[Node<'_, '_>]) -> bool {
    let lists = match prop.property {
        mago_syntax::cst::Property::Plain(p) => &p.attribute_lists,
        mago_syntax::cst::Property::Hooked(p) => &p.attribute_lists,
    };
    if DOCTRINE_MAPPING
        .iter()
        .any(|c| has_attribute(path, lists, c))
    {
        return true;
    }
    let Some(doc) = Annotated::of(source, Node::Property(prop.property), path).and_then(|a| a.doc)
    else {
        return false;
    };
    doc.text_all().split('@').skip(1).any(|tag| {
        let len = tag
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '\\'))
            .unwrap_or(tag.len());
        let written = &tag[..len];
        let resolved = match written.strip_prefix('\\') {
            Some(fq) => fq.to_string(),
            None => imported(path, written).unwrap_or_else(|| match super::namespace_of(path) {
                Some(ns) => format!("{ns}\\{written}"),
                None => written.to_string(),
            }),
        };
        DOCTRINE_MAPPING
            .iter()
            .any(|c| c.eq_ignore_ascii_case(&resolved))
    })
}
