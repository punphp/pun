//! The rules on PHPStan's ClassPropertyNode (one per declared property and per promoted constructor parameter):
//! PropertiesInInterfaceRule, PropertyInClassRule, ReadOnlyPropertyRule and ReadOnlyByPhpDocPropertyRule.

use mago_codex::symbol::SymbolKind;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::declarations::{READONLY_TAGS, has_tag};
use super::{Checker, overriding_property};

/// ClassPropertyNode.
pub(super) struct PropertyNode<'n, 'arena> {
    /// Where the node is (the property's variable, or the promoted parameter).
    pub at: Span,
    /// The name with its `$`.
    pub name: &'n [u8],
    pub modifiers: &'n [Modifier<'arena>],
    pub hint: Option<&'n Hint<'arena>>,
    pub attributes: &'n [AttributeList<'arena>],
    pub default: bool,
    pub hooks: &'n [PropertyHook<'arena>],
    pub readonly_by_phpdoc: bool,
    pub allowed_private_mutation: bool,
    /// where a declared property's PHPDoc is read from (`None` for a promoted one)
    pub decl: Option<Span>,
}

const ALLOW_PRIVATE_MUTATION_TAGS: &[&str] = &[
    "@phpstan-readonly-allow-private-mutation",
    "@phpstan-allow-private-mutation",
    "@psalm-readonly-allow-private-mutation",
    "@psalm-allow-private-mutation",
];

impl PropertyNode<'_, '_> {
    pub(super) fn has(&self, f: impl Fn(&Modifier) -> bool) -> bool {
        self.modifiers.iter().any(f)
    }
    pub(super) fn is_public(&self) -> bool {
        // Modifiers::VISIBILITY_MASK: the read visibilities
        self.has(|m| m.is_public()) || !self.has(|m| m.is_read_visibility())
    }
    pub(super) fn is_static(&self) -> bool {
        self.has(|m| m.is_static())
    }
    fn hook_final(h: &PropertyHook) -> bool {
        h.modifiers.contains_final()
    }
    fn hook_abstract(h: &PropertyHook) -> bool {
        matches!(h.body, PropertyHookBody::Abstract(_))
    }
}

/// The ClassPropertyNodes of a property declaration.
pub(super) fn property<'arena>(c: &mut Checker<'_, 'arena>, p: &Property<'arena>) {
    let tags = c.tags_before(p);
    let readonly_by_phpdoc = has_tag(&tags, READONLY_TAGS);
    let allowed_private_mutation = has_tag(&tags, ALLOW_PRIVATE_MUTATION_TAGS);
    let (items, hooks, attributes): (Vec<&PropertyItem<'arena>>, &[PropertyHook<'arena>], _) =
        match p {
            Property::Plain(p) => (p.items.iter().collect(), &[], p.attribute_lists.as_slice()),
            Property::Hooked(h) => (
                vec![&h.item],
                h.hook_list.hooks.as_slice(),
                h.attribute_lists.as_slice(),
            ),
        };
    for item in items {
        let variable = item.variable();
        let node = PropertyNode {
            at: variable.span(),
            name: variable.name,
            modifiers: p.modifiers().as_slice(),
            hint: p.hint(),
            attributes,
            default: matches!(item, PropertyItem::Concrete(_)),
            hooks,
            readonly_by_phpdoc,
            allowed_private_mutation,
            decl: Some(p.span()),
        };
        check(c, &node);
    }
}

/// The ClassPropertyNodes of a constructor's promoted parameters. Their default is always null, so whether they are
/// read-only by PHPDoc never matters.
pub(super) fn promoted<'arena>(
    c: &mut Checker<'_, 'arena>,
    parameters: &FunctionLikeParameterList<'arena>,
) {
    for p in &parameters.parameters {
        let hooks = p.hooks.as_ref().map_or(&[][..], |h| h.hooks.as_slice());
        if p.modifiers.is_empty() && hooks.is_empty() {
            continue;
        }
        let node = PropertyNode {
            at: p.span(),
            name: p.variable.name,
            modifiers: p.modifiers.as_slice(),
            hint: p.hint.as_ref(),
            attributes: p.attribute_lists.as_slice(),
            default: false,
            hooks,
            readonly_by_phpdoc: false,
            allowed_private_mutation: false,
            decl: None,
        };
        check(c, &node);
    }
}

