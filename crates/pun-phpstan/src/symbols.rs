//! PHPStan 2.2.7's reflection of built-in functions, classes and constants (`data/symbols.json`, written by
//! `scripts/export-phpstan-symbols.php`). Types are kept as the strings PHPStan's `toPhpDocNode()` printed, in
//! the syntax [`crate::resolve`] reads back.

use std::cell::Cell;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

use std::fmt;
use std::marker::PhantomData;

use serde::Deserialize;
use serde::de::{Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhpVersion {
    Php84,
    Php85,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByRef {
    No,
    CreatesNewVariable,
    ReadsArgument,
}

#[derive(Debug, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: String,
    pub optional: bool,
    pub variadic: bool,
    pub by_ref: ByRef,
    pub default: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub params: Vec<Param>,
    pub ret: String,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    /// `hasSideEffects()`: `Yes`, `No` or `Maybe`
    pub side_effects: String,
    pub deprecated: bool,
    pub variants: Vec<Variant>,
    /// `getNamedArgumentsVariants()`, when it differs from the positional ones
    pub named: Option<Vec<Variant>>,
    pub is_static: bool,
}

#[derive(Debug, PartialEq)]
pub struct Template {
    pub name: String,
    pub bound: String,
    pub default: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Class {
    pub name: String,
    pub ancestors: Vec<String>,
    pub templates: Vec<Template>,
    pub cases: Vec<String>,
    pub is_interface: bool,
    pub is_trait: bool,
    pub is_enum: bool,
    /// `isFinalByKeyword()`
    pub is_final: bool,
    /// public instance properties, declared or inherited
    pub properties: Vec<String>,
    pub constants: Vec<(String, String)>,
    /// by lowercase name
    pub methods: HashMap<String, Function>,
    /// a `Traversable`'s iterable key and value types, over its own templates
    pub iterable: Option<(String, String)>,
    /// generic ancestors' type arguments over the class's own templates
    pub generics: HashMap<String, Vec<String>>,
}

pub struct Symbols {
    pub functions: HashMap<String, Function>,
    pub classes: HashMap<String, Class>,
    pub constants: HashMap<String, String>,
}

/// A JSON object read in order; PHP writes an empty map as `[]`.
struct Entries<T>(Vec<(String, T)>);

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Entries<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V<T>(PhantomData<T>);
        impl<'de, T: Deserialize<'de>> Visitor<'de> for V<T> {
            type Value = Entries<T>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a map, or [] for an empty one")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut m: A) -> Result<Entries<T>, A::Error> {
                let mut out = Vec::new();
                while let Some(e) = m.next_entry()? {
                    out.push(e);
                }
                Ok(Entries(out))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut s: A) -> Result<Entries<T>, A::Error> {
                // as `Value::as_object()` read it: a list is no map
                while s.next_element::<IgnoredAny>()?.is_some() {}
                Ok(Entries(Vec::new()))
            }
        }
        d.deserialize_any(V(PhantomData))
    }
}

/// symbols.json as scripts/export-phpstan-symbols.php writes it, read straight into these types (a `serde_json::Value`
/// of the 6 MB file costs as much again).
#[derive(Deserialize)]
struct RawParam {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    optional: bool,
    variadic: bool,
    #[serde(rename = "byRef")]
    by_ref: String,
    default: Option<String>,
}

#[derive(Deserialize)]
struct RawVariant {
    params: Vec<RawParam>,
    #[serde(rename = "return")]
    ret: String,
}

#[derive(Deserialize)]
struct RawFunction {
    name: String,
    #[serde(rename = "sideEffects")]
    side_effects: String,
    deprecated: bool,
    variants: Vec<RawVariant>,
    named: Option<Vec<RawVariant>>,
    #[serde(rename = "static", default)]
    is_static: bool,
}

#[derive(Deserialize)]
struct RawClass {
    name: String,
    ancestors: Vec<String>,
    templates: Entries<(String, Option<String>)>,
    cases: Vec<String>,
    interface: bool,
    #[serde(rename = "final")]
    is_final: bool,
    properties: Vec<String>,
    constants: Entries<String>,
    methods: Entries<RawFunction>,
    iterable: Option<(String, String)>,
    generics: Option<Entries<Vec<String>>>,
}

/// `null`: gone in 8.5.
#[derive(Deserialize)]
struct RawPhp85 {
    functions: Entries<Option<RawFunction>>,
    classes: Entries<Option<RawClass>>,
}

#[derive(Deserialize)]
struct RawSymbols {
    functions: Entries<RawFunction>,
    classes: Entries<RawClass>,
    constants: Entries<String>,
    php85: RawPhp85,
}

fn variants(v: Vec<RawVariant>) -> Vec<Variant> {
    v.into_iter()
        .map(|v| Variant {
            params: v
                .params
                .into_iter()
                .map(|p| Param {
                    name: p.name,
                    ty: p.ty,
                    optional: p.optional,
                    variadic: p.variadic,
                    by_ref: match p.by_ref.as_str() {
                        "no" => ByRef::No,
                        "createsNewVariable" => ByRef::CreatesNewVariable,
                        _ => ByRef::ReadsArgument,
                    },
                    default: p.default,
                })
                .collect(),
            ret: v.ret,
        })
        .collect()
}

