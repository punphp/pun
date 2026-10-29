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

use crate::rules::phpdoc_writer::doc_info::{Kind, child_ranges, lex, tag_name};
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
}

/// One argument of a Doctrine annotation, as Rector's `ArrayParser` makes its `ArrayItemNode`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Item {
    pub key: Option<String>,
    pub value: ItemValue,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ItemValue {
    /// `StringNode`: a `"..."` value, its quotes off (`""` read as `"`).
    String(String),
    /// Any other value as written: a name, a constant, a number, `true`, `false`, `null`.
    Plain(String),
}

/// A Doctrine annotation: Rector's `SpacelessPhpDocTagNode` with a `DoctrineAnnotationTagValueNode`.
#[derive(Clone, Debug)]
pub(crate) struct Annotation {
    /// The class `ClassAnnotationMatcher` resolves the tag name to (`RESOLVED_CLASS`).
    pub class: String,
    /// The arguments, `None` where Pun does not read them (a `{...}` list, a nested annotation, a multi-line value,
    /// a description after the `)`).
    pub items: Option<Vec<Item>>,
    /// Whether the tag has parentheses (phpdoc-parser's `DoctrineTagValueNode`) rather than being a generic tag
    /// whose name is a class.
    parenthesized: bool,
    /// The names inside the arguments `UnusedImportRemovingPostRector` reads: nested annotations' names, and the
    /// classes of `Class::NAME` values.
    pub inner_names: Vec<String>,
}

/// What a child of the doc is.
#[derive(Clone, Debug)]
pub(crate) enum Value {
    /// `PhpDocTextNode`: its text.
    Text(String),
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
    tokens: Vec<(Kind, usize, usize)>,
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
/// The other tags `PhpDocParser::parseTagValue()` gives a value node other than a generic one.
const OTHER: &[&str] = &[
    "@param-immediately-invoked-callable",
    "@phpstan-param-immediately-invoked-callable",
    "@param-later-invoked-callable",
    "@phpstan-param-later-invoked-callable",
    "@pure-unless-callable-is-impure",
    "@phpstan-pure-unless-callable-is-impure",
    "@pure-unless-parameter-passed",
    "@phpstan-pure-unless-parameter-passed",
    "@deprecated",
    "@method",
    "@phpstan-method",
    "@psalm-method",
    "@phan-method",
    "@phpstan-type",
    "@psalm-type",
    "@phan-type",
    "@phpstan-import-type",
    "@psalm-import-type",
    "@phpstan-assert",
    "@phpstan-assert-if-true",
    "@phpstan-assert-if-false",
    "@psalm-assert",
    "@psalm-assert-if-true",
    "@psalm-assert-if-false",
    "@phan-assert",
    "@phan-assert-if-true",
    "@phan-assert-if-false",
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
    /// Read the doc comment `text`; `resolve_tag` is `ClassAnnotationMatcher::resolveTagFullyQualifiedName()` at the
    /// node the doc belongs to. `None` where Pun does not follow how phpdoc-parser and Rector's decorators draw the
    /// children: a `@\Class` annotation, a token over a line break, a generic tag whose brackets stay open.
    pub fn parse(text: &str, resolve_tag: &dyn Fn(&str) -> String) -> Option<Doc> {
        if text.contains("@\\") {
            return None;
        }
        let tokens = lex(text);
        let mut children = Vec::new();
        for (start, end) in child_ranges(text, &tokens) {
            let (tag, value) = child_value(text, &tokens, start, end, resolve_tag)?;
            children.push(Child {
                start_end: (start, end),
                tag,
                value,
                changed: false,
                unprintable: false,
            });
        }
        Some(Doc {
            text: text.to_string(),
            single_line: !tokens.iter().any(|t| t.0 == Kind::Eol),
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
                    for t in &typed.types {
                        let mut t = t.clone();
                        walk(&mut t, template, &mut |name, parent| push(name, parent));
                    }
                }
                Value::Annotation(annotation) => {
                    // the `IdentifierTypeNode` of the annotation's name
                    push(c.tag.as_deref().unwrap_or_default(), false);
                    let _ = annotation;
                }
                Value::Other(names) => names.iter().for_each(|n| push(n, false)),
                Value::Text(_) | Value::Generic(_) => {}
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
            }
            // the annotation's name is printed from the tag, never from its identifier
            _ => child.unprintable = true,
        }
    }

    /// Mark the child `child` changed (a value inside a Doctrine annotation was set).
    pub fn set_changed(&mut self, child: usize) {
        let child = &mut self.children[child];
        child.changed = true;
        if let Value::Annotation(annotation) = &child.value
            && (annotation.items.is_none() || !annotation.parenthesized)
        {
            child.unprintable = true;
        }
    }