fn check<'arena>(c: &mut Checker<'_, 'arena>, node: &PropertyNode<'_, 'arena>) {
    let decl = c.decls.last().expect("in a class-like");
    let (kind, immutable_doc, parent) = (decl.kind, decl.immutable_doc, decl.parent.clone());
    let is_readonly = node.has(|m| m.is_readonly()) || decl.is_readonly;
    if kind == SymbolKind::Interface {
        in_interface(c, node, is_readonly);
    } else if kind == SymbolKind::Class {
        in_class(c, node, is_readonly);
    }
    readonly(c, node, is_readonly);
    overriding_property::check(c, node, is_readonly);
    super::property_hooks::check(c, node);
    // isReadOnlyByPhpDoc(): a `@readonly` tag, or the class is immutable (its PHPDoc or a parent's)
    let by_phpdoc = node.readonly_by_phpdoc
        || immutable_doc
        || parent.is_some_and(|p| c.class_immutable(&p, 0));
    if let Some(decl) = node.decl {
        super::variance::property(c, node, decl, by_phpdoc);
    }
    if node.default && !is_readonly && !node.allowed_private_mutation && by_phpdoc {
        c.report(
            node.at,
            "property.readOnlyByPhpDocDefaultValue",
            "@readonly property cannot have a default value.".to_string(),
            None,
        );
    }
}

fn error(c: &mut Checker, node: &PropertyNode, identifier: &str, message: &str) {
    c.report_non_ignorable(node.at, identifier, message.to_string(), None);
}

/// PropertiesInInterfaceRule.
fn in_interface(c: &mut Checker, node: &PropertyNode, is_readonly: bool) {
    let (identifier, message) = if !c.surely_at_least(80400) {
        (
            "property.inInterface",
            "Interfaces can include properties only on PHP 8.4 and later.",
        )
    } else if node.hooks.is_empty() {
        (
            "property.nonHookedInInterface",
            "Interfaces can only include hooked properties.",
        )
    } else if !node.is_public() {
        (
            "property.nonPublicInInterface",
            "Interfaces cannot include non-public properties.",
        )
    } else if is_readonly {
        (
            "property.readOnlyInInterface",
            "Interfaces cannot include readonly hooked properties.",
        )
    } else if node.is_static() {
        (
            "property.hookedStatic",
            "Hooked properties cannot be static.",
        )
    } else if node.has(|m| m.is_abstract()) {
        (
            "property.abstractInInterface",
            "Property in interface cannot be explicitly abstract.",
        )
    } else if node.has(|m| m.is_final()) {
        (
            "property.finalInInterface",
            "Interfaces cannot include final properties.",
        )
    } else if node.hooks.iter().any(PropertyNode::hook_final) {
        (
            "property.abstractFinalHook",
            "Property hook cannot be both abstract and final.",
        )
    } else if node.hooks.iter().any(|h| !PropertyNode::hook_abstract(h)) {
        (
            "property.hookBodyInInterface",
            "Interfaces cannot include property hooks with bodies.",
        )
    } else {
        return;
    };
    error(c, node, identifier, message);
}

