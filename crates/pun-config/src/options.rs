//! Format rule options under their Pun names (context/plan/format.md Decisions, "Option names"): `[options.*]` of
//! rules/mapping.toml. pun.toml's `[format.rules]` values are translated to the options of the PHP-CS-Fixer fixer
//! behind the rule (`to_fixer`), and a fixer's options back to pun.toml's spelling (`to_pun`) for `pun migrate` and
//! `pun config`.

use crate::MAPPING;
use anyhow::{Result, anyhow, bail};
use serde_json::{Map, Value};
use std::sync::LazyLock;

/// The shape of an option's value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Bool,
    Int,
    String,
    /// One of `values`.
    Enum,
    /// A list of strings, each one of `values` when it is set.
    List,
    /// A list of lists of strings.
    ListOfLists,
    /// A table with string keys (limited to `keys` when set) and string values (limited to `values` when set).
    Map,
    /// An integer, or a table of integers.
    IntOrMap,
}

/// One option of a format rule.
#[derive(Debug)]
pub struct RuleOption {
    /// The Pun name (kebab-case), as pun.toml writes it.
    pub name: String,
    /// The PHP-CS-Fixer option name.
    pub upstream: String,
    pub kind: Kind,
    /// Pun value => upstream value: the closed list of values, or empty when any string goes.
    pub values: Vec<(String, String)>,
    /// Pun key => upstream key, for a map with a closed key list.
    pub keys: Vec<(String, String)>,
    /// The string that stands for upstream's `null`.
    pub null: Option<String>,
    /// Upstream also takes `null`, written in pun.toml by leaving the option out.
    pub nullable: bool,
}

/// The `type` of an option in rules/mapping.toml.
const KINDS: [(&str, Kind); 8] = [
    ("bool", Kind::Bool),
    ("int", Kind::Int),
    ("string", Kind::String),
    ("enum", Kind::Enum),
    ("list", Kind::List),
    ("list-of-lists", Kind::ListOfLists),
    ("map", Kind::Map),
    ("int-or-map", Kind::IntOrMap),
];

/// Every `[options."<rule ID>"]` table of rules/mapping.toml.
fn all() -> &'static Vec<(String, Vec<RuleOption>)> {
    static T: LazyLock<Vec<(String, Vec<RuleOption>)>> = LazyLock::new(|| {
        let start = MAPPING.find("\n[options.").expect("mapping options");
        let end = MAPPING.find("\n[upstream.").unwrap();
        let t = toml::from_str::<toml::Table>(&MAPPING[start..end]).expect("mapping.toml");
        let rules = t["options"].as_table().unwrap();
        let pairs = |e: &toml::Value, k: &str| -> Vec<(String, String)> {
            (e.get(k)
                .and_then(toml::Value::as_table)
                .into_iter()
                .flatten())
            .map(|(p, u)| (p.clone(), u.as_str().unwrap().to_string()))
            .collect()
        };
        rules
            .iter()
            .map(|(id, opts)| {
                let opts = (opts.as_table().unwrap().iter())
                    .map(|(name, e)| {
                        let s = |k: &str| e.get(k).and_then(toml::Value::as_str).map(String::from);
                        RuleOption {
                            name: name.clone(),
                            upstream: s("upstream").unwrap(),
                            kind: KINDS
                                .iter()
                                .find(|(k, _)| s("type").unwrap() == *k)
                                .unwrap()
                                .1,
                            values: pairs(e, "values"),
                            keys: pairs(e, "keys"),
                            null: s("null"),
                            nullable: e.get("nullable").is_some(),
                        }
                    })
                    .collect();
                (id.clone(), opts)
            })
            .collect()
    });
    &T
}

/// The options of a format rule (`format/imports/ordered`), in name order; empty when it has none.
pub fn rule_options(id: &str) -> &'static [RuleOption] {
    all()
        .iter()
        .find(|(r, _)| r == id)
        .map_or(&[], |(_, o)| o.as_slice())
}

/// The one option a rule's value can stand for directly (`"arrays/syntax" = "short"`): a rule with a single option
/// that isn't a bool (`true` would be ambiguous) or a table (a table is the options).
fn shorthand(opts: &[RuleOption]) -> Option<&RuleOption> {
    match opts {
        [o] if !matches!(o.kind, Kind::Bool | Kind::Map | Kind::IntOrMap) => Some(o),
        _ => None,
    }
}

