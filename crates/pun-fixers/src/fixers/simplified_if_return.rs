//! Port of `PhpCsFixer\Fixer\ControlStructure\SimplifiedIfReturnFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

struct SimplifiedIfReturnFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SimplifiedIfReturnFixer)
}

const OPEN: Proto<'static> = Proto::Char(b"{");
const CLOSE: Proto<'static> = Proto::Char(b"}");
const SEMI: Proto<'static> = Proto::Char(b";");
const RETURN: Proto<'static> = Proto::Kind(T_RETURN);
const TRUE: Proto<'static> = Proto::KindContent(T_STRING, b"true");
const FALSE: Proto<'static> = Proto::KindContent(T_STRING, b"false");

/// `$sequences`: (isNegative, sequence).
const SEQUENCES: &[(bool, &[Proto<'static>])] = &[
    (
        false,
        &[OPEN, RETURN, TRUE, SEMI, CLOSE, RETURN, FALSE, SEMI],
    ),
    (
        true,
        &[OPEN, RETURN, FALSE, SEMI, CLOSE, RETURN, TRUE, SEMI],
    ),
    (false, &[RETURN, TRUE, SEMI, RETURN, FALSE, SEMI]),
    (true, &[RETURN, FALSE, SEMI, RETURN, TRUE, SEMI]),
];

impl Fixer for SimplifiedIfReturnFixer {
    fn name(&self) -> &'static str {
        "simplified_if_return"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_IF, T_RETURN, T_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut slices = BTreeMap::new();
        let mut next_if_index = None;
        for if_index in (0..tokens.count()).rev() {
            if !tokens[if_index].is_given_kind(&[T_IF, T_ELSEIF]) {
                continue;
            }
            let prev = tokens.get_prev_meaningful_token(if_index).unwrap();
            if tokens[prev].equals(")", true) {
                continue; // in a loop without braces
            }
            let start_parenthesis_index = tokens
                .get_next_token_of_kind(if_index, &["(".into()], true)
                .unwrap();
            let end_parenthesis_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start_parenthesis_index);
            let first_candidate_index = tokens
                .get_next_meaningful_token(end_parenthesis_index)
                .unwrap();
            for &(is_negative, sequence) in SEQUENCES {
                let Some(mut indices_to_clear) =
                    tokens.find_sequence(sequence, first_candidate_index, next_if_index, |_| true)
                else {
                    continue;
                };
                if indices_to_clear[0] != first_candidate_index {
                    continue;
                }
                indices_to_clear.pop(); // Preserve last semicolon
                for &index in indices_to_clear.iter().rev() {
                    tokens.clear_token_and_merge_surrounding_whitespace(index);
                }
                let mut new_tokens = vec![
                    Token::new(T_RETURN, "return"),
                    Token::new(T_WHITESPACE, " "),
                ];
                if is_negative {
                    new_tokens.push(Token::char("!"));
                } else {
                    new_tokens.push(Token::new(T_BOOL_CAST, "(bool)"));
                }
                slices.insert(if_index, new_tokens);
                tokens.clear_at(if_index);
                // Limit the search to this branch by stopping at the next if/elseif.
                next_if_index = Some(if_index);
            }
        }
        if !slices.is_empty() {
            tokens.insert_slices(slices);
        }
    }
}
