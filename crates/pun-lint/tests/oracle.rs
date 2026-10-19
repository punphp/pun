//! Cases beyond Rector's fixtures. Each tests/oracle/*.php.inc is an input, `-----`, and what Rector 2.6.1
//! (`.oracle/rector-bin`, `rector process` with the rule and `withPhpVersion(80400)`) wrote for it.

use std::path::Path;

use pun_lint::{Linter, Rule, rules};

/// The rule the oracle files below were written for.
fn spike_rules() -> Vec<Box<dyn Rule>> {
    vec![Box::new(rules::PreferCompoundAssignment)]
}

fn check_oracle(name: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/oracle")
        .join(name);
    let content = std::fs::read_to_string(&path).unwrap();
    let (input, expected) = content.split_once("-----\n").unwrap();
    let fixed = Linter::new(spike_rules(), 80400).fix(&path, input).unwrap();
    assert_eq!(fixed.source, expected, "{name}");
}

#[test]
fn compound_assignment() {
    check_oracle("prefer_compound_assignment.php.inc");
}

#[test]
fn compound_assignment_keeps_parentheses_around_low_precedence_operators() {
    // Rector writes `$a += $b or $c;`, which PHP reads as `($a += $b) or $c`. Pun keeps the meaning (hand-derived).
    let linter = Linter::new(spike_rules(), 80400);
    let fixed = linter
        .fix(
            Path::new("a.php"),
            "<?php\n$a = $a + ($b or $c);\n$a = $a . (($b xor $c));\n$a = $a - ($b and $c);\n",
        )
        .unwrap();
    assert_eq!(
        fixed.source,
        "<?php\n$a += ($b or $c);\n$a .= (($b xor $c));\n$a -= ($b and $c);\n"
    );
}

#[test]
fn unused_foreach_key_with_a_missing_array_element_in_compact() {
    // As above for the dead-code rules' copy of CompactFuncCallAnalyzer: the null item is skipped and 'k' is found,
    // so the key stays.
    let code = "<?php\nforeach ($a as $k => $v) { compact([, 'k']); }\n";
    assert_eq!(fix_with(rules::NoUnusedForeachKey, code), code);
}

/// Like `check_oracle`, for a file of one dead-code rule: Rector ran with only that rule's class (`withRules`) and
/// `withPhpVersion(80400)`.
fn check_rule_oracle(rule: impl Rule + 'static, name: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/oracle")
        .join(name);
    let content = std::fs::read_to_string(&path).unwrap();
    let (input, expected) = content.split_once("-----\n").unwrap();
    let fixed = Linter::new(vec![Box::new(rule)], 80400)
        .fix(&path, input)
        .unwrap();
    assert_eq!(fixed.source, expected, "{name}");
}

/// What one rule alone writes for `code` (hand-derived expectations, where Pun keeps the meaning Rector's printer
/// loses).
fn fix_with(rule: impl Rule + 'static, code: &str) -> String {
    Linter::new(vec![Box::new(rule)], 80400)
        .fix(Path::new("a.php"), code)
        .unwrap()
        .source
}

#[test]
fn and_true() {
    // `\true` and `TRUE`, kept parentheses where the place needs them, `and` (not `&&`), nested `&&` of trues.
    check_rule_oracle(rules::NoAndTrue, "no_and_true.php.inc");
}

#[test]
fn and_true_keeps_parentheses_that_hold_the_meaning() {
    // Rector writes `$x || $b ?? $c`, which PHP reads as `($x || $b) ?? $c`.
    assert_eq!(
        fix_with(rules::NoAndTrue, "<?php\n$a = $x || true && ($b ?? $c);\n"),
        "<?php\n$a = $x || ($b ?? $c);\n"
    );
}

#[test]
fn cast_in_concat() {
    // `(binary)`, both sides, parentheses around the cast, a cast on a call, `.=` (not a concat) and `(int)`.
    check_rule_oracle(rules::NoCastInConcat, "no_cast_in_concat.php.inc");
}

#[test]
fn cast_in_concat_keeps_parentheses_around_a_ternary() {
    // Rector writes `'a' . $b ? 'x' : 'y'`, which PHP reads as `('a' . $b) ? 'x' : 'y'`.
    assert_eq!(
        fix_with(
            rules::NoCastInConcat,
            "<?php\n$a = 'a' . (string) ($b ? 'x' : 'y');\n"
        ),
        "<?php\n$a = 'a' . ($b ? 'x' : 'y');\n"
    );
}

#[test]
fn useless_continue() {
    // Trailing comments (php-parser's Nop) keep the `continue`; `continue 0`, `continue (1)`, a `;` after it, and
    // alternative syntax.
    check_rule_oracle(rules::NoUselessContinue, "no_useless_continue.php.inc");
}

