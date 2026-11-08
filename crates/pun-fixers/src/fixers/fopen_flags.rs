//! Port of `PhpCsFixer\Fixer\FunctionNotation\FopenFlagsFixer`.
use crate::analyzers::abstract_fopen_flag_fixer::{
    self as base, flag_index, is_valid_mode_string, split_literal,
};
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::php_compat::str_replace;
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct FopenFlagsFixer {
    b_mode: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(FopenFlagsFixer { b_mode: false })
}

impl FopenFlagsFixer {
    fn fix_fopen_flag_token(&self, tokens: &mut Tokens, start: usize, end: usize) {
        let Some(index) = flag_index(tokens, start, end) else {
            return;
        };
        let content = tokens[index].get_content();
        let (bin_prefix, quote, mode) = split_literal(content);
        if !is_valid_mode_string(mode) {
            return;
        }
        let mut mode = str_replace(b"t", b"", mode);
        if self.b_mode {
            if !mode.contains(&b'b') {
                mode.push(b'b');
            }
        } else {
            mode = str_replace(b"b", b"", &mode);
        }
        let mut new_content = bin_prefix.to_vec();
        new_content.push(quote);
        new_content.extend_from_slice(&mode);
        new_content.push(quote);
        if content != new_content {
            tokens.set(index, Token::new(T_CONSTANT_ENCAPSED_STRING, new_content));
        }
    }
}

impl Fixer for FopenFlagsFixer {
    fn name(&self) -> &'static str {
        "fopen_flags"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["b_mode"])?;
        self.b_mode = c.bool("b_mode", true)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        base::is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        base::apply_fix(tokens, |t, s, e| self.fix_fopen_flag_token(t, s, e));
    }
}
