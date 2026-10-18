//! Port of `PhpCsFixer\Tokenizer\Analyzer\PhpUnitTestCaseAnalyzer`.
use crate::analyzers::preg::Preg;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;

pub(crate) struct PhpUnitTestCaseAnalyzer;

impl PhpUnitTestCaseAnalyzer {
    /// Generator step of `findPhpUnitClasses`: the last PHPUnit class whose T_CLASS index is below `before` (that
    /// index and the body range), or `None`. Upstream starts at `count() - 1` and stops before index 0.
    pub(crate) fn next_php_unit_class(
        &self,
        tokens: &Tokens,
        before: usize,
    ) -> Option<(usize, (usize, usize))> {
        for i in (1..before).rev() {
            if self.is_php_unit_class(tokens, i) {
                let start_index = tokens
                    .get_next_token_of_kind(i, &["{".into()], true)
                    .unwrap();
                let end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, start_index);
                return Some((i, (start_index, end_index)));
            }
        }
        None
    }

    fn is_php_unit_class(&self, tokens: &Tokens, mut index: usize) -> bool {
        if !tokens[index].is_kind(T_CLASS) {
            return false;
        }
        let extends_index = tokens
            .get_next_token_of_kind(index, &["{".into(), T_EXTENDS.into()], true)
            .unwrap();
        if !tokens[extends_index].is_kind(T_EXTENDS) {
            return false;
        }
        // (Upstream also matches /(?:Test|TestCase)$/ against the T_CLASS token's own content, `class`: never true.)
        while let Some(i) = tokens.get_next_meaningful_token(index) {
            index = i;
            if tokens[i].equals("{", true) {
                break;
            }
            if !tokens[i].is_kind(T_STRING) {
                continue; // not part of extends nor part of implements; so continue
            }
            if Preg::r#match("/(?:Test|TestCase)(?:Interface)?$/", &tokens[i].content) {
                return true;
            }
        }
        false
    }
}
