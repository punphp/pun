//! Port of `PhpCsFixer\Fixer\CastNotation\NoShortBoolCastFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoShortBoolCastFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoShortBoolCastFixer)
}

impl Fixer for NoShortBoolCastFixer {
    fn name(&self) -> &'static str {
        "no_short_bool_cast"
    }
    fn priority(&self) -> i32 {
        -9
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("!")
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() as isize - 1;
        while index > 1 {
            if tokens[index as usize].equals("!", true) {
                index = fix_short_cast(tokens, index as usize) as isize;
            }
            index -= 1;
        }
    }
}

fn fix_short_cast(tokens: &mut Tokens, index: usize) -> usize {
    let mut i = index - 1;
    while i > 1 {
        if tokens[i].equals("!", true) {
            fix_short_cast_to_bool_cast(tokens, i, index);
            break;
        }
        if !tokens[i].is_comment() && !tokens[i].is_whitespace(None) {
            break;
        }
        i -= 1;
    }
    i
}

fn fix_short_cast_to_bool_cast(tokens: &mut Tokens, mut start: usize, end: usize) {
    while start <= end {
        if !tokens[start].is_comment() && !(tokens[start].is_whitespace(None) && tokens[start - 1].is_comment()) {
            tokens.clear_at(start);
        }
        start += 1;
    }
    tokens.insert_at(start, vec![Token::new(T_BOOL_CAST, "(bool)")]);
}
