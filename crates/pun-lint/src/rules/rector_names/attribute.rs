//! Rector's `PhpAttribute` on a Doctrine annotation or a simple tag: `PhpAttributeGroupFactory` with the
//! `AnnotationToAttributeMapper` and its mappers, `NamedArgsFactory`, `AnnotationToAttributeIntegerValueCaster`,
//! `AttributeArrayNameInliner` and `AttributeNameFactory` (`UseAliasNameMatcher`), the Php80 attribute decorators,
//! `AttributeValueResolver`, and php-parser's printer for the nodes they make.

use super::doc::{Annotation, is_class_name};
use super::phpdoc_parser::{ArrayItem, Key, Val, print_string};

/// `DocTagNodeState::REMOVE_ARRAY`.
const REMOVE_ARRAY: &str = "__REMOVE_ARRAY__";

/// `SilentKeyMap::CLASS_NAMES_TO_SILENT_KEYS`.
const SILENT_KEYS: &[(&str, &str)] = &[("Symfony\\Component\\Routing\\Annotation\\Route", "path")];

/// `String_::KIND_SINGLE_QUOTED` or `KIND_DOUBLE_QUOTED`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Quote {
    Single,
    Double,
}

/// An expression node the mappers make.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expr {
    Str(String, Quote),
    Int(i64),
    Float(f64),
    /// `ConstFetch`: `true`, `false` or `null`.
    Const(&'static str),
    /// `ClassConstFetch`: the class as printed, the constant.
    ClassConst(String, String),
    Array(Vec<Item>),
    /// `New_` of the class as written.
    New(String, Vec<Arg>),
}

/// `ArrayItem`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Item {
    pub key: Option<Expr>,
    pub value: Expr,
}

/// `Arg`, its name for a named argument.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Arg {
    pub name: Option<String>,
    pub value: Expr,
}

/// An `Attribute` in its own `AttributeGroup`: its name as printed, its arguments.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Attribute {
    pub name: String,
    pub args: Vec<Arg>,
}

/// One `AnnotationToAttribute` of the configuration.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct AttributeConfig {
    pub tag: String,
    pub attribute_class: String,
    pub class_reference_fields: Vec<String>,
    pub use_value_as_attribute_argument: bool,
}

/// What a mapper gives.
enum Mapped {
    Expr(Expr),
    /// `DocTagNodeState::REMOVE_ARRAY`.
    Remove,
}

/// `AnnotationToAttributeMapper::map()` on the values of an annotation (a PHP array of `ArrayItemNode`s);
/// `nested` is whether nested annotations become `new` (PHP 8.1).
fn map_array(items: &[ArrayItem], nested: bool) -> Vec<Item> {
    // `ArrayAnnotationToAttributeMapper`: each `ArrayItemNode` maps to an `ArrayItem`
    items.iter().map(|item| map_item(item, nested)).collect()
}

/// `ArrayItemNodeAnnotationToAttributeMapper`.
fn map_item(item: &ArrayItem, nested: bool) -> Item {
    let value = match map_val(&item.value, nested) {
        Mapped::Expr(expr) => expr,
        Mapped::Remove => {
            return Item {
                key: None,
                value: Expr::Str(REMOVE_ARRAY.into(), Quote::Single),
            };
        }
    };
    let key = match &item.key {
        Some(Key::Val(key)) => match map_val(key, nested) {
            Mapped::Expr(expr) => Some(expr),
            Mapped::Remove => Some(Expr::Str(REMOVE_ARRAY.into(), Quote::Single)),
        },
        Some(Key::Int(key)) => Some(Expr::Int(*key)),
        None => {
            // `hasNoParenthesesAnnotation()`: `@Class` without arguments, read again as an annotation
            if let Val::Str(value) = &item.value
                && value.starts_with('@')
                && !value.ends_with(')')
                && is_class_name(value.trim_start_matches('@'))
            {
                let annotation = Val::Annotation(Box::new(super::phpdoc_parser::Nested {
                    identifier: value.clone(),
                    class: String::new(),
                    values: Vec::new(),
                }));
                return map_item(&ArrayItem { key: None, value: annotation }, nested);
            }
            None
        }
    };
    Item { key, value }
}

