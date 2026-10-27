//! The `refactor` group's configurable codemods, ported from Rector 2.6.1 (rules/Renaming, rules/Removing,
//! rules/Transform), each in its own file, and what they share.
//!
//! `add-override-attribute` takes no options. Every other codemod does nothing until pun.toml gives it options. The options are the Rector rule's configuration as
//! `pun migrate` writes it (crates/pun-config/src/migrate/rector.rs): a data map keeps its keys
//! (`"lint/refactor/rename-function" = { view = "render" }`), and a list goes under `items`, a value object as a table of
//! its properties kebab-cased (`items = [{ function = "f", argument-position = 2 }]`). `configure` checks them as the
//! Rector rule's `configure()` and its value objects do (`Assert`, `RectorAssert`).

mod add_override_attribute;
mod remove_function_argument;
mod replace_array_access;
mod remove_function_call;
mod rename_cast;
mod rename_class_constant;
mod rename_constant;
mod rename_function;
mod replace_function;

pub use add_override_attribute::AddOverrideAttribute;
pub use remove_function_argument::RemoveFunctionArgument;
pub use replace_array_access::{ArrayAccessCall, ReplaceArrayAccess};
pub use remove_function_call::RemoveFunctionCall;
pub use rename_cast::RenameCast;
pub use rename_class_constant::RenameClassConstant;
pub use rename_constant::RenameConstant;
pub use rename_function::RenameFunction;
pub use replace_function::{
    ReplaceFunctionWithConstant, ReplaceFunctionWithNew, ReplaceFunctionWithStatic,
};

use std::sync::LazyLock;

use mago_syntax::cst::{Expression, Identifier, Node, PartialArgument};
use pun_problem::{Edit, Fix, Safety, Span};
use regex::bytes::Regex;
use serde_json::{Map, Value};

use super::rector_batch5::{imported, imported_member, scope_statements};
use super::types::facts::facts;
use crate::{Context, Report, Rule};

/// Every rule of the group, in a fixed order, unconfigured.
pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(AddOverrideAttribute),
        Box::new(RemoveFunctionArgument::default()),
        Box::new(ReplaceArrayAccess::default()),
        Box::new(RemoveFunctionCall::default()),
        Box::new(RenameCast::default()),
        Box::new(RenameClassConstant::default()),
        Box::new(RenameConstant::default()),
        Box::new(RenameFunction::default()),
        Box::new(ReplaceFunctionWithConstant::default()),
        Box::new(ReplaceFunctionWithNew::default()),
        Box::new(ReplaceFunctionWithStatic::default()),
    ]
}

/// The options table, `None` when pun.toml gives none.
fn table(options: &Value) -> Result<Option<&Map<String, Value>>, String> {
    match options {
        Value::Null => Ok(None),
        Value::Object(map) => Ok(Some(map)),
        _ => Err("options must be a table".into()),
    }
}

/// A configuration that is a map of strings to strings (`Assert::allString`), in the order pun.toml writes it.
pub(crate) fn string_map(options: &Value) -> Result<Vec<(String, String)>, String> {
    let Some(map) = table(options)? else {
        return Ok(Vec::new());
    };
    map.iter()
        .map(|(k, v)| match v.as_str() {
            Some(s) => Ok((k.clone(), s.to_string())),
            None => Err(format!("`{k}` must be a string")),
        })
        .collect()
}

/// A configuration that is a list: the `items` array.
pub(crate) fn items(options: &Value) -> Result<Vec<&Value>, String> {
    let Some(map) = table(options)? else {
        return Ok(Vec::new());
    };
    if let Some(key) = map.keys().find(|k| *k != "items") {
        return Err(format!("unknown option `{key}`; the list goes under `items`"));
    }
    match map.get("items") {
        Some(Value::Array(items)) => Ok(items.iter().collect()),
        _ => Err("`items` must be an array".into()),
    }
}

/// One value object of an `items` list: a table whose keys are among `allowed`, every one of `required` present.
pub(crate) fn object<'a>(
    item: &'a Value,
    allowed: &[&str],
    required: &[&str],
) -> Result<&'a Map<String, Value>, String> {
    let Value::Object(map) = item else {
        return Err("each of `items` must be a table".into());
    };
    if let Some(key) = map.keys().find(|k| !allowed.contains(&k.as_str())) {
        return Err(format!(
            "unknown key `{key}` in `items`; the keys are {}",
            allowed.join(", ")
        ));
    }
    if let Some(key) = required.iter().find(|k| !map.contains_key(**k)) {
        return Err(format!("each of `items` needs `{key}`"));
    }
    Ok(map)
}

/// A string property of a value object.
pub(crate) fn string(map: &Map<String, Value>, key: &str) -> Result<String, String> {
    map[key]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("`{key}` must be a string"))
}

/// An integer property of a value object.
pub(crate) fn int(map: &Map<String, Value>, key: &str) -> Result<i64, String> {
    map[key]
        .as_i64()
        .ok_or_else(|| format!("`{key}` must be an integer"))
}

