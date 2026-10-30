//! The restricted-usage rules on members (config.level2.neon, under `featureToggles.internalTag`, and with
//! phpstan-deprecation-rules): RestrictedMethodUsageRule, RestrictedStaticMethodUsageRule, RestrictedPropertyUsageRule,
//! RestrictedStaticPropertyUsageRule and RestrictedUsageOfDeprecatedStringCastRule, with the internal extensions (an
//! `@internal` member, or a member of an `@internal` class-like, used from outside the root namespace that declares
//! it) and the deprecated ones (deprecations.rs).

use std::collections::HashSet;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::flags::MetadataFlags;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::Checker;
use super::members::Method;

/// The lowercase names of the methods and properties the restricted-usage extensions can report: `@internal` (with
/// `internal`) or deprecated (with `deprecated`) ones and those of such class-likes. A member access whose name is not
/// among them needs no type.
pub struct Names {
    methods: HashSet<Vec<u8>>,
    properties: HashSet<Vec<u8>>,
}

impl Names {
    pub fn of(codebase: &CodebaseMetadata, php_version: u32, internal: bool, deprecated: bool) -> Names {
        let restricted = |flags: MetadataFlags| (internal && flags.is_internal()) || (deprecated && flags.is_deprecated());
        let class = |name: &[u8]| codebase.get_class_like(name).is_some_and(|k| restricted(k.flags));
        let mut methods: HashSet<Vec<u8>> = codebase
            .function_likes
            .iter()
            .filter(|((class_name, _), f)| {
                !class_name.is_empty() && (restricted(f.flags) || class(class_name.as_bytes()))
            })
            .map(|((_, method), _)| method.as_bytes().to_ascii_lowercase())
            .collect();
        if deprecated {
            let builtins = super::reflection::builtins(php_version);
            methods.extend(
                builtins
                    .classes
                    .values()
                    .flat_map(|k| k.methods.iter())
                    .filter(|(_, f)| f.deprecated)
                    .map(|(name, _)| name.as_bytes().to_ascii_lowercase()),
            );
        }
        let properties = codebase
            .class_likes
            .values()
            .flat_map(|k| {
                let all = restricted(k.flags);
                k.properties
                    .iter()
                    .filter(move |(_, p)| all || restricted(p.flags))
                    .map(|(name, _)| name.as_bytes()[1..].to_ascii_lowercase())
            })
            .collect();
        Names { methods, properties }
    }

    fn contains(&self, name: &[u8]) -> bool {
        self.methods.contains(&name.to_ascii_lowercase())
    }

    fn property(&self, name: &[u8]) -> bool {
        self.properties.contains(&name.to_ascii_lowercase())
    }
}

/// The calls whose object or class expression the rules type.
struct Typed<'n> {
    names: &'n Names,
    spans: Vec<Span>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Typed<'_> {
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        if let ClassLikeMemberSelector::Identifier(id) = &call.method
            && self.names.contains(id.value)
        {
            self.spans.push(call.object.span());
        }
    }
    fn walk_in_null_safe_method_call(
        &mut self,
        call: &'ast NullSafeMethodCall<'arena>,
        _: &mut (),
    ) {
        if let ClassLikeMemberSelector::Identifier(id) = &call.method
            && self.names.contains(id.value)
        {
            self.spans.push(call.object.span());
        }
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.static_class(call.class, &call.method);
    }
    fn walk_in_property_access(&mut self, p: &'ast PropertyAccess<'arena>, _: &mut ()) {
        if let ClassLikeMemberSelector::Identifier(id) = &p.property
            && self.names.property(id.value)
        {
            self.spans.push(p.object.span());
        }
    }
    fn walk_in_static_property_access(&mut self, p: &'ast StaticPropertyAccess<'arena>, _: &mut ()) {
        if let Variable::Direct(v) = &p.property
            && self.names.property(&v.name[1..])
            && !matches!(
                p.class,
                Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)
            )
        {
            self.spans.push(p.class.span());
        }
    }
    fn walk_in_class_constant_access(&mut self, a: &'ast ClassConstantAccess<'arena>, _: &mut ()) {
        if !matches!(
            a.class,
            Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)
        ) {
            self.spans.push(a.class.span());
        }
    }
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        if matches!(u.operator, UnaryPrefixOperator::StringCast(..) | UnaryPrefixOperator::BinaryCast(..))
            && self.names.contains(b"__tostring")
        {
            self.spans.push(u.operand.span());
        }
    }
    fn walk_in_method_partial_application(&mut self, m: &'ast MethodPartialApplication<'arena>, _: &mut ()) {
        if let ClassLikeMemberSelector::Identifier(id) = &m.method
            && self.names.contains(id.value)
        {
            self.spans.push(m.object.span());
        }
    }
    fn walk_in_static_method_partial_application(
        &mut self,
        m: &'ast StaticMethodPartialApplication<'arena>,
        _: &mut (),
    ) {
        self.static_class(m.class, &m.method);
    }
}

