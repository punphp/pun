//! ParseLexer against real PHP: tests/fixtures/lint_cases.json (scripts/export-lint-cases.py) holds what
//! `token_get_all($code, TOKEN_PARSE)` throws. Every scanner exception must come out of the lexer, with PHP's
//! line; code PHP accepts must lex to the end (except junk after `__halt_compiler();`, which PHP never lexes).

use pun_tokens::{LexError, ParseLexer};

/// Messages raised by Zend/zend_language_scanner.l rather than the grammar.
fn is_scanner_error(m: &str) -> bool {
    [
        "Invalid ",
        "Unclosed '",
        "Unmatched '",
        "Unterminated comment",
        "The (real) cast",
    ]
    .iter()
    .any(|p| m.starts_with(p))
}

fn drain(code: &str) -> Result<usize, LexError> {
    let mut lx = ParseLexer::new(code.as_bytes());
    let mut n = 0;
    while lx.next_token()?.is_some() {
        n += 1;
    }
    Ok(n)
}

#[test]
fn scanner_errors_match_php() {
    let cases: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("fixtures/lint_cases.json")).unwrap();
    let mut checked = 0;
    for case in &cases {
        let code = case["code"].as_str().unwrap();
        if code.to_ascii_lowercase().contains("__halt_compiler") {
            continue; // the parser decides where lexing stops
        }
        match case["error"]
            .as_str()
            .and_then(|e| e.strip_prefix("Parse error: "))
        {
            Some(m) if is_scanner_error(m) => {
                let (msg, line) = m.rsplit_once(" on line ").unwrap();
                let got = drain(code).expect_err(code);
                assert_eq!(
                    (got.message.as_str(), got.line),
                    (msg, line.trim_end_matches('.').parse().unwrap()),
                    "{code:?}"
                );
                checked += 1;
            }
            // Accepted, or a grammar error: the scanner itself never throws.
            _ => {
                assert!(drain(code).is_ok(), "{code:?}: {:?}", drain(code));
            }
        }
    }
    assert!(checked > 60, "{checked}");
}

#[test]
fn halt_compiler_stops_lexing() {
    // `zend_stop_lexing()`: nothing after the halt is lexed, so the unclosed `(` is never seen.
    let mut lx = ParseLexer::new(b"<?php __halt_compiler(); (");
    for _ in 0..4 {
        lx.next_token().unwrap().unwrap();
    }
    lx.stop();
    assert_eq!(lx.next_token(), Ok(None));
    assert_eq!(
        drain("<?php __halt_compiler(); ("),
        Err(LexError {
            message: "Unclosed '('".into(),
            line: 1
        })
    );
}

#[test]
fn tokens_the_parser_rejects_still_lex() {
    // PHP: `syntax error, unexpected string content "x"` on line 2: an unterminated single-quoted string is one
    // T_ENCAPSED_AND_WHITESPACE whose newlines the scanner does not count.
    let mut lx = ParseLexer::new(b"<?php\n$a = 'x\ny");
    assert_eq!(
        lx.next_token().unwrap().unwrap().kind,
        Some(pun_tokens::Kind::T_VARIABLE)
    );
    lx.next_token().unwrap();
    let s = lx.next_token().unwrap().unwrap();
    assert_eq!(
        (s.kind, s.start, s.end, lx.lineno()),
        (Some(pun_tokens::Kind::T_ENCAPSED_AND_WHITESPACE), 11, 15, 2)
    );
    // PHP: `syntax error, unexpected string content "", expecting ...` on line 2: ST_VAR_OFFSET hands the parser
    // an empty T_ENCAPSED_AND_WHITESPACE before the space.
    let kinds: Vec<_> = std::iter::from_fn({
        let mut lx = ParseLexer::new(b"<?php\n\"$a[ 1]\";");
        move || lx.next_token().unwrap()
    })
    .map(|t| (t.kind, t.end - t.start))
    .collect();
    assert_eq!(
        kinds[3],
        (Some(pun_tokens::Kind::T_ENCAPSED_AND_WHITESPACE), 0)
    );
}
