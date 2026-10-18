//! Port of `PhpCsFixer\Fixer\ArrayNotation\WhitespaceAfterCommaInArrayFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Default)]
struct WhitespaceAfterCommaInArrayFixer {
    ensure_single_space: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(WhitespaceAfterCommaInArrayFixer::default())
}

impl Fixer for WhitespaceAfterCommaInArrayFixer {
    fn name(&self) -> &'static str {
        "whitespace_after_comma_in_array"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["ensure_single_space"])?;
        self.ensure_single_space = o.bool("ensure_single_space", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut tokens_to_insert: BTreeMap<usize, Vec<Token>> = BTreeMap::new();
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_given_kind(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN]) {
                continue;
            }
            let (start_index, end_index) = if tokens[index].is_kind(CT_ARRAY_BRACKET_OPEN) {
                (
                    index,
                    tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, index),
                )
            } else {
                let s = tokens
                    .get_next_token_of_kind(index, &["(".into()], true)
                    .unwrap();
                (s, tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, s))
            };
            let mut i = end_index - 1;
            while i > start_index {
                i = skip_non_array_elements(i, tokens);
                if tokens[i].equals(",", true) {
                    if !tokens[i + 1].is_whitespace(None) {
                        tokens_to_insert.insert(i + 1, vec![Token::new(T_WHITESPACE, " ")]);
                    } else if self.ensure_single_space
                        && tokens[i + 1].content != b" "
                        && Preg::r#match("/^\\h+$/", &tokens[i + 1].content)
                        && (!tokens[i + 2].is_comment()
                            || Preg::r#match("/^\\h+$/", &tokens[i + 3].content))
                    {
                        tokens.set(i + 1, Token::new(T_WHITESPACE, " "));
                    }
                }
                i -= 1;
            }
        }
        if !tokens_to_insert.is_empty() {
            tokens.insert_slices(tokens_to_insert);
        }
    }
}

fn skip_non_array_elements(mut index: usize, tokens: &Tokens) -> usize {
    if tokens[index].equals("}", true) {
        return tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, index);
    }
    if tokens[index].equals(")", true) {
        let start_index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, index);
        let start_index = tokens.get_prev_meaningful_token(start_index).unwrap();
        if !tokens[start_index].is_given_kind(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN]) {
            return start_index;
        }
    }
    if tokens[index].equals(",", true) && comma_is_part_of_implements_list(index, tokens) {
        index -= 1;
    }
    index
}

fn comma_is_part_of_implements_list(mut index: usize, tokens: &Tokens) -> bool {
    loop {
        index = tokens.get_prev_meaningful_token(index).unwrap();
        let current = &tokens[index];
        if !(current.is_kind(T_STRING) || current.equals(",", true)) {
            return current.is_kind(T_IMPLEMENTS);
        }
    }
}
