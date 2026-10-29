//! `lint/refactor/prefer-attribute-over-annotation`, ported from Rector 2.6.1's `AnnotationToAttributeRector`
//! (rules/Php80/Rector/Class_) with its `AnnotationToAttribute` configuration, on the name handling's doc comments
//! (`rector_names::doc`) and attributes (`rector_names::attribute`).
//!
//! Options: `items`, a list of tables with `tag` (the annotation's class, or a simple tag's name), `attribute-class`
//! (the tag when left out), `class-reference-fields` (the arguments whose string becomes `Class::class`) and
//! `use-value-as-attribute-argument` (a simple tag's value is the attribute's argument), as `pun migrate` writes
//! Rector's `AnnotationToAttribute` objects. It does nothing until configured.
//!
//! In the doc comment of a class, an interface, a method, a property, a parameter or a function, each Doctrine
//! annotation of a configured class whose attribute class exists and is an attribute becomes that attribute (its
//! arguments named, nested annotations `new` from PHP 8.1), and each simple tag of a configured name whose attribute
//! class exists becomes the attribute with the tag's value. The tags go from the doc (a text after an annotation's
//! closing parenthesis stays), and the attributes come after the node's own.

use mago_syntax::cst::{Node, NodeKind, Program};
use pun_problem::{Edit, Span};
use serde_json::Value;

use super::{items, object, report};
use crate::rules::phpunit::annotations::Annotated;
use crate::rules::rector_names::attribute::{self, AttributeConfig, Attribute};
use crate::rules::rector_names::doc::{Doc, Value as DocValue};
use crate::rules::rector_names::resolve::{self, Scope, has_class};
use crate::rules::rector_names::view::{FileView, UseView};
use crate::rules::rector_stmts::line_indent;
use crate::rules::types::facts::facts;
use crate::{Context, Report, Rule};

#[derive(Default)]
pub struct AnnotationToAttribute {
    configs: Vec<AttributeConfig>,
}

impl Rule for AnnotationToAttribute {
    fn id(&self) -> &'static str {
        "lint/refactor/prefer-attribute-over-annotation"
    }

    fn configure(&mut self, options: &Value) -> Result<(), String> {
        self.configs.clear();
        for item in items(options)? {
            let map = object(
                item,
                &["tag", "attribute-class", "class-reference-fields", "use-value-as-attribute-argument"],
                &["tag"],
            )?;
            let class_name = |key: &str| -> Result<Option<String>, String> {
                match map.get(key) {
                    None | Some(Value::Null) => Ok(None),
                    Some(Value::String(name)) if is_class_name(name) => Ok(Some(name.clone())),
                    Some(Value::String(name)) => Err(format!("\"{name}\" is not a valid class name")),
                    Some(_) => Err(format!("`{key}` must be a string")),
                }
            };
            let tag = class_name("tag")?.unwrap_or_default();
            let attribute_class = class_name("attribute-class")?.unwrap_or_else(|| tag.clone());
            let class_reference_fields = match map.get("class-reference-fields") {
                None => Vec::new(),
                Some(Value::Array(fields)) => fields
                    .iter()
                    .map(|f| f.as_str().map(str::to_string))
                    .collect::<Option<_>>()
                    .ok_or("`class-reference-fields` must be a list of strings")?,
                Some(_) => return Err("`class-reference-fields` must be a list of strings".into()),
            };
            let use_value_as_attribute_argument = match map.get("use-value-as-attribute-argument") {
                None => false,
                Some(value) => value.as_bool().ok_or("`use-value-as-attribute-argument` must be a boolean")?,
            };
            self.configs.push(AttributeConfig {
                tag,
                attribute_class,
                class_reference_fields,
                use_value_as_attribute_argument,
            });
        }
        // `resolveWithChangedAttributesClass()`: an annotation already written with the attribute's class too
        for config in self.configs.clone() {
            if config.attribute_class != config.tag {
                self.configs.push(AttributeConfig {
                    tag: config.attribute_class.clone(),
                    ..config
                });
            }
        }
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else {
            return;
        };
        // `MinPhpVersionInterface`: attributes
        if self.configs.is_empty() || ctx.php_version < 80000 {
            return;
        }
        let Some(edits) = self.fix(ctx, program, node) else {
            return;
        };
        for edit in edits {
            out.push(report(
                edit.span,
                "Replace the annotation by an attribute.".into(),
                vec![edit],
            ));
        }
    }
}

