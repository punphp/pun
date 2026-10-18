//! Port of Mago's `no-only` (Pest): removes `->only()` from `test()` / `it()` / `describe()` / hook chains.
//! Opt-in only (`Pun/no_only`). Unlike Mago, the whitespace before `->only()` goes too, so a chain broken
//! over lines doesn't leave a lone `;`.
use crate::analyzers::pest::chain_root_call;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use std::path::Path;

const PEST_TEST_FUNCTIONS: &[&str] = &[
    "test",
    "it",
    "describe",
    "todo",
    "beforeEach",
    "afterEach",
    "beforeAll",
    "afterAll",
];

struct NoOnly;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoOnly)
}

impl Fixer for NoOnly {
    fn name(&self) -> &'static str {
        "Pun/no_only"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_OBJECT_OPERATOR, T_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (1..tokens.count()).rev() {
            if !tokens[i].is_kind(T_STRING) || tokens[i].get_content() != b"only" {
                continue;
            }
            let Some(arrow) = tokens
                .get_prev_meaningful_token(i)
                .filter(|&a| tokens[a].is_kind(T_OBJECT_OPERATOR))
            else {
                continue;
            };
            let Some(open) = tokens
                .get_next_meaningful_token(i)
                .filter(|&o| tokens[o].equals("(", true))
            else {
                continue;
            };
            let Some(close) = tokens
                .get_next_meaningful_token(open)
                .filter(|&c| tokens[c].equals(")", true))
            else {
                continue;
            };
            if (arrow..close).any(|k| tokens[k].is_comment())
                || !chain_root_is_pest_function(tokens, arrow)
            {
                continue;
            }
            let start = if tokens[arrow - 1].is_whitespace(None) {
                arrow - 1
            } else {
                arrow
            };
            tokens.clear_range(start, close);
        }
    }
}

/// Whether the chain ending at `arrow` starts with a call to one of Pest's test functions.
fn chain_root_is_pest_function(tokens: &Tokens, arrow: usize) -> bool {
    chain_root_call(tokens, arrow).is_some_and(|(name, _, _)| {
        let n = tokens[name].get_content();
        PEST_TEST_FUNCTIONS
            .iter()
            .any(|f| n.eq_ignore_ascii_case(f.as_bytes()))
    })
}

#[cfg(test)]
mod tests {
    fn fix(code: &str) -> String {
        crate::fix_with(&*super::fixer(), code)
    }

    #[test]
    fn mago_cases() {
        // test_with_only, it_with_only, describe_with_only, multiple_only
        assert_eq!(
            fix(
                "<?php\n\ntest('example', function () {\n    expect(true)->toBeTrue();\n})->only();\n"
            ),
            "<?php\n\ntest('example', function () {\n    expect(true)->toBeTrue();\n});\n"
        );
        assert_eq!(
            fix(
                "<?php\n\nit('does something', function () {\n    expect(1)->toBe(1);\n})->only();\n"
            ),
            "<?php\n\nit('does something', function () {\n    expect(1)->toBe(1);\n});\n"
        );
        assert_eq!(
            fix(
                "<?php\n\ndescribe('group', function () {\n    test('test', function () {});\n})->only();\n"
            ),
            "<?php\n\ndescribe('group', function () {\n    test('test', function () {});\n});\n"
        );
        assert_eq!(
            fix(
                "<?php\n\ntest('first', function () {})->only();\nit('second', function () {})->only();\n"
            ),
            "<?php\n\ntest('first', function () {});\nit('second', function () {});\n"
        );
        // test_without_only
        let same = "<?php\n\ntest('example', function () {\n    expect(true)->toBeTrue();\n});\n\nit('does something', function () {\n    expect(1)->toBe(1);\n});\n";
        assert_eq!(fix(same), same);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php it('x', fn () => true)\n    ->group('a')\n    ->only()\n    ->skip();"),
            "<?php it('x', fn () => true)\n    ->group('a')\n    ->skip();"
        );
        assert_eq!(
            fix("<?php \\test('x', fn () => 1)->only();"),
            "<?php \\test('x', fn () => 1);"
        );
        for same in [
            "<?php $request->only('name');",
            "<?php $collection->only();",
            "<?php collect($x)->only();",
            "<?php Foo::test('x')->only();",
            "<?php $this->it('x')->only();",
            "<?php test('x', fn () => 1)->only(/* keep */);",
            "<?php test('x', fn () => 1)->ONLY();",
            "<?php $f()->only();",
            "<?php test->only();",
            "<?php only(); $a->b;",
            "<?php test('x')->only;",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
