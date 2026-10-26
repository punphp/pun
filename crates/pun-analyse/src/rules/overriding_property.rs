//! OverridingPropertyRule (ClassPropertyNode): the property's prototype is the parent's non-private native property,
//! else the first interface's. User classes come from Mago's codex, built-in ones from PHPStan's reflection
//! (data/native-methods.json). checkPhpDocMethodSignatures (property.phpDocType) and
//! checkMissingOverridePropertyAttribute are off below level 3.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

use mago_allocator::LocalArena;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::property::PropertyMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::visibility::Visibility;
use mago_database::file::{File, FileId};
use mago_span::{HasSpan, Position, Span};
use mago_syntax::cst::{AnonymousClass, Class as ClassNode, ClassLikeConstant, Constant, FunctionLikeParameter, Interface, Modifier, Property};
use mago_syntax::parser::parse_file;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Level, Type, is_super_type_of};

use super::declarations::{doc_tags, has_tag, is_abstract, is_private};
use super::declared;
use super::overriding::{ANCESTRY, Class};
use super::properties::{PropertyNode, is_virtual};
use super::Checker;

/// The prototype as PhpPropertyReflection gives it.
struct Prototype {
    /// `getDeclaringClass()->getDisplayName()`
    class: String,
    is_public: bool,
    is_private: bool,
    is_static: bool,
    readonly: bool,
    is_abstract: bool,
    final_by_keyword: bool,
    is_final: bool,
    is_virtual: bool,
    /// `hasHook('get')`, `hasHook('set')`
    hooks: (bool, bool),
    readable: bool,
    writable: bool,
    native: Option<Type>,
}

/// `isReadable()` / `isWritable()` of a native property: a virtual one needs the hook.
fn accessible(is_static: bool, is_virtual: bool, hook: bool) -> bool {
    is_static || !is_virtual || hook
}

struct DeclarationIndex {
    starts: Arc<HashMap<u32, u32>>,
    readonly: HashSet<u32>,
    interfaces: HashMap<u32, Vec<Span>>,
}

