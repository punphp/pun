//! nette/schema 1.2.5 as PHPStan builds it from `parametersSchema` statements (`ContainerFactory::processSchema`):
//! `Expect::structure/anyOf/arrayOf/listOf/<type>()` plus `nullable/min/max/pattern/default/...`, then
//! `Processor::process()` with its exact messages.

use crate::value::{Array, Key, Value};
use pun_tokens::php_compat::float_to_string;

#[derive(Clone, Debug)]
enum Kind {
    Type {
        ty: String,
        items: Option<(Box<Schema>, Option<Box<Schema>>)>,
        pattern: Option<String>,
        merge: bool,
    },
    Structure {
        items: Vec<(Key, Schema)>,
        other: Option<Box<Schema>>,
    },
    AnyOf(Vec<Alt>),
}

#[derive(Clone, Debug)]
enum Alt {
    Schema(Schema),
    Value(Value),
}

#[derive(Clone, Debug)]
pub struct Schema {
    kind: Kind,
    range: (Option<f64>, Option<f64>),
    default: Value,
    required: bool,
}

fn new(kind: Kind) -> Schema {
    Schema {
        kind,
        range: (None, None),
        default: Value::Null,
        required: false,
    }
}

fn type_schema(ty: &str) -> Schema {
    let mut s = new(Kind::Type {
        ty: ty.into(),
        items: None,
        pattern: None,
        merge: true,
    });
    if ty.contains("[]") && !ty.starts_with("[]") || ty == "list" || ty == "array" {
        s.default = Value::Array(Array::new());
    }
    s
}

/// A non-string entity (a chain or `null()`) throws ShouldNotHappenException before `Expect::{$entity}()`.
fn entity_name(v: &Value) -> Result<&str, String> {
    v.as_str().ok_or_else(|| "Internal error.".to_string())
}

/// `ContainerFactory::processArgument()`.
fn argument(arg: &Value, required: bool) -> Result<Arg, String> {
    match arg {
        Value::Entity(e) if e.value == Value::str("schema") => {
            let statements: Vec<&Value> = e.attributes.values().collect();
            if statements.iter().any(|s| !matches!(s, Value::Entity(_))) {
                return Err("schema() should contain another statement().".into());
            }
            Ok(Arg::Schema(process_schema(&statements, required)?))
        }
        Value::Entity(_) => Ok(Arg::Schema(process_schema(&[arg], required)?)),
        Value::Array(a) => {
            let mut items = Vec::new();
            for (k, v) in a.iter() {
                let ks = k.as_string();
                let (key, req) = match ks.strip_prefix('?') {
                    Some(rest) => (Key::from_string(rest.to_string()), false),
                    None => (k.clone(), true),
                };
                items.push((key, argument(v, req)?));
            }
            Ok(Arg::Map(items))
        }
        v => Ok(Arg::Value(v.clone())),
    }
}

enum Arg {
    Schema(Schema),
    Map(Vec<(Key, Arg)>),
    Value(Value),
}

impl Arg {
    fn schema(self) -> Result<Schema, String> {
        match self {
            Arg::Schema(s) => Ok(s),
            Arg::Value(Value::Str(t)) => Ok(type_schema(&t)),
            _ => Err("expected a schema".into()),
        }
    }

    fn number(&self) -> Option<f64> {
        match self {
            Arg::Value(Value::Int(i)) => Some(*i as f64),
            Arg::Value(Value::Float(f)) => Some(*f),
            _ => None,
        }
    }

    /// The argument as a PHP value (a nested schema stays an object no value equals: null here).
    fn value(self) -> Value {
        match self {
            Arg::Value(v) => v,
            Arg::Map(items) => {
                let mut a = Array::new();
                for (k, v) in items {
                    a.set(k, v.value());
                }
                Value::Array(a)
            }
            Arg::Schema(_) => Value::Null,
        }
    }
}

/// `ContainerFactory::processSchema()`: `Expect::{first}(...)`, then `->{next}(...)` for each further statement.
fn process_schema(statements: &[&Value], required: bool) -> Result<Schema, String> {
    let mut schema: Option<Schema> = None;
    for st in statements {
        let Value::Entity(e) = st else { unreachable!() };
        let name = entity_name(&e.value)?;
        let mut args = Vec::new();
        for a in e.attributes.values() {
            args.push(argument(a, true)?);
        }
        schema = Some(match schema {
            None => expect(name, args)?,
            Some(s) => call(s, name, args)?,
        });
    }
    let mut schema = schema.ok_or("schema() should have at least one argument.")?;
    if required {
        schema.required = true;
    }
    Ok(schema)
}

