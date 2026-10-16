//! Port of `PhpCsFixer\Fixer\ControlStructure\NoUnneededBracesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

struct NoUnneededBracesFixer {
    namespaces: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUnneededBracesFixer { namespaces: false })
}

impl Fixer for NoUnneededBracesFixer {
    fn name(&self) -> &'static str {
        "no_unneeded_braces"
    }
    fn priority(&self) -> i32 {
        40
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("}") || tokens.is_token_kind_found(CT_GROUP_IMPORT_BRACE_CLOSE)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["namespaces"])?;
        self.namespaces = o.bool("namespaces", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // findBraceOpen: a generator over a decreasing index; clearing keeps indices stable
        let mut i = tokens.count();
        while i > 1 {
            i -= 1;
            if tokens[i].equals_any(&["{".into(), CT_GROUP_IMPORT_BRACE_OPEN.into()], true) && is_over_complete(tokens, i) {
                clear_over_complete_braces(tokens, i);
            }
        }
        if self.namespaces {
            clear_if_is_over_complete_namespace_block(tokens);
        }
    }
}

fn clear_over_complete_braces(tokens: &mut Tokens, open_index: usize) {
    let (block_type, _) = Tokens::detect_block_type(&tokens[open_index]).unwrap();
    let close_index = tokens.find_block_end(block_type, open_index);
    tokens.clear_token_and_merge_surrounding_whitespace(close_index);
    tokens.clear_token_and_merge_surrounding_whitespace(open_index);
}

fn is_over_complete(tokens: &Tokens, index: usize) -> bool {
    if tokens[index].is_kind(CT_GROUP_IMPORT_BRACE_OPEN) {
        let comma_or_close_brace_index =
            tokens.get_next_token_of_kind(index, &[",".into(), CT_GROUP_IMPORT_BRACE_CLOSE.into()], true).unwrap();
        if TokensAnalyzer::new(tokens).is_block_multiline(index) {
            return false;
        }
        return tokens[comma_or_close_brace_index].is_kind(CT_GROUP_IMPORT_BRACE_CLOSE);
    }
    tokens[tokens.get_prev_meaningful_token(index).unwrap()]
        .equals_any(&["{".into(), "}".into(), T_OPEN_TAG.into(), ":".into(), ";".into()], true)
}

fn clear_if_is_over_complete_namespace_block(tokens: &mut Tokens) {
    if tokens.count_token_kind(T_NAMESPACE) != 1 {
        return; // fast check, we never fix if multiple namespaces are defined
    }
    let index = tokens.get_next_token_of_kind(0, &[T_NAMESPACE.into()], true).unwrap();
    let mut namespace_is_named = false;
    let mut index = tokens.get_next_meaningful_token(index).unwrap();
    while tokens[index].is_given_kind(&[T_STRING, T_NS_SEPARATOR]) {
        index = tokens.get_next_meaningful_token(index).unwrap();
        namespace_is_named = true;
    }
    if !namespace_is_named {
        return;
    }
    if !tokens[index].equals("{", true) {
        return; // `;`
    }
    let close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
    if let Some(after_close_index) = tokens.get_next_meaningful_token(close_index) {
        if !tokens[after_close_index].is_kind(T_CLOSE_TAG) || tokens.get_next_meaningful_token(after_close_index).is_some() {
            return;
        }
    }
    // clear up
    tokens.clear_token_and_merge_surrounding_whitespace(close_index);
    tokens.set(index, Token::char(";"));
    if tokens[index - 1].is_whitespace(Some(b" \t")) && !tokens[index - 2].is_comment() {
        tokens.clear_token_and_merge_surrounding_whitespace(index - 1);
    }
}
