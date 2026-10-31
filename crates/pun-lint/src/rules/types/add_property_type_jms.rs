//! `lint/types/add-property-type-jms-scalar` and `lint/types/add-property-type-jms-object`, ported from Rector 2.6.1's
//! `ScalarTypedPropertyFromJMSSerializerAttributeTypeRector` and `ObjectTypedPropertyFromJMSSerializerAttributeTypeRector`
//! (rules/TypeDeclaration/Rector/Class_) with `JMSTypeAnalyzer`, `JMSTypePropertyTypeFactory` and
//! `ScalarStringToTypeMapper`: an untyped property without a default whose `#[JMS\Serializer\Annotation\Type(...)]`
//! attribute names a type gets that type, nullable, and a `null` default.
//!
//! The attribute's first argument is read as a string (a string literal or `X::class`); `DateTime<...>` reads as
//! `DateTime`. The scalar rule maps the names `ScalarStringToTypeMapper` knows to the declaration Rector writes for a
//! property (`string` for every string type, `int` for the integer ranges, `bool` for `true` and `false`, `float`,
//! `array`, `iterable`, `object`); `float` with a `@var string` (or `string` with a `@var float`) gives the doc's type
//! and drops the `@var`. The object rule takes any other name without a `<` as a class name, when the class exists.
//! A name written with a leading `\` Rector prints as `\\Foo`, which does not parse; Pun leaves that property.
//! Properties that also have a Doctrine to-one or to-many attribute, and those `PropertyTypeChangeGuard` refuses, are
//! left alone.
//!
//! Rector crashes on `lowercase-string` and `uppercase-string` (no type mapper for their accessory types) and then
//! leaves the whole file; Pun leaves the class. Other constant expressions Rector's `ValueResolver` evaluates are not
//! read.

use mago_syntax::cst::{
    Access, Attribute, ClassLikeConstantSelector, Expression, Literal, Node, NodeKind,
    PartialArgument, Property,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::facts::facts;
use super::property_type::{ClassView, Prop, VarTag, is_legal, variable_span};
use crate::rules::phpunit::phpdoc_type::TagType;
use crate::rules::rector_batch5::{has_attribute, resolve_class_name};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

/// `PhpVersionFeature::ATTRIBUTES`.
const ATTRIBUTES: u32 = 80000;
/// `ClassName::JMS_TYPE`.
const JMS_TYPE: &str = "JMS\\Serializer\\Annotation\\Type";
/// `CollectionMapping::TO_MANY_CLASSES` and `TO_ONE_CLASSES`.
const COLLECTION_MAPPING: [&str; 8] = [
    "Doctrine\\ORM\\Mapping\\OneToMany",
    "Doctrine\\ORM\\Mapping\\ManyToMany",
    "Doctrine\\ODM\\MongoDB\\Mapping\\Annotations\\ReferenceMany",
    "Doctrine\\ODM\\MongoDB\\Mapping\\Annotations\\EmbedMany",
    "Doctrine\\ORM\\Mapping\\ManyToOne",
    "Doctrine\\ORM\\Mapping\\OneToOne",
    "Doctrine\\ODM\\MongoDB\\Mapping\\Annotations\\ReferenceOne",
    "Doctrine\\ODM\\MongoDB\\Mapping\\Annotations\\EmbedOne",
];

pub struct AddPropertyTypeJmsScalar;

pub struct AddPropertyTypeJmsObject;

impl Rule for AddPropertyTypeJmsScalar {
    fn id(&self) -> &'static str {
        "lint/types/add-property-type-jms-scalar"
    }

    fn min_php_version(&self) -> u32 {
        ATTRIBUTES
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
        visit(node, ancestors, ctx, out, Kind::Scalar);
    }
}

impl Rule for AddPropertyTypeJmsObject {
    fn id(&self) -> &'static str {
        "lint/types/add-property-type-jms-object"
    }

    fn min_php_version(&self) -> u32 {
        ATTRIBUTES
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
        visit(node, ancestors, ctx, out, Kind::Object);
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Scalar,
    Object,
}