    /// `hasByTypes(NodeTypes::TYPE_AWARE_NODES)`.
    pub fn has_type_aware(&self) -> bool {
        self.children.iter().any(is_type_aware)
    }

    /// The first child that is an annotation of one of `classes` (`DoctrineAnnotationTagValueNode::hasClassName()`:
    /// its name without `@`, or the class it resolves to).
    pub fn annotation(&self, classes: &[&str]) -> Option<usize> {
        self.children.iter().position(|c| match &c.value {
            Value::Annotation(a) => {
                let name = c.tag.as_deref().unwrap_or_default().trim_matches('@');
                classes.iter().any(|class| *class == name || *class == a.class)
            }
            _ => false,
        })
    }

    /// The arguments of the annotation child `child`, `None` where Pun does not read them.
    pub fn items_mut(&mut self, child: usize) -> Option<&mut Vec<Item>> {
        match &mut self.children[child].value {
            Value::Annotation(a) => a.items.as_mut(),
            _ => None,
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
                Value::Annotation(a) => {
                    for inner in &a.inner_names {
                        match inner.strip_prefix('@') {
                            Some(nested) => names.push(nested.to_string()),
                            None => names.push(resolve(inner)),
                        }
                    }
                }
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
            if let Value::Annotation(a) = &child.value {
                names.extend(a.inner_names.iter().filter(|n| n.starts_with('@')).cloned());
            }
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
        let text = |i: usize| self.tokens.get(i).map_or("", |t| &self.text[t.1..t.2]);
        let mut from = from;
        // include also space before, in case of inlined docs
        if from > 0 && self.tokens[from - 1].0 == Kind::Ws {
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

/// The tag name and value of the child `[start, end)`; `None` where Pun does not follow the decorator.
fn child_value(
    text: &str,
    tokens: &[(Kind, usize, usize)],
    start: usize,
    end: usize,
    resolve_tag: &dyn Fn(&str) -> String,
) -> Option<(Option<String>, Value)> {
    let from = tokens[start].1;
    let to = tokens[end].1;
    let first = &text[tokens[start].1..tokens[start].2];
    let Some(name) = (tokens[start].0 == Kind::Other).then(|| tag_name(first)).flatten() else {
        // a text node; one with `})` is merged into the generic tag before it (`mergeNestedDoctrineAnnotations()`)
        let child_text = joined(text, tokens, start, end);
        if child_text.contains("})") {
            return None;
        }
        return Some((None, Value::Text(child_text)));
    };
    let value_from = from + name.len();
    let rest = &text[value_from..to];
    let tag = Some(name.to_string());
    if let Some(value) = typed(text, tokens, name, value_from, to) {
        return Some((tag, value));
    }
    if OTHER.contains(&name) || PARAM.contains(&name) || VAR.contains(&name) || PLAIN.contains(&name)
        || NAMED.contains(&name) || TEMPLATE.contains(&name)
    {
        // a value its parser rejects is an `InvalidTagValueNode`, which the decorator makes a Doctrine annotation of
        // the tag's name; a parsed `@method` and its kin keep their own value node
        let doc = format!("/** {} */", &text[from..to]);
        let names = phpdoc::tags(&doc)
            .and_then(|tags| tags.into_iter().next())
            .map(|tag| match &tag.value {
                phpdoc::TagValue::Invalid(..) => vec![name.to_string()],
                _ => other_idents(&tag),
            })
            .unwrap_or_default();
        return Some((tag, Value::Other(names)));
    }
    let trimmed = rest.trim_start_matches([' ', '\t']);
    if trimmed.starts_with('(') {
        // phpdoc-parser's `DoctrineTagValueNode`, which the decorator always takes for an annotation
        let class = resolve_tag(name);
        return Some((tag, Value::Annotation(annotation(text, tokens, start, end, value_from, class, true))));
    }
    let value = generic_value(text, tokens, value_from, to);
    // `transformGenericTagValueNodesToDoctrineAnnotationTagValueNodes()`
    let class = resolve_tag(name);
    if class.contains('\\') || class == "Target" {
        if !closed_content(&value) {
            return None;
        }
        return Some((tag, Value::Annotation(annotation(text, tokens, start, end, value_from, class, false))));
    }
    if !closed_content(&value) {
        return None;
    }
    Some((tag, Value::Generic(value)))
}

/// The text of tokens `[start, end)`, each line break as `\n`.
fn joined(text: &str, tokens: &[(Kind, usize, usize)], start: usize, end: usize) -> String {
    tokens[start..end]
        .iter()
        .map(|t| match t.0 {
            Kind::Eol => "\n",
            _ => &text[t.1..t.2],
        })
        .collect()
}

/// A `GenericTagValueNode`'s value: what follows the tag name, its leading white space aside, its lines joined by
/// `\n`.
fn generic_value(text: &str, tokens: &[(Kind, usize, usize)], from: usize, to: usize) -> String {
    let mut value = String::new();
    for t in tokens.iter().filter(|t| t.1 >= from && t.2 <= to) {
        match t.0 {
            Kind::Eol => value.push('\n'),
            _ => value.push_str(&text[t.1..t.2]),
        }
    }
    if let Some(t) = tokens.iter().find(|t| t.1 < from && t.2 > from) {
        value.insert_str(0, &text[from..t.2]);
    }
    value.trim_matches([' ', '\t']).trim_end_matches('\n').to_string()
}

/// `Lexer::generateRegexp()`'s alternatives in order (phpdoc-parser 2.3; PCRE alternation: the first that matches
/// wins). Only the token values matter here, so the types are left out.
const LEXER_PATTERNS: &[&str] = &[
    r"[\x09\x20]++",
    r"(?:[\\]?+[a-z_\x80-\xFF][0-9a-z_\x80-\xFF-]*+)++",
    r"\$this(?![0-9a-z_\x80-\xFF])",
    r"\$[a-z_\x80-\xFF][0-9a-z_\x80-\xFF]*+",
    r"&(?=\s*+(?:[.,=)]|(?:\$(?!this(?![0-9a-z_\x80-\xFF])))))",
    r"\|",
    "&",
    r"\?",
    "!",
    r"\(",
    r"\)",
    "<",
    ">",
    r"\[",
    r"\]",
    r"\{",
    r"\}",
    ",",
    r"\/\/[^\r\n]*(?=\n|\r|\*/)",
    r"\.\.\.",
    "::",
    "=>",
    "->",
    "=",
    ":",
    r"/\*\*(?=\s)\x20?+",
    r"\*/",
    r"@(?:[a-z][a-z0-9-\\]+:)?[a-z][a-z0-9-\\]*+",
    r"@[a-z_\\][a-z0-9_\:\\]*[a-z_][a-z0-9_]*",
    r"\r?+\n[\x09\x20]*+(?:\*(?!/)\x20?+)?",
    r"[+\-]?(?:(?:[0-9]++(?:_[0-9]++)*\.[0-9]*+(?:_[0-9]++)*(?:e[+\-]?[0-9]++(?:_[0-9]++)*)?)|(?:[0-9]*+(?:_[0-9]++)*\.[0-9]++(?:_[0-9]++)*(?:e[+\-]?[0-9]++(?:_[0-9]++)*)?)|(?:[0-9]++(?:_[0-9]++)*e[+\-]?[0-9]++(?:_[0-9]++)*))",
    r"[+\-]?(?:(?:0b[0-1]++(?:_[0-1]++)*)|(?:0o[0-7]++(?:_[0-7]++)*)|(?:0x[0-9a-f]++(?:_[0-9a-f]++)*)|(?:[0-9]++(?:_[0-9]++)*))",
    r"'(?:\\[^\r\n]|[^'\r\n\\])*+'",
    r#""(?:\\[^\r\n]|[^"\r\n\\])*+""#,
    r#""(?:""|[^"])*+""#,
    r"\*",
    r"(?:(?!\*/)[^\s])++",
];

/// phpdoc-parser's `Lexer::tokenize()`, the token values without `TOKEN_END`. Like `preg_match_all` with the `A`
/// modifier, lexing stops at the first byte no alternative matches (a lone `\r`, a vertical tab).
fn lexer_values(input: &str) -> Vec<&str> {
    use std::sync::OnceLock;
    static RE: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        pcre2::bytes::RegexBuilder::new()
            .caseless(true)
            .dotall(true)
            .build(&format!(r"\G(?:{})", LEXER_PATTERNS.join("|")))
            .expect("phpdoc lexer regex")
    });
    let mut values = Vec::new();
    let mut offset = 0;
    while let Some(found) = re.find_at(input.as_bytes(), offset).ok().flatten() {
        values.push(&input[found.start()..found.end()]);
        offset = found.end();
    }
    values
}

/// `DoctrineAnnotationDecorator::isClosedContent()` (phpdoc-parser's tokens holding a bracket, counted).
fn closed_content(value: &str) -> bool {
    let (mut open, mut close) = (0, 0);
    for token in lexer_values(value) {
        open += usize::from(token.contains(['(', '{']));
        close += usize::from(token.contains([')', '}']));
    }
    open == close
}

/// A Doctrine annotation child: its arguments read where they are flat and on one line.
fn annotation(
    text: &str,
    tokens: &[(Kind, usize, usize)],
    start: usize,
    end: usize,
    value_from: usize,
    class: String,
    parenthesized: bool,
) -> Annotation {
    let value = generic_value(text, tokens, value_from, tokens[end].1);
    let _ = start;
    let inner_names = inner_names(&value);
    let items = match parenthesized {
        true => flat_items(&value),
        false => Some(Vec::new()),
    };
    Annotation {
        class,
        items,
        parenthesized,
        inner_names,
    }
}

/// The names of the annotations nested in an annotation's arguments, and the classes of its `Class::NAME` values.
fn inner_names(value: &str) -> Vec<String> {
    let mut names = Vec::new();
    let b = value.as_bytes();
    let word = |i: usize| {
        i + b[i..]
            .iter()
            .take_while(|c| c.is_ascii_alphanumeric() || **c == b'_' || **c == b'\\' || **c >= 0x80)
            .count()
    };
    let mut i = 0;
    let mut in_string = false;
    while i < b.len() {
        match b[i] {
            b'"' => in_string = !in_string,
            b'@' if !in_string => {
                let end = word(i + 1);
                if end > i + 1 {
                    names.push(format!("@{}", &value[i + 1..end]));
                }
                i = end;
                continue;
            }
            c if !in_string && (c.is_ascii_alphabetic() || c == b'_' || c == b'\\') => {
                let end = word(i);
                if value[end..].starts_with("::") {
                    names.push(value[i..end].to_string());
                }
                i = end;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    names
}

/// The arguments of `(...)` as `ArrayParser` makes them, where each is `key=value` or a value, the value a `"..."`
/// string or a plain word on the one line; `None` otherwise.
fn flat_items(value: &str) -> Option<Vec<Item>> {
    let inner = value.strip_prefix('(')?.trim_end().strip_suffix(')')?;
    if inner.contains(['\n', '{', '}', '@', '(', ')']) {
        return None;
    }
    let mut items = Vec::new();
    let mut rest = inner.trim();
    while !rest.is_empty() {
        // a key: a word followed by `=`
        let mut key = None;
        let word_len = rest
            .bytes()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == b'_')
            .count();
        if word_len > 0 && rest[word_len..].trim_start().starts_with('=') {
            key = Some(rest[..word_len].to_string());
            rest = rest[word_len..].trim_start()[1..].trim_start();
        }
        let (value, after) = if let Some(quoted) = rest.strip_prefix('"') {
            // `""` stands for one `"`
            let mut end = None;
            let qb = quoted.as_bytes();
            let mut i = 0;
            while i < qb.len() {
                if qb[i] == b'"' {
                    if qb.get(i + 1) == Some(&b'"') {
                        i += 2;
                        continue;
                    }
                    end = Some(i);
                    break;
                }
                i += 1;
            }
            let end = end?;
            (ItemValue::String(quoted[..end].replace("\"\"", "\"")), &quoted[end + 1..])
        } else {
            let len = rest.find(',').unwrap_or(rest.len());
            let plain = rest[..len].trim();
            if plain.is_empty() || plain.contains([' ', '\t', '\'', '=']) {
                return None;
            }
            (ItemValue::Plain(plain.to_string()), &rest[len..])
        };
        items.push(Item { key, value });
        let after = after.trim_start();
        rest = match after.strip_prefix(',') {
            Some(next) => next.trim_start(),
            None if after.is_empty() => after,
            None => return None,
        };
    }
    Some(items)
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
        Value::Annotation(Annotation {
            items: Some(items),
            parenthesized: true,
            ..
        }) => {
            // `SpacelessPhpDocTagNode`: the name, then the annotation's `(%s)` of its items
            let printed: Vec<String> = items
                .iter()
                .map(|item| {
                    let value = match &item.value {
                        ItemValue::String(s) => format!("\"{}\"", s.replace('"', "\"\"")),
                        ItemValue::Plain(p) => match p.to_ascii_lowercase().as_str() {
                            "true" | "false" => p.to_ascii_lowercase(),
                            "null" => String::new(),
                            _ => p.clone(),
                        },
                    };
                    match &item.key {
                        Some(key) => format!("{key}={value}"),
                        None => value,
                    }
                })
                .collect();
            Some(match printed.is_empty() {
                true => name.to_string(),
                false => format!("{name}({})", printed.join(", ")),
            })
        }
        _ => None,
    }
}

/// The value of a typed tag from `from` to `to` (bytes of the doc), `None` for a tag Pun does not read this way or a
/// value its parser rejects.
fn typed(text: &str, tokens: &[(Kind, usize, usize)], name: &str, from: usize, to: usize) -> Option<Value> {
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
    }))
}
