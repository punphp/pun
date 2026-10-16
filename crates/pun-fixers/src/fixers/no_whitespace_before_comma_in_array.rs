//! Port of `PhpCsFixer\Fixer\ArrayNotation\NoWhitespaceBeforeCommaInArrayFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use serde_json::Value;
use std::path::Path;

struct NoWhitespaceBeforeCommaInArrayFixer {
    after_heredoc: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoWhitespaceBeforeCommaInArrayFixer { after_heredoc: false })
}

impl Fixer for NoWhitespaceBeforeCommaInArrayFixer {
    fn name(&self) -> &'static str {
        "no_whitespace_before_comma_in_array"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["after_heredoc"])?;
        // Future::getV4OrV3(true, false)
        self.after_heredoc = o.bool("after_heredoc", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (1..tokens.count()).rev() {
            if tokens[index].is_given_kind(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN]) {
                self.fix_spacing(index, tokens);
            }
        }
    }
}

impl NoWhitespaceBeforeCommaInArrayFixer {
    fn fix_spacing(&self, index: usize, tokens: &mut Tokens) {
        let (start_index, end_index) = if tokens[index].is_kind(CT_ARRAY_BRACKET_OPEN) {
            (index, tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, index))
        } else {
            let s = tokens.get_next_token_of_kind(index, &["(".into()], true).unwrap();
            (s, tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, s))
        };

        let mut i = end_index - 1;
        while i > start_index {
            i = skip_non_array_elements(i, tokens);
            let prev_index = tokens.get_prev_non_whitespace(i - 1, None).unwrap();
            if tokens[i].equals(",", true)
                && !tokens[prev_index].is_comment()
                && (self.after_heredoc || !tokens[prev_index].is_kind(T_END_HEREDOC))
            {
                tokens.remove_leading_whitespace(i, None);
            }
            i -= 1;
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
