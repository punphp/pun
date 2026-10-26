//! Level 3's property writes, over PropertyReflectionFinder: TypesAssignedToPropertiesRule (`assign.propertyType`),
//! ReadOnlyPropertyAssignRule and ReadOnlyByPhpDocPropertyAssignRule (`property.readOnly*`, `assign.readOnlyProperty`
//! is not reported), ReadOnlyPropertyAssignRefRule and ReadOnlyByPhpDocPropertyAssignRefRule (`*AssignByRef`), and
//! from level 2 WritingToReadOnlyPropertiesRule (`assign.propertyReadOnly`) beyond `$this` (see read_only_write.rs).

use std::collections::HashSet;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::property::PropertyMetadata;
use mago_codex::visibility::Visibility;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_word::{Word, word};
use pun_phpstan::types::{NeverKind, Type, ancestor_args, decide_type, is_super_type_of, mixed, recommended_level};

use super::super::declarations::immutable_tags;
use super::super::doc::tags;
use super::super::overriding_property::{declaration_readonly, hook_names};
use super::super::property_access::{has_instance_property, has_static_property};
use super::super::{Checker, declared};
use super::flow::unparen;

pub const IDENTIFIERS: &[&str] = &[
    "assign.propertyType",
    "property.readOnlyAssignOutOfClass",
    "property.readOnlyAssignNotOnThis",
    "property.readOnlyAssignNotInConstructor",
    "property.readOnlyAssignByRef",
    "property.readOnlyByPhpDocAssignOutOfClass",
    "property.readOnlyByPhpDocAssignNotOnThis",
    "property.readOnlyByPhpDocAssignNotInConstructor",
    "property.readOnlyByPhpDocAssignByRef",
];

/// A property fetch: the expression holding it (`None`: a class name, resolved), its name with `$`, the name's span.
struct Fetch<'e, 'arena> {
    holder: Option<&'e Expression<'arena>>,
    class: Option<&'e Expression<'arena>>,
    name: Vec<u8>,
    at: Span,
    instance: bool,
}

fn fetch<'e, 'arena>(e: &'e Expression<'arena>) -> Option<Fetch<'e, 'arena>> {
    match unparen(e) {
        Expression::Access(Access::Property(p)) => {
            let ClassLikeMemberSelector::Identifier(id) = &p.property else { return None };
            Some(Fetch {
                holder: Some(p.object),
                class: None,
                name: [b"$", id.value].concat(),
                at: id.span(),
                instance: true,
            })
        }
        Expression::Access(Access::StaticProperty(p)) => {
            let Variable::Direct(v) = &p.property else { return None };
            Some(Fetch { holder: None, class: Some(p.class), name: v.name.to_vec(), at: v.span(), instance: false })
        }
        _ => None,
    }
}

/// The property fetches a write target writes (offsets of one too, `true`), as PropertyAssignNodes.
fn targets<'e, 'arena>(e: &'e Expression<'arena>, offset: bool, out: &mut Vec<(Fetch<'e, 'arena>, bool)>) {
    let e = unparen(e);
    if let Some(f) = fetch(e) {
        out.push((f, offset));
        return;
    }
    let elements = match e {
        Expression::ArrayAccess(a) => return targets(a.array, true, out),
        Expression::ArrayAppend(a) => return targets(a.array, true, out),
        Expression::List(l) => l.elements.as_slice(),
        Expression::Array(l) => l.elements.as_slice(),
        _ => return,
    };
    for el in elements {
        match el {
            ArrayElement::KeyValue(kv) => targets(kv.value, offset, out),
            ArrayElement::Value(v) => targets(v.value, offset, out),
            _ => {}
        }
    }
}

/// The holders whose types the rules read.
pub(super) fn typed(e: &Expression, spans: &mut Vec<Span>) {
    let mut out = Vec::new();
    match e {
        Expression::Assignment(a) => {
            targets(a.lhs, false, &mut out);
            spans.extend([a.rhs.span(), e.span()]);
            if let Expression::UnaryPrefix(u) = a.rhs {
                spans.push(u.operand.span());
                out.extend(fetch(u.operand).map(|f| (f, false)));
            }
        }
        Expression::UnaryPrefix(u) if u.operator.is_increment_or_decrement() => targets(u.operand, false, &mut out),
        Expression::UnaryPostfix(u) => targets(u.operand, false, &mut out),
        _ => {}
    }
    spans.extend(out.iter().filter_map(|(f, _)| f.holder.or(f.class)).map(|h| h.span()));
}

