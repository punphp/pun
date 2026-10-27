//! `lint/phpunit/prefer-attribute-over-annotation`, ported from rector-phpunit's
//! `AnnotationWithValueToAttributeRector` (rules/AnnotationsToAttributes/Rector/Class_) and its
//! `AnnotationWithValueToAttribute` configuration.
//!
//! In a test class, the class's and each method's generic tags of the configured annotations become groups of the
//! configured attribute class with the tag's value (through the value map, lower-cased, when there is one) after the
//! attributes of the node, or of the class for a class-level annotation. It does nothing until configured, as Rector.
//! Rector runs it only for PHPUnit 10 and later; Pun leaves that to the rule's group.

use mago_syntax::cst::{ClassLikeMember, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};
use serde_json::Value;

use super::annotations::{Annotated, TagValue, map_item, named_tags};
use super::prefer_test_with_attribute::print_float;
use super::{class_body, is_in_test_class, report};
use crate::{Context, Report, Rule};

/// One `AnnotationWithValueToAttribute`.
struct Annotation {
    name: String,
    attribute_class: String,
    value_map: Vec<(String, Value)>,
    is_on_class_level: bool,
}

#[derive(Default)]
pub struct PreferAttributeOverAnnotation {
    annotations: Vec<Annotation>,
}

impl Rule for PreferAttributeOverAnnotation {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-attribute-over-annotation"
    }

    /// `configure`: `items`, a list of tables with `annotation-name`, `attribute-class`, `value-map` (a table of
    /// lower-cased values to what the attribute gets) and `is-on-class-level`, as `pun migrate` writes Rector's
    /// `AnnotationWithValueToAttribute` objects.
    fn configure(&mut self, options: &Value) -> Result<(), String> {
        let items = match options {
            Value::Null => return Ok(()),
            Value::Object(map) => {
                if let Some(key) = map.keys().find(|k| *k != "items") {
                    return Err(format!("unknown option `{key}`"));
                }
                map.get("items").and_then(Value::as_array)
            }
            _ => return Err("options must be a table".into()),
        };
        let Some(items) = items else {
            return Err("`items` must be a list of tables".into());
        };
        for item in items {
            let Some(item) = item.as_object() else {
                return Err("`items` must be a list of tables".into());
            };
            let mut annotation = Annotation {
                name: String::new(),
                attribute_class: String::new(),
                value_map: Vec::new(),
                is_on_class_level: false,
            };
            for (key, value) in item {
                match (key.as_str(), value) {
                    ("annotation-name", Value::String(s)) => annotation.name = s.clone(),
                    ("attribute-class", Value::String(s)) => {
                        annotation.attribute_class = s.trim_start_matches('\\').to_string()
                    }
                    ("value-map", Value::Object(map)) => {
                        annotation.value_map =
                            map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
                    }
                    // PHP's `[]`
                    ("value-map", Value::Array(a)) if a.is_empty() => {}
                    ("is-on-class-level", Value::Bool(b)) => annotation.is_on_class_level = *b,
                    _ => {
                        return Err(format!(
                            "unknown option `{key}` or a value of the wrong type in `items`"
                        ));
                    }
                }
            }
            if annotation.name.is_empty() || annotation.attribute_class.is_empty() {
                return Err("each of `items` needs `annotation-name` and `attribute-class`".into());
            }
            self.annotations.push(annotation);
        }
        Ok(())
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
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let mut class_groups = Vec::new();
        let mut edits = Vec::new();
        let mut span = None;
        // handle class level
        let class = Annotated::of(ctx.source, node, ancestors);
        let mut class_doc = None;
        if let Some(class) = &class {
            let Some(found) = self.refactor(ctx, &scope, class, true, &mut class_groups) else {
                return;
            };
            class_doc = found;
        }
        // handle method level
        for member in members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let annotated = Annotated::of(ctx.source, Node::Method(method), &scope).unwrap();
            let mut groups = Vec::new();
            let Some(doc) = self.refactor(ctx, &scope, &annotated, false, &mut class_groups) else {
                return;
            };
            if let Some((doc_span, new_doc, own)) = doc {
                groups.extend(own);
                span.get_or_insert(doc_span);
                let Some(method_edits) = annotated.edits(ctx.source, Some(&new_doc), &groups)
                else {
                    return;
                };
                edits.extend(method_edits);
            }
        }
        let (class_doc_span, class_new_doc) = match class_doc {
            Some((s, d, _)) => (Some(s), Some(d)),
            None => (None, None),
        };
        // The class's groups in the order their tags came.
        let groups = class_groups;
        if class_doc_span.is_none() && groups.is_empty() && edits.is_empty() {
            return;
        }
        match node {
            Node::AnonymousClass(anonymous) if !groups.is_empty() => {
                // `new #[...] class`: php-parser prints each group with a space after it.
                if !anonymous.attribute_lists.is_empty() || !anonymous.modifiers.is_empty() {
                    return;
                }
                let at = anonymous.class.span.start.offset as usize;
                let text: String = groups.iter().map(|g| format!("{g} ")).collect();
                edits.push(Edit::new(Span::new(at, at), text));
            }
            _ => {
                if let Some(class) = &class {
                    let Some(class_edits) =
                        class.edits(ctx.source, class_new_doc.as_deref(), &groups)
                    else {
                        return;
                    };
                    edits.extend(class_edits);
                }
            }
        }
        // a group or an edit comes from a doc that changed
        let span = class_doc_span.or(span).unwrap();
        out.push(report(
            span,
            "Use attributes instead of the PHPUnit annotations with a value.",
            Safety::Safe,
            edits,
        ));
    }
}

