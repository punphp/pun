//! Port of `PhpCsFixer\Fixer\ControlStructure\TrailingCommaInMultilineFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

const ELEMENTS: &[&str] = &["array_destructuring", "arguments", "arrays", "parameters", "match"];

struct TrailingCommaInMultilineFixer {
    after_heredoc: bool,
    elements: Vec<String>,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(TrailingCommaInMultilineFixer { after_heredoc: false, elements: vec!["arrays".into()] })
}

impl Fixer for TrailingCommaInMultilineFixer {
    fn name(&self) -> &'static str {
        "trailing_comma_in_multiline"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN, CT_DESTRUCTURING_BRACKET_OPEN])
            || tokens.is_token_kind_found("(")
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["after_heredoc", "elements"])?;
        // Future::getV4OrV3(true, false)
        self.after_heredoc = o.bool("after_heredoc", false)?;
        self.elements = o.strings("elements", &["arrays"], ELEMENTS)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let has = |e: &str| self.elements.iter().any(|x| x == e);
        let fix_arrays = has("arrays");
        let fix_arguments = has("arguments");
        let fix_parameters = has("parameters");
        let fix_match = has("match");
        let fix_destructuring = has("array_destructuring");

        for index in (0..tokens.count()).rev() {
            if tokens[index].is_kind(CT_DESTRUCTURING_BRACKET_OPEN) {
                if fix_destructuring {
                    // array destructing short syntax
                    self.fix_block(tokens, index);
                }
                continue;
            }
            if tokens[index].is_kind(CT_ARRAY_BRACKET_OPEN) {
                if fix_arrays {
                    // array short syntax
                    self.fix_block(tokens, index);
                }
                continue;
            }
            if !tokens[index].equals("(", true) {
                continue;
            }
            let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[prev_index].is_kind(T_ARRAY) {
                if fix_arrays {
                    // array long syntax
                    self.fix_block(tokens, index);
                }
                continue;
            }
            if tokens[prev_index].is_kind(T_LIST) {
                if fix_destructuring || fix_arguments {
                    // array destructing long syntax
                    self.fix_block(tokens, index);
                }
                continue;
            }
            if fix_match && tokens[prev_index].is_kind(T_MATCH) {
                let brace = tokens.get_next_token_of_kind(index, &["{".into()], true).unwrap();
                self.fix_block(tokens, brace);
                continue;
            }
            let prev_prev_index = tokens.get_prev_meaningful_token(prev_index);
            let prev_prev_is_function = prev_prev_index.is_some_and(|p| tokens[p].is_kind(T_FUNCTION));
            if fix_arguments
                && tokens[prev_index].equals_any(
                    &["]".into(), T_CLASS.into(), T_STRING.into(), T_VARIABLE.into(), T_STATIC.into(), T_ISSET.into(), T_UNSET.into(), T_LIST.into()],
                    true,
                )
                && !prev_prev_is_function
            {
                self.fix_block(tokens, index);
                continue;
            }
            if fix_parameters
                && ((tokens[prev_index].is_kind(T_STRING) && prev_prev_is_function)
                    || tokens[prev_index].is_given_kind(&[T_FN, T_FUNCTION]))
            {
                self.fix_block(tokens, index);
            }
        }
    }
}

impl TrailingCommaInMultilineFixer {
    fn fix_block(&self, tokens: &mut Tokens, start_index: usize) {
        if !TokensAnalyzer::new(tokens).is_block_multiline(start_index) {
            return;
        }
        let (block_type, _) = Tokens::detect_block_type(&tokens[start_index]).unwrap();
        let end_index = tokens.find_block_end(block_type, start_index);
        let before_end_index = tokens.get_prev_meaningful_token(end_index).unwrap();
        if !tokens.is_partial_code_multiline(before_end_index, end_index) {
            return;
        }
        let before_end_token = &tokens[before_end_index];
        // if there is some item between braces then add `,` after it
        if start_index != before_end_index
            && !before_end_token.equals_any(&[",".into(), CT_FIRST_CLASS_CALLABLE.into()], true)
            && (self.after_heredoc || !before_end_token.is_kind(T_END_HEREDOC))
        {
            tokens.insert_at(before_end_index + 1, vec![Token::char(",")]);
            let end_token = &tokens[end_index];
            if !end_token.is_comment() && !end_token.is_whitespace(None) {
                tokens.ensure_whitespace_at_index(end_index, 1, b" ");
            }
        }
    }
}