fn expect(name: &str, args: Vec<Arg>) -> Result<Schema, String> {
    Ok(match name {
        "anyOf" => new(Kind::AnyOf(
            args.into_iter()
                .map(|a| match a {
                    Arg::Schema(s) => Alt::Schema(s),
                    a => Alt::Value(a.value()),
                })
                .collect(),
        )),
        "structure" => {
            let Some(Arg::Map(items)) = args.into_iter().next() else {
                return Err("structure() expects an array".into());
            };
            let items = items
                .into_iter()
                .map(|(k, a)| match a {
                    Arg::Schema(s) => Ok((k, s)),
                    _ => Err("Structure items must be schemas (PHP's TypeError)".to_string()),
                })
                .collect::<Result<_, String>>()?;
            let mut s = new(Kind::Structure { items, other: None });
            s.required = true;
            s
        }
        "arrayOf" | "listOf" => {
            let mut it = args.into_iter();
            let value = it.next().ok_or("arrayOf() expects a type")?.schema()?;
            let key = it.next().map(Arg::schema).transpose()?;
            let mut s = type_schema(if name == "listOf" { "list" } else { "array" });
            if let Kind::Type { items, .. } = &mut s.kind {
                *items = Some((Box::new(value), key.map(Box::new)));
            }
            s
        }
        "type" => type_schema(
            &args
                .into_iter()
                .next()
                .ok_or(TOO_FEW)?
                .value()
                .to_php_string()
                .unwrap_or_default(),
        ),
        ty => {
            let mut s = type_schema(ty);
            if let Some(d) = args.into_iter().next() {
                s.default = d.value();
            }
            s
        }
    })
}

/// PHP's ArgumentCountError (its text names the phar's path).
const TOO_FEW: &str = "Too few arguments";

fn call(mut s: Schema, name: &str, args: Vec<Arg>) -> Result<Schema, String> {
    let first_number = args.first().and_then(Arg::number);
    match (name, &mut s.kind) {
        ("nullable", Kind::Type { ty, .. }) => *ty = format!("null|{ty}"),
        ("nullable", Kind::AnyOf(set)) => set.push(Alt::Value(Value::Null)),
        ("min", _) => s.range.0 = first_number,
        ("max", _) => s.range.1 = first_number,
        ("pattern", Kind::Type { pattern, .. }) => {
            *pattern = args
                .into_iter()
                .next()
                .map(Arg::value)
                .and_then(|v| v.to_php_string())
        }
        ("mergeDefaults", Kind::Type { merge, .. }) => {
            *merge = args
                .into_iter()
                .next()
                .is_none_or(|a| a.value() == Value::Bool(true))
        }
        ("firstIsDefault", Kind::AnyOf(set)) => {
            s.default = match set.first() {
                Some(Alt::Value(v)) => v.clone(),
                _ => Value::Null,
            }
        }
        ("otherItems", Kind::Structure { other, .. }) => {
            *other = Some(Box::new(
                args.into_iter()
                    .next()
                    .map_or_else(|| Ok(type_schema("mixed")), Arg::schema)?,
            ))
        }
        ("default", _) => s.default = args.into_iter().next().ok_or(TOO_FEW)?.value(),
        ("required", _) => {
            s.required = args
                .into_iter()
                .next()
                .is_none_or(|a| a.value() == Value::Bool(true))
        }
        // Deprecations are warnings (E_USER_DEPRECATED), which PHPStan's Configurator swallows.
        ("deprecated", _) => {}
        (m, kind) => {
            let class = match kind {
                Kind::Type { .. } => "Type",
                Kind::Structure { .. } => "Structure",
                Kind::AnyOf(_) => "AnyOf",
            };
            return Err(format!(
                "Call to undefined method _PHPStan_95d9e8b05\\Nette\\Schema\\Elements\\{class}::{m}()."
            ));
        }
    }
    Ok(s)
}

