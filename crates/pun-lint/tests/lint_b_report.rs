//! The report rules of lint-b's groups that come from Mago's linter and from phpstan-strict-rules.
//!
//! tests/lint_b_report/<group>/<name>/*.php.inc is an input, `-----`, and what the upstream tool reported for it,
//! one problem a line. For a Mago rule the inputs are the rule's own test cases and examples (mago-linter 1.57.0,
//! rule/<category>/<rule>.rs) plus hand-written ones, and the lines are `line:column: message` as
//! `mago --php-version 8.4 lint --only <code> --reporting-format emacs` prints them. For a PHPStan rule the lines are
//! `line: message` as PHPStan 2.3.1 with phpstan-strict-rules 2.0.12 (level 0, only that strict rule on) reports
//! them with `--error-format=json`, filtered to the rule's identifier.
//!
//! tests/lint_b_report_fix/<group>/<name>/*.php.inc is an input and, after `-----`, what
//! `mago --php-version 8.4 lint --only <code> --fix` wrote for it (one case a file).

use std::path::Path;

use pun_lint::{Linter, rules};

/// The rules whose expected lines come from PHPStan, which reports a line and no column.
const PHPSTAN: &[&str] = &[
    "lint/style/no-empty",
    "lint/style/no-short-ternary",
    "lint/style/no-variable-variable",
    "lint/classes/no-explicit-constructor-call",
];

fn line_col(source: &str, offset: usize) -> (usize, usize) {
    let before = &source[..offset];
    let line = before.matches('\n').count() + 1;
    let col = before[before.rfind('\n').map_or(0, |i| i + 1)..]
        .chars()
        .count()
        + 1;
    (line, col)
}

fn check(id: &str, dir: &Path) {
    let rule = rules::all()
        .into_iter()
        .find(|r| r.id() == id)
        .unwrap_or_else(|| panic!("{id} is not registered"));
    let linter = Linter::new(vec![rule], 80400);
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no cases in {}", dir.display());
    for path in paths {
        let content = std::fs::read_to_string(&path).unwrap();
        let at = content.rfind("-----\n").unwrap();
        let (input, expected) = (&content[..at], &content[at + 6..]);
        let problems = linter.check(&path, input).unwrap();
        let mut got: Vec<String> = problems
            .iter()
            .map(|p| {
                let (line, col) = line_col(input, p.span.start);
                if PHPSTAN.contains(&id) {
                    format!("{line}: {}", p.message)
                } else {
                    format!("{line}:{col}: {}", p.message)
                }
            })
            .collect();
        got.sort_by_key(|l| {
            let mut n = l.split(':').map(|s| s.trim().parse::<usize>().unwrap_or(0));
            (n.next().unwrap(), n.next().unwrap())
        });
        let want: Vec<String> = expected.lines().map(str::to_string).collect();
        assert_eq!(got, want, "{id}: {}", path.display());
    }
}

#[test]
fn upstream_cases() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/lint_b_report");
    let mut groups: Vec<_> = std::fs::read_dir(&root)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    groups.sort();
    for group in groups {
        let mut names: Vec<_> = std::fs::read_dir(&group)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        names.sort();
        for name in names {
            let id = format!(
                "lint/{}/{}",
                group.file_name().unwrap().to_string_lossy(),
                name.file_name().unwrap().to_string_lossy()
            );
            check(&id, &name);
        }
    }
}

#[test]
fn version_gates() {
    // no-underscore-class needs PHP 8.4 and no-void-reference-return PHP 8.2 (Mago's RuleRequirements::PHPVersion).
    let code = "<?php\nclass _ {}\nfunction &f(): void {}\n";
    let ids = |v| {
        Linter::new(
            vec![
                Box::new(rules::NoUnderscoreClass),
                Box::new(rules::NoVoidReferenceReturn),
            ],
            v,
        )
        .rule_ids()
    };
    assert_eq!(ids(80399), ["lint/functions/no-void-reference-return"]);
    assert!(ids(80199).is_empty());
    let found = Linter::new(vec![Box::new(rules::NoUnderscoreClass)], 80400)
        .check(Path::new("a.php"), code)
        .unwrap();
    assert_eq!(found.len(), 1);
}

#[test]
fn hash_emoji_fix() {
    // What `mago lint --only no-hash-emoji --fix` wrote for tests/lint_b_report/style/no-hash-emoji/a.php.inc.
    let input = "<?php\n\n#\u{fe0f}\u{20e3} a comment\n# plain\n#\u{fe0f}\u{20e3}[Attr]\nfunction f() {} #\u{fe0f}\u{20e3} trailing\n// #\u{fe0f}\u{20e3} not hash\n";
    let fixed = Linter::new(vec![Box::new(rules::NoHashEmoji)], 80400)
        .fix(Path::new("a.php"), input)
        .unwrap();
    assert_eq!(
        fixed.source,
        "<?php\n\n# a comment\n# plain\n#[Attr]\nfunction f() {} # trailing\n// #\u{fe0f}\u{20e3} not hash\n"
    );
}

#[test]
fn upstream_fixes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/lint_b_report_fix");
    for group in std::fs::read_dir(&root).unwrap() {
        let group = group.unwrap().path();
        for name in std::fs::read_dir(&group).unwrap() {
            let name = name.unwrap().path();
            let id = format!(
                "lint/{}/{}",
                group.file_name().unwrap().to_string_lossy(),
                name.file_name().unwrap().to_string_lossy()
            );
            let rule = rules::all().into_iter().find(|r| r.id() == id).unwrap();
            let linter = Linter::new(vec![rule], 80400);
            for case in std::fs::read_dir(&name).unwrap() {
                let path = case.unwrap().path();
                let content = std::fs::read_to_string(&path).unwrap();
                let (input, expected) = content.split_once("-----\n").unwrap();
                let fixed = linter.fix(&path, input).unwrap();
                assert_eq!(fixed.source, expected, "{id}: {}", path.display());
            }
        }
    }
}