fn quote(names: impl Iterator<Item = impl std::fmt::Display>) -> String {
    names
        .map(|n| format!("`{n}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn strings<'a>(values: impl Iterator<Item = &'a String>) -> String {
    values
        .map(|v| format!("{v:?}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// A `[format.rules]` value (`true`, `false`, a table of options or the shorthand value) as the fixer takes it:
/// `true`/`false`, or its options under upstream names and values. `rule` is the key as written, `id` the rule ID.
pub(crate) fn to_fixer(rule: &str, id: &str, v: toml::Value) -> Result<Value> {
    let opts = rule_options(id);
    let table = match v {
        toml::Value::Boolean(b) => return Ok(Value::Bool(b)),
        toml::Value::Table(t) if opts.is_empty() => {
            bail!(
                "`{rule}` has no options: it is true or false{}",
                if t.is_empty() {
                    ""
                } else {
                    ", without a table"
                }
            );
        }
        toml::Value::Table(t) => t,
        v => match shorthand(opts) {
            Some(o) => toml::Table::from_iter([(o.name.clone(), v)]),
            None => {
                let one = match opts {
                    [] => "true or false".to_string(),
                    _ => "true, false or a table of options".to_string(),
                };
                bail!("`{rule}` must be {one}");
            }
        },
    };
    let mut out = Map::new();
    for (k, v) in table {
        let Some(o) = opts.iter().find(|o| o.name == k) else {
            // An upstream option name, or its kebab-case spelling from before the TOML names.
            let up = opts
                .iter()
                .find(|o| o.upstream == k || o.upstream.replace('_', "-") == k);
            return Err(anyhow!(match up {
                Some(o) => format!("`{rule}` has no option `{k}`: use `{}`", o.name),
                None => format!(
                    "`{rule}` has no option `{k}`; its options are {}",
                    quote(opts.iter().map(|o| &o.name))
                ),
            }));
        };
        let got = v.to_string();
        let value = fixer_value(o, v)
            .ok_or_else(|| anyhow!("`{rule}` option `{k}` = {got}: expected {}", expected(o)))?;
        out.insert(o.upstream.clone(), value);
    }
    Ok(Value::Object(out))
}

/// What an option takes, for an error.
fn expected(o: &RuleOption) -> String {
    let list = strings(o.values.iter().map(|(p, _)| p));
    let base = match o.kind {
        Kind::Bool => "true or false".to_string(),
        Kind::Int => "an integer".to_string(),
        Kind::String => "a string".to_string(),
        Kind::Enum => format!("one of {list}"),
        Kind::List if o.values.is_empty() => "a list of strings".to_string(),
        Kind::List => format!("a list of {list}"),
        Kind::ListOfLists => "a list of lists of strings".to_string(),
        Kind::Map => {
            let keys = match &o.keys[..] {
                [] => "string keys".to_string(),
                k => format!("the keys {}", strings(k.iter().map(|(p, _)| p))),
            };
            let values = match &o.values[..] {
                [] => "string values".to_string(),
                _ => format!("the values {list}"),
            };
            format!("a table with {keys} and {values}")
        }
        Kind::IntOrMap => "an integer or a table of integers".to_string(),
    };
    match &o.null {
        Some(n) => format!("{base}, or {n:?}"),
        None => base,
    }
}

/// `v` under the option's upstream spelling, or `None` when the option doesn't take it.
fn fixer_value(o: &RuleOption, v: toml::Value) -> Option<Value> {
    use toml::Value as T;
    // A table's `null` is one of its values (binary_operator_spaces' `operators`), never the table.
    if let (T::String(s), Some(n)) = (&v, &o.null)
        && s == n
        && o.kind != Kind::Map
    {
        return Some(Value::Null);
    }
    let up = |s: &str| -> Option<Value> {
        match &o.values[..] {
            [] => Some(Value::from(s)),
            vs => vs
                .iter()
                .find(|(p, _)| p == s)
                .map(|(_, u)| Value::from(u.as_str())),
        }
    };
    let str_list =
        |a: &[T]| -> Option<Vec<Value>> { a.iter().map(|e| e.as_str().map(Value::from)).collect() };
    Some(match (o.kind, v) {
        (Kind::Bool, T::Boolean(b)) => Value::Bool(b),
        (Kind::Int | Kind::IntOrMap, T::Integer(i)) => Value::from(i),
        (Kind::String, T::String(s)) => Value::String(s),
        (Kind::Enum, T::String(s)) => up(&s)?,
        (Kind::List, T::Array(a)) => Value::Array(
            a.iter()
                .map(|e| e.as_str().and_then(up))
                .collect::<Option<_>>()?,
        ),
        (Kind::ListOfLists, T::Array(a)) => Value::Array(
            a.iter()
                .map(|e| e.as_array().and_then(|l| str_list(l)).map(Value::Array))
                .collect::<Option<_>>()?,
        ),
        (Kind::Map, T::Table(t)) => Value::Object(
            t.into_iter()
                .map(|(k, v)| {
                    let k = match &o.keys[..] {
                        [] => k,
                        ks => ks.iter().find(|(p, _)| *p == k)?.1.clone(),
                    };
                    let v = match v {
                        T::String(s) if o.null.as_ref() == Some(&s) => Value::Null,
                        T::String(s) => up(&s)?,
                        _ => return None,
                    };
                    Some((k, v))
                })
                .collect::<Option<_>>()?,
        ),
        (Kind::IntOrMap, T::Table(t)) => Value::Object(
            t.into_iter()
                .map(|(k, v)| v.as_integer().map(|i| (k, Value::from(i))))
                .collect::<Option<_>>()?,
        ),
        _ => return None,
    })
}

/// A fixer's options (upstream names and values; `Null` for none) as pun.toml writes them for rule `id`: `true` for
/// none, the bare value for a rule's only option ([`shorthand`]), else a table under Pun names. An option left at
/// upstream's `null` with no Pun spelling is left out, which means the same. `Err` names what PHP-CS-Fixer itself
/// rejects (an option it doesn't define, a value it doesn't accept), in its words.
pub fn to_pun(fixer: &str, id: &str, options: &Value) -> Result<Value> {
    let Value::Object(m) = options else {
        return Ok(Value::Bool(true));
    };
    let opts = rule_options(id);
    if opts.is_empty() {
        bail!("[{fixer}] Is not configurable.");
    }
    let invalid = |msg: String| anyhow!("[{fixer}] Invalid configuration: {msg}");
    let mut out = Map::new();
    for (k, v) in m {
        let Some(o) = opts.iter().find(|o| o.upstream == *k) else {
            let defined = (opts.iter().map(|o| format!("{:?}", o.upstream)))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(invalid(format!(
                "The option {k:?} does not exist. Defined options are: {defined}."
            )));
        };
        if let Some(msg) = element_types(o, v).or_else(|| sort_types(fixer, k, v)) {
            return Err(invalid(msg));
        }
        let Some(p) = pun_value(o, v) else {
            return Err(invalid(rejected(fixer, o, v)));
        };
        if !p.is_null() {
            out.insert(o.name.clone(), p);
        }
    }
    Ok(match shorthand(opts) {
        Some(o) if out.len() == 1 && out.contains_key(&o.name) => out.remove(&o.name).unwrap(),
        _ => Value::Object(out),
    })
}

/// PHP-CS-Fixer's `Utils::naturalLanguageJoin`: `"a", "b" and "c"`.
fn natural<'a>(names: impl Iterator<Item = &'a str>) -> String {
    let mut names: Vec<String> = names.map(|n| format!("\"{n}\"")).collect();
    let last = names.pop().unwrap();
    match names.is_empty() {
        true => last,
        false => format!("{} and {last}", names.join(", ")),
    }
}

/// ordered_imports' own check of `imports_order`: it names every import type and nothing else. The message for a
/// list that misses one or names another, else `None`.
fn sort_types(fixer: &str, option: &str, v: &Value) -> Option<String> {
    const SUPPORTED: [&str; 3] = ["class", "const", "function"];
    let (("ordered_imports", "imports_order"), Value::Array(a)) = ((fixer, option), v) else {
        return None;
    };
    let given: Vec<&str> = a.iter().filter_map(Value::as_str).collect();
    let missing: Vec<&str> = SUPPORTED
        .into_iter()
        .filter(|t| !given.contains(t))
        .collect();
    let unknown: Vec<&str> = given
        .into_iter()
        .filter(|t| !SUPPORTED.contains(t))
        .collect();
    let (what, list) = match (&missing[..], &unknown[..]) {
        ([], []) => return None,
        ([], u) => ("Unknown", u),
        (m, _) => ("Missing", m),
    };
    let types = if list.len() == 1 { "type" } else { "types" };
    Some(format!(
        "{what} sort {types} {}.",
        natural(list.iter().copied())
    ))
}

/// The types upstream's option resolver allows for `o`, each one a separate allowed type.
fn allowed_types(o: &RuleOption) -> &'static [&'static str] {
    match o.kind {
        Kind::Bool => &["bool"],
        Kind::Int => &["int"],
        Kind::String => &["string"],
        Kind::ListOfLists => &["string[][]"],
        Kind::IntOrMap => &["int", "int[]"],
        // `array<string, ?string>` is declared as `array`, `array<string, string>` as `string[]`.
        Kind::Map if o.null.is_some() => &["array"],
        Kind::List | Kind::Map => &["string[]"],
        // A closed list of values, checked by value alone.
        Kind::Enum => &[],
    }
}

/// PHP's `get_debug_type` of a JSON value.
fn debug_type(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(n) if n.is_f64() => "float",
        Value::Number(_) => "int",
        Value::String(_) => "string",
        Value::Array(_) | Value::Object(_) => "array",
    }
}

/// Symfony OptionsResolver's `verifyTypes` for one type: whether `v` is a `ty`, adding the debug types that fail it
/// to `invalid` (only the first at the top level, every one inside a list).
fn verify_type(ty: &str, v: &Value, invalid: &mut Vec<&'static str>, level: usize) -> bool {
    let entries: Vec<&Value> = match v {
        Value::Array(a) => a.iter().collect(),
        Value::Object(m) => m.values().collect(),
        _ => Vec::new(),
    };
    if let (Some(inner), "array") = (ty.strip_suffix("[]"), debug_type(v)) {
        let failed = (entries.into_iter())
            .filter(|e| !verify_type(inner, e, invalid, level + 1))
            .count();
        return failed == 0;
    }
    let t = debug_type(v);
    if t == ty {
        return true;
    }
    if (invalid.is_empty() || level > 0) && !invalid.contains(&t) {
        invalid.push(t);
    }
    false
}

/// The option resolver's message for a list or table `v` with an entry of a type `o` doesn't allow, checked before
/// the values themselves, else `None`.
fn element_types(o: &RuleOption, v: &Value) -> Option<String> {
    let types = allowed_types(o);
    if !v.is_array() && !v.is_object() || !types.iter().any(|t| t.ends_with("[]")) {
        return None;
    }
    let mut invalid = Vec::new();
    if types.iter().any(|t| verify_type(t, v, &mut invalid, 0)) {
        return None;
    }
    Some(format!(
        "The option {:?} with value array is expected to be of type \"{}\", but one of the elements is of type \"{}\".",
        o.upstream,
        types.join("\" or \""),
        invalid.join("|")
    ))
}

/// PHP-CS-Fixer's message for `v`, a value option `o` of `fixer` doesn't take: the option resolver's for a `null`
/// or a value outside a closed list, the fixer's own where it checks a table's or a list's entries itself.
fn rejected(fixer: &str, o: &RuleOption, v: &Value) -> String {
    let k = &o.upstream;
    let ups = |l: &[(String, String)]| natural(l.iter().map(|(_, u)| u.as_str()));
    let bad = |l: &[(String, String)], s: &str| !l.iter().any(|(_, u)| u == s);
    // The table entry upstream's check stops at: its key when outside `keys`, else its string value.
    let entry = v.as_object().and_then(|m| {
        m.iter().find_map(|(e, s)| match s.as_str() {
            _ if !o.keys.is_empty() && bad(&o.keys, e) => Some((e.as_str(), None)),
            Some(s) if bad(&o.values, s) => Some((e.as_str(), Some(s))),
            _ => None,
        })
    });
    // The list entry upstream's check stops at: the first string outside `values`.
    let item = (v.as_array().into_iter().flatten())
        .filter_map(Value::as_str)
        .find(|s| bad(&o.values, s));
    match (fixer, v, entry, item) {
        (_, Value::Null, ..) if o.kind != Kind::Enum => {
            let types = allowed_types(o).join("\" or \"");
            format!(
                "The option {k:?} with value null is expected to be of type \"{types}\", but is of type \"null\"."
            )
        }
        ("class_attributes_separation", _, Some((e, None)), _) => format!(
            "Unexpected element type, expected any of {}, got \"string#{e}\".",
            ups(&o.keys)
        ),
        // Upstream prints the spacing where it means the element type.
        ("class_attributes_separation", _, Some((_, Some(s))), _) => format!(
            "Unexpected spacing for element type \"{s}\", expected any of {}, got \"string#{s}\".",
            ups(&o.values)
        ),
        ("binary_operator_spaces", _, Some((e, None)), _) => format!(
            "Unexpected \"operators\" key, expected any of {}, got \"string#{e}\".",
            ups(&o.keys)
        ),
        ("binary_operator_spaces", _, Some((e, Some(s))), _) => {
            let values: Vec<String> = (o.values.iter().map(|(_, u)| format!("'{u}'")))
                .chain(["null".to_string()])
                .collect();
            format!(
                "Unexpected value for operator \"{e}\", expected any of {}, got \"string#{s}\".",
                natural(values.iter().map(String::as_str))
            )
        }
        ("phpdoc_tag_type", _, Some((_, Some(s))), _) => format!("Unknown tag type \"{s}\"."),
        ("group_import", _, _, Some(t)) => format!(
            "Invalid group type: {t}, allowed types: {}.",
            ups(&o.values)
        ),
        _ => {
            let accepted = match o.kind {
                Kind::Enum => {
                    let list = (o.values.iter().map(|(_, u)| format!("{u:?}")))
                        .chain(o.null.iter().map(|_| "null".into()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!(" Accepted values are: {list}.")
                }
                _ => String::new(),
            };
            format!(
                "The option {k:?} with value {} is invalid.{accepted}",
                php_value(v)
            )
        }
    }
}

/// How PHP-CS-Fixer's errors print a value: strings quoted, arrays as `array`.
fn php_value(v: &Value) -> String {
    match v {
        Value::Array(_) | Value::Object(_) => "array".into(),
        v => v.to_string(),
    }
}

/// An upstream value under Pun's spelling; `Some(Null)` for a `null` written by leaving the option out, `None` for a
/// value the option doesn't take: outside its closed list, or a `null` it has no use for. Other values pass as they
/// are: the fixer checks their types.
fn pun_value(o: &RuleOption, v: &Value) -> Option<Value> {
    let pun = |v: &Value| -> Option<Value> {
        match (v, &o.values[..]) {
            (Value::Null, _) if o.nullable => Some(Value::Null),
            (Value::Null, _) => o.null.as_deref().map(Value::from),
            (Value::String(s), vs) if !vs.is_empty() => vs
                .iter()
                .find(|(_, u)| u == s)
                .map(|(p, _)| Value::from(p.as_str())),
            (v, _) => Some(v.clone()),
        }
    };
    match (o.kind, v) {
        // PHP's `[]` is also the empty map.
        (Kind::Map | Kind::IntOrMap, Value::Array(a)) if a.is_empty() => {
            Some(Value::Object(Map::new()))
        }
        (Kind::Enum, Value::String(_) | Value::Null) => pun(v),
        (Kind::Enum, _) | (Kind::Map, Value::Null) => None,
        (Kind::List, Value::Array(a)) => a.iter().map(pun).collect::<Option<_>>().map(Value::Array),
        (Kind::Map, Value::Object(m)) => m
            .iter()
            .map(|(k, v)| {
                let k = match &o.keys[..] {
                    [] => k.clone(),
                    ks => ks.iter().find(|(_, u)| u == k)?.0.clone(),
                };
                Some((k, pun(v)?))
            })
            .collect::<Option<_>>()
            .map(Value::Object),
        (_, Value::Null) => pun(v),
        _ => Some(v.clone()),
    }
}