fn read_function(v: RawFunction) -> Function {
    Function {
        name: v.name,
        side_effects: v.side_effects,
        deprecated: v.deprecated,
        variants: variants(v.variants),
        named: v.named.map(variants),
        is_static: v.is_static,
    }
}

fn read_class(v: RawClass) -> Class {
    Class {
        name: v.name,
        ancestors: v.ancestors,
        templates: v
            .templates
            .0
            .into_iter()
            .map(|(name, (bound, default))| Template {
                name,
                bound,
                default,
            })
            .collect(),
        is_enum: !v.cases.is_empty(),
        cases: v.cases,
        is_interface: v.interface,
        is_trait: false,
        is_final: v.is_final,
        properties: v.properties,
        constants: v.constants.0,
        methods: v
            .methods
            .0
            .into_iter()
            .map(|(k, m)| (k, read_function(m)))
            .collect(),
        iterable: v.iterable,
        generics: v
            .generics
            .map(|g| g.0.into_iter().collect())
            .unwrap_or_default(),
    }
}

fn load(version: PhpVersion) -> Symbols {
    let data: RawSymbols =
        serde_json::from_str(include_str!("../data/symbols.json")).expect("symbols.json");
    let mut symbols = Symbols {
        functions: data
            .functions
            .0
            .into_iter()
            .map(|(k, v)| (k, read_function(v)))
            .collect(),
        classes: data
            .classes
            .0
            .into_iter()
            .map(|(k, v)| (k, read_class(v)))
            .collect(),
        constants: data.constants.0.into_iter().collect(),
    };
    if version == PhpVersion::Php85 {
        for (name, f) in data.php85.functions.0 {
            symbols.functions.remove(&name);
            if let Some(f) = f {
                symbols.functions.insert(name, read_function(f));
            }
        }
        for (name, c) in data.php85.classes.0 {
            symbols.classes.remove(&name);
            if let Some(c) = c {
                symbols.classes.insert(name, read_class(c));
            }
        }
    }
    symbols
}

static PHP84: OnceLock<Symbols> = OnceLock::new();
static PHP85: OnceLock<Symbols> = OnceLock::new();

pub fn symbols(version: PhpVersion) -> &'static Symbols {
    match version {
        PhpVersion::Php84 => PHP84.get_or_init(|| load(version)),
        PhpVersion::Php85 => PHP85.get_or_init(|| load(version)),
    }
}

/// A built-in class by any-case name (a leading `\` is ignored). Class facts the type system needs (canonical
/// name, ancestors) do not differ between 8.4 and 8.5.
pub fn class(name: &str) -> Option<&'static Class> {
    symbols(PhpVersion::Php84)
        .classes
        .get(&name.trim_start_matches('\\').to_ascii_lowercase())
}

/// A class [`lookup()`] found: a built-in one, or one the [`with_classes()`] provider gave.
#[derive(Clone)]
pub enum ClassRef {
    Builtin(&'static Class),
    User(Arc<Class>),
}

impl Deref for ClassRef {
    type Target = Class;
    fn deref(&self) -> &Class {
        match self {
            ClassRef::Builtin(c) => c,
            ClassRef::User(c) => c,
        }
    }
}

type Provider = dyn Fn(&str) -> Option<Arc<Class>>;

thread_local! {
    static PROVIDER: Cell<Option<*const Provider>> = const { Cell::new(None) };
}

/// Runs `f` with `provider` answering [`lookup()`] for the classes PHP does not define (the analysed project's and
/// its dependencies'), as PHPStan's ReflectionProvider knows them, on this thread.
pub fn with_classes<R>(provider: &dyn Fn(&str) -> Option<Arc<Class>>, f: impl FnOnce() -> R) -> R {
    struct Restore(Option<*const Provider>);
    impl Drop for Restore {
        fn drop(&mut self) {
            PROVIDER.with(|p| p.set(self.0));
        }
    }
    let ptr: *const (dyn Fn(&str) -> Option<Arc<Class>> + '_) = provider;
    // SAFETY: only the lifetime is erased. `lookup()` dereferences the pointer while `f` runs, within the borrow of
    // `provider`; `Restore` puts the previous provider back before this function returns, also on unwinding.
    let ptr: *const Provider = unsafe { std::mem::transmute(ptr) };
    let _restore = Restore(PROVIDER.with(|p| p.replace(Some(ptr))));
    f()
}

/// `ReflectionProvider::getClass()`: a built-in class, else one of the [`with_classes()`] provider.
pub fn lookup(name: &str) -> Option<ClassRef> {
    if let Some(c) = class(name) {
        return Some(ClassRef::Builtin(c));
    }
    let provider = PROVIDER.with(Cell::get)?;
    // SAFETY: set by `with_classes()`, whose borrow outlives every call made while it is set.
    let provider = unsafe { &*provider };
    provider(name.trim_start_matches('\\')).map(ClassRef::User)
}

pub fn function(name: &str) -> Option<&'static Function> {
    symbols(PhpVersion::Php84)
        .functions
        .get(&name.trim_start_matches('\\').to_ascii_lowercase())
}
