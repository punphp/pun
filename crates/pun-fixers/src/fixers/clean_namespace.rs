//! Port of `PhpCsFixer\Fixer\NamespaceNotation\CleanNamespaceFixer`. Only a candidate on PHP < 8.0
//! (`\PHP_VERSION_ID < 8_00_00`); the oracle runs PHP 8.5, so it never applies.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

struct CleanNamespaceFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(CleanNamespaceFixer)
}

/// PHP_VERSION_ID of the oracle (PHP 8.5).
const PHP_VERSION_ID: u32 = 8_05_00;

impl CleanNamespaceFixer {
    fn fix_namespace(tokens: &mut Tokens, mut index: usize) -> usize {
        let mut till_index = index;
        // go to the end of the namespace
        while tokens[till_index].is_given_kind(&[T_NS_SEPARATOR, T_STRING]) {
            till_index = tokens.get_next_meaningful_token(till_index).unwrap();
        }
        till_index = tokens.get_prev_meaningful_token(till_index).unwrap();
        let mut space_indices = Vec::new();
        while index <= till_index {
            if tokens[index].is_kind(T_WHITESPACE) {
                space_indices.push(index);
            } else if tokens[index].is_comment() {
                tokens.clear_at(index);
            }
            index += 1;
        }
        if tokens[index - 1].is_whitespace(None) {
            space_indices.pop();
        }
        for i in space_indices {
            tokens.clear_at(i);
        }
        index
    }
}

impl Fixer for CleanNamespaceFixer {
    fn name(&self) -> &'static str {
        "clean_namespace"
    }
    fn priority(&self) -> i32 {
        // Must run before PhpUnitDataProviderReturnTypeFixer.
        10
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        PHP_VERSION_ID < 8_00_00 && tokens.is_token_kind_found(T_NS_SEPARATOR)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let count = tokens.count();
        let mut index = 0;
        while index < count {
            if tokens[index].is_kind(T_NS_SEPARATOR) {
                let previous_index = tokens.get_prev_meaningful_token(index).unwrap();
                index = Self::fix_namespace(tokens, if tokens[previous_index].is_kind(T_STRING) { previous_index } else { index });
            }
            index += 1;
        }
    }
}
