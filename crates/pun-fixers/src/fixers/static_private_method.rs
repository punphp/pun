//! Port of `PhpCsFixer\Fixer\ClassNotation\StaticPrivateMethodFixer`.
use pun_format::Fixer;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Kind::*, Token, Tokens, TokensAnalyzer};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

const MAGIC_METHODS: &[&[u8]] = &[b"__clone", b"__construct", b"__destruct", b"__wakeup"];

struct StaticPrivateMethodFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(StaticPrivateMethodFixer)
}

impl Fixer for StaticPrivateMethodFixer {
    fn name(&self) -> &'static str {
        "static_private_method"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_CLASS, T_PRIVATE, T_FUNCTION])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        loop {
            let mut anything_changed = false;
            let end = tokens.count() - 3; // min. number of tokens to form a class candidate to fix
            for index in (1..=end).rev() {
                if !tokens[index].is_kind(T_CLASS) {
                    continue;
                }
                let class_open = tokens
                    .get_next_token_of_kind(index, &["{".into()], true)
                    .unwrap();
                let class_close = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, class_open);
                anything_changed |= fix_class(tokens, class_open, class_close);
            }
            if !anything_changed {
                break;
            }
        }
    }
}

fn fix_class(tokens: &mut Tokens, class_open: usize, class_close: usize) -> bool {
    let mut fixed_methods: HashSet<Vec<u8>> = HashSet::new();
    // Upstream's generator yields from the end, and each insert lands at or after the method it fixes, so the
    // methods still to come keep their indices.
    for (function_keyword_index, method_open, method_close) in
        get_class_methods(tokens, class_open, class_close)
    {
        if skip_method(tokens, function_keyword_index, method_open, method_close) {
            continue;
        }
        let method_name_index = tokens
            .get_next_meaningful_token(function_keyword_index)
            .unwrap();
        fixed_methods.insert(tokens[method_name_index].get_content().to_vec());
        tokens.insert_slices(BTreeMap::from([(
            function_keyword_index,
            vec![
                Token::new(T_STATIC, "static"),
                Token::new(T_WHITESPACE, " "),
            ],
        )]));
    }
    if fixed_methods.is_empty() {
        return false;
    }
    let class_close = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, class_open);
    for (_, method_open, method_close) in get_class_methods(tokens, class_open, class_close) {
        fix_references_in_function(tokens, method_open, method_close, &fixed_methods);
    }
    true
}

fn skip_method(
    tokens: &Tokens,
    function_keyword_index: usize,
    method_open: usize,
    method_close: usize,
) -> bool {
    let analyzer = TokensAnalyzer::new(tokens);
    let prev = |i: usize| tokens.get_prev_meaningful_token(i).unwrap();
    let next = |i: usize| tokens.get_next_meaningful_token(i).unwrap();
    let method_name_index = next(function_keyword_index);
    let method_name = strtolower(tokens[method_name_index].get_content());
    if MAGIC_METHODS.contains(&&method_name[..]) {
        return true;
    }
    let mut prev_token_index = prev(function_keyword_index);
    if tokens[prev_token_index].is_kind(T_FINAL) {
        prev_token_index = prev(prev_token_index);
    }
    if !tokens[prev_token_index].is_kind(T_PRIVATE) {
        return true;
    }
    if tokens[prev(prev_token_index)].is_kind(T_STATIC) {
        return true;
    }
    let mut index = method_open + 1;
    while index < method_close - 1 {
        if tokens[index].is_kind(T_CLASS) && analyzer.is_anonymous_class(index) {
            let anonymous_class_open = tokens
                .get_next_token_of_kind(index, &["{".into()], true)
                .unwrap();
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, anonymous_class_open) + 1;
            continue;
        }
        if tokens[index].is_kind(T_FUNCTION) {
            return true;
        }
        if tokens[index].equals((T_VARIABLE, "$this"), true) {
            let operator_index = next(index);
            let method_name_index = next(operator_index);
            let arguments_brace_index = next(method_name_index);
            if !tokens[operator_index].is_kind(T_OBJECT_OPERATOR)
                || method_name != tokens[method_name_index].get_content()
                || !tokens[arguments_brace_index].equals("(", true)
            {
                return true;
            }
        }
        if tokens[index].equals((T_STRING, "debug_backtrace"), true) {
            return true;
        }
        index += 1;
    }
    false
}

fn fix_references_in_function(
    tokens: &mut Tokens,
    method_open: usize,
    method_close: usize,
    fixed_methods: &HashSet<Vec<u8>>,
) {
    let mut index = method_open + 1;
    while index < method_close - 1 {
        if tokens[index].is_kind(T_FUNCTION) {
            let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            let closure_start = tokens
                .get_next_token_of_kind(index, &["{".into()], true)
                .unwrap();
            let closure_end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, closure_start);
            if !tokens[prev_index].is_kind(T_STATIC) {
                fix_references_in_function(tokens, closure_start, closure_end, fixed_methods);
            }
            index = closure_end + 1;
            continue;
        }
        if tokens[index].is_kind(T_CLASS) && TokensAnalyzer::new(tokens).is_anonymous_class(index) {
            let anonymous_class_open = tokens
                .get_next_token_of_kind(index, &["{".into()], true)
                .unwrap();
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, anonymous_class_open) + 1;
            continue;
        }
        if !tokens[index].equals((T_VARIABLE, "$this"), true) {
            index += 1;
            continue;
        }
        let object_operator_index = tokens.get_next_meaningful_token(index).unwrap();
        let method_name_index = tokens
            .get_next_meaningful_token(object_operator_index)
            .unwrap();
        let arguments_brace_index = tokens.get_next_meaningful_token(method_name_index).unwrap();
        if tokens[object_operator_index].is_kind(T_OBJECT_OPERATOR)
            && tokens[arguments_brace_index].equals("(", true)
            && fixed_methods.contains(tokens[method_name_index].get_content())
        {
            tokens.set(index, Token::new(T_STRING, "self"));
            tokens.set(object_operator_index, Token::new(T_DOUBLE_COLON, "::"));
        }
        index += 1;
    }
}

fn get_class_methods(
    tokens: &Tokens,
    class_open: usize,
    class_close: usize,
) -> Vec<(usize, usize, usize)> {
    let analyzer = TokensAnalyzer::new(tokens);
    let mut methods = Vec::new();
    let mut index = class_close - 1;
    while index > class_open + 1 {
        if tokens[index].equals("}", true) {
            index = tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, index) - 1;
            continue;
        }
        if tokens[index].is_kind(T_FUNCTION) && !analyzer.get_method_attributes(index).abstract_ {
            let method_open = tokens
                .get_next_token_of_kind(index, &["{".into()], true)
                .unwrap();
            let method_close = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, method_open);
            methods.push((index, method_open, method_close));
        }
        index -= 1;
    }
    methods
}
