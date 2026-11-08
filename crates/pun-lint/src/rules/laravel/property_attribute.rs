//! What rector-laravel's `*PropertyTo*AttributeRector` rules (src/Rector/Class_) share. rector-laravel has no base
//! class for them: each `refactor()` is the same steps on its own class, property, attribute and value check. Pun ports
//! those steps once ([`refactor`]), and each rule is a [`Config`] of them (the [`property_rules!`] list).
//!
//! The steps: the class must be `final` (the queue rules) and an object of the rule's class (`isObjectType`, which reads
//! the class's ancestors through PHPStan's reflection); it must not have the attribute already
//! (`PhpAttributeAnalyzer::hasPhpAttribute`); `Class_::getProperty()` finds the first property statement with an item
//! of the name (case-sensitive), whose first item's default is the value; the property must be `protected`
//! (`isProtected()`) or public (`isPublic()`, no visibility counts); the value must pass the rule's check. Then the
//! attribute group is appended to the class's attributes and the whole property statement goes, its doc comment and
//! other items with it.
//!
//! Printing follows php-parser's format-preserving printer: the new group goes on its own line after the class's last
//! attribute list, or before the class at its indentation (inline before `class` for an anonymous class); the value
//! keeps its source text, moved to the class's indentation (`TokenStream::getTokenCode`). A class body left with no
//! statement (php-parser keeps a trailing comment as a `Nop`) makes php-parser print the whole class anew.

use crate::rules::rector_code_quality::is_constant;
use mago_syntax::cst::*;
use pun_problem::{Edit, Safety, Span};

use crate::rules::rector_batch5::{has_attribute, member_from, remove_items};
use crate::rules::rector_conditions::{indentation, reprint};
use crate::rules::rector_stmts::{first_comment, line_indent};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report};

pub(crate) const MODEL: &str = "Illuminate\\Database\\Eloquent\\Model";
pub(crate) const COMMAND: &str = "Illuminate\\Console\\Command";
pub(crate) const SHOULD_QUEUE: &str = "Illuminate\\Contracts\\Queue\\ShouldQueue";
pub(crate) const JSON_RESOURCE: &str = "Illuminate\\Http\\Resources\\Json\\JsonResource";
pub(crate) const FORM_REQUEST: &str = "Illuminate\\Foundation\\Http\\FormRequest";

/// Which visibility the property must have.
#[derive(Clone, Copy)]
pub(crate) enum Visibility {
    /// `Property::isProtected()`.
    Protected,
    /// `Property::isPublic()`: `public`, or no visibility.
    Public,
}

/// What the rule asks of the property's default.
#[derive(Clone, Copy)]
pub(crate) enum Check {
    /// Any value.
    Any,
    /// A `String_`.
    String,
    /// A `ClassConstFetch` or a `String_`.
    ClassConstOrString,
    /// A non-empty `Array_` whose every item's value is a `String_`.
    StringArray,
    /// A non-empty `Array_` whose no item's value has a type that is never a string (`isString()->no()`).
    MaybeStringArray,
    /// A non-empty `Array_` whose every item's value has a string type (`isString()->yes()`).
    TypedStringArray,
    /// `getType($value)->isInteger()->yes()`.
    Integer,
    /// `getType($value)->isTrue()->yes()`.
    True,
    /// `isTrue()->yes()`, or the constant `true`.
    TrueOrConstant,
    /// The constant `false`.
    FalseConstant,
}

/// One rule: its ID and message, the class it applies to, the property, and the attribute it writes.
pub(crate) struct Config {
    pub id: &'static str,
    pub message: &'static str,
    pub class: &'static str,
    /// The queue rules ask for `$node->isFinal()` first.
    pub final_only: bool,
    pub property: &'static str,
    pub visibility: Visibility,
    pub check: Check,
    pub attribute: &'static str,
    /// Whether a class with the attribute already is left alone (all but `ConnectionPropertyToConnectionAttributeRector`).
    pub skip_existing: bool,
}

/// The class-like `node` (a class or an anonymous class) that rector-laravel's rules visit, as a `Class_`.
pub(crate) struct ClassView<'ast, 'arena> {
    pub node: Node<'ast, 'arena>,
    pub attribute_lists: &'ast Sequence<'arena, AttributeList<'arena>>,
    pub modifiers: &'ast Sequence<'arena, Modifier<'arena>>,
    pub members: &'ast Sequence<'arena, ClassLikeMember<'arena>>,
    pub extends: &'ast Option<Extends<'arena>>,
    pub implements: &'ast Option<Implements<'arena>>,
    pub left_brace: mago_span::Span,
    pub right_brace: mago_span::Span,
    pub kind: Kind<'ast, 'arena>,
}