/// Declaration boundaries, native readonly modifiers and ordered interface parents, parsed once per file content.
fn declaration_index(id: FileId, contents: &[u8]) -> Arc<DeclarationIndex> {
    #[derive(Default)]
    struct Starts {
        starts: HashMap<u32, u32>,
        readonly: HashSet<u32>,
        interfaces: HashMap<u32, Vec<Span>>,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Starts {
        fn walk_in_property(&mut self, p: &'ast Property<'arena>, _: &mut ()) {
            for v in p.variables() {
                self.starts.insert(v.span.start.offset, p.span().start.offset);
                if p.modifiers().iter().any(Modifier::is_readonly) {
                    self.readonly.insert(v.span.start.offset);
                }
            }
        }
        fn walk_in_function_like_parameter(&mut self, p: &'ast FunctionLikeParameter<'arena>, _: &mut ()) {
            if p.modifiers.iter().any(Modifier::is_readonly) {
                self.readonly.insert(p.variable.span.start.offset);
            }
        }
        fn walk_in_class(&mut self, c: &'ast ClassNode<'arena>, _: &mut ()) {
            if c.modifiers.iter().any(Modifier::is_readonly) {
                self.readonly.insert(c.span().start.offset);
            }
        }
        fn walk_in_anonymous_class(&mut self, c: &'ast AnonymousClass<'arena>, _: &mut ()) {
            if c.modifiers.iter().any(Modifier::is_readonly) {
                self.readonly.insert(c.span().start.offset);
            }
        }
        fn walk_in_interface(&mut self, i: &'ast Interface<'arena>, _: &mut ()) {
            self.interfaces.insert(i.span().start.offset, i.extends.iter().flat_map(|e| e.types.iter().map(HasSpan::span)).collect());
        }
        fn walk_in_class_like_constant(&mut self, k: &'ast ClassLikeConstant<'arena>, _: &mut ()) {
            for item in k.items.iter() {
                self.starts.insert(item.span().start.offset, k.span().start.offset);
            }
        }
        fn walk_in_constant(&mut self, k: &'ast Constant<'arena>, _: &mut ()) {
            for item in k.items.iter() {
                self.starts.insert(item.span().start.offset, k.span().start.offset);
            }
        }
    }
    static CACHE: OnceLock<Mutex<HashMap<(FileId, u64), Arc<DeclarationIndex>>>> = OnceLock::new();
    let key = (id, {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        contents.hash(&mut h);
        h.finish()
    });
    let cache = CACHE.get_or_init(Default::default);
    if let Some(found) = cache.lock().expect("starts cache").get(&key) {
        return found.clone();
    }
    let arena = LocalArena::new();
    let file = File::ephemeral(
        std::borrow::Cow::Borrowed(b"starts.php".as_slice()),
        std::borrow::Cow::Owned(contents.to_vec()),
    );
    let mut starts = Starts::default();
    starts.walk_program(parse_file(&arena, &file), &mut ());
    let starts = Arc::new(DeclarationIndex { starts: Arc::new(starts.starts), readonly: starts.readonly, interfaces: starts.interfaces });
    cache.lock().expect("starts cache").insert(key, starts.clone());
    starts
}

/// Where each property/constant declaration starts, by the offset of each item's name.
pub(super) fn declaration_starts(id: FileId, contents: &[u8]) -> Arc<HashMap<u32, u32>> {
    declaration_index(id, contents).starts.clone()
}

/// A property's own native modifier, or a class's readonly modifier (not PHPDoc, comments or attributes).
pub(super) fn declaration_readonly(at: Span, contents: &[u8]) -> bool {
    declaration_index(at.file_id, contents).readonly.contains(&at.start.offset)
}

/// Nominal interface parents in declaration order (codex's populated interface sets are unordered).
pub(super) fn declaration_interfaces<'a>(c: &Checker<'a, '_>, k: &ClassLikeMetadata) -> Vec<&'a ClassLikeMetadata> {
    let Some(contents) = c.file_contents(k.span.file_id) else { return Vec::new() };
    let index = declaration_index(k.span.file_id, &contents);
    let scope = super::doc::scopes_of(k.span.file_id, &contents).at(k.span.start.offset);
    index.interfaces.get(&k.span.start.offset).into_iter().flatten().filter_map(|span| {
        let name = scope.resolve(mago_names::kind::NameKind::Default, &contents[span.start.offset as usize..span.end.offset as usize]).0;
        c.codebase.get_class_like(&name).filter(|i| i.kind == SymbolKind::Interface)
    }).collect()
}

impl<'a> Checker<'a, '_> {
    /// The span of property `p`'s declaration (from its attributes or modifiers), and of its name.
    pub(super) fn declaration(&self, p: &PropertyMetadata) -> Option<(Span, Span)> {
        let name = p.name_span?;
        if let Some(span) = p.span {
            return Some((span, name));
        }
        let contents = self.file_contents(name.file_id)?;
        let start = *declaration_starts(name.file_id, &contents).get(&name.start.offset)?;
        Some((Span::new(name.file_id, Position::new(start), name.end), name))
    }

    /// The declared property `name` (with its `$`) of user class `class` (its own, a trait's or an ancestor's): the
    /// class PHP reports declaring it, the class-like whose code declares it, and the property.
    pub(super) fn declared_property(
        &self,
        class: &'a ClassLikeMetadata,
        name: &[u8],
    ) -> Option<(&'a ClassLikeMetadata, &'a ClassLikeMetadata, &'a PropertyMetadata)> {
        let origin = self.codebase.get_declaring_property_class(class.name.as_bytes(), name)?;
        let origin = self.codebase.get_class_like(origin.as_bytes())?;
        let property = origin.properties.get(&mago_word::word(name))?;
        let declaring = if origin.kind == SymbolKind::Trait {
            let appearing = class.appearing_property_ids.get(&mago_word::word(name))?;
            self.codebase.get_class_like(appearing.as_bytes())?
        } else {
            origin
        };
        Some((declaring, origin, property))
    }

    /// BetterReflection's `getHooks()` of the property `name` declared in `class`: its own, else its parent class's.
    pub(super) fn property_hooks(&self, class: &ClassLikeMetadata, own: (bool, bool), name: &[u8]) -> (bool, bool) {
        self.property_hooks_at(class, own, name, 0)
    }

    fn property_hooks_at(&self, class: &ClassLikeMetadata, own: (bool, bool), name: &[u8], depth: usize) -> (bool, bool) {
        if (own.0 && own.1) || depth > ANCESTRY {
            return own;
        }
        let parent = class
            .direct_parent_class
            .and_then(|p| self.codebase.get_class_like(p.as_bytes()))
            .and_then(|p| self.declared_property(p, name));
        match parent {
            Some((d, _, p)) => {
                let inherited = self.property_hooks_at(d, hook_names(p), name, depth + 1);
                (own.0 || inherited.0, own.1 || inherited.1)
            }
            None => own,
        }
    }

    /// The native property `name` (with its `$`) of `class` as PhpClassReflectionExtension finds it (a parent's
    /// private one excluded): the class declaring it and `isReadable()`.
    pub(super) fn native_readable(&self, class: &'a ClassLikeMetadata, name: &[u8]) -> Option<(String, bool)> {
        let (declaring, _, p) = self.declared_property(class, name)?;
        if p.read_visibility == Visibility::Private && declaring.name != class.name {
            return None;
        }
        let virtual_ = self.property_virtual(declaring, p, name);
        let hooks = self.property_hooks(declaring, hook_names(p), name);
        Some((self.display(declaring), accessible(p.flags.is_static(), virtual_, hooks.0)))
    }

    /// BetterReflection's `isVirtual()`: virtual hooks, and a virtual parent property if there is one.
    pub(super) fn property_virtual(&self, class: &ClassLikeMetadata, p: &PropertyMetadata, name: &[u8]) -> bool {
        self.property_virtual_at(class, p, name, 0)
    }

    fn property_virtual_at(&self, class: &ClassLikeMetadata, p: &PropertyMetadata, name: &[u8], depth: usize) -> bool {
        p.flags.is_virtual_property()
            && (depth > ANCESTRY
                || class
                    .direct_parent_class
                    .and_then(|parent| self.codebase.get_class_like(parent.as_bytes()))
                    .and_then(|parent| self.declared_property(parent, name))
                    .is_none_or(|(d, _, p)| self.property_virtual_at(d, p, name, depth + 1)))
    }

    /// `getNativeProperty($name)` of `class` as class `k` reaches it.
    fn native_property(&self, class: Class<'a>, name: &str, k: &ClassLikeMetadata) -> Option<Prototype> {
        match class {
            Class::Builtin(mut b) => loop {
                if let Some(p) = b.properties.get(name) {
                    return Some(Prototype {
                        class: self.ancestor_display(k, &b.name),
                        is_public: p.is_public,
                        is_private: false,
                        is_static: p.is_static,
                        readonly: p.readonly,
                        is_abstract: p.is_abstract,
                        final_by_keyword: p.is_final,
                        is_final: p.final_doc,
                        is_virtual: p.virtual_,
                        hooks: p.hooks,
                        readable: p.readable,
                        writable: p.writable,
                        native: p.native.clone(),
                    });
                }
                // a built-in class's parent is built in
                b = super::overriding::natives(self.php_version).get(&b.parent.as_deref()?.to_ascii_lowercase())?;
            },
            Class::User(u) => {
                let dollar = format!("${name}");
                let (declaring, origin, p) = self.declared_property(u, dollar.as_bytes())?;
                let declaration = self.declaration(p);
                let modifier = |m: &[u8]| {
                    declaration
                        .and_then(|(span, at)| self.has_modifier(span, at, m))
                        .unwrap_or(false)
                };
                let class_readonly = declaring
                    .name_span
                    .and_then(|at| self.has_modifier(declaring.span, at, b"readonly"))
                    .unwrap_or(false);
                let private_set =
                    p.write_visibility == Visibility::Private && p.read_visibility != Visibility::Private;
                let final_by_keyword = modifier(b"final") || private_set;
                let doc_final = declaration
                    .and_then(|(span, _)| declared::doc_at(self, span))
                    .is_some_and(|(doc, _)| has_tag(&doc_tags(&doc), &["@final"]));
                let is_static = p.flags.is_static();
                let is_virtual = self.property_virtual(declaring, p, dollar.as_bytes());
                let hooks = self.property_hooks(declaring, hook_names(p), dollar.as_bytes());
                let native = p.type_declaration_metadata.as_ref().map(|t| {
                    self.localize(&declared::from_tunion(self, &t.type_union, origin), origin, declaring)
                });
                Some(Prototype {
                    class: self.ancestor_display(k, &self.display(declaring)),
                    is_public: p.read_visibility == Visibility::Public,
                    is_private: p.read_visibility == Visibility::Private,
                    is_static,
                    readonly: modifier(b"readonly") || class_readonly,
                    is_abstract: modifier(b"abstract") || declaring.kind == SymbolKind::Interface,
                    final_by_keyword,
                    is_final: final_by_keyword || doc_final,
                    is_virtual,
                    hooks,
                    readable: accessible(is_static, is_virtual, hooks.0),
                    writable: accessible(is_static, is_virtual, hooks.1),
                    native,
                })
            }
        }
    }

    /// OverridingPropertyRule::findPrototype().
    fn property_prototype(&self, k: &'a ClassLikeMetadata, name: &str) -> Option<Prototype> {
        if let Some(parent) = self.parent_class(Class::User(k))
            && let Some(p) = self.native_property(parent, name, k)
            && !p.is_private
        {
            return Some(p);
        }
        self.all_interfaces(Class::User(k))
            .into_iter()
            .find_map(|i| self.native_property(i, name, k))
    }
}

