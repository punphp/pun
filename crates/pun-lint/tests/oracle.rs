//! Cases beyond Rector's fixtures. Each tests/oracle/*.php.inc is an input, `-----`, and what Rector 2.6.1
//! (`.oracle/rector-bin`, `rector process` with both rules and `withPhpVersion(80400)`) wrote for it.

use std::path::Path;

use pun_lint::{Linter, rules};

fn check_oracle(name: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/oracle")
        .join(name);
    let content = std::fs::read_to_string(&path).unwrap();
    let (input, expected) = content.split_once("-----\n").unwrap();
    let fixed = Linter::new(rules::all(), 80400).fix(&path, input).unwrap();
    assert_eq!(fixed.source, expected, "{name}");
}

#[test]
fn catch_variable_uses() {
    // Property declarations and `A::$e` are not uses; variable variables, include, compact() and `$this->$e` are.
    check_oracle("no_unused_catch_variable.php.inc");
}

#[test]
fn catch_variable_in_top_level_and_nested_lists() {
    check_oracle("no_unused_catch_variable_top_level.php.inc");
}

#[test]
fn compound_assignment() {
    check_oracle("prefer_compound_assignment.php.inc");
}

#[test]
fn compound_assignment_keeps_parentheses_around_low_precedence_operators() {
    // Rector writes `$a += $b or $c;`, which PHP reads as `($a += $b) or $c`. Pun keeps the meaning (hand-derived).
    let linter = Linter::new(rules::all(), 80400);
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
fn compact_with_a_missing_array_element() {
    // `compact([, 'e'])` is a compile error in PHP but parses; CompactFuncCallAnalyzer skips the null item and still
    // finds 'e' (hand-derived from its code).
    let linter = Linter::new(rules::all(), 80400);
    let code = "<?php\ntry { } catch (A $e) { compact([, 'e']); }\n";
    assert_eq!(linter.fix(Path::new("a.php"), code).unwrap().source, code);
}
