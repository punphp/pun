//! OverridingMethodRule and ConsistentConstructorRule (InClassMethodNode), with MethodPrototypeFinder,
//! MethodVisibilityComparisonHelper, MethodParameterComparisonHelper and BetterReflection's
//! `ReflectionMethod::getPrototype()` (the tentative return types). User classes come from Mago's codex, built-in
//! ones from PHPStan's reflection (data/native-methods.json, written by scripts/export-native-methods.php).
//! checkPhpDocMethodSignatures (MethodSignatureRule) and checkMissingOverrideMethodAttribute are off below level 3.
//! ponytail: PHP 8.1 and up only, so the helpers' PHP 7 branches (no contravariance, covariance, tentative return
//! types or abstract trait methods) are left out; the trait walk still breaks competing declarations by name.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::OnceLock;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::flags::MetadataFlags;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::metadata::parameter::FunctionLikeParameterMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::visibility::Visibility;
use mago_syntax::cst::*;
use pun_phpstan::resolve::resolve_str;
use pun_phpstan::symbols;
use pun_phpstan::types::{Level, Type, intersect, is_super_type_of, mixed, union};
use serde_json::Value;

use super::declarations::{doc_tags, has_tag, is_abstract, is_private};
use super::declared::{self, default_is_null, original_name};

/// How many ancestors the walks up a class hierarchy visit: a circular one (which PHP refuses) ends there.
pub(super) const ANCESTRY: usize = 64;
use super::Checker;

const TENTATIVE_TIP: &str = "Make it covariant, or use the #[\\ReturnTypeWillChange] attribute to temporarily suppress the error.";

/// A parameter as `ExtendedParameterReflection` gives it.
struct Param {
    name: String,
    native: Type,
    optional: bool,
    variadic: bool,
    by_ref: bool,
}

/// A built-in method as NativeMethodReflection gives it.
struct NativeMethod {
    name: String,
    visibility: Visibility,
    is_static: bool,
    is_abstract: bool,
    is_final: bool,
    final_doc: bool,
    builtin: bool,
    variants: usize,
    params: Vec<Param>,
    ret: Type,
    tentative: Option<Type>,
    prototype: Option<String>,
    will_change: bool,
}

pub(super) struct NativeClass {
    pub(super) name: String,
    pub(super) parent: Option<String>,
    interfaces: Vec<String>,
    methods: HashMap<String, NativeMethod>,
    /// The non-private properties it declares, by name.
    pub(super) properties: HashMap<String, NativeProperty>,
    /// `ClassConstantReflection::getNativeType()`, not the constant's inferred value type.
    pub(super) constant_types: HashMap<String, Type>,
}

/// A built-in property as PhpPropertyReflection gives it.
pub(super) struct NativeProperty {
    pub(super) is_public: bool,
    pub(super) is_static: bool,
    pub(super) readonly: bool,
    pub(super) is_abstract: bool,
    pub(super) is_final: bool,
    pub(super) final_doc: bool,
    pub(super) virtual_: bool,
    pub(super) hooks: (bool, bool),
    pub(super) readable: bool,
    pub(super) writable: bool,
    pub(super) native: Option<Type>,
}

fn native_class(v: &Value) -> NativeClass {
    let name = v["name"].as_str().unwrap_or_default().to_string();
    let ty = |t: &Value| t.as_str().map_or_else(mixed, |s| resolve_str(s, Some(&name)));
    let methods = v["methods"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(lower, m)| {
            let flag = |k: &str| m[k].as_bool().unwrap_or(false);
            let params = m["params"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|p| Param {
                    name: p[0].as_str().unwrap_or_default().to_string(),
                    native: ty(&p[1]),
                    optional: p[2].as_bool().unwrap_or(false),
                    variadic: p[3].as_bool().unwrap_or(false),
                    by_ref: p[4].as_bool().unwrap_or(false),
                })
                .collect();
            let method = NativeMethod {
                name: m["name"].as_str().unwrap_or_default().to_string(),
                visibility: match m["visibility"].as_str() {
                    Some("private") => Visibility::Private,
                    Some("protected") => Visibility::Protected,
                    _ => Visibility::Public,
                },
                is_static: flag("static"),
                is_abstract: flag("abstract"),
                is_final: flag("final"),
                final_doc: flag("finalDoc"),
                builtin: m["builtin"].as_bool().unwrap_or(true),
                variants: m["variants"].as_u64().unwrap_or(1) as usize,
                params,
                ret: ty(&m["return"]),
                tentative: m.get("tentative").map(ty),
                prototype: m["prototype"].as_str().map(str::to_string),
                will_change: flag("willChange"),
            };
            (lower.clone(), method)
        })
        .collect();
    let properties = v["properties"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(n, p)| {
            let flag = |k: &str| p[k].as_bool().unwrap_or(false);
            let hook = |h: &str| p["hooks"].as_array().is_some_and(|a| a.iter().any(|x| x == h));
            let property = NativeProperty {
                is_public: p["visibility"] != "protected",
                is_static: flag("static"),
                readonly: flag("readonly"),
                is_abstract: flag("abstract"),
                is_final: flag("final"),
                final_doc: flag("finalDoc"),
                virtual_: flag("virtual"),
                hooks: (hook("get"), hook("set")),
                readable: p["readable"].as_bool().unwrap_or(true),
                writable: p["writable"].as_bool().unwrap_or(true),
                native: p["type"].as_str().map(|s| resolve_str(s, Some(&name))),
            };
            (n.clone(), property)
        })
        .collect();
    NativeClass {
        properties,
        parent: v["parent"].as_str().map(str::to_string),
        interfaces: v["interfaces"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|i| i.as_str().map(str::to_string))
            .collect(),
        methods,
        constant_types: v["constantTypes"]
            .as_object()
            .into_iter()
            .flatten()
            .filter_map(|(n, t)| Some((n.clone(), resolve_str(t.as_str()?, Some(&name)))))
            .collect(),
        name,
    }
}

