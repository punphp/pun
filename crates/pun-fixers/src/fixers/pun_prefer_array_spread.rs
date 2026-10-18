//! Port of Mago's `prefer-array-spread`: `array_merge($a, $b)` → `[...$a, ...$b]`. Array-literal arguments are
//! inlined when that keeps the keys (no integer-like keys), empty ones are dropped.
//! Opt-in only (`Pun/prefer_array_spread`). Risky: needs PHP >= 8.1 (string keys in spreads), and a spread
//! also accepts `Traversable`s, where `array_merge()` throws a `TypeError`.
use crate::analyzers::helpers::split_list;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::collections::BTreeSet;
use std::path::Path;

struct PreferArraySpread;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PreferArraySpread)
}

enum Arg {
    Drop,
    Spread,
    /// Inline the literal's elements (first and last element token).
    Inline(usize, usize),
}

impl Fixer for PreferArraySpread {
    fn name(&self) -> &'static str {
        "Pun/prefer_array_spread"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let analyzer = FunctionsAnalyzer::new();
        for i in (0..tokens.count()).rev() {
            if !tokens[i].is_kind(T_STRING)
                || !tokens[i].get_content().eq_ignore_ascii_case(b"array_merge")
                || !analyzer.is_global_function_call(tokens, i)
            {
                continue;
            }
            let open = tokens.get_next_meaningful_token(i).unwrap();
            let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
            let start = tokens
                .get_prev_meaningful_token(i)
                .filter(|&p| tokens[p].is_kind(T_NS_SEPARATOR))
                .unwrap_or(i);
            if (start..=close).any(|k| tokens[k].is_comment()) {
                continue;
            }
            let args = split_list(tokens, open, close);
            if args.iter().any(|&(s, _)| {
                tokens[s].is_given_kind(&[
                    T_ELLIPSIS,
                    CT_NAMED_ARGUMENT_NAME,
                    CT_FIRST_CLASS_CALLABLE,
                ])
            }) {
                continue;
            }
            let kinds: Vec<Arg> = args.iter().map(|&(s, e)| classify(tokens, s, e)).collect();
            let kept: Vec<usize> = (0..args.len())
                .filter(|&a| !matches!(kinds[a], Arg::Drop))
                .collect();
            let mut out = vec![Token::new(CT_ARRAY_BRACKET_OPEN, "[")];
            if !kept.is_empty() {
                let mut removed = BTreeSet::new();
                for a in (0..args.len()).filter(|&a| matches!(kinds[a], Arg::Drop)) {
                    match kept.iter().find(|&&j| j > a) {
                        Some(&j) => removed.extend(args[a].0..args[j].0),
                        None => removed.extend(
                            args[*kept.iter().rev().find(|&&h| h < a).unwrap()].1 + 1..=args[a].1,
                        ),
                    }
                }
                // The last argument keeps its own layout (`[...$a, \n    'k' => 1,\n]`): only its brackets go.
                let last_kept = kept[kept.len() - 1];
                for (a, kind) in kinds.iter().enumerate() {
                    let Arg::Inline(first, last) = *kind else {
                        continue;
                    };
                    let (s, e) = args[a];
                    if a == last_kept {
                        let open_d = if tokens[s].is_kind(T_ARRAY) {
                            tokens.get_next_meaningful_token(s).unwrap()
                        } else {
                            s
                        };
                        removed.extend(s..=open_d);
                        removed.extend(e..close);
                        let multiline = |w: usize| tokens[w].get_content().contains(&b'\n');
                        if tokens[open_d + 1].is_whitespace(None) {
                            if !multiline(open_d + 1) {
                                removed.insert(open_d + 1);
                            } else if tokens[s - 1].is_whitespace(None) {
                                removed.insert(s - 1);
                            }
                        }
                        if tokens[e - 1].is_whitespace(None) && !multiline(e - 1) {
                            removed.insert(e - 1);
                        }
                    } else {
                        removed.extend(s..first);
                        removed.extend(last + 1..=e);
                    }
                }
                for k in open + 1..close {
                    if removed.contains(&k) {
                        continue;
                    }
                    if let Some(a) = args.iter().position(|&(s, e)| s <= k && k <= e) {
                        match kinds[a] {
                            Arg::Spread if k == args[a].0 => {
                                out.push(Token::new(T_ELLIPSIS, "..."))
                            }
                            _ => {}
                        }
                    }
                    out.push(tokens[k].clone());
                }
            }
            out.push(Token::new(CT_ARRAY_BRACKET_CLOSE, "]"));
            tokens.override_range(start, close, out);
        }
    }
}