/// A named class or an anonymous one.
#[derive(Clone, Copy)]
pub(crate) enum Kind<'ast, 'arena> {
    Named(&'ast Class<'arena>),
    Anonymous(&'ast AnonymousClass<'arena>),
}

impl<'ast, 'arena> ClassView<'ast, 'arena> {
    /// The view of a class or an anonymous class; `None` for any other node.
    pub(crate) fn of(node: Node<'ast, 'arena>) -> Option<Self> {
        if let Node::Class(c) = node {
            return Some(ClassView {
                node,
                attribute_lists: &c.attribute_lists,
                modifiers: &c.modifiers,
                members: &c.members,
                extends: &c.extends,
                implements: &c.implements,
                left_brace: c.left_brace,
                right_brace: c.right_brace,
                kind: Kind::Named(c),
            });
        }
        let Node::AnonymousClass(c) = node else {
            return None;
        };
        Some(ClassView {
            node,
            attribute_lists: &c.attribute_lists,
            modifiers: &c.modifiers,
            members: &c.members,
            extends: &c.extends,
            implements: &c.implements,
            left_brace: c.left_brace,
            right_brace: c.right_brace,
            kind: Kind::Anonymous(c),
        })
    }

    /// `Class_::isFinal()`.
    pub(crate) fn is_final(&self) -> bool {
        self.modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Final(_)))
    }

    /// The name PHPStan's reflection has for the class: its namespaced name, or Mago's name of an anonymous class.
    pub(crate) fn name(&self, ctx: &Context<'_>, ancestors: &[Node<'_, '_>]) -> String {
        match self.kind {
            Kind::Named(c) => {
                let name = String::from_utf8_lossy(c.name.value);
                match crate::rules::types::namespace_of(ancestors) {
                    Some(ns) => format!("{ns}\\{name}"),
                    None => name.into_owned(),
                }
            }
            Kind::Anonymous(c) => facts(ctx).anonymous_class_name(mago_span::HasSpan::span(c)),
        }
    }

    /// `NodeTypeResolver::isObjectType($class, new ObjectType($target))`: the class is `target` or has it among its
    /// ancestors, and `target` is a class PHPStan's reflection finds (Rector's check fails when it cannot load it).
    pub(crate) fn is_object_type(
        &self,
        ctx: &Context<'_>,
        ancestors: &[Node<'_, '_>],
        target: &str,
    ) -> bool {
        facts(ctx)
            .codebase
            .get_class_like(target.as_bytes())
            .is_some()
            && super::is_a(ctx, &self.name(ctx, ancestors), &[target])
    }

    /// `PhpAttributeAnalyzer::hasPhpAttribute($class, $attribute)`.
    pub(crate) fn has_attribute(&self, ancestors: &[Node<'_, '_>], attribute: &str) -> bool {
        has_attribute(ancestors, self.attribute_lists, attribute)
    }

    /// `Class_::getProperty($name)`: the first property statement with an item named `name`, and its index.
    pub(crate) fn property(&self, name: &str) -> Option<(usize, PropertyView<'ast, 'arena>)> {
        self.members.iter().enumerate().find_map(|(i, m)| {
            let ClassLikeMember::Property(p) = m else {
                return None;
            };
            let view = PropertyView::of(p);
            view.items
                .iter()
                .any(|item| &item.variable().name[1..] == name.as_bytes())
                .then_some((i, view))
        })
    }

    /// `Class_::getMethod($name)`: the first method named `name` (ASCII case-insensitive), and its index.
    pub(crate) fn method(&self, name: &str) -> Option<(usize, &'ast Method<'arena>)> {
        self.members.iter().enumerate().find_map(|(i, m)| match m {
            ClassLikeMember::Method(method)
                if method.name.value.eq_ignore_ascii_case(name.as_bytes()) =>
            {
                Some((i, method))
            }
            _ => None,
        })
    }
}

/// A property statement as php-parser's `Property`: its modifiers and items.
pub(crate) struct PropertyView<'ast, 'arena> {
    pub modifiers: &'ast Sequence<'arena, Modifier<'arena>>,
    pub items: Vec<&'ast PropertyItem<'arena>>,
}

impl<'ast, 'arena> PropertyView<'ast, 'arena> {
    fn of(property: &'ast Property<'arena>) -> Self {
        match property {
            Property::Plain(p) => PropertyView {
                modifiers: &p.modifiers,
                items: p.items.iter().collect(),
            },
            Property::Hooked(p) => PropertyView {
                modifiers: &p.modifiers,
                items: vec![&p.item],
            },
        }
    }