/// The hooks a declared property has itself.
pub(super) fn hook_names(p: &PropertyMetadata) -> (bool, bool) {
    let has = |h: &[u8]| p.hooks.contains_key(&mago_word::word(h));
    (has(b"get"), has(b"set"))
}

/// OverridingPropertyRule on a ClassPropertyNode, `is_readonly` being its `isReadOnly()`.
pub fn check(c: &mut Checker, node: &PropertyNode, is_readonly: bool) {
    // a trait's property is walked only in a using class's context, where the scope is that class
    let k = c.class_scope().expect("a property is walked inside its class");
    let errors = declared::with_classes(c, || errors(c, node, is_readonly, k));
    for (identifier, message, ignorable) in errors {
        if ignorable {
            c.report(node.at, identifier, message, None);
        } else {
            c.report_non_ignorable(node.at, identifier, message, None);
        }
    }
}

fn errors<'a>(
    c: &Checker<'a, '_>,
    node: &PropertyNode,
    is_readonly: bool,
    k: &'a ClassLikeMetadata,
) -> Vec<(&'static str, String, bool)> {
    let name = String::from_utf8_lossy(&node.name[1..]).into_owned();
    let class = c.display(k);
    let Some(prototype) = c.property_prototype(k, &name) else {
        let override_attribute = node
            .attributes
            .iter()
            .flat_map(|l| l.attributes.iter())
            .any(|a| c.resolved(&a.name).eq_ignore_ascii_case("override"));
        if override_attribute {
            return vec![(
                "property.override",
                format!("Property {class}::${name} has #[\\Override] attribute but does not override any property."),
                false,
            )];
        }
        return vec![];
    };
    let proto = &prototype.class;
    let mut errors = Vec::new();
    let mut error = |identifier, message| errors.push((identifier, message, false));
    let is_static = node.is_static();
    if prototype.is_static {
        if !is_static {
            error(
                "property.nonStatic",
                format!("Non-static property {class}::${name} overrides static property {proto}::${name}."),
            );
        }
    } else if is_static {
        error(
            "property.static",
            format!("Static property {class}::${name} overrides non-static property {proto}::${name}."),
        );
    }
    let hooks_supported = c.php_version >= 80400;
    if prototype.readonly {
        if !is_readonly {
            error(
                "property.readWrite",
                format!("Readwrite property {class}::${name} overrides readonly property {proto}::${name}."),
            );
        }
    } else if is_readonly && (!hooks_supported || prototype.writable) {
        error(
            "property.readOnly",
            format!("Readonly property {class}::${name} overrides readwrite property {proto}::${name}."),
        );
    }
    if hooks_supported {
        let own = (
            node.hooks.iter().any(|h| h.name.value == b"get"),
            node.hooks.iter().any(|h| h.name.value == b"set"),
        );
        let hooks = c.property_hooks(k, own, node.name);
        let is_virtual = is_virtual(c, node);
        if prototype.readable && !accessible(is_static, is_virtual, hooks.0) {
            error(
                "property.notReadable",
                format!(
                    "Property {class}::${name} overriding readable property {proto}::${name} also has to be readable."
                ),
            );
        }
        if prototype.writable && !accessible(is_static, is_virtual, hooks.1) {
            error(
                "property.notWritable",
                format!(
                    "Property {class}::${name} overriding writable property {proto}::${name} also has to be writable."
                ),
            );
        }
        if node.has(is_abstract) && !prototype.is_abstract {
            for (hook, own, theirs) in [("get", hooks.0, prototype.hooks.0), ("set", hooks.1, prototype.hooks.1)] {
                if own && theirs {
                    error(
                        "property.abstractOverridingNonAbstractHook",
                        format!(
                            "Cannot make non-abstract method {proto}::${name}::{hook}() abstract in class {class}."
                        ),
                    );
                }
            }
        }
    }
    if prototype.is_public {
        if !node.is_public() {
            let visibility = if node.has(is_private) { "Private" } else { "Protected" };
            error(
                "property.visibility",
                format!(
                    "{visibility} property {class}::${name} overriding public property {proto}::${name} should also be public."
                ),
            );
        }
    } else if node.has(is_private) {
        error(
            "property.visibility",
            format!(
                "Private property {class}::${name} overriding protected property {proto}::${name} should be protected or public."
            ),
        );
    }
    if prototype.final_by_keyword {
        error(
            "property.parentPropertyFinal",
            format!("Property {class}::${name} overrides final property {proto}::${name}."),
        );
    } else if prototype.is_final {
        errors.push((
            "property.parentPropertyFinalByPhpDoc",
            format!("Property {class}::${name} overrides @final property {proto}::${name}."),
            true,
        ));
    }
    let native = node.hint.map(|h| declared::native(c, h, Some(k)));
    let describe = |t: &Type| t.describe(Level::TypeOnly);
    match (&prototype.native, native) {
        (Some(theirs), None) => errors.push((
            "property.missingNativeType",
            format!(
                "Property {class}::${name} overriding property {proto}::${name} ({0}) should also have native type {0}.",
                describe(theirs)
            ),
            false,
        )),
        // Type::equals(): the same type, however its union is ordered
        (Some(theirs), Some(ours))
            if !(is_super_type_of(theirs, &ours).yes() && is_super_type_of(&ours, theirs).yes()) =>
        {
            let (variance, ok) = if hooks_supported
                && (prototype.is_virtual || prototype.is_abstract)
                && (!prototype.readable || !prototype.writable)
            {
                if !prototype.readable {
                    ("contravariant", is_super_type_of(&ours, theirs).yes())
                } else {
                    ("covariant", is_super_type_of(theirs, &ours).yes())
                }
            } else {
                ("the same as", false)
            };
            if !ok {
                let variance = if variance == "the same as" { variance.to_string() } else { format!("{variance} with") };
                errors.push((
                    "property.nativeType",
                    format!(
                        "Type {} of property {class}::${name} is not {variance} type {} of overridden property {proto}::${name}.",
                        describe(&ours),
                        describe(theirs)
                    ),
                    false,
                ));
            }
        }
        (None, Some(ours)) => errors.push((
            "property.extraNativeType",
            format!(
                "Property {class}::${name} ({}) overriding property {proto}::${name} should not have a native type.",
                describe(&ours)
            ),
            false,
        )),
        _ => {}
    }
    errors
}
