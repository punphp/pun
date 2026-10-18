//! Port of `PhpCsFixer\Fixer\PhpTag\NoClosingTagFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

struct NoClosingTagFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoClosingTagFixer)
}

impl Fixer for NoClosingTagFixer {
    fn name(&self) -> &'static str {
        "no_closing_tag"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.count() >= 2 && tokens.is_monolithic_php() && tokens.is_token_kind_found(T_CLOSE_TAG)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let index = tokens.find_given_kind(T_CLOSE_TAG, 0, None)[0];
        if index >= 1 && tokens[index - 1].is_whitespace(None) {
            tokens.clear_at(index - 1);
        }
        tokens.clear_at(index);
        let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
        if !tokens[prev_index].equals_any(&[";".into(), "}".into(), T_OPEN_TAG.into()], true) {
            tokens.insert_at(prev_index + 1, vec![Token::char(";")]);
        }
    }
}
