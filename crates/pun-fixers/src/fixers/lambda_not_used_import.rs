//! Port of `PhpCsFixer\Fixer\FunctionNotation\LambdaNotUsedImportFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Kind, Tokens, TokensAnalyzer};
use indexmap::IndexMap;
use std::collections::BTreeMap;
use std::path::Path;

/// Insertion-ordered `array<string, int>`.
type Imports = IndexMap<Vec<u8>, usize>;

/// `unset($imports[$key])`, true when it was set.
fn unset(imports: &mut Imports, key: &[u8]) -> bool {
    imports.shift_remove(key).is_some()
}

struct LambdaNotUsedImportFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(LambdaNotUsedImportFixer)
}

impl Fixer for LambdaNotUsedImportFixer {
    fn name(&self) -> &'static str {
        "lambda_not_used_import"
    }
    fn priority(&self) -> i32 {
        31
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[Kind::T_FUNCTION, Kind::CT_USE_LAMBDA])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() as isize - 4;
        while index > 0 {
            if let Some(lambda_use_index) = get_lambda_use_index(tokens, index as usize) {
                fix_lambda(tokens, lambda_use_index);
            }
            index -= 1;
        }
    }
}

fn fix_lambda(tokens: &mut Tokens, lambda_use_index: usize) {
    let open = tokens
        .get_next_token_of_kind(lambda_use_index, &["(".into()], true)
        .unwrap();
    let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
    let arguments = ArgumentsAnalyzer.get_arguments(tokens, open, close);
    let imports = filter_arguments(tokens, &arguments);
    if imports.is_empty() {
        return;
    }
    let not_used = find_not_used_lambda_imports(tokens, imports, close);
    if not_used.is_empty() {
        return;
    }
    if not_used.len() == arguments.len() {
        clear_imports_and_use(tokens, lambda_use_index, close);
        return;
    }
    clear_imports(tokens, not_used.iter().rev().map(|(_, i)| *i));
}

fn find_not_used_lambda_imports(
    tokens: &Tokens,
    mut imports: Imports,
    lambda_use_close_brace_index: usize,
) -> Imports {
    let functions = FunctionsAnalyzer::new();
    let mut index = tokens
        .get_next_token_of_kind(lambda_use_close_brace_index, &["{".into()], true)
        .unwrap();
    let mut level = 0;
    loop {
        let token = &tokens[index];
        if token.equals("{", true) {
            level += 1;
            index += 1;
            continue;
        }
        if token.equals("}", true) {
            level -= 1;
            if level == 0 {
                break;
            }
            index += 1;
            continue;
        }
        if token.is_kind(Kind::T_STRING)
            && strtolower(token.get_content()) == b"compact"
            && functions.is_global_function_call(tokens, index)
        {
            return Imports::new();
        }
        if token.is_given_kind(&[
            Kind::CT_DYNAMIC_VAR_BRACE_OPEN,
            Kind::T_EVAL,
            Kind::T_INCLUDE,
            Kind::T_INCLUDE_ONCE,
            Kind::T_REQUIRE,
            Kind::T_REQUIRE_ONCE,
        ]) {
            return Imports::new();
        }
        if token.equals("$", true)
            && tokens[tokens.get_next_meaningful_token(index).unwrap()].is_kind(Kind::T_VARIABLE)
        {
            return Imports::new();
        }
        if token.is_kind(Kind::T_VARIABLE)
            && unset(&mut imports, token.get_content())
            && imports.is_empty()
        {
            return imports;
        }
        if token.is_kind(Kind::T_STRING_VARNAME)
            && unset(&mut imports, &[b"$", token.get_content()].concat())
            && imports.is_empty()
        {
            return imports;
        }
        if token.is_classy() {
            index = tokens
                .get_next_token_of_kind(index, &["(".into(), "{".into()], true)
                .unwrap();
            if tokens[index].equals("(", true) {
                let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
                let arguments = ArgumentsAnalyzer.get_arguments(tokens, index, close);
                imports = count_imports_used_as_argument(tokens, imports, &arguments);
                index = tokens
                    .get_next_token_of_kind(close, &["{".into()], true)
                    .unwrap();
            }
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index) + 1;
            continue;
        }
        if token.is_kind(Kind::T_FUNCTION) {
            let open = tokens
                .get_next_token_of_kind(index, &["(".into()], true)
                .unwrap();
            let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
            let arguments = ArgumentsAnalyzer.get_arguments(tokens, open, close);
            imports = count_imports_used_as_argument(tokens, imports, &arguments);
            index = tokens
                .get_next_token_of_kind(index, &[Kind::CT_USE_LAMBDA.into(), "{".into()], true)
                .unwrap();
            if tokens[index].is_kind(Kind::CT_USE_LAMBDA) {
                let open = tokens
                    .get_next_token_of_kind(index, &["(".into()], true)
                    .unwrap();
                let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
                let arguments = ArgumentsAnalyzer.get_arguments(tokens, open, close);
                imports = count_imports_used_as_argument(tokens, imports, &arguments);
                index = tokens
                    .get_next_token_of_kind(close, &["{".into()], true)
                    .unwrap();
            }
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index) + 1;
            continue;
        }
        index += 1;
    }
    imports
}

