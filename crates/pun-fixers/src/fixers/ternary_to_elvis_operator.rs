//! Port of `PhpCsFixer\Fixer\Operator\TernaryToElvisOperatorFixer`.
use crate::analyzers::range_analyzer::{Range, range_equals_range};
use pun_format::Fixer;
use pun_tokens::{Kind::*, Proto, Tokens};
use std::path::Path;

const VALID_BEFORE_ENDTYPES: &[Proto<'static>] = &[
    Proto::Char(b"="),
    Proto::Kind(T_OPEN_TAG),
    Proto::Kind(T_OPEN_TAG_WITH_ECHO),
    Proto::Char(b"("),
    Proto::Char(b","),
    Proto::Char(b";"),
    Proto::Char(b"["),
    Proto::Char(b"{"),
    Proto::Char(b"}"),
    Proto::Kind(CT_ARRAY_INDEX_BRACE_OPEN),
    Proto::Kind(T_AND_EQUAL),
    Proto::Kind(T_CONCAT_EQUAL),
    Proto::Kind(T_DIV_EQUAL),
    Proto::Kind(T_MINUS_EQUAL),
    Proto::Kind(T_MOD_EQUAL),
    Proto::Kind(T_MUL_EQUAL),
    Proto::Kind(T_OR_EQUAL),
    Proto::Kind(T_PLUS_EQUAL),
    Proto::Kind(T_POW_EQUAL),
    Proto::Kind(T_SL_EQUAL),
    Proto::Kind(T_SR_EQUAL),
    Proto::Kind(T_XOR_EQUAL),
];

struct TernaryToElvisOperatorFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(TernaryToElvisOperatorFixer)
}

impl Fixer for TernaryToElvisOperatorFixer {
    fn name(&self) -> &'static str {
        "ternary_to_elvis_operator"
    }
    fn priority(&self) -> i32 {
        2
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("?")
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() as isize - 5;
        while index > 1 {
            let i = index as usize;
            index -= 1;
            if !tokens[i].equals("?", true) {
                continue;
            }
            let next_index = tokens.get_next_meaningful_token(i).unwrap();
            if tokens[next_index].equals(":", true) {
                continue; // Elvis is alive!
            }
            let Some(before_operator) = get_before_operator(tokens, i) else {
                continue; // contains something we cannot fix because of priorities
            };
            let after_operator = get_after_operator(tokens, i);
            if range_equals_range(tokens, before_operator, after_operator) {
                clear_meaningful_from_range(tokens, after_operator);
            }
        }
    }
}

fn get_before_operator(tokens: &Tokens, index: usize) -> Option<Range> {
    let prev = |i: usize| tokens.get_prev_meaningful_token(i).unwrap();
    let mut index = prev(index);
    let end = index;
    let mut start = None;
    while !tokens[index].equals_any(VALID_BEFORE_ENDTYPES, true) {
        if tokens[index].is_given_kind(&[T_INC, T_DEC]) {
            return None;
        }
        let detected = Tokens::detect_block_type(&tokens[index]);
        let Some((block_type, false)) = detected else {
            start = Some(index);
            index = prev(index);
            continue;
        };
        let (block_start, block_end) = Tokens::block_edges(block_type);
        let mut open_count = 1;
        loop {
            index = prev(index);
            if tokens[index].is_given_kind(&[T_INC, T_DEC]) {
                return None;
            }
            if tokens[index].equals(block_start, true) {
                open_count += 1;
            } else if tokens[index].equals(block_end, true) {
                open_count -= 1;
            }
            if open_count > 1 {
                break;
            }
        }
        start = Some(index);
        index = prev(index);
    }
    Some(Range { start: start?, end })
}

fn get_after_operator(tokens: &Tokens, index: usize) -> Range {
    let mut index = tokens.get_next_meaningful_token(index).unwrap();
    let start = index;
    loop {
        if let Some((block_type, _)) = Tokens::detect_block_type(&tokens[index]) {
            index = tokens.find_block_end(block_type, index);
        }
        let end = index;
        index = tokens.get_next_meaningful_token(index).unwrap();
        if tokens[index].equals(":", true) {
            return Range { start, end };
        }
    }
}

fn clear_meaningful_from_range(tokens: &mut Tokens, range: Range) {
    // range.end must be meaningful!
    let mut i = range.end;
    while i >= range.start {
        tokens.clear_token_and_merge_surrounding_whitespace(i);
        i = tokens.get_prev_meaningful_token(i).unwrap();
    }
}
