//! A doc comment as the name handling reads and rewrites it: Rector's `PhpDocInfo` (phpdoc-parser 2.3's children, its
//! `DoctrineAnnotationDecorator` taking a tag whose name resolves to a class for a Doctrine annotation), the
//! `IdentifierTypeNode`s of its tags in `PhpDocNodeTraverser` order, and `PhpDocInfoPrinter::printFormatPreserving()`
//! with the children whose identifiers changed reprinted (`(string) $phpDocChildNode`).
//!
//! Pun reprints the tags whose value it reads: `@param`, `@var`, `@return`, `@throws`, `@property`, `@template`,
//! `@extends` and their kin (with the `phpstan-`, `psalm-` and `phan-` prefixes), and a Doctrine annotation whose
//! arguments are strings, names, numbers and booleans. [`Doc::reprintable`] tells the callers where it cannot, so they
//! leave the file alone rather than write half of Rector's change.

use pun_phpstan::phpdoc::{self, Node, ShapeKey};

use super::phpdoc_parser::{self as parser, ArrayItem, Key, Val, t};
use crate::rules::phpdoc_writer::type_node::TypeNode;

/// How a typed tag's value is laid out around its type.
#[derive(Clone, Debug)]
enum Form {
    /// `ParamTagValueNode`: `Type &...$name description`.
    Param {
        reference: bool,
        variadic: bool,
        variable: String,
    },
    /// `VarTagValueNode`: `Type $name description`, the name optional.
    Var(String),
    /// `ReturnTagValueNode` and its kin: `Type description`.
    Plain,
    /// `PropertyTagValueNode`, `ParamOutTagValueNode`, `ParamClosureThisTagValueNode`: `Type $name description`.
    Named(String),
    /// Rector's `SpacingAwareTemplateTagValueNode`: `T of Bound description` (`of` or `as` as written).
    Template { name: String, preposition: String },
}

/// A tag whose value holds types Pun reads and reprints.
#[derive(Clone, Debug)]
pub(crate) struct Typed {
    form: Form,
    /// The type, or a template's bound (none without one).
    types: Vec<Node>,
    description: String,
    /// The identifiers [`Doc::set_ident`] replaced: a new `IdentifierTypeNode` has no `PARENT` attribute, so the
    /// name importing no longer sees its template tag.
    replaced: Vec<usize>,
}

/// A Doctrine annotation: Rector's `SpacelessPhpDocTagNode` with a `DoctrineAnnotationTagValueNode`.
#[derive(Clone, Debug)]
pub(crate) struct Annotation {
    /// The class the annotation's name resolves to (`RESOLVED_CLASS`).
    pub class: String,
    /// The name of its `IdentifierTypeNode`: the tag's name as written, until a rule replaces it.
    pub identifier: String,
    /// `originalContent`: the arguments as the decorator read them.
    original: String,
    /// The arguments, as `StaticDoctrineAnnotationParser` makes them.
    pub values: Vec<ArrayItem>,
}

/// What a child of the doc is.
#[derive(Clone, Debug)]
pub(crate) enum Value {
    /// `PhpDocTextNode`.
    Text,
    Typed(Typed),
    /// `GenericTagValueNode`: its value.
    Generic(String),
    Annotation(Annotation),
    /// Any other tag (`@method`, `@phpstan-assert`, a value its parser rejects...): the identifiers in it, never
    /// reprinted.
    Other(Vec<String>),
}

#[derive(Clone, Debug)]
pub(crate) struct Child {
    /// `START_AND_END`: the first token and the token after the last.
    start_end: (usize, usize),
    /// The tag's name, its `@` included; `None` for text.
    pub tag: Option<String>,
    pub value: Value,
    /// Whether a node inside was replaced (`ChangedPhpDocNodeVisitor`): the child is reprinted.
    pub changed: bool,
    /// Whether the change cannot be reprinted as Rector does.
    unprintable: bool,
}

/// An `IdentifierTypeNode` of the doc: its child, its place among the child's identifiers, its name, and whether its
/// parent is a `TemplateTagValueNode`.
#[derive(Clone, Debug)]
pub(crate) struct Ident {
    pub child: usize,
    pub index: usize,
    pub name: String,
    pub template_parent: bool,
}

/// A doc comment.
#[derive(Clone, Debug)]
pub(crate) struct Doc {
    pub text: String,
    tokens: Vec<parser::Tok>,
    pub children: Vec<Child>,
    single_line: bool,
    /// `LAST_PHP_DOC_TOKEN_POSITION`.
    last_token_position: Option<usize>,
}

