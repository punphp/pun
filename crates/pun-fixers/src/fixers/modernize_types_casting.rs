//! Port of `PhpCsFixer\Fixer\CastNotation\ModernizeTypesCastingFixer`.
use crate::analyzers::abstract_function_reference_fixer::{
    FunctionReferenceFinder, IS_RISKY, is_candidate,
};
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct ModernizeTypesCastingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ModernizeTypesCastingFixer)
}

impl Fixer for ModernizeTypesCastingFixer {
    fn name(&self) -> &'static str {
        "modernize_types_casting"
    }
    fn priority(&self) -> i32 {
        31
    }
    fn is_risky(&self) -> bool {
        IS_RISKY
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let finder = FunctionReferenceFinder::new();
        let arguments_analyzer = ArgumentsAnalyzer;
        for (function_identity, new_token) in [
            ("intval", Token::new(T_INT_CAST, "(int)")),
            ("floatval", Token::new(T_DOUBLE_CAST, "(float)")),
            ("doubleval", Token::new(T_DOUBLE_CAST, "(float)")),
            ("strval", Token::new(T_STRING_CAST, "(string)")),
            ("boolval", Token::new(T_BOOL_CAST, "(bool)")),
        ] {
            let mut curr_index = 0;
            // try getting function reference and translate boundaries for humans
            while let Some((function_name, open_parenthesis, close_parenthesis)) = finder.find(
                function_identity,
                tokens,
                curr_index,
                Some(tokens.count() - 1),
            ) {
                // analysing cursor shift
                curr_index = open_parenthesis;
                // indicator that the function is overridden
                if arguments_analyzer.count_arguments(tokens, open_parenthesis, close_parenthesis)
                    != 1
                {
                    continue;
                }
                let mut param_content_end = close_parenthesis;
                let comma_candidate = tokens.get_prev_meaningful_token(param_content_end).unwrap();
                if tokens[comma_candidate].equals(",", true) {
                    tokens.remove_trailing_whitespace(comma_candidate, None);
                    tokens.clear_at(comma_candidate);
                    param_content_end = comma_candidate;
                }
                // check if something complex passed as an argument and preserve parentheses then
                let count_param_tokens = (open_parenthesis + 1..param_content_end)
                    .filter(|&i| !tokens[i].is_kind(T_WHITESPACE))
                    .count();
                let preserve_parentheses = count_param_tokens > 1;
                let after_close_parenthesis_token =
                    &tokens[tokens.get_next_meaningful_token(close_parenthesis).unwrap()];
                let wrap_in_parentheses = after_close_parenthesis_token
                    .equals_any(&["[".into(), "{".into()], true)
                    || after_close_parenthesis_token.is_kind(T_POW);
                // analyse namespace specification (root one or none) and decide what to do
                let prev_token_index = tokens.get_prev_meaningful_token(function_name).unwrap();
                if tokens[prev_token_index].is_kind(T_NS_SEPARATOR) {
                    // get rid of root namespace when it used
                    tokens.remove_trailing_whitespace(prev_token_index, None);
                    tokens.clear_at(prev_token_index);
                }
                // perform transformation
                let mut replacement_sequence =
                    vec![new_token.clone(), Token::new(T_WHITESPACE, " ")];
                if wrap_in_parentheses {
                    replacement_sequence.insert(0, Token::char("("));
                }
                if !preserve_parentheses {
                    // closing parenthesis removed with leading spaces
                    tokens.remove_leading_whitespace(close_parenthesis, None);
                    tokens.clear_at(close_parenthesis);
                    // opening parenthesis removed with trailing spaces
                    tokens.remove_leading_whitespace(open_parenthesis, None);
                    tokens.remove_trailing_whitespace(open_parenthesis, None);
                    tokens.clear_at(open_parenthesis);
                } else {
                    // we'll need to provide a space after a casting operator
                    tokens.remove_trailing_whitespace(function_name, None);
                }
                if wrap_in_parentheses {
                    tokens.insert_at(close_parenthesis, vec![Token::char(")")]);
                }
                tokens.override_range(function_name, function_name, replacement_sequence);
                // nested transformations support
                curr_index = function_name;
            }
        }
    }
}