/// `AnnotationToAttributeMapper::map()` on a value, its mappers in their order.
fn map_val(value: &Val, nested: bool) -> Mapped {
    Mapped::Expr(match value {
        Val::List(items) => Expr::Array(map_array(items, nested)),
        // `ClassConstFetchAnnotationToAttributeMapper`
        Val::Str(value) if value.contains("::") && !value.starts_with('"') => class_const_fetch(value),
        // `ConstExprNodeAnnotationToAttributeMapper`
        Val::Const(value) => match value.as_str() {
            "true" => Expr::Const("true"),
            "false" => Expr::Const("false"),
            _ => Expr::Int(php_int(value)),
        },
        // `CurlyListNodeAnnotationToAttributeMapper`
        Val::Curly(items) => {
            let mut array = Vec::new();
            let mut position = -1;
            for item in items {
                let mut mapped = map_item(item, nested);
                let key = match &item.key {
                    Some(Key::Val(Val::Str(key))) if is_numeric(key) => php_int(key),
                    Some(Key::Int(key)) => *key,
                    _ => {
                        array.push(mapped);
                        continue;
                    }
                };
                position += 1;
                if position != key {
                    mapped.key = Some(Expr::Int(key));
                }
                array.push(mapped);
            }
            Expr::Array(array)
        }
        // `DoctrineAnnotationAnnotationToAttributeMapper`
        Val::Annotation(annotation) if nested => {
            let args = match annotation.values.is_empty() {
                true => Vec::new(),
                false => inline_array_node(map_array(&annotation.values, nested)),
            };
            Expr::New(annotation.identifier.trim_start_matches('@').to_string(), args)
        }
        Val::Annotation(_) => return Mapped::Remove,
        // `StringAnnotationToAttributeMapper`
        Val::Str(value) => {
            let quote = match value.contains('\'') && !value.contains('\n') {
                true => Quote::Double,
                false => Quote::Single,
            };
            match value.starts_with('"') && value.ends_with('"') {
                true => Expr::Str(value.trim_matches('"').to_string(), quote),
                false => Expr::Str(value.clone(), quote),
            }
        }
        // `StringNodeAnnotationToAttributeMapper`, the kind `StringNode` gives itself
        Val::StringNode(value) => Expr::Str(value.clone(), string_node_quote(value)),
        // `BuilderHelpers::normalizeValue(null)`
        Val::Null => Expr::Const("null"),
    })
}

/// The kind `StringNode` gives a value: double-quoted where it has a `'` and no line break.
fn string_node_quote(value: &str) -> Quote {
    match value.contains('\'') && !value.contains('\n') {
        true => Quote::Double,
        false => Quote::Single,
    }
}

/// `ClassConstFetchAnnotationToAttributeMapper::map()`.
fn class_const_fetch(value: &str) -> Expr {
    let string = || Expr::Str(value.to_string(), Quote::Single);
    let [class, constant] = value.split("::").collect::<Vec<_>>()[..] else {
        return string();
    };
    if class.is_empty() || !is_class_name(class.trim_start_matches('\\')) || !is_constant_name(constant) {
        return string();
    }
    Expr::ClassConst(class.to_string(), constant.to_string())
}

/// `RectorAssert::constantName()`.
fn is_constant_name(name: &str) -> bool {
    let b = name.as_bytes();
    !b.is_empty()
        && (b[0].is_ascii_alphabetic() || b[0] == b'_' || b[0] >= 0x80)
        && b.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c >= 0x80)
}

/// `AttributeArrayNameInliner::inlineArrayNode()`: a string key names the argument.
fn inline_array_node(items: Vec<Item>) -> Vec<Arg> {
    items
        .into_iter()
        .map(|item| Arg {
            name: match item.key {
                Some(Expr::Str(key, _)) => Some(key),
                _ => None,
            },
            value: item.value,
        })
        .collect()
}

