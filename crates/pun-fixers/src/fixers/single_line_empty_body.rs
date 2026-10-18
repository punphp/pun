//! Port of `PhpCsFixer\Fixer\Basic\SingleLineEmptyBodyFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

struct SingleLineEmptyBodyFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(SingleLineEmptyBodyFixer)
}

impl Fixer for SingleLineEmptyBodyFixer {
    fn name(&self) -> &'static str {
        "single_line_empty_body"
    }
    fn priority(&self) -> i32 {
        -19
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_INTERFACE, T_CLASS, T_FUNCTION, T_TRAIT, T_ENUM])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (1..tokens.count()).rev() {
            if !tokens[index].is_given_kind(&[T_CLASS, T_TRAIT, T_INTERFACE, T_ENUM, T_FUNCTION]) {
                continue;
            }
            let open_brace_index = tokens
                .get_next_token_of_kind(index, &["{".into(), ";".into()], true)
                .unwrap();
            if !tokens[open_brace_index].equals("{", true) {
                continue;
            }
            let close_brace_index = tokens
                .get_next_non_whitespace(open_brace_index, None)
                .unwrap();
            if !tokens[close_brace_index].equals("}", true) {
                continue;
            }
            tokens.ensure_whitespace_at_index(open_brace_index + 1, 0, b"");
            let before_open_brace_index = tokens
                .get_prev_non_whitespace(open_brace_index, None)
                .unwrap();
            if !tokens[before_open_brace_index].is_given_kind(&[T_COMMENT, T_DOC_COMMENT]) {
                tokens.ensure_whitespace_at_index(open_brace_index - 1, 1, b" ");
            }
        }
    }
}
