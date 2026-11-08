//! Port of `PhpCsFixer\Fixer\ReturnNotation\SimplifiedNullReturnFixer`.
use pun_format::Fixer;
use pun_tokens::php_compat::{ltrim_chars, rtrim_chars, strtolower};
use pun_tokens::{Kind::*, Proto, Tokens};
use std::path::Path;

const STATEMENT_END: &[Proto<'static>] = &[Proto::Char(b";"), Proto::Kind(T_CLOSE_TAG)];

struct SimplifiedNullReturnFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(SimplifiedNullReturnFixer)
}

impl Fixer for SimplifiedNullReturnFixer {
    fn name(&self) -> &'static str {
        "simplified_null_return"
    }
    fn priority(&self) -> i32 {
        16
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_RETURN)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let end = tokens.count() - 1;
        fix_range(tokens, 1, end);
    }
}

fn fix_range(tokens: &mut Tokens, start_index: usize, end_index: usize) {
    let mut index = start_index;
    while index < end_index {
        if tokens[index].is_kind(CT_PROPERTY_HOOK_BRACE_OPEN) {
            let property_hook_close_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PROPERTY_HOOK, index);
            scan_range(tokens, index, property_hook_close_index);
            index = property_hook_close_index + 1;
            continue;
        }
        if tokens[index].is_kind(T_RETURN) && need_fixing(tokens, index) {
            clear(tokens, index);
        }
        index += 1;
    }
}

fn scan_range(tokens: &mut Tokens, start_index: usize, end_index: usize) {
    let mut index = start_index;
    while index < end_index {
        if tokens[index].is_kind(T_FUNCTION) {
            let brace_open_index = tokens
                .get_next_token_of_kind(index, &["{".into()], true)
                .unwrap();
            let brace_close_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, brace_open_index);
            fix_range(tokens, brace_open_index, brace_close_index);
            index = brace_close_index;
        }
        index += 1;
    }
}

fn clear(tokens: &mut Tokens, mut index: usize) {
    loop {
        index += 1;
        if tokens[index].equals_any(STATEMENT_END, true) {
            break;
        }
        if should_clear_token(tokens, index) {
            tokens.clear_at(index);
        }
    }
}

fn need_fixing(tokens: &Tokens, mut index: usize) -> bool {
    if is_strict_or_nullable_return_type_function(tokens, index) {
        return false;
    }
    let mut content = Vec::new();
    while !tokens[index].equals_any(STATEMENT_END, true) {
        index = tokens.get_next_meaningful_token(index).unwrap();
        content.extend_from_slice(tokens[index].get_content());
    }
    let last_token_content = tokens[index].get_content();
    let content = &content[..content.len() - last_token_content.len()];
    let content = rtrim_chars(ltrim_chars(content, b"("), b")");
    strtolower(content) == b"null"
}

fn is_strict_or_nullable_return_type_function(tokens: &Tokens, return_index: usize) -> bool {
    let mut function_index = return_index;
    let opening_curly_brace_index = loop {
        let Some(found) = tokens.get_prev_token_of_kind(function_index, &[T_FUNCTION.into()], true)
        else {
            return false;
        };
        function_index = found;
        let opening = tokens
            .get_next_token_of_kind(function_index, &["{".into()], true)
            .unwrap();
        let closing = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, opening);
        if closing >= return_index {
            break opening;
        }
    };
    let possible_void_index = tokens
        .get_prev_meaningful_token(opening_curly_brace_index)
        .unwrap();
    let is_strict_return_type = tokens[possible_void_index]
        .is_given_kind(&[T_STRING, CT_ARRAY_TYPEHINT])
        && tokens[possible_void_index].get_content() != b"void";
    let nullable_type_index =
        tokens.get_next_token_of_kind(function_index, &[CT_NULLABLE_TYPE.into()], true);
    let is_nullable_return_type =
        nullable_type_index.is_some_and(|i| i < opening_curly_brace_index);
    is_strict_return_type || is_nullable_return_type
}

fn should_clear_token(tokens: &Tokens, index: usize) -> bool {
    let token = &tokens[index];
    if token.is_comment() {
        return false;
    }
    if !token.is_whitespace(None) {
        return true;
    }
    !(tokens[index + 1].is_comment()
        || tokens[index + 1].is_kind(T_CLOSE_TAG)
        || (tokens[index - 1].is_comment() && tokens[index + 1].equals(";", true)))
}