/// `PhpAttributeGroupFactory::create()` (then `AttributeGroupNamedArgumentManipulator::decorate()`) for the
/// annotation `annotation`; `name` is the attribute's name as `AttributeNameFactory` makes it, `constructor` the
/// attribute class's constructor parameters (name, whether its type has `int`), `nested` whether nested annotations
/// become `new`.
pub(crate) fn create(
    annotation: &Annotation,
    config: &AttributeConfig,
    name: String,
    constructor: &[(String, bool)],
    nested: bool,
) -> Attribute {
    // `getValuesWithSilentKey()`
    let mut values = annotation.values.clone();
    if let Some((_, key)) = SILENT_KEYS.iter().find(|(class, _)| *class == annotation.class)
        && let Some(item) = values.iter_mut().find(|i| i.key.is_none())
    {
        item.key = Some(Key::Val(Val::Str(key.to_string())));
    }
    // `createArgsFromItems()`: the items, the class references, then the named arguments (`NamedArgsFactory`)
    let mut items = map_array(&values, nested);
    for item in &mut items {
        // `mapClassReferences()`
        if let Some(Expr::Str(key, _)) = &item.key
            && config.class_reference_fields.contains(key)
            && let Expr::Str(value, _) = &item.value
            && !value.is_empty()
        {
            item.value = Expr::ClassConst(format!("\\{}", value.trim_start_matches('\\')), "class".into());
        }
    }
    let mut args: Vec<Arg> = items
        .into_iter()
        .map(|item| {
            let mut value = item.value;
            // `normalizeStringDoubleQuote()`
            if let Expr::Str(string, quote) = &mut value
                && string.contains('\'')
                && !string.contains('\n')
            {
                *quote = Quote::Double;
            }
            Arg {
                name: match item.key {
                    Some(Expr::Str(key, _)) => Some(key),
                    _ => None,
                },
                value,
            }
        })
        .collect();
    // `AnnotationToAttributeIntegerValueCaster::castAttributeTypes()`
    for (parameter, has_int) in constructor {
        for arg in &mut args {
            if let Expr::Array(items) = &mut arg.value
                && let Some(Item {
                    key: Some(Expr::Str(key, _)),
                    value,
                }) = items.first_mut()
                && key == parameter
                && *has_int
                && let Expr::Str(string, _) = value
                && is_numeric(string)
            {
                *value = Expr::Int(php_int(string));
            }
        }
    }
    // `AttributeArrayNameInliner::inlineArray()`
    for arg in &mut args {
        if config.attribute_class == "OpenApi\\Attributes\\Property" && arg.name.as_deref() == Some("example") {
            continue;
        }
        if let Expr::Str(string, _) = &arg.value
            && is_numeric(string)
        {
            let float = php_float(string);
            arg.value = match float == php_int(string) as f64 {
                true => Expr::Int(php_int(string)),
                false => Expr::Float(float),
            };
        }
    }
    let mut attribute = Attribute { name, args };
    decorate(&config.attribute_class, &mut attribute);
    attribute
}

/// Whether an attribute's argument is the nested annotation that stays (`hasRemoveArrayState()`).
pub(crate) fn has_remove_array_state(attribute: &Attribute) -> bool {
    attribute.args.iter().any(|arg| {
        matches!(&arg.value, Expr::Array(items) if items.iter().any(|i| matches!(&i.value, Expr::Str(s, _) if s == REMOVE_ARRAY)))
    })
}

/// The converter attribute decorators: `DoctrineConverterAttributeDecorator` and
/// `SensioParamConverterAttributeDecorator`.
fn decorate(attribute_class: &str, attribute: &mut Attribute) {
    match attribute_class {
        "Doctrine\\ORM\\Mapping\\Column" => {
            if let Some(arg) = attribute.args.iter_mut().find(|a| a.name.as_deref() == Some("nullable"))
                && let Expr::Str(value, _) = &arg.value
                && (value == "true" || value == "false")
            {
                arg.value = Expr::Const(if value == "true" { "true" } else { "false" });
            }
        }
        // make first named arg silent
        "Sensio\\Bundle\\FrameworkExtraBundle\\Configuration\\ParamConverter" => {
            if let Some(arg) = attribute.args.first_mut() {
                arg.name = None;
            }
        }
        _ => {}
    }
}

/// `PhpAttributeGroupFactory::createFromSimpleTag()`: the attribute class, fully qualified, with the value as its
/// argument.
pub(crate) fn from_simple_tag(config: &AttributeConfig, value: Option<&str>) -> Attribute {
    Attribute {
        name: format!("\\{}", config.attribute_class),
        args: match value {
            Some(value) if !value.is_empty() => vec![Arg {
                name: None,
                value: Expr::Str(value.to_string(), Quote::Single),
            }],
            _ => Vec::new(),
        },
    }
}

/// `AttributeValueResolver::resolve()` for a tag whose value is `value` (its `__toString()`) and, for an
/// annotation, `original` (`getOriginalContent()`): the attribute's value and the doc comment that stays.
pub(crate) fn resolve_value(value: &str, original: Option<&str>) -> (String, String) {
    let mut value = value.to_string();
    if let Some(original) = original {
        if value.is_empty() {
            value = original.to_string();
        } else {
            let comment = ltrim(original, &value);
            if !comment.is_empty() {
                value = format!("{value}\n{comment}");
            }
        }
    }
    let mut comment = String::new();
    // special case for newline
    if value.contains('\n') {
        let mut keep_joining = true;
        let mut joined = String::new();
        let mut previous_end_slash = false;
        for (key, line) in value.split('\n').map(|l| l.strip_suffix('\r').unwrap_or(l)).enumerate() {
            if keep_joining {
                joined.push_str(line.trim_end_matches('\\'));
            }
            if line.ends_with('\\') {
                previous_end_slash = true;
                continue;
            }
            if !previous_end_slash && key > 0 {
                match comment.is_empty() {
                    true => comment.push_str(line),
                    false => comment.push_str(&format!("\n * {line}")),
                }
            }
            keep_joining = false;
        }
        value = joined;
    }
    (value, comment)
}

