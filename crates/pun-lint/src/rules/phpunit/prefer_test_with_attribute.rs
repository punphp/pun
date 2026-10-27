//! `lint/phpunit/prefer-test-with-attribute`, ported from rector-phpunit's `TestWithAnnotationToAttributeRector`
//! (rules/AnnotationsToAttributes/Rector/ClassMethod).
//!
//! A test method's generic `@testWith` (then `@testwith`) tags become one
//! `#[\PHPUnit\Framework\Attributes\TestWith(...)]` group per line of JSON, after its attributes, when that attribute
//! class exists. Rector runs it only for PHPUnit 10 and later; Pun leaves that to the rule's group. A line that is
//! not JSON makes Rector fail on the whole file; Pun leaves such a method alone.

use mago_syntax::cst::{Method, Node, NodeKind};
use pun_problem::Safety;
use serde_json::Value;

use super::annotations::{Annotated, TagValue, map_item, named_tags};
use super::{is_public, report};
use crate::rules::lint_b_doc::Doc;
use crate::rules::rector_batch5::has_attribute;
use crate::rules::types::facts::facts;
use crate::{Context, Report, Rule};

const TEST_WITH_ATTRIBUTE: &str = "PHPUnit\\Framework\\Attributes\\TestWith";

pub struct PreferTestWithAttribute;