    pub(crate) fn is_protected(&self) -> bool {
        self.modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Protected(_)))
    }

    /// php-parser's `isPublic()`: `public`, or no visibility at all (PHP allows one visibility modifier).
    pub(crate) fn is_public(&self) -> bool {
        !(self.modifiers.iter()).any(|m| matches!(m, Modifier::Protected(_) | Modifier::Private(_)))
    }

    pub(crate) fn has(&self, visibility: Visibility) -> bool {
        match visibility {
            Visibility::Protected => self.is_protected(),
            Visibility::Public => self.is_public(),
        }
    }

    /// `$property->props[0]->default`, without the parentheses php-parser does not keep.
    pub(crate) fn default(&self) -> Option<&'ast Expression<'arena>> {
        match self.items[0] {
            PropertyItem::Concrete(c) => Some(unparenthesize(c.value)),
            PropertyItem::Abstract(_) => None,
        }
    }
}

/// php-parser's `String_`: a quoted string, or a heredoc or nowdoc without interpolation.
pub(crate) fn is_string_node(expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Literal(Literal::String(_)) => true,
        Expression::CompositeString(CompositeString::Document(d)) => {
            d.parts.iter().all(|p| matches!(p, StringPart::Literal(_)))
        }
        _ => false,
    }
}

/// The items of an `Array_` (`[...]` or `array(...)`).
pub(crate) fn array_elements<'ast, 'arena>(
    expr: &'ast Expression<'arena>,
) -> Option<&'ast TokenSeparatedSequence<'arena, ArrayElement<'arena>>> {
    match expr {
        Expression::Array(a) => Some(&a.elements),
        Expression::LegacyArray(a) => Some(&a.elements),
        _ => None,
    }
}

/// An item's value (`$item->value`); `None` for a missing item (`$item === null`).
fn element_value<'ast, 'arena>(
    element: &'ast ArrayElement<'arena>,
) -> Option<&'ast Expression<'arena>> {
    match element {
        ArrayElement::KeyValue(e) => Some(e.value),
        ArrayElement::Value(e) => Some(e.value),
        ArrayElement::Variadic(e) => Some(e.value),
        ArrayElement::Missing(_) => None,
    }
}

/// PHPStan's `getType()` of `expr` (docblocks read), as far as Mago's analyser reached it.
fn type_of<'f>(
    facts: &'f crate::rules::types::facts::Facts,
    expr: &Expression<'_>,
) -> Option<&'f mago_codex::ttype::union::TUnion> {
    facts.full_type(node_span(unparenthesize(expr)))
}

/// `isString()->no()`: every type the value can have is never a string.
fn never_string(t: &mago_codex::ttype::union::TUnion) -> bool {
    use mago_codex::ttype::atomic::TAtomic;
    use mago_codex::ttype::atomic::scalar::TScalar;
    !t.types.is_empty()
        && t.types.iter().all(|a| {
            matches!(
                a,
                TAtomic::Scalar(TScalar::Bool(_) | TScalar::Integer(_) | TScalar::Float(_))
                    | TAtomic::Null
                    | TAtomic::Array(_)
                    | TAtomic::Object(_)
            )
        })
}

impl Config {
    /// The value check on the property's default.
    fn passes(&self, ctx: &Context<'_>, value: &Expression<'_>) -> bool {
        let facts = || facts(ctx);
        match self.check {
            Check::Any => true,
            Check::String => is_string_node(value),
            Check::ClassConstOrString => {
                matches!(value, Expression::Access(Access::ClassConstant(_)))
                    || is_string_node(value)
            }
            Check::StringArray | Check::MaybeStringArray | Check::TypedStringArray => {
                let Some(elements) = array_elements(value) else {
                    return false;
                };
                if elements.is_empty() {
                    return false;
                }
                let facts = facts();
                elements.iter().all(|e| {
                    let Some(v) = element_value(e) else {
                        return false;
                    };
                    match self.check {
                        Check::StringArray => is_string_node(v),
                        Check::MaybeStringArray => !type_of(&facts, v).is_some_and(never_string),
                        _ => type_of(&facts, v)
                            .is_some_and(|t| t.is_any_string() && !t.types.is_empty()),
                    }
                })
            }
            Check::Integer => {
                type_of(&facts(), value).is_some_and(|t| t.is_int() && !t.types.is_empty())
            }
            Check::True => type_of(&facts(), value).is_some_and(|t| t.is_true()),
            Check::TrueOrConstant => {
                type_of(&facts(), value).is_some_and(|t| t.is_true()) || is_constant(value, "true")
            }
            Check::FalseConstant => is_constant(value, "false"),
        }
    }

