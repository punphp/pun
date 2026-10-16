//! Port of Mago's `no-only` (Pest): removes `->only()` from `test()` / `it()` / `describe()` / hook chains.
//! Opt-in only (`Pun/no_only`). Unlike Mago, the whitespace before `->only()` goes too, so a chain broken
//! over lines doesn't leave a lone `;`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use std::path::Path;

const PEST_TEST_FUNCTIONS: &[&str] = &["test", "it", "describe", "todo", "beforeEach", "afterEach", "beforeAll", "afterAll"];

struct NoOnly;

pub fn fixer() -> Box<dyn Fixer> {
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
            let Some(arrow) = tokens.get_prev_meaningful_token(i).filter(|&a| tokens[a].is_kind(T_OBJECT_OPERATOR)) else { continue };
            let Some(open) = tokens.get_next_meaningful_token(i).filter(|&o| tokens[o].equals("(", true)) else { continue };
            let Some(close) = tokens.get_next_meaningful_token(open).filter(|&c| tokens[c].equals(")", true)) else { continue };
            if (arrow..close).any(|k| tokens[k].is_comment()) || !chain_root_is_pest_function(tokens, arrow) {
                continue;
            }
            let start = if tokens[arrow - 1].is_whitespace(None) { arrow - 1 } else { arrow };
            tokens.clear_range(start, close);
        }
    }
}

/// Walks `...->a()->b->` back to its root and checks it is a call to one of Pest's test functions.
fn chain_root_is_pest_function(tokens: &Tokens, arrow: usize) -> bool {
    let is_arrow = |k: usize| tokens[k].is_given_kind(&[T_OBJECT_OPERATOR, T_NULLSAFE_OBJECT_OPERATOR]);
    let Some(mut p) = tokens.get_prev_meaningful_token(arrow) else { return false };
    loop {
        let name = if tokens[p].equals(")", true) {
            let open = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, p);
            match tokens.get_prev_meaningful_token(open) {
                Some(n) if tokens[n].is_kind(T_STRING) => n,
                _ => return false,
            }
        } else if tokens[p].is_kind(T_STRING) {
            p
        } else {
            return false;
        };
        match tokens.get_prev_meaningful_token(name) {
            Some(b) if is_arrow(b) => match tokens.get_prev_meaningful_token(b) {
                Some(n) => p = n,
                None => return false,
            },
            before => {
                // The root must be a call, `name(...)`, of a global function.
                if p == name {
                    return false;
                }
                let is_function_call = before.is_none_or(|b| {
                    !tokens[b].is_given_kind(&[T_DOUBLE_COLON, T_FUNCTION, T_NEW])
                        && !(tokens[b].is_kind(T_NS_SEPARATOR)
                            && tokens.get_prev_meaningful_token(b).is_some_and(|q| tokens[q].is_given_kind(&[T_STRING, T_NAMESPACE])))
                });
                let n = tokens[name].get_content();
                return is_function_call && PEST_TEST_FUNCTIONS.iter().any(|f| n.eq_ignore_ascii_case(f.as_bytes()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    fn fix(code: &str) -> String {
        let mut tokens = pun_tokens::Tokens::from_code(code.as_bytes());
        super::fixer().fix(std::path::Path::new("t.php"), &mut tokens);
        String::from_utf8(tokens.generate_code()).unwrap()
    }

    #[test]
    fn mago_cases() {
        // test_with_only, it_with_only, describe_with_only, multiple_only
        assert_eq!(
            fix("<?php\n\ntest('example', function () {\n    expect(true)->toBeTrue();\n})->only();\n"),
            "<?php\n\ntest('example', function () {\n    expect(true)->toBeTrue();\n});\n"
        );
        assert_eq!(
            fix("<?php\n\nit('does something', function () {\n    expect(1)->toBe(1);\n})->only();\n"),
            "<?php\n\nit('does something', function () {\n    expect(1)->toBe(1);\n});\n"
        );
        assert_eq!(
            fix("<?php\n\ndescribe('group', function () {\n    test('test', function () {});\n})->only();\n"),
            "<?php\n\ndescribe('group', function () {\n    test('test', function () {});\n});\n"
        );
        assert_eq!(
            fix("<?php\n\ntest('first', function () {})->only();\nit('second', function () {})->only();\n"),
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
        assert_eq!(fix("<?php \\test('x', fn () => 1)->only();"), "<?php \\test('x', fn () => 1);");
        for same in [
            "<?php $request->only('name');",
            "<?php $collection->only();",
            "<?php collect($x)->only();",
            "<?php Foo::test('x')->only();",
            "<?php $this->it('x')->only();",
            "<?php test('x', fn () => 1)->only(/* keep */);",
            "<?php test('x', fn () => 1)->ONLY();",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