#[test]
fn empty_block() {
    // `;` bodies, comment-only bodies, alternative syntax, `else` without braces, every inverted condition, header
    // variables used later, a `try` block's catches as the next statements, a `case` list.
    check_rule_oracle(rules::NoEmptyBlock, "no_empty_block.php.inc");
}

#[test]
fn empty_block_keeps_parentheses_around_an_inverted_ternary() {
    // Rector writes `if (!$a ? $b : $c)`, which PHP reads as `if ((!$a) ? $b : $c)`.
    assert_eq!(
        fix_with(
            rules::NoEmptyBlock,
            "<?php\nif ($a ? $b : $c) {} else { x(); }\n"
        ),
        "<?php\nif (!($a ? $b : $c)) {\n    x();\n}\n"
    );
}

#[test]
fn empty_block_keeps_parentheses_around_an_assignment_in_a_comparison() {
    // Rector writes `if ($a = 1 !== $b)`, which PHP reads as `if ($a = (1 !== $b))`.
    assert_eq!(
        fix_with(
            rules::NoEmptyBlock,
            "<?php\nif (($a = 1) === $b) {} else { x(); }\n"
        ),
        "<?php\nif (($a = 1) !== $b) {\n    x();\n}\n"
    );
}

#[test]
fn useless_catch() {
    // A trailing comment moves with the statements and leaves an empty line, heredocs keep their indent, `throw
    // ($e)`, a catch without a variable, a multi-type catch, a `try` as an `if`'s only statement.
    check_rule_oracle(rules::NoUselessCatch, "no_useless_catch.php.inc");
}

#[test]
fn double_self_assignment() {
    check_rule_oracle(
        rules::NoDoubleSelfAssignment,
        "no_double_self_assignment.php.inc",
    );
}

#[test]
fn duplicate_array_key() {
    // Comments go with their item, `'a'` and `"a"` are different keys, class constants, `static::`, negative
    // numbers, constants, destructuring, interpolated keys, magic constants, floats.
    check_rule_oracle(rules::NoDuplicateArrayKey, "no_duplicate_array_key.php.inc");
}

#[test]
fn null_property_init() {
    check_rule_oracle(rules::NoNullPropertyInit, "no_null_property_init.php.inc");
}

#[test]
fn null_property_init_in_parentheses() {
    // Rector's printer fails on `= (null)` ("Encountered unexpected token") and leaves the file; the default is
    // still null (hand-derived).
    assert_eq!(
        fix_with(
            rules::NoNullPropertyInit,
            "<?php\nclass A { public $h = (null); }\n"
        ),
        "<?php\nclass A { public $h; }\n"
    );
}

#[test]
fn redundant_final_const() {
    check_rule_oracle(
        rules::NoRedundantFinalConst,
        "no_redundant_final_const.php.inc",
    );
}

#[test]
fn unused_closure_variable() {
    check_rule_oracle(
        rules::NoUnusedClosureVariable,
        "no_unused_closure_variable.php.inc",
    );
}

#[test]
fn unused_foreach_key() {
    // `@var` tags for the key go, the doc comment too when nothing else is in it.
    check_rule_oracle(rules::NoUnusedForeachKey, "no_unused_foreach_key.php.inc");
}

#[test]
fn return_in_constructor() {
    // A one-line block and a `case` are written again a statement per line, an `if` without braces gets them.
    check_rule_oracle(
        rules::NoReturnInConstructor,
        "no_return_in_constructor.php.inc",
    );
}

#[test]
fn return_in_constructor_in_parentheses() {
    // Rector writes `return ();`, a syntax error (hand-derived).
    assert_eq!(
        fix_with(
            rules::NoReturnInConstructor,
            "<?php\nclass A {\n    public function __construct() {\n        return (1);\n    }\n}\n"
        ),
        "<?php\nclass A {\n    public function __construct() {\n        return;\n    }\n}\n"
    );
}

#[test]
fn useless_block() {
    check_rule_oracle(rules::NoUselessBlock, "no_useless_block.php.inc");
}

#[test]
fn empty_array_identical_on_a_parameter_written_later() {
    // PHPStan's scope keeps a parameter's native type until a write; one after the check (outside a loop) leaves it.
    check_rule_oracle(
        rules::PreferEmptyArrayIdentical,
        "prefer_empty_array_identical.php.inc",
    );
}

#[test]
fn arrow_function_return_type_of_a_member_of_an_unknown_class() {
    // Mago types a member of `?Unknown` as `null` and of `new Unknown()` as `never`; PHPStan knows no type for either.
    check_rule_oracle(
        rules::AddArrowFunctionReturn,
        "add_arrow_function_return_type_unknown_class.php.inc",
    );
}

#[test]
fn statement_padding_skips_a_file_with_inline_html() {
    // NewlineAfterStatementRector is HTML-averse: the `if` would get its blank line without the `<p>` after `?>`.
    check_rule_oracle(
        rules::RequireStatementPadding,
        "require_statement_padding_html.php.inc",
    );
}
