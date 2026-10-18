//! Port of `PhpCsFixer\Fixer\FunctionNotation\FunctionDeclarationFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct FunctionDeclarationFixer {
    closure_function_spacing_none: bool,
    closure_fn_spacing_none: bool,
    trailing_comma_single_line: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(FunctionDeclarationFixer::default())
}

const SINGLE_LINE_WHITESPACE_OPTIONS: &[u8] = b" \t";
const SUPPORTED_SPACINGS: &[&str] = &["none", "one"];

impl Fixer for FunctionDeclarationFixer {
    fn name(&self) -> &'static str {
        "function_declaration"
    }
    fn priority(&self) -> i32 {
        31
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(
            self.name(),
            options,
            &[
                "closure_function_spacing",
                "closure_fn_spacing",
                "trailing_comma_single_line",
            ],
        )?;
        self.closure_function_spacing_none =
            c.string("closure_function_spacing", "one", SUPPORTED_SPACINGS)? == "none";
        // Future::getV4OrV3('none', 'one'): v3 default.
        self.closure_fn_spacing_none =
            c.string("closure_fn_spacing", "one", SUPPORTED_SPACINGS)? == "none";
        self.trailing_comma_single_line = c.bool("trailing_comma_single_line", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_FUNCTION, T_FN])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_given_kind(&[T_FUNCTION, T_FN]) {
                continue;
            }
            let token = tokens[index].clone();
            // Upstream also stops at `;`/`?>` and skips; in valid code a T_FUNCTION/T_FN always reaches its `(` first
            // (`use function` is CT_FUNCTION_IMPORT).
            let start_parenthesis_index = tokens
                .get_next_token_of_kind(index, &["(".into()], true)
                .unwrap();
            let end_parenthesis_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start_parenthesis_index);
            if !self.trailing_comma_single_line
                && !tokens.is_partial_code_multiline(index, end_parenthesis_index)
            {
                let comma_index = tokens
                    .get_prev_meaningful_token(end_parenthesis_index)
                    .unwrap();
                if tokens[comma_index].equals(",", true) {
                    tokens.clear_token_and_merge_surrounding_whitespace(comma_index);
                }
            }
            let start_brace_index = tokens
                .get_next_token_of_kind(
                    end_parenthesis_index,
                    &[";".into(), "{".into(), T_DOUBLE_ARROW.into()],
                    true,
                )
                .unwrap();
            if tokens[start_brace_index].equals_any(&["{".into(), T_DOUBLE_ARROW.into()], true)
                && (!tokens[start_brace_index - 1].is_whitespace(None)
                    || tokens[start_brace_index - 1]
                        .is_whitespace(Some(SINGLE_LINE_WHITESPACE_OPTIONS)))
            {
                tokens.ensure_whitespace_at_index(start_brace_index - 1, 1, b" ");
            }
            let after_parenthesis_index = tokens
                .get_next_non_whitespace(end_parenthesis_index, None)
                .unwrap();
            if tokens[after_parenthesis_index].is_kind(CT_USE_LAMBDA) {
                // fix whitespace after CT:T_USE_LAMBDA (we might add a token, so do this before determining start and end parenthesis)
                tokens.ensure_whitespace_at_index(after_parenthesis_index + 1, 0, b" ");
                let use_start_parenthesis_index = tokens
                    .get_next_token_of_kind(after_parenthesis_index, &["(".into()], true)
                    .unwrap();
                let use_end_parenthesis_index = tokens
                    .find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, use_start_parenthesis_index);
                if !self.trailing_comma_single_line
                    && !tokens.is_partial_code_multiline(index, use_end_parenthesis_index)
                {
                    let comma_index = tokens
                        .get_prev_meaningful_token(use_end_parenthesis_index)
                        .unwrap();
                    if tokens[comma_index].equals(",", true) {
                        tokens.clear_token_and_merge_surrounding_whitespace(comma_index);
                    }
                }
                fix_parenthesis_inner_edge(
                    tokens,
                    use_start_parenthesis_index,
                    use_end_parenthesis_index,
                );
                tokens.ensure_whitespace_at_index(after_parenthesis_index - 1, 1, b" ");
            }
            fix_parenthesis_inner_edge(tokens, start_parenthesis_index, end_parenthesis_index);
            let is_lambda = TokensAnalyzer::new(tokens).is_lambda(index);
            if !is_lambda
                && tokens[start_parenthesis_index - 1].is_whitespace(None)
                && !tokens[tokens
                    .get_prev_non_whitespace(start_parenthesis_index - 1, None)
                    .unwrap()]
                .is_comment()
            {
                tokens.clear_at(start_parenthesis_index - 1);
            }
            let spacing_none = if token.is_kind(T_FN) {
                self.closure_fn_spacing_none
            } else {
                self.closure_function_spacing_none
            };
            if is_lambda && spacing_none {
                if tokens[index + 1].is_whitespace(None) {
                    tokens.clear_at(index + 1);
                }
            } else {
                tokens.ensure_whitespace_at_index(index + 1, 0, b" ");
            }
            if is_lambda {
                let prev = tokens.get_prev_meaningful_token(index).unwrap();
                if tokens[prev].is_kind(T_STATIC) {
                    tokens.ensure_whitespace_at_index(prev + 1, 0, b" ");
                }
            }
        }
    }
}

fn fix_parenthesis_inner_edge(tokens: &mut Tokens, start: usize, mut end: usize) {
    loop {
        end -= 1;
        if !tokens.is_empty_at(end) {
            break;
        }
    }
    if tokens[end].is_whitespace(Some(SINGLE_LINE_WHITESPACE_OPTIONS)) {
        tokens.clear_at(end);
    }
    if tokens[start + 1].is_whitespace(Some(SINGLE_LINE_WHITESPACE_OPTIONS)) {
        tokens.clear_at(start + 1);
    }
}