/// `Expect::structure($parametersSchema)` from the merged `parametersSchema` section.
pub fn parameters_schema(section: &Array) -> Result<Schema, String> {
    let mut items = Vec::new();
    for (k, v) in section.iter() {
        let ks = k.as_string();
        let (key, req) = match ks.strip_prefix('?') {
            Some(rest) => (Key::from_string(rest.to_string()), false),
            None => (k.clone(), true),
        };
        items.push((key, argument(v, req)?.schema()?));
    }
    let mut s = new(Kind::Structure { items, other: None });
    s.required = true;
    Ok(s)
}

struct Message {
    template: String,
    path: Vec<String>,
    is_key: bool,
    value: Value,
    vars: Vec<(&'static str, String)>,
    has_expected: bool,
}

impl Message {
    /// `Message::toString()`: `preg_replace_callback('~( ?)%(\w+)%~', ...)`.
    fn render(&self) -> String {
        let t = &self.template;
        let mut out = String::new();
        let mut i = 0;
        while i < t.len() {
            let space = t[i..].starts_with(" %");
            let start = i + usize::from(space);
            let name_len = t[start..].strip_prefix('%').map(|r| {
                r.bytes()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == b'_')
                    .count()
            });
            if let Some(n) = name_len.filter(|n| *n > 0 && t[start + 1 + n..].starts_with('%')) {
                let name = &t[start + 1..start + 1 + n];
                let val = match name {
                    "label" => (if self.is_key { "key of item" } else { "item" }).to_string(),
                    // (the path always starts with `parameters`)
                    "path" => format!("'{}'", self.path.join("\u{a0}›\u{a0}")),
                    "value" => format_value(&self.value),
                    n => self
                        .vars
                        .iter()
                        .find(|(k, _)| *k == n)
                        .map(|(_, v)| v.clone())
                        .unwrap_or_default(),
                };
                out.push_str(if space { " " } else { "" });
                out.push_str(&val);
                i = start + n + 2;
                continue;
            }
            let ch = t[i..].chars().next().expect("char");
            out.push(ch);
            i += ch.len_utf8();
        }
        out
    }
}

#[derive(Default)]
struct Context {
    path: Vec<String>,
    is_key: bool,
    errors: Vec<Message>,
}

impl Context {
    fn error(
        &mut self,
        template: &str,
        value: &Value,
        vars: Vec<(&'static str, String)>,
    ) -> &mut Message {
        let has_expected = vars.iter().any(|(k, v)| *k == "expected" && !v.is_empty());
        self.errors.push(Message {
            template: template.into(),
            path: self.path.clone(),
            is_key: self.is_key,
            value: value.clone(),
            vars,
            has_expected,
        });
        self.errors.last_mut().expect("pushed")
    }
}

/// `Nette\Schema\Helpers::formatValue()`.
pub(crate) fn format_value(v: &Value) -> String {
    match v {
        Value::Entity(_) | Value::OptionalPath(_) | Value::DateTime(_) => {
            format!("object {}", object_class(v))
        }
        Value::Str(s) => format!("'{}'", truncate(s, 15, "...")),
        v if v.is_scalar() => v.var_export(),
        Value::Null => "null".into(),
        _ => "array".into(),
    }
}

/// (OptionalPath values are unwrapped before validation.)
fn object_class(v: &Value) -> &'static str {
    match v {
        Value::DateTime(_) => "DateTimeImmutable",
        _ => "_PHPStan_95d9e8b05\\Nette\\DI\\Definitions\\Statement",
    }
}

/// `Nette\Utils\Strings::truncate($s, $maxLen, $append)`.
pub(crate) fn truncate(s: &str, max: usize, append: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        return s.to_string();
    }
    let max = max - append.chars().count();
    let boundary = |c: char| {
        c.is_whitespace() || matches!(c, '\x00'..='/' | ':'..='@' | '['..='`' | '{'..='~')
    };
    for n in (1..=max).rev() {
        if chars.get(n).is_some_and(|c| boundary(*c)) {
            return chars[..n].iter().collect::<String>() + append;
        }
    }
    chars[..max].iter().collect::<String>() + append
}