const PARAM: &[&str] = &["@param", "@phpstan-param", "@psalm-param", "@phan-param"];
const VAR: &[&str] = &["@var", "@phpstan-var", "@psalm-var", "@phan-var"];
const PLAIN: &[&str] = &[
    "@return",
    "@phpstan-return",
    "@psalm-return",
    "@phan-return",
    "@phan-real-return",
    "@throws",
    "@phpstan-throws",
    "@mixin",
    "@phan-mixin",
    "@psalm-require-extends",
    "@phpstan-require-extends",
    "@psalm-require-implements",
    "@phpstan-require-implements",
    "@phpstan-sealed",
    "@psalm-inheritors",
    "@phpstan-this-out",
    "@phpstan-self-out",
    "@psalm-this-out",
    "@psalm-self-out",
    "@extends",
    "@phpstan-extends",
    "@phan-extends",
    "@phan-inherits",
    "@template-extends",
    "@implements",
    "@phpstan-implements",
    "@template-implements",
    "@use",
    "@phpstan-use",
    "@template-use",
];
const NAMED: &[&str] = &[
    "@property",
    "@property-read",
    "@property-write",
    "@phpstan-property",
    "@phpstan-property-read",
    "@phpstan-property-write",
    "@psalm-property",
    "@psalm-property-read",
    "@psalm-property-write",
    "@phan-property",
    "@phan-property-read",
    "@phan-property-write",
    "@param-out",
    "@phpstan-param-out",
    "@psalm-param-out",
    "@param-closure-this",
    "@phpstan-param-closure-this",
];
const TEMPLATE: &[&str] = &[
    "@template",
    "@phpstan-template",
    "@psalm-template",
    "@phan-template",
    "@template-covariant",
    "@phpstan-template-covariant",
    "@psalm-template-covariant",
    "@template-contravariant",
    "@phpstan-template-contravariant",
    "@psalm-template-contravariant",
];
/// `NodeTypes::TYPE_AWARE_NODES`: the value nodes whose presence lets `ClassRenamer` rename the doc's types.
pub(crate) fn is_type_aware(child: &Child) -> bool {
    let Value::Typed(typed) = &child.value else {
        return false;
    };
    let name = child.tag.as_deref().unwrap_or_default();
    matches!(typed.form, Form::Template { .. })
        || PARAM.contains(&name)
        || VAR.contains(&name)
        || NAMED[..12].contains(&name)
        || ["@return", "@phpstan-return", "@psalm-return", "@phan-return", "@phan-real-return"].contains(&name)
        || ["@throws", "@phpstan-throws"].contains(&name)
}

impl Doc {
    /// Read the doc comment `text`; `resolve_tag` is `ClassAnnotationMatcher::resolveTagFullyQualifiedName()` and
    /// `narrow` the class `ObjectTypeSpecifier::narrowToFullyQualifiedOrAliasedObjectType()` gives (`None` for a
    /// template), at the node the doc belongs to. `None` where Pun does not follow how Rector's
    /// `DoctrineAnnotationDecorator` draws the children: a `@\Class` inside a text, nested annotations split over
    /// tags (`mergeNestedDoctrineAnnotations()`), arguments Rector's parsers never finish reading.
    pub fn parse(
        text: &str,
        resolve_tag: &dyn Fn(&str) -> String,
        narrow: &dyn Fn(&str) -> Option<String>,
    ) -> Option<Doc> {
        let tokens = parser::tokenize(text);
        let type_end = |start: usize| phpdoc::parse_prefix(&text[start..]).ok().map(|(_, offset)| start + offset);
        let raw = parser::DocParser::new(text, &tokens, &type_end).parse_with_node();
        let mut children = Vec::new();
        for child in raw {
            let (tag, value) = match child.kind {
                // `processTextSpacelessInTextNode()` splits a text with a `@\Class` into annotations
                parser::Kind::Text(text) if text.contains("@\\") || text.contains("})") => return None,
                parser::Kind::Text(_) => (None, Value::Text),
                parser::Kind::Tag { name, value, .. } => {
                    let from = tokens[child.start].start + name.len();
                    let to = tokens[child.end].start;
                    let value = tag_value(text, &tokens, &name, value, from, to, resolve_tag, narrow)?;
                    (Some(name), value)
                }
            };
            children.push(Child {
                start_end: (child.start, child.end),
                tag,
                value,
                changed: false,
                unprintable: false,
            });
        }
        Some(Doc {
            text: text.to_string(),
            single_line: !tokens.iter().any(|t| t.ty == t::PHPDOC_EOL),
            last_token_position: children.last().map(|c| c.start_end.1),
            tokens,
            children,
        })
    }

    /// The doc's `IdentifierTypeNode`s, in traversal order.
    pub fn idents(&self) -> Vec<Ident> {
        let mut out = Vec::new();
        for (child, c) in self.children.iter().enumerate() {
            let mut index = 0;
            let mut push = |name: &str, template_parent: bool| {
                out.push(Ident {
                    child,
                    index,
                    name: name.to_string(),
                    template_parent,
                });
                index += 1;
            };
            match &c.value {
                Value::Typed(typed) => {
                    let template = matches!(typed.form, Form::Template { .. });
                    let mut at = 0;
                    for t in &typed.types {
                        let mut t = t.clone();
                        walk(&mut t, template, &mut |name, parent| {
                            push(name, parent && !typed.replaced.contains(&at));
                            at += 1;
                        });
                    }
                }
                Value::Annotation(annotation) => {
                    // the annotation's own `IdentifierTypeNode`, then those of the annotations nested in its values
                    push(&annotation.identifier, false);
                    nested_identifiers(&annotation.values, &mut |name| push(name, false));
                }
                Value::Other(names) => names.iter().for_each(|n| push(n, false)),
                Value::Text | Value::Generic(_) => {}
            }
        }
        out
    }

