//! `lint/phpunit/prefer-covers-attribute`, ported from rector-phpunit's
//! `CoversAnnotationWithValueToAttributeRector` (rules/AnnotationsToAttributes/Rector/Class_).
//!
//! A test class gets `#[\PHPUnit\Framework\Attributes\Covers*(...)]` groups after its attributes for its
//! `@coversDefaultClass` tags, its `@covers` tags that start with `\` (or `::` without a default class), and the
//! `@covers` tags of its methods that start with `\` or `::` (a `::method` one of the default class's), one per
//! value; those class tags leave its doc. Each test method's doc loses every generic `@covers` tag. Rector runs it
//! only for PHPUnit 10 and later, and only when the four attribute classes exist; Pun leaves the first to the rule's
//! group.

use mago_syntax::cst::{ClassLikeMember, Method, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::annotations::{Annotated, TagValue, is_class_constant_item, map_item, named_tags};
use super::{class_body, is_in_test_class, report};
use crate::rules::lint_b_doc::Doc;
use crate::rules::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const COVERS_FUNCTION: &str = "PHPUnit\\Framework\\Attributes\\CoversFunction";
const COVERS_CLASS: &str = "PHPUnit\\Framework\\Attributes\\CoversClass";
const COVERS_TRAIT: &str = "PHPUnit\\Framework\\Attributes\\CoversTrait";
const COVERS_METHOD: &str = "PHPUnit\\Framework\\Attributes\\CoversMethod";

pub struct PreferCoversAttribute;

impl Rule for PreferCoversAttribute {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-covers-attribute"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if !ctx.source.contains("@covers") {
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
        // avoid partial apply that may cause error
        let codebase = &facts(ctx).codebase;
        if !in_test_class
            || [COVERS_FUNCTION, COVERS_CLASS, COVERS_TRAIT, COVERS_METHOD]
                .iter()
                .any(|class| codebase.get_class_like(class.as_bytes()).is_none())
        {
            return;
        }
        if !matches!(node, Node::Method(_)) {
            let mut scope = ancestors.to_vec();
            scope.push(node);
            if let Ok(Some(report)) = class_attributes(ctx, node, &scope) {
                out.push(report);
            }
            return;
        }
        // The class's own part declines for a doc Pun does not read; the method's tags then stay too.
        if let Some(at) = ancestors
            .iter()
            .rposition(|a| matches!(a, Node::Class(_) | Node::AnonymousClass(_)))
            && class_attributes(ctx, ancestors[at], &ancestors[..=at]).is_err()
        {
            return;
        }
        let Some(annotated) = Annotated::of(ctx.source, node, ancestors) else {
            return;
        };
        let Some(doc) = annotated.doc.as_ref() else {
            return;
        };
        let Some(tags) = named_tags(ctx, ancestors, doc, "covers") else {
            return;
        };
        // removeMethodCoversAnnotations()
        let mut removed = vec![false; doc.len()];
        for (i, value) in tags {
            if let TagValue::Generic(_) = value {
                removed[i] = true;
            }
        }
        if !removed.contains(&true) {
            return;
        }
        let new_doc = doc.print(&removed);
        out.push(report(
            doc.span,
            "Use the `#[Covers*]` attributes on the class instead of the `@covers` annotation.",
            Safety::Safe,
            annotated.edits(ctx.source, Some(&new_doc), &[]),
        ));
    }
}