/// `Nette\Utils\Validators::is($value, $expected)` for the types schemas use.
fn is_type(v: &Value, expected: &str) -> bool {
    expected.split('|').any(|item| {
        let item = match item.strip_prefix('?') {
            Some(_) if *v == Value::Null => return true,
            Some(rest) => rest,
            None => item,
        };
        match item {
            "array" => matches!(v, Value::Array(_)),
            "list" => matches!(v, Value::Array(a) if a.is_list()),
            "bool" | "boolean" => matches!(v, Value::Bool(_)),
            "int" | "integer" => matches!(v, Value::Int(_)),
            "float" => matches!(v, Value::Float(_)),
            "number" => matches!(v, Value::Int(_) | Value::Float(_)),
            "string" | "unicode" => matches!(v, Value::Str(_)),
            "null" => *v == Value::Null,
            "scalar" => v.is_scalar(),
            "mixed" => true,
            "object" => matches!(
                v,
                Value::Entity(_) | Value::OptionalPath(_) | Value::DateTime(_)
            ),
            // ponytail: `instanceof` of the exact class only (no parents/interfaces).
            class => {
                matches!(
                    v,
                    Value::Entity(_) | Value::OptionalPath(_) | Value::DateTime(_)
                ) && class.eq_ignore_ascii_case(object_class(v))
            }
        }
    })
}

fn strip_prevent(v: &mut Value) -> bool {
    if let Value::Array(a) = v
        && crate::di::has_prevent(a)
    {
        a.remove(&Key::from(crate::di::PREVENT_MERGING));
        return true;
    }
    false
}

fn range_text(r: (Option<f64>, Option<f64>)) -> String {
    let f = |x: Option<f64>| x.map(float_to_string).unwrap_or_default();
    format!("{}..{}", f(r.0), f(r.1))
}

fn in_range(n: f64, r: (Option<f64>, Option<f64>)) -> bool {
    r.0.is_none_or(|lo| n >= lo) && r.1.is_none_or(|hi| n <= hi)
}

impl Schema {
    fn normalize(&self, mut value: Value, ctx: &mut Context) -> Value {
        match &self.kind {
            Kind::AnyOf(_) => value,
            Kind::Type { items, .. } => {
                let prevent = strip_prevent(&mut value);
                if let (Value::Array(a), Some((item, key))) = (&value, items) {
                    let mut res = Array::new();
                    for (k, v) in a.clone() {
                        ctx.path.push(k.as_string());
                        ctx.is_key = true;
                        let k = match key {
                            Some(ks) => Key::from(ks.normalize(Value::from(k), ctx)),
                            None => k,
                        };
                        ctx.is_key = false;
                        res.set(k, item.normalize(v, ctx));
                        ctx.path.pop();
                    }
                    value = Value::Array(res);
                }
                if prevent && let Value::Array(a) = &mut value {
                    a.set(Key::from(crate::di::PREVENT_MERGING), Value::Bool(true));
                }
                value
            }
            Kind::Structure { items, other } => {
                let prevent = strip_prevent(&mut value);
                if let Value::Array(a) = &mut value {
                    let keys: Vec<Key> = a.keys().cloned().collect();
                    for k in keys {
                        let schema = items
                            .iter()
                            .find(|(ik, _)| *ik == k)
                            .map(|(_, s)| s)
                            .or(other.as_deref());
                        if let Some(schema) = schema {
                            ctx.path.push(k.as_string());
                            let v = a.get_mut(&k).map(std::mem::take).unwrap_or(Value::Null);
                            a.set(k, schema.normalize(v, ctx));
                            ctx.path.pop();
                        }
                    }
                    if prevent {
                        a.set(Key::from(crate::di::PREVENT_MERGING), Value::Bool(true));
                    }
                }
                value
            }
        }
    }

