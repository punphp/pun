//! Port of `PhpCsFixer\Tokenizer\Analyzer\RangeAnalyzer`.
use pun_tokens::{Kind::*, Proto, Tokens};

/// `array{start: int, end: int}`.
#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

const ARRAY_OPEN_TYPES: &[Proto<'static>] =
    &[Proto::Char(b"["), Proto::Kind(CT_ARRAY_INDEX_BRACE_OPEN)];
const ARRAY_CLOSE_TYPES: &[Proto<'static>] =
    &[Proto::Char(b"]"), Proto::Kind(CT_ARRAY_INDEX_BRACE_CLOSE)];

/// The range's start and end with a leading non-meaningful token skipped and wrapping parentheses unwrapped.
fn trim(tokens: &Tokens, range: Range) -> (usize, usize) {
    let (mut start, mut end) = (range.start, range.end);
    if tokens[start].is_given_kind(&[T_WHITESPACE, T_COMMENT, T_DOC_COMMENT]) {
        start = tokens.get_next_meaningful_token(start).unwrap();
    }
    while tokens[start].equals("(", true) && tokens[end].equals(")", true) {
        start = tokens.get_next_meaningful_token(start).unwrap();
        end = tokens.get_prev_meaningful_token(end).unwrap();
    }
    (start, end)
}

/// Meaningful compare of tokens within ranges.
pub fn range_equals_range(tokens: &Tokens, range1: Range, range2: Range) -> bool {
    let (mut left_start, left_end) = trim(tokens, range1);
    let (mut right_start, right_end) = trim(tokens, range2);
    loop {
        let left_token = &tokens[left_start];
        let right_token = &tokens[right_start];
        if !left_token.equals_token(right_token, true)
            && !(left_token.equals_any(ARRAY_OPEN_TYPES, true)
                && right_token.equals_any(ARRAY_OPEN_TYPES, true))
            && !(left_token.equals_any(ARRAY_CLOSE_TYPES, true)
                && right_token.equals_any(ARRAY_CLOSE_TYPES, true))
        {
            return false;
        }
        let left_next = tokens.get_next_meaningful_token(left_start);
        let right_next = tokens.get_next_meaningful_token(right_start);
        // reached end or moved over
        let reached_left_end = left_next.is_none_or(|i| i > left_end);
        let reached_right_end = right_next.is_none_or(|i| i > right_end);
        if !reached_left_end && !reached_right_end {
            left_start = left_next.unwrap();
            right_start = right_next.unwrap();
            continue;
        }
        return reached_left_end && reached_right_end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RangeAnalyzerTest::provideRangeEqualsRangeCases (the pinned PHP-CS-Fixer's own cases).
    #[test]
    fn upstream_range_equals_range_cases() {
        let r = |start, end| Range { start, end };
        let parens = "<?php
                    $a = 1;
                    ($a = 1);
                    (($a = 1 ));
                ";
        let cases = [
            (true, parens, r(2, 6), r(10, 14)),
            (true, parens, r(10, 14), r(20, 24)),
            (true, parens, r(1, 6), r(20, 25)),
            (false, "<?php echo 1;", r(0, 1), r(1, 2)),
            (
                true,
                "<?php
                foo(1);
                /* */ foo/* */(1) /* */ ;
            ",
                r(1, 5),
                r(9, 17),
            ),
        ];
        for (expected, code, range1, range2) in cases {
            let tokens = Tokens::from_code(code.as_bytes());
            assert_eq!(
                range_equals_range(&tokens, range1, range2),
                expected,
                "{code}"
            );
        }
    }
}