/// A FoundPropertyReflection.
struct Prop<'a> {
    declaring: &'a ClassLikeMetadata,
    meta: &'a PropertyMetadata,
    name: String,
    display: String,
    writable: Option<Type>,
    native_readonly: bool,
    phpdoc_readonly: bool,
    private_mutation: bool,
    private: bool,
}

const READONLY: [&str; 6] = [
    "@readonly",
    "@phan-read-only",
    "@psalm-readonly",
    "@phpstan-readonly",
    "@phpstan-readonly-allow-private-mutation",
    "@psalm-readonly-allow-private-mutation",
];
const PRIVATE_MUTATION: [&str; 4] = [
    "@phpstan-readonly-allow-private-mutation",
    "@phpstan-allow-private-mutation",
    "@psalm-readonly-allow-private-mutation",
    "@psalm-allow-private-mutation",
];

/// `$scope->getType()` of the fetch's holder (a class name's type for `Foo::$a`).
fn holder_type(c: &Checker, f: &Fetch) -> Option<Type> {
    if let Some(h) = f.holder {
        // (Mago types no `$this` it reads by reference)
        return c.phpstan_type(h).or_else(|| {
            let this = matches!(unparen(h), Expression::Variable(Variable::Direct(v)) if v.name == b"$this");
            c.class_scope().filter(|_| this).map(|k| Type::This(c.reflected_name(k.name.as_bytes())))
        });
    }
    let class = f.class?;
    let scope = c.class_scope();
    let name = |k: &ClassLikeMetadata| c.reflected_name(k.name.as_bytes());
    match unparen(class) {
        Expression::Identifier(id) => Some(Type::Object(c.reflected_name(c.resolved(id).as_bytes()), vec![])),
        Expression::Self_(_) => scope.map(|k| Type::Object(name(k), vec![])),
        Expression::Static(_) => scope.map(|k| Type::Static(name(k), vec![])),
        Expression::Parent(_) => {
            let parent = scope?.direct_parent_class.as_ref()?;
            Some(Type::Object(c.reflected_name(parent.as_bytes()), vec![]))
        }
        e => c.phpstan_type(e),
    }
}

/// Native modifiers only: Mago also marks PHPDoc-readonly properties readonly.
fn says_readonly(c: &Checker, at: Option<Span>) -> bool {
    let Some(at) = at else { return false };
    c.file_contents(at.file_id).is_some_and(|contents| declaration_readonly(at, &contents))
}

/// A class template `T` of `scope` as the holder's argument.
pub(super) fn with_args(t: &Type, scope: &str, names: &[String], args: &[Type]) -> Type {
    match t {
        Type::Template(tpl) if tpl.scope == scope => {
            names.iter().position(|n| *n == tpl.name).and_then(|i| args.get(i)).cloned().unwrap_or_else(|| t.clone())
        }
        _ => t.traverse(&|i| with_args(i, scope, names, args)),
    }
}

/// `findPropertyReflectionsFromNode()`: the property when every class of the holder has it.
fn find<'a>(c: &Checker<'a, '_>, f: &Fetch) -> Option<Prop<'a>> {
    find_on(c, f, holder_type(c, f)?, false, false)
}

enum UnsetReflection<'a> {
    Native(Prop<'a>),
    NonNative,
}

/// FoundPropertyReflection::getNativeReflection() unwraps a single native reflection, not a union/intersection
/// of properties or an annotation-only/enum reflection. An intersection with just one declared property keeps it.
pub(in super::super) fn native_unset_property<'a>(
    c: &Checker<'a, '_>,
    e: &Expression,
) -> Option<(&'a ClassLikeMetadata, &'a PropertyMetadata, bool, bool, bool)> {
    let f = fetch(e)?;
    let UnsetReflection::Native(p) = unset_reflection(c, &f, &holder_type(c, &f)?, false, &mut HashSet::new())? else { return None };
    Some((p.declaring, p.meta, p.native_readonly, p.phpdoc_readonly, p.private))
}

