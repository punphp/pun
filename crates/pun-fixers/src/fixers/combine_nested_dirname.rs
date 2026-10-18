//! Port of `PhpCsFixer\Fixer\FunctionNotation\CombineNestedDirnameFixer`.
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct CombineNestedDirnameFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(CombineNestedDirnameFixer)
}

/// `getDirnameInfo`'s array.
struct DirnameInfo {
    indices: Vec<usize>,
    second_argument: Option<usize>,
    levels: i64,
    end: usize,
}

impl Fixer for CombineNestedDirnameFixer {
    fn name(&self) -> &'static str {
        "combine_nested_dirname"
    }
    fn priority(&self) -> i32 {
        35
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() as isize - 1;
        while 0 <= index {
            let Some(mut dirname_info) = get_dirname_info(tokens, index as usize, None) else {
                index -= 1;
                continue;
            };
            let mut prev = tokens
                .get_prev_meaningful_token(dirname_info.indices[0])
                .unwrap();
            if !tokens[prev].equals("(", true) {
                index -= 1;
                continue;
            }
            prev = tokens.get_prev_meaningful_token(prev).unwrap();
            let mut first_argument_end = dirname_info.end;
            let mut dirname_info_array = vec![];
            loop {
                dirname_info_array.push(dirname_info);
                let Some(next) = get_dirname_info(tokens, prev, Some(first_argument_end)) else {
                    break;
                };
                dirname_info = next;
                prev = tokens
                    .get_prev_meaningful_token(dirname_info.indices[0])
                    .unwrap();
                if !tokens[prev].equals("(", true) {
                    dirname_info_array.push(dirname_info);
                    break;
                }
                prev = tokens.get_prev_meaningful_token(prev).unwrap();
                first_argument_end = dirname_info.end;
            }
            if dirname_info_array.len() > 1 {
                combine_dirnames(tokens, dirname_info_array);
            }
            index = prev as isize - 1;
        }
    }
}

fn get_dirname_info(
    tokens: &Tokens,
    index: usize,
    first_argument_end_index: Option<usize>,
) -> Option<DirnameInfo> {
    if !tokens[index].equals((T_STRING, "dirname"), false) {
        return None;
    }
    if !FunctionsAnalyzer::new().is_global_function_call(tokens, index) {
        return None;
    }
    let mut indices = vec![];
    let prev = tokens.get_prev_meaningful_token(index).unwrap();
    if tokens[prev].is_kind(T_NS_SEPARATOR) {
        indices.push(prev);
    }
    indices.push(index);
    let mut next = tokens.get_next_meaningful_token(index).unwrap();
    indices.push(next);
    if let Some(first_argument_end_index) = first_argument_end_index {
        next = tokens
            .get_next_meaningful_token(first_argument_end_index)
            .unwrap();
    } else {
        next = tokens.get_next_meaningful_token(next).unwrap();
        if tokens[next].equals(")", true) {
            return None;
        }
        while !tokens[next].equals_any(&[",".into(), ")".into()], true) {
            if let Some((block_type, _)) = Tokens::detect_block_type(&tokens[next]) {
                next = tokens.find_block_end(block_type, next);
            }
            next = tokens.get_next_meaningful_token(next).unwrap();
        }
    }
    indices.push(next);
    if tokens[next].equals(",", true) {
        next = tokens.get_next_meaningful_token(next).unwrap();
        indices.push(next);
    }
    if tokens[next].equals(")", true) {
        return Some(DirnameInfo {
            indices,
            second_argument: None,
            levels: 1,
            end: next,
        });
    }
    if !tokens[next].is_kind(T_LNUMBER) {
        return None;
    }
    let second_argument = next;
    let levels = php_int(tokens[next].get_content());
    next = tokens.get_next_meaningful_token(next).unwrap();
    if tokens[next].equals(",", true) {
        indices.push(next);
        next = tokens.get_next_meaningful_token(next).unwrap();
    }
    if !tokens[next].equals(")", true) {
        return None;
    }
    indices.push(next);
    Some(DirnameInfo {
        indices,
        second_argument: Some(second_argument),
        levels,
        end: next,
    })
}

/// `(int) $content` for a `T_LNUMBER`: PHP's string to int cast reads leading decimal digits only (`0x10` is 0,
/// `1_000` is 1), saturating on overflow.
fn php_int(content: &[u8]) -> i64 {
    content
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .fold(0i64, |n, b| {
            n.saturating_mul(10).saturating_add(i64::from(b - b'0'))
        })
}

fn combine_dirnames(tokens: &mut Tokens, mut dirname_info_array: Vec<DirnameInfo>) {
    let outer_dirname_info = dirname_info_array.pop().unwrap();
    let mut levels = outer_dirname_info.levels;
    for dirname_info in &dirname_info_array {
        levels = levels.saturating_add(dirname_info.levels);
        for &index in &dirname_info.indices {
            tokens.remove_leading_whitespace(index, None);
            tokens.clear_token_and_merge_surrounding_whitespace(index);
        }
    }
    let levels_token = Token::new(T_LNUMBER, levels.to_string());
    if let Some(second_argument) = outer_dirname_info.second_argument {
        tokens.set(second_argument, levels_token);
    } else {
        let prev = tokens
            .get_prev_meaningful_token(outer_dirname_info.end)
            .unwrap();
        let mut items = vec![];
        if !tokens[prev].equals(",", true) {
            items = vec![Token::char(","), Token::new(T_WHITESPACE, " ")];
        }
        items.push(levels_token);
        tokens.insert_at(outer_dirname_info.end, items);
    }
}