/// The imports `UseImportsResolver::resolveBareUses()` gives: the `use` statements (no group) of the file's only
/// namespace or of a file without one, as `(name, alias)`, with each name's span.
fn bare_uses(view: &FileView) -> Vec<(String, Option<String>, Span)> {
    if view.namespaces > 1 {
        return Vec::new();
    }
    view.uses_root()
        .uses
        .iter()
        .filter(|u| u.prefix.is_none())
        .flat_map(|u| u.items.iter().map(|i| (i.name.clone(), i.alias.clone(), i.name_span)))
        .collect()
}

impl AnnotationToAttribute {
    /// The edits of every node the rule changes, `None` where Pun does not follow Rector on one.
    fn fix(&self, ctx: &Context<'_>, program: &Program<'_>, root: Node<'_, '_>) -> Option<Vec<Edit>> {
        let view = FileView::of(ctx.source, program);
        let uses = bare_uses(&view);
        let original: Vec<(String, Option<String>)> = uses.iter().map(|(n, a, _)| (n.clone(), a.clone())).collect();
        // the import each renamed `use` gets (`AttributeNameFactory`), by index
        let mut renamed: Vec<Option<String>> = vec![None; uses.len()];
        let mut edits = Vec::new();
        // the nodes in the order the traverser enters them
        let mut stack = vec![(root, Vec::new())];
        while let Some((node, ancestors)) = stack.pop() {
            let mut path = ancestors.clone();
            path.push(node);
            for child in node.children().into_iter().rev() {
                stack.push((child, path.clone()));
            }
            if !is_target(node) {
                continue;
            }
            let Some(annotated) = Annotated::of_last(ctx.source, node, &ancestors) else {
                continue;
            };
            let Some(doc_comment) = &annotated.doc else {
                continue;
            };
            let span = doc_comment.span;
            let Some(doc_view) = view.docs.iter().find(|d| d.span == span) else {
                continue;
            };
            let scope_uses: &[UseView] = &view.uses_root().uses;
            let scope = Scope {
                namespace: doc_view.namespace.as_deref(),
                class: doc_view.class.as_deref(),
                uses: scope_uses,
            };
            let text = &ctx.source[span.start..span.end];
            let mut doc = Doc::parse(
                text,
                &|tag| resolve::resolve_tag(ctx, scope, tag),
                &|name| resolve::narrow(ctx, scope, name).class_name().map(str::to_string),
            )?;
            let mut groups = self.doctrine_annotations(ctx, &mut doc, &original, &mut renamed);
            groups.extend(self.simple_tags(ctx, &mut doc));
            if groups.is_empty() {
                continue;
            }
            let printed: Vec<String> = groups.iter().map(ToString::to_string).collect();
            let new_doc = doc.print();
            // `TokenStream::getIndent()`: a tab counts as 4 columns, and the printer indents with spaces
            let width: usize = line_indent(ctx.source, annotated.start).chars().map(|c| if c == '\t' { 4 } else { 1 }).sum();
            let nl = format!("\n{}", " ".repeat(width));
            match annotated.edit_with_comments(ctx.source, &new_doc) {
                // other comments before the doc: `pComments()` reprints them all, then the attributes follow
                Some(mut edit) => match annotated.last_attribute_end {
                    Some(end) => {
                        edits.push(edit);
                        edits.push(Edit::new(Span::new(end, end), printed.iter().map(|g| format!("{nl}{g}")).collect::<String>()));
                    }
                    None => {
                        edit.text.extend(printed.iter().map(|g| format!("{g}{nl}")));
                        edits.push(edit);
                    }
                },
                None => edits.extend(annotated.edits_at(ctx.source, Some(&new_doc), &printed, &nl)?),
            }
        }
        for ((name, _, span), new) in uses.iter().zip(renamed) {
            if let Some(new) = new.filter(|new| new != name) {
                edits.push(Edit::new(*span, new));
            }
        }
        Some(edits)
    }

