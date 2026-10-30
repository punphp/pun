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
    /// The strings written in the value, each as printed ([`strings_of`]).
    strings: Vec<(Vec<u8>, Option<String>)>,
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

impl Annotation {
    /// `getOriginalContent()`.
    pub fn original(&self) -> &str {
        &self.original
    }

    /// `DoctrineAnnotationTagValueNode::__toString()` (the node is always changed): the values in parentheses, or
    /// the empty parentheses as written.
    pub fn values_string(&self) -> String {
        match self.values.is_empty() {
            false => format!("({})", parser::join(&self.values)),
            // empty brackets
            true => ["", "()"][usize::from(self.original == "()")].to_string(),
        }
    }
}

/// What a child of the doc is.
#[derive(Clone, Debug)]
pub(crate) enum Value {
    /// `PhpDocTextNode`: its text.
    Text(String),
    Typed(Typed),
    /// `GenericTagValueNode`: its value.
    Generic(String),
    /// `DoctrineTagValueNode` whose name is no class (a template): its arguments as phpdoc-parser prints them, and
    /// its description.
    Doctrine {
        args: Vec<parser::Printed>,
        description: String,
    },
    /// `InvalidTagValueNode` whose name is no class: its value.
    Invalid(String),
    Annotation(Annotation),
    /// Any other tag phpdoc-parser gives a value node of its own (`@method`, `@phpstan-type`,
    /// `@phpstan-import-type`, `@phpstan-assert`, a typeless `@param`...).
    Other(Other),
}

/// A tag value `pun_phpstan` parses, with what its node prints that `pun_phpstan` leaves out.
#[derive(Clone, Debug)]
pub(crate) struct Other {
    value: phpdoc::TagValue,
    /// The `description` property, for a value node that has one.
    description: Option<String>,
    /// `TemplatePhpDocNodeVisitor::resolvePreposition()` of each `@method` template.
    prepositions: Vec<String>,
    /// Each string in the `@method` parameters' defaults as printed, in order.
    quotes: Vec<String>,
    strings: Vec<(Vec<u8>, Option<String>)>,
}

impl Other {
    /// The tag's `IdentifierTypeNode`s in `PhpDocNodeTraverser` order, with whether the parent of each is a template
    /// (a `@method` template is a `SpacingAwareTemplateTagValueNode` without its lower bound and default).
    fn idents_mut(&mut self, f: &mut dyn FnMut(&mut String, bool)) {
        match &mut self.value {
            phpdoc::TagValue::Method {
                ret,
                params,
                templates,
                ..
            } => {
                if let Some(ret) = ret {
                    walk(ret, false, f);
                }
                for bound in templates.iter_mut().filter_map(|t| t.bound.as_mut()) {
                    walk(bound, true, f);
                }
                for ty in params.iter_mut().filter_map(|p| p.ty.as_mut()) {
                    walk(ty, false, f);
                }
            }
            phpdoc::TagValue::TypeAlias(_, Ok(ty)) | phpdoc::TagValue::Assert { ty, .. } => {
                walk(ty, false, f)
            }
            phpdoc::TagValue::TypeAliasImport(_, from, _) => f(from, false),
            _ => {}
        }
    }

    /// The value node's `__toString()`.
    fn print(&self) -> Option<String> {
        let quote = |v: &[u8]| quote_of(&self.strings, v);
        let ty = |node: &Node| TypeNode::from_parsed_all(node, &quote).map(|t| t.to_string());
        let description = self.description.as_deref().unwrap_or_default();
        Some(match &self.value {
            phpdoc::TagValue::Method {
                is_static,
                ret,
                name,
                params,
                templates,
            } => {
                let mut quotes = self.quotes.iter();
                let mut printed = Vec::new();
                // `MethodTagValueParameterNode`
                for p in params {
                    let mut param = match &p.ty {
                        Some(t) => format!("{} ", ty(t)?),
                        None => String::new(),
                    };
                    param.push_str(if p.by_ref { "&" } else { "" });
                    param.push_str(if p.variadic { "..." } else { "" });
                    param.push_str(&p.name);
                    if let Some(default) = &p.default {
                        param.push_str(&format!(" = {}", const_expr(default, &mut quotes)?));
                    }
                    printed.push(param);
                }
                // `SpacingAwareTemplateTagValueNode`
                let mut generics = Vec::new();
                for (t, preposition) in templates.iter().zip(&self.prepositions) {
                    generics.push(match &t.bound {
                        Some(bound) => format!("{} {preposition} {}", t.name, ty(bound)?),
                        None => t.name.clone(),
                    });
                }
                format!(
                    "{}{}{name}{}({}){}",
                    if *is_static { "static " } else { "" },
                    match ret {
                        Some(ret) => format!("{} ", ty(ret)?),
                        None => String::new(),
                    },
                    match generics.is_empty() {
                        true => String::new(),
                        false => format!("<{}>", generics.join(", ")),
                    },
                    printed.join(", "),
                    match description.is_empty() {
                        true => String::new(),
                        false => format!(" {description}"),
                    },
                )
            }
            phpdoc::TagValue::TypeAlias(alias, Ok(t)) => format!("{alias} {}", ty(t)?),
            phpdoc::TagValue::TypeAliasImport(alias, from, as_) => match as_ {
                Some(as_) => format!("{alias} from {from} as {as_}"),
                None => format!("{alias} from {from}"),
            },
            phpdoc::TagValue::Assert {
                ty: t,
                parameter,
                member,
                negated,
                equality,
            } => format!(
                "{}{}{} {parameter}{} {description}",
                if *negated { "!" } else { "" },
                if *equality { "=" } else { "" },
                ty(t)?,
                match member {
                    Some((property, false)) => format!("->{property}"),
                    Some((method, true)) => format!("->{method}()"),
                    None => String::new(),
                },
            ),
            phpdoc::TagValue::Param {
                ty: None,
                by_ref,
                variadic,
                name,
            } => {
                format!(
                    "{}{}{name} {description}",
                    if *by_ref { "&" } else { "" },
                    if *variadic { "..." } else { "" }
                )
            }
            phpdoc::TagValue::ParamName(name) => format!("{name} {description}"),
            // `@deprecated` (no other value has an identifier that changes)
            _ => description.to_string(),
        })
    }
}