impl Typed<'_> {
    fn static_class(&mut self, class: &Expression, method: &ClassLikeMemberSelector) {
        if let ClassLikeMemberSelector::Identifier(id) = method
            && self.names.contains(id.value)
            && !matches!(
                class,
                Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)
            )
        {
            self.spans.push(class.span());
        }
    }
}

/// The expressions whose types the rules read.
pub fn typed_expressions(names: Option<&Names>, program: &Program) -> Vec<Span> {
    let Some(names) = names else {
        return Vec::new();
    };
    let mut typed = Typed {
        names,
        spans: Vec::new(),
    };
    typed.walk_program(program, &mut ());
    typed.spans
}

/// The classes of an expression's object types (`getObjectClassNames()`), as Mago infers them.
pub(super) fn object_classes<'a>(c: &Checker<'a, '_>, e: &Expression) -> Vec<&'a ClassLikeMetadata> {
    c.type_of(e)
        .into_iter()
        .flat_map(|t| t.types.iter())
        .filter_map(|a| match a {
            TAtomic::Object(TObject::Named(o)) => c.codebase.get_class_like(o.name.as_bytes()),
            TAtomic::Object(TObject::Enum(o)) => c.codebase.get_class_like(o.name.as_bytes()),
            _ => None,
        })
        .collect()
}

/// `$obj->name()` and `$obj?->name()` (`call` spans the whole call: the error is on its first line).
pub fn method_call(
    c: &mut Checker,
    call: Span,
    object: &Expression,
    method: &ClassLikeMemberSelector,
) {
    let ClassLikeMemberSelector::Identifier(id) = method else {
        return;
    };
    if !c.internal.is_some_and(|n| n.contains(id.value)) {
        return;
    }
    for k in object_classes(c, object) {
        if let Some(m) = super::members::method(c, k, id.value) {
            let usages: Vec<_> = METHOD_EXTENSIONS.iter().filter_map(|ext| ext(c, &m, None)).collect();
            for (message, identifier) in usages {
                c.report(call, &identifier, message, None);
            }
        }
    }
}

type MethodExtension = fn(&Checker, &Method, Option<&ClassLikeMetadata>) -> Option<(String, String)>;

/// The RestrictedMethodUsageExtension services, in registration order.
const METHOD_EXTENSIONS: [MethodExtension; 2] = [restricted, super::deprecations::method];

/// `Name::name()`, `self::`/`static::`/`parent::name()` and `$expr::name()`.
pub fn static_method_call(c: &mut Checker, call: &StaticMethodCall) {
    static_method(c, call.span(), call.class, &call.method);
}

/// `$obj->name(...)` (RestrictedMethodCallableUsageRule).
pub fn method_callable(c: &mut Checker, m: &MethodPartialApplication) {
    if is_callable(&m.argument_list) {
        method_call(c, m.span(), m.object, &m.method);
    }
}

/// `Name::name(...)` (RestrictedStaticMethodCallableUsageRule).
pub fn static_method_callable(c: &mut Checker, m: &StaticMethodPartialApplication) {
    if is_callable(&m.argument_list) {
        static_method(c, m.span(), m.class, &m.method);
    }
}

/// A first-class callable's `(...)`.
pub(super) fn is_callable(args: &PartialArgumentList) -> bool {
    matches!(args.arguments.iter().collect::<Vec<_>>().as_slice(), [PartialArgument::VariadicPlaceholder(_)])
}