/// PropertyInClassRule.
fn in_class(c: &mut Checker, node: &PropertyNode, is_readonly: bool) {
    let class_abstract = c.decls.last().is_some_and(|d| d.is_abstract);
    let abstract_ = node.has(|m| m.is_abstract());
    let final_ = node.has(|m| m.is_final());
    let hooked = !node.hooks.is_empty();
    let any_abstract_hook = node.hooks.iter().any(PropertyNode::hook_abstract);
    let (identifier, message) = if final_ && !c.surely_at_least(80400) {
        (
            "property.final",
            "Final properties are supported only on PHP 8.4 and later.",
        )
    } else if abstract_ && !hooked {
        (
            "property.abstractNonHooked",
            "Only hooked properties can be declared abstract.",
        )
    } else if abstract_ && !any_abstract_hook {
        (
            "property.abstractWithoutAbstractHook",
            "Abstract properties must specify at least one abstract hook.",
        )
    } else if abstract_ && !class_abstract {
        (
            "property.abstract",
            "Non-abstract classes cannot include abstract properties.",
        )
    } else if !abstract_ && any_abstract_hook {
        (
            "property.hookWithoutBody",
            "Non-abstract properties cannot include hooks without bodies.",
        )
    } else if node.has(|m| m.is_private()) && abstract_ {
        (
            "property.abstractPrivate",
            "Property cannot be both abstract and private.",
        )
    } else if node.has(|m| m.is_private()) && final_ {
        (
            "property.finalPrivate",
            "Property cannot be both final and private.",
        )
    } else if node.has(|m| m.is_private()) && node.hooks.iter().any(PropertyNode::hook_final) {
        (
            "property.finalPrivateHook",
            "Private property cannot have a final hook.",
        )
    } else if abstract_
        && (final_
            || node
                .hooks
                .iter()
                .any(|h| PropertyNode::hook_abstract(h) && PropertyNode::hook_final(h)))
    {
        (
            "property.abstractFinal",
            "Property cannot be both abstract and final.",
        )
    } else if is_readonly && hooked {
        (
            "property.hookReadOnly",
            "Hooked properties cannot be readonly.",
        )
    } else if node.is_static() && hooked {
        (
            "property.hookedStatic",
            "Hooked properties cannot be static.",
        )
    } else if node.is_static()
        && !c.surely_at_least(80500)
        && node.has(|m| {
            matches!(
                m,
                Modifier::PrivateSet(_) | Modifier::ProtectedSet(_) | Modifier::PublicSet(_)
            )
        })
    {
        (
            "property.staticAsymmetricVisibility",
            "Asymmetric visibility for static properties is supported only on PHP 8.5 and later.",
        )
    } else if node.default && is_virtual(c, node) {
        (
            "property.virtualDefault",
            "Virtual hooked properties cannot have a default value.",
        )
    } else if !c.surely_at_least(80400) && hooked {
        (
            "property.hooksNotSupported",
            "Property hooks are supported only on PHP 8.4 and later.",
        )
    } else {
        return;
    };
    error(c, node, identifier, message);
}

/// ReadOnlyPropertyRule.
fn readonly(c: &mut Checker, node: &PropertyNode, is_readonly: bool) {
    if !is_readonly {
        return;
    }
    // property.readOnlyNotSupported (before PHP 8.1) never fires: php-parser lexes `readonly` as a keyword from 8.1 on
    if node.hint.is_none() {
        error(
            c,
            node,
            "property.readOnlyNoNativeType",
            "Readonly property must have a native type.",
        );
    }
    if node.default {
        error(
            c,
            node,
            "property.readOnlyDefaultValue",
            "Readonly property cannot have a default value.",
        );
    }
    if node.is_static() {
        error(
            c,
            node,
            "property.readOnlyStatic",
            "Readonly property cannot be static.",
        );
    }
}

/// BetterReflection's ReflectionProperty::isVirtual(): hooks that never touch `$this->name` (a `set => expr` hook
/// always does), unless the parent class's property of that name is not virtual.
pub(super) fn is_virtual(c: &Checker, node: &PropertyNode) -> bool {
    if node.hooks.is_empty() {
        return false;
    }
    let name = &node.name[1..];
    let hook = |n: &[u8]| node.hooks.iter().find(|h| h.name.value == n);
    // an abstract hook (no body) is virtual
    let virtual_hook = |h: &PropertyHook| match &h.body {
        PropertyHookBody::Concrete(PropertyHookConcreteBody::Expression(e)) => {
            h.name.value != b"set" && !fetches(e.expression, name)
        }
        body => !matches!(body, PropertyHookBody::Concrete(PropertyHookConcreteBody::Block(b)) if {
            let mut f = Fetches { name, found: false };
            f.walk_block(b, &mut ());
            f.found
        }),
    };
    if hook(b"set").is_some_and(|h| !virtual_hook(h)) {
        return false;
    }
    if hook(b"get").is_some_and(|h| !virtual_hook(h)) {
        return false;
    }
    let parent = c.decls.last().and_then(|d| d.parent.as_deref());
    parent
        .and_then(|p| c.codebase.get_declaring_property(p.as_bytes(), node.name))
        .is_none_or(|p| p.flags.is_virtual_property())
}

fn fetches(e: &Expression, name: &[u8]) -> bool {
    let mut f = Fetches { name, found: false };
    f.walk_expression(e, &mut ());
    f.found
}

/// Finds `$this->name`.
struct Fetches<'n> {
    name: &'n [u8],
    found: bool,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Fetches<'_> {
    fn walk_in_property_access(&mut self, a: &'ast PropertyAccess<'arena>, (): &mut ()) {
        if let (Expression::Variable(Variable::Direct(v)), ClassLikeMemberSelector::Identifier(p)) =
            (a.object, &a.property)
            && v.name == b"$this"
            && p.value == self.name
        {
            self.found = true;
        }
    }
}
