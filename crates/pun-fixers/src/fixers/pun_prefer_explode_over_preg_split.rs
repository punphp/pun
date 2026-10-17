//! Port of Mago's `prefer-explode-over-preg-split`: `preg_split('/, /', $s)` → `explode(', ', $s)` when the
//! pattern is a plain string (common delimiter, no modifiers, no regex meta-characters).
//! Opt-in only (`Pun/prefer_explode_over_preg_split`). Deviations from Mago, which would change results:
//! - a `-1`/`0`/`null` limit is dropped (no limit) instead of kept: `explode()` reads a negative limit as
//!   "all but the last N" and `0` as `1`. Non-literal limits are skipped.
//! - an empty pattern (`'//'`, split into characters) is skipped: `explode('')` throws.
//! - the call keeps its own qualification (`preg_split` → `explode`, `\preg_split` → `\explode`) and the
//!   separator keeps its quote style.
use crate::analyzers::helpers::split_list;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

const META_CHARS: &[u8] = b"\\.+*?[^]$(){}=!<>|:-#";

struct PreferExplodeOverPregSplit;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PreferExplodeOverPregSplit)
}

impl Fixer for PreferExplodeOverPregSplit {
    fn name(&self) -> &'static str {
        "Pun/prefer_explode_over_preg_split"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_STRING, T_CONSTANT_ENCAPSED_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let analyzer = FunctionsAnalyzer::new();
        for i in (0..tokens.count()).rev() {
            if !tokens[i].is_kind(T_STRING)
                || !tokens[i].get_content().eq_ignore_ascii_case(b"preg_split")
                || !analyzer.is_global_function_call(tokens, i)
            {
                continue;
            }
            let open = tokens.get_next_meaningful_token(i).unwrap();
            let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
            let args = split_list(tokens, open, close);
            if !(2..=4).contains(&args.len())
                || args
                    .iter()
                    .any(|&(s, _)| tokens[s].is_given_kind(&[T_ELLIPSIS, CT_NAMED_ARGUMENT_NAME]))
                || (open..=close).any(|k| tokens[k].is_comment())
            {
                continue;
            }
            let (ps, pe) = args[0];
            if ps != pe || !tokens[ps].is_kind(T_CONSTANT_ENCAPSED_STRING) {
                continue;
            }
            let Some(separator) = plain_separator(tokens[ps].get_content()) else {
                continue;
            };
            // Remove limit and flags (`drop_from` = first argument to remove), or keep a positive literal limit.
            let drop_from = match args.get(2).map(|&(s, e)| {
                (s..=e)
                    .filter(|&k| !tokens[k].is_whitespace(None))
                    .map(|k| &tokens[k])
                    .collect::<Vec<_>>()
            }) {
                None => None,
                Some(limit) => {
                    let lit = |t: &Token| {
                        t.is_kind(T_LNUMBER) && t.get_content().iter().all(u8::is_ascii_digit)
                    };
                    let no_limit = match limit.as_slice() {
                        [n] if lit(n) => n.get_content().iter().all(|&c| c == b'0'),
                        [n] if n.is_kind(T_STRING)
                            && n.get_content().eq_ignore_ascii_case(b"null") =>
                        {
                            true
                        }
                        [m, n] if m.equals("-", true) && lit(n) => true,
                        _ => continue,
                    };
                    if let Some(&(s, e)) = args.get(3)
                        && !(s == e
                            && tokens[s].is_kind(T_LNUMBER)
                            && tokens[s].get_content() == b"0")
                    {
                        continue;
                    }
                    if no_limit {
                        Some(2)
                    } else if args.len() == 4 {
                        Some(3)
                    } else {
                        None
                    }
                }
            };
            if let Some(d) = drop_from {
                tokens.clear_range(args[d - 1].1 + 1, args[args.len() - 1].1);
            }
            tokens.set(ps, Token::new(T_CONSTANT_ENCAPSED_STRING, separator));
            tokens.set(i, Token::new(T_STRING, "explode"));
        }
    }
}

/// For a pattern literal like `'/, /'` or `"/\n/"`, the separator literal (`', '`, `"\n"`), when the pattern
/// has a plain delimiter, no modifiers, a non-empty body and no regex meta-characters.
fn plain_separator(raw: &[u8]) -> Option<Vec<u8>> {
    let quote = raw[0];
    let src = &raw[1..raw.len() - 1];
    let mut pattern = vec![];
    let mut k = 0;
    while k < src.len() {
        let c = src[k];
        if c == b'\\' {
            // Single-quoted `\\` / `\'`, and double-quoted escapes other than control bytes, are not plain.
            if quote != b'"' {
                return None;
            }
            let n = *src.get(k + 1)?;
            let (byte, len) = match n {
                b'n' => (b'\n', 2),
                b't' => (b'\t', 2),
                b'r' => (b'\r', 2),
                b'v' => (0x0b, 2),
                b'e' => (0x1b, 2),
                b'f' => (0x0c, 2),
                b'x' => {
                    let hex: Vec<u8> = src[k + 2..]
                        .iter()
                        .take(2)
                        .copied()
                        .take_while(u8::is_ascii_hexdigit)
                        .collect();
                    (
                        u8::from_str_radix(std::str::from_utf8(&hex).ok()?, 16).ok()?,
                        2 + hex.len(),
                    )
                }
                b'0'..=b'7' => {
                    let oct: Vec<u8> = src[k + 1..]
                        .iter()
                        .take(3)
                        .copied()
                        .take_while(|c| (b'0'..=b'7').contains(c))
                        .collect();
                    (
                        (u32::from_str_radix(std::str::from_utf8(&oct).ok()?, 8).ok()? & 0xff)
                            as u8,
                        1 + oct.len(),
                    )
                }
                _ => return None,
            };
            pattern.push(byte);
            k += len;
        } else {
            if quote == b'"' && c == b'$' {
                return None;
            }
            pattern.push(c);
            k += 1;
        }
    }
    let delimiter = *pattern.first()?;
    if !b"/#~!@%,;|".contains(&delimiter) || pattern.len() < 3 || *pattern.last()? != delimiter {
        return None;
    }
    let body = &pattern[1..pattern.len() - 1];
    if body
        .iter()
        .any(|c| *c == delimiter || META_CHARS.contains(c))
        || src[0] != delimiter
        || src[src.len() - 1] != delimiter
    {
        return None;
    }
    // The delimiters are plain characters in the source too, so the separator is the source minus them.
    Some([&[quote], &src[1..src.len() - 1], &[quote]].concat())
}

