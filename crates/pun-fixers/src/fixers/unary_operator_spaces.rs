//! Port of `PhpCsFixer\Fixer\Operator\UnaryOperatorSpacesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct UnaryOperatorSpacesFixer {
    only_dec_inc: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(UnaryOperatorSpacesFixer::default())
}

impl Fixer for UnaryOperatorSpacesFixer {
    fn name(&self) -> &'static str {
        "unary_operator_spaces"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["only_dec_inc"])?;
        self.only_dec_inc = o.bool("only_dec_inc", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if self.only_dec_inc && !tokens[index].is_given_kind(&[T_DEC, T_INC]) {
                continue;
            }
            // TokensAnalyzer holds no state beyond the collection; rebuilt per query since we mutate.
            if TokensAnalyzer::new(tokens).is_unary_successor_operator(index) {
                let prev = tokens.get_prev_non_whitespace(index, None).unwrap();
                if !tokens[prev].is_comment() {
                    tokens.remove_leading_whitespace(index, None);
                }
                continue;
            }
            if TokensAnalyzer::new(tokens).is_unary_predecessor_operator(index) {
                tokens.remove_trailing_whitespace(index, None);
                continue;
            }
        }
    }
}
