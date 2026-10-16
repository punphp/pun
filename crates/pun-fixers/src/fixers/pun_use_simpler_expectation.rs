//! Port of Mago's `use-simpler-expectation` (Pest): `expect($a > $b)->toBeTrue()` → `expect($a)->toBeGreaterThan($b)`,
//! likewise `>=`, `<`, `<=`, `===` (`toBe`), `!==` (`not->toBe`) and `instanceof` (`toBeInstanceOf(X::class)`).
//! Opt-in only (`Pun/use_simpler_expectation`). Only fixes when the comparison is the whole `expect()` argument
//! and the matcher follows `expect()` directly. Not ported:
//! - `expect(!$x)->toBeTrue()` → `expect($x)->toBeFalse()` (and the `toBeFalse` mirror): `!$x` is true for every
//!   falsy value, `toBeFalse()` only for `false`.
//! - matchers after `->not` or `->and()`: Mago's fix drops the `not` / rewrites the wrong expectation.
//! - `instanceof $var`: `$var::class` fails when `$var` holds a class-name string.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct UseSimplerExpectation;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(UseSimplerExpectation)
}

impl Fixer for UseSimplerExpectation {
    fn name(&self) -> &'static str {
        "Pun/use_simpler_expectation"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_OBJECT_OPERATOR, T_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (1..tokens.count()).rev() {
            if !tokens[i].is_kind(T_STRING) || !tokens[i].get_content().eq_ignore_ascii_case(b"toBeTrue") {
                continue;
            }
            let Some((expect_open, expect_close, close)) = match_expect_to_be_true(tokens, i) else { continue };
            if (expect_open..=close).any(|k| tokens[k].is_comment()) {
                continue;
            }
            let Some((op, matcher)) = single_comparison(tokens, expect_open, expect_close) else { continue };
            let lhs_end = tokens.get_prev_meaningful_token(op).unwrap();
            let rhs_start = tokens.get_next_meaningful_token(op).unwrap();
            let mut rhs_end = tokens.get_prev_meaningful_token(expect_close).unwrap();
            if tokens[rhs_end].equals(",", true) {
                rhs_end = tokens.get_prev_meaningful_token(rhs_end).unwrap();
            }
            let instanceof = tokens[op].is_kind(T_INSTANCEOF);
            if instanceof && !(rhs_start..=rhs_end).all(|k| tokens[k].is_given_kind(&[T_STRING, T_NS_SEPARATOR, T_STATIC]) || tokens[k].is_whitespace(None)) {
                continue;
            }
            let mut out = vec![Token::char(")"), Token::new(T_OBJECT_OPERATOR, "->")];
            if tokens[op].is_kind(T_IS_NOT_IDENTICAL) {
                out.extend([Token::new(T_STRING, "not"), Token::new(T_OBJECT_OPERATOR, "->")]);
            }
            out.extend([Token::new(T_STRING, matcher), Token::char("(")]);
            out.extend((rhs_start..=rhs_end).map(|k| tokens[k].clone()));
            if instanceof {
                out.extend([Token::new(T_DOUBLE_COLON, "::"), Token::new(CT_CLASS_CONSTANT, "class")]);
            }
            out.push(Token::char(")"));
            tokens.override_range(lhs_end + 1, close, out);
        }
    }
}

/// Matches `expect(<arg>)->toBeTrue()` with `toBeTrue` at `i`; returns (expect `(`, expect `)`, matcher `)`).
fn match_expect_to_be_true(tokens: &Tokens, i: usize) -> Option<(usize, usize, usize)> {
    let arrow = tokens.get_prev_meaningful_token(i).filter(|&a| tokens[a].is_kind(T_OBJECT_OPERATOR))?;
    let open = tokens.get_next_meaningful_token(i).filter(|&o| tokens[o].equals("(", true))?;
    let close = tokens.get_next_meaningful_token(open).filter(|&c| tokens[c].equals(")", true))?;
    let p = tokens.get_prev_meaningful_token(arrow).filter(|&p| tokens[p].equals(")", true))?;
    let expect_open = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, p);
    let name = tokens.get_prev_meaningful_token(expect_open).filter(|&n| tokens[n].is_kind(T_STRING))?;
    if !tokens[name].get_content().eq_ignore_ascii_case(b"expect") {
        return None;
    }
    let is_function_call = tokens.get_prev_meaningful_token(name).is_none_or(|b| {
        !tokens[b].is_given_kind(&[T_OBJECT_OPERATOR, T_NULLSAFE_OBJECT_OPERATOR, T_DOUBLE_COLON, T_FUNCTION, T_NEW])
            && !(tokens[b].is_kind(T_NS_SEPARATOR)
                && tokens.get_prev_meaningful_token(b).is_some_and(|q| tokens[q].is_given_kind(&[T_STRING, T_NAMESPACE])))
    });
    is_function_call.then_some((expect_open, p, close))
}