fn static_method(c: &mut Checker, span: Span, class: &Expression, method: &ClassLikeMemberSelector) {
    let ClassLikeMemberSelector::Identifier(id) = method else {
        return;
    };
    if !c.internal.is_some_and(|n| n.contains(id.value)) {
        return;
    }
    let classes = match class {
        Expression::Identifier(name) => c
            .codebase
            .get_class_like(c.resolved(name).as_bytes())
            .into_iter()
            .collect(),
        Expression::Self_(_) | Expression::Static(_) => c.class_scope().into_iter().collect(),
        Expression::Parent(_) => c
            .class_scope()
            .and_then(|k| k.direct_parent_class)
            .and_then(|p| c.codebase.get_class_like(p.as_bytes()))
            .into_iter()
            .collect(),
        e => object_classes(c, e),
    };
    for k in classes {
        let Some(m) = super::members::method(c, k, id.value) else {
            continue;
        };
        for ext in METHOD_EXTENSIONS {
            let Some((message, identifier)) = ext(c, &m, None) else {
                continue;
            };
            // RewrittenDeclaringClassMethodReflection: the call must stay restricted with the class it names declaring it
            if !k
                .name
                .as_bytes()
                .eq_ignore_ascii_case(m.declaring_name().as_bytes())
                && ext(c, &m, Some(k)).is_none()
            {
                continue;
            }
            c.report(span, &identifier, message, None);
        }
    }
}

/// RestrictedInternalMethodUsageExtension::isRestrictedMethodUsage(): the message and identifier, with `declaring`
/// standing in for the method's declaring class when given.
fn restricted(
    c: &Checker,
    m: &Method,
    declaring: Option<&ClassLikeMetadata>,
) -> Option<(String, String)> {
    let Method::User { declaring: own, f, .. } = m else {
        return None;
    };
    if !c.internal_members {
        return None;
    }
    let declaring = declaring.unwrap_or(own);
    let method_internal = f.flags.is_internal();
    if !method_internal && !declaring.flags.is_internal() {
        return None;
    }
    let declaring_name = String::from_utf8_lossy(declaring.original_name.as_bytes()).into_owned();
    if !should_be_reported(c, &declaring_name) {
        return None;
    }
    let (kind, prefix) = if m.is_static() {
        ("staticMethod", "static ")
    } else {
        ("method", "")
    };
    let description = match declaring.kind {
        SymbolKind::Interface => "Interface",
        SymbolKind::Trait => "Trait",
        SymbolKind::Enum => "Enum",
        _ => "Class",
    };
    let display = c.display(declaring);
    let name = m.name();
    let outside = match root_namespace(&declaring_name) {
        Some(ns) => format!(" from outside its root namespace {ns}"),
        None => String::new(),
    };
    Some(if method_internal {
        (
            format!("Call to internal {prefix}method {display}::{name}(){outside}."),
            format!("{kind}.internal"),
        )
    } else {
        (
            format!(
                "Call to {prefix}method {name}() of internal {} {display}{outside}.",
                description.to_ascii_lowercase()
            ),
            format!("{kind}.internal{description}"),
        )
    })
}

/// The first segment of a name's namespace (`array_slice(explode('\\', $name), 0, -1)[0]`).
pub(super) fn root_namespace(name: &str) -> Option<&str> {
    let (namespace, _) = name.rsplit_once('\\')?;
    namespace.split('\\').next()
}

/// RestrictedInternalUsageHelper::shouldBeReported().
pub(super) fn should_be_reported(c: &Checker, name: &str) -> bool {
    match c.namespaces.last().cloned().flatten() {
        None => match c.class_scope() {
            None => true,
            Some(k) => !k.original_name.as_bytes().eq(name.as_bytes()),
        },
        Some(current) => {
            let current = current.split('\\').next().unwrap_or_default();
            let namespace = root_namespace(name).unwrap_or_default();
            !format!("{namespace}\\").starts_with(&format!("{current}\\"))
        }
    }
}