/// `ConstExprNode::__toString()`; `quotes` gives each string as printed.
fn const_expr(
    expr: &phpdoc::ConstExpr,
    quotes: &mut std::slice::Iter<'_, String>,
) -> Option<String> {
    use phpdoc::ConstExpr as C;
    Some(match expr {
        C::Int(v) | C::Float(v) => v.clone(),
        C::True => "true".into(),
        C::False => "false".into(),
        C::Null => "null".into(),
        C::Fetch(class, name) if class.is_empty() => name.clone(),
        C::Fetch(class, name) => format!("{class}::{name}"),
        C::Str(_) => quotes.next()?.clone(),
        C::Array(items) => {
            let mut printed = Vec::new();
            for (key, value) in items {
                printed.push(match key {
                    Some(key) => format!(
                        "{} => {}",
                        const_expr(key, quotes)?,
                        const_expr(value, quotes)?
                    ),
                    None => const_expr(value, quotes)?,
                });
            }
            format!("[{}]", printed.join(", "))
        }
    })
}

#[derive(Clone, Debug)]
pub(crate) struct Child {
    /// `START_AND_END`: the first token and the token after the last; `None` for a node the decorator made.
    start_end: Option<(usize, usize)>,
    /// The node object's identity.
    id: usize,
    /// The tag's name, its `@` included; `None` for text.
    pub tag: Option<String>,
    pub value: Value,
    /// Whether a node inside was replaced (`ChangedPhpDocNodeVisitor`): the child is reprinted.
    pub changed: bool,
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
    /// The key of each child in PHP's array of them (`unset()` leaves gaps).
    keys: Vec<usize>,
    /// The children that went (`PhpDocTagRemover`, a `NODE_REMOVE` traversal) or were replaced, with their keys.
    removed: Vec<(usize, Child)>,
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
        || [
            "@return",
            "@phpstan-return",
            "@psalm-return",
            "@phan-return",
            "@phan-real-return",
        ]
        .contains(&name)
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
        // where the type ends, or the token `TypeParser` throws at (the offset its message gives)
        let type_end = |start: usize| match phpdoc::parse_prefix(&text[start..]) {
            Ok((_, offset)) => Ok(start + offset),
            Err(error) => Err(start + error_offset(&error.message())),
        };
        let raw = parser::DocParser::new(text, &tokens, &type_end).parse_with_node();
        let mut slots = Slots::default();
        for child in raw {
            let (tag, value) = match child.kind {
                parser::Kind::Text(text) => (None, Value::Text(text)),
                parser::Kind::Tag { name, value, .. } => {
                    let from = tokens[child.start].start + name.len();
                    let to = tokens[child.end].start;
                    let value = tag_value(text, &tokens, &name, value, from, to)?;
                    (Some(name), value)
                }
            };
            let child = slots.child(Some((child.start, child.end)), tag, value);
            let key = slots.items.len();
            slots.items.push((key, child));
        }
        merge_nested_doctrine_annotations(&mut slots);
        transform(&mut slots, resolve_tag, narrow)?;
        let (keys, children): (Vec<usize>, Vec<Child>) = slots.items.into_iter().unzip();
        Some(Doc {
            text: text.to_string(),
            single_line: !tokens.iter().any(|t| t.ty == t::PHPDOC_EOL),
            // `setPositionOfLastToken()`
            last_token_position: children.last().and_then(|c| c.start_end).map(|se| se.1),
            tokens,
            children,
            keys,
            removed: Vec::new(),
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
                Value::Other(other) => other
                    .clone()
                    .idents_mut(&mut |name, parent| push(name, parent)),
                value @ (Value::Doctrine { .. }
                | Value::Text(_)
                | Value::Generic(_)
                | Value::Invalid(_)) => doctrine_identifiers(&mut value.clone())
                    .into_iter()
                    .for_each(|name| push(name, false)),
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
            Value::Annotation(annotation) if ident.index == 0 => {
                annotation.identifier = name.to_string()
            }
            Value::Annotation(annotation) => {
                let mut index = 0;
                nested_identifiers_mut(&mut annotation.values, &mut |current| {
                    index += 1;
                    if index == ident.index {
                        *current = name.to_string();
                    }
                });
            }
            Value::Other(other) => {
                let mut index = 0;
                other.idents_mut(&mut |current, _| {
                    if index == ident.index {
                        *current = name.to_string();
                    }
                    index += 1;
                });
            }
            value => {
                let identifiers = doctrine_identifiers(value);
                *identifiers
                    .into_iter()
                    .nth(ident.index)
                    .expect("a doctrine identifier") = name.to_string();
            }
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
    /// nested ones included, as its child and that class.
    pub fn find_annotation<'c>(&self, classes: &[&'c str]) -> Option<(usize, &'c str)> {
        for class in classes {
            let is = |identifier: &str, resolved: &str| has_class_name(identifier, resolved, class);
            for (i, child) in self.children.iter().enumerate() {
                let Value::Annotation(a) = &child.value else {
                    continue;
                };
                if is(&a.identifier, &a.class) || locate(&a.values, &is) {
                    return Some((i, class));
                }
            }
        }
        None
    }