/// The built-in classes at the project's PHP version, by lowercase name.
pub(super) fn natives(php_version: u32) -> &'static HashMap<String, NativeClass> {
    static PHP84: OnceLock<HashMap<String, NativeClass>> = OnceLock::new();
    static PHP85: OnceLock<HashMap<String, NativeClass>> = OnceLock::new();
    let php84 = php_version < 80500;
    (if php84 { &PHP84 } else { &PHP85 }).get_or_init(|| {
        let data: Value = serde_json::from_str(include_str!("../../data/native-methods.json"))
            .expect("data/native-methods.json");
        let mut classes = data["classes"].as_object().cloned().unwrap_or_default();
        if php84 {
            for (k, v) in data["php84"].as_object().into_iter().flatten() {
                classes.insert(k.clone(), v.clone());
            }
        }
        classes
            .iter()
            .filter(|(_, v)| !v.is_null())
            .map(|(k, v)| (k.clone(), native_class(v)))
            .collect()
    })
}

/// A class-like as PHPStan's reflection provider knows it.
#[derive(Clone, Copy)]
pub(super) enum Class<'a> {
    User(&'a ClassLikeMetadata),
    Builtin(&'static NativeClass),
}

/// A method found on a class (`getNativeMethod()`), with the class it is implemented in (a trait's user).
#[derive(Clone, Copy)]
enum Found<'a> {
    User {
        declaring: &'a ClassLikeMetadata,
        implementing: Class<'a>,
        f: &'a FunctionLikeMetadata,
        /// The reflected lookup name and adapted visibility are independent of trait storage identity.
        name: mago_word::Word,
        visibility: Visibility,
    },
    Builtin {
        declaring: &'static NativeClass,
        m: &'static NativeMethod,
    },
}

/// What the comparisons read of a method (ExtendedMethodReflection).
struct Facts {
    declaring: String,
    accepts_named_arguments: bool,
    name: String,
    visibility: Visibility,
    is_static: bool,
    is_abstract: bool,
    final_by_keyword: bool,
    is_final: bool,
    builtin: bool,
    variants: usize,
    params: Vec<Param>,
    ret: Type,
    will_change: bool,
}

/// `isOptional()` of each parameter: it and every later one has a default or is variadic.
fn optional_flags(defaults: impl DoubleEndedIterator<Item = bool>) -> Vec<bool> {
    let mut optional = true;
    let mut out: Vec<bool> = defaults
        .rev()
        .map(|d| {
            optional &= d;
            optional
        })
        .collect();
    out.reverse();
    out
}

fn has_attribute(f: &FunctionLikeMetadata, name: &[u8]) -> bool {
    f.attributes
        .iter()
        .any(|a| a.name.as_bytes().eq_ignore_ascii_case(name))
}

