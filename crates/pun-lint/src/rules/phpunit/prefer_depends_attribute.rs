//! `lint/phpunit/prefer-depends-attribute`, ported from rector-phpunit's
//! `DependsAnnotationWithValueToAttributeRector` (rules/AnnotationsToAttributes/Rector/ClassMethod).
//!
//! In a test class, each method's generic `@depends` tags become `#[\PHPUnit\Framework\Attributes\Depends('m')]`
//! (another method of the class), `DependsOnClass(X::class)`, `DependsUsingDeepClone('m')` (`clone m`) or
//! `DependsExternal(X::class, 'm')` groups after its attributes, when the `Depends` attribute class exists. A tag that
//! fits none stays. Rector runs it only for PHPUnit 10 and later; Pun leaves that to the rule's group.

use mago_syntax::cst::{ClassLikeMember, Method, Node, NodeKind};
use pun_problem::Safety;

use super::annotations::{Annotated, TagValue, map_item, named_tags};
use super::{class_body, is_in_test_class, report};
use crate::rules::types::facts::facts;
use crate::{Context, Report, Rule};

const DEPENDS_ATTRIBUTE: &str = "PHPUnit\\Framework\\Attributes\\Depends";

pub struct PreferDependsAttribute;

impl Rule for PreferDependsAttribute {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-depends-attribute"
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
        let (members, _, _) = class_body(node).unwrap();
        let methods: Vec<&Method<'_>> = members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(method) => Some(method),
                _ => None,
            })
            .collect();
        if !ctx.source.contains("@depends") {
            return;
        }
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None)
            || facts(ctx)
                .codebase
                .get_class_like(DEPENDS_ATTRIBUTE.as_bytes())
                .is_none()
        {
            return;
        }
        let mut edits = Vec::new();
        let mut span = None;
        for method in &methods {
            let annotated = Annotated::of(ctx.source, Node::Method(method), &scope).unwrap();
            let Some(doc) = annotated.doc.as_ref() else {
                continue;
            };
            let Some(tags) = named_tags(ctx, &scope, doc, "depends") else {
                continue;
            };
            let current = String::from_utf8_lossy(method.name.value);
            let mut removed = vec![false; doc.len()];
            let mut groups = Vec::new();
            for (i, value) in tags {
                let TagValue::Generic(value) = value else {
                    continue;
                };
                let Some((class, items)) = resolve(&methods, &current, &value) else {
                    continue;
                };
                let items: Vec<String> = items.iter().map(|i| map_item(i)).collect();
                groups.push(format!("#[\\{class}({})]", items.join(", ")));
                // cleanup
                removed[i] = true;
            }
            if groups.is_empty() {
                continue;
            }
            span.get_or_insert(doc.span);
            let new_doc = doc.print(&removed);
            let Some(method_edits) = annotated.edits(ctx.source, Some(&new_doc), &groups) else {
                return;
            };
            edits.extend(method_edits);
        }
        let Some(span) = span else {
            return;
        };
        out.push(report(
            span,
            "Use the `#[Depends*]` attributes instead of the `@depends` annotation.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `resolveAttributeNameAndItemsFromValue()`: the attribute class and its items for a `@depends` value of the
/// method `current`, whose class has `methods`.
fn resolve(methods: &[&Method<'_>], current: &str, value: &str) -> Option<(String, Vec<String>)> {
    // `Class_::getMethod()`: the name without case.
    let has_method = |name: &str| {
        methods
            .iter()
            .any(|m| m.name.value.eq_ignore_ascii_case(name.as_bytes()))
    };
    // process depends other ClassMethod
    if current != value && has_method(value) {
        return Some((DEPENDS_ATTRIBUTE.to_string(), vec![value.to_string()]));
    }
    // other: depends other Class_
    if let Some(class) = value.strip_suffix("::class") {
        return Some((
            "PHPUnit\\Framework\\Attributes\\DependsOnClass".to_string(),
            vec![format!("{class}::class")],
        ));
    }
    // other: depends clone ClassMethod
    if value.starts_with("clone ") {
        let cloned = value.split("clone ").nth(1).unwrap_or_default();
        if current != cloned && has_method(cloned) {
            return Some((
                "PHPUnit\\Framework\\Attributes\\DependsUsingDeepClone".to_string(),
                vec![cloned.to_string()],
            ));
        }
        // deep clone variant is handled separately
        return None;
    }
    // other: depends external ClassMethod, e.g. "OtherTest::testMethod"
    let (class, method) = value.split_once("::")?;
    if class.is_empty() || method.is_empty() || method == "class" {
        return None;
    }
    Some((
        "PHPUnit\\Framework\\Attributes\\DependsExternal".to_string(),
        vec![format!("{class}::class"), method.to_string()],
    ))
}
