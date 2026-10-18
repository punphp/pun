//! Port of `PhpCsFixer\Fixer\FunctionNotation\NoUnreachableDefaultArgumentValueFixer`.
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use std::path::Path;

struct NoUnreachableDefaultArgumentValueFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUnreachableDefaultArgumentValueFixer)
}

impl Fixer for NoUnreachableDefaultArgumentValueFixer {
    fn name(&self) -> &'static str {
        "no_unreachable_default_argument_value"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_FUNCTION, T_FN])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let l = tokens.count();
        for i in 0..l {
            if !tokens[i].is_given_kind(&[T_FUNCTION, T_FN]) {
                continue;
            }
            fix_function_definition(tokens, i);
        }
    }
}

fn fix_function_definition(tokens: &mut Tokens, start_index: usize) {
    let mut remove_default_argument = false;
    let arguments = FunctionsAnalyzer::new().get_function_arguments(tokens, start_index);
    for (_, argument_analysis) in arguments.into_iter().rev() {
        let name_index = argument_analysis.name_index.unwrap();
        let prev_variable_index = tokens.get_prev_meaningful_token(name_index).unwrap();
        if tokens[prev_variable_index].is_kind(T_ELLIPSIS) {
            continue;
        }
        let Some(default) = &argument_analysis.default else {
            remove_default_argument = true;
            continue;
        };
        if !remove_default_argument {
            continue;
        }
        if default.eq_ignore_ascii_case(b"null")
            && argument_analysis
                .type_analysis
                .as_ref()
                .is_some_and(|t| !t.is_nullable())
        {
            continue;
        }
        let end = get_default_value_end(tokens, name_index);
        remove_default_value(tokens, name_index, end);
    }
}

fn remove_default_value(tokens: &mut Tokens, start_index: usize, end_index: usize) {
    let mut i = tokens.get_next_meaningful_token(start_index).unwrap();
    while i <= end_index {
        clear_whitespaces_before_index(tokens, i);
        tokens.clear_token_and_merge_surrounding_whitespace(i);
        i = tokens.get_next_meaningful_token(i).unwrap(); // the argument list's `,` or `)` follows
    }
}

fn get_default_value_end(tokens: &Tokens, mut index: usize) -> usize {
    while let Some(i) = tokens.get_next_meaningful_token(index) {
        index = i;
        if tokens[index].equals_any(&[",".into(), CT_PROPERTY_HOOK_BRACE_OPEN.into()], true) {
            break;
        }
        if let Some((bt, true)) = Tokens::detect_block_type(&tokens[index]) {
            index = tokens.find_block_end(bt, index);
        }
    }
    tokens.get_prev_meaningful_token(index).unwrap()
}

fn clear_whitespaces_before_index(tokens: &mut Tokens, index: usize) {
    let prev_index = tokens.get_non_empty_sibling(index, -1).unwrap();
    if !tokens[prev_index].is_whitespace(None) {
        return;
    }
    let prev_non_white_index = tokens.get_prev_non_whitespace(prev_index, None);
    if prev_non_white_index.is_none_or(|p| !tokens[p].is_comment()) {
        tokens.clear_token_and_merge_surrounding_whitespace(prev_index);
    }
}