fn visit<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    ancestors: &[Node<'ast, 'arena>],
    ctx: &Context<'_>,
    out: &mut Vec<Report>,
    kind: Kind,
) {
    let facts = facts(ctx);
    let Some(view) = ClassView::of(&facts, node, ancestors) else {
        return;
    };
    let mut edits: Vec<Edit> = Vec::new();
    for prop in &view.properties() {
        // shouldSkipProperty
        if prop.default().is_some()
            || !has_jms_type(prop, &view.path)
            || !is_legal(&facts, &view, prop, true)
        {
            continue;
        }
        let Some(value) = type_value(prop, &view.path) else {
            continue;
        };
        let lowered = value.to_ascii_lowercase();
        if lowered == "lowercase-string" || lowered == "uppercase-string" {
            return;
        }
        let declaration = match kind {
            Kind::Scalar => {
                let var = VarTag::of(ctx.source, prop.property, &view.path);
                let doc = match &var.tag {
                    Ok(Some(tag)) => match tag.ty {
                        TagType::Name(n) => Some(n.to_ascii_lowercase()),
                        _ => None,
                    },
                    _ => None,
                };
                // createScalarTypeNode: a `@var` of the other type wins, and goes
                let swapped = match (lowered.as_str(), doc.as_deref()) {
                    ("float", Some("string" | "class-string")) => Some("string"),
                    ("string", Some("float" | "double" | "real")) => Some("float"),
                    _ => None,
                };
                match swapped {
                    Some(t) => {
                        let Some(e) = var.remove(ctx.source) else {
                            continue;
                        };
                        edits.extend(e);
                        t.to_string()
                    }
                    None => match scalar_declaration(&lowered) {
                        Some(t) => t.to_string(),
                        None => continue,
                    },
                }
            }
            Kind::Object => {
                if value.contains('<') || scalar_name(&lowered) {
                    continue;
                }
                // Rector prints a written leading `\` after its own, `\\Foo`, which does not parse; Pun leaves it
                if value.starts_with('\\')
                    || facts.codebase.get_class_like(value.as_bytes()).is_none()
                {
                    continue;
                }
                format!("\\{value}")
            }
        };
        let at = variable_span(prop);
        edits.push(Edit::new(
            Span::new(at.start, at.start),
            format!("?{declaration} "),
        ));
        edits.push(Edit::new(Span::new(at.end, at.end), " = null"));
    }
    let Some(first) = edits.first() else { return };
    out.push(Report {
        span: first.span,
        message: "Type the properties from their JMS Serializer type.".to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits,
        }),
    });
}

fn attribute_lists<'a>(
    prop: &Prop<'a, 'a>,
) -> &'a mago_syntax::cst::Sequence<'a, mago_syntax::cst::AttributeList<'a>> {
    match prop.property {
        Property::Plain(p) => &p.attribute_lists,
        Property::Hooked(p) => &p.attribute_lists,
    }
}

/// `JMSTypeAnalyzer::hasPropertyJMSTypeAttribute`: the `Type` attribute, and no Doctrine to-one or to-many one.
fn has_jms_type(prop: &Prop<'_, '_>, path: &[Node<'_, '_>]) -> bool {
    let lists = attribute_lists(prop);
    has_attribute(path, lists, JMS_TYPE)
        && !COLLECTION_MAPPING
            .iter()
            .any(|c| has_attribute(path, lists, c))
}

/// `JMSTypeAnalyzer::resolveTypeAttributeValue`: the `Type` attribute's first argument as a string.
fn type_value(prop: &Prop<'_, '_>, path: &[Node<'_, '_>]) -> Option<String> {
    let attribute: &Attribute<'_> = attribute_lists(prop)
        .iter()
        .flat_map(|l| l.attributes.iter())
        .find(|a| resolve_class_name(path, &a.name).eq_ignore_ascii_case(JMS_TYPE))?;
    let value = match attribute.argument_list.as_ref()?.arguments.iter().next()? {
        PartialArgument::Positional(p) => p.value,
        PartialArgument::Named(n) => n.value,
        _ => return None,
    };
    let value = match unparenthesize(value) {
        Expression::Literal(Literal::String(s)) => String::from_utf8(s.value?.to_vec()).ok()?,
        Expression::Access(Access::ClassConstant(c)) => match (c.class, &c.constant) {
            (Expression::Identifier(id), ClassLikeConstantSelector::Identifier(k))
                if k.value.eq_ignore_ascii_case(b"class") =>
            {
                resolve_class_name(path, id)
            }
            _ => return None,
        },
        _ => return None,
    };
    // `#DateTime\<(.*?)\>#`, a special case for DateTime
    if let Some(at) = value.find("DateTime<")
        && value[at..].contains('>')
    {
        return Some("DateTime".into());
    }
    Some(value)
}

/// Whether `ScalarStringToTypeMapper::mapScalarStringToType` maps the lowercased name to a type other than an implicit
/// `mixed`.
fn scalar_name(lowered: &str) -> bool {
    scalar_declaration(lowered).is_some()
        || matches!(
            lowered,
            "null"
                | "void"
                | "resource"
                | "callback"
                | "callable"
                | "never"
                | "never-return"
                | "never-returns"
                | "no-return"
                | "array-key"
                | "list"
                | "lowercase-string"
                | "uppercase-string"
        )
}

/// The declaration Rector writes for a property of the type `ScalarStringToTypeMapper` maps the lowercased name to,
/// when it is one name (`Identifier`); `mixed` is skipped.
fn scalar_declaration(lowered: &str) -> Option<&'static str> {
    Some(match lowered {
        "string"
        | "non-empty-string"
        | "literal-string"
        | "numeric-string"
        | "non-falsy-string"
        | "truthy-string"
        | "class-string"
        | "callable-string"
        | "non-empty-lowercase-string"
        | "non-empty-uppercase-string"
        | "non-empty-literal-string"
        | "decimal-int-string"
        | "non-decimal-int-string" => "string",
        "float" | "real" | "double" => "float",
        "int" | "integer" | "positive-int" | "negative-int" | "non-negative-int"
        | "non-positive-int" => "int",
        "bool" | "boolean" | "true" | "false" => "bool",
        "array" | "non-empty-array" => "array",
        "iterable" => "iterable",
        "object" => "object",
        _ => return None,
    })
}
