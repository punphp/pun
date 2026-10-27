//! `lint/phpunit/prefer-requires-attribute`, ported from rector-phpunit's
//! `RequiresAnnotationWithValueToAttributeRector` (rules/AnnotationsToAttributes/Rector/Class_) and its
//! `RequiresAttributeFactory`.
//!
//! A test class or test method's generic `@requires <type> <value>` tags become `#[\PHPUnit\Framework\Attributes\
//! Requires*(...)]` groups after its attributes, one per distinct tag value; a tag of another type stays. Rector runs
//! it only for PHPUnit 10 and later; Pun leaves that to the rule's group.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::annotations::{Annotated, TagValue, map_item, named_tags};
use super::{is_in_test_class, report};
use crate::{Context, Report, Rule};

pub struct PreferRequiresAttribute;

impl Rule for PreferRequiresAttribute {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-requires-attribute"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let annotated = Annotated::of(ctx.source, node, ancestors).unwrap();
        let Some(doc) = annotated.doc.as_ref() else {
            return;
        };
        if !doc.text_all().contains("@requires") {
            return;
        }
        let in_test_class = match node {
            Node::Method(method) => is_in_test_class(ctx, ancestors, Some(method)),
            _ => {
                let mut scope = ancestors.to_vec();
                scope.push(node);
                is_in_test_class(ctx, &scope, None)
            }
        };
        if !in_test_class {
            return;
        }
        let Some(tags) = named_tags(ctx, ancestors, doc, "requires") else {
            return;
        };
        // `handleRequires()`: the groups keyed by the tag's value, in the order of their first tag.
        let mut removed = vec![false; doc.len()];
        let mut keyed: Vec<(String, String)> = Vec::new();
        for (i, value) in tags {
            let TagValue::Generic(requires) = value else {
                continue;
            };
            let Some(group) = create(&requires) else {
                continue;
            };
            if !keyed.iter().any(|(key, _)| *key == requires) {
                keyed.push((requires, group));
            }
            removed[i] = true;
        }
        if keyed.is_empty() {
            return;
        }
        let groups: Vec<String> = keyed.into_iter().map(|(_, group)| group).collect();
        let new_doc = doc.print(&removed);
        let Some(edits) = annotated.edits(ctx.source, Some(&new_doc), &groups) else {
            return;
        };
        out.push(report(
            doc.span,
            "Use the `#[Requires*]` attributes instead of the `@requires` annotation.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `RequiresAttributeFactory::create()`: the group for a `@requires` value, printed; `None` for an unknown type.
fn create(annotation_value: &str) -> Option<String> {
    let (kind, value) = match annotation_value.split_once(' ') {
        Some((kind, value)) => (kind, Some(value)),
        None => (annotation_value, None),
    };
    let at_least = |value: Option<&str>| -> Option<String> {
        // only version is used, we need to prefix with >=
        value.map(|v| match v.as_bytes().first() {
            Some(c) if c.is_ascii_digit() => format!(">= {v}"),
            _ => v.to_string(),
        })
    };
    let two = |value: Option<&str>| -> Vec<Option<String>> {
        let value = value.unwrap_or_default();
        match value.split_once(' ') {
            Some((a, b)) => vec![Some(a.to_string()), Some(b.to_string())],
            None => vec![Some(value.to_string())],
        }
    };
    let owned = |value: Option<&str>| vec![value.map(str::to_string)];
    let (class, items) = match kind {
        "PHP" => ("RequiresPhp", vec![at_least(value)]),
        "PHPUnit" => ("RequiresPhpunit", vec![at_least(value)]),
        "OS" => ("RequiresOperatingSystem", owned(value)),
        "OSFAMILY" => ("RequiresOperatingSystemFamily", owned(value)),
        "function" => match value {
            Some(value) if value.contains("::") => {
                let mut items: Vec<Option<String>> =
                    value.split("::").map(|p| Some(p.to_string())).collect();
                if let Some(Some(first)) = items.first_mut() {
                    first.push_str("::class");
                }
                ("RequiresMethod", items)
            }
            _ => ("RequiresFunction", owned(value)),
        },
        "extension" => ("RequiresPhpExtension", two(value)),
        "setting" => ("RequiresSetting", two(value)),
        _ => return None,
    };
    let arguments: Vec<String> = items
        .iter()
        .map(|item| match item {
            Some(item) => map_item(item),
            None => "null".to_string(),
        })
        .collect();
    Some(format!(
        "#[\\PHPUnit\\Framework\\Attributes\\{class}({})]",
        arguments.join(", ")
    ))
}
