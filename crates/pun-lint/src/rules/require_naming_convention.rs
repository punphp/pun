//! `lint/style/require-naming-convention`, ported from nine Mago 1.57.0 rules (mago-linter src/rule/consistency/):
//! `class-name`, `constant-name`, `enum-name`, `function-name`, `interface-name`, `method-name`, `property-name`,
//! `trait-name` and `variable-name`, with mago-casing 1.57.0's case checks.
//!
//! The options are a table per Mago rule code, holding that rule's options and `enabled`, which defaults as in Mago:
//! the method, property and variable checks are off until enabled. A name is only reported when the suggested name
//! is a valid identifier. Mago's Drupal integration (which skips `hook_` functions in `*.api.php` files) is not
//! ported: Pun has no integrations, and Mago leaves it off by default.
//!
//! mago-casing's `is_*_case(x)` holds exactly when converting `x` gives `x` back (class case also needs an uppercase
//! first letter), so each check here is a conversion and a comparison.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax_core::{part_of_identifier, start_of_identifier};

use super::mago_helpers::{boolean, configure};
use crate::syntax::span;
use crate::{Context, Report, Rule};

/// The options of one Mago rule.
#[derive(Clone, Copy)]
pub struct Kind {
    pub enabled: bool,
    /// `psr` of the class, interface and trait rules.
    pub psr: bool,
    /// `camel` of the function, method, property and variable rules.
    pub camel: bool,
    pub either: bool,
    pub use_snake_case_for_tests: bool,
    pub check_parameters: bool,
}

impl Kind {
    const fn new(enabled: bool, camel: bool, either: bool) -> Kind {
        Kind {
            enabled,
            psr: false,
            camel,
            either,
            use_snake_case_for_tests: false,
            check_parameters: true,
        }
    }
}

pub struct RequireNamingConvention {
    pub class: Kind,
    pub constant: Kind,
    pub r#enum: Kind,
    pub function: Kind,
    pub interface: Kind,
    pub method: Kind,
    pub property: Kind,
    pub r#trait: Kind,
    pub variable: Kind,
}

impl Default for RequireNamingConvention {
    fn default() -> Self {
        RequireNamingConvention {
            class: Kind::new(true, false, false),
            constant: Kind::new(true, false, false),
            r#enum: Kind::new(true, false, false),
            function: Kind::new(true, false, false),
            interface: Kind::new(true, false, false),
            method: Kind::new(false, true, false),
            property: Kind::new(false, true, false),
            r#trait: Kind::new(true, false, false),
            variable: Kind::new(false, false, true),
        }
    }
}