#[cfg(test)]
mod tests {
    fn fix(code: &str) -> String {
        let mut tokens = pun_tokens::Tokens::from_code(code.as_bytes());
        // Gate on is_candidate like Pipeline::run does, so a too-strict candidate check fails the cases.
        let f = super::fixer();
        if f.is_candidate(&tokens) {
            f.fix(std::path::Path::new("t.php"), &mut tokens);
        }
        String::from_utf8(tokens.generate_code()).unwrap()
    }

    #[test]
    fn mago_cases() {
        for (from, to) in [
            (
                "$parts = preg_split('/, /', $s);",
                "$parts = explode(', ', $s);",
            ),
            (
                "$parts = preg_split('/, /', $s, 3);",
                "$parts = explode(', ', $s, 3);",
            ),
            // fix_drops_zero_flags: Mago keeps the -1, which explode() reads as "drop the last piece".
            (
                "$parts = preg_split('/, /', $s, -1, 0);",
                "$parts = explode(', ', $s);",
            ),
            (
                "$parts = preg_split('#abc#', $s);",
                "$parts = explode('abc', $s);",
            ),
            // fix_hash_delimited_pattern_in_namespace: Mago writes `\explode`, Pun keeps the call's qualification.
            (
                "namespace App;\n$parts = preg_split('#abc#', $s);",
                "namespace App;\n$parts = explode('abc', $s);",
            ),
            (
                "$parts = preg_split(\"/\\n\\n/\", $s);",
                "$parts = explode(\"\\n\\n\", $s);",
            ),
            (
                "$parts = preg_split(\"/\\t/\", $s);",
                "$parts = explode(\"\\t\", $s);",
            ),
            (
                "$parts = preg_split(\"/\\r\\n/\", $s);",
                "$parts = explode(\"\\r\\n\", $s);",
            ),
            (
                "$parts = preg_split(\"/\\x01/\", $s);",
                "$parts = explode(\"\\x01\", $s);",
            ),
        ] {
            assert_eq!(
                fix(&format!("<?php\n\n{from}\n")),
                format!("<?php\n\n{to}\n")
            );
        }
        for same in [
            "<?php\n\n$parts = preg_split('/\\s+/', $s);\n",
            "<?php\n\n$parts = preg_split('/, /i', $s);\n",
            "<?php\n\n$parts = preg_split('/, /', $s, -1, PREG_SPLIT_NO_EMPTY);\n",
            "<?php\n\n$parts = preg_split($pattern, $s);\n",
            "<?php\n\n$parts = preg_split('/a-z/', $s);\n",
        ] {
            assert_eq!(fix(same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php \\preg_split('~;~', $s, 0);"),
            "<?php \\explode(';', $s);"
        );
        assert_eq!(
            fix("<?php preg_split('/,/', $s, null);"),
            "<?php explode(',', $s);"
        );
        assert_eq!(
            fix("<?php preg_split('/,/', $s, 2, 0);"),
            "<?php explode(',', $s, 2);"
        );
        assert_eq!(
            fix("<?php preg_split(\"/\\x2c/\", $s);"),
            "<?php explode(\"\\x2c\", $s);"
        );
        assert_eq!(
            fix("<?php preg_split(\"/\\v\\e\\f/\", $s);"),
            "<?php explode(\"\\v\\e\\f\", $s);"
        );
        assert_eq!(
            fix("<?php preg_split(\"/\\54/\", trim($s));"),
            "<?php explode(\"\\54\", trim($s));"
        );
        for same in [
            "<?php preg_split('//', $s);",
            "<?php preg_split('/,/', $s, $limit);",
            "<?php preg_split('/,/', $s, -1, $flags);",
            "<?php preg_split('/\\\\/', $s);",
            "<?php preg_split(\"/\\x2e/\", $s);",
            "<?php preg_split(\"/\\d/\", $s);",
            "<?php preg_split('{,}', $s);",
            "<?php preg_split('/,/');",
            "<?php preg_split(pattern: '/,/', subject: $s);",
            "<?php $o->preg_split('/,/', $s);",
            "<?php preg_split('/a/b/', $s);",
            "<?php preg_split(\"/a$/\", $s);",
            "<?php preg_split($p, 'a,b');",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