/// `resolveClassAttributes()` for the class `node` (the last of `scope`): the report that moves its covers to
/// attribute groups, `Ok(None)` when there are none, `Err` when a doc of it is one Pun does not read.
fn class_attributes(
    ctx: &Context<'_>,
    node: Node<'_, '_>,
    scope: &[Node<'_, '_>],
) -> Result<Option<Report>, ()> {
    let Some((members, _, _)) = class_body(node) else {
        return Ok(None);
    };
    let annotated = Annotated::of(ctx.source, node, &scope[..scope.len() - 1]);
    let doc = annotated.as_ref().and_then(|a| a.doc.as_ref());
    let mut default_groups: Vec<String> = Vec::new();
    // The groups keyed by their tag's value: a later one with the same key takes the first one's place.
    let mut keyed: Vec<(String, String)> = Vec::new();
    let add = |keyed: &mut Vec<(String, String)>, key: String, group: String| match keyed
        .iter_mut()
        .find(|(k, _)| *k == key)
    {
        Some(entry) => entry.1 = group,
        None => keyed.push((key, group)),
    };
    let mut default_class = None;
    let mut doc_edit = None;
    if let Some(doc) = doc {
        let mut removed = vec![false; doc.len()];
        // handleCoversDefaultClass()
        let defaults = named_tags(ctx, scope, doc, "coversDefaultClass").ok_or(())?;
        for (i, value) in defaults {
            let TagValue::Generic(value) = value else {
                continue;
            };
            let (group, first) = create_attribute_group(ctx, &value);
            if default_groups.is_empty() {
                // If there is a ::coversDefaultClass, @covers ::function will refer to class methods
                default_class = is_class_constant_item(&first).then_some(first);
            }
            default_groups.push(group);
            removed[i] = true;
        }
        let has_covers_default = !default_groups.is_empty();
        // handleCovers()
        for (i, value) in named_tags(ctx, scope, doc, "covers").ok_or(())? {
            let TagValue::Generic(covers) = value else {
                continue;
            };
            if covers.starts_with('\\') || (!has_covers_default && covers.starts_with("::")) {
                let (group, _) = create_attribute_group(ctx, &covers);
                add(&mut keyed, covers, group);
                removed[i] = true;
            } else if has_covers_default && covers.starts_with("::") {
                removed[i] = true;
            }
        }
        if removed.contains(&true) {
            doc_edit = Some(doc.print(&removed));
        }
    }
    let methods = members.iter().filter_map(|m| match m {
        ClassLikeMember::Method(method) => Some(method),
        _ => None,
    });
    for method in methods {
        for (covers, group) in method_attributes(ctx, scope, method, default_class.as_deref())? {
            add(&mut keyed, covers, group);
        }
    }
    let groups: Vec<String> = default_groups
        .into_iter()
        .chain(keyed.into_iter().map(|(_, group)| group))
        .collect();
    if groups.is_empty() {
        return Ok(None);
    }
    let message = "Use the `#[Covers*]` attributes instead of the `@covers` annotations.";
    let edits = match (node, &annotated) {
        (Node::AnonymousClass(class), _) => {
            // `new #[...] class`: php-parser prints each group with a space after it.
            if !class.attribute_lists.is_empty() || !class.modifiers.is_empty() {
                return Err(());
            }
            let at = class.class.span.start.offset as usize;
            let text: String = groups.iter().map(|g| format!("{g} ")).collect();
            vec![Edit::new(Span::new(at, at), text)]
        }
        (_, Some(annotated)) => annotated.edits(ctx.source, doc_edit.as_deref(), &groups),
        _ => return Ok(None),
    };
    let span = doc.map_or_else(
        || {
            let s = node_span(&node);
            Span::new(s.start, s.start)
        },
        |d: &Doc<'_>| d.span,
    );
    Ok(Some(report(span, message, Safety::Safe, edits)))
}

/// `resolveMethodAttributes()`: the groups of a method's `@covers` tags that start with `\` or `::`, keyed by the
/// tag's value (a `::method` one after the default class's name).
fn method_attributes(
    ctx: &Context<'_>,
    scope: &[Node<'_, '_>],
    method: &Method<'_>,
    default_class: Option<&str>,
) -> Result<Vec<(String, String)>, ()> {
    let Some(annotated) = Annotated::of(ctx.source, Node::Method(method), scope) else {
        return Ok(Vec::new());
    };
    let Some(doc) = annotated.doc.as_ref() else {
        return Ok(Vec::new());
    };
    let mut groups = Vec::new();
    for (_, value) in named_tags(ctx, scope, doc, "covers").ok_or(())? {
        let TagValue::Generic(mut covers) = value else {
            continue;
        };
        if !covers.starts_with('\\') && !covers.starts_with("::") {
            continue;
        }
        if let Some(default_class) = default_class
            && covers.starts_with("::")
        {
            covers = format!("{default_class}{covers}");
        }
        let (group, _) = create_attribute_group(ctx, &covers);
        groups.push((covers, group));
    }
    Ok(groups)
}

/// `createAttributeGroup()`: the group for a covers value, printed, and its first item before mapping.
fn create_attribute_group(ctx: &Context<'_>, value: &str) -> (String, String) {
    let (class, items) = if value.starts_with("::") {
        (
            COVERS_FUNCTION,
            vec![value.trim_matches([':', '(', ')']).to_string()],
        )
    } else if value.contains("::") {
        (
            COVERS_METHOD,
            vec![format!("{}::class", get_class(value)), get_method(value)],
        )
    } else {
        let trait_ = facts(ctx)
            .codebase
            .get_class_like(value.trim_start_matches('\\').as_bytes())
            .is_some_and(|c| c.kind.is_trait());
        let class = if trait_ { COVERS_TRAIT } else { COVERS_CLASS };
        let trimmed = value.trim_matches([' ', '\t', '\n', '\r', '\0', '\x0B']);
        (class, vec![format!("{trimmed}::class")])
    };
    let printed: Vec<String> = items.iter().map(|i| map_item(i)).collect();
    let first = items[0].clone();
    (format!("#[\\{class}({})]", printed.join(", ")), first)
}

/// `Strings::replace($value, '/::.*$/')`: the value up to its first `::` whose rest of the value holds no line break.
fn get_class(value: &str) -> String {
    let mut from = 0;
    while let Some(at) = value[from..].find("::") {
        let at = from + at;
        if !value[at..].contains('\n') {
            return value[..at].to_string();
        }
        from = at + 1;
    }
    value.to_string()
}

/// `Strings::replace($value, '/^.*::/')`: the value after the last `::` of its first line.
fn get_method(value: &str) -> String {
    let line = value.split('\n').next().unwrap_or_default();
    match line.rfind("::") {
        Some(at) => value[at + 2..].to_string(),
        None => value.to_string(),
    }
}