/// Instance-property existence and selection before native unwrapping. `NonNative` is a successful, terminal
/// reflection: it also contributes to an intersection even if another route names the same native declaration.
fn unset_reflection<'a>(
    c: &Checker<'a, '_>, f: &Fetch, holder: &Type, out_of_class: bool, active: &mut HashSet<Word>,
) -> Option<UnsetReflection<'a>> {
    let holder = holder.resolve_to_bounds();
    match &holder {
        Type::Union(types) | Type::Benevolent(types) => {
            return types.iter().all(|t| unset_reflection(c, f, t, out_of_class, active).is_some()).then_some(UnsetReflection::NonNative);
        }
        Type::Intersection(types) => {
            let mut properties = types.iter().filter_map(|t| unset_reflection(c, f, t, out_of_class, active));
            let first = properties.next()?;
            return Some(if properties.next().is_some() { UnsetReflection::NonNative } else { first });
        }
        _ => {}
    }
    let names = holder.object_class_names();
    let [name] = names.as_slice() else { return None };
    let class = c.codebase.get_class_like(name.as_bytes()).or_else(|| {
        // Conversion uses an anonymous class's display name, codex its storage key.
        c.codebase.class_likes.values().find(|k| k.name.as_bytes().starts_with(b"{anonymous-class:") && c.display(k) == *name).map(Box::as_ref)
    })?;
    if !active.insert(class.name) { return None }
    let property = if has_instance_property(c, class, &f.name) {
        Some(match find_on(c, f, Type::Object(class.name.to_string(), vec![]), true, out_of_class) {
            Some(p) => UnsetReflection::Native(p),
            None => UnsetReflection::NonNative,
        })
    } else {
        super::super::require_tags::property_requirements(c, class).iter()
            .find_map(|required| unset_reflection(c, f, required, true, active))
    };
    active.remove(&class.name);
    property
}

