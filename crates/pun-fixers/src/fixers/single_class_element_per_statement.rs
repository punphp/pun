//! Port of `PhpCsFixer\Fixer\ClassNotation\SingleClassElementPerStatementFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind, Kind::*, Token, Tokens, TokensAnalyzer, php_compat, token::CLASSY_KINDS};
use serde_json::Value;
use std::path::Path;

struct SingleClassElementPerStatementFixer {
    elements: Vec<String>,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SingleClassElementPerStatementFixer {
        elements: vec!["const".into(), "property".into()],
    })
}

impl Fixer for SingleClassElementPerStatementFixer {
    fn name(&self) -> &'static str {
        "single_class_element_per_statement"
    }
    fn priority(&self) -> i32 {
        56
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CLASSY_KINDS)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["elements"])?;
        self.elements = o.strings("elements", &["const", "property"], &["const", "property"])?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let elements = TokensAnalyzer::new(tokens).get_classy_elements();
        for (&index, element) in elements.iter().rev() {
            let ty = element.kind.as_str();
            if !self.elements.iter().any(|e| e == ty) {
                continue; // not in configuration
            }
            fix_element(tokens, ty, index);
        }
    }
}

fn fix_element(tokens: &mut Tokens, ty: &str, index: usize) {
    let mut repeat_index = index;
    loop {
        repeat_index = tokens.get_next_meaningful_token(repeat_index).unwrap();
        let repeat_token = &tokens[repeat_index];
        if TokensAnalyzer::new(tokens).is_array(repeat_index) {
            if repeat_token.is_kind(T_ARRAY) {
                repeat_index = tokens
                    .get_next_token_of_kind(repeat_index, &["(".into()], true)
                    .unwrap();
                repeat_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, repeat_index);
            } else {
                repeat_index =
                    tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, repeat_index);
            }
            continue;
        }
        if repeat_token.equals_any(&[";".into(), CT_PROPERTY_HOOK_BRACE_OPEN.into()], true) {
            return; // no repeating found, no fixing needed
        }
        if repeat_token.equals(",", true) {
            break;
        }
    }
    let start = tokens
        .get_prev_token_of_kind(index, &[";".into(), "{".into(), "}".into()], true)
        .unwrap();
    let start_index = tokens.get_next_meaningful_token(start).unwrap();
    let end_index = tokens
        .get_next_token_of_kind(index, &[";".into()], true)
        .unwrap();
    expand_element(tokens, ty, start_index, end_index);
}

fn expand_element(tokens: &mut Tokens, ty: &str, start_index: usize, end_index: usize) {
    let mut division_content: Option<Vec<u8>> = None;
    if tokens[start_index - 1].is_whitespace(None) {
        let mut d = tokens[start_index - 1].content.clone();
        if let Some(m) = Preg::match_groups("#(\\n|\\r\\n)#", &d, 0) {
            d = [
                m.str(0),
                php_compat::ltrim_chars(php_compat::rtrim_chars(&d, b"\r\n"), b"\r\n"),
            ]
            .concat();
        }
        division_content = Some(d);
    }

    let mut i = end_index - 1;
    while i > start_index {
        let token = &tokens[i];
        if token.equals(")", true) {
            i = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, i) - 1;
            continue;
        }
        if token.is_kind(CT_ARRAY_BRACKET_CLOSE) {
            i = tokens.find_block_start(Tokens::BLOCK_TYPE_ARRAY_BRACKET, i) - 1;
            continue;
        }
        if !token.equals(",", true) {
            i -= 1;
            continue;
        }
        tokens.set(i, Token::char(";"));
        if tokens[i + 1].is_whitespace(None) {
            tokens.clear_at(i + 1);
        }
        if let Some(d) = division_content.as_ref().filter(|d| !d.is_empty()) {
            tokens.insert_at(i + 1, vec![Token::new(T_WHITESPACE, d.clone())]);
        }
        let sequence = if ty == "const" {
            get_constant_modifiers_sequence(tokens, start_index)
        } else {
            get_property_modifiers_sequence(tokens, start_index, end_index)
        };
        tokens.insert_at(i + 2, sequence);
        i -= 1;
    }
}

fn get_constant_modifiers_sequence(tokens: &Tokens, start_index: usize) -> Vec<Token> {
    let assignment_index = tokens
        .get_next_token_of_kind(start_index, &["=".into()], true)
        .unwrap();
    let name_index = tokens.get_prev_meaningful_token(assignment_index).unwrap();
    (start_index..name_index)
        .filter(|&i| !tokens[i].is_comment())
        .map(|i| tokens[i].clone())
        .collect()
}

fn get_property_modifiers_sequence(
    tokens: &Tokens,
    start_index: usize,
    end_index: usize,
) -> Vec<Token> {
    const TOKEN_KINDS: &[Kind] = &[
        T_PUBLIC,
        T_PROTECTED,
        T_PRIVATE,
        T_FINAL,
        T_STATIC,
        T_VAR,
        T_STRING,
        T_NS_SEPARATOR,
        CT_NULLABLE_TYPE,
        CT_ARRAY_TYPEHINT,
        CT_TYPE_ALTERNATION,
        CT_TYPE_INTERSECTION,
        CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN,
        CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
        T_READONLY,
        T_PRIVATE_SET,
        T_PROTECTED_SET,
        T_PUBLIC_SET,
    ];
    let mut sequence = Vec::new();
    for i in start_index..end_index - 1 {
        if tokens[i].is_comment() {
            continue;
        }
        if !tokens[i].is_whitespace(None) && !tokens[i].is_given_kind(TOKEN_KINDS) {
            break;
        }
        sequence.push(tokens[i].clone());
    }
    sequence
}