/// `AnnotationToAttributeRector::resolveTrailingComment()`: the text after the closing parenthesis of the
/// annotation's content, where it has a letter.
pub(crate) fn trailing_comment(original: &str) -> Option<String> {
    if !original.starts_with('(') {
        return None;
    }
    let bytes = original.as_bytes();
    let (mut depth, mut quote, mut i) = (0, None, 0);
    while i < bytes.len() {
        let c = bytes[i];
        match quote {
            Some(q) => {
                if c == b'\\' {
                    // skip escaped char
                    i += 1;
                } else if c == q {
                    quote = None;
                }
            }
            None if c == b'"' || c == b'\'' => quote = Some(c),
            None if c == b'(' => depth += 1,
            None if c == b')' => {
                depth -= 1;
                if depth == 0 {
                    let comment = original[i + 1..].trim_matches(PHP_WS);
                    // keep only a real doc comment, not leftover annotation syntax
                    return comment.chars().any(char::is_alphabetic).then(|| comment.to_string());
                }
            }
            None => {}
        }
        i += 1;
    }
    None
}

/// PHP's `trim()` characters.
const PHP_WS: [char; 6] = [' ', '\t', '\n', '\r', '\0', '\x0B'];

/// `AttributeNameFactory::create()`: the attribute's name as printed and, where `UseAliasNameMatcher` matches an
/// aliased import, the import's index in `uses` and the name it gets. `uses` are the imports `(name, alias)` of
/// `resolveBareUses()`, as written.
pub(crate) fn attribute_name(
    config: &AttributeConfig,
    identifier: &str,
    uses: &[(String, Option<String>)],
) -> (String, Option<(usize, String)>) {
    // A. attribute and class name are the same, so we re-use the short form
    if config.attribute_class == config.tag {
        let name = identifier.trim_start_matches('@');
        return match name.strip_prefix('\\') {
            Some(fq) => (format!("\\{}", fq.trim_start_matches('\\')), None),
            None => (name.to_string(), None),
        };
    }
    // B. different name
    let short = identifier.trim_matches('@');
    for (i, (name, alias)) in uses.iter().enumerate() {
        let Some(alias) = alias else {
            continue;
        };
        if !short.starts_with(alias.as_str()) {
            continue;
        }
        let fully_qualified = format!("{name}{}", ltrim(short, alias));
        if fully_qualified != config.tag {
            continue;
        }
        let annotation_parts = fully_qualified.split('\\').count();
        let attribute_parts: Vec<&str> = config.attribute_class.split('\\').collect();
        // requirement for matching single part rename
        if annotation_parts != attribute_parts.len() {
            continue;
        }
        let import_parts = name.matches('\\').count() + 1;
        let new_import = attribute_parts[..import_parts.min(attribute_parts.len())].join("\\");
        // `array_slice($attributeParts, -$shortNamePartCount)`, all of them for none
        let short_parts = attribute_parts.len() - import_parts.min(attribute_parts.len());
        let rest = &attribute_parts[attribute_parts.len() - if short_parts == 0 { attribute_parts.len() } else { short_parts }..];
        return (format!("{alias}\\{}", rest.join("\\")), Some((i, new_import)));
    }
    // 3. the class is not aliased and is completely new... return the FQN version
    (format!("\\{}", config.attribute_class), None)
}

/// PHP's `ltrim($string, $characters)`, `..` making a range.
fn ltrim<'a>(string: &'a str, characters: &str) -> &'a str {
    let set = character_mask(characters);
    let at = string.bytes().position(|b| !set[b as usize]).unwrap_or(string.len());
    &string[at..]
}

/// `php_charmask()`.
fn character_mask(characters: &str) -> [bool; 256] {
    let mut set = [false; 256];
    let b = characters.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if i + 3 < b.len() && b[i + 1] == b'.' && b[i + 2] == b'.' && b[i + 3] >= b[i] {
            for c in b[i]..=b[i + 3] {
                set[c as usize] = true;
            }
            i += 4;
        } else {
            set[b[i] as usize] = true;
            i += 1;
        }
    }
    set
}

