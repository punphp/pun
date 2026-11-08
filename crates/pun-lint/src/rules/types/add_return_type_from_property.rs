//! `lint/types/add-return-type-from-property`, ported from Rector 2.8.4's `ReturnTypeFromStrictTypedPropertyRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): a method whose every `return` returns a property with a declared type
//! (other than `mixed`) gets the union of the returned values' native types.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_span::HasSpan;
use mago_syntax::cst::{Access, ClassLikeMemberSelector, Expression, Node, NodeKind, Variable};

use super::declare::{return_declaration, text_order};
use super::facts::{Facts, facts};
use super::native::native_type;
use super::silent_void::Place;
use super::{
    ClassScope, add_return_type, class_scope, has_only_return_with_expr, returns_scoped, untyped,
};
use crate::rules::types::names::resolve_class_name;
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromProperty;

/// `PhpVersionFeature::TYPED_PROPERTIES`.
const TYPED_PROPERTIES: u32 = 70400;

impl Rule for AddReturnTypeFromProperty {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-property"
    }

    fn min_php_version(&self) -> u32 {
        TYPED_PROPERTIES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = untyped(node, ancestors, ctx) else {
            return;
        };
        let scope = class_scope(ancestors);
        let facts = facts(ctx);
        let returns = returns_scoped(f.body);
        if returns.is_empty() {
            return;
        }
        // resolveReturnPropertyType
        let mut types: Vec<&TAtomic> = Vec::new();
        let mut order: Vec<String> = Vec::new();
        for r in &returns {
            let Some(value) = r.value else {
                return;
            };
            let Some(declared) = typed_property(&facts, ancestors, scope.as_ref(), value) else {
                return;
            };
            let Some(t) = native_type(&facts, value) else {
                return;
            };
            types.extend(t.types.iter());
            order.extend(facts.text(declared).map(text_order).unwrap_or_default());
        }
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        if !has_only_return_with_expr(&place, f.body, &returns) {
            return;
        }
        if types.iter().any(|t| matches!(t, TAtomic::Mixed(_))) {
            return;
        }
        if let Some(r#type) =
            return_declaration(types, &facts.codebase, ctx.php_version, &order, true)
        {
            out.push(add_return_type(f.span, f.at, &r#type));
        }
    }
}

/// The span of the declared type of the property `$object->name` or `Class::$name`, when it has one other than
/// `mixed`.
fn typed_property(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    scope: Option<&ClassScope>,
    expr: &Expression<'_>,
) -> Option<mago_span::Span> {
    let (class, name) = match expr {
        Expression::Access(Access::Property(p)) => {
            let ClassLikeMemberSelector::Identifier(id) = &p.property else {
                return None;
            };
            let class = match p.object {
                Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => {
                    this_class(facts, ancestors, scope)
                }
                object => match native_type(facts, object).map(|t| t.types.as_ref()) {
                    Some([TAtomic::Object(TObject::Named(named))]) => Some(named.name.to_string()),
                    _ => None,
                },
            };
            let class = class?;
            (class, format!("${}", String::from_utf8_lossy(id.value)))
        }
        Expression::Access(Access::StaticProperty(p)) => {
            let Variable::Direct(v) = &p.property else {
                return None;
            };
            let class = match p.class {
                Expression::Self_(_) | Expression::Static(_) => this_class(facts, ancestors, scope),
                Expression::Parent(_) => scope.and_then(|s| s.parent_class.clone()),
                Expression::Identifier(id) => Some(resolve_class_name(ancestors, id)),
                _ => None,
            };
            let class = class?;
            (class, String::from_utf8_lossy(v.name).into_owned())
        }
        _ => return None,
    };
    facts
        .codebase
        .get_declaring_property(class.as_bytes(), name.as_bytes())
        .and_then(|p| p.type_declaration_metadata.as_ref())
        .filter(|t| !t.type_union.is_mixed())
        .map(|t| t.span)
}

/// The class `$this` and `self` stand for: the enclosing class, by Mago's name for an anonymous one.
fn this_class(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    scope: Option<&ClassScope>,
) -> Option<String> {
    if let Some(Node::AnonymousClass(class)) = ancestors.iter().rev().find(|a| {
        matches!(
            a,
            Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
        )
    }) {
        return Some(facts.anonymous_class_name(class.span()));
    }
    scope.and_then(|s| s.name.clone())
}