    /// Replace the identifier `ident` by `name`: its child is reprinted.
    pub fn set_ident(&mut self, ident: &Ident, name: &str) {
        let child = &mut self.children[ident.child];
        child.changed = true;
        match &mut child.value {
            Value::Typed(typed) => {
                let mut index = 0;
                let template = matches!(typed.form, Form::Template { .. });
                for t in &mut typed.types {
                    walk(t, template, &mut |current, _| {
                        if index == ident.index {
                            *current = name.to_string();
                        }
                        index += 1;
                    });
                }
                typed.replaced.push(ident.index);
            }
            // the annotation's own identifier, never printed (the tag's name is), or a nested one
            Value::Annotation(annotation) if ident.index == 0 => annotation.identifier = name.to_string(),
            Value::Annotation(annotation) => {
                let mut index = 0;
                nested_identifiers_mut(&mut annotation.values, &mut |current| {
                    index += 1;
                    if index == ident.index {
                        *current = name.to_string();
                    }
                });
            }
            _ => child.unprintable = true,
        }
    }

    /// Mark the child `child` changed (a value inside a Doctrine annotation was set).
    pub fn set_changed(&mut self, child: usize) {
        self.children[child].changed = true;
    }

    /// `NameImportingPhpDocNodeVisitor::enterSpacelessPhpDocTagNode()`: the annotation `child` named `name` (its
    /// `@` included), its value reprinted as `processDoctrineAnnotationTagValueNode()` marks it changed.
    pub fn set_tag(&mut self, child: usize, name: &str) {
        self.children[child].tag = Some(name.to_string());
        self.set_changed(child);
    }

    /// `hasByTypes(NodeTypes::TYPE_AWARE_NODES)`.
    pub fn has_type_aware(&self) -> bool {
        self.children.iter().any(is_type_aware)
    }

    /// `PhpDocInfo::getByAnnotationClasses()`: the first annotation of the first of `classes` that has one
    /// (`DoctrineAnnotationTagValueNode::hasClassName()`: its identifier without `@`, or the class it resolves to),
    /// nested ones included, as its child and the path to it ([`Self::values_mut`]).
    pub fn find_annotation(&self, classes: &[&str]) -> Option<(usize, Vec<(usize, bool)>)> {
        for class in classes {
            let is = |identifier: &str, resolved: &str| identifier.trim_matches('@') == *class || resolved == *class;
            for (i, child) in self.children.iter().enumerate() {
                let Value::Annotation(a) = &child.value else {
                    continue;
                };
                if is(&a.identifier, &a.class) {
                    return Some((i, Vec::new()));
                }
                let mut path = Vec::new();
                if locate(&a.values, &is, &mut path) {
                    return Some((i, path));
                }
            }
        }
        None
    }

    /// The class the annotation at `child` and `path` resolves to, and its values.
    pub fn values_mut(&mut self, child: usize, path: &[(usize, bool)]) -> (&str, &mut Vec<ArrayItem>) {
        let Value::Annotation(a) = &mut self.children[child].value else {
            unreachable!("not an annotation")
        };
        match path.is_empty() {
            true => (&a.class, &mut a.values),
            false => navigate(&mut a.values, path),
        }
    }