/// PHP's `is_numeric()` on a string: leading white space, a sign, digits with an optional fraction and exponent,
/// trailing white space.
pub(crate) fn is_numeric(value: &str) -> bool {
    let trimmed = value.trim_start_matches([' ', '\t', '\n', '\r', '\x0B', '\x0C']).trim_end_matches([
        ' ', '\t', '\n', '\r', '\x0B', '\x0C',
    ]);
    let body = trimmed.strip_prefix(['+', '-']).unwrap_or(trimmed);
    let (mantissa, exponent) = match body.find(['e', 'E']) {
        Some(at) => (&body[..at], Some(&body[at + 1..])),
        None => (body, None),
    };
    let (int, fraction) = match mantissa.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (mantissa, None),
    };
    let digits = |s: &str| s.bytes().all(|c| c.is_ascii_digit());
    let mantissa_ok = digits(int) && fraction.is_none_or(digits) && !(int.is_empty() && fraction.is_none_or(str::is_empty));
    let exponent_ok = exponent.is_none_or(|e| {
        let e = e.strip_prefix(['+', '-']).unwrap_or(e);
        !e.is_empty() && digits(e)
    });
    mantissa_ok && exponent_ok
}

/// PHP's `(float)` of a numeric string.
fn php_float(value: &str) -> f64 {
    value.trim().parse().unwrap_or(0.0)
}

/// PHP's `(int)` of a numeric string: its float value cut towards zero (saturating) where it has a fraction or an
/// exponent, else its digits.
fn php_int(value: &str) -> i64 {
    let trimmed = value.trim();
    match trimmed.parse::<i64>() {
        Ok(int) => int,
        Err(_) => php_float(trimmed) as i64,
    }
}

impl std::fmt::Display for Attribute {
    /// `pAttributeGroup()` of the attribute's group.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[{}", self.name)?;
        if !self.args.is_empty() {
            write!(f, "({})", print_args(&self.args))?;
        }
        f.write_str("]")
    }
}

/// `pCommaSeparated()` of arguments (`pArg()`).
fn print_args(args: &[Arg]) -> String {
    args.iter()
        .map(|arg| match &arg.name {
            Some(name) => format!("{name}: {}", print(&arg.value)),
            None => print(&arg.value),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// php-parser's printer (with Rector's short arrays) on `expr`.
fn print(expr: &Expr) -> String {
    match expr {
        Expr::Str(value, Quote::Single) => single_quoted(value),
        // `escapeString()`, as `ConstExprStringNode` does
        Expr::Str(value, Quote::Double) => print_string(value.as_bytes(), "\""),
        Expr::Int(value) => value.to_string(),
        Expr::Float(value) => print_float(*value),
        Expr::Const(name) => name.to_string(),
        Expr::ClassConst(class, constant) => format!("{class}::{constant}"),
        Expr::Array(items) => format!(
            "[{}]",
            items
                .iter()
                .map(|item| match &item.key {
                    Some(key) => format!("{} => {}", print(key), print(&item.value)),
                    None => print(&item.value),
                })
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expr::New(class, args) => format!("new {class}({})", print_args(args)),
    }
}

/// `pSingleQuotedString()`: a `'` escaped, and a `\` that a `'`, a `\` or the end follows or a `\` precedes.
fn single_quoted(value: &str) -> String {
    let b = value.as_bytes();
    let mut out = String::from("'");
    for (i, c) in value.char_indices() {
        let escape = match c {
            '\'' => true,
            '\\' => matches!(b.get(i + 1), None | Some(b'\'' | b'\\')) || (i > 0 && b[i - 1] == b'\\'),
            _ => false,
        };
        if escape {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('\'');
    out
}

/// `pScalar_Float()`: `%.16G`, or `%.17G` where that does not give the value back, with `.0` after an integer.
fn print_float(value: f64) -> String {
    let printed = format_g(value, 16);
    let printed = match printed.parse::<f64>() == Ok(value) {
        true => printed,
        false => format_g(value, 17),
    };
    match printed.trim_start_matches('-').bytes().all(|c| c.is_ascii_digit()) {
        true => format!("{printed}.0"),
        false => printed,
    }
}

/// C's `%.{precision}G`.
fn format_g(value: f64, precision: usize) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    let exponent = format!("{:.*e}", precision - 1, value);
    let (mantissa, exp) = exponent.split_once('e').unwrap_or((&exponent, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let trim = |s: &str| match s.contains('.') {
        true => s.trim_end_matches('0').trim_end_matches('.').to_string(),
        false => s.to_string(),
    };
    if exp < -4 || exp >= precision as i32 {
        let sign = if exp < 0 { '-' } else { '+' };
        return format!("{}E{sign}{:02}", trim(mantissa), exp.abs());
    }
    trim(&format!("{:.*}", (precision as i32 - 1 - exp).max(0) as usize, value))
}
