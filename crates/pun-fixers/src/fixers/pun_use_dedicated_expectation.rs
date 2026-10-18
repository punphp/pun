//! Port of Mago's `use-dedicated-expectation` (Pest): `expect(is_array($x))->toBeTrue()` → `expect($x)->toBeArray()`,
//! `expect(str_starts_with($s, 'a'))->toBeTrue()` → `expect($s)->toStartWith('a')`.
//! Opt-in only (`Pun/use_dedicated_expectation`). Risky: `toStartWith`/`toEndWith`/`toBeFile`/`toBeDirectory`
//! also require a string and `toHaveProperty` an object, where the PHP functions coerce or accept class names.
//! Not ported (the matcher means something else): `is_file` → `toBeFile` (that is `file_exists`, true for
//! directories), `is_readable`/`is_writable` → `toBeReadable`/`toBeWritable` (no such Pest matchers),
//! `ctype_alpha`/`ctype_alnum` (Pest casts to string first: `ctype_alpha(65)` is true, `toBeAlpha()` of 65 is not),
//! `is_resource` (PHPUnit also accepts closed resources), and `in_array`/`array_key_exists` (Mago has no fix either).
use crate::analyzers::pest::match_expect_matcher;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

const SINGLE_ARG: &[(&str, &str)] = &[
    ("is_array", "toBeArray"),
    ("is_string", "toBeString"),
    ("is_int", "toBeInt"),
    ("is_integer", "toBeInt"),
    ("is_float", "toBeFloat"),
    ("is_double", "toBeFloat"),
    ("is_bool", "toBeBool"),
    ("is_numeric", "toBeNumeric"),
    ("is_callable", "toBeCallable"),
    ("is_iterable", "toBeIterable"),
    ("is_object", "toBeObject"),
    ("is_scalar", "toBeScalar"),
    ("is_null", "toBeNull"),
    ("is_dir", "toBeDirectory"),
    ("file_exists", "toBeFile"),
];
const TWO_ARGS: &[(&str, &str)] = &[
    ("str_starts_with", "toStartWith"),
    ("str_ends_with", "toEndWith"),
    ("property_exists", "toHaveProperty"),
];

struct UseDedicatedExpectation;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(UseDedicatedExpectation)
}

impl Fixer for UseDedicatedExpectation {
    fn name(&self) -> &'static str {
        "Pun/use_dedicated_expectation"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_OBJECT_OPERATOR, T_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (1..tokens.count()).rev() {
            if !tokens[i].is_kind(T_STRING)
                || !tokens[i].get_content().eq_ignore_ascii_case(b"toBeTrue")
            {
                continue;
            }
            let Some(m) = match_expect_matcher(tokens, i) else {
                continue;
            };
            // The expect argument must be exactly one global call `f(...)`.
            let mut fname = tokens.get_next_meaningful_token(m.expect_open).unwrap();
            let fstart = fname;
            if tokens[fname].is_kind(T_NS_SEPARATOR) {
                fname = tokens.get_next_meaningful_token(fname).unwrap();
            }
            if !tokens[fname].is_kind(T_STRING) {
                continue;
            }
            let Some(fopen) = tokens
                .get_next_meaningful_token(fname)
                .filter(|&o| tokens[o].equals("(", true))
            else {
                continue;
            };
            let fclose = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, fopen);
            let mut after = tokens.get_next_meaningful_token(fclose).unwrap();
            if tokens[after].equals(",", true) {
                after = tokens.get_next_meaningful_token(after).unwrap();
            }
            if after != m.expect_close || (fstart..=m.close).any(|k| tokens[k].is_comment()) {
                continue;
            }
            let Some(args) = plain_arguments(tokens, fopen, fclose) else {
                continue;
            };
            let name = tokens[fname].get_content().to_ascii_lowercase();
            let lookup = |table: &[(&str, &'static str)]| {
                table
                    .iter()
                    .find(|(f, _)| f.as_bytes() == name)
                    .map(|(_, m)| *m)
            };
            let mut not_tokens = vec![];
            if m.has_not {
                not_tokens = vec![
                    Token::new(T_OBJECT_OPERATOR, "->"),
                    Token::new(T_STRING, "not"),
                ];
            }
            if let (Some(matcher), 1) = (lookup(SINGLE_ARG), args.len()) {
                tokens.override_range(
                    i,
                    m.close,
                    vec![
                        Token::new(T_STRING, matcher),
                        Token::char("("),
                        Token::char(")"),
                    ],
                );
                tokens.clear_at(fclose);
                tokens.clear_range(fstart, fopen);
            } else if let (Some(matcher), 2) = (lookup(TWO_ARGS), args.len()) {
                if matcher == "toHaveProperty"
                    && tokens[args[0].0].is_given_kind(&[
                        T_CONSTANT_ENCAPSED_STRING,
                        T_STRING,
                        T_NS_SEPARATOR,
                    ])
                {
                    continue; // `property_exists(Foo::class, ...)`: toHaveProperty() needs an object
                }
                let mut out: Vec<Token> =
                    (args[0].0..=args[0].1).map(|k| tokens[k].clone()).collect();
                out.push(Token::char(")"));
                out.extend(not_tokens);
                out.extend([
                    Token::new(T_OBJECT_OPERATOR, "->"),
                    Token::new(T_STRING, matcher),
                    Token::char("("),
                ]);
                out.extend((args[1].0..=args[1].1).map(|k| tokens[k].clone()));
                out.push(Token::char(")"));
                tokens.override_range(fstart, m.close, out);
            }
        }
    }
}

