//! The class reflection pun-phpstan's type model asks about the classes PHP does not define (the project's and the
//! fixtures' `Source/`): Mago's codebase as `pun_phpstan::symbols::Class`, ported from pun-analyse's `Classes::build`
//! (rules/declared.rs). The comparator only asks a class's name, ancestors, kind, finality and constants, so the
//! parts built from PHPDocs there (templates, generic ancestors, methods) stay empty.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use pun_phpstan::symbols::{self, Class};
use pun_phpstan::types::{Type, to_phpdoc};

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// Runs `f` with the type model knowing the classes of `codebase`.
pub(crate) fn with_classes<R>(codebase: &CodebaseMetadata, f: impl FnOnce() -> R) -> R {
    let cache: RefCell<HashMap<String, Option<Arc<Class>>>> = RefCell::default();
    let get = |name: &str| {
        let key = name.to_ascii_lowercase();
        if let Some(found) = cache.borrow().get(&key) {
            return found.clone();
        }
        let built = build(codebase, name).map(Arc::new);
        cache.borrow_mut().insert(key, built.clone());
        built
    };
    let functions = |name: &str| codebase.get_function(name.as_bytes()).is_some();
    symbols::with_classes(&get, || symbols::with_functions(&functions, f))
}

/// `Classes::build`: the class-like `name` of `codebase`.
fn build(codebase: &CodebaseMetadata, name: &str) -> Option<Class> {
    let k = codebase.get_class_like(name.as_bytes())?;
    let canonical = |w: &[u8]| {
        codebase
            .get_class_like(w)
            .map_or_else(|| text(w), |c| text(c.original_name.as_bytes()))
    };
    // BetterReflection's nominal hierarchy: the declared parents and interfaces, followed up
    let mut pending: Vec<&[u8]> = (k.direct_parent_class.iter())
        .chain(k.direct_parent_interfaces.iter())
        .map(mago_word::Word::as_bytes)
        .collect();
    let mut ancestors = Vec::new();
    let mut seen = HashSet::from([k.name.as_bytes().to_ascii_lowercase()]);
    while let Some(name) = pending.pop() {
        if name.starts_with(b"__internal_do_not_use__") || !seen.insert(name.to_ascii_lowercase()) {
            continue;
        }
        ancestors.push(canonical(name));
        if let Some(parent) = codebase.get_class_like(name) {
            pending.extend(
                (parent.direct_parent_class.iter())
                    .chain(parent.direct_parent_interfaces.iter())
                    .map(mago_word::Word::as_bytes),
            );
        }
    }
    // an enum's implicit interfaces
    if k.kind == SymbolKind::Enum {
        ancestors.retain(|a| a != "UnitEnum" && a != "BackedEnum");
        ancestors.push("UnitEnum".to_string());
        ancestors.extend(k.enum_type.is_some().then(|| "BackedEnum".to_string()));
    }
    Some(Class {
        name: text(k.original_name.as_bytes()),
        ancestors,
        templates: Vec::new(),
        cases: k.enum_cases.keys().map(|w| text(w.as_bytes())).collect(),
        case_values: Vec::new(),
        is_interface: k.kind == SymbolKind::Interface,
        is_trait: k.kind == SymbolKind::Trait,
        is_enum: k.kind == SymbolKind::Enum,
        is_final: k.flags.is_final(),
        properties: Vec::new(),
        allows_dynamic_properties: k.flags.allows_dynamic_properties(),
        // a constant whose value is not a literal reads as `mixed`, as in pun-analyse
        constants: (k.constants.values())
            .map(|m| {
                let value = (m.inferred_type.as_ref())
                    .filter(|t| t.is_single())
                    .and_then(|t| literal(t.get_single()));
                (
                    text(m.name.as_bytes()),
                    value.map_or_else(|| "mixed".to_string(), |t| to_phpdoc(&t)),
                )
            })
            .collect(),
        methods: HashMap::new(),
        iterable: None,
        generics: HashMap::new(),
        property_docs: HashMap::new(),
    })
}

/// A literal constant value as PHPStan's constant type.
fn literal(a: &TAtomic) -> Option<Type> {
    match a {
        TAtomic::Scalar(TScalar::Integer(TInteger::Literal(i))) => Some(Type::ConstInt(*i)),
        TAtomic::Scalar(TScalar::Float(f)) => f.get_literal_value().map(Type::ConstFloat),
        TAtomic::Scalar(TScalar::String(s)) => s
            .get_known_literal_value()
            .map(|v| Type::ConstString(v.to_vec())),
        TAtomic::Scalar(TScalar::Bool(b)) => b.value.map(Type::ConstBool),
        TAtomic::Null => Some(Type::Null),
        _ => None,
    }
}
