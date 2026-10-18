//! Port of `PhpCsFixer\Fixer\Phpdoc\NoEmptyPhpdocFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

struct NoEmptyPhpdocFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoEmptyPhpdocFixer)
}

impl Fixer for NoEmptyPhpdocFixer {
    fn name(&self) -> &'static str {
        "no_empty_phpdoc"
    }
    fn priority(&self) -> i32 {
        3
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        while index < tokens.count() {
            if !tokens[index].is_kind(Kind::T_DOC_COMMENT)
                || !Preg::r#match(r"#^/\*\*[\s\*]*\*/$#", tokens[index].get_content())
            {
                index += 1;
                continue;
            }
            if tokens[index - 1].is_given_kind(&[Kind::T_OPEN_TAG, Kind::T_WHITESPACE])
                && tokens[index - 1].get_content().contains(&b'\n')
                && tokens[index + 1].is_kind(Kind::T_WHITESPACE)
                && Preg::r#match(r"/^\R/", tokens[index + 1].get_content())
            {
                let prev = &tokens[index - 1];
                let t = Token::new(
                    prev.get_id().unwrap(),
                    Preg::replace(r"/\h*$/", "", prev.get_content()),
                );
                tokens.set(index - 1, t);
                let new_content = Preg::replace(r"/^\R/", "", tokens[index + 1].get_content());
                if new_content.is_empty() {
                    tokens.clear_at(index + 1);
                } else {
                    tokens.set(index + 1, Token::new(Kind::T_WHITESPACE, new_content));
                }
            }
            tokens.clear_token_and_merge_surrounding_whitespace(index);
            index += 1;
        }
    }
}