fn find_on<'a>(c: &Checker<'a, '_>, f: &Fetch, holder: Type, native_only: bool, out_of_class: bool) -> Option<Prop<'a>> {
    let names = holder.object_class_names();
    let classes: Vec<&'a ClassLikeMetadata> =
        names.iter().map(|n| c.codebase.get_class_like(n.as_bytes())).collect::<Option<_>>()?;
    let has = |k: &ClassLikeMetadata| {
        if f.instance { has_instance_property(c, k, &f.name) } else { has_static_property(c, k, &f.name) }
    };
    // ponytail: a union of classes reads the first one's property (UnionTypePropertyReflection merges them)
    let class = *classes.first().filter(|_| classes.iter().all(|k| has(k)))?;
    let name = String::from_utf8_lossy(&f.name[1..]).into_owned();
    let magic_property = || {
        let declaring = class.magic_property_ids.get(&word(&f.name))?;
        let meta = c.codebase.get_magic_property(declaring.as_bytes(), &f.name)?;
        Some((c.codebase.get_class_like(declaring.as_bytes())?, meta, true))
    };
    // the native property an `@property` replaces
    let mut shadowed = None;
    let (declaring, mut meta, mut magic) = match c.declared_property(class, &f.name) {
        // an `@property` at most as far up the hierarchy as the native property replaces it
        Some((declaring, origin, meta)) => match magic_property() {
            Some(m)
                if f.instance
                    && declaring.kind != mago_codex::symbol::SymbolKind::Enum
                    && (c.allows_dynamic_properties(class)
                        || if out_of_class { meta.read_visibility == Visibility::Public } else { can_access(c, declaring, meta.read_visibility) })
                    && (meta.read_visibility == Visibility::Public
                        || out_of_class
                        || c.class_scope().is_none_or(|k| k.name != declaring.name))
                    && distance(c, class, &m.0.name) <= distance(c, class, &origin.name) =>
            {
                shadowed = Some((declaring, meta));
                m
            }
            _ => (declaring, meta, false),
        },
        None => magic_property()?,
    };
    // PhpClassReflectionExtension's annotation overlay is public even when the underlying native property is private.
    let private = !magic && meta.read_visibility == Visibility::Private;
    if native_only && magic {
        // An annotation overlay keeps its declaring class, but its native reflection supplies modifiers/hooks.
        // An annotation-only property has no native reflection to unwrap.
        meta = shadowed?.1;
        magic = false;
    }
    if native_only && declaring.kind == mago_codex::symbol::SymbolKind::Enum {
        return None;
    }
    let declaring = if native_only && declaring.kind == mago_codex::symbol::SymbolKind::Trait {
        // Mago's appearing id still names the trait. PHP reflects its property on the class that directly
        // imports it (possibly through another trait), not on a later inheriting class.
        std::iter::successors(Some(class), |k| k.direct_parent_class.and_then(|p| c.codebase.get_class_like(p.as_bytes())))
            .take(super::super::overriding::ANCESTRY)
            .find(|k| k.direct_trait_names().any(|t| t == declaring.name))?
    } else {
        declaring
    };
    let ty = if magic {
        let readable = meta.type_metadata.as_ref().map_or_else(mixed, |t| c.to_phpstan(&t.type_union));
        match shadowed {
            Some((declaring, native)) => overlay_write_type(c, f, declaring, native, meta, readable),
            None => readable,
        }
    } else {
        declared::property_of(c, declaring, meta, &f.name[1..])
    };
    let mut display = c.display(declaring);
    let overlay_args = if magic && shadowed.is_some() {
        match &holder {
            Type::Object(_, args) | Type::Static(_, args) => Some(args.clone()),
            Type::This(_) => Some(declared::class_templates(c, class).into_iter().map(|t| Type::Template(Box::new(t))).collect()),
            _ => None,
        }
    } else {
        None
    };
    let ty = if let Some(args) = overlay_args {
        let scope = c.reflected_name(declaring.name.as_bytes());
        let args = if class.name == declaring.name {
            args
        } else {
            pun_phpstan::symbols::lookup(&c.reflected_name(class.name.as_bytes()))
                .and_then(|k| ancestor_args(&k, &scope, &args)).unwrap_or_default()
        };
        let templates = declared::class_templates(c, declaring);
        let args: Vec<_> = templates.iter().enumerate().map(|(i, t)| {
            args.get(i).cloned().unwrap_or_else(|| t.default.as_ref().unwrap_or(&t.bound).clone())
        }).collect();
        if !args.is_empty() {
            // ClassReflection::getDisplayName() uses type-only arguments and no separator spaces.
            display = format!("{display}<{}>", args.iter().map(|t| t.describe(pun_phpstan::types::Level::TypeOnly)).collect::<Vec<_>>().join(","));
        }
        let names = templates.iter().map(|t| t.name.clone()).collect::<Vec<_>>();
        with_args(&ty, &format!("class {scope}"), &names, &args)
    } else {
        match &holder {
            Type::Object(_, args) if !args.is_empty() => {
                let names: Vec<String> = declaring.get_template_type_names().iter().map(|n| n.to_string()).collect();
                with_args(&ty, &format!("class {}", c.reflected_name(declaring.name.as_bytes())), &names, args)
            }
            _ => ty,
        }
    };
    // an enum's `name` and `value` (EnumPropertyReflection) are neither writable nor readonly properties
    let enum_ = declaring.kind == mago_codex::symbol::SymbolKind::Enum;
    let writable = if enum_ {
        false
    } else if magic {
        // AnnotationsPropertiesClassReflectionExtension promotes a read-only tag only for a non-private,
        // non-static writable native property. The resulting PhpPropertyReflection still needs a set hook
        // when virtual, even if the annotation itself is writable.
        (!meta.flags.is_readonly() || shadowed.is_some_and(|(d, p)| {
            p.read_visibility != Visibility::Private && !p.flags.is_static() && native_writable(c, d, p, &f.name)
        })) && shadowed.is_none_or(|(d, p)| native_writable(c, d, p, &f.name))
    } else {
        native_writable(c, declaring, meta, &f.name)
    };
    let readonly = meta.flags.is_readonly() && !magic && !enum_;
    let native_readonly = readonly && (says_readonly(c, meta.name_span) || says_readonly(c, Some(declaring.span)));
    // `ClassReflection::isImmutable()` (up the parents) or the property's own readonly tag
    let doc = |span: Option<Span>| span.and_then(|s| declared::doc_at(c, s)).map(|(d, _)| tags(&d)).unwrap_or_default();
    let tagged = |span, names: &[&str]| doc(span).iter().any(|(t, _)| names.contains(&t.as_str()));
    let immutable = std::iter::once(&declaring.name)
        .chain(&declaring.all_parent_classes)
        .filter_map(|n| c.codebase.get_class_like(n.as_bytes()))
        .any(|k| immutable_tags(&c.class_tags(k)));
    let span = declared::property_span(c, meta);
    let phpdoc_readonly = !magic && !enum_ && !native_readonly && (immutable || tagged(span, &READONLY));
    Some(Prop {
        declaring,
        meta,
        name,
        display,
        writable: writable.then_some(ty),
        phpdoc_readonly,
        private_mutation: tagged(span, &PRIVATE_MUTATION),
        native_readonly,
        private,
    })
}

