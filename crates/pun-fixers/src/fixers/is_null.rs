//! Port of `PhpCsFixer\Fixer\LanguageConstruct\IsNullFixer`.
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

struct IsNullFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(IsNullFixer)
}

impl Fixer for IsNullFixer {
    fn name(&self) -> &'static str {
        "is_null"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let mut curr_index = 0;
        // recalculate "end" because we might have added tokens in previous iteration
        while let Some(matches) = tokens.find_sequence(
            &[(T_STRING, "is_null").into(), "(".into()],
            curr_index,
            Some(tokens.count() - 1),
            |_| false,
        ) {
            // move the cursor just after the sequence
            let (is_null_index, open) = (matches[0], matches[1]);
            curr_index = open;
            if !functions_analyzer.is_global_function_call(tokens, is_null_index) {
                continue;
            }
            let next = tokens.get_next_meaningful_token(curr_index).unwrap();
            if tokens[next].equals(")", true) {
                continue;
            }
            // an unpacked argument is not a value that can be compared to `null`, so leave such call as is
            if tokens[next].is_kind(T_ELLIPSIS) {
                continue;
            }
            let reference_end = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
            // `is_null()` takes a single parameter, so only the first token inside the parentheses can carry a name
            if let Some(colon) = tokens.get_next_meaningful_token(next)
                && tokens[colon].is_kind(CT_NAMED_ARGUMENT_COLON)
            {
                // a name other than the one `is_null()` declares (`value`) does not resolve to its parameter
                if !tokens[next].equals((CT_NAMED_ARGUMENT_NAME, "value"), true) {
                    continue;
                }
                // the name is redundant for a single-parameter call, drop it to not break the transformation
                tokens.clear_token_and_merge_surrounding_whitespace(colon);
                tokens.clear_token_and_merge_surrounding_whitespace(next);
                tokens.remove_trailing_whitespace(open, None);
            }
            let mut prev_token_index = tokens.get_prev_meaningful_token(is_null_index).unwrap();
            // handle function references with namespaces
            if tokens[prev_token_index].is_kind(T_NS_SEPARATOR) {
                tokens.remove_trailing_whitespace(prev_token_index, None);
                tokens.clear_at(prev_token_index);
                prev_token_index = tokens.get_prev_meaningful_token(prev_token_index).unwrap();
            }
            // check if inversion being used, text comparison is due to not existing constant
            let mut is_inverted_null_check = false;
            if tokens[prev_token_index].equals("!", true) {
                is_inverted_null_check = true;
                // get rid of inverting for proper transformations
                tokens.remove_trailing_whitespace(prev_token_index, None);
                tokens.clear_at(prev_token_index);
            }
            // before getting rind of `()` around a parameter, ensure it's not assignment/ternary invariant
            let is_containing_dangerous_constructs = (open..=reference_end)
                .any(|i| [&b"?"[..], b"?:", b"=", b"??"].contains(&tokens[i].get_content()));
            // edge cases: is_null() followed/preceded by ==, ===, !=, !==, <>, (int-or-other-casting)
            let parent_left_token =
                &tokens[tokens.get_prev_meaningful_token(is_null_index).unwrap()];
            let parent_right_token =
                &tokens[tokens.get_next_meaningful_token(reference_end).unwrap()];
            let parent_operations = &[
                T_IS_EQUAL,
                T_IS_NOT_EQUAL,
                T_IS_IDENTICAL,
                T_IS_NOT_IDENTICAL,
            ];
            let wrap_into_parentheses = parent_left_token.is_cast()
                || parent_left_token.is_given_kind(parent_operations)
                || parent_right_token.is_given_kind(parent_operations);
            // possible trailing comma removed
            let prev_index = tokens.get_prev_meaningful_token(reference_end).unwrap();
            if tokens[prev_index].equals(",", true) {
                tokens.clear_token_and_merge_surrounding_whitespace(prev_index);
            }
            if !is_containing_dangerous_constructs {
                // closing parenthesis removed with leading spaces
                tokens.remove_leading_whitespace(reference_end, None);
                tokens.clear_at(reference_end);
                // opening parenthesis removed with trailing spaces
                tokens.remove_leading_whitespace(open, None);
                tokens.remove_trailing_whitespace(open, None);
                tokens.clear_at(open);
            }
            // sequence which we'll use as a replacement
            let mut replacement = vec![
                Token::new(T_STRING, "null"),
                Token::new(T_WHITESPACE, " "),
                if is_inverted_null_check {
                    Token::new(T_IS_NOT_IDENTICAL, "!==")
                } else {
                    Token::new(T_IS_IDENTICAL, "===")
                },
                Token::new(T_WHITESPACE, " "),
            ];
            if wrap_into_parentheses {
                replacement.insert(0, Token::char("("));
                tokens.insert_at(reference_end + 1, vec![Token::char(")")]);
            }
            tokens.override_range(is_null_index, is_null_index, replacement);
            // nested is_null calls support
            curr_index = is_null_index;
        }
    }
}