/// An argument that is an array literal is dropped when empty and inlined when every key is a non-integer
/// string literal (integer keys are renumbered by `array_merge()` but kept by an inlined literal).
fn classify(tokens: &Tokens, s: usize, e: usize) -> Arg {
    let open = if tokens[s].is_kind(CT_ARRAY_BRACKET_OPEN) {
        s
    } else if tokens[s].is_kind(T_ARRAY) {
        tokens.get_next_meaningful_token(s).unwrap()
    } else {
        return Arg::Spread;
    };
    let block = if tokens[open].is_kind(CT_ARRAY_BRACKET_OPEN) {
        Tokens::BLOCK_TYPE_ARRAY_BRACKET
    } else {
        Tokens::BLOCK_TYPE_PARENTHESIS
    };
    if tokens.find_block_end(block, open) != e {
        return Arg::Spread;
    }
    let elements = split_list(tokens, open, e);
    if elements.is_empty() {
        return Arg::Drop;
    }
    let inlinable = elements.iter().all(|&(es, ee)| {
        match (es..=ee).find(|&k| tokens[k].is_kind(T_DOUBLE_ARROW) && !inside_block(tokens, es, k))
        {
            None => true,
            Some(arrow) => {
                tokens.get_next_meaningful_token(es) == Some(arrow)
                    && tokens[es].is_kind(T_CONSTANT_ENCAPSED_STRING)
                    && is_string_array_key(tokens[es].get_content())
            }
        }
    });
    if inlinable {
        Arg::Inline(elements[0].0, elements[elements.len() - 1].1)
    } else {
        Arg::Spread
    }
}

fn inside_block(tokens: &Tokens, from: usize, k: usize) -> bool {
    let mut j = from;
    while j < k {
        if let Some((bt, true)) = Tokens::detect_block_type(&tokens[j]) {
            let end = tokens.find_block_end(bt, j);
            if end > k {
                return true;
            }
            j = end;
        }
        j += 1;
    }
    false
}

/// `'5'` is the integer key 5 in PHP; `''`, `'01'`, `'-0'`, `'+5'`, `' 5'` and out-of-range numbers stay strings.
fn is_string_array_key(raw: &[u8]) -> bool {
    let inner = &raw[1..raw.len() - 1];
    let value: Vec<u8> = match raw[0] {
        b'\'' => {
            let mut v = vec![];
            let mut it = inner.iter().peekable();
            while let Some(&c) = it.next() {
                if c == b'\\' && matches!(it.peek(), Some(b'\\' | b'\'')) {
                    v.push(*it.next().unwrap());
                } else {
                    v.push(c);
                }
            }
            v
        }
        b'"' if !inner.iter().any(|&c| c == b'\\' || c == b'$') => inner.to_vec(),
        _ => return false,
    };
    let digits = value.strip_prefix(b"-").unwrap_or(&value);
    digits.is_empty()
        || !digits.iter().all(u8::is_ascii_digit)
        || (digits[0] == b'0' && value != b"0")
        || std::str::from_utf8(&value).is_ok_and(|v| v.parse::<i64>().is_err())
}

#[cfg(test)]
mod tests {
    fn fix(code: &str) -> String {
        crate::fix_with(&*super::fixer(), code)
    }

