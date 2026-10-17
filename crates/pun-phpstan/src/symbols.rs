//! PHPStan 2.2.7's reflection of built-in functions, classes and constants (`data/symbols.json`, written by
//! `scripts/export-phpstan-symbols.php`). Types are kept as the strings PHPStan's `toPhpDocNode()` printed, in
//! the syntax [`crate::resolve`] reads back.

use std::collections::HashMap;
use std::sync::OnceLock;

use serde_json::Value;

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
    /// `isFinalByKeyword()`
    pub is_final: bool,
    pub constants: Vec<(String, String)>,
    /// by lowercase name
    pub methods: HashMap<String, Function>,
}

pub struct Symbols {
    pub functions: HashMap<String, Function>,
    pub classes: HashMap<String, Class>,
    pub constants: HashMap<String, String>,
}

fn entries(v: &Value) -> Vec<(&String, &Value)> {
    // PHP writes an empty map as `[]`.
    v.as_object()
        .map(|o| o.iter().collect())
        .unwrap_or_default()
}

fn string(v: &Value) -> String {
    v.as_str().expect("string").to_string()
}

fn variants(v: &Value) -> Vec<Variant> {
    v.as_array()
        .expect("variants")
        .iter()
        .map(|v| Variant {
            params: v["params"]
                .as_array()
                .expect("params")
                .iter()
                .map(|p| Param {
                    name: string(&p["name"]),
                    ty: string(&p["type"]),
                    optional: p["optional"] == true,
                    variadic: p["variadic"] == true,
                    by_ref: match p["byRef"].as_str() {
                        Some("no") => ByRef::No,
                        Some("createsNewVariable") => ByRef::CreatesNewVariable,
                        _ => ByRef::ReadsArgument,
                    },
                    default: p.get("default").map(string),
                })
                .collect(),
            ret: string(&v["return"]),
        })
        .collect()
}

fn read_function(v: &Value) -> Function {
    Function {
        name: string(&v["name"]),
        side_effects: string(&v["sideEffects"]),
        deprecated: v["deprecated"] == true,
        variants: variants(&v["variants"]),
        named: v.get("named").map(variants),
        is_static: v.get("static") == Some(&Value::Bool(true)),
    }
}

fn read_class(v: &Value) -> Class {
    Class {
        name: string(&v["name"]),
        ancestors: v["ancestors"]
            .as_array()
            .expect("ancestors")
            .iter()
            .map(string)
            .collect(),
        templates: entries(&v["templates"])
            .into_iter()
            .map(|(k, t)| Template {
                name: k.clone(),
                bound: string(&t[0]),
                default: t[1].as_str().map(str::to_string),
            })
            .collect(),
        cases: v["cases"]
            .as_array()
            .expect("cases")
            .iter()
            .map(string)
            .collect(),
        is_interface: v["interface"] == true,
        is_final: v["final"] == true,
        constants: entries(&v["constants"])
            .into_iter()
            .map(|(k, t)| (k.clone(), string(t)))
            .collect(),
        methods: entries(&v["methods"])
            .into_iter()
            .map(|(k, m)| (k.clone(), read_function(m)))
            .collect(),
    }
}

fn load(version: PhpVersion) -> Symbols {
    let data: Value =
        serde_json::from_str(include_str!("../data/symbols.json")).expect("symbols.json");
    let mut symbols = Symbols {
        functions: entries(&data["functions"])
            .into_iter()
            .map(|(k, v)| (k.clone(), read_function(v)))
            .collect(),
        classes: entries(&data["classes"])
            .into_iter()
            .map(|(k, v)| (k.clone(), read_class(v)))
            .collect(),
        constants: entries(&data["constants"])
            .into_iter()
            .map(|(k, v)| (k.clone(), string(v)))
            .collect(),
    };
    if version == PhpVersion::Php85 {
        for (name, f) in entries(&data["php85"]["functions"]) {
            match f {
                Value::Null => symbols.functions.remove(name),
                f => symbols.functions.insert(name.clone(), read_function(f)),
            };
        }
        for (name, c) in entries(&data["php85"]["classes"]) {
            match c {
                Value::Null => symbols.classes.remove(name),
                c => symbols.classes.insert(name.clone(), read_class(c)),
            };
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

pub fn function(name: &str) -> Option<&'static Function> {
    symbols(PhpVersion::Php84)
        .functions
        .get(&name.trim_start_matches('\\').to_ascii_lowercase())
}