    /// [`Self::find_annotation`]'s annotation: its child, the class it resolves to and its values.
    pub fn annotation_mut(
        &mut self,
        classes: &[&str],
    ) -> Option<(usize, &str, &mut Vec<ArrayItem>)> {
        let (child, class) = self.find_annotation(classes)?;
        let is = |identifier: &str, resolved: &str| has_class_name(identifier, resolved, class);
        self.children
            .iter_mut()
            .enumerate()
            .find_map(|(i, c)| match &mut c.value {
                Value::Annotation(a) if i == child => match is(&a.identifier, &a.class) {
                    true => Some((i, a.class.as_str(), &mut a.values)),
                    false => {
                        locate_mut(&mut a.values, &is).map(|(class, values)| (i, class, values))
                    }
                },
                _ => None,
            })
    }

    /// The class names `UnusedImportRemovingPostRector::findNamesInDocBlocks()` reads from the doc:
    /// `getAnnotationClassNames()`, `getConstFetchNodeClassNames()`, `getGenericTagClassNames()`,
    /// `getInlineGenericUsesTagClassNames()` and `getArrayItemNodeClassNames()`; `resolve` is
    /// `NameScope::resolveStringName()` where the doc is.
    pub fn used_names(&self, resolve: &dyn Fn(&str) -> String) -> Vec<String> {
        let mut names: Vec<String> = self
            .idents()
            .into_iter()
            .map(|i| i.name.trim_start_matches('@').to_string())
            .collect();
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
                            let leading = value
                                .split([' ', '\t', '\n', '\r'])
                                .find(|s| !s.is_empty())
                                .unwrap_or_default();
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
            let Some(tag) = ["uses", "used-by", "see"]
                .into_iter()
                .find(|t| rest.starts_with(t))
            else {
                continue;
            };
            let after = &rest[tag.len()..];
            let ws = after.len()
                - after
                    .trim_start_matches([' ', '\t', '\n', '\r', '\x0B', '\x0C'])
                    .len();
            if ws == 0 {
                continue;
            }
            let reference: &str = after[ws..]
                .split(|c: char| c == '}' || c.is_ascii_whitespace())
                .next()
                .unwrap_or_default();
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
            names.extend(
                idents
                    .iter()
                    .filter(|id| id.child == i)
                    .map(|id| id.name.clone()),
            );
        }
        names
    }

    /// `PhpDocTagRemover::removeTagValueFromNode()`, a `NODE_REMOVE` traversal: the child `i` goes, the others keep
    /// their keys (`unset()`).
    pub fn remove(&mut self, i: usize) {
        let child = self.children.remove(i);
        let key = self.keys.remove(i);
        self.removed.push((key, child));
    }

    /// `$phpDocNode->children[$key] = new PhpDocTextNode($text)`: the child `i` becomes a new text node.
    pub fn replace_with_text(&mut self, i: usize, text: String) {
        let text = Child {
            start_end: None,
            id: 0,
            tag: None,
            value: Value::Text(text),
            changed: false,
        };
        let child = std::mem::replace(&mut self.children[i], text);
        self.removed.push((self.keys[i], child));
    }

    /// `RemoveNodesStartAndEndResolver::resolve()`: for each child that went (`array_diff()` by the string of each,
    /// in their order), the token before its first token and the token after it.
    ///
    /// (A child that went is a tag that the rule took: one of the same text is taken too, so `array_diff()` drops
    /// none. Its start is no `PHPDOC_EOL`, and comes after the end of the one before it (or after `/**` for the
    /// first, where `null < $seekPosition`), so the seek to the start of the line stops where it starts.)
    fn jumps(&self) -> Vec<(usize, usize)> {
        let mut removed = self.removed.clone();
        removed.sort_by_key(|(key, _)| *key);
        // it's not there when comment block has empty row "\s\*\n"
        removed
            .iter()
            .filter_map(|(_, c)| c.start_end)
            .map(|(start, end)| (start - 1, end))
            .collect()
    }

    /// Whether every changed child can be printed as Rector prints it.
    pub fn reprintable(&self) -> bool {
        self.children
            .iter()
            .filter(|c| c.changed || c.start_end.is_none())
            .all(|c| print_child(c).is_some())
    }

    /// Whether a child changed.
    pub fn is_changed(&self) -> bool {
        self.children.iter().any(|c| c.changed)
    }

    /// `PhpDocInfoPrinter::printFormatPreserving()`. Call it only when [`Self::reprintable`] holds.
    pub fn print(&self) -> String {
        // `EmptyPhpDocDetector::isPhpDocNodeEmpty()`
        if self
            .children
            .iter()
            .all(|c| matches!(&c.value, Value::Text(text) if text.is_empty()))
        {
            return String::new();
        }
        let jumps = self.jumps();
        let mut current = 0;
        let mut output = String::new();
        let count = self.children.len();
        for (child, key) in self.children.iter().zip(&self.keys) {
            // `printDocChildNode()`, with the child's key in the array
            let key = key + 1;
            match child.start_end {
                Some((start, end)) if !child.changed => {
                    // `correctPreviouslyReprintedFirstNode()`
                    if current == 0 && key != 1 {
                        current = start - usize::from(self.tokens[start - 1].ty == t::PHPDOC_EOL);
                    }
                    let part =
                        self.add_tokens_from_to(String::new(), current, end, count == key, &jumps);
                    current = end;
                    output.push_str(part.trim_end_matches(PHP_WS));
                }
                start_end => {
                    if let Some((_, end)) = start_end {
                        current = end;
                    }
                    let printed = print_child(child).unwrap_or_default();
                    if self.single_line {
                        output.push_str(&format!(" {printed}"));
                    } else {
                        output.push_str("\n *");
                        if !printed.is_empty() {
                            output.push_str(&format!(" {printed}"));
                        }
                    }
                }
            }
        }
        // `printEnd()`
        let last = self.last_token_position.unwrap_or(current);
        output = match last {
            0 => output + "\n */",
            _ => self.add_tokens_from_to(output, last, self.tokens.len(), true, &jumps),
        };
        if !["//", "/**", "/*", "#"]
            .iter()
            .any(|s| output.starts_with(s))
            && !output.is_empty()
        {
            output.insert_str(0, "/**");
        }
        // `NEW_LINE_WITH_SPACE_REGEX`, then the `CALLABLE_REGEX` hotfix
        let output = output.replace(" \r\n", "\r\n").replace(" \n", "\n");
        callable_hotfix(&output)
    }

    /// `addTokensFromTo()`, the removed children's tokens skipped (`appendToOutput()`).
    fn add_tokens_from_to(
        &self,
        mut output: String,
        from: usize,
        to: usize,
        skip_empty_lines_above: bool,
        jumps: &[(usize, usize)],
    ) -> String {
        let text = |i: usize| {
            self.tokens
                .get(i)
                .map_or("", |t| &self.text[t.start..t.end])
        };
        let mut from = from;
        // include also space before, in case of inlined docs
        if from > 0 && self.tokens[from - 1].ty == t::HORIZONTAL_WS {
            from -= 1;
        }
        // skip extra empty lines above if this is the last one
        if skip_empty_lines_above && text(from).contains('\n') && text(from + 1).contains('\n') {
            from += 1;
        }
        let mut i = from;
        while i < to {
            while let Some(&(_, end)) = jumps.iter().find(|j| j.0 == i) {
                i = end;
            }
            output.push_str(text(i));
            i += 1;
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
        Node::Nullable(inner) | Node::Array(inner) | Node::Projection(_, inner) => {
            walk(inner, false, f)
        }
        Node::Union(types) | Node::Inter(types) => types.iter_mut().for_each(|t| walk(t, false, f)),
        Node::Generic { base, args } => {
            f(base, false);
            args.iter_mut().for_each(|t| walk(t, false, f));
        }
        // Rector's `SpacingAwareCallableTypeNode` keeps no templates
        Node::Callable {
            ident, params, ret, ..
        } => {
            f(ident, false);
            params.iter_mut().for_each(|p| walk(&mut p.ty, false, f));
            walk(ret, false, f);
        }
        Node::ArrayShape {
            items, unsealed, ..
        } => {
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
        Node::Nullable(inner) | Node::Array(inner) | Node::Projection(_, inner) => {
            const_fetches(inner, f)
        }
        Node::Union(types) | Node::Inter(types) => types.iter().for_each(|t| const_fetches(t, f)),
        Node::Generic { args, .. } => args.iter().for_each(|t| const_fetches(t, f)),
        Node::ArrayShape { items, .. } => items.iter().for_each(|i| const_fetches(&i.value, f)),
        _ => {}
    }
}

/// `RectorAssert::className()`.
pub(crate) fn is_class_name(name: &str) -> bool {
    !name.is_empty()
        && name.split('\\').all(|part| {
            let b = part.as_bytes();
            !b.is_empty()
                && (b[0].is_ascii_alphabetic() || b[0] == b'_' || b[0] >= 0x7f)
                && b.iter()
                    .all(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c >= 0x7f)
        })
}

/// The `Other` value of the tag `name` that `pun_phpstan` parsed as `tag`; `value` is the doc's text from the
/// tag's value to the end of the child, and `None` where Pun does not find where its description starts.
fn other(tag: phpdoc::Tag, value: &str, description: impl Fn(usize) -> String) -> Option<Value> {
    let (mut prepositions, mut quotes, mut description_at) = (Vec::new(), Vec::new(), None);
    match &tag.value {
        phpdoc::TagValue::Method { .. } => {
            let (at, p, q) = scan_method(value)?;
            (prepositions, quotes, description_at) = (p, q, Some(at));
        }
        phpdoc::TagValue::Assert { .. } => description_at = Some(scan_assert(value)?),
        // `TypelessParamTagValueNode` and `ParamImmediatelyInvokedCallableTagValueNode` and its kin: after the name
        phpdoc::TagValue::Param { .. } | phpdoc::TagValue::ParamName(_) => {
            description_at = Some(
                tokens_from(value, 0)
                    .iter()
                    .find(|t| t.ty == t::VARIABLE)?
                    .end,
            )
        }
        phpdoc::TagValue::Deprecated => description_at = Some(0),
        _ => {}
    }
    Some(Value::Other(Other {
        value: tag.value,
        description: description_at.map(description),
        prepositions,
        quotes,
        strings: strings_of(value),
    }))
}

/// The strings written in `value`: each as `pun_phpstan` reads it (`StringUnescaper`) and as `ConstExprStringNode`
/// prints it, `None` where that print is no UTF-8.
fn strings_of(value: &str) -> Vec<(Vec<u8>, Option<String>)> {
    let mut strings = Vec::new();
    for tok in parser::tokenize(value) {
        if let t::SINGLE_QUOTED_STRING | t::DOUBLE_QUOTED_STRING = tok.ty {
            let unescaped = parser::unescape_string(&value[tok.start..tok.end]);
            let printed = parser::print_string(&unescaped, &value[tok.start..tok.end]);
            strings.push((unescaped, printed));
        }
    }
    strings
}

/// The string `value` as printed; `None` where Rector would print bytes that are no UTF-8.
fn quote_of(strings: &[(Vec<u8>, Option<String>)], value: &[u8]) -> Option<String> {
    strings
        .iter()
        .find(|(s, _)| s == value)
        .and_then(|(_, printed)| printed.clone())
}

/// The byte of the input a `ParserException` message names (`getCurrentTokenOffset()`).
fn error_offset(message: &str) -> usize {
    let (_, after) = message.rsplit_once(" at offset ").unwrap_or_default();
    after
        .split(' ')
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or_default()
}

/// Where the type at the start of `value` (after white space) ends.
fn type_end(value: &str, at: usize) -> Option<(Node, usize)> {
    let start = at + value[at..].len() - value[at..].trim_start_matches([' ', '\t']).len();
    let (node, offset) = phpdoc::parse_prefix(&value[start..]).ok()?;
    Some((node, start + offset))
}

/// The tokens of `value` from `at`, white space left out, each with its bytes made absolute.
fn tokens_from(value: &str, at: usize) -> Vec<parser::Tok> {
    parser::tokenize(&value[at..])
        .into_iter()
        .filter(|t| t.ty != t::HORIZONTAL_WS)
        .map(|t| parser::Tok {
            ty: t.ty,
            start: at + t.start,
            end: at + t.end,
        })
        .collect()
}

/// `parseMethodTagValue()` on `value`: where its description starts, the preposition of each template, and each
/// string in the parameters' defaults as printed.
fn scan_method(value: &str) -> Option<(usize, Vec<String>, Vec<String>)> {
    let (first, mut end) = type_end(value, 0)?;
    if first == Node::Ident("static".into()) {
        end = type_end(value, end)?.1;
    }
    let toks = tokens_from(value, end);
    let mut i = usize::from(toks[0].ty == t::IDENTIFIER);
    let text = |tok: &parser::Tok| &value[tok.start..tok.end];
    // the tokens of each bracketed run, split at its commas: `<...>`, then `(...)`
    let run = |open: u8, close: u8, i: &mut usize| -> Vec<Vec<parser::Tok>> {
        let mut parts = vec![Vec::new()];
        if toks[*i].ty != open {
            return Vec::new();
        }
        *i += 1;
        let mut depth = 1;
        loop {
            let tok = toks[*i];
            *i += 1;
            match tok.ty {
                t::OPEN_PARENTHESES
                | t::OPEN_ANGLE_BRACKET
                | t::OPEN_SQUARE_BRACKET
                | t::OPEN_CURLY_BRACKET => depth += 1,
                t::CLOSE_PARENTHESES
                | t::CLOSE_ANGLE_BRACKET
                | t::CLOSE_SQUARE_BRACKET
                | t::CLOSE_CURLY_BRACKET => depth -= 1,
                t::COMMA if depth == 1 => {
                    parts.push(Vec::new());
                    continue;
                }
                _ => {}
            }
            if depth == 0 && tok.ty == close || *i == toks.len() {
                return parts;
            }
            parts.last_mut().unwrap().push(tok);
        }
    };
    let prepositions = run(t::OPEN_ANGLE_BRACKET, t::CLOSE_ANGLE_BRACKET, &mut i)
        .iter()
        .map(|part| {
            let found = part
                .iter()
                .find(|tok| tok.ty == t::IDENTIFIER && matches!(text(tok), "as" | "of"));
            found.map_or("of", |tok| text(tok)).to_string()
        })
        .collect();
    let mut quotes = Vec::new();
    for part in run(t::OPEN_PARENTHESES, t::CLOSE_PARENTHESES, &mut i) {
        let mut depth = 0;
        let mut default = false;
        for tok in part {
            match tok.ty {
                t::OPEN_PARENTHESES
                | t::OPEN_ANGLE_BRACKET
                | t::OPEN_SQUARE_BRACKET
                | t::OPEN_CURLY_BRACKET => depth += 1,
                t::CLOSE_PARENTHESES
                | t::CLOSE_ANGLE_BRACKET
                | t::CLOSE_SQUARE_BRACKET
                | t::CLOSE_CURLY_BRACKET => depth -= 1,
                t::EQUAL if depth == 0 => default = true,
                t::SINGLE_QUOTED_STRING | t::DOUBLE_QUOTED_STRING if default => {
                    let unescaped = parser::unescape_string(&value[tok.start..tok.end]);
                    let written = &value[tok.start..tok.end];
                    quotes.push(
                        parser::print_string(&unescaped, written)
                            .unwrap_or_else(|| written.to_string()),
                    );
                }
                _ => {}
            }
        }
    }
    Some((toks.get(i.checked_sub(1)?)?.end, prepositions, quotes))
}

/// `parseAssertTagValue()` on `value`: where its description starts.
fn scan_assert(value: &str) -> Option<usize> {
    let at = value.len() - value.trim_start_matches([' ', '\t', '!', '=']).len();
    let (_, end) = type_end(value, at)?;
    let toks = tokens_from(value, end);
    let mut i = 1;
    if toks[i].ty == t::ARROW {
        i += 2;
        if toks[i].ty == t::OPEN_PARENTHESES {
            i += 2;
        }
    }
    Some(toks[i - 1].end)
}

/// The value phpdoc-parser gives the tag `name` from `from` to `to` (bytes of the doc); `None` where Pun does not
/// read it.
fn tag_value(
    text: &str,
    tokens: &[parser::Tok],
    name: &str,
    value: parser::Value,
    from: usize,
    to: usize,
) -> Option<Value> {
    match value {
        parser::Value::Known => {
            if let Some(value) = typed(text, tokens, name, from, to) {
                return Some(value);
            }
            // a value its parser rejects is an `InvalidTagValueNode`; a parsed `@method` and its kin keep their own
            // value node
            let doc = format!("/** {} */", &text[from - name.len()..to]);
            let tag = phpdoc::tags(&doc)?.into_iter().next()?;
            if let phpdoc::TagValue::Invalid(..) = tag.value {
                return Some(Value::Invalid(generic_value(text, tokens, from, to)));
            }
            other(tag, &text[from..to], |at| {
                generic_value(text, tokens, from + at, to).replace('\n', "\n * ")
            })
        }
        parser::Value::Invalid(description) => Some(Value::Invalid(description)),
        parser::Value::Doctrine { args, description } => {
            Some(Value::Doctrine { args, description })
        }
        parser::Value::Generic(value) => Some(Value::Generic(value)),
    }
}

/// PHP's `trim()` characters.
const PHP_WS: [char; 6] = [' ', '\t', '\n', '\r', '\0', '\x0B'];

/// The children of the doc while `DoctrineAnnotationDecorator` works on them: PHP's array, each child under its key.
#[derive(Default)]
struct Slots {
    items: Vec<(usize, Child)>,
    ids: usize,
}

impl Slots {
    /// A new node object.
    fn child(
        &mut self,
        start_end: Option<(usize, usize)>,
        tag: Option<String>,
        value: Value,
    ) -> Child {
        self.ids += 1;
        Child {
            start_end,
            id: self.ids,
            tag,
            value,
            changed: false,
        }
    }

    fn position(&self, key: usize) -> Option<usize> {
        self.items.iter().position(|(k, _)| *k == key)
    }

    /// `$children[$key] = $child`: in its place, or last for a key the array lacks.
    fn set(&mut self, key: usize, child: Child) {
        match self.position(key) {
            Some(at) => self.items[at].1 = child,
            None => self.items.push((key, child)),
        }
    }

    /// `unset($children[$key])`.
    fn unset(&mut self, key: usize) {
        self.items.retain(|(k, _)| *k != key);
    }

    /// `array_splice($children, $offset, 0, $nodes)`: inserted at the position `offset`, the keys made anew.
    fn splice(&mut self, offset: usize, nodes: Vec<(String, Value)>) {
        let nodes: Vec<(usize, Child)> = nodes
            .into_iter()
            .map(|(tag, value)| (0, self.child(None, Some(tag), value)))
            .collect();
        let at = offset.min(self.items.len());
        self.items.splice(at..at, nodes);
        for (key, item) in self.items.iter_mut().enumerate() {
            item.0 = key;
        }
    }

    /// `while (isset($children[$key]) && $children[$key] !== $node) ++$key;`.
    fn find(&self, mut key: usize, id: usize) -> usize {
        while self
            .position(key)
            .is_some_and(|at| self.items[at].1.id != id)
        {
            key += 1;
        }
        key
    }
}

/// `DoctrineAnnotationDecorator::mergeNestedDoctrineAnnotations()`: a generic value joined with the `})` text and
/// the generic tags that follow it until it closes.
fn merge_nested_doctrine_annotations(slots: &mut Slots) {
    let mut removed = Vec::new();
    for key in 0..slots.items.len() {
        if removed.contains(&key) {
            continue;
        }
        let Value::Generic(mut value) = slots.items[key].1.value.clone() else {
            continue;
        };
        let mut start_end = slots.items[key].1.start_end;
        for next in key + 1..slots.items.len() {
            let child = &slots.items[next].1;
            let combined = start_end.zip(child.start_end).map(|(a, b)| (a.0, b.1));
            match (&child.tag, &child.value) {
                // `NESTED_ANNOTATION_END_REGEX`; the child after the text goes too
                (_, Value::Text(text)) if text.contains("})") => {
                    value = format!("{value}\n{text}");
                    start_end = combined;
                    removed.extend([next, next + 1]);
                }
                (Some(name), Value::Generic(next_value)) => {
                    if closed_content(&value) {
                        break;
                    }
                    value = format!("{value}\n{name}{next_value}");
                    start_end = combined;
                    removed.push(next);
                }
                _ => {}
            }
        }
        slots.items[key].1.value = Value::Generic(value);
        slots.items[key].1.start_end = start_end;
    }
    slots.items.retain(|(key, _)| !removed.contains(key));
}

/// `transformGenericTagValueNodesToDoctrineAnnotationTagValueNodes()` over the children as the array held them
/// when it started; `None` where Rector's annotation parsers never finish.
fn transform(
    slots: &mut Slots,
    resolve_tag: &dyn Fn(&str) -> String,
    narrow: &dyn Fn(&str) -> Option<String>,
) -> Option<()> {
    for (key, child) in slots.items.clone() {
        let name = child.tag.clone().unwrap_or_default();
        match &child.value {
            // the `@\FQN` use case
            Value::Text(text) => {
                let nodes = resolve_fqn_annotations(text, resolve_tag)?;
                if nodes.is_empty() {
                    continue;
                }
                // `processTextSpacelessInTextNode()`
                let other = first_text(text);
                let keep = !other.starts_with("@\\") && !other.trim_matches(PHP_WS).is_empty();
                if keep {
                    let text = slots.child(None, None, Value::Text(other));
                    slots.set(key, text);
                } else {
                    slots.unset(key);
                }
                slots.splice(key + usize::from(keep), nodes);
            }
            Value::Invalid(values) => process_doctrine(
                slots,
                &child,
                key,
                values.clone(),
                None,
                resolve_tag,
                narrow,
            )?,
            Value::Doctrine { args, description } => {
                let values = args
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                process_doctrine(
                    slots,
                    &child,
                    key,
                    values,
                    Some(description),
                    resolve_tag,
                    narrow,
                )?;
            }
            Value::Generic(value) => {
                // known doc tag to annotation class
                let class = resolve_tag(&name);
                if !class.contains('\\') && class != "Target" {
                    continue;
                }
                let key = slots.find(key, child.id);
                let nodes = resolve_fqn_annotations(value, resolve_tag)?;
                let content = match nodes.is_empty() {
                    true => value.clone(),
                    false => first_text(value),
                };
                // its `ORIG_NODE` set to null, Rector still prints it as written
                let annotation = slots.child(
                    child.start_end,
                    child.tag.clone(),
                    annotation(&name, content, class, resolve_tag)?,
                );
                slots.set(key, annotation);
                if !nodes.is_empty() {
                    slots.splice(key + 1, nodes);
                }
            }
            _ => process_description_as_spaceless(slots, &child, key, resolve_tag)?,
        }
    }
    Some(())
}

/// `processDoctrine()`: `values` are the printed arguments of a `DoctrineTagValueNode` (with its `description`) or
/// an `InvalidTagValueNode`'s value. A name that is no class (a template) leaves the tag as it is.
#[allow(clippy::too_many_arguments)]
fn process_doctrine(
    slots: &mut Slots,
    child: &Child,
    key: usize,
    values: String,
    description: Option<&String>,
    resolve_tag: &dyn Fn(&str) -> String,
    narrow: &dyn Fn(&str) -> Option<String>,
) -> Option<()> {
    let name = child
        .tag
        .as_deref()
        .unwrap_or_default()
        .trim_start_matches('@');
    let Some(class) = narrow(name) else {
        return Some(());
    };
    let mut values = values;
    if !values.is_empty() {
        values = star_comment(&values);
        if let Some(description) = description {
            values = format!("({values}){description}");
        }
    }
    let tag = format!("@{name}");
    let annotation = annotation(&tag, values, class, resolve_tag)?;
    let annotation = slots.child(child.start_end, Some(tag), annotation);
    // the key the child had when the decorator started
    slots.set(key, annotation);
    Some(())
}

/// `processDescriptionAsSpacelessPhpDoctagNode()`: a description over lines with `@\Class` annotations gives them as
/// children after the tag, which moves to the end of the array without them.
fn process_description_as_spaceless(
    slots: &mut Slots,
    child: &Child,
    key: usize,
    resolve_tag: &dyn Fn(&str) -> String,
) -> Option<()> {
    let mut value = child.value.clone();
    let Some(description) = value.description_mut() else {
        return Some(());
    };
    if !description.contains('\n') {
        return Some(());
    }
    let nodes = resolve_fqn_annotations(description, resolve_tag)?;
    if nodes.is_empty() {
        return Some(());
    }
    let key = slots.find(key, child.id);
    slots.unset(key);
    // `Strings::replace($description, LONG_ANNOTATION_REGEX, '')`, then `substr($description, 0, -7)`
    let stripped = parser::replace_all(long_annotation(), description, |_| String::new());
    let cut = stripped.len().saturating_sub(7);
    *description = String::from_utf8_lossy(&stripped.as_bytes()[..cut]).into_owned();
    let node = slots.child(None, child.tag.clone(), value);
    slots.set(key, node);
    slots.splice(key + 1, nodes);
    Some(())
}

/// `LONG_ANNOTATION_REGEX`.
fn long_annotation() -> &'static pcre2::bytes::Regex {
    static RE: std::sync::OnceLock<pcre2::bytes::Regex> = std::sync::OnceLock::new();
    RE.get_or_init(|| {
        pcre2::bytes::Regex::new(r"@\\(?<class_name>.*?)(?<annotation_content>\(.*?\)|,|\r?\n|$)")
            .expect("annotation regex")
    })
}