    /// `processDoctrineAnnotationClasses()`: the attributes of the doc's Doctrine annotations of a configured class
    /// (the doc loses them), none where one has a nested annotation that stays.
    fn doctrine_annotations(
        &self,
        ctx: &Context<'_>,
        doc: &mut Doc,
        uses: &[(String, Option<String>)],
        renamed: &mut [Option<String>],
    ) -> Vec<Attribute> {
        let mut matched = Vec::new();
        for (i, child) in doc.children.iter().enumerate() {
            let DocValue::Annotation(annotation) = &child.value else {
                continue;
            };
            // `matchAnnotationToAttribute()`: `hasClassName()`
            let identifier = annotation.identifier.trim_matches('@');
            let Some(config) = self.configs.iter().find(|c| identifier == c.tag || annotation.class == c.tag) else {
                continue;
            };
            // processed with the simple tags
            if config.use_value_as_attribute_argument || !is_attribute_class(ctx, &config.attribute_class) {
                continue;
            }
            matched.push((i, config));
        }
        let mut attributes = Vec::new();
        let mut renames = Vec::new();
        for (i, config) in &matched {
            let DocValue::Annotation(annotation) = &doc.children[*i].value else {
                continue;
            };
            let (name, rename) = attribute::attribute_name(config, &annotation.identifier, uses);
            renames.extend(rename);
            let constructor = constructor_parameters(ctx, &config.attribute_class);
            attributes.push(attribute::create(annotation, config, name, &constructor, ctx.php_version >= 80100));
        }
        for (i, new) in renames {
            renamed[i] = Some(new);
        }
        if attributes.iter().any(attribute::has_remove_array_state) {
            return Vec::new();
        }
        // the annotations go, last first so the indexes hold; a text after the closing parenthesis stays
        for (i, _) in matched.iter().rev() {
            let DocValue::Annotation(annotation) = &doc.children[*i].value else {
                continue;
            };
            match attribute::trailing_comment(annotation.original()) {
                Some(comment) => doc.replace_with_text(*i, comment),
                None => doc.remove(*i),
            }
        }
        attributes
    }

    /// `processGenericTags()`: the attributes of the doc's simple tags (a name without `\`) of a configured name
    /// whose attribute class exists (the doc loses them, or keeps the comment after a value).
    fn simple_tags(&self, ctx: &Context<'_>, doc: &mut Doc) -> Vec<Attribute> {
        let mut attributes = Vec::new();
        let mut i = 0;
        while i < doc.children.len() {
            let child = &doc.children[i];
            let (value, original) = match &child.value {
                DocValue::Generic(value) => (value.clone(), None),
                DocValue::Annotation(annotation) => (annotation.values_string(), Some(annotation.original().to_string())),
                _ => {
                    i += 1;
                    continue;
                }
            };
            let tag = child.tag.as_deref().unwrap_or_default().trim_matches('@').to_string();
            let config = self
                .configs
                .iter()
                .filter(|c| c.tag.eq_ignore_ascii_case(&tag))
                .find(|c| has_class(ctx, &c.attribute_class));
            let Some(config) = config.filter(|_| !tag.contains('\\')) else {
                i += 1;
                continue;
            };
            let resolved = config
                .use_value_as_attribute_argument
                .then(|| attribute::resolve_value(&value, original.as_deref()));
            attributes.push(attribute::from_simple_tag(config, resolved.as_ref().map(|(v, _)| v.as_str())));
            // keep partial original comment, if useful
            match resolved {
                Some((_, comment)) if !comment.is_empty() => {
                    doc.replace_with_text(i, comment);
                    i += 1;
                }
                _ => doc.remove(i),
            }
        }
        attributes
    }
}

/// The nodes `AnnotationToAttributeRector` refactors (`Closure` and `ArrowFunction` aside).
fn is_target(node: Node<'_, '_>) -> bool {
    matches!(
        node,
        Node::Class(_) | Node::Interface(_) | Node::Method(_) | Node::Property(_) | Node::Function(_) | Node::FunctionLikeParameter(_)
    )
}

/// `RectorAssert::className()`.
fn is_class_name(name: &str) -> bool {
    crate::rules::rector_names::doc::is_class_name(name)
}

/// `isExistingAttributeClass()`: the class exists and is marked `#[Attribute]`.
fn is_attribute_class(ctx: &Context<'_>, class: &str) -> bool {
    has_class(ctx, class)
        && facts(ctx)
            .codebase
            .get_class_like(class.as_bytes())
            .is_some_and(|c| c.attribute_flags.is_some())
}

/// The constructor parameters of `class`: each name and whether its type has `int` (`containsInteger()`).
fn constructor_parameters(ctx: &Context<'_>, class: &str) -> Vec<(String, bool)> {
    let codebase = &facts(ctx).codebase;
    let Some(constructor) = codebase.get_method(class.as_bytes(), b"__construct") else {
        return Vec::new();
    };
    constructor
        .parameters
        .iter()
        .map(|p| {
            let name = p.name.0.as_str().unwrap_or_default().trim_start_matches('$').to_string();
            let has_int = p
                .type_metadata
                .as_ref()
                .is_some_and(|t| t.type_union.types.iter().any(|a| a.is_int()));
            (name, has_int)
        })
        .collect()
}