fn native_writable(c: &Checker, declaring: &ClassLikeMetadata, p: &PropertyMetadata, name: &[u8]) -> bool {
    p.flags.is_static() || !c.property_virtual(declaring, p, name) || c.property_hooks(declaring, hook_names(p), name).1
}

/// PhpClassReflectionExtension's overlay and PhpPropertyReflection::getWritableType(): an absent writable
/// PHPDoc type (`@property-read`) falls back to the readable PHPDoc, not to the original native/`@var` type.
fn overlay_write_type(
    c: &Checker, f: &Fetch, declaring: &ClassLikeMetadata, native: &PropertyMetadata,
    annotation: &PropertyMetadata, readable: Type,
) -> Type {
    let writable = annotation.write_type_metadata.as_ref().map(|t| c.to_phpstan(&t.type_union))
        .or_else(|| annotation.flags.is_writeonly().then(|| readable.clone()));
    let readable = if annotation.flags.is_writeonly() { Type::Never(NeverKind::Implicit) } else { readable };
    let native_type = native.type_declaration_metadata.as_ref()
        .map_or_else(mixed, |t| declared::from_tunion(c, &t.type_union, declaring));
    let native_type = if is_super_type_of(&native_type, &readable).yes()
        || !can_access(c, declaring, native.read_visibility)
    {
        mixed()
    } else {
        native_type
    };
    // TypesAssignedToPropertiesRule checks a write to the backing value against getReadableType().
    if in_own_hook(f, native) {
        return decide_type(native_type, Some(readable));
    }
    // A native set hook's parameter takes precedence over the annotation, including inherited hooks.
    let mut property = Some((declaring, native));
    for _ in 0..super::super::overriding::ANCESTRY {
        let Some((declaring, p)) = property else { break };
        if let Some(parameter) = p.hooks.get(&word(b"set")).and_then(|h| h.parameter.as_ref()) {
            // Mago copies the property's effective PHPDoc type into an implicit parameter's native slot.
            let native = parameter.type_declaration_metadata.as_ref().filter(|t| !t.from_docblock)
                .or(p.type_declaration_metadata.as_ref());
            let native = native
                .map_or_else(mixed, |t| declared::from_tunion(c, &t.type_union, declaring));
            let phpdoc = parameter.type_metadata.as_ref().filter(|t| t.from_docblock)
                .or(p.type_metadata.as_ref().filter(|t| t.from_docblock)).map(|t| c.to_phpstan(&t.type_union));
            return decide_type(native, phpdoc);
        }
        property = declaring.direct_parent_class.and_then(|n| c.codebase.get_class_like(n.as_bytes()))
            .and_then(|parent| c.declared_property(parent, &f.name).map(|(d, _, p)| (d, p)));
    }
    match writable {
        Some(writable) if !matches!(writable, Type::Never(_)) && writable != readable => writable,
        _ => decide_type(native_type, Some(readable)),
    }
}

/// A write of `$this->p` in one of `p`'s own hooks (`getHookedPropertyName()`).
fn in_own_hook(f: &Fetch, p: &PropertyMetadata) -> bool {
    let this =
        f.holder.is_some_and(|h| matches!(unparen(h), Expression::Variable(Variable::Direct(v)) if v.name == b"$this"));
    this && p.span.is_some_and(|s| s.has_offset(f.at.start.offset))
}

/// `ClassReflection::getClassHierarchyDistances()`'s distance of `of` from `class` (its traits, then each parent and
/// its traits).
fn distance(c: &Checker, class: &ClassLikeMetadata, of: &Word) -> usize {
    let mut at = 0;
    let mut current = Some(class);
    loop {
        let Some(k) = current else { return usize::MAX };
        if k.name == *of || k.used_traits.contains(of) {
            return at + usize::from(k.name != *of);
        }
        at += 1 + k.used_traits.len();
        current = k.direct_parent_class.and_then(|p| c.codebase.get_class_like(p.as_bytes()));
    }
}

/// `$scope->canReadProperty()` or `canWriteProperty()` for a member of `declaring` with visibility `v`.
fn can_access(c: &Checker, declaring: &ClassLikeMetadata, v: Visibility) -> bool {
    let inside = |k: &ClassLikeMetadata| k.name == declaring.name || k.all_parent_classes.contains(&declaring.name);
    match (v, c.class_scope()) {
        (Visibility::Public, _) => true,
        (_, None) => false,
        (Visibility::Protected, Some(k)) => inside(k) || declaring.all_parent_classes.contains(&k.name),
        (Visibility::Private, Some(k)) => k.name == declaring.name,
    }
}