/// `Strings::split($text, NEWLINE_ANNOTATION_FQCN_REGEX)[0]`: the text before the first line that starts with `@\`.
fn first_text(text: &str) -> String {
    static RE: std::sync::OnceLock<pcre2::bytes::Regex> = std::sync::OnceLock::new();
    let re = RE
        .get_or_init(|| pcre2::bytes::Regex::new(r"\r?\n@\\").expect("new line annotation regex"));
    let end = re
        .find(text.as_bytes())
        .ok()
        .flatten()
        .map_or(text.len(), |m| m.start());
    text[..end].to_string()
}

/// `resolveFqnAnnotationSpacelessPhpDocTagNode()`: an annotation of each `@\Class` in `text`, with its tag; `None`
/// where Rector's parsers never finish reading one.
fn resolve_fqn_annotations(
    text: &str,
    resolve_tag: &dyn Fn(&str) -> String,
) -> Option<Vec<(String, Value)>> {
    let mut nodes = Vec::new();
    for caps in long_annotation().captures_iter(text.as_bytes()).flatten() {
        let group = |name: &str| caps.name(name).map_or("", |m| &text[m.start()..m.end()]);
        let open: Vec<&str> = group("class_name").split('(').collect();
        // `resolveAnnotationContent()`
        let content = match open.get(1).map(|o| o.trim_matches(PHP_WS)) {
            Some(o) if !o.ends_with('{') && !o.is_empty() => {
                format!("(\"{}\")", o.trim_matches(['"', '\'']))
            }
            _ => group("annotation_content").to_string(),
        };
        let tag = format!("@\\{}", open[0]);
        let value = annotation(&tag, content, open[0].to_string(), resolve_tag)?;
        nodes.push((tag, value));
    }
    Some(nodes)
}

