//! Port of `PhpCsFixer\Fixer\ReturnNotation\NoUselessReturnFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind, Tokens};
use std::path::Path;

struct NoUselessReturnFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUselessReturnFixer)
}

impl Fixer for NoUselessReturnFixer {
    fn name(&self) -> &'static str {
        "no_useless_return"
    }
    fn priority(&self) -> i32 {
        -18
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[Kind::T_FUNCTION, Kind::T_RETURN])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in 0..tokens.count() {
            if !tokens[i].is_kind(Kind::T_FUNCTION) {
                continue;
            }
            let index = tokens.get_next_token_of_kind(i, &[";".into(), "{".into()], true).unwrap();
            if tokens[index].equals("{", true) {
                let end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
                fix_function(tokens, index, end);
            }
        }
    }
}

fn fix_function(tokens: &mut Tokens, start: usize, end: usize) {
    let mut index = end;
    while index > start {
        if tokens[index].is_kind(Kind::T_RETURN) {
            let next_at = tokens.get_next_meaningful_token(index).unwrap();
            if tokens[next_at].equals(";", true)
                && tokens.get_next_meaningful_token(next_at) == Some(end)
                && !tokens[tokens.get_prev_meaningful_token(index).unwrap()].equals_any(&[Kind::T_ELSE.into(), ")".into()], true)
            {
                tokens.clear_token_and_merge_surrounding_whitespace(index);
                tokens.clear_token_and_merge_surrounding_whitespace(next_at);
            }
        }
        index -= 1;
    }
}
