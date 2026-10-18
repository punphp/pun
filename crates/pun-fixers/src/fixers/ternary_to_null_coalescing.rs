//! Port of `PhpCsFixer\Fixer\Operator\TernaryToNullCoalescingFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct TernaryToNullCoalescingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(TernaryToNullCoalescingFixer)
}

impl Fixer for TernaryToNullCoalescingFixer {
    fn name(&self) -> &'static str {
        "ternary_to_null_coalescing"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_ISSET)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let isset_indices = tokens.find_given_kind(T_ISSET, 0, None);
        for &isset_index in isset_indices.iter().rev() {
            fix_isset(tokens, isset_index);
        }
    }
}

fn fix_isset(tokens: &mut Tokens, index: usize) {
    let prev_token_index = tokens.get_prev_meaningful_token(index).unwrap();
    if is_higher_precedence_associativity_operator(&tokens[prev_token_index]) {
        return;
    }
    let start_brace_index = tokens
        .get_next_token_of_kind(index, &["(".into()], true)
        .unwrap();
    let end_brace_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start_brace_index);
    // A `?>` or `;` always follows an `isset(...)` statement.
    let ternary_question_mark_index = tokens.get_next_meaningful_token(end_brace_index).unwrap();
    if !tokens[ternary_question_mark_index].equals("?", true) {
        return;
    }
    let isset_tokens = meaningful_sequence(tokens, start_brace_index, end_brace_index);
    if has_changing_content(&isset_tokens) {
        return;
    }
    let isset_code = code(&isset_tokens);
    if isset_code == b"$this" {
        return;
    }
    let ternary_colon_index = tokens
        .get_next_token_of_kind(ternary_question_mark_index, &[":".into()], true)
        .unwrap();
    let first_operand =
        meaningful_sequence(tokens, ternary_question_mark_index, ternary_colon_index);
    if isset_code != code(&first_operand) {
        return;
    }
    let ternary_first_operand_index = tokens
        .get_next_meaningful_token(ternary_question_mark_index)
        .unwrap();
    // preserve comments and spaces
    let mut comments = Vec::new();
    let mut comment_started = false;
    for loop_index in index..ternary_first_operand_index {
        if tokens[loop_index].is_comment() {
            comments.push(tokens[loop_index].clone());
            comment_started = true;
        } else if comment_started {
            if tokens[loop_index].is_whitespace(None) {
                comments.push(tokens[loop_index].clone());
            }
            comment_started = false;
        }
    }
    tokens.set(ternary_colon_index, Token::new(T_COALESCE, "??"));
    tokens.override_range(index, ternary_first_operand_index - 1, comments);
}

/// `getMeaningfulSequence`: the meaningful tokens strictly between `start` and `end`.
fn meaningful_sequence(tokens: &Tokens, start: usize, end: usize) -> Vec<Token> {
    let mut sequence = Vec::new();
    let mut index = start;
    while let Some(next) = tokens.get_next_meaningful_token(index) {
        if next >= end {
            break;
        }
        sequence.push(tokens[next].clone());
        index = next;
    }
    sequence
}

fn code(tokens: &[Token]) -> Vec<u8> {
    tokens
        .iter()
        .flat_map(|t| t.content.iter().copied())
        .collect()
}

fn is_higher_precedence_associativity_operator(token: &Token) -> bool {
    token.is_given_kind(&[
        T_ARRAY_CAST,
        T_BOOLEAN_AND,
        T_BOOLEAN_OR,
        T_BOOL_CAST,
        T_COALESCE,
        T_DEC,
        T_DOUBLE_CAST,
        T_INC,
        T_INT_CAST,
        T_IS_EQUAL,
        T_IS_GREATER_OR_EQUAL,
        T_IS_IDENTICAL,
        T_IS_NOT_EQUAL,
        T_IS_NOT_IDENTICAL,
        T_IS_SMALLER_OR_EQUAL,
        T_OBJECT_CAST,
        T_POW,
        T_SL,
        T_SPACESHIP,
        T_SR,
        T_STRING_CAST,
        T_UNSET_CAST,
    ]) || token.equals_any(
        &[
            "!".into(),
            "%".into(),
            "&".into(),
            "*".into(),
            "+".into(),
            "-".into(),
            "/".into(),
            ":".into(),
            "^".into(),
            "|".into(),
            "~".into(),
            ".".into(),
        ],
        true,
    )
}

/// `hasChangingContent`: the `isset()` content may change if evaluated twice.
fn has_changing_content(tokens: &[Token]) -> bool {
    tokens
        .iter()
        .any(|t| t.is_given_kind(&[T_DEC, T_INC, T_YIELD, T_YIELD_FROM]) || t.equals("(", true))
}