/// RestrictedInternalPropertyUsageExtension::isRestrictedPropertyUsage(): property `$name` that `declaring`
/// declares (`owner` standing in for it when given).
fn internal_property(
    c: &Checker,
    declaring: &ClassLikeMetadata,
    owner: Option<&ClassLikeMetadata>,
    name: &str,
    is_static: bool,
    property_internal: bool,
) -> Option<(String, String)> {
    let declaring = owner.unwrap_or(declaring);
    if !c.internal_members || (!property_internal && !declaring.flags.is_internal()) {
        return None;
    }
    let declaring_name = String::from_utf8_lossy(declaring.original_name.as_bytes()).into_owned();
    if !should_be_reported(c, &declaring_name) {
        return None;
    }
    let (kind, prefix) = if is_static { ("staticProperty", "static ") } else { ("property", "") };
    let description = super::deprecations::type_description(declaring);
    let display = c.display(declaring);
    let outside = root_namespace(&declaring_name).map_or_else(String::new, |ns| format!(" from outside its root namespace {ns}"));
    Some(if property_internal {
        (format!("Access to internal {prefix}property {display}::${name}{outside}."), format!("{kind}.internal"))
    } else {
        (
            format!(
                "Access to {prefix}property ${name} of internal {} {display}{outside}.",
                description.to_ascii_lowercase()
            ),
            format!("{kind}.internal{description}"),
        )
    })
}

/// A property for the restricted-usage extensions: its declaring class, whether `@internal`, its deprecation.
struct PropertyFacts<'a> {
    declaring: &'a ClassLikeMetadata,
    internal: bool,
    deprecation: Option<super::deprecations::Deprecation>,
}

/// `getInstanceProperty()` / `getStaticProperty()` of `k`: a native property (static as `is_static` asks), else for an
/// instance property an `@property` tag (neither internal nor deprecated itself).
fn property_facts<'a>(c: &Checker<'a, '_>, k: &'a ClassLikeMetadata, name: &[u8], is_static: bool) -> Option<PropertyFacts<'a>> {
    let dollar = [b"$", name].concat();
    if let Some((declaring, _, p)) = c.declared_property(k, &dollar) {
        if p.flags.is_static() != is_static {
            return None;
        }
        // a trait's property is declared by the class using the trait
        let declaring = if declaring.kind == SymbolKind::Trait { using_class(c, k, declaring) } else { declaring };
        return Some(PropertyFacts {
            declaring,
            internal: p.flags.is_internal(),
            deprecation: if c.deprecations { c.property_deprecation(declaring, &dollar) } else { None },
        });
    }
    if is_static || !c.allows_dynamic_properties(k) {
        return None;
    }
    let declaring = k.magic_property_ids.get(&mago_word::word(&dollar))?;
    Some(PropertyFacts {
        declaring: c.codebase.get_class_like(declaring.as_bytes())?,
        internal: false,
        deprecation: None,
    })
}

/// The nearest of `k` and its ancestors that uses trait `t`, else `k`.
fn using_class<'a>(c: &Checker<'a, '_>, k: &'a ClassLikeMetadata, t: &ClassLikeMetadata) -> &'a ClassLikeMetadata {
    let mut current = Some(k);
    for _ in 0..super::overriding::ANCESTRY {
        let Some(a) = current else {
            break;
        };
        if a.used_traits.iter().any(|u| u.as_bytes().eq_ignore_ascii_case(t.name.as_bytes())) {
            return a;
        }
        current = a.direct_parent_class.and_then(|p| c.codebase.get_class_like(p.as_bytes()));
    }
    k
}

/// RestrictedPropertyUsageRule: `$expr->name` (not a nullsafe fetch).
pub fn property_fetch(c: &mut Checker, p: &PropertyAccess) {
    let ClassLikeMemberSelector::Identifier(id) = &p.property else {
        return;
    };
    if !c.internal.is_some_and(|n| n.property(id.value)) {
        return;
    }
    let name = String::from_utf8_lossy(id.value).into_owned();
    for k in object_classes(c, p.object) {
        let Some(facts) = property_facts(c, k, id.value, false) else {
            continue;
        };
        let usages = [
            internal_property(c, facts.declaring, None, &name, false, facts.internal),
            super::deprecations::property(c, facts.declaring, None, &name, false, facts.deprecation.clone()),
        ];
        for (message, identifier) in usages.into_iter().flatten() {
            c.report(p, &identifier, message, None);
        }
    }
}

