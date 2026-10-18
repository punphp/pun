//! Port of `PhpCsFixer\Fixer\StringNotation\ExplicitStringVariableFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

struct ExplicitStringVariableFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ExplicitStringVariableFixer)
}

/// One `$variableTokens` entry: the indices of its tokens (first to last, in order).
struct VariableSet {
    tokens: Vec<usize>,
}

impl Fixer for ExplicitStringVariableFixer {
    fn name(&self) -> &'static str {
        "explicit_string_variable"
    }
    fn priority(&self) -> i32 {
        // Must run before NoUselessConcatOperatorFixer.
        // Must run after BacktickToShellExecFixer.
        6
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_VARIABLE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut backtick_started = false;
        let mut index = tokens.count() - 1;
        while index > 0 {
            let token = &tokens[index];
            if token.equals("`", true) {
                backtick_started = !backtick_started;
            } else if !backtick_started
                && token.is_kind(T_VARIABLE)
                && is_string_part_token(&tokens[index - 1])
            {
                fix_variables(tokens, index);
            }
            index -= 1;
        }
    }
}

fn fix_variables(tokens: &mut Tokens, index: usize) {
    let mut distinct_variable_index = index;
    // krsort($variableTokens): a BTreeMap walked in reverse.
    let mut variable_tokens: BTreeMap<usize, VariableSet> = BTreeMap::new();
    variable_tokens.insert(index, VariableSet { tokens: vec![index] });
    let mut next_index = index + 1;
    let mut square_bracket_count = 0;
    while !is_string_part_token(&tokens[next_index]) {
        if tokens[next_index].is_kind(T_CURLY_OPEN) {
            next_index = tokens
                .get_next_token_of_kind(next_index, &[CT_CURLY_CLOSE.into()], true)
                .unwrap();
        } else if tokens[next_index].is_kind(T_VARIABLE) && square_bracket_count != 1 {
            distinct_variable_index = next_index;
            variable_tokens.insert(
                distinct_variable_index,
                VariableSet {
                    tokens: vec![next_index],
                },
            );
        } else {
            variable_tokens
                .get_mut(&distinct_variable_index)
                .unwrap()
                .tokens
                .push(next_index);
            if tokens[next_index].equals_any(&["[".into(), "]".into()], true) {
                square_bracket_count += 1;
            }
        }
        next_index += 1;
    }

    for set in variable_tokens.into_values().rev() {
        if let [single_variable_index] = set.tokens[..] {
            let content = tokens[single_variable_index].content.clone();
            tokens.override_range(
                single_variable_index,
                single_variable_index,
                vec![
                    Token::new(T_CURLY_OPEN, "{"),
                    Token::new(T_VARIABLE, content),
                    Token::new(CT_CURLY_CLOSE, "}"),
                ],
            );
            continue;
        }
        for &variable_part_index in &set.tokens {
            let part = &tokens[variable_part_index];
            if part.is_kind(T_NUM_STRING) {
                let content = part.content.clone();
                tokens.set(variable_part_index, Token::new(T_LNUMBER, content));
            } else if part.is_kind(T_STRING) && tokens[variable_part_index + 1].equals("]", true) {
                let content = [b"'", &part.content[..], b"'"].concat();
                tokens.set(
                    variable_part_index,
                    Token::new(T_CONSTANT_ENCAPSED_STRING, content),
                );
            }
        }
        let first = set.tokens[0];
        let last = *set.tokens.last().unwrap();
        tokens.insert_at(last + 1, vec![Token::new(CT_CURLY_CLOSE, "}")]);
        tokens.insert_at(first, vec![Token::new(T_CURLY_OPEN, "{")]);
    }
}

/// Check if token is a part of a string.
fn is_string_part_token(token: &Token) -> bool {
    token.is_kind(T_ENCAPSED_AND_WHITESPACE)
        || token.is_kind(T_START_HEREDOC)
        || token.content == b"\""
        || strtolower(&token.content) == b"b\""
}