/// `RectorAssert`'s name patterns (byte-wise, as PCRE without `/u` reads them).
static CLASS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?-u)^[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*(\\[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*)*$")
        .unwrap()
});
static METHOD_OR_CONSTANT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?-u)^[a-zA-Z_\x80-\xff][a-zA-Z0-9_\x80-\xff]*$").unwrap());
static FUNCTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?-u)^([a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff\\]*[a-zA-Z0-9_\x7f-\xff]\\)?([a-zA-Z_\x80-\xff][a-zA-Z0-9_\x80-\xff]*)$",
    )
    .unwrap()
});

/// `RectorAssert::elementName`: `name` matches the pattern of its kind, or the error Rector throws.
fn assert_name(name: &str, regex: &Regex, kind: &str) -> Result<(), String> {
    match regex.is_match(name.as_bytes()) {
        true => Ok(()),
        false => Err(format!("\"{name}\" is not a valid {kind} name")),
    }
}

/// `RectorAssert::className`.
pub(crate) fn assert_class(name: &str) -> Result<(), String> {
    assert_name(name, &CLASS, "class")
}

/// `RectorAssert::constantName`.
pub(crate) fn assert_constant(name: &str) -> Result<(), String> {
    assert_name(name, &METHOD_OR_CONSTANT, "constant")
}

/// `RectorAssert::methodName`.
pub(crate) fn assert_method(name: &str) -> Result<(), String> {
    assert_name(name, &METHOD_OR_CONSTANT, "method")
}

/// `RectorAssert::functionName`.
pub(crate) fn assert_function(name: &str) -> Result<(), String> {
    assert_name(name, &FUNCTION, "function")
}

/// What a function or constant name stands for.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Function,
    Constant,
}

/// `NodeNameResolver::getName()` of a function or constant `Name` node, after the `NameResolver` PHPStan's parser runs:
/// a fully qualified name without its `\`; a name a `use function` / `use const` imports, or a qualified one, resolved
/// (a qualified one through a class import of its first part, else into the namespace); any other name as written,
/// since the global fallback leaves it unresolved.
pub(crate) fn name_of(ancestors: &[Node<'_, '_>], name: &Identifier<'_>, kind: Kind) -> String {
    let written = String::from_utf8_lossy(name.value()).into_owned();
    match name {
        Identifier::FullyQualified(_) => written[1..].to_string(),
        Identifier::Local(local) => {
            imported_member(ancestors, local.value, kind == Kind::Function).unwrap_or(written)
        }
        Identifier::Qualified(_) => imported(ancestors, &written).unwrap_or_else(|| {
            match scope_statements(ancestors).0 {
                Some(ns) => format!("{ns}\\{written}"),
                None => written,
            }
        }),
    }
}

/// `FuncCallNameResolver`: [`name_of`] the called name, except that an unqualified name the namespace declares a
/// function of (`ReflectionProvider::hasFunction` on its namespaced name) is that function.
pub(crate) fn function_name(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], name: &Identifier<'_>) -> String {
    let resolved = name_of(ancestors, name, Kind::Function);
    if let Identifier::Local(local) = name
        && resolved.as_bytes() == local.value
        && let Some(ns) = scope_statements(ancestors).0
    {
        let namespaced = format!("{ns}\\{resolved}");
        if facts(ctx).codebase.function_exists(namespaced.as_bytes()) {
            return namespaced;
        }
    }
    resolved
}

/// What php-parser's `FuncCall` `node` calls: a call's function, or a first-class callable's (`f(...)`, which php-parser
/// reads as a call with a `VariadicPlaceholder`). Mago's other partial applications are no php-parser node.
pub(crate) fn called<'ast, 'arena>(node: Node<'ast, 'arena>) -> Option<&'ast Expression<'arena>> {
    match node {
        Node::FunctionCall(call) => Some(call.function),
        Node::FunctionPartialApplication(p) => {
            let args = &p.argument_list.arguments;
            (args.len() == 1 && matches!(args.get(0), Some(PartialArgument::VariadicPlaceholder(_))))
                .then_some(p.function)
        }
        _ => None,
    }
}

/// `NodeNameResolver::isStringName`: the same name without regard to ASCII case; an empty name matches nothing.
pub(crate) fn is_name(resolved: &str, desired: &str) -> bool {
    !desired.is_empty() && resolved.eq_ignore_ascii_case(desired)
}

/// php-parser's `Name` for a new name: written as given, or fully qualified (with a leading `\`) for a
/// `FullyQualified`.
pub(crate) fn fully_qualified(name: &str) -> String {
    format!("\\{name}")
}

/// A report whose fix is `edits`, unsafe as every codemod is (it changes which code runs).
pub(crate) fn report(span: Span, message: String, edits: Vec<Edit>) -> Report {
    Report {
        span,
        message,
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits,
        }),
    }
}

/// `isObjectTypeOfObjectType`: the same name, or, when the codebase knows `required`, a class it knows to extend or
/// implement it (a class whose parent it does not know extends nothing it knows of).
pub(crate) fn is_class(ctx: &Context<'_>, resolved: &str, required: &str) -> bool {
    let codebase = &facts(ctx).codebase;
    resolved == required
        || (codebase.get_class_like(required.as_bytes()).is_some()
            && codebase.is_instance_of(resolved.as_bytes(), required.as_bytes()))
}