    /// The class names `UnusedImportRemovingPostRector::findNamesInDocBlocks()` reads from the doc:
    /// `getAnnotationClassNames()`, `getConstFetchNodeClassNames()`, `getGenericTagClassNames()`,
    /// `getInlineGenericUsesTagClassNames()` and `getArrayItemNodeClassNames()`; `resolve` is
    /// `NameScope::resolveStringName()` where the doc is.
    pub fn used_names(&self, resolve: &dyn Fn(&str) -> String) -> Vec<String> {
        let mut names: Vec<String> = self.idents().into_iter().map(|i| i.name.trim_start_matches('@').to_string()).collect();
        for child in &self.children {
            match &child.value {
                Value::Typed(typed) => {
                    for t in &typed.types {
                        const_fetches(t, &mut |class| names.push(resolve(class)));
                    }
                }
                // `getArrayItemNodeClassNames()`: a string value `Class::NAME` (`ArrayItemClassNameDecorator`)
                Value::Annotation(a) => array_items(&a.values, &mut |item| {
                    if let Val::Str(value) = &item.value
                        && let [class, _] = value.split("::").collect::<Vec<_>>()[..]
                    {
                        names.push(resolve(class));
                    }
                }),
                Value::Generic(value) if !value.is_empty() => {
                    names.push(value.clone());
                    match value.contains("::") {
                        false => {
                            let leading = value.split([' ', '\t', '\n', '\r']).find(|s| !s.is_empty()).unwrap_or_default();
                            if leading != value && is_class_name(leading) {
                                names.push(leading.to_string());
                            }
                        }
                        true => match child.tag.as_deref() {
                            Some("@uses" | "@used-by" | "@see") => {
                                names.push(resolve(value.split("::").next().unwrap_or_default()))
                            }
                            _ => names.push(value.clone()),
                        },
                    }
                }
                _ => {}
            }
        }
        // `INLINE_GENERIC_USES_CLASS_REFERENCE_REGEX`: `{@uses X}`, `{@used-by X}`, `{@see X}`
        let mut inline = Vec::new();
        let mut rest = self.text.as_str();
        while let Some(at) = rest.find("{@") {
            rest = &rest[at + 2..];
            let Some(tag) = ["uses", "used-by", "see"].into_iter().find(|t| rest.starts_with(t)) else {
                continue;
            };
            let after = &rest[tag.len()..];
            let ws = after.len() - after.trim_start_matches([' ', '\t', '\n', '\r', '\x0B', '\x0C']).len();
            if ws == 0 {
                continue;
            }
            let reference: &str = after[ws..].split(|c: char| c == '}' || c.is_ascii_whitespace()).next().unwrap_or_default();
            if reference.is_empty() {
                continue;
            }
            let reference = reference.split('|').next().unwrap_or_default();
            let reference = reference.split("::").next().unwrap_or_default();
            let trimmed = reference.trim_start_matches('\\');
            if !is_class_name(trimmed) {
                continue;
            }
            if reference.starts_with('\\') {
                inline.push(reference.to_string());
                continue;
            }
            let resolved = resolve(reference);
            let both = trimmed.contains('\\') && resolved != trimmed;
            inline.push(resolved);
            if both {
                inline.push(trimmed.to_string());
            }
        }
        for name in inline {
            if !names.contains(&name) {
                names.push(name);
            }
        }
        names
    }

    /// The names `ShortNameResolver::resolveFromStmtsDocBlocks()` reads, in order: the names of the tags that do not
    /// start with a lower-case letter, and every identifier.
    pub fn short_names(&self) -> Vec<String> {
        let idents = self.idents();
        let mut names = Vec::new();
        for (i, child) in self.children.iter().enumerate() {
            if let Some(tag) = &child.tag {
                let name = tag.trim_matches('@');
                if !name.starts_with(|c: char| c.is_ascii_lowercase()) {
                    names.push(name.to_string());
                }
            }
            names.extend(idents.iter().filter(|id| id.child == i).map(|id| id.name.clone()));
        }
        names
    }

    /// Whether every changed child can be printed as Rector prints it.
    pub fn reprintable(&self) -> bool {
        self.children
            .iter()
            .filter(|c| c.changed)
            .all(|c| !c.unprintable && print_child(c).is_some())
    }

    /// Whether a child changed.
    pub fn is_changed(&self) -> bool {
        self.children.iter().any(|c| c.changed)
    }

    /// `PhpDocInfoPrinter::printFormatPreserving()`. Call it only when [`Self::reprintable`] holds.
    pub fn print(&self) -> String {
        let mut current = 0;
        let mut output = String::new();
        let count = self.children.len();
        for (key, child) in self.children.iter().enumerate() {
            // `printDocChildNode()`
            if child.changed {
                current = child.start_end.1;
                let printed = print_child(child).unwrap_or_default();
                if self.single_line {
                    output.push_str(&format!(" {printed}"));
                } else {
                    output.push_str("\n *");
                    if !printed.is_empty() {
                        output.push_str(&format!(" {printed}"));
                    }
                }
                continue;
            }
            let end = child.start_end.1;
            let part = self.add_tokens_from_to(String::new(), current, end, count == key + 1);
            current = end;
            output.push_str(part.trim_end_matches([' ', '\t', '\n', '\r', '\0', '\x0B']));
        }
        // `printEnd()`
        let last = self.last_token_position.unwrap_or(current);
        output = match last {
            0 => output + "\n */",
            _ => self.add_tokens_from_to(output, last, self.tokens.len(), true),
        };
        if !["//", "/**", "/*", "#"].iter().any(|s| output.starts_with(s)) && !output.is_empty() {
            output.insert_str(0, "/**");
        }
        // `NEW_LINE_WITH_SPACE_REGEX`, then the `CALLABLE_REGEX` hotfix
        let output = output.replace(" \r\n", "\r\n").replace(" \n", "\n");
        callable_hotfix(&output)
    }

    /// `addTokensFromTo()` (no child is removed).
    fn add_tokens_from_to(&self, mut output: String, from: usize, to: usize, skip_empty_lines_above: bool) -> String {
        let text = |i: usize| self.tokens.get(i).map_or("", |t| &self.text[t.start..t.end]);
        let mut from = from;
        // include also space before, in case of inlined docs
        if from > 0 && self.tokens[from - 1].ty == t::HORIZONTAL_WS {
            from -= 1;
        }
        // skip extra empty lines above if this is the last one
        if skip_empty_lines_above && text(from).contains('\n') && text(from + 1).contains('\n') {
            from += 1;
        }
        for i in from..to {
            output.push_str(text(i));
        }
        output
    }
}

