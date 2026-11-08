//! Port of `PhpCsFixer\Fixer\FunctionNotation\FopenFlagOrderFixer`.
use crate::analyzers::abstract_fopen_flag_fixer::{
    self as base, flag_index, is_valid_mode_string, split_literal,
};
use crate::analyzers::preg::{PREG_SPLIT_DELIM_CAPTURE, PREG_SPLIT_NO_EMPTY, Preg};
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use std::cmp::Ordering;
use std::path::Path;

struct FopenFlagOrderFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(FopenFlagOrderFixer)
}

fn fix_fopen_flag_token(tokens: &mut Tokens, start: usize, end: usize) {
    let Some(index) = flag_index(tokens, start, end) else {
        return;
    };
    let content = tokens[index].get_content();
    let (bin_prefix, quote, mode) = split_literal(content);
    if mode.len() < 2 {
        return; // nothing to sort
    }
    if !is_valid_mode_string(mode) {
        return;
    }
    let mut split: Vec<Vec<u8>> = Preg::split(
        r"#([^\+]\+?)#",
        mode,
        -1,
        PREG_SPLIT_NO_EMPTY | PREG_SPLIT_DELIM_CAPTURE,
    )
    .into_iter()
    .map(|p| p.text)
    .collect();
    split.sort_by(|a, b| sort_flags(a, b));
    let mut new_content = bin_prefix.to_vec();
    new_content.push(quote);
    new_content.extend(split.concat());
    new_content.push(quote);
    if content != new_content {
        tokens.set(index, Token::new(T_CONSTANT_ENCAPSED_STRING, new_content));
    }
}

/// `sortFlags`' comparator: `b` last, `t` before it, the rest by byte order.
fn sort_flags(flag1: &[u8], flag2: &[u8]) -> Ordering {
    // A valid mode holds each flag once, so two flags are never equal (upstream's `0` branch).
    for last in [b"b", b"t"] {
        if flag1 == last {
            return Ordering::Greater;
        }
        if flag2 == last {
            return Ordering::Less;
        }
    }
    flag1.cmp(flag2)
}

impl Fixer for FopenFlagOrderFixer {
    fn name(&self) -> &'static str {
        "fopen_flag_order"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        base::is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        base::apply_fix(tokens, fix_fopen_flag_token);
    }
}
