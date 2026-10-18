//! Port of `PhpCsFixer\Fixer\Operator\NoSpaceAroundDoubleColonFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

struct NoSpaceAroundDoubleColonFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoSpaceAroundDoubleColonFixer)
}

impl NoSpaceAroundDoubleColonFixer {
    fn remove_space(tokens: &mut Tokens, index: usize, direction: isize) {
        let side = index.wrapping_add_signed(direction);
        if !tokens[side].is_whitespace(None) {
            return;
        }
        if tokens[tokens
            .get_non_whitespace_sibling(index, direction, None)
            .unwrap()]
        .is_comment()
        {
            return;
        }
        tokens.clear_at(side);
    }
}

impl Fixer for NoSpaceAroundDoubleColonFixer {
    fn name(&self) -> &'static str {
        "no_space_around_double_colon"
    }
    fn priority(&self) -> i32 {
        // Must run before MethodChainingIndentationFixer.
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOUBLE_COLON)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() as isize - 2;
        while index > 1 {
            let i = index as usize;
            if tokens[i].is_kind(T_DOUBLE_COLON) {
                Self::remove_space(tokens, i, 1);
                Self::remove_space(tokens, i, -1);
            }
            index -= 1;
        }
    }
}