impl<'a> Checker<'a, '_> {
    /// `ReflectionProvider::getClass()`.
    pub(super) fn reflected(&self, name: &str) -> Option<Class<'a>> {
        if let Some(k) = self
            .codebase
            .get_class_like(name.as_bytes())
            .filter(|k| !k.flags.is_built_in())
        {
            return Some(Class::User(k));
        }
        natives(self.php_version)
            .get(&name.to_ascii_lowercase())
            .map(Class::Builtin)
    }

    /// `hasNativeMethod()` / `getNativeMethod()`: the class's own methods, its traits', its parents' and its
    /// interfaces'.
    fn native_method(&self, class: Class<'a>, lower: &str) -> Option<Found<'a>> {
        match class {
            Class::User(k) => {
                let id = k.declaring_method_ids.get(&mago_word::word(lower))?;
                let f = self.codebase.get_method_by_id(id)?;
                if f.flags.contains(MetadataFlags::MAGIC_METHOD) {
                    return None;
                }
                match self.reflected(&String::from_utf8_lossy(id.get_class_name().as_bytes()))? {
                    Class::User(declaring) => {
                        let implementing = k
                            .appearing_method_ids
                            .get(&mago_word::word(lower))
                            .and_then(|a| {
                                self.reflected(&String::from_utf8_lossy(a.get_class_name().as_bytes()))
                            })
                            .unwrap_or(Class::User(declaring));
                        Some(Found::User {
                            declaring,
                            implementing,
                            f,
                            name: mago_word::word(lower),
                            visibility: self.codebase.get_method_visibility(k.name.as_bytes(), lower.as_bytes())
                                .unwrap_or(Visibility::Public),
                        })
                    }
                    builtin => self.native_method(builtin, lower),
                }
            }
            Class::Builtin(b) => {
                if let Some(m) = b.methods.get(lower) {
                    return Some(Found::Builtin { declaring: b, m });
                }
                b.parent
                    .iter()
                    .chain(&b.interfaces)
                    .filter_map(|p| self.reflected(p))
                    .find_map(|p| self.native_method(p, lower))
            }
        }
    }

    /// ClassReflection::getImmediateInterfaces(): exclude parent-provided and transitively inherited interfaces,
    /// retaining native reflection's declaration order. Built-in entries already export this exact list.
    fn immediate_interfaces(&self, class: Class<'a>) -> Vec<Class<'a>> {
        let names: Vec<String> = match class {
            Class::User(k) => {
                let is_enum = k.kind == SymbolKind::Enum;
                // Mago adds an enum's interfaces under its own names; BetterReflection appends them
                let mut names: Vec<String> = k
                    .direct_parent_interfaces
                    .iter()
                    .map(|i| String::from_utf8_lossy(i.as_bytes()).into_owned())
                    .filter(|n| !is_enum || !(n.starts_with("__internal_do_not_use__") || n == "unitenum" || n == "backedenum"))
                    .collect();
                names.sort();
                // The populated codex uses a set. Recover only its ordering from the declaration's CST, resolving
                // imported names in that file rather than sorting either the aliases or resolved names.
                if names.len() > 1 && let Some(contents) = self.file_contents(k.span.file_id) {
                    let arena = mago_allocator::LocalArena::new();
                    let file = mago_database::file::File::ephemeral(
                        std::borrow::Cow::Borrowed(b"interfaces.php".as_slice()),
                        std::borrow::Cow::Owned([b"<?php ".as_slice(), &contents[k.span.to_range_usize()], b";"].concat()),
                    );
                    let program = mago_syntax::parser::parse_file(&arena, &file);
                    let scope = super::doc::scopes_of(k.span.file_id, &contents).at(k.span.start.offset);
                    let mut pending = vec![Node::Program(program)];
                    let mut order = Vec::new();
                    while let Some(node) = pending.pop() {
                        let types = match node {
                            Node::Class(k) => k.implements.as_ref().map(|i| &i.types),
                            Node::Interface(k) => k.extends.as_ref().map(|i| &i.types),
                            Node::Enum(k) => k.implements.as_ref().map(|i| &i.types),
                            Node::AnonymousClass(k) => k.implements.as_ref().map(|i| &i.types),
                            _ => {
                                pending.extend(node.children().into_iter().rev());
                                continue;
                            }
                        };
                        for id in types.into_iter().flatten() {
                            let resolved = scope.resolve(mago_names::kind::NameKind::Default, id.value()).0;
                            order.push(String::from_utf8_lossy(&resolved).to_ascii_lowercase());
                        }
                        break;
                    }
                    names.sort_by_key(|n| order.iter().position(|written| written == n).unwrap_or(usize::MAX));
                }
                if is_enum {
                    names.push("UnitEnum".to_string());
                    names.extend(k.enum_type.is_some().then(|| "BackedEnum".to_string()));
                }
                names
            }
            Class::Builtin(b) => b.interfaces.clone(),
        };
        let interfaces: Vec<_> = names.iter().filter_map(|n| self.reflected(n)).collect();
        let mut indirect = self.parent_class(class).map(|p| self.native_interface_names(p)).unwrap_or_default();
        for interface in &interfaces {
            indirect.extend(self.native_interface_names(*interface));
        }
        interfaces.into_iter().filter(|i| !indirect.contains(&class_display(self, *i).to_ascii_lowercase())).collect()
    }

    /// Native reflection's complete interface-name set. User metadata already includes transitive ancestors;
    /// the exported built-in data instead lists immediate interfaces, so expand just those native edges.
    fn native_interface_names(&self, class: Class<'a>) -> HashSet<String> {
        let mut names = HashSet::new();
        let mut pending = vec![class];
        let mut seen = HashSet::new();
        while let Some(class) = pending.pop() {
            match class {
                Class::User(k) => names.extend(k.all_parent_interfaces.iter().map(|i| i.to_string().to_ascii_lowercase())),
                Class::Builtin(b) => {
                    if seen.insert(&b.name) {
                        for name in &b.interfaces {
                            names.insert(name.to_ascii_lowercase());
                            pending.extend(self.reflected(name));
                        }
                        pending.extend(self.parent_class(class));
                    }
                }
            }
        }
        names
    }

    pub(super) fn parent_class(&self, class: Class<'a>) -> Option<Class<'a>> {
        match class {
            Class::User(k) => self.reflected(&String::from_utf8_lossy(k.direct_parent_class?.as_bytes())),
            Class::Builtin(b) => self.reflected(b.parent.as_deref()?),
        }
    }

    /// BetterReflection's `ReflectionMethod::getPrototype()` of method `lower` implemented in `implementing`.
    fn br_prototype(&self, implementing: Class<'a>, lower: &str) -> Option<Found<'a>> {
        self.br_prototype_at(implementing, lower, 0)
    }

    /// [`Self::br_prototype()`] `depth` ancestors up: a circular hierarchy (which PHP refuses) ends there.
    fn br_prototype_at(&self, implementing: Class<'a>, lower: &str, depth: usize) -> Option<Found<'a>> {
        if depth > ANCESTRY {
            return None;
        }
        if let Class::Builtin(b) = implementing {
            let p = self.reflected(b.methods.get(lower)?.prototype.as_deref()?)?;
            return self.native_method(p, lower);
        }
        // a trait has no interfaces
        for i in self.immediate_interfaces(implementing) {
            if let Some(m) = self.native_method(i, lower) {
                return Some(m);
            }
        }
        let parent = self.native_method(self.parent_class(implementing)?, lower)?;
        // findPrototype()
        let prototype = if self.found_abstract(parent) {
            parent
        } else if self.found_visibility(parent).is_private() {
            return None;
        } else {
            self.br_prototype_at(implementing_of(parent), lower, depth + 1).unwrap_or(parent)
        };
        (lower != "__construct" || self.found_abstract(prototype)).then_some(prototype)
    }

    /// `isAbstract()` of a found method.
    fn found_abstract(&self, found: Found<'a>) -> bool {
        match found {
            Found::User { declaring, f, .. } => self.method_abstract(declaring, f),
            Found::Builtin { m, .. } => m.is_abstract,
        }
    }

    fn found_visibility(&self, found: Found<'a>) -> Visibility {
        match found {
            Found::User { visibility, .. } => visibility,
            Found::Builtin { m, .. } => m.visibility,
        }
    }

    /// The ancestor's template types of class `k` in a `static` of the ancestor (the method reflection is the
    /// ancestor's as `k` sees it), and in a trait method `self` and `static` are the class using it.
    pub(super) fn localize(&self, t: &Type, declaring: &ClassLikeMetadata, k: &ClassLikeMetadata) -> Type {
        let t = if declaring.kind == SymbolKind::Trait {
            let name = original_name(declaring);
            let class = original_name(k);
            let map = |t: &Type| -> Type {
                fn go(t: &Type, name: &str, class: &str) -> Type {
                    match t {
                        Type::Object(n, args, exact) if n.eq_ignore_ascii_case(name) => Type::Object(class.to_string(), args.clone(), *exact),
                        Type::Static(n, args) if n.eq_ignore_ascii_case(name) => Type::Static(class.to_string(), args.clone()),
                        _ => t.traverse(&|t| go(t, name, class)),
                    }
                }
                go(t, &name, &class)
            };
            map(t)
        } else {
            t.clone()
        };
        self.with_ancestor_args(&t, k)
    }

    fn with_ancestor_args(&self, t: &Type, k: &ClassLikeMetadata) -> Type {
        match t {
            Type::Static(n, args) if args.is_empty() => match self.ancestor_args(k, n) {
                Some(args) => Type::Static(n.clone(), args),
                None => t.clone(),
            },
            _ => t.traverse(&|t| self.with_ancestor_args(t, k)),
        }
    }

    /// The template types class `k` gives its ancestor `name`.
    fn ancestor_args(&self, k: &ClassLikeMetadata, name: &str) -> Option<Vec<Type>> {
        let s = symbols::lookup(&original_name(k))?;
        let (_, args) = s.generics.iter().find(|(a, _)| a.eq_ignore_ascii_case(name))?;
        Some(args.iter().map(|a| resolve_str(a, None)).collect())
    }

    /// `isVariadic()` of a declared parameter: its `...` (Mago also flags one a `@param` tag makes variadic).
    fn native_variadic(&self, p: &FunctionLikeParameterMetadata) -> bool {
        p.flags.is_variadic()
            && self.file_contents(p.span.file_id).is_none_or(|contents| {
                let head = &contents[p.span.start.offset as usize..p.name_span.start.offset as usize];
                let head = head.iter().rposition(|b| *b == b']').map_or(head, |i| &head[i + 1..]);
                head.windows(3).any(|w| w == b"...")
            })
    }

    fn facts(&self, found: Found<'a>, k: &ClassLikeMetadata) -> Facts {
        match found {
            Found::Builtin { declaring, m } => Facts {
                declaring: declaring.name.clone(),
                accepts_named_arguments: true,
                name: m.name.clone(),
                visibility: m.visibility,
                is_static: m.is_static,
                is_abstract: m.is_abstract,
                final_by_keyword: m.is_final,
                is_final: m.final_doc,
                builtin: m.builtin,
                variants: m.variants,
                params: m
                    .params
                    .iter()
                    .map(|p| Param {
                        name: p.name.clone(),
                        native: self.with_ancestor_args(&p.native, k),
                        optional: p.optional,
                        variadic: p.variadic,
                        by_ref: p.by_ref,
                    })
                    .collect(),
                ret: self.with_ancestor_args(&m.ret, k),
                will_change: m.will_change,
            },
            Found::User { declaring, implementing, f, name, visibility } => {
                let ty = |t: &Type| self.localize(t, declaring, k);
                let meta = f.method_metadata.as_ref();
                let final_by_keyword = meta.is_some_and(|m| m.is_final)
                    && f.name_span
                        .and_then(|name| self.has_modifier(f.span, name, b"final"))
                        .unwrap_or(true);
                let doc_final = declared::doc_at(self, f.span)
                    .is_some_and(|(doc, _)| has_tag(&doc_tags(&doc), &["@final"]));
                let optional = optional_flags(
                    f.parameters
                        .iter()
                        .map(|p| p.flags.has_default() || self.native_variadic(p)),
                );
                let owner = if declaring.kind.is_trait() {
                    match implementing { Class::User(k) => k, Class::Builtin(_) => declaring }
                } else { declaring };
                let reflected_name = self.codebase.get_method_original_name(owner.name.as_bytes(), name.as_bytes())
                    .unwrap_or(f.original_name);
                Facts {
                    declaring: declared::original_name(owner),
                    accepts_named_arguments: declared::reflected_accepts_named(self, owner, &name.to_string()),
                    name: String::from_utf8_lossy(reflected_name.as_bytes()).into_owned(),
                    visibility,
                    is_static: meta.is_some_and(|m| m.is_static),
                    is_abstract: self.method_abstract(declaring, f),
                    final_by_keyword,
                    is_final: final_by_keyword || doc_final,
                    builtin: false,
                    variants: 1,
                    params: f
                        .parameters
                        .iter()
                        .zip(optional)
                        .map(|(p, optional)| {
                            let native = p.type_declaration_metadata.as_ref().map_or_else(mixed, |t| {
                                ty(&declared::from_tunion(self, &t.type_union, declaring))
                            });
                            let null_default = !p.flags.is_promoted_property()
                                && p.default_type.as_ref().is_some_and(|d| d.type_union.is_null());
                            Param {
                                name: String::from_utf8_lossy(&p.name.0.as_bytes()[1..]).into_owned(),
                                native: if null_default { native.add_null() } else { native },
                                optional,
                                variadic: self.native_variadic(p),
                                by_ref: p.flags.is_by_reference(),
                            }
                        })
                        .collect(),
                    ret: f.return_type_declaration_metadata.as_ref().map_or_else(mixed, |t| {
                        ty(&declared::from_tunion(self, &t.type_union, declaring))
                    }),
                    will_change: has_attribute(f, b"ReturnTypeWillChange"),
                }
            }
        }
    }

    /// The class declaring a found method, as `getDeclaringClass()->getDisplayName()` of the reflection reached from
    /// class `k` shows it: with the template types `k`'s ancestry gives it.
    fn declaring_display(&self, k: &ClassLikeMetadata, found: Found<'a>) -> String {
        let name = match found {
            Found::User { declaring, implementing, .. } if declaring.kind.is_trait() => class_display(self, implementing),
            Found::User { declaring, .. } => self.display(declaring),
            Found::Builtin { declaring, .. } => declaring.name.clone(),
        };
        self.ancestor_display(k, &name)
    }

    /// `getAncestorWithClassName($name)->getDisplayName()` of class `k`.
    pub(super) fn ancestor_display(&self, k: &ClassLikeMetadata, name: &str) -> String {
        match self.ancestor_args(k, name) {
            Some(args) => format!(
                "{name}<{}>",
                args.iter()
                    .map(|a| a.describe(Level::TypeOnly))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            None => name.to_string(),
        }
    }

    /// MethodPrototypeFinder::findPrototype(): the signature prototype, whether its visibility binds, and the
    /// inheritance prototype.
    fn find_prototype(&self, k: &'a ClassLikeMetadata, lower: &str) -> Option<(Found<'a>, bool, Found<'a>)> {
        let class = Class::User(k);
        for i in self.immediate_interfaces(class) {
            if let Some(m) = self.native_method(i, lower) {
                return Some((m, true, m));
            }
        }
        for t in self.traits(k) {
            if let Some(m) = self.native_method(Class::User(t), lower)
                && self.found_abstract(m)
            {
                return Some((m, false, m));
            }
        }
        let method = self.native_method(self.parent_class(class)?, lower)?;
        if self.found_visibility(method).is_private() {
            return None;
        }
        let mut prototype = method;
        if lower == "__construct" {
            // the parent's own constructor binds only when its prototype is abstract
            let real = self.br_prototype(implementing_of(method), lower).unwrap_or(method);
            if !self.found_abstract(real) {
                return None;
            }
            if let Some(m) = self
                .all_interfaces(self.parent_class(class)?)
                .into_iter()
                .find_map(|i| self.native_method(i, lower))
            {
                prototype = m;
            }
        }
        Some((prototype, true, method))
    }

    /// `getTraits(true)`: breadth-first `collectTraits()`, then the parents' traits, keyed by canonical name.
    /// This reflection guard does not affect the statement handler's independent trait-use contexts.
    fn traits(&self, k: &'a ClassLikeMetadata) -> Vec<&'a ClassLikeMetadata> {
        let mut out = Vec::new();
        let mut current = Some(k);
        let mut classes = HashSet::new();
        let mut traits = HashSet::new();
        while let Some(k) = current
            && classes.insert(k.name)
        {
            let direct = |k: &ClassLikeMetadata| {
                let mut names: Vec<_> = k.direct_trait_names().collect();
                names.sort();
                names
                    .into_iter()
                    .filter_map(|t| self.codebase.get_class_like(t.as_bytes()))
                    .collect::<VecDeque<_>>()
            };
            let mut pending = direct(k);
            while let Some(t) = pending.pop_front() {
                if !traits.insert(t.name) {
                    continue;
                }
                out.push(t);
                pending.extend(direct(t));
            }
            current = k
                .direct_parent_class
                .and_then(|p| self.codebase.get_class_like(p.as_bytes()));
        }
        out
    }

    /// ClassReflection::getInterfaces(): own immediate interfaces, parents' immediate interfaces and their
    /// ancestors, then own interfaces' ancestors. Replacing an existing associative-array key keeps its position.
    pub(super) fn all_interfaces(&self, class: Class<'a>) -> Vec<Class<'a>> {
        let immediate = self.immediate_interfaces(class);
        let mut out = immediate.clone();
        let mut seen: HashSet<_> = out.iter().map(|i| class_display(self, *i)).collect();
        for parent in std::iter::successors(self.parent_class(class), |p| self.parent_class(*p)).take(ANCESTRY) {
            for interface in self.immediate_interfaces(parent) {
                if seen.insert(class_display(self, interface)) {
                    out.push(interface);
                }
                self.collect_interfaces(interface, &mut out, &mut seen);
            }
        }
        for interface in immediate {
            self.collect_interfaces(interface, &mut out, &mut seen);
        }
        out
    }

    /// ClassReflection::collectInterfaces(), including its stack order and per-call visited set.
    fn collect_interfaces(&self, interface: Class<'a>, out: &mut Vec<Class<'a>>, seen: &mut HashSet<String>) {
        let mut pending = vec![interface];
        let mut collected = HashSet::new();
        let mut steps = 0;
        while let Some(c) = pending.pop()
            && steps <= ANCESTRY * ANCESTRY
        {
            steps += 1;
            for i in self.immediate_interfaces(c) {
                let name = class_display(self, i);
                if collected.insert(name.clone()) {
                    if seen.insert(name) {
                        out.push(i);
                    }
                    pending.push(i);
                }
            }
        }
    }
}

fn implementing_of(found: Found) -> Class {
    match found {
        Found::User { implementing, .. } => implementing,
        Found::Builtin { declaring, .. } => Class::Builtin(declaring),
    }
}

/// An error a comparison found: identifier, message, tip, ignorable.
type Message = (&'static str, String, Option<&'static str>, bool);

/// MethodParameterComparisonHelper::isTypeCompatible() on PHP 8.
fn compatible(a: &Type, b: &Type, mixed_explicitness: bool) -> bool {
    match (a, b) {
        (Type::Mixed { explicit }, Type::Mixed { explicit: other }) if mixed_explicitness => {
            !explicit || *other
        }
        (Type::Mixed { .. }, _) => true,
        _ => is_super_type_of(a, b).yes(),
    }
}

/// MethodVisibilityComparisonHelper::compare().
fn compare_visibility(
    prototype: &Facts,
    prototype_class: &str,
    method: &Facts,
    method_class: &str,
) -> Option<Message> {
    let message = if prototype.visibility.is_public() {
        if method.visibility.is_public() {
            return None;
        }
        format!(
            "{} method {method_class}::{}() overriding public method {prototype_class}::{}() should also be public.",
            if method.visibility.is_private() { "Private" } else { "Protected" },
            method.name,
            prototype.name,
        )
    } else if method.visibility.is_private() {
        format!(
            "Private method {method_class}::{}() overriding protected method {prototype_class}::{}() should be protected or public.",
            method.name, prototype.name,
        )
    } else {
        return None;
    };
    Some(("method.visibility", message, None, false))
}

/// MethodParameterComparisonHelper::compare() on PHP 8.
fn compare_parameters(
    prototype: &Facts,
    prototype_class: &str,
    method: &Facts,
    method_class: &str,
    ignorable: bool,
    collected: &mut Vec<String>,
) -> Vec<Message> {
    let mut out = Vec::new();
    let mut push = |id: &'static str, message: String| out.push((id, message, None, ignorable));
    let (pname, mname) = (&prototype.name, &method.name);
    let params = &method.params;
    let mut after_variadic = false;
    for (i, pp) in prototype.params.iter().enumerate() {
        let n = i + 1;
        let Some(mp) = params.get(i) else {
            push(
                "parameter.missing",
                format!(
                    "Method {method_class}::{mname}() overrides method {prototype_class}::{pname}() but misses parameter #{n} ${}.",
                    pp.name
                ),
            );
            continue;
        };
        if prototype.accepts_named_arguments && pp.name != mp.name {
            collected.push(super::named_argument::rename(&prototype.declaring, pname, &method.declaring, &pp.name, &mp.name));
        }
        let these = format!("Parameter #{n} ${} of method {method_class}::{mname}()", mp.name);
        let that = format!("parameter #{n} ${} of method {prototype_class}::{pname}()", pp.name);
        if !pp.by_ref {
            if mp.by_ref {
                push(
                    "parameter.byRef",
                    format!("{these} is passed by reference but {that} is not passed by reference."),
                );
            }
        } else if !mp.by_ref {
            push(
                "parameter.notByRef",
                format!("{these} is not passed by reference but {that} is passed by reference."),
            );
        }
        if pp.variadic {
            after_variadic = true;
            if !mp.variadic {
                if !mp.optional {
                    if params.len() != n {
                        push("parameter.notOptional", format!("{these} is not optional."));
                        continue;
                    }
                    push(
                        "parameter.notVariadic",
                        format!("{these} is not variadic but {that} is variadic."),
                    );
                    continue;
                } else if params.len() == n {
                    push("parameter.notVariadic", format!("{these} is not variadic."));
                }
            }
        } else if mp.variadic {
            for (j, rest) in prototype.params[i..].iter().enumerate() {
                if is_super_type_of(&mp.native, &rest.native).yes() {
                    continue;
                }
                push(
                    "method.childParameterType",
                    format!(
                        "Parameter #{n} ...${} ({}) of method {method_class}::{mname}() is not contravariant with parameter #{} ${} ({}) of method {prototype_class}::{pname}().",
                        mp.name,
                        mp.native.describe(Level::TypeOnly),
                        i + j + 1,
                        rest.name,
                        rest.native.describe(Level::TypeOnly),
                    ),
                );
            }
            break;
        }
        if pp.optional && !mp.optional {
            push(
                "parameter.notOptional",
                format!("{these} is required but {that} is optional."),
            );
        }
        if !compatible(&mp.native, &pp.native, false) {
            push(
                "method.childParameterType",
                format!(
                    "Parameter #{n} ${} ({}) of method {method_class}::{mname}() is not contravariant with parameter #{n} ${} ({}) of method {prototype_class}::{pname}().",
                    mp.name,
                    mp.native.describe(Level::TypeOnly),
                    pp.name,
                    pp.native.describe(Level::TypeOnly),
                ),
            );
        }
    }
    let last = params.len().saturating_sub(1);
    for (j, mp) in params.iter().enumerate().skip(prototype.params.len()) {
        let these = format!("Parameter #{} ${} of method {method_class}::{mname}()", j + 1, mp.name);
        if j == last && after_variadic && !mp.variadic {
            push("parameter.notVariadic", format!("{these} is not variadic."));
        } else if !mp.optional {
            push("parameter.notOptional", format!("{these} is not optional."));
        }
    }
    out
}

/// The analysed method as PhpMethodFromParserNodeReflection gives it, `name` being its name in the class (a trait
/// method's alias).
fn analysed(c: &Checker, m: &Method, name: &str, k: &ClassLikeMetadata) -> Facts {
    let has = |f: fn(&Modifier) -> bool| m.modifiers.iter().any(f);
    let lower = name.to_ascii_lowercase();
    // a trait method keeps its visibility unless the `use` changes it (`foo as public`)
    let aliased = c
        .in_trait()
        .and_then(|_| k.trait_visibility_map.get(&mago_word::word(&lower)).copied());
    let visibility = if let Some(v) = aliased {
        v
    } else if has(is_private) {
        Visibility::Private
    } else if has(|x| matches!(x, Modifier::Protected(_))) {
        Visibility::Protected
    } else {
        Visibility::Public
    };
    let params = &m.parameter_list.parameters;
    let optional = optional_flags(
        params
            .iter()
            .map(|p| p.default_value.is_some() || p.ellipsis.is_some()),
    );
    let native_ret = m
        .return_type_hint
        .as_ref()
        .map_or_else(mixed, |r| declared::native(c, &r.hint, Some(k)));
    let ret = match lower.as_str() {
        "__construct" | "__destruct" | "__unset" | "__wakeup" | "__clone" | "__set" | "__unserialize" => Type::Void,
        "__tostring" => Type::String,
        "__isset" => Type::Bool,
        "__sleep" => Type::array(Type::Int, Type::String),
        "__set_state" => intersect(vec![Type::ObjectWithoutClass, native_ret]),
        "__debuginfo" => intersect(vec![
            union(vec![Type::array(Type::Mixed { explicit: true }, Type::Mixed { explicit: true }), Type::Null]),
            native_ret,
        ]),
        "__serialize" => Type::array(Type::Mixed { explicit: true }, Type::Mixed { explicit: true }),
        _ => native_ret,
    };
    // transformStaticType(): `static` is the class's object type in a final class
    fn static_to_object(t: &Type) -> Type {
        match t {
            Type::Static(n, args) => Type::Object(n.clone(), args.clone(), false),
            _ => t.traverse(&static_to_object),
        }
    }
    Facts {
        declaring: declared::original_name(k),
        accepts_named_arguments: true,
        name: name.to_string(),
        visibility,
        is_static: has(|x| matches!(x, Modifier::Static(_))),
        is_abstract: has(is_abstract),
        final_by_keyword: false,
        is_final: false,
        builtin: false,
        variants: 1,
        params: params
            .iter()
            .zip(optional)
            .map(|(p, optional)| {
                let native = p.hint.as_ref().map_or_else(mixed, |h| declared::native(c, h, Some(k)));
                Param {
                    name: String::from_utf8_lossy(&p.variable.name[1..]).into_owned(),
                    native: if default_is_null(p) && !p.is_promoted_property() {
                        native.add_null()
                    } else {
                        native
                    },
                    optional,
                    variadic: p.ellipsis.is_some(),
                    by_ref: p.ampersand.is_some(),
                }
            })
            .collect(),
        ret: if k.flags.is_final() { static_to_object(&ret) } else { ret },
        will_change: false,
    }
}

/// A `#[name]` attribute on the method (names resolved).
fn attribute(c: &Checker, m: &Method, name: &str) -> bool {
    m.attribute_lists
        .iter()
        .flat_map(|l| l.attributes.iter())
        .any(|a| c.resolved(&a.name).eq_ignore_ascii_case(name))
}

/// OverridingMethodRule and ConsistentConstructorRule on method `m` (named `name` in the class).
pub fn method<'arena>(c: &mut Checker<'_, 'arena>, m: &Method<'arena>, name: &str) {
    let k = c.class_scope().expect("a method is walked inside its class-like");
    let mut collected = Vec::new();
    let messages = declared::with_classes(c, || {
        let mut out = Vec::new();
        if c.method_rules.overriding_method {
            out = overriding_method(c, m, name, k, &mut collected);
            // OverridingMethodRule::addErrors(): MethodSignatureRule (checkPhpDocMethodSignatures, level 3) when the
            // rule itself found nothing
            if out.is_empty() && c.level >= 3 && c.method_purity_override {
                out = purity_override(c, m, name, k);
            }
        }
        if c.method_rules.consistent_constructor {
            out.extend(consistent_constructor(c, m, name, k, &mut collected));
        }
        out
    });
    for record in collected {
        c.report(m, super::named_argument::ID, record, None);
    }
    for (id, message, tip, ignorable) in messages {
        if ignorable {
            c.report(m, id, message, tip);
        } else if id == "method.visibility" {
            // MethodVisibilityComparisonHelper reports on the method name's line
            c.report_non_ignorable(m.name, id, message, tip);
        } else {
            c.report_non_ignorable(m, id, message, tip);
        }
    }
}

fn overriding_method<'a>(c: &Checker<'a, '_>, m: &Method, name: &str, k: &'a ClassLikeMetadata, collected: &mut Vec<String>) -> Vec<Message> {
    let lower = name.to_ascii_lowercase();
    let method = analysed(c, m, name, k);
    let class = c.display(k);
    let Some((prototype, check_visibility, inheritance)) = c.find_prototype(k, &lower) else {
        if lower == "__construct"
            && let Some(parent) = c.parent_class(Class::User(k))
            && let Some(constructor) = c.native_method(parent, "__construct")
        {
            let parent_display = match parent {
                Class::User(p) => c.ancestor_display(k, &c.display(p)),
                Class::Builtin(p) => c.ancestor_display(k, &p.name),
            };
            let facts = c.facts(constructor, k);
            if facts.final_by_keyword {
                return vec![(
                    "method.parentMethodFinal",
                    format!("Method {class}::{name}() overrides final method {parent_display}::{}().", facts.name),
                    None,
                    false,
                )];
            }
            if facts.is_final {
                return vec![(
                    "method.parentMethodFinalByPhpDoc",
                    format!("Method {class}::{name}() overrides @final method {parent_display}::{}().", facts.name),
                    None,
                    true,
                )];
            }
            if method.is_abstract && !facts.is_abstract {
                return vec![(
                    "method.abstractOverridingNonAbstract",
                    format!(
                        "Cannot make non-abstract method {}::{}() abstract in class {class}.",
                        c.declaring_display(k, constructor),
                        facts.name
                    ),
                    None,
                    false,
                )];
            }
        }
        if attribute(c, m, "Override") {
            return vec![(
                "method.override",
                format!("Method {class}::{name}() has #[\\Override] attribute but does not override any method."),
                None,
                false,
            )];
        }
        return Vec::new();
    };
    let mut out = Vec::new();
    let inheritance_facts = c.facts(inheritance, k);
    let inheritance_class = c.declaring_display(k, inheritance);
    if inheritance_facts.final_by_keyword {
        out.push((
            "method.parentMethodFinal",
            format!(
                "Method {class}::{name}() overrides final method {inheritance_class}::{}().",
                inheritance_facts.name
            ),
            None,
            false,
        ));
    } else if inheritance_facts.is_final {
        out.push((
            "method.parentMethodFinalByPhpDoc",
            format!(
                "Method {class}::{name}() overrides @final method {inheritance_class}::{}().",
                inheritance_facts.name
            ),
            None,
            true,
        ));
    }
    let facts = c.facts(prototype, k);
    let prototype_class = c.declaring_display(k, prototype);
    let pname = &facts.name;
    if facts.is_static {
        if !method.is_static {
            out.push((
                "method.nonStatic",
                format!("Non-static method {class}::{name}() overrides static method {prototype_class}::{pname}()."),
                None,
                false,
            ));
        }
    } else if method.is_static {
        out.push((
            "method.static",
            format!("Static method {class}::{name}() overrides non-static method {prototype_class}::{pname}()."),
            None,
            false,
        ));
    }
    if method.is_abstract && !facts.is_abstract {
        out.push((
            "method.abstractOverridingNonAbstract",
            format!("Cannot make non-abstract method {prototype_class}::{pname}() abstract in class {class}."),
            None,
            false,
        ));
    }
    if check_visibility {
        out.extend(compare_visibility(&facts, &prototype_class, &method, &class));
    }
    if facts.variants != 1 {
        return out;
    }
    let real = c.br_prototype(Class::User(k), &lower);
    let tentative = match real {
        Some(Found::Builtin { m, .. }) => m.tentative.clone(),
        _ => None,
    };
    if let (Some(real), Some(tentative)) = (real, &tentative)
        && !attribute(c, m, "ReturnTypeWillChange")
        && !facts.will_change
        && !compatible(tentative, &method.ret, true)
    {
        out.push((
            "method.tentativeReturnType",
            format!(
                "Return type {} of method {class}::{name}() is not covariant with tentative return type {} of method {}::{}().",
                method.ret.describe(Level::TypeOnly),
                tentative.describe(Level::TypeOnly),
                c.declaring_display(k, real),
                c.facts(real, k).name,
            ),
            Some(TENTATIVE_TIP),
            false,
        ));
    }
    out.extend(compare_parameters(&facts, &prototype_class, &method, &class, false, collected));
    let report_return = real.is_none() || tentative.is_none() || !facts.builtin;
    if report_return && !compatible(&facts.ret, &method.ret, true) {
        out.push((
            "method.childReturnType",
            format!(
                "Return type {} of method {class}::{name}() is not covariant with return type {} of method {prototype_class}::{pname}().",
                method.ret.describe(Level::TypeOnly),
                facts.ret.describe(Level::TypeOnly),
            ),
            None,
            false,
        ));
    }
    out
}

/// MethodSignatureRule's purity checks (`featureToggles.reportMethodPurityOverride`) against
/// ParentMethodHelper::collectParentMethods(): the parent class's method, every interface's, and the abstract
/// methods the class's traits (and theirs, and its parents') declare. ponytail: the rule's return and parameter type
/// checks are not ported, and a built-in parent's purity (the signature map's `hasSideEffects`) is not consulted.
fn purity_override<'a>(c: &Checker<'a, '_>, m: &Method, name: &str, k: &'a ClassLikeMetadata) -> Vec<Message> {
    let lower = name.to_ascii_lowercase();
    // ponytail: `reportStaticMethodSignatures` stays at its default, off
    let is_static = m.modifiers.iter().any(|x| matches!(x, Modifier::Static(_)));
    if lower == "__construct" || is_static || m.modifiers.iter().any(is_private) {
        return Vec::new();
    }
    if declared::method_purity(c, k, lower.as_bytes()).0 != Some(false) {
        return Vec::new();
    }
    let mut parents: Vec<Found> = Vec::new();
    if let Some(parent) = c.parent_class(Class::User(k))
        && let Some(found) = c.native_method(parent, &lower)
        && c.found_visibility(found) != Visibility::Private
    {
        parents.push(found);
    }
    parents.extend(c.all_interfaces(Class::User(k)).into_iter().filter_map(|i| c.native_method(i, &lower)));
    for t in c.traits(k) {
        if let Some(f) = c.codebase.get_method(t.name.as_bytes(), lower.as_bytes())
            && f.method_metadata.as_ref().is_some_and(|mm| mm.is_abstract)
        {
            parents.push(Found::User {
                declaring: t,
                implementing: Class::User(t),
                f,
                name: f.name,
                visibility: f.method_metadata.as_ref().map_or(Visibility::Public, |m| m.visibility),
            });
        }
    }
    let class = c.display(k);
    parents
        .into_iter()
        .filter_map(|found| {
            let Found::User { declaring, .. } = found else {
                return None;
            };
            let (pure, unless_callable, unless_passed) = declared::method_purity(c, declaring, lower.as_bytes());
            let parent = format!("{}::{}()", c.declaring_display(k, found), c.facts(found, k).name);
            let (id, message) = if pure == Some(true) {
                ("method.impure", format!("Impure method {class}::{name}() overrides pure method {parent}."))
            } else if unless_callable {
                (
                    "method.impureOverridePureUnlessCallable",
                    format!("Impure method {class}::{name}() overrides method {parent} marked @pure-unless-callable-is-impure."),
                )
            } else if unless_passed {
                (
                    "method.impureOverridePureUnlessParameterPassed",
                    format!("Impure method {class}::{name}() overrides method {parent} marked @pure-unless-parameter-passed."),
                )
            } else {
                return None;
            };
            Some((id, message, None, true))
        })
        .collect()
}

/// ConsistentConstructorRule.
fn consistent_constructor<'a>(c: &Checker<'a, '_>, m: &Method, name: &str, k: &'a ClassLikeMetadata, collected: &mut Vec<String>) -> Vec<Message> {
    if !name.eq_ignore_ascii_case("__construct") {
        return Vec::new();
    }
    // ConsistentConstructorHelper::findConsistentConstructor()
    let mut current = c.parent_class(Class::User(k));
    let mut depth = 0;
    let constructor = loop {
        let Some(Class::User(p)) = current else {
            return Vec::new();
        };
        depth += 1;
        if depth > ANCESTRY {
            return Vec::new();
        }
        let tags = c.class_tags(p);
        // Mago gives a class with the tag a constructor when it declares none (DummyConstructorReflection's: public,
        // without parameters)
        if has_tag(
            &tags,
            &["@consistent-constructor", "@phpstan-consistent-constructor", "@psalm-consistent-constructor"],
        ) && let Some(f) = c.native_method(Class::User(p), "__construct")
        {
            break (c.facts(f, k), c.declaring_display(k, f));
        }
        current = c.parent_class(Class::User(p));
    };
    let (facts, declaring) = constructor;
    let method = analysed(c, m, name, k);
    let class = c.display(k);
    let mut out = compare_parameters(&facts, &declaring, &method, &class, true, collected);
    out.extend(compare_visibility(&facts, &declaring, &method, &class));
    out
}

/// WrongCaseOfInheritedMethodRule (phpstan-strict-rules, `matchingInheritedMethodNames`): method `name` of the
/// class-like being walked written in another case than the same method of its parent class or of one of its
/// interfaces (`hasNativeMethod()`, private ones included).
pub fn inherited_case(c: &mut Checker, m: &Method, name: &str) {
    if !c.strict.matching_inherited_method_names {
        return;
    }
    let k = c.class_scope().expect("a method is walked inside its class-like");
    let class = Class::User(k);
    let lower = name.to_ascii_lowercase();
    let mut ancestors: Vec<(Class, &str)> = c.parent_class(class).map(|p| (p, "parent")).into_iter().collect();
    let mut seen = Vec::new();
    for i in c.all_interfaces(class) {
        let display = class_display(c, i);
        if !seen.contains(&display) {
            seen.push(display);
            ancestors.push((i, "interface"));
        }
    }
    let messages = declared::with_classes(c, || {
        let mut messages = Vec::new();
        for (ancestor, kind) in ancestors {
            let Some(found) = c.native_method(ancestor, &lower) else {
                continue;
            };
            let found_name = c.facts(found, k).name;
            if found_name != name {
                messages.push(format!(
                    "Method {}::{name}() does not match {kind} method name: {}::{found_name}().",
                    c.display(k),
                    c.ancestor_display(k, &class_display(c, ancestor))
                ));
            }
        }
        messages
    });
    for message in messages {
        c.report(m, "method.nameCase", message, None);
    }
}

fn class_display(c: &Checker, class: Class) -> String {
    match class {
        Class::User(p) => c.display(p),
        Class::Builtin(b) => b.name.clone(),
    }
}