/// RestrictedStaticPropertyUsageRule: `Name::$name`, `self::`/`static::`/`parent::$name` and `$expr::$name`.
pub fn static_property_fetch(c: &mut Checker, p: &StaticPropertyAccess) {
    let Variable::Direct(v) = &p.property else {
        return;
    };
    let name = &v.name[1..];
    if !c.internal.is_some_and(|n| n.property(name)) {
        return;
    }
    let classes: Vec<&ClassLikeMetadata> = match p.class {
        Expression::Identifier(id) => c.codebase.get_class_like(c.resolved(id).as_bytes()).into_iter().collect(),
        Expression::Self_(_) | Expression::Static(_) => c.class_scope().into_iter().collect(),
        Expression::Parent(_) => c
            .class_scope()
            .and_then(|k| k.direct_parent_class)
            .and_then(|parent| c.codebase.get_class_like(parent.as_bytes()))
            .into_iter()
            .collect(),
        // RuleLevelHelper::findTypeToCheck(): `$this` only below level 2
        Expression::Variable(Variable::Direct(v)) if c.level < 2 && v.name == b"$this" => {
            c.class_scope().into_iter().collect()
        }
        _ if c.level < 2 => Vec::new(),
        e => object_classes(c, e),
    };
    let property = String::from_utf8_lossy(name).into_owned();
    for k in classes {
        let Some(facts) = property_facts(c, k, name, true) else {
            continue;
        };
        let rewritten = !k.name.as_bytes().eq_ignore_ascii_case(facts.declaring.name.as_bytes());
        let internal = |owner| internal_property(c, facts.declaring, owner, &property, true, facts.internal);
        let deprecated = |owner| {
            super::deprecations::property(c, facts.declaring, owner, &property, true, facts.deprecation.clone())
        };
        let mut usages = Vec::new();
        if let Some(u) = internal(None)
            && (!rewritten || internal(Some(k)).is_some())
        {
            usages.push(u);
        }
        if let Some(u) = deprecated(None)
            && (!rewritten || deprecated(Some(k)).is_some())
        {
            usages.push(u);
        }
        for (message, identifier) in usages {
            c.report(p, &identifier, message, None);
        }
    }
}

/// RestrictedUsageOfDeprecatedStringCastRule: `(string) $expr` of an object whose `__toString()` is restricted.
pub fn string_cast(c: &mut Checker, u: &UnaryPrefix) {
    if !matches!(u.operator, UnaryPrefixOperator::StringCast(..) | UnaryPrefixOperator::BinaryCast(..))
        || !c.internal.is_some_and(|n| n.contains(b"__tostring"))
    {
        return;
    }
    for k in object_classes(c, u.operand) {
        // `hasNativeMethod('__toString')`
        let Some(m) = super::members::method(c, k, b"__toString") else {
            continue;
        };
        let usages: Vec<_> = METHOD_EXTENSIONS.iter().filter_map(|ext| ext(c, &m, None)).collect();
        for (message, identifier) in usages {
            c.report(u, &identifier, message, None);
        }
    }
}

/// The static property a STATIC_PROPERTY_ACCESS location names (`hasStaticProperty()`), for the class-name extensions.
pub fn static_property_member(c: &Checker, k: &ClassLikeMetadata, name: &[u8]) -> Option<super::class_names::Member> {
    let facts = property_facts(c, k, name, true)?;
    Some(super::class_names::Member {
        name: String::from_utf8_lossy(name).into_owned(),
        internal: facts.internal || facts.declaring.flags.is_internal(),
        deprecated: c.deprecations
            && (facts.deprecation.is_some() || c.class_deprecation(facts.declaring).is_some()),
    })
}

/// The method a STATIC_METHOD_CALL location names (`hasMethod()` and `getMethod()`), for the class-name extensions.
pub fn method_member(c: &Checker, m: &Method) -> super::class_names::Member {
    let (internal, declaring_deprecated) = match m {
        Method::User { declaring, f, .. } => (
            f.flags.is_internal() || declaring.flags.is_internal(),
            c.deprecations && c.class_deprecation(declaring).is_some(),
        ),
        Method::Builtin { .. } => (false, false),
    };
    super::class_names::Member {
        name: m.name(),
        internal,
        deprecated: declaring_deprecated || (c.deprecations && c.method_deprecation(m).is_some()),
    }
}