/// The one top-level comparison in `expect(...)` when it is the root of the argument expression: no other
/// comparison and no operator of lower precedence (`==`, bitwise, logical, `??`, ternary, assignment, ...).
fn single_comparison(tokens: &Tokens, open: usize, close: usize) -> Option<(usize, &'static str)> {
    let mut found = None;
    let mut has_not = false;
    let mut k = open + 1;
    while k < close {
        let t = &tokens[k];
        if let Some((bt, true)) = Tokens::detect_block_type(t) {
            k = tokens.find_block_end(bt, k) + 1;
            continue;
        }
        let matcher = if t.is_kind(T_IS_IDENTICAL) {
            Some("toBe")
        } else if t.is_kind(T_IS_NOT_IDENTICAL) {
            Some("toBe")
        } else if t.is_kind(T_IS_GREATER_OR_EQUAL) {
            Some("toBeGreaterThanOrEqual")
        } else if t.is_kind(T_IS_SMALLER_OR_EQUAL) {
            Some("toBeLessThanOrEqual")
        } else if t.equals(">", true) {
            Some("toBeGreaterThan")
        } else if t.equals("<", true) {
            Some("toBeLessThan")
        } else if t.is_kind(T_INSTANCEOF) {
            Some("toBeInstanceOf")
        } else {
            None
        };
        if let Some(m) = matcher {
            if found.is_some() {
                return None;
            }
            found = Some((k, m));
        } else if t.equals("!", true) {
            has_not = true;
        } else if t.is_given_kind(&[
            T_IS_EQUAL, T_IS_NOT_EQUAL, T_SPACESHIP, T_BOOLEAN_AND, T_BOOLEAN_OR, T_LOGICAL_AND, T_LOGICAL_OR, T_LOGICAL_XOR,
            T_COALESCE, T_COALESCE_EQUAL, T_PLUS_EQUAL, T_MINUS_EQUAL, T_MUL_EQUAL, T_DIV_EQUAL, T_CONCAT_EQUAL, T_MOD_EQUAL,
            T_AND_EQUAL, T_OR_EQUAL, T_XOR_EQUAL, T_SL_EQUAL, T_SR_EQUAL, T_POW_EQUAL, T_DOUBLE_ARROW, T_FN, T_FUNCTION,
            T_PRINT, T_YIELD, T_YIELD_FROM, T_THROW, T_INCLUDE, T_INCLUDE_ONCE, T_REQUIRE, T_REQUIRE_ONCE, T_PIPE, T_ELLIPSIS,
            CT_NAMED_ARGUMENT_NAME, T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG, T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG,
        ]) || t.equals_any(&["=".into(), "&".into(), "|".into(), "^".into(), "?".into(), ":".into()], true)
            || (t.equals(",", true) && tokens.get_next_meaningful_token(k) != Some(close))
        {
            return None;
        }
        k += 1;
    }
    // `!$a instanceof B` is `!($a instanceof B)`.
    found.filter(|&(op, _)| !(has_not && tokens[op].is_kind(T_INSTANCEOF)))
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
            ("expect($a > $b)->toBeTrue();", "expect($a)->toBeGreaterThan($b);"),
            ("expect($a >= $b)->toBeTrue();", "expect($a)->toBeGreaterThanOrEqual($b);"),
            ("expect($a < $b)->toBeTrue();", "expect($a)->toBeLessThan($b);"),
            ("expect($a <= $b)->toBeTrue();", "expect($a)->toBeLessThanOrEqual($b);"),
            ("expect($a === $b)->toBeTrue();", "expect($a)->toBe($b);"),
            ("expect($a !== $b)->toBeTrue();", "expect($a)->not->toBe($b);"),
            ("expect($obj instanceof ClassName)->toBeTrue();", "expect($obj)->toBeInstanceOf(ClassName::class);"),
            ("expect($obj instanceof \\Namespace\\ClassName)->toBeTrue();", "expect($obj)->toBeInstanceOf(\\Namespace\\ClassName::class);"),
        ] {
            assert_eq!(fix(&format!("<?php\n\n{from}\n")), format!("<?php\n\n{to}\n"));
        }
        // multiple_violations: the negation and range check have no exact fix.
        assert_eq!(
            fix("<?php\n\nexpect(!$a)->toBeTrue();\nexpect($a > $b)->toBeTrue();\nexpect($a === $b)->toBeTrue();\nexpect($obj instanceof Foo)->toBeTrue();\nexpect($x >= 1 && $x <= 10)->toBeTrue();\n"),
            "<?php\n\nexpect(!$a)->toBeTrue();\nexpect($a)->toBeGreaterThan($b);\nexpect($a)->toBe($b);\nexpect($obj)->toBeInstanceOf(Foo::class);\nexpect($x >= 1 && $x <= 10)->toBeTrue();\n"
        );
        for same in [
            // direct_expectations, dedicated_matchers, non_pattern_expressions
            "<?php\n\nexpect($condition)->toBeTrue();\nexpect($condition)->toBeFalse();\n",
            "<?php\n\nexpect($a)->toBeGreaterThan($b);\nexpect($a)->not->toBe($b);\nexpect($obj)->toBeInstanceOf(ClassName::class);\nexpect($value)->toBeBetween(1, 10);\n",
            "<?php\n\nexpect($a && $b)->toBeTrue();\nexpect($a || $b)->toBeFalse();\nexpect($a == $b)->toBeTrue();\nexpect($a != $b)->toBeTrue();\n",
            // negated_to_be_true / negated_to_be_false / range checks: Mago fixes or reports, Pun leaves them.
            "<?php\n\nexpect(!$condition)->toBeTrue();\nexpect(!$condition)->ToBeFalse();\nexpect($x <= 10 && $x >= 1)->toBeTrue();\n",
        ] {
            assert_eq!(fix(same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(fix("<?php expect(count($a) + 1 >= $b->max(1, 2),)->TOBETRUE();"), "<?php expect(count($a) + 1)->toBeGreaterThanOrEqual($b->max(1, 2));");
        assert_eq!(fix("<?php expect($u->role === Role::Admin)->toBeTrue();"), "<?php expect($u->role)->toBe(Role::Admin);");
        assert_eq!(fix("<?php expect($x instanceof static)->toBeTrue();"), "<?php expect($x)->toBeInstanceOf(static::class);");
        assert_eq!(fix("<?php expect(!$a === $b)->toBeTrue();"), "<?php expect(!$a)->toBe($b);");
        assert_eq!(fix("<?php \\expect($a > $b)->toBeTrue();"), "<?php \\expect($a)->toBeGreaterThan($b);");
        for same in [
            "<?php expect($a > $b)->not->toBeTrue();",
            "<?php expect($x)->and($a > $b)->toBeTrue();",
            "<?php expect($a > $b === true)->toBeTrue();",
            "<?php expect($a > $b ? 1 : 2)->toBeTrue();",
            "<?php expect($a === $b ?? $c)->toBeTrue();",
            "<?php expect($a === $b & $c)->toBeTrue();",
            "<?php expect($a > $b, 'msg')->toBeTrue();",
            "<?php expect($x instanceof $class)->toBeTrue();",
            "<?php expect(!$x instanceof Foo)->toBeTrue();",
            "<?php expect($a /* c */ > $b)->toBeTrue();",
            "<?php expect(fn () => $a > $b)->toBeTrue();",
            "<?php expect($a > $b)->toBeTrue('message');",
            "<?php $this->expect($a > $b)->toBeTrue();",
            "<?php Foo\\expect($a > $b)->toBeTrue();",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