    fn complete(&self, mut value: Value, ctx: &mut Context) -> Value {
        let before = ctx.errors.len();
        match &self.kind {
            Kind::Type {
                ty,
                items,
                pattern,
                merge,
            } => {
                let mut merge = *merge;
                if strip_prevent(&mut value) {
                    merge = false;
                }
                if value == Value::Null && matches!(self.default, Value::Array(_)) {
                    value = Value::Array(Array::new());
                }
                validate_type(&value, ty, ctx);
                if ctx.errors.len() == before {
                    validate_range(&value, self.range, ctx, ty);
                }
                if ctx.errors.len() == before
                    && let (Some(p), Value::Str(s)) = (pattern, &value)
                {
                    let re = pcre2::bytes::RegexBuilder::new()
                        .utf(true)
                        .ucp(true)
                        .build(&format!("^(?:{p})$(?!\\n)"));
                    if !re.is_ok_and(|re| re.is_match(s.as_bytes()).unwrap_or(false)) {
                        ctx.error("The %label% %path% expects to match pattern '%pattern%', %value% given.", &value, vec![("pattern", p.clone())]);
                    }
                }
                if ctx.errors.len() == before
                    && let (Value::Array(a), Some((item, key))) = (&value, items)
                {
                    let mut res = Array::new();
                    for (k, v) in a.clone() {
                        ctx.path.push(k.as_string());
                        ctx.is_key = true;
                        let k = match key {
                            Some(ks) => Key::from(ks.complete(Value::from(k), ctx)),
                            None => k,
                        };
                        ctx.is_key = false;
                        res.set(k, item.complete(v, ctx));
                        ctx.path.pop();
                    }
                    value = Value::Array(res);
                }
                if ctx.errors.len() == before && merge {
                    value = crate::di::merge(value, self.default.clone());
                }
            }
            Kind::Structure { items, other } => {
                if value == Value::Null {
                    value = Value::Array(Array::new());
                }
                validate_type(&value, "array", ctx);
                if ctx.errors.len() == before {
                    validate_range(&value, self.range, ctx, "");
                }
                if ctx.errors.len() == before
                    && let Value::Array(a) = &mut value
                {
                    let mut items = items.clone();
                    let extra: Vec<Key> = a
                        .keys()
                        .filter(|k| !items.iter().any(|(ik, _)| ik == *k))
                        .cloned()
                        .collect();
                    if let Some(other) = other {
                        items.extend(extra.into_iter().map(|k| (k, (**other).clone())));
                    } else {
                        let names: Vec<String> = items.iter().map(|(k, _)| k.as_string()).collect();
                        for k in extra {
                            let hint = suggestion(&names, &k.as_string());
                            let template = if hint.is_some() {
                                "Unexpected item %path%, did you mean '%hint%'?"
                            } else {
                                "Unexpected item %path%."
                            };
                            ctx.error(
                                template,
                                &Value::Null,
                                vec![("hint", hint.unwrap_or_default())],
                            )
                            .path
                            .push(k.as_string());
                        }
                    }
                    for (k, schema) in &items {
                        ctx.path.push(k.as_string());
                        match a.get_mut(k).map(std::mem::take) {
                            Some(v) => a.set(k.clone(), schema.complete(v, ctx)),
                            None => {
                                let d = schema.complete_default(ctx);
                                a.set(k.clone(), d);
                            }
                        }
                        ctx.path.pop();
                    }
                }
            }
            Kind::AnyOf(set) => {
                let mut expecteds: Vec<String> = Vec::new();
                let mut inner = Vec::new();
                let mut found = None;
                for alt in set {
                    match alt {
                        Alt::Schema(s) => {
                            let mut dolly = Context {
                                path: ctx.path.clone(),
                                ..Context::default()
                            };
                            let n = s.normalize(value.clone(), &mut dolly);
                            let res = s.complete(n, &mut dolly);
                            if dolly.errors.is_empty() {
                                found = Some(res);
                                break;
                            }
                            for e in dolly.errors {
                                if e.path != ctx.path || !e.has_expected {
                                    inner.push(e);
                                } else {
                                    expecteds.push(
                                        e.vars
                                            .iter()
                                            .find(|(k, _)| *k == "expected")
                                            .map(|(_, v)| v.clone())
                                            .unwrap_or_default(),
                                    );
                                }
                            }
                        }
                        Alt::Value(v) => {
                            if *v == value {
                                found = Some(value.clone());
                                break;
                            }
                            expecteds.push(format_value(v));
                        }
                    }
                }
                if let Some(v) = found {
                    return v;
                }
                if !inner.is_empty() {
                    ctx.errors.extend(inner);
                } else {
                    let mut unique: Vec<String> = Vec::new();
                    for e in expecteds {
                        if !unique.contains(&e) {
                            unique.push(e);
                        }
                    }
                    ctx.error(
                        "The %label% %path% expects to be %expected%, %value% given.",
                        &value,
                        vec![("expected", unique.join("|"))],
                    );
                }
            }
        }
        if ctx.errors.len() == before {
            value
        } else {
            Value::Null
        }
    }