impl Rule for PreferTestWithAttribute {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-test-with-attribute"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else {
            return;
        };
        let Some(annotated) = Annotated::of(ctx.source, node, ancestors) else {
            return;
        };
        let Some(doc) = annotated.doc.as_ref() else {
            return;
        };
        if !doc.text_all().contains("@testWith") && !doc.text_all().contains("@testwith") {
            return;
        }
        if is_test_class_method(ctx, ancestors, method, doc) != Some(true)
            // make sure the attribute class exists
            || facts(ctx)
                .codebase
                .get_class_like(TEST_WITH_ATTRIBUTE.as_bytes())
                .is_none()
        {
            return;
        }
        let (Some(upper), Some(lower)) = (
            named_tags(ctx, ancestors, doc, "testWith"),
            named_tags(ctx, ancestors, doc, "testwith"),
        ) else {
            return;
        };
        let mut removed = vec![false; doc.len()];
        let mut groups = Vec::new();
        for (i, value) in upper.into_iter().chain(lower) {
            let TagValue::Generic(value) = value else {
                // not supported yet
                continue;
            };
            // test from doc blocks
            removed[i] = true;
            for item in php_trim(&value).split('\n') {
                let Some(argument) = decode(php_trim(item)) else {
                    return;
                };
                groups.push(format!("#[\\{TEST_WITH_ATTRIBUTE}({argument})]"));
            }
        }
        if groups.is_empty() {
            return;
        }
        let new_doc = doc.print(&removed);
        let Some(edits) = annotated.edits(ctx.source, Some(&new_doc), &groups) else {
            return;
        };
        out.push(report(
            doc.span,
            "Use the `#[TestWith]` attribute instead of the `@testWith` annotation.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `TestsNodeAnalyzer::isTestClassMethod()`: public, and named `test…`, or with a `#[Test]` attribute or a `@test`
/// tag; `None` for a doc Pun does not read.
fn is_test_class_method(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    method: &Method<'_>,
    doc: &Doc<'_>,
) -> Option<bool> {
    if !is_public(method) {
        return Some(false);
    }
    if method.name.value.starts_with(b"test")
        || has_attribute(
            ancestors,
            &method.attribute_lists,
            "PHPUnit\\Framework\\Attributes\\Test",
        )
    {
        return Some(true);
    }
    Some(!named_tags(ctx, ancestors, doc, "test")?.is_empty())
}

/// PHP's `trim()`.
fn php_trim(s: &str) -> &str {
    s.trim_matches([' ', '\t', '\n', '\r', '\0', '\x0B'])
}

/// `Json::decode($json, forceArrays: true)` mapped by `AnnotationToAttributeMapper` and printed by php-parser;
/// `None` where it throws, or for an integer past 64 bits (`JSON_BIGINT_AS_STRING`), which Pun does not follow.
fn decode(json: &str) -> Option<String> {
    let mut digits = 0;
    for c in json.bytes() {
        digits = if c.is_ascii_digit() { digits + 1 } else { 0 };
        if digits >= 19 {
            return None;
        }
    }
    let value: Value = serde_json::from_str(json).ok()?;
    print_value(&value)
}

/// A decoded JSON value as an attribute argument: arrays through `ArrayAnnotationToAttributeMapper` (an integer key
/// is left out), strings through [`map_item`], the rest through `BuilderHelpers::normalizeValue()`.
fn print_value(value: &Value) -> Option<String> {
    Some(match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => match n.as_i64() {
            Some(i) => i.to_string(),
            None => print_float(n.as_f64()?),
        },
        Value::String(s) => {
            // `DocTagNodeState::REMOVE_ARRAY`, which the mapper drops
            if s == "__remove_array__" {
                return None;
            }
            map_item(s)
        }
        Value::Array(items) => {
            let items: Option<Vec<String>> = items.iter().map(print_value).collect();
            format!("[{}]", items?.join(", "))
        }
        Value::Object(map) => {
            let mut items = Vec::new();
            for (key, value) in map {
                let value = print_value(value)?;
                // json_decode() makes a decimal integer key an int
                let int_key = key == "0"
                    || (key
                        .trim_start_matches('-')
                        .starts_with(|c: char| ('1'..='9').contains(&c))
                        && key[usize::from(key.starts_with('-'))..]
                            .bytes()
                            .all(|c| c.is_ascii_digit())
                        && key.parse::<i64>().is_ok());
                items.push(if int_key {
                    value
                } else {
                    format!("{} => {value}", map_item(key))
                });
            }
            format!("[{}]", items.join(", "))
        }
    })
}

/// php-parser's `pScalar_Float`: `%.16G`, or `%.17G` when that does not read back as the same float, with `.0` after a
/// whole number.
pub(super) fn print_float(value: f64) -> String {
    let mut printed = php_g(value, 16);
    if printed.parse::<f64>().ok() != Some(value) {
        printed = php_g(value, 17);
    }
    let digits = printed.strip_prefix('-').unwrap_or(&printed);
    if digits.bytes().all(|c| c.is_ascii_digit()) {
        printed.push_str(".0");
    }
    printed
}

/// PHP's `sprintf('%.{precision}G', $value)`: the shorter of `%E` (a whole mantissa with `.0`, the exponent without
/// padding) and `%F`, without trailing zeros.
fn php_g(value: f64, precision: usize) -> String {
    if value == 0.0 {
        return if value.is_sign_negative() { "-0" } else { "0" }.to_string();
    }
    let scientific = format!("{:.*e}", precision - 1, value);
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let exponent: i32 = exponent.parse().unwrap_or(0);
    let (sign, mantissa) = match mantissa.strip_prefix('-') {
        Some(m) => ("-", m),
        None => ("", mantissa),
    };
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    if exponent < -4 || exponent >= precision as i32 {
        let digits = digits.trim_end_matches('0');
        let (first, rest) = digits.split_at(1);
        let rest = if rest.is_empty() { "0" } else { rest };
        let exponent_sign = if exponent < 0 { '-' } else { '+' };
        return format!("{sign}{first}.{rest}E{exponent_sign}{}", exponent.abs());
    }
    let fixed = if exponent < 0 {
        format!("0.{}{digits}", "0".repeat((-exponent - 1) as usize))
    } else {
        let point = exponent as usize + 1;
        format!("{}.{}", &digits[..point], &digits[point..])
    };
    let fixed = fixed.trim_end_matches('0').trim_end_matches('.');
    format!("{sign}{fixed}")
}
