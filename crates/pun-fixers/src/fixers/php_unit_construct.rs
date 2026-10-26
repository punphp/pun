//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitConstructFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

const ASSERT_METHODS: &[&str] = &[
    "assertEquals",
    "assertNotEquals",
    "assertNotSame",
    "assertSame",
];

struct PhpUnitConstructFixer {
    assertions: Vec<String>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitConstructFixer {
        assertions: ASSERT_METHODS.iter().map(|m| m.to_string()).collect(),
    })
}

impl Fixer for PhpUnitConstructFixer {
    fn name(&self) -> &'static str {
        "php_unit_construct"
    }
    fn priority(&self) -> i32 {
        -8
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["assertions"])?;
        self.assertions = c.strings("assertions", ASSERT_METHODS, ASSERT_METHODS)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            for assertion_method in &self.assertions {
                // `fixAssertPositive` or `fixAssertNegative`
                let map: [&str; 3] =
                    if assertion_method == "assertSame" || assertion_method == "assertEquals" {
                        ["assertFalse", "assertNull", "assertTrue"]
                    } else {
                        ["assertNotFalse", "assertNotNull", "assertNotTrue"]
                    };
                let mut index = start_index;
                while index < end_index {
                    let Some(next) = fix_assert(map, tokens, index, assertion_method) else {
                        break;
                    };
                    index = next + 1;
                }
            }
        });
    }
}

/// `fixAssert`; `map` holds the replacements for `false`, `null` and `true`.
fn fix_assert(map: [&str; 3], tokens: &mut Tokens, index: usize, method: &str) -> Option<usize> {
    let sequence = tokens.find_sequence(
        &[(T_STRING, method).into(), "(".into()],
        index,
        None,
        |_| true,
    )?;
    if !FunctionsAnalyzer::new().is_the_same_class_call(tokens, sequence[0]) {
        return None;
    }
    let first_parameter_index = tokens.get_next_meaningful_token(sequence[1]).unwrap();
    let first_parameter = &tokens[first_parameter_index];
    let replacement = match first_parameter.is_kind(T_STRING) {
        true => match strtolower(first_parameter.get_content()).as_slice() {
            b"false" => map[0],
            b"null" => map[1],
            b"true" => map[2],
            _ => return Some(first_parameter_index),
        },
        false => return Some(first_parameter_index),
    };
    let comma_index = tokens
        .get_next_meaningful_token(first_parameter_index)
        .unwrap();
    // return if first method argument is an expression, not value
    if !tokens[comma_index].equals(",", true) {
        return Some(comma_index);
    }
    tokens.set(
        sequence[0],
        Token::new(T_STRING, replacement.as_bytes().to_vec()),
    );
    let end = tokens.get_next_non_whitespace(comma_index, None).unwrap();
    tokens.clear_range(first_parameter_index, end - 1);
    Some(comma_index)
}