/// (first, last) meaningful token of each argument; `None` for spread or named arguments.
fn plain_arguments(tokens: &Tokens, open: usize, close: usize) -> Option<Vec<(usize, usize)>> {
    let mut args = vec![];
    let mut k = tokens.get_next_meaningful_token(open)?;
    while k < close {
        let start = k;
        let mut last = k;
        while k < close && !tokens[k].equals(",", true) {
            if let Some((t, true)) = Tokens::detect_block_type(&tokens[k]) {
                k = tokens.find_block_end(t, k);
            }
            last = k;
            k = tokens.get_next_meaningful_token(k)?;
        }
        if tokens[start].is_given_kind(&[T_ELLIPSIS, CT_NAMED_ARGUMENT_NAME]) {
            return None;
        }
        args.push((start, last));
        if k < close {
            k = tokens.get_next_meaningful_token(k)?;
        }
    }
    Some(args)
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
                "expect(is_array($value))->toBeTrue();",
                "expect($value)->toBeArray();",
            ),
            (
                "expect(is_string($value))->toBeTrue();",
                "expect($value)->toBeString();",
            ),
            (
                "expect(is_int($value))->toBeTrue();",
                "expect($value)->toBeInt();",
            ),
            (
                "expect(is_integer($value))->toBeTrue();",
                "expect($value)->toBeInt();",
            ),
            (
                "expect(is_null($value))->toBeTrue();",
                "expect($value)->toBeNull();",
            ),
            (
                "expect(str_starts_with($string, 'prefix'))->toBeTrue();",
                "expect($string)->toStartWith('prefix');",
            ),
            (
                "expect(str_ends_with($string, 'suffix'))->toBeTrue();",
                "expect($string)->toEndWith('suffix');",
            ),
            (
                "expect(is_dir($path))->toBeTrue();",
                "expect($path)->toBeDirectory();",
            ),
            (
                "expect(file_exists($path))->toBeTrue();",
                "expect($path)->toBeFile();",
            ),
            (
                "expect(property_exists($obj, 'name'))->toBeTrue();",
                "expect($obj)->toHaveProperty('name');",
            ),
            (
                "expect(is_array($value))->TOBETRUE();",
                "expect($value)->toBeArray();",
            ),
        ] {
            assert_eq!(
                fix(&format!("<?php\n\n{from}\n")),
                format!("<?php\n\n{to}\n")
            );
        }
        // multiple_violations: in_array and is_file are reported by Mago but have no exact matcher.
        assert_eq!(
            fix(
                "<?php\n\nexpect(is_array($a))->toBeTrue();\nexpect(is_string($b))->toBeTrue();\nexpect(str_starts_with($c, 'x'))->toBeTrue();\nexpect(in_array($d, $arr))->toBeTrue();\nexpect(is_file($e))->toBeTrue();\n"
            ),
            "<?php\n\nexpect($a)->toBeArray();\nexpect($b)->toBeString();\nexpect($c)->toStartWith('x');\nexpect(in_array($d, $arr))->toBeTrue();\nexpect(is_file($e))->toBeTrue();\n"
        );
        // dedicated_matchers, non_matching_function_calls, and the unported Mago fixes
        for same in [
            "<?php\n\nexpect($value)->toBeArray();\nexpect($string)->toStartWith('prefix');\nexpect($obj)->toHaveProperty('name');\n",
            "<?php\n\nexpect(strlen($s) > 0)->toBeTrue();\nexpect(custom_check($x))->toBeTrue();\nexpect(count($array) > 0)->toBeTrue();\n",
            "<?php\n\nexpect(ctype_alpha($string))->toBeTrue();\nexpect(array_key_exists('key', $array))->toBeTrue();\nexpect(is_readable($path))->toBeTrue();\n",
        ] {
            assert_eq!(fix(same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php expect(\\is_array( $x ))->not->toBeTrue();"),
            "<?php expect( $x )->not->toBeArray();"
        );
        assert_eq!(
            fix("<?php expect(str_ends_with($user->name, \"son\"),)->not->toBeTrue();"),
            "<?php expect($user->name)->not->toEndWith(\"son\");"
        );
        assert_eq!(
            fix("<?php expect(is_callable([$a, 'b']))->toBeTrue();"),
            "<?php expect([$a, 'b'])->toBeCallable();"
        );
        assert_eq!(
            fix("<?php \\expect(is_array($x))->toBeTrue();"),
            "<?php \\expect($x)->toBeArray();"
        );
        for same in [
            "<?php expect(is_callable($x, true))->toBeTrue();",
            "<?php expect(is_array($x), 'msg')->toBeTrue();",
            "<?php expect(is_array($x) && $y)->toBeTrue();",
            "<?php expect(is_array($x))->toBeFalse();",
            "<?php expect(is_array($x))->toBeTrue('msg');",
            "<?php expect($a)->and(is_array($x))->toBeTrue();",
            "<?php expect(is_array(...$x))->toBeTrue();",
            "<?php expect(is_array(value: $x))->toBeTrue();",
            "<?php expect(Foo::is_array($x))->toBeTrue();",
            "<?php expect($o->is_array($x))->toBeTrue();",
            "<?php expect(is_array(/* c */ $x))->toBeTrue();",
            "<?php expect(property_exists(Foo::class, 'x'))->toBeTrue();",
            "<?php expect(property_exists('Foo', 'x'))->toBeTrue();",
            "<?php $this->expect(is_array($x))->toBeTrue();",
            "<?php Foo\\expect(is_array($x))->toBeTrue();",
            "<?php $e->toBeTrue();",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