impl PreferAttributeOverAnnotation {
    /// `refactorClassMethodOrClass()` for the class (`is_class`) or method `annotated`: its doc's span, its new doc
    /// and its own groups when a tag moved (the class's and the class-level groups go to `class_groups`); `None` for a
    /// doc Pun does not read.
    #[allow(clippy::type_complexity)]
    fn refactor(
        &self,
        ctx: &Context<'_>,
        scope: &[Node<'_, '_>],
        annotated: &Annotated<'_>,
        is_class: bool,
        class_groups: &mut Vec<String>,
    ) -> Option<Option<(Span, String, Vec<String>)>> {
        let Some(doc) = annotated.doc.as_ref() else {
            return Some(None);
        };
        let mut removed = vec![false; doc.len()];
        let mut own = Vec::new();
        for annotation in &self.annotations {
            // `PhpDocInfo::getTagsByName()`: for simple tag names only
            let name = annotation.name.trim_start_matches('@');
            if name.contains('\\') {
                continue;
            }
            for (i, value) in named_tags(ctx, scope, doc, name)? {
                let TagValue::Generic(value) = value else {
                    continue;
                };
                let argument = self.resolve_attribute_value(annotation, &value);
                let group = format!("#[\\{}({argument})]", annotation.attribute_class);
                if is_class || annotation.is_on_class_level {
                    class_groups.push(group);
                } else {
                    own.push(group);
                }
                // cleanup
                removed[i] = true;
            }
        }
        if !removed.contains(&true) {
            return Some(None);
        }
        Some(Some((doc.span, doc.print(&removed), own)))
    }

    /// `resolveAttributeValue()`, printed: the value as it is without a map, else the map's value for it lower-cased
    /// (`null` for a value the map lacks).
    fn resolve_attribute_value(&self, annotation: &Annotation, value: &str) -> String {
        if annotation.value_map.is_empty() {
            // no map? convert value as it is
            return map_item(value);
        }
        let lower = value.to_ascii_lowercase();
        match annotation.value_map.iter().find(|(k, _)| *k == lower) {
            Some((_, Value::Bool(b))) => b.to_string(),
            Some((_, Value::Number(n))) => match n.as_i64() {
                Some(i) => i.to_string(),
                None => print_float(n.as_f64().unwrap_or_default()),
            },
            Some((_, Value::String(s))) => map_item(s),
            _ => "null".to_string(),
        }
    }
}