fn count_imports_used_as_argument(
    tokens: &Tokens,
    mut imports: Imports,
    arguments: &BTreeMap<usize, usize>,
) -> Imports {
    for (&start, &end) in arguments {
        let info = ArgumentsAnalyzer.get_argument_info(tokens, start, end);
        if let Some(name) = info.name
            && unset(&mut imports, &name)
            && imports.is_empty()
        {
            return imports;
        }
    }
    imports
}

fn get_lambda_use_index(tokens: &Tokens, index: usize) -> Option<usize> {
    if !tokens[index].is_kind(Kind::T_FUNCTION) || !TokensAnalyzer::new(tokens).is_lambda(index) {
        return None;
    }
    let mut i = tokens.get_next_meaningful_token(index).unwrap();
    if tokens[i].is_kind(Kind::CT_RETURN_REF) {
        i = tokens.get_next_meaningful_token(i).unwrap();
    }
    i = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, i);
    i = tokens.get_next_meaningful_token(i).unwrap();
    tokens[i].is_kind(Kind::CT_USE_LAMBDA).then_some(i)
}

fn filter_arguments(tokens: &Tokens, arguments: &BTreeMap<usize, usize>) -> Imports {
    let mut imports = Imports::new();
    let a = TokensAnalyzer::new(tokens);
    for (&start, &end) in arguments {
        let info = ArgumentsAnalyzer.get_argument_info(tokens, start, end);
        let argument = info.name_index.unwrap();
        if tokens[tokens.get_prev_meaningful_token(argument).unwrap()].equals("&", true) {
            continue;
        }
        let content = tokens[argument].get_content();
        if content == b"$this" || a.is_super_global(argument) {
            continue;
        }
        // PHP array assignment keeps the first key's position
        imports.insert(content.to_vec(), argument);
    }
    imports
}

fn clear_imports(tokens: &mut Tokens, imports: impl Iterator<Item = usize>) {
    for remove_index in imports {
        tokens.clear_token_and_merge_surrounding_whitespace(remove_index);
        let prev = tokens.get_prev_meaningful_token(remove_index).unwrap();
        if tokens[prev].equals(",", true) {
            tokens.clear_token_and_merge_surrounding_whitespace(prev);
        } else if tokens[prev].equals("(", true) {
            let next = tokens.get_next_meaningful_token(remove_index).unwrap();
            tokens.clear_token_and_merge_surrounding_whitespace(next);
        }
    }
}

fn clear_imports_and_use(tokens: &mut Tokens, lambda_use_index: usize, close: usize) {
    let mut i = close;
    while i >= lambda_use_index {
        let skip = tokens[i].is_comment()
            || (tokens[i].is_whitespace(None)
                && tokens[tokens.get_prev_non_whitespace(i, None).unwrap()].is_comment());
        if !skip {
            tokens.clear_token_and_merge_surrounding_whitespace(i);
        }
        i -= 1;
    }
}
