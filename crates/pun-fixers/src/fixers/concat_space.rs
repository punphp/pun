//! Port of `PhpCsFixer\Fixer\Operator\ConcatSpaceFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spacing {
    One,
    None,
}

struct ConcatSpaceFixer {
    spacing: Spacing,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(ConcatSpaceFixer {
        spacing: Spacing::None,
    })
}

impl ConcatSpaceFixer {
    fn fix_concatenation_to_no_space(tokens: &mut Tokens, index: usize) {
        let prev_non_whitespace_token =
            &tokens[tokens.get_prev_non_whitespace(index, None).unwrap()];
        if !prev_non_whitespace_token.is_given_kind(&[T_LNUMBER, T_COMMENT, T_DOC_COMMENT])
            || prev_non_whitespace_token.get_content().starts_with(b"/*")
        {
            tokens.remove_leading_whitespace(index, Some(b" \t"));
        }
        if !tokens[tokens.get_next_non_whitespace(index, None).unwrap()].is_given_kind(&[
            T_LNUMBER,
            T_COMMENT,
            T_DOC_COMMENT,
        ]) {
            tokens.remove_trailing_whitespace(index, Some(b" \t"));
        }
    }

    fn fix_concatenation_to_single_space(tokens: &mut Tokens, index: usize) {
        Self::fix_white_space_around_concat_token(tokens, index, 1);
        Self::fix_white_space_around_concat_token(tokens, index, -1);
    }

    fn fix_white_space_around_concat_token(tokens: &mut Tokens, index: usize, offset: isize) {
        let offset_index = index.wrapping_add_signed(offset);
        if !tokens[offset_index].is_whitespace(None) {
            tokens.insert_at(
                index + if offset == 1 { 1 } else { 0 },
                vec![Token::new(T_WHITESPACE, " ")],
            );
            return;
        }
        if tokens[offset_index].get_content().contains(&b'\n') {
            return;
        }
        if tokens[index.wrapping_add_signed(offset * 2)].is_comment() {
            return;
        }
        tokens.set(offset_index, Token::new(T_WHITESPACE, " "));
    }
}

impl Fixer for ConcatSpaceFixer {
    fn name(&self) -> &'static str {
        "concat_space"
    }
    fn priority(&self) -> i32 {
        // Must run after NoUnneededControlParenthesesFixer, SingleLineThrowFixer.
        0
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["spacing"])?;
        self.spacing = r.choice(
            "spacing",
            "none",
            &[("one", Spacing::One), ("none", Spacing::None)],
        )?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(".")
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if tokens[index].equals(".", true) {
                if self.spacing == Spacing::One {
                    Self::fix_concatenation_to_single_space(tokens, index);
                } else {
                    Self::fix_concatenation_to_no_space(tokens, index);
                }
            }
        }
    }
}