/// `Strings::replace($phpDocString, '#callable(\s+)\(#', 'callable(')`.
fn callable_hotfix(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(at) = rest.find("callable") {
        let after = &rest[at + 8..];
        let ws = after.len() - after.trim_start().len();
        if ws > 0 && after[ws..].starts_with('(') {
            out.push_str(&rest[..at]);
            out.push_str("callable(");
            rest = &after[ws + 1..];
        } else {
            out.push_str(&rest[..at + 8]);
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// Every `IdentifierTypeNode` of `node` in `PhpDocNodeTraverser` order, `parent_template` for the node that is a
/// template tag's bound itself.
fn walk(node: &mut Node, parent_template: bool, f: &mut dyn FnMut(&mut String, bool)) {
    match node {
        Node::Ident(name) => f(name, parent_template),
        Node::Nullable(inner) | Node::Array(inner) | Node::Projection(_, inner) => walk(inner, false, f),
        Node::Union(types) | Node::Inter(types) => types.iter_mut().for_each(|t| walk(t, false, f)),
        Node::Generic { base, args } => {
            f(base, false);
            args.iter_mut().for_each(|t| walk(t, false, f));
        }
        Node::Callable {
            ident,
            templates,
            params,
            ret,
        } => {
            f(ident, false);
            for template in templates {
                for t in [&mut template.bound, &mut template.lower, &mut template.default].into_iter().flatten() {
                    walk(t, true, f);
                }
            }
            params.iter_mut().for_each(|p| walk(&mut p.ty, false, f));
            walk(ret, false, f);
        }
        Node::ArrayShape { items, unsealed, .. } => {
            for item in items {
                if let Some(ShapeKey::Ident(key)) = &mut item.key {
                    f(key, false);
                }
                walk(&mut item.value, false, f);
            }
            if let Some((key, value)) = unsealed {
                walk(value, false, f);
                if let Some(key) = key {
                    walk(key, false, f);
                }
            }
        }
        Node::ObjectShape(items) => items.iter_mut().for_each(|(_, _, t)| walk(t, false, f)),
        Node::Conditional {
            subject,
            target,
            then,
            otherwise,
            ..
        } => {
            for t in [subject, target, then, otherwise] {
                walk(t, false, f);
            }
        }
        Node::ConditionalForParameter {
            target,
            then,
            otherwise,
            ..
        } => {
            for t in [target, then, otherwise] {
                walk(t, false, f);
            }
        }
        Node::OffsetAccess(base, offset) => {
            walk(base, false, f);
            walk(offset, false, f);
        }
        Node::This | Node::Star(_) | Node::Const(_) => {}
    }
}

/// The classes of the `Class::NAME` types in `node`.
fn const_fetches(node: &Node, f: &mut dyn FnMut(&str)) {
    match node {
        Node::Const(phpdoc::ConstExpr::Fetch(class, _)) if !class.is_empty() => f(class),
        Node::Nullable(inner) | Node::Array(inner) | Node::Projection(_, inner) => const_fetches(inner, f),
        Node::Union(types) | Node::Inter(types) => types.iter().for_each(|t| const_fetches(t, f)),
        Node::Generic { args, .. } => args.iter().for_each(|t| const_fetches(t, f)),
        Node::ArrayShape { items, .. } => items.iter().for_each(|i| const_fetches(&i.value, f)),
        _ => {}
    }
}

/// `RectorAssert::className()`.
fn is_class_name(name: &str) -> bool {
    !name.is_empty()
        && name.split('\\').all(|part| {
            let b = part.as_bytes();
            !b.is_empty()
                && (b[0].is_ascii_alphabetic() || b[0] == b'_' || b[0] >= 0x7f)
                && b.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c >= 0x7f)
        })
}

/// The names of the identifiers in a tag `pun_phpstan` parses but Pun does not reprint.
fn other_idents(tag: &phpdoc::Tag) -> Vec<String> {
    let mut nodes: Vec<Node> = Vec::new();
    let template = |t: &phpdoc::TemplateTag, nodes: &mut Vec<Node>| {
        nodes.extend([&t.bound, &t.lower, &t.default].into_iter().flatten().cloned());
    };
    match &tag.value {
        phpdoc::TagValue::Method {
            ret, params, templates, ..
        } => {
            nodes.extend(ret.iter().cloned());
            for t in templates {
                template(t, &mut nodes);
            }
            nodes.extend(params.iter().filter_map(|p| p.ty.clone()));
        }
        phpdoc::TagValue::TypeAlias(_, Ok(t)) | phpdoc::TagValue::Assert { ty: t, .. } => nodes.push(t.clone()),
        phpdoc::TagValue::TypeAliasImport(_, from, _) => nodes.push(Node::Ident(from.clone())),
        _ => {}
    }
    let mut names = Vec::new();
    for mut node in nodes {
        walk(&mut node, false, &mut |name, _| names.push(name.clone()));
    }
    names
}

/// The value of the tag `name` from `from` to `to` (bytes of the doc), as `DoctrineAnnotationDecorator` leaves it;
/// `None` where Pun does not follow the decorator.
#[allow(clippy::too_many_arguments)]
fn tag_value(
    text: &str,
    tokens: &[parser::Tok],
    name: &str,
    value: parser::Value,
    from: usize,
    to: usize,
    resolve_tag: &dyn Fn(&str) -> String,
    narrow: &dyn Fn(&str) -> Option<String>,
) -> Option<Value> {
    let bare = name.trim_start_matches('@');
    match value {
        parser::Value::Known => {
            // `processDescriptionAsSpacelessPhpDoctagNode()` splits a description over lines with a `@\Class`
            if text[from..to].contains("@\\") && text[from..to].contains('\n') {
                return None;
            }
            if let Some(value) = typed(text, tokens, name, from, to) {
                return Some(value);
            }
            // a value its parser rejects is an `InvalidTagValueNode`, which the decorator makes an annotation of the
            // tag's name; a parsed `@method` and its kin keep their own value node
            let doc = format!("/** {} */", &text[from - name.len()..to]);
            let tag = phpdoc::tags(&doc).and_then(|tags| tags.into_iter().next());
            match tag.as_ref().map(|t| &t.value) {
                Some(phpdoc::TagValue::Invalid(..)) => {
                    let description = generic_value(text, tokens, from, to);
                    process_doctrine(bare, description, None, resolve_tag, narrow)
                }
                _ => Some(Value::Other(tag.map(|t| other_idents(&t)).unwrap_or_default())),
            }
        }
        parser::Value::Invalid(description) => process_doctrine(bare, description, None, resolve_tag, narrow),
        parser::Value::Doctrine { args, description } => {
            process_doctrine(bare, args.join(", "), Some(description), resolve_tag, narrow)
        }
        parser::Value::Generic(value) => {
            // `mergeNestedDoctrineAnnotations()` joins an open value with the tags after it
            if !closed_content(&value) {
                return None;
            }
            let class = resolve_tag(name);
            if !class.contains('\\') && class != "Target" {
                return Some(Value::Generic(value));
            }
            // `resolveFqnAnnotationSpacelessPhpDocTagNode()` splits a value with a `@\Class` into annotations
            if value.contains("@\\") {
                return None;
            }
            annotation(name, value, class, resolve_tag)
        }
    }
}

/// `DoctrineAnnotationDecorator::processDoctrine()`: `values` are the printed arguments of a `DoctrineTagValueNode`
/// (with its `description`) or an `InvalidTagValueNode`'s value.
fn process_doctrine(
    name: &str,
    values: String,
    description: Option<String>,
    resolve_tag: &dyn Fn(&str) -> String,
    narrow: &dyn Fn(&str) -> Option<String>,
) -> Option<Value> {
    // a template is no object type: Rector leaves the tag a `DoctrineTagValueNode`
    let class = narrow(name)?;
    let mut values = values;
    if !values.is_empty() {
        values = star_comment(&values);
        if let Some(description) = description {
            values = format!("({values}){description}");
        }
    }
    annotation(&format!("@{name}"), values, class, resolve_tag)
}

/// `STAR_COMMENT_REGEX`: the `*` that starts a line (white space before it), removed.
fn star_comment(values: &str) -> String {
    values
        .split('\n')
        .map(|line| match line.trim_start().strip_prefix('*') {
            Some(rest) => rest,
            None => line,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// `createDoctrineSpacelessPhpDocTagNode()`; `None` where Rector's parsers never finish reading `content`.
fn annotation(tag: &str, content: String, class: String, resolve_tag: &dyn Fn(&str) -> String) -> Option<Value> {
    let values = parser::annotation_values(&content, resolve_tag).ok()?;
    Some(Value::Annotation(Annotation {
        class,
        identifier: tag.to_string(),
        original: content,
        values,
    }))
}

/// A `GenericTagValueNode`'s value: what follows the tag name, its leading white space aside, its lines joined by
/// `\n`.
fn generic_value(text: &str, tokens: &[parser::Tok], from: usize, to: usize) -> String {
    let mut value = String::new();
    for t in tokens.iter().filter(|t| t.start >= from && t.end <= to) {
        match t.ty {
            t::PHPDOC_EOL => value.push('\n'),
            _ => value.push_str(&text[t.start..t.end]),
        }
    }
    if let Some(t) = tokens.iter().find(|t| t.start < from && t.end > from) {
        value.insert_str(0, &text[from..t.end]);
    }
    value.trim_matches([' ', '\t']).trim_end_matches('\n').to_string()
}

/// `DoctrineAnnotationDecorator::isClosedContent()` (phpdoc-parser's tokens holding a bracket, counted).
fn closed_content(value: &str) -> bool {
    let (mut open, mut close) = (0, 0);
    for token in parser::tokenize(value) {
        let token = &value[token.start..token.end];
        open += usize::from(token.contains(['(', '{']));
        close += usize::from(token.contains([')', '}']));
    }
    open == close
}

/// The identifiers of the annotations nested in `values`, each before its own values.
fn nested_identifiers(values: &[ArrayItem], f: &mut dyn FnMut(&str)) {
    for item in values {
        for (_, v) in item_vals(item) {
            match v {
                Val::Curly(items) | Val::List(items) => nested_identifiers(items, f),
                Val::Annotation(nested) => {
                    f(&nested.identifier);
                    nested_identifiers(&nested.values, f);
                }
                _ => {}
            }
        }
    }
}

/// [`nested_identifiers`], to replace them.
fn nested_identifiers_mut(values: &mut [ArrayItem], f: &mut dyn FnMut(&mut String)) {
    for item in values {
        let key = match &mut item.key {
            Some(Key::Val(key)) => Some(key),
            _ => None,
        };
        for v in std::iter::once(&mut item.value).chain(key) {
            match v {
                Val::Curly(items) | Val::List(items) => nested_identifiers_mut(items, f),
                Val::Annotation(nested) => {
                    f(&mut nested.identifier);
                    nested_identifiers_mut(&mut nested.values, f);
                }
                _ => {}
            }
        }
    }
}

/// The value of `item`, then its key (`ArrayItemNode`'s properties in order).
fn item_vals(item: &ArrayItem) -> impl Iterator<Item = (bool, &Val)> {
    let key = match &item.key {
        Some(Key::Val(key)) => Some((true, key)),
        _ => None,
    };
    std::iter::once((false, &item.value)).chain(key)
}

/// Whether a nested annotation in `values` is one `is` accepts, the path to it then in `path`.
fn locate(values: &[ArrayItem], is: &dyn Fn(&str, &str) -> bool, path: &mut Vec<(usize, bool)>) -> bool {
    for (i, item) in values.iter().enumerate() {
        for (in_key, v) in item_vals(item) {
            path.push((i, in_key));
            let found = match v {
                Val::Annotation(nested) => is(&nested.identifier, &nested.class) || locate(&nested.values, is, path),
                Val::Curly(items) | Val::List(items) => locate(items, is, path),
                _ => false,
            };
            if found {
                return true;
            }
            path.pop();
        }
    }
    false
}

/// The nested annotation at `path` in `values`: the class it resolves to and its values.
fn navigate<'a>(values: &'a mut [ArrayItem], path: &[(usize, bool)]) -> (&'a str, &'a mut Vec<ArrayItem>) {
    let (i, in_key) = path[0];
    let item = &mut values[i];
    let v = match (in_key, &mut item.key) {
        (true, Some(Key::Val(key))) => key,
        _ => &mut item.value,
    };
    match (v, path.len()) {
        (Val::Annotation(nested), 1) => (&nested.class, &mut nested.values),
        (Val::Annotation(nested), _) => navigate(&mut nested.values, &path[1..]),
        (Val::Curly(items) | Val::List(items), _) => navigate(items, &path[1..]),
        _ => unreachable!("a path to an annotation"),
    }
}

/// Every `ArrayItemNode` in `values`, nested ones included.
fn array_items(values: &[ArrayItem], f: &mut dyn FnMut(&ArrayItem)) {
    for item in values {
        f(item);
        match &item.value {
            Val::Curly(items) | Val::List(items) => array_items(items, f),
            Val::Annotation(nested) => array_items(&nested.values, f),
            _ => {}
        }
    }
}

/// `(string) $phpDocChildNode` of a changed child; `None` where Pun does not print it.
fn print_child(child: &Child) -> Option<String> {
    let name = child.tag.as_deref()?;
    match &child.value {
        Value::Typed(typed) => {
            let ty = |i: usize| -> Option<String> { Some(TypeNode::from_parsed(&typed.types[i])?.to_string()) };
            let description = &typed.description;
            let value = match &typed.form {
                Form::Param {
                    reference,
                    variadic,
                    variable,
                } => format!(
                    "{} {}{}{variable} {description}",
                    ty(0)?,
                    if *reference { "&" } else { "" },
                    if *variadic { "..." } else { "" },
                ),
                Form::Var(variable) => format!("{} {}", ty(0)?, format!("{variable} {description}").trim()),
                Form::Plain => format!("{} {description}", ty(0)?),
                Form::Named(variable) => format!("{} {variable} {description}", ty(0)?),
                Form::Template { name, preposition } => {
                    let bound = match typed.types.is_empty() {
                        true => String::new(),
                        false => format!(" {preposition} {}", ty(0)?),
                    };
                    format!("{name}{bound} {description}")
                }
            };
            Some(format!("{name} {}", value.trim()).trim().to_string())
        }
        // `SpacelessPhpDocTagNode`: the name, then `DoctrineAnnotationTagValueNode::__toString()` (always changed)
        Value::Annotation(a) => Some(match (a.values.is_empty(), a.original == "()") {
            (false, _) => format!("{name}({})", parser::join(&a.values)),
            (true, true) => format!("{name}()"),
            (true, false) => name.to_string(),
        }),
        _ => None,
    }
}

/// The value of a typed tag from `from` to `to` (bytes of the doc), `None` for a tag Pun does not read this way or a
/// value its parser rejects.
fn typed(text: &str, tokens: &[parser::Tok], name: &str, from: usize, to: usize) -> Option<Value> {
    let value = &text[from..to];
    let skip = |at: usize| at + value[at..].len() - value[at..].trim_start_matches([' ', '\t']).len();
    let preceded_by_ws = |at: usize| at > 0 && matches!(value.as_bytes()[at - 1], b' ' | b'\t');
    let variable_len = |s: &str| {
        let b = s.as_bytes();
        (b.first() == Some(&b'$') && b.get(1).is_some_and(|c| c.is_ascii_alphabetic() || *c == b'_' || *c >= 0x80))
            .then(|| 1 + b[1..].iter().take_while(|c| c.is_ascii_alphanumeric() || **c == b'_' || **c >= 0x80).count())
    };
    let is_this = |s: &str| {
        s.strip_prefix("$this").is_some_and(|after| {
            !after.bytes().next().is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_' || c >= 0x80)
        })
    };
    let leading = skip(0);
    let rest = &value[leading..];
    let mut types = Vec::new();
    let mut limit_start = false;
    let (form, mut at) = if TEMPLATE.contains(&name) {
        // `parseTemplateTagValue()`: a name, then `of` or `as` and the bound
        let name_len = rest
            .bytes()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c == b'\\' || *c == b'-' || *c >= 0x80)
            .count();
        if name_len == 0 {
            return None;
        }
        let template_name = rest[..name_len].to_string();
        let mut at = skip(leading + name_len);
        let mut preposition = "of".to_string();
        for word in ["of", "as"] {
            let after = &value[at..];
            if after.starts_with(word) && after[2..].starts_with([' ', '\t']) {
                preposition = word.to_string();
                let bound_from = skip(at + 2);
                let (parsed, offset) = phpdoc::parse_prefix(&value[bound_from..]).ok()?;
                types.push(parsed);
                at = bound_from + value[bound_from..bound_from + offset].trim_end_matches([' ', '\t']).len();
                break;
            }
        }
        // a lower bound or a default Pun does not read
        let next = &value[skip(at)..];
        if next.starts_with("super ") || next.starts_with('=') {
            return None;
        }
        (
            Form::Template {
                name: template_name,
                preposition,
            },
            at,
        )
    } else if PARAM.contains(&name) || VAR.contains(&name) || PLAIN.contains(&name) || NAMED.contains(&name) {
        // a typeless `@param $a`
        if PARAM.contains(&name) && ((rest.starts_with('$') && !is_this(rest)) || rest.starts_with('&') || rest.starts_with("...")) {
            return None;
        }
        let (parsed, offset) = phpdoc::parse_prefix(rest).ok()?;
        types.push(parsed);
        let mut at = leading + rest[..offset].trim_end_matches([' ', '\t']).len();
        let form = if PARAM.contains(&name) {
            at = skip(at);
            let reference = value[at..].starts_with('&');
            if reference {
                at = skip(at + 1);
            }
            let variadic = value[at..].starts_with("...");
            if variadic {
                at = skip(at + 3);
            }
            // `parseRequiredVariableName()`: a `TOKEN_VARIABLE`, which `$this` is not
            let len = variable_len(&value[at..]).filter(|_| !is_this(&value[at..]))?;
            let variable = value[at..at + len].to_string();
            at += len;
            Form::Param {
                reference,
                variadic,
                variable,
            }
        } else if VAR.contains(&name) {
            let next = skip(at);
            match variable_len(&value[next..]) {
                Some(len) => {
                    let variable = value[next..next + len].to_string();
                    at = next + len;
                    Form::Var(variable)
                }
                None => {
                    limit_start = true;
                    Form::Var(String::new())
                }
            }
        } else if NAMED.contains(&name) {
            at = skip(at);
            let len = match is_this(&value[at..]) {
                true => 5,
                false => variable_len(&value[at..])?,
            };
            let variable = value[at..at + len].to_string();
            at += len;
            Form::Named(variable)
        } else {
            limit_start = true;
            Form::Plain
        };
        (form, at)
    } else {
        return None;
    };
    // `parseOptionalDescription($tokens, $limitStartToken)`
    let next = skip(at);
    if limit_start {
        let starts = &value[next..];
        if next < value.len() && !starts.starts_with(['\r', '\n']) && !preceded_by_ws(next) {
            return None;
        }
    }
    at = at.min(value.len());
    // the description's lines joined by "\n", then `BetterPhpDocParser` puts ` * ` after each line break
    let description = generic_value(text, tokens, from + at, to).replace('\n', "\n * ");
    Some(Value::Typed(Typed {
        form,
        types,
        description,
        replaced: Vec::new(),
    }))
}
