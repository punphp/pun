//! Port of `PhpCsFixer\Fixer\FunctionNotation\NoSpacesAfterFunctionNameFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind, Kind::*, Tokens};
use std::path::Path;

struct NoSpacesAfterFunctionNameFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoSpacesAfterFunctionNameFixer)
}

const FUNCTIONY_TOKEN_KINDS: &[Kind] = &[
    T_ARRAY,
    T_ECHO,
    T_EMPTY,
    T_EVAL,
    T_EXIT,
    T_INCLUDE,
    T_INCLUDE_ONCE,
    T_ISSET,
    T_LIST,
    T_PRINT,
    T_REQUIRE,
    T_REQUIRE_ONCE,
    T_UNSET,
    T_VARIABLE,
];

impl NoSpacesAfterFunctionNameFixer {
    fn fix_function_call(tokens: &mut Tokens, index: usize) {
        if tokens[index - 1].is_whitespace(None) {
            tokens.clear_at(index - 1);
        }
    }
}

impl Fixer for NoSpacesAfterFunctionNameFixer {
    fn name(&self) -> &'static str {
        "no_spaces_after_function_name"
    }
    fn priority(&self) -> i32 {
        // Must run before FunctionToConstantFixer, GetClassToClassKeywordFixer.
        // Must run after PowToExponentiationFixer.
        3
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        let mut kinds = vec![T_STRING];
        kinds.extend_from_slice(FUNCTIONY_TOKEN_KINDS);
        tokens.is_any_token_kinds_found(&kinds)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].equals("(", true) {
                continue;
            }
            // last non-whitespace token, can never be `null` always at least PHP open tag before it
            let last_token_index = tokens.get_prev_non_whitespace(index, None).unwrap();
            let end_parenthesis_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
            let next_non_white_space = tokens.get_next_meaningful_token(end_parenthesis_index);
            // (`)` is never the last meaningful token of code that parses)
            if let Some(n) = next_non_white_space
                && !tokens[n].equals(";", true)
                && tokens[last_token_index].is_given_kind(&[
                    T_ECHO,
                    T_PRINT,
                    T_INCLUDE,
                    T_INCLUDE_ONCE,
                    T_REQUIRE,
                    T_REQUIRE_ONCE,
                ])
            {
                continue;
            }
            if tokens[last_token_index].is_given_kind(FUNCTIONY_TOKEN_KINDS) {
                Self::fix_function_call(tokens, index);
            } else if tokens[last_token_index].is_kind(T_STRING) {
                let possible_definition_index =
                    tokens.get_prev_meaningful_token(last_token_index).unwrap();
                if !tokens[possible_definition_index].is_kind(T_FUNCTION) {
                    Self::fix_function_call(tokens, index);
                }
            } else if tokens[last_token_index].equals_any(
                &[
                    ")".into(),
                    "]".into(),
                    CT_DYNAMIC_VAR_BRACE_CLOSE.into(),
                    CT_ARRAY_INDEX_BRACE_CLOSE.into(),
                ],
                true,
            ) {
                let block = Tokens::detect_block_type(&tokens[last_token_index]).map(|(t, _)| t);
                if [
                    Tokens::BLOCK_TYPE_INDEX_BRACE,
                    Tokens::BLOCK_TYPE_DYNAMIC_VAR_BRACE,
                    Tokens::BLOCK_TYPE_INDEX_BRACKET,
                    Tokens::BLOCK_TYPE_PARENTHESIS,
                ]
                .iter()
                .any(|&t| Some(t) == block)
                {
                    Self::fix_function_call(tokens, index);
                }
            }
        }
    }
}