/// `$scope->canWriteProperty()`.
fn can_write(c: &Checker, p: &Prop) -> bool {
    can_access(c, p.declaring, p.meta.write_visibility)
}

/// ReadOnlyPropertyAssignRule and ReadOnlyByPhpDocPropertyAssignRule for one write.
fn read_only(c: &mut Checker, f: &Fetch, p: &Prop, offset: bool) {
    if !f.instance || !can_write(c, p) || !(p.native_readonly || p.phpdoc_readonly) {
        return;
    }
    if p.phpdoc_readonly && in_own_hook(f, p.meta) {
        return;
    }
    let (kind, id) = if p.native_readonly { ("Readonly", "readOnly") } else { ("@readonly", "readOnlyByPhpDoc") };
    let what = format!("{kind} property {}::${}", c.display(p.declaring), p.name);
    let report = |c: &mut Checker, suffix: &str, message: &str| {
        c.report(f.at, &format!("property.{id}{suffix}"), format!("{what} {message}"), None);
    };
    let Some(scope) = c.class_scope() else {
        return report(c, "AssignOutOfClass", "is assigned outside of its declaring class.");
    };
    if scope.name != p.declaring.name {
        // supportsAsymmetricVisibility(): a subclass may write a readonly property that is not private(set)
        let subclass = p.native_readonly
            && c.php_version >= 80400
            && p.meta.write_visibility != Visibility::Private
            && scope.all_parent_classes.contains(&p.declaring.name);
        if !subclass {
            return report(c, "AssignOutOfClass", "is assigned outside of its declaring class.");
        }
    }
    let method = c.functions.last().map(|m| m.name.to_ascii_lowercase()).unwrap_or_default();
    let clone = p.native_readonly && c.php_version >= 80300 && method == "__clone";
    if method == "__construct" || method == "__unserialize" || clone {
        let on_this = f.holder.and_then(|h| c.phpstan_type(h)).is_some_and(|t| find_this(&t));
        if !on_this {
            report(c, "AssignNotOnThis", "is not assigned on $this.");
        }
        // ponytail: `is already assigned` (assign.readOnlyProperty) needs the clone's reinitialisation tracking
        return;
    }
    // isAllowedPrivateMutation()
    let allowed = p.phpdoc_readonly && p.private_mutation;
    // isArrayAccessOffsetWrite()
    let array_access = Type::Object("ArrayAccess".into(), vec![]);
    if !allowed && !(offset && p.writable.as_ref().is_some_and(|t| is_super_type_of(&array_access, t).yes())) {
        report(c, "AssignNotInConstructor", "is assigned outside of the constructor.");
    }
}

/// WritingToReadOnlyPropertiesRule of a fetch not on `$this` (whose writes read_only_write.rs checks at every level).
fn not_writable(c: &mut Checker, f: &Fetch, p: &Prop) {
    let this = |h: &Expression| matches!(unparen(h), Expression::Variable(Variable::Direct(v)) if v.name == b"$this");
    let enum_ = p.declaring.kind == mago_codex::symbol::SymbolKind::Enum;
    let Some(holder) = f.holder.filter(|h| enum_ || !this(h)) else { return };
    if p.writable.is_some() || !can_write(c, p) {
        return;
    }
    let Some(on) = c.phpstan_type(holder) else { return };
    let declaring = Type::Object(c.reflected_name(p.declaring.name.as_bytes()), vec![]);
    let class = declared::with_classes(c, || {
        if is_super_type_of(&declaring, &on).yes() {
            c.display(p.declaring)
        } else {
            on.describe(pun_phpstan::types::Level::TypeOnly)
        }
    });
    c.report(f.at, "assign.propertyReadOnly", format!("Property {class}::${} is not writable.", p.name), None);
}

/// `TypeUtils::findThisType()`.
// ponytail: Mago's union of `$this` and its class is the class, so only a plain `$this` counts
fn find_this(t: &Type) -> bool {
    matches!(t, Type::This(_))
}

