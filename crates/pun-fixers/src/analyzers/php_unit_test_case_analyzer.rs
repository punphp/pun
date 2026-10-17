//! Port of `PhpCsFixer\Tokenizer\Analyzer\PhpUnitTestCaseAnalyzer`.
use crate::analyzers::preg::Preg;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;

pub struct PhpUnitTestCaseAnalyzer;

impl PhpUnitTestCaseAnalyzer {
    /// Generator step: the next PHPUnit class at or below `index` (its T_CLASS index and body range).
    pub fn next_php_unit_class(
        &self,
        tokens: &Tokens,
        mut index: isize,
    ) -> Option<(usize, (usize, usize))> {
        while index > 0 {
            let i = index as usize;
            if self.is_php_unit_class(tokens, i) {
                let start_index = tokens
                    .get_next_token_of_kind(i, &["{".into()], true)
                    .unwrap();
                let end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, start_index);
                return Some((i, (start_index, end_index)));
            }
            index -= 1;
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