    #[test]
    fn mago_cases() {
        for (from, to) in [
            (
                "$merged = array_merge($a, $b, $c);",
                "$merged = [...$a, ...$b, ...$c];",
            ),
            ("$copied = array_merge($a);", "$copied = [...$a];"),
            ("$empty = array_merge();", "$empty = [];"),
            (
                "$merged = array_merge($a, [1, 2, 3]);",
                "$merged = [...$a, 1, 2, 3];",
            ),
            (
                "$merged = array_merge($a, array(1, 2, 3));",
                "$merged = [...$a, 1, 2, 3];",
            ),
            (
                "$merged = array_merge($a, ['x' => 1, 'y' => 2]);",
                "$merged = [...$a, 'x' => 1, 'y' => 2];",
            ),
            (
                "$single = array_merge([5 => 'x']);",
                "$single = [...[5 => 'x']];",
            ),
            (
                "$twoLiterals = array_merge([5 => 'x'], [7 => 'y']);",
                "$twoLiterals = [...[5 => 'x'], ...[7 => 'y']];",
            ),
            (
                "$variableThenLiteral = array_merge($y, [5 => 'x']);",
                "$variableThenLiteral = [...$y, ...[5 => 'x']];",
            ),
            (
                "$mixedKeys = array_merge(['k' => 'v', 5 => 'x'], [5 => 'y']);",
                "$mixedKeys = [...['k' => 'v', 5 => 'x'], ...[5 => 'y']];",
            ),
            (
                "$legacy = array_merge(array(5 => 'x'));",
                "$legacy = [...array(5 => 'x')];",
            ),
            (
                "$merged = array_merge(['5' => 1], [KEY => 2], [$key => 3], [1.5 => 4], [true => 5], [null => 6]);",
                "$merged = [...['5' => 1], ...[KEY => 2], ...[$key => 3], ...[1.5 => 4], ...[true => 5], ...[null => 6]];",
            ),
            (
                "$merged = array_merge(['' => 0, '01' => 1, '-0' => 2, '+5' => 3, ' 5' => 4, '9223372036854775808' => 5]);",
                "$merged = ['' => 0, '01' => 1, '-0' => 2, '+5' => 3, ' 5' => 4, '9223372036854775808' => 5];",
            ),
            (
                "$merged = array_merge([1, 2], $b, [3, 4]);",
                "$merged = [1, 2, ...$b, 3, 4];",
            ),
            (
                "$merged = array_merge([1, 2,], $b);",
                "$merged = [1, 2, ...$b];",
            ),
            ("$merged = array_merge($a, []);", "$merged = [...$a];"),
            ("$merged = array_merge([], $a);", "$merged = [...$a];"),
            (
                "$merged = array_merge($a, [], $b);",
                "$merged = [...$a, ...$b];",
            ),
            (
                "$merged = array_merge($a, [], [], $b);",
                "$merged = [...$a, ...$b];",
            ),
            ("$merged = array_merge([]);", "$merged = [];"),
            ("$merged = array_merge([], []);", "$merged = [];"),
            ("$merged = array_merge($a, array());", "$merged = [...$a];"),
            (
                "$merged = array_merge($a, get_extras());",
                "$merged = [...$a, ...get_extras()];",
            ),
            (
                "namespace App;\n$merged = \\array_merge($a, $b);",
                "namespace App;\n$merged = [...$a, ...$b];",
            ),
        ] {
            assert_eq!(
                fix(&format!("<?php\n\n{from}\n")),
                format!("<?php\n\n{to}\n")
            );
        }
        // spread_operator_is_fine, call_with_unpacked_argument_is_not_reported
        for same in [
            "<?php\n\n$merged = [...$first, ...$second];\n",
            "<?php\n\n$merged = array_merge(...$arrays);\n",
        ] {
            assert_eq!(fix(same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php $x = array_merge(\n    $defaults,\n    $options,\n);"),
            "<?php $x = [\n    ...$defaults,\n    ...$options,\n];"
        );
        assert_eq!(
            fix("<?php $x = array_merge($a, [\n    'k' => 1,\n    'l' => 2,\n]);"),
            "<?php $x = [...$a,\n    'k' => 1,\n    'l' => 2,\n];"
        );
        assert_eq!(
            fix("<?php $x = array_merge(['k' => 1,], ['l' => 2,],);"),
            "<?php $x = ['k' => 1, 'l' => 2,];"
        );
        assert_eq!(
            fix("<?php $x = array_merge($a, array ( 1 ));"),
            "<?php $x = [...$a, 1];"
        );
        assert_eq!(
            fix("<?php $x = array_merge($a, array_merge($b, $c));"),
            "<?php $x = [...$a, ...$b, ...$c];"
        );
        assert_eq!(
            fix("<?php $x = array_merge($a, ['k' => fn ($v) => [1 => $v]]);"),
            "<?php $x = [...$a, 'k' => fn ($v) => [1 => $v]];"
        );
        assert_eq!(
            fix("<?php $x = array_merge($a, [\"b\\n\" => 1]);"),
            "<?php $x = [...$a, ...[\"b\\n\" => 1]];"
        );
        // A nested array's keys don't count; a computed key or a literal inside a larger expression is spread.
        assert_eq!(
            fix("<?php $x = array_merge($a, [[1 => 2]]);"),
            "<?php $x = [...$a, [1 => 2]];"
        );
        assert_eq!(
            fix("<?php $x = array_merge($a, [f(1) => 2]);"),
            "<?php $x = [...$a, ...[f(1) => 2]];"
        );
        assert_eq!(
            fix("<?php $x = array_merge($a, [1] + $b);"),
            "<?php $x = [...$a, ...[1] + $b];"
        );
        assert_eq!(
            fix("<?php $x = array_merge($a, ['a\\'b' => 1]);"),
            "<?php $x = [...$a, 'a\\'b' => 1];"
        );
        for same in [
            "<?php $x = array_merge(array: $a);",
            "<?php $f = array_merge(...);",
            "<?php $x = $obj->array_merge($a, $b);",
            "<?php $x = Foo::array_merge($a, $b);",
            "<?php $x = array_merge($a, /* keep */ $b);",
            "<?php function array_merge() {}",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
