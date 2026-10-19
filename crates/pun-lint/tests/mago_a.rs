//! Mago's own examples and test snippets for the lint-a rules ported from Mago 1.57.0's linter (copied from
//! mago-linter's rule files into tests/mago/<group>-<name>/Mago), and Pun's own inputs (Own/). Each `<case>.expect`
//! is what Mago 1.57.0 (`.oracle/mago`, `mago lint --only <code> --reporting-format json`) reported for the input:
//! one line per issue, `start end message` of its primary annotation. A `<case>.fixed` is the file after
//! `mago lint --fix --unsafe --potentially-unsafe`. Pun/ holds cases where Mago is wrong, with the expectation derived
//! by hand (each rule's module docs say why).

use std::path::{Path, PathBuf};

use pun_lint::{Linter, Rule, rules};

fn cases(dir: &str) -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/mago")
        .join(dir);
    let mut paths = Vec::new();
    for sub in std::fs::read_dir(&root).unwrap() {
        for entry in std::fs::read_dir(sub.unwrap().path()).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "php") {
                paths.push(path);
            }
        }
    }
    paths.sort();
    assert!(!paths.is_empty(), "no cases in {}", root.display());
    paths
}

fn run(rule: impl Rule + 'static, dir: &str) {
    let linter = Linter::new(vec![Box::new(rule)], 80400);
    for path in cases(dir) {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let input = std::fs::read_to_string(&path).unwrap();
        let expected = std::fs::read_to_string(path.with_extension("expect")).unwrap();
        let got: String = linter
            .check(&path, &input)
            .unwrap()
            .iter()
            .map(|p| format!("{} {} {}\n", p.span.start, p.span.end, p.message))
            .collect();
        assert_eq!(got, expected, "{dir}/{name}");
        if let Ok(fixed) = std::fs::read_to_string(path.with_extension("fixed")) {
            assert_eq!(
                linter.fix(&path, &input).unwrap().source,
                fixed,
                "{dir}/{name}: fix"
            );
        }
    }
}

#[test]
fn no_empty_catch() {
    run(rules::NoEmptyCatch, "control-flow-no-empty-catch");
}

#[test]
fn no_unsafe_finally() {
    run(rules::NoUnsafeFinally, "control-flow-no-unsafe-finally");
}

#[test]
fn no_assign_in_condition() {
    run(
        rules::NoAssignInCondition,
        "control-flow-no-assign-in-condition",
    );
}

#[test]
fn no_unused_label() {
    run(rules::NoUnusedLabel, "complexity-no-unused-label");
}

#[test]
fn no_redundant_write_visibility() {
    run(
        rules::NoRedundantWriteVisibility,
        "complexity-no-redundant-write-visibility",
    );
}

#[test]
fn no_iterator_to_array_in_foreach() {
    run(
        rules::NoIteratorToArrayInForeach,
        "complexity-no-iterator-to-array-in-foreach",
    );
}

#[test]
fn no_invalid_open_tag() {
    run(rules::NoInvalidOpenTag, "tags-no-invalid-open-tag");
}

#[test]
fn no_negated_condition() {
    run(rules::NoNegatedCondition, "refactor-no-negated-condition");
}

#[test]
fn no_negated_condition_fix() {
    // Mago only reports; the expected code is derived by hand from Mago's help: drop the `!` (or turn `!==` into
    // `===`, `!=` and `<>` into `==`) and swap the branches, keeping the parentheses the moved code needs.
    let linter = Linter::new(vec![Box::new(rules::NoNegatedCondition)], 80400);
    let input = "<?php\n\
        $a = !$b ? 1 : 2;\n\
        $c = $d !== null ? $d : 'x';\n\
        $e = $f != 1 ? 'a' : 'b';\n\
        $g = $h <> 1 ? 'a' : 'b';\n\
        $i = !$j ?: 3;\n\
        $k = !($l = 1) ? $n = 1 : 2;\n\
        $o = !$p = 1;\n\
        $r = $s !== 1 ? $t ? 1 : 2 : 3;\n\
        $u = $v ? 1 : (!$w ? 2 : 3);\n";
    let expected = "<?php\n\
        $a = $b ? 2 : 1;\n\
        $c = $d === null ? 'x' : $d;\n\
        $e = $f == 1 ? 'b' : 'a';\n\
        $g = $h == 1 ? 'b' : 'a';\n\
        $i = !$j ?: 3;\n\
        $k = ($l = 1) ? 2 : ($n = 1);\n\
        $o = !$p = 1;\n\
        $r = $s === 1 ? 3 : ($t ? 1 : 2);\n\
        $u = $v ? 1 : ($w ? 3 : 2);\n";
    assert_eq!(
        linter.fix(Path::new("a.php"), input).unwrap().source,
        expected
    );
}

#[test]
fn no_unused_global() {
    run(rules::NoUnusedGlobal, "dead-code-no-unused-global");
}

#[test]
fn no_unused_static() {
    run(rules::NoUnusedStatic, "dead-code-no-unused-static");
}

#[test]
fn no_duplicate_match_arm() {
    run(
        rules::NoDuplicateMatchArm,
        "complexity-no-duplicate-match-arm",
    );
}
