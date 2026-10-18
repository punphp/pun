//! Port of `PhpCsFixer\Fixer\Operator\LongToShorthandOperatorFixer`.
use crate::analyzers::abstract_short_operator_fixer::{ShortOperator, apply_fix};
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::{Proto, Token, Tokens, TokensAnalyzer};
use std::path::Path;

struct LongToShorthandOperatorFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(LongToShorthandOperatorFixer)
}

/// `OPERATORS`: operator => shorthand token.
const OPERATORS: &[(&str, Kind, &str)] = &[
    ("+", T_PLUS_EQUAL, "+="),
    ("-", T_MINUS_EQUAL, "-="),
    ("*", T_MUL_EQUAL, "*="),
    ("/", T_DIV_EQUAL, "/="),
    ("&", T_AND_EQUAL, "&="),
    (".", T_CONCAT_EQUAL, ".="),
    ("%", T_MOD_EQUAL, "%="),
    ("|", T_OR_EQUAL, "|="),
    ("^", T_XOR_EQUAL, "^="),
];

impl ShortOperator for LongToShorthandOperatorFixer {
    fn is_operator_token_candidate(&self, tokens: &Tokens, index: usize) -> bool {
        let operator_types: Vec<Proto<'_>> = OPERATORS.iter().map(|&(o, _, _)| o.into()).collect();
        if !tokens[index].equals_any(&operator_types, true) {
            return false;
        }
        let analyzer = TokensAnalyzer::new(tokens);
        let mut index = index;
        loop {
            // A statement always ends in `;` or `?>`; upstream's `$tokens[null]` throws otherwise.
            index = tokens.get_next_meaningful_token(index).unwrap();
            let other_token = &tokens[index];
            if other_token.equals_any(&[";".into(), T_CLOSE_TAG.into()], true) {
                return true;
            }
            // fast precedence check
            if other_token.equals("?", true) || other_token.is_kind(T_INSTANCEOF) {
                return false;
            }
            if let Some((block_type, is_start)) = Tokens::detect_block_type(other_token) {
                if !is_start {
                    return true;
                }
                index = tokens.find_block_end(block_type, index);
                continue;
            }
            // precedence check
            if analyzer.is_binary_operator(index) {
                return false;
            }
        }
    }
    fn get_replacement_token(&self, token: &Token) -> Token {
        let &(_, kind, content) = OPERATORS
            .iter()
            .find(|(o, _, _)| token.equals(*o, true))
            .unwrap();
        Token::new(kind, content)
    }
}

impl Fixer for LongToShorthandOperatorFixer {
    fn name(&self) -> &'static str {
        "long_to_shorthand_operator"
    }
    fn priority(&self) -> i32 {
        17
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        OPERATORS
            .iter()
            .any(|&(o, _, _)| tokens.is_token_kind_found(o))
            || tokens.is_any_token_kinds_found(&[
                T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG,
                T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG,
            ])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(self, tokens);
    }
}