    /// Whether the attribute takes the value as its argument (the checks for `true` and `false` write a bare one).
    fn with_argument(&self) -> bool {
        !matches!(
            self.check,
            Check::True | Check::TrueOrConstant | Check::FalseConstant
        )
    }

    /// The rule's `refactor()` on the class `node`.
    pub(crate) fn refactor(
        &self,
        node: Node<'_, '_>,
        ancestors: &[Node<'_, '_>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(class) = ClassView::of(node) else {
            return;
        };
        if (self.final_only && !class.is_final())
            || !class.is_object_type(ctx, ancestors, self.class)
            || (self.skip_existing && class.has_attribute(ancestors, self.attribute))
        {
            return;
        }
        let Some((index, property)) = class.property(self.property) else {
            return;
        };
        if !property.has(self.visibility) {
            return;
        }
        let Some(value) = property.default() else {
            return;
        };
        if !self.passes(ctx, value) {
            return;
        }
        let args = if self.with_argument() {
            format!("({})", moved_value(ctx.source, &class, value))
        } else {
            String::new()
        };
        let group = format!("#[\\{}{args}]", self.attribute);
        let Some(edits) = rewrite(ctx.source, &class, &[group], &[], &[index]) else {
            return;
        };
        out.push(report(&class, self.message, edits));
    }
}

/// The report of a rule's change to the class.
pub(crate) fn report(class: &ClassView<'_, '_>, message: &str, edits: Vec<Edit>) -> Report {
    Report::fixed(
        node_span(&class.node),
        message.to_string(),
        Safety::Unsafe,
        edits,
    )
}

/// The source of `expr` as php-parser prints a kept node in a new one at the class's indentation: every line after
/// the first moved by the difference of the two indentations, string contents aside.
pub(crate) fn moved_value(
    source: &str,
    class: &ClassView<'_, '_>,
    expr: &Expression<'_>,
) -> String {
    let span = node_span(unparenthesize(expr));
    let adjust =
        indentation(source, class_start(class)) as isize - indentation(source, span.start) as isize;
    reprint(source, &crate::rules::php_parser::lex(source), span, adjust)
}

/// Where php-parser's class node starts: its first attribute list or modifier, or `class` (`new` for an anonymous
/// class).
fn class_start(class: &ClassView<'_, '_>) -> usize {
    node_span(&class.node).start
}

/// The edits for adding the attribute groups `groups` to the class, replacing the groups at `replaced` (indices of
/// its attribute lists) with the first new ones, and removing the members at `removed`. `None` where Pun does not
/// follow the printer: a tab in the class's indentation.
pub(crate) fn rewrite(
    source: &str,
    class: &ClassView<'_, '_>,
    groups: &[String],
    replaced: &[usize],
    removed: &[usize],
) -> Option<Vec<Edit>> {
    let start = class_start(class);
    let indent = line_indent(source, start);
    if indent.contains('\t') {
        return None;
    }
    let nl = format!("\n{indent}");
    let (open, close) = (
        class.left_brace.end.offset as usize,
        class.right_brace.start.offset as usize,
    );
    let flags: Vec<bool> = (0..class.members.len())
        .map(|i| removed.contains(&i))
        .collect();
    let mut spans: Vec<Span> = class.members.iter().map(node_span).collect();
    let mut froms: Vec<usize> = (0..spans.len())
        .map(|i| member_from(class.members, class.left_brace, i))
        .collect();
    let mut kept_flags = flags.clone();
    // A comment after the last member is php-parser's `Nop`, a statement that stays.
    let after = spans.last().map_or(open, |s| s.end);
    if let Some(comment) = first_comment(source, after, close) {
        let end = after + source[after..close].trim_end().len();
        froms.push(after);
        spans.push(Span::new(comment, end));
        kept_flags.push(false);
    }
    let Some(mut edits) = remove_items(source, &spans, &froms, &kept_flags) else {
        // Nothing left in the body: php-parser prints the class anew.
        return Some(vec![Edit::new(
            Span::new(start, close + 1),
            reprinted(source, class, groups, replaced, &nl),
        )]);
    };
    let lists: Vec<Span> = class.attribute_lists.iter().map(node_span).collect();
    let mut new = groups.iter();
    for &i in replaced {
        edits.push(Edit::new(lists[i], new.next().unwrap().clone()));
    }
    let rest: Vec<&String> = new.collect();
    if !rest.is_empty() {
        let sep = if matches!(class.kind, Kind::Anonymous(_)) {
            " ".to_string()
        } else {
            nl.clone()
        };
        match lists.last() {
            Some(last) => {
                let text: String = rest.iter().map(|g| format!("{sep}{g}")).collect();
                edits.push(Edit::new(Span::new(last.end, last.end), text));
            }
            None => {
                // Before the modifiers or `class`; an anonymous class's attributes follow `new`.
                let at = match class.kind {
                    Kind::Anonymous(c) => {
                        let new_end = c.new.span.end.offset as usize;
                        new_end + source[new_end..].len() - source[new_end..].trim_start().len()
                    }
                    Kind::Named(_) => start,
                };
                let text: String = rest.iter().map(|g| format!("{g}{sep}")).collect();
                edits.push(Edit::new(Span::new(at, at), text));
            }
        }
    }
    edits.sort_by_key(|e| e.span.start);
    Some(edits)
}

/// The class as php-parser's `pClassCommon` prints it with no statements: its attribute groups (the new ones after
/// the kept ones, each on a line), its modifiers in php-parser's order, `class`, the name or the anonymous class's
/// arguments, `extends`, `implements`, and an empty body on lines of their own.
fn reprinted(
    source: &str,
    class: &ClassView<'_, '_>,
    groups: &[String],
    replaced: &[usize],
    nl: &str,
) -> String {
    let text = |s: Span| &source[s.start..s.end];
    let mut out = String::new();
    if matches!(class.kind, Kind::Anonymous(_)) {
        out.push_str("new ");
    }
    let mut new = groups.iter();
    for (i, list) in class.attribute_lists.iter().enumerate() {
        match replaced.contains(&i) {
            true => out.push_str(new.next().unwrap()),
            false => out.push_str(text(node_span(list))),
        }
        out.push_str(nl);
    }
    for g in new {
        out.push_str(g);
        out.push_str(nl);
    }
    let has = |f: fn(&Modifier<'_>) -> bool| class.modifiers.iter().any(f);
    if has(|m| matches!(m, Modifier::Final(_))) {
        out.push_str("final ");
    }
    if has(|m| matches!(m, Modifier::Abstract(_))) {
        out.push_str("abstract ");
    }
    if has(|m| matches!(m, Modifier::Readonly(_))) {
        out.push_str("readonly ");
    }
    out.push_str("class");
    match class.kind {
        Kind::Named(c) => {
            out.push(' ');
            out.push_str(&String::from_utf8_lossy(c.name.value));
        }
        Kind::Anonymous(c) => {
            if let Some(args) = c.argument_list.as_ref().filter(|a| !a.arguments.is_empty()) {
                let args: Vec<&str> = args.arguments.iter().map(|a| text(node_span(a))).collect();
                out.push_str(&format!("({})", args.join(", ")));
            }
        }
    }
    let names = |types: &TokenSeparatedSequence<'_, Identifier<'_>>| {
        types
            .iter()
            .map(|t| text(node_span(t)).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    if let Some(e) = class.extends {
        out.push_str(" extends ");
        out.push_str(&names(&e.types));
    }
    if let Some(i) = class.implements {
        out.push_str(" implements ");
        out.push_str(&names(&i.types));
    }
    out.push_str(nl);
    out.push('{');
    out.push_str(nl);
    out.push('}');
    out
}

/// Declares the unit struct `$name` as the [`Config`] `$config`, a rule on classes and anonymous classes.
macro_rules! property_rules {
    ($($name:ident = $config:expr;)*) => {$(
        pub struct $name;

        impl $name {
            const CONFIG: $crate::rules::laravel::property_attribute::Config = $config;
        }

        impl $crate::Rule for $name {
            fn id(&self) -> &'static str {
                Self::CONFIG.id
            }

            fn targets(&self) -> Option<&'static [mago_syntax::cst::NodeKind]> {
                Some(&[mago_syntax::cst::NodeKind::Class, mago_syntax::cst::NodeKind::AnonymousClass])
            }

            fn visit<'ast, 'arena>(
                &self,
                node: mago_syntax::cst::Node<'ast, 'arena>,
                ancestors: &[mago_syntax::cst::Node<'ast, 'arena>],
                ctx: &$crate::Context<'_>,
                out: &mut Vec<$crate::Report>,
            ) {
                Self::CONFIG.refactor(node, ancestors, ctx, out);
            }
        }
    )*};
}
pub(crate) use property_rules;
