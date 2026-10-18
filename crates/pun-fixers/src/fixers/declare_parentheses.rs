//! Port of `PhpCsFixer\Fixer\LanguageConstruct\DeclareParenthesesFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

struct DeclareParenthesesFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(DeclareParenthesesFixer)
}

impl Fixer for DeclareParenthesesFixer {
    fn name(&self) -> &'static str {
        "declare_parentheses"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DECLARE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_kind(T_DECLARE) {
                continue;
            }
            tokens.remove_trailing_whitespace(index, None);
            let start_parenthesis_index = tokens
                .get_next_token_of_kind(index, &["(".into()], true)
                .unwrap();
            tokens.remove_trailing_whitespace(start_parenthesis_index, None);
            let end_parenthesis_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start_parenthesis_index);
            tokens.remove_leading_whitespace(end_parenthesis_index, None);
        }
    }
}
