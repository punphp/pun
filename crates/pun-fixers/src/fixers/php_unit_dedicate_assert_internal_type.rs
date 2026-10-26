//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitDedicateAssertInternalTypeFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate};
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
struct PhpUnitDedicateAssertInternalTypeFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitDedicateAssertInternalTypeFixer)
}

const TYPE_TO_DEDICATED_ASSERT: &[(&[u8], &str)] = &[
    (b"array", "assertIsArray"),
    (b"boolean", "assertIsBool"),
    (b"bool", "assertIsBool"),
    (b"double", "assertIsFloat"),
    (b"float", "assertIsFloat"),
    (b"integer", "assertIsInt"),
    (b"int", "assertIsInt"),
    (b"null", "assertNull"),
    (b"numeric", "assertIsNumeric"),
    (b"object", "assertIsObject"),
    (b"real", "assertIsFloat"),
    (b"resource", "assertIsResource"),
    (b"string", "assertIsString"),
    (b"scalar", "assertIsScalar"),
    (b"callable", "assertIsCallable"),
    (b"iterable", "assertIsIterable"),
];

impl Fixer for PhpUnitDedicateAssertInternalTypeFixer {
    fn name(&self) -> &'static str {
        "php_unit_dedicate_assert_internal_type"
    }
    fn priority(&self) -> i32 {
        -16
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["target"])?;
        c.string("target", "newest", &["7.5", "newest"])?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, apply_php_unit_class_fix);
    }
}

fn apply_php_unit_class_fix(tokens: &mut Tokens, start_index: usize, end_index: usize) {
    let mut anonymous_class_indices: HashMap<usize, usize> = HashMap::new();
    for index in start_index..end_index {
        if !tokens[index].is_kind(T_CLASS) || !TokensAnalyzer::new(tokens).is_anonymous_class(index)
        {
            continue;
        }
        let opening = tokens
            .get_next_token_of_kind(index, &["{".into()], true)
            .unwrap();
        let closing = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, opening);
        anonymous_class_indices.insert(closing, opening);
    }
    let mut index = end_index - 1;
    while index > start_index {
        if let Some(&opening) = anonymous_class_indices.get(&index) {
            index = opening - 1;
            continue;
        }
        fix_at(tokens, index);
        index -= 1;
    }
}

fn fix_at(tokens: &mut Tokens, index: usize) {
    if !tokens[index].is_kind(T_STRING) {
        return;
    }
    let function_name = strtolower(tokens[index].get_content());
    let not = match function_name.as_slice() {
        b"assertinternaltype" => false,
        b"assertnotinternaltype" => true,
        _ => return,
    };
    let bracket = tokens.get_next_meaningful_token(index).unwrap();
    if !tokens[bracket].equals("(", true) {
        return;
    }
    let expected_type_index = tokens.get_next_meaningful_token(bracket).unwrap();
    if !tokens[expected_type_index].is_kind(T_CONSTANT_ENCAPSED_STRING) {
        return;
    }
    let content = tokens[expected_type_index].get_content();
    let start = content
        .iter()
        .position(|b| *b != b'\'' && *b != b'"')
        .unwrap_or(content.len());
    let end = content
        .iter()
        .rposition(|b| *b != b'\'' && *b != b'"')
        .map_or(start, |e| e + 1);
    let expected_type = &content[start..end.max(start)];
    let Some((_, assertion)) = TYPE_TO_DEDICATED_ASSERT
        .iter()
        .find(|(t, _)| *t == expected_type)
    else {
        return;
    };
    let comma = tokens
        .get_next_meaningful_token(expected_type_index)
        .unwrap();
    if !tokens[comma].equals(",", true) {
        return;
    }
    let mut new_assertion = assertion.to_string();
    if not {
        new_assertion = new_assertion
            .replace("Is", "IsNot")
            .replace("Null", "NotNull");
    }
    let next = tokens.get_next_meaningful_token(comma).unwrap();
    tokens.override_range(
        index,
        next - 1,
        vec![
            Token::new(T_STRING, new_assertion.into_bytes()),
            Token::char(b"(".to_vec()),
        ],
    );
}
