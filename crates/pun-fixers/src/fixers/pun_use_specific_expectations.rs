//! Port of the exact-semantics subset of Mago's `use-specific-expectations` (Pest):
//! `expect($x)->toBe(true|false|null)` → `->toBeTrue()` / `->toBeFalse()` / `->toBeNull()`.
//! Opt-in only (`Pun/use_specific_expectations`). Mago's other rewrites change what a test asserts and are
//! not ported: `toEqual(true)` (loose) → `toBeTrue()` (strict), `toBe([])`/`toBe('')` → `toBeEmpty()` (looser),
//! and `not->toBeFalse()` → `toBeTrue()` (`expect(1)->not->toBeFalse()` passes, `expect(1)->toBeTrue()` fails).
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct UseSpecificExpectations;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(UseSpecificExpectations)
}

impl Fixer for UseSpecificExpectations {
    fn name(&self) -> &'static str {
        "Pun/use_specific_expectations"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_OBJECT_OPERATOR, T_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (1..tokens.count()).rev() {
            if !tokens[i].is_kind(T_STRING) || !tokens[i].get_content().eq_ignore_ascii_case(b"toBe") {
                continue;
            }
            let Some(arrow) = tokens.get_prev_meaningful_token(i).filter(|&a| tokens[a].is_kind(T_OBJECT_OPERATOR)) else { continue };
            let Some(open) = tokens.get_next_meaningful_token(i).filter(|&o| tokens[o].equals("(", true)) else { continue };
            let Some(arg) = tokens.get_next_meaningful_token(open).filter(|&a| tokens[a].is_kind(T_STRING)) else { continue };
            let Some(mut close) = tokens.get_next_meaningful_token(arg) else { continue };
            if tokens[close].equals(",", true) {
                let Some(c) = tokens.get_next_meaningful_token(close) else { continue };
                close = c;
            }
            if !tokens[close].equals(")", true) || (i..close).any(|k| tokens[k].is_comment()) {
                continue;
            }
            let method = match tokens[arg].get_content().to_ascii_lowercase().as_slice() {
                b"true" => "toBeTrue",
                b"false" => "toBeFalse",
                b"null" => "toBeNull",
                _ => continue,
            };
            if !chain_starts_with_expect(tokens, arrow) {
                continue;
            }
            tokens.override_range(i, close, vec![Token::new(T_STRING, method), Token::char("("), Token::char(")")]);
        }
    }
}

/// Walks `...->a()->b->` back to its root and checks it is a function call `expect(<arg>)`.
fn chain_starts_with_expect(tokens: &Tokens, arrow: usize) -> bool {
    let is_arrow = |k: usize| tokens[k].is_given_kind(&[T_OBJECT_OPERATOR, T_NULLSAFE_OBJECT_OPERATOR]);
    let Some(mut p) = tokens.get_prev_meaningful_token(arrow) else { return false };
    loop {
        if tokens[p].equals(")", true) {
            let open = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, p);
            let Some(name) = tokens.get_prev_meaningful_token(open) else { return false };
            if !tokens[name].is_given_kind(&[T_STRING, T_NAME_FULLY_QUALIFIED]) {
                return false;
            }
            let before = tokens.get_prev_meaningful_token(name);
            match before {
                // An arrow always has at least the open tag before it.
                Some(b) if is_arrow(b) => p = tokens.get_prev_meaningful_token(b).expect("open tag"),
                _ => {
                    let n = tokens[name].get_content();
                    let is_expect = n.eq_ignore_ascii_case(b"expect") || n.eq_ignore_ascii_case(b"\\expect");
                    let is_function_call = before.is_none_or(|b| !tokens[b].is_given_kind(&[T_DOUBLE_COLON, T_FUNCTION, T_NEW]));
                    let has_argument = tokens.get_next_meaningful_token(open).is_some_and(|a| a != p);
                    return is_expect && is_function_call && has_argument;
                }
            }
        } else if tokens[p].is_kind(T_STRING) {
            match tokens.get_prev_meaningful_token(p) {
                // An arrow always has at least the open tag before it.
                Some(b) if is_arrow(b) => p = tokens.get_prev_meaningful_token(b).expect("open tag"),
                _ => return false,
            }
        } else {
            return false;
        }
    }
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
        // to_be_true_literal, to_be_false_literal, to_be_null_literal, case_insensitive_to_be
        assert_eq!(fix("<?php\n\nexpect($value)->toBe(true);\n"), "<?php\n\nexpect($value)->toBeTrue();\n");
        assert_eq!(fix("<?php\n\nexpect($flag)->toBe(false);\n"), "<?php\n\nexpect($flag)->toBeFalse();\n");
        assert_eq!(fix("<?php\n\nexpect($value)->toBe(null);\n"), "<?php\n\nexpect($value)->toBeNull();\n");
        assert_eq!(fix("<?php\n\nexpect($value)->TOBE(true);\n"), "<?php\n\nexpect($value)->toBeTrue();\n");
        // to_be_literal_after_and
        assert_eq!(
            fix("<?php\n\nexpect($a)->toBeNull()->and($b)->toBe(true);\n"),
            "<?php\n\nexpect($a)->toBeNull()->and($b)->toBeTrue();\n"
        );
        // multiple_violations: only the exact-semantics ones are fixed.
        assert_eq!(
            fix("<?php\n\nexpect($a)->toBe(true);\nexpect($b)->toBe(false);\nexpect($c)->toBe(null);\nexpect($d)->toBe([]);\nexpect($e)->not->toBeFalse();\nexpect($f)->not->toBeTrue();\n"),
            "<?php\n\nexpect($a)->toBeTrue();\nexpect($b)->toBeFalse();\nexpect($c)->toBeNull();\nexpect($d)->toBe([]);\nexpect($e)->not->toBeFalse();\nexpect($f)->not->toBeTrue();\n"
        );
        // success cases, plus the deliberately unported (semantics-changing) failure cases
        for same in [
            "<?php\n\nexpect($value)->toBeTrue();\nexpect($flag)->toBeFalse();\n",
            "<?php\n\nexpect($value)->toBe(42);\nexpect($text)->toBe('hello');\nexpect($result)->toEqual($expected);\nexpect($array)->toBe([1, 2, 3]);\n",
            "<?php\n\nexpect($value)->toEqual(true);\nexpect($value)->ToEqual(null);\n",
            "<?php\n\nexpect($array)->toBe([]);\nexpect($string)->toBe('');\n",
            "<?php\n\nexpect($value)->not->toBeFalse();\n",
        ] {
            assert_eq!(fix(same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php expect($user)\n    ->not->toBeNull()\n    ->isAdmin->toBe( TRUE, );"),
            "<?php expect($user)\n    ->not->toBeNull()\n    ->isAdmin->toBeTrue();"
        );
        assert_eq!(fix("<?php \\expect($x)->not->toBe(null);"), "<?php \\expect($x)->not->toBeNull();");
        for same in [
            "<?php expect($x)->toBe(true, 'message');",
            "<?php $this->assertThat($x)->toBe(true);",
            "<?php Foo::expect($x)->toBe(true);",
            "<?php expect()->toBe(true);",
            "<?php $x->toBe(true);",
            "<?php expect($x)->toBe(\\true);",
            "<?php expect($x)->toBe(FOO);",
            "<?php $f($x)->toBe(true);",
            "<?php foo->toBe(true);",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