impl Value {
    /// The value node's `description` property, for one that has it.
    fn description_mut(&mut self) -> Option<&mut String> {
        match self {
            Value::Typed(typed) => Some(&mut typed.description),
            Value::Other(Other {
                description: Some(description),
                ..
            }) => Some(description),
            _ => None,
        }
    }
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
fn annotation(
    tag: &str,
    content: String,
    class: String,
    resolve_tag: &dyn Fn(&str) -> String,
) -> Option<Value> {
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
    value
        .trim_matches([' ', '\t'])
        .trim_end_matches('\n')
        .to_string()
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
        for v in item_vals(item) {
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
fn item_vals(item: &ArrayItem) -> impl Iterator<Item = &Val> {
    let key = match &item.key {
        Some(Key::Val(key)) => Some(key),
        _ => None,
    };
    std::iter::once(&item.value).chain(key)
}

/// The `IdentifierTypeNode`s of a `DoctrineTagValueNode`'s keys and values (a text value has none).
fn doctrine_identifiers(value: &mut Value) -> Vec<&mut String> {
    match value {
        Value::Doctrine { args, .. } => args
            .iter_mut()
            .flat_map(|a| &mut a.0)
            .filter(|p| p.1)
            .map(|p| &mut p.0)
            .collect(),
        _ => Vec::new(),
    }
}

/// `DoctrineAnnotationTagValueNode::hasClassName()`.
fn has_class_name(identifier: &str, resolved: &str, class: &str) -> bool {
    identifier.trim_matches('@') == class || resolved == class
}

/// Whether a nested annotation in `values` is one `is` accepts.
fn locate(values: &[ArrayItem], is: &dyn Fn(&str, &str) -> bool) -> bool {
    values.iter().any(|item| {
        item_vals(item).any(|v| match v {
            Val::Annotation(nested) => {
                is(&nested.identifier, &nested.class) || locate(&nested.values, is)
            }
            Val::Curly(items) | Val::List(items) => locate(items, is),
            _ => false,
        })
    })
}

/// The first nested annotation in `values` that `is` accepts, in [`locate`]'s order: the class it resolves to and its
/// values.
fn locate_mut<'a>(
    values: &'a mut [ArrayItem],
    is: &dyn Fn(&str, &str) -> bool,
) -> Option<(&'a str, &'a mut Vec<ArrayItem>)> {
    for ArrayItem { key, value } in values {
        let key = match key {
            Some(Key::Val(key)) => Some(key),
            _ => None,
        };
        for v in std::iter::once(value).chain(key) {
            match v {
                Val::Annotation(nested) => {
                    let found = match is(&nested.identifier, &nested.class) {
                        true => Some((nested.class.as_str(), &mut nested.values)),
                        false => locate_mut(&mut nested.values, is),
                    };
                    if found.is_some() {
                        return found;
                    }
                }
                Val::Curly(items) | Val::List(items) => {
                    if let Some(found) = locate_mut(items, is) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
    }
    None
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
    let name = child.tag.as_deref().unwrap_or_default();
    match &child.value {
        Value::Typed(typed) => {
            let quote = |v: &[u8]| quote_of(&typed.strings, v);
            let ty = |i: usize| -> Option<String> {
                Some(TypeNode::from_parsed_all(&typed.types[i], &quote)?.to_string())
            };
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
                Form::Var(variable) => {
                    format!("{} {}", ty(0)?, format!("{variable} {description}").trim())
                }
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
        Value::Annotation(a) => Some(format!("{name}{}", a.values_string())),
        Value::Other(other) => Some(format!("{name} {}", other.print()?).trim().to_string()),
        // `DoctrineTagValueNode::__toString()`: the annotation's name and arguments, then the description
        Value::Doctrine { args, description } => {
            let args = args
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            Some(
                format!("{name}({args}) {description}")
                    .trim_matches(PHP_WS)
                    .to_string(),
            )
        }
        // `PhpDocTextNode` (no name), `GenericTagValueNode`, `InvalidTagValueNode`
        Value::Text(value) | Value::Generic(value) | Value::Invalid(value) => {
            Some(format!("{name} {value}").trim_matches(PHP_WS).to_string())
        }
    }
}

/// The value of a typed tag from `from` to `to` (bytes of the doc), `None` for a tag Pun does not read this way or a
/// value its parser rejects.
fn typed(text: &str, tokens: &[parser::Tok], name: &str, from: usize, to: usize) -> Option<Value> {
    let value = &text[from..to];
    let skip =
        |at: usize| at + value[at..].len() - value[at..].trim_start_matches([' ', '\t']).len();
    let preceded_by_ws = |at: usize| at > 0 && matches!(value.as_bytes()[at - 1], b' ' | b'\t');
    let variable_len = |s: &str| {
        let b = s.as_bytes();
        (b.first() == Some(&b'$')
            && b.get(1)
                .is_some_and(|c| c.is_ascii_alphabetic() || *c == b'_' || *c >= 0x80))
        .then(|| {
            1 + b[1..]
                .iter()
                .take_while(|c| c.is_ascii_alphanumeric() || **c == b'_' || **c >= 0x80)
                .count()
        })
    };
    let is_this = |s: &str| {
        s.strip_prefix("$this").is_some_and(|after| {
            !after
                .bytes()
                .next()
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_' || c >= 0x80)
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
            .take_while(|c| {
                c.is_ascii_alphanumeric() || *c == b'_' || *c == b'\\' || *c == b'-' || *c >= 0x80
            })
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
                at = bound_from
                    + value[bound_from..bound_from + offset]
                        .trim_end_matches([' ', '\t'])
                        .len();
                break;
            }
        }
        // a lower bound and a default, parsed: `SpacingAwareTemplateTagValueNode` keeps neither, so they are neither
        // traversed nor printed
        let ident_char =
            |c: u8| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-' | b'\\') || c >= 0x80;
        for word in ["super", "="] {
            let next = skip(at);
            let after = &value[next..];
            let found = match word {
                "super" => {
                    after.starts_with(word) && !after[5..].bytes().next().is_some_and(ident_char)
                }
                _ => after.starts_with('=') && !after.starts_with("=>"),
            };
            if found {
                let from = skip(next + word.len());
                let (_, offset) = phpdoc::parse_prefix(&value[from..]).ok()?;
                at = from
                    + value[from..from + offset]
                        .trim_end_matches([' ', '\t'])
                        .len();
            }
        }
        (
            Form::Template {
                name: template_name,
                preposition,
            },
            at,
        )
    } else if PARAM.contains(&name)
        || VAR.contains(&name)
        || PLAIN.contains(&name)
        || NAMED.contains(&name)
    {
        // a typeless `@param $a`
        if PARAM.contains(&name)
            && ((rest.starts_with('$') && !is_this(rest))
                || rest.starts_with('&')
                || rest.starts_with("..."))
        {
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
            // `parseRequiredVariableName()`: a `TOKEN_VARIABLE`, which `$this` is not
            let len = variable_len(&value[at..]).filter(|_| !is_this(&value[at..]))?;
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
        strings: strings_of(value),
        replaced: Vec::new(),
    }))
}