impl Rule for RequireNamingConvention {
    fn id(&self) -> &'static str {
        "lint/style/require-naming-convention"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::Class,
            NodeKind::Constant,
            NodeKind::ClassLikeConstant,
            NodeKind::Enum,
            NodeKind::Function,
            NodeKind::Interface,
            NodeKind::Method,
            NodeKind::PropertyItem,
            NodeKind::FunctionLikeParameter,
            NodeKind::Trait,
            NodeKind::Assignment,
        ])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |code, value| {
            // The options each rule takes besides `enabled`.
            let (kind, takes): (&mut Kind, &[&str]) = match code {
                "class-name" => (&mut self.class, &["psr"]),
                "constant-name" => (&mut self.constant, &[]),
                "enum-name" => (&mut self.r#enum, &[]),
                "function-name" => (&mut self.function, &["camel", "either"]),
                "interface-name" => (&mut self.interface, &["psr"]),
                "method-name" => (
                    &mut self.method,
                    &["camel", "either", "use-snake-case-for-tests"],
                ),
                "property-name" => (&mut self.property, &["camel", "either"]),
                "trait-name" => (&mut self.r#trait, &["psr"]),
                "variable-name" => (&mut self.variable, &["camel", "either", "check-parameters"]),
                _ => return None,
            };
            for (key, value) in value.as_object()? {
                let value = boolean(value)?;
                match key.as_str() {
                    "enabled" => kind.enabled = value,
                    key if !takes.contains(&key) => return None,
                    "psr" => kind.psr = value,
                    "camel" => kind.camel = value,
                    "either" => kind.either = value,
                    "use-snake-case-for-tests" => kind.use_snake_case_for_tests = value,
                    _ => kind.check_parameters = value,
                }
            }
            Some(())
        })
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        match node {
            Node::Class(class) if self.class.enabled => {
                let name = text(class.name.value);
                check_class_case(out, "Class", name, &class.name);
                if class.modifiers.contains_abstract()
                    && self.class.psr
                    && !name.starts_with("Abstract")
                {
                    let suggested = format!("Abstract{}", to_class_case(name));
                    push_if_valid(
                        out,
                        &class.name,
                        &suggested,
                        format!("Abstract class name `{name}` should be prefixed with `Abstract`."),
                        format!(
                            "Consider renaming it to `{suggested}` to adhere to the naming convention."
                        ),
                    );
                }
            }
            Node::Enum(r#enum) if self.r#enum.enabled => {
                check_class_case(out, "Enum", text(r#enum.name.value), &r#enum.name);
            }
            Node::Interface(interface) if self.interface.enabled => {
                self.check_class_like(out, "Interface", &interface.name, self.interface.psr);
            }
            Node::Trait(r#trait) if self.r#trait.enabled => {
                self.check_class_like(out, "Trait", &r#trait.name, self.r#trait.psr);
            }
            Node::Constant(constant) if self.constant.enabled => {
                constant
                    .items
                    .iter()
                    .for_each(|i| check_constant(out, &i.name));
            }
            Node::ClassLikeConstant(constant) if self.constant.enabled => {
                constant
                    .items
                    .iter()
                    .for_each(|i| check_constant(out, &i.name));
            }
            Node::Function(function) if self.function.enabled => {
                check_case(
                    out,
                    self.function,
                    "Function",
                    text(function.name.value),
                    "",
                    &function.name,
                );
            }
            Node::Method(method) if self.method.enabled => self.check_method(out, method),
            Node::PropertyItem(item) if self.property.enabled => {
                let variable = item.variable();
                check_case(
                    out,
                    self.property,
                    "Property",
                    text(variable.name),
                    "$",
                    variable,
                );
            }
            Node::FunctionLikeParameter(parameter) if parameter.is_promoted_property() => {
                if self.property.enabled {
                    let variable = &parameter.variable;
                    check_case(
                        out,
                        self.property,
                        "Property",
                        text(variable.name),
                        "$",
                        variable,
                    );
                }
            }
            Node::FunctionLikeParameter(parameter)
                if self.variable.enabled && self.variable.check_parameters =>
            {
                self.check_variable(out, &parameter.variable);
            }
            Node::Assignment(assignment)
                if self.variable.enabled && assignment.operator.is_assign() =>
            {
                if let Expression::Variable(Variable::Direct(variable)) = assignment.lhs
                    && !is_special_variable(variable.name)
                {
                    self.check_variable(out, variable);
                }
            }
            _ => {}
        }
    }
}

impl RequireNamingConvention {
    /// The interface and trait rules: class case, and with `psr` the kind as a suffix.
    fn check_class_like(
        &self,
        out: &mut Vec<Report>,
        kind: &str,
        identifier: &LocalIdentifier<'_>,
        psr: bool,
    ) {
        let name = text(identifier.value);
        check_class_case(out, kind, name, identifier);
        if psr && !name.ends_with(kind) {
            let suggested = format!("{name}{kind}");
            push_if_valid(
                out,
                identifier,
                &suggested,
                format!("{kind} name `{name}` should be suffixed with `{kind}`."),
                format!(
                    "Consider renaming it to `{suggested}` to adhere to the naming convention."
                ),
            );
        }
    }

    fn check_method(&self, out: &mut Vec<Report>, method: &Method<'_>) {
        let name = text(method.name.value);
        if name.starts_with("__") {
            return;
        }
        if self.method.use_snake_case_for_tests
            && (name.starts_with("test") || name.starts_with("Test"))
        {
            if !is_snake_case(name) {
                let suggested = to_snake_case(name);
                push_if_valid(
                    out,
                    &method.name,
                    &suggested,
                    format!("Test method name `{name}` should be in snake case."),
                    format!(
                        "Consider renaming it to `{suggested}` to adhere to the naming convention."
                    ),
                );
            }
            return;
        }
        check_case(out, self.method, "Method", name, "", &method.name);
    }

    fn check_variable(&self, out: &mut Vec<Report>, variable: &DirectVariable<'_>) {
        let name = text(variable.name);
        let clean = name.trim_start_matches(|c: char| c == '$' || c == '_' || c.is_ascii_digit());
        check_named_case(out, self.variable, "Variable", name, clean, "$", variable);
    }
}

fn text(bytes: &[u8]) -> &str {
    // Pun lints `&str` source, and an identifier is a run of whole tokens, so its bytes are UTF-8.
    std::str::from_utf8(bytes).unwrap_or_default()
}

/// `is_valid_identifier` (mago-linter rule/consistency/mod.rs).
fn is_valid_identifier(identifier: &str) -> bool {
    let Some((first, rest)) = identifier.as_bytes().split_first() else {
        return false;
    };
    matches!(*first, start_of_identifier!())
        && rest.iter().all(|b| matches!(*b, part_of_identifier!()))
}

fn push_if_valid(
    out: &mut Vec<Report>,
    at: &impl HasSpan,
    suggested: &str,
    message: String,
    help: String,
) {
    if is_valid_identifier(suggested) {
        out.push(Report {
            span: span(at.span()),
            message,
            help: Some(help),
            fix: None,
        });
    }
}

fn check_class_case(out: &mut Vec<Report>, kind: &str, name: &str, at: &impl HasSpan) {
    if !is_class_case(name) {
        let suggested = to_class_case(name);
        push_if_valid(
            out,
            at,
            &suggested,
            format!("{kind} name `{name}` should be in class case."),
            format!("Consider renaming it to `{suggested}` to adhere to the naming convention."),
        );
    }
}

fn check_constant(out: &mut Vec<Report>, identifier: &LocalIdentifier<'_>) {
    let name = text(identifier.value);
    if !is_constant_case(name) {
        let suggested = to_constant_case(name);
        push_if_valid(
            out,
            identifier,
            &suggested,
            format!("Constant name `{name}` should be in constant case."),
            format!("Consider renaming it to `{suggested}` to adhere to the naming convention."),
        );
    }
}

/// The function, method and property rules: the name checked as written, less a leading `$`.
fn check_case(
    out: &mut Vec<Report>,
    cfg: Kind,
    kind: &str,
    name: &str,
    sigil: &str,
    at: &impl HasSpan,
) {
    check_named_case(
        out,
        cfg,
        kind,
        name,
        name.strip_prefix('$').unwrap_or(name),
        sigil,
        at,
    );
}

/// `either`, then `camel`, then snake case, on `clean`; the messages name `name`.
fn check_named_case(
    out: &mut Vec<Report>,
    cfg: Kind,
    kind: &str,
    name: &str,
    clean: &str,
    sigil: &str,
    at: &impl HasSpan,
) {
    let advice = "to adhere to the naming convention.";
    if cfg.either {
        if is_camel_case(clean) || is_snake_case(clean) {
            return;
        }
        let (camel, snake) = (to_camel_case(clean), to_snake_case(clean));
        if is_valid_identifier(&camel) {
            push_if_valid(
                out,
                at,
                &snake,
                format!("{kind} name `{name}` should be in either camel case or snake case."),
                format!("Consider renaming it to `{sigil}{camel}` or `{sigil}{snake}` {advice}"),
            );
        }
    } else if cfg.camel {
        if !is_camel_case(clean) {
            let suggested = to_camel_case(clean);
            push_if_valid(
                out,
                at,
                &suggested,
                format!("{kind} name `{name}` should be in camel case."),
                format!("Consider renaming it to `{sigil}{suggested}` {advice}"),
            );
        }
    } else if !is_snake_case(clean) {
        let suggested = to_snake_case(clean);
        push_if_valid(
            out,
            at,
            &suggested,
            format!("{kind} name `{name}` should be in snake case."),
            format!("Consider renaming it to `{sigil}{suggested}` {advice}"),
        );
    }
}

fn is_special_variable(name: &[u8]) -> bool {
    matches!(
        name,
        b"$this"
            | b"$_"
            | b"$_GET"
            | b"$_POST"
            | b"$_SERVER"
            | b"$_REQUEST"
            | b"$_SESSION"
            | b"$_COOKIE"
            | b"$_FILES"
            | b"$_ENV"
            | b"$GLOBALS"
            | b"$argc"
            | b"$argv"
    )
}

// mago-casing 1.57.0 (common.rs, camel.rs, class.rs, snake.rs, constant.rs). A separator is any byte that is not
// an ASCII letter or digit; trailing separators are dropped.

fn trim_right(input: &[u8]) -> &[u8] {
    let end = input
        .iter()
        .rposition(u8::is_ascii_alphanumeric)
        .map_or(0, |i| i + 1);
    &input[..end]
}

/// `convert_camel_like`, for camel case (`new_word` false) and the camel part of class case (`new_word` true):
/// no separator, not inverted, digits concatenated.
fn convert_camel_like(input: &[u8], mut new_word: bool) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut last_char = b' ';
    let mut found_real_char = false;
    for &byte in trim_right(input) {
        if !byte.is_ascii_alphanumeric() {
            new_word |= found_real_char;
            continue;
        }
        found_real_char = true;
        if byte.is_ascii_digit() {
            new_word = true;
            output.push(byte);
        } else if new_word || (last_char.is_ascii_lowercase() && byte.is_ascii_uppercase()) {
            new_word = false;
            output.push(byte.to_ascii_uppercase());
        } else {
            last_char = byte;
            output.push(byte.to_ascii_lowercase());
        }
    }
    output
}

/// `convert_snake_like` with `_`: lower case for snake case, upper case (where a digit also starts a word) for
/// constant case.
fn convert_snake_like(input: &[u8], upper: bool) -> Vec<u8> {
    let input = trim_right(input);
    let mut output = Vec::with_capacity(input.len() * 2);
    let mut first_character = true;
    for (index, &byte) in input.iter().enumerate() {
        if !byte.is_ascii_alphanumeric() {
            if !first_character {
                first_character = true;
                output.push(b'_');
            }
            continue;
        }
        let upper_like = if upper {
            byte == byte.to_ascii_uppercase()
        } else {
            byte.is_ascii_uppercase()
        };
        let lowercase_neighbor = input.get(index + 1).is_some_and(u8::is_ascii_lowercase)
            || (index > 0 && input[index - 1].is_ascii_lowercase());
        if !first_character && upper_like && lowercase_neighbor {
            output.push(b'_');
        }
        output.push(if upper {
            byte.to_ascii_uppercase()
        } else {
            byte.to_ascii_lowercase()
        });
        first_character = false;
    }
    output
}

/// `class_prefix_len`: a run of uppercase letters and digits, and the lowercase letters and digits after it.
fn class_prefix_len(input: &[u8]) -> usize {
    let head = input
        .iter()
        .take_while(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
        .count();
    if head == 0 || !input.get(head).is_some_and(u8::is_ascii_lowercase) {
        return head;
    }
    head + 1
        + input[head + 1..]
            .iter()
            .take_while(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
            .count()
}

fn convert_class_case(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut cursor = 0;
    while cursor < input.len() {
        let segment = &input[cursor..];
        let prefix = class_prefix_len(segment);
        if prefix == 0 {
            output.extend(convert_camel_like(segment, true));
            break;
        }
        output.extend_from_slice(&segment[..prefix]);
        cursor += prefix;
    }
    output
}

fn string(bytes: Vec<u8>) -> String {
    // Only ASCII bytes are changed or added, and whole multi-byte characters are dropped, so the bytes stay UTF-8.
    String::from_utf8(bytes).unwrap_or_default()
}

fn to_camel_case(name: &str) -> String {
    string(convert_camel_like(name.as_bytes(), false))
}

fn is_camel_case(name: &str) -> bool {
    to_camel_case(name) == name
}

fn to_class_case(name: &str) -> String {
    string(convert_class_case(name.as_bytes()))
}

fn is_class_case(name: &str) -> bool {
    name.as_bytes().first().is_some_and(u8::is_ascii_uppercase) && to_class_case(name) == name
}

fn to_snake_case(name: &str) -> String {
    string(convert_snake_like(name.as_bytes(), false))
}

fn is_snake_case(name: &str) -> bool {
    to_snake_case(name) == name
}

fn to_constant_case(name: &str) -> String {
    string(convert_snake_like(name.as_bytes(), true))
}

fn is_constant_case(name: &str) -> bool {
    to_constant_case(name) == name
}