    fn complete_default(&self, ctx: &mut Context) -> Value {
        if matches!(self.kind, Kind::Structure { .. }) {
            return if self.required {
                self.complete(Value::Array(Array::new()), ctx)
            } else {
                Value::Null
            };
        }
        if self.required {
            ctx.error(
                "The mandatory item %path% is missing.",
                &Value::Null,
                Vec::new(),
            );
            return Value::Null;
        }
        self.default.clone()
    }

    /// `(new Processor)->process($schema, $data)` with the context path PHPStan sets (`['parameters']`); the error
    /// messages in order, empty when valid.
    pub fn validate(&self, data: &Value, path: &[&str]) -> Vec<String> {
        let mut ctx = Context {
            path: path.iter().map(|s| s.to_string()).collect(),
            ..Context::default()
        };
        let n = self.normalize(data.clone(), &mut ctx);
        if ctx.errors.is_empty() {
            self.complete(n, &mut ctx);
        }
        ctx.errors.iter().map(Message::render).collect()
    }
}

fn validate_type(v: &Value, expected: &str, ctx: &mut Context) {
    if !is_type(v, expected) {
        let e = expected.replace('|', " or ").replace(':', " in range ");
        ctx.error(
            "The %label% %path% expects to be %expected%, %value% given.",
            v,
            vec![("expected", e)],
        );
    }
}

fn validate_range(v: &Value, range: (Option<f64>, Option<f64>), ctx: &mut Context, types: &str) {
    let len = match v {
        Value::Array(a) => Some((a.len(), "items")),
        Value::Str(s) if types.split('|').any(|t| t == "unicode") => {
            Some((s.chars().count(), "characters"))
        }
        Value::Str(s) => Some((s.len(), "bytes")),
        _ => None,
    };
    if let Some((n, label)) = len {
        if !in_range(n as f64, range) {
            let template = format!(
                "The length of %label% %path% expects to be in range %expected%, %length% {label} given."
            );
            ctx.error(
                &template,
                v,
                vec![("length", n.to_string()), ("expected", range_text(range))],
            );
        }
    } else if let Some(n) = match v {
        Value::Int(i) => Some(*i as f64),
        Value::Float(f) => Some(*f),
        _ => None,
    } {
        if !in_range(n, range) {
            ctx.error(
                "The %label% %path% expects to be in range %expected%, %value% given.",
                v,
                vec![("expected", range_text(range))],
            );
        }
    }
}

/// PHP `levenshtein($a, $b, $insert, $replace, $delete)`.
pub(crate) fn levenshtein(a: &str, b: &str, ins: usize, rep: usize, del: usize) -> usize {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut p1: Vec<usize> = (0..=b.len()).map(|i| i * ins).collect();
    let mut p2 = vec![0; b.len() + 1];
    for ca in a {
        p2[0] = p1[0] + del;
        for (j, cb) in b.iter().enumerate() {
            let c0 = p1[j] + if ca == cb { 0 } else { rep };
            let c1 = p1[j + 1] + del;
            let c2 = p2[j] + ins;
            p2[j + 1] = c0.min(c1).min(c2);
        }
        std::mem::swap(&mut p1, &mut p2);
    }
    p1[b.len()]
}

/// `Nette\Utils\ObjectHelpers::getSuggestion()`.
pub(crate) fn suggestion(possibilities: &[String], value: &str) -> Option<String> {
    let prefix = |s: &str, with: &str| -> String {
        for p in ["get", "set", "has", "is", "add"] {
            if let Some(rest) = s.strip_prefix(p)
                && rest.starts_with(|c: char| c.is_ascii_uppercase())
            {
                return format!("{with}{rest}");
            }
        }
        s.to_string()
    };
    let norm = prefix(value, "+");
    let mut best = None;
    let mut min = (value.len() as f64 / 4.0 + 1.0) * 10.0 + 0.1;
    // (array_unique() and the `$item === $value` skip change nothing here: `value` is never a possibility)
    for item in possibilities {
        let len = levenshtein(item, value, 10, 11, 10);
        let len = if (len as f64) < min {
            Some(len)
        } else {
            Some(levenshtein(&prefix(item, "*"), &norm, 10, 11, 10)).filter(|l| (*l as f64) < min)
        };
        if let Some(len) = len {
            min = len as f64;
            best = Some(item.clone());
        }
    }
    best
}