/// TypesAssignedToPropertiesRule for a direct write of `value`.
fn assigned(c: &mut Checker, f: &Fetch, p: &Prop, value: Option<Type>, at: Span) {
    let (Some(ty), Some(value)) = (&p.writable, value) else { return };
    let strict = super::returns::strict_types(c);
    let levels = c.levels();
    let kind = if f.instance { "Property" } else { "Static property" };
    let message = declared::with_classes(c, || {
        if levels.accepts(ty, &value, strict) {
            return None;
        }
        let level = recommended_level(ty, Some(&value));
        Some(format!(
            "{kind} {}::${} ({}) does not accept {}.",
            p.display,
            p.name,
            ty.describe(level),
            value.describe(level)
        ))
    });
    if let Some(message) = message {
        c.report(at, "assign.propertyType", message, None);
    }
}

pub(in super::super) fn assignment(c: &mut Checker, a: &Assignment) {
    if c.level < 2 {
        return;
    }
    let mut writes = Vec::new();
    targets(a.lhs, false, &mut writes);
    // `= &$x`: the referenced side as written, `.=` and co.: the result
    let by_ref = match a.rhs {
        Expression::UnaryPrefix(u) if u.operator.is_reference() => Some(u.operand),
        _ => None,
    };
    let value = match (&a.operator, by_ref) {
        (AssignmentOperator::Assign(_), Some(r)) => Some(r),
        (AssignmentOperator::Assign(_), None) => Some(a.rhs),
        _ => None,
    };
    // ponytail: the values a list destructuring gives its properties are not checked
    let direct = fetch(a.lhs).is_some();
    for (f, offset) in &writes {
        let Some(p) = declared::with_classes(c, || find(c, f)) else { continue };
        not_writable(c, f, &p);
        if c.level < 3 {
            continue;
        }
        read_only(c, f, &p, *offset);
        if direct && !offset {
            let ty = match value {
                Some(v) => c.phpstan_type(v),
                None => c.types.get(&a.span()).map(|t| c.to_phpstan(t)),
            };
            assigned(c, f, &p, ty, value.unwrap_or(a.rhs).span());
        }
    }
    let Some(f) = by_ref.and_then(fetch).filter(|f| f.instance && c.level >= 3) else { return };
    let Some(p) = declared::with_classes(c, || find(c, &f)) else { return };
    if (p.native_readonly || p.phpdoc_readonly) && can_write(c, &p) {
        let (kind, id) = if p.native_readonly { ("Readonly", "readOnly") } else { ("@readonly", "readOnlyByPhpDoc") };
        let message = format!("{kind} property {}::${} is assigned by reference.", c.display(p.declaring), p.name);
        c.report(a, &format!("property.{id}AssignByRef"), message, None);
    }
}

/// `++`/`--` of a property (a write, but its type is not checked here).
pub(in super::super) fn increment(c: &mut Checker, operand: &Expression) {
    writes(c, operand, false);
}

/// `unset($this->p[$k])`, a write of `$this->p` (but not `unset($this->p)`).
pub(in super::super) fn unset(c: &mut Checker, u: &Unset) {
    for v in u.values.iter() {
        writes(c, v, true);
    }
}

fn writes(c: &mut Checker, e: &Expression, offsets_only: bool) {
    if c.level < 2 {
        return;
    }
    let mut writes = Vec::new();
    targets(e, false, &mut writes);
    for (f, offset) in writes.iter().filter(|(_, offset)| *offset || !offsets_only) {
        let Some(p) = declared::with_classes(c, || find(c, f)) else { continue };
        not_writable(c, f, &p);
        if c.level >= 3 {
            read_only(c, f, &p, *offset);
        }
    }
}

/// The properties a write target writes (`findPropertyReflectionsFromNode()`), each with its declaring class, its
/// name (without `$`), whether the target is the property itself (not an offset of it or a list element) and whether
/// it writes an offset of the property, `None` for one not found.
pub(super) fn written<'a>(
    c: &Checker<'a, '_>,
    target: &Expression,
) -> Vec<Option<(&'a ClassLikeMetadata, String, bool, bool)>> {
    let mut out = Vec::new();
    targets(target, false, &mut out);
    let direct = fetch(target).is_some();
    let dynamic = matches!(unparen(target), Expression::Access(Access::Property(_) | Access::StaticProperty(_))) && !direct;
    let found = out.iter().map(|(f, offset)| declared::with_classes(c, || find(c, f)).map(|p| (p.declaring, p.name, direct && !offset, *offset)));
    // a dynamic name finds no property
    found.chain(dynamic.then_some(None)).collect()
}
