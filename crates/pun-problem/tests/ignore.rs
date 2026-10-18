//! `@pun-ignore` comments (context/plan/rules.md §4). Expected values are derived by hand from the table there.

use pun_problem::ignore::{Ignores, covers};
use pun_problem::{Level, LineIndex, Problem, Span, apply_fixes};

/// A problem of `rule` at the first byte of 1-based `line`.
fn at(source: &str, rule: &str, line: usize) -> Problem {
    let start = LineIndex::new(source).line_range(line).unwrap().start;
    Problem {
        rule: rule.into(),
        path: "a.php".into(),
        span: Span::new(start, start),
        message: String::new(),
        help: None,
        level: Level::Error,
        fix: None,
    }
}

/// The lines of the problems `source`'s directives leave, out of one problem of `rule` per line.
fn kept(source: &str, rule: &str) -> Vec<usize> {
    let index = LineIndex::new(source);
    let mut problems: Vec<Problem> = (1..=index.line_count())
        .map(|l| at(source, rule, l))
        .collect();
    Ignores::parse(source).retain(&mut problems, &index);
    problems
        .iter()
        .map(|p| index.line_col(p.span.start).line)
        .collect()
}

#[test]
fn ids_cover_the_rule_its_short_form_and_its_group() {
    let rule = "lint/security/no-eval";
    for id in [
        "lint/security/no-eval",
        "security/no-eval",
        "lint/security",
        "security",
    ] {
        assert!(covers(id, rule), "{id}");
    }
    for id in [
        "lint/security/no-eva",
        "analyse/security",
        "lint",
        "no-eval",
        "style/no-eval",
        "lint/sec",
    ] {
        assert!(!covers(id, rule), "{id}");
    }
    // A rule ID without a group (`syntax-error`) only matches itself.
    assert!(!covers("x", "syntax-error"));
    assert!(covers("syntax-error", "syntax-error"));
}

#[test]
fn a_trailing_comment_covers_its_own_line() {
    let s = "<?php\n$a = eval($b); // @pun-ignore lint/security/no-eval\n$c = 1;\n";
    assert_eq!(kept(s, "lint/security/no-eval"), [1, 3, 4]);
    // Another rule on that line stays.
    assert_eq!(kept(s, "lint/style/x"), [1, 2, 3, 4]);
}

#[test]
fn a_comment_alone_on_its_line_covers_the_next() {
    let s =
        "<?php\n    // @pun-ignore security/no-eval -- legacy loader\n$a = eval($b);\n$c = 1;\n";
    assert_eq!(kept(s, "lint/security/no-eval"), [1, 2, 4, 5]);
    // `#`, `/* */` and a PHPDoc over several lines: the line after the comment's last line.
    let s = "<?php\n# @pun-ignore security\n$a;\n/**\n * @pun-ignore lint/security/no-eval\n */\n$b;\n/* @pun-ignore-next-line security/no-eval */ $c;\n$d;\n";
    assert_eq!(kept(s, "lint/security/no-eval"), [1, 2, 4, 5, 6, 8, 10]);
}

#[test]
fn next_line_file_and_regions() {
    let s = "<?php\n$a; // @pun-ignore-next-line lint/x/y\n$b;\n$c;\n";
    assert_eq!(kept(s, "lint/x/y"), [1, 2, 4, 5]);
    let s = "<?php\n$a;\n// @pun-ignore-file lint/x\n$b;\n";
    assert_eq!(kept(s, "lint/x/y"), [] as [usize; 0]);
    // From the start comment's line to the end comment's line; an unclosed region runs to the end.
    let s = "<?php\n// @pun-ignore-start x/y\n$a;\n$b;\n// @pun-ignore-end\n$c;\n// @pun-ignore-start x\n$d;\n";
    assert_eq!(kept(s, "lint/x/y"), [1, 6]);
}

#[test]
fn several_ids_and_their_use() {
    let s = "<?php\n$a; // @pun-ignore lint/x/one, lint/x/two,lint/x/three -- why\n";
    let index = LineIndex::new(s);
    let mut ignores = Ignores::parse(s);
    let mut problems = vec![at(s, "lint/x/two", 2), at(s, "lint/x/four", 2)];
    ignores.retain(&mut problems, &index);
    assert_eq!(problems.len(), 1);
    assert_eq!(problems[0].rule, "lint/x/four");
    // `lint/x/three` didn't run: not judged.
    let unused = ignores.unused(s, |id| id != "lint/x/three");
    assert_eq!(unused.len(), 1);
    assert_eq!(unused[0].id, "lint/x/one");
    assert_eq!(&s[unused[0].span.start..unused[0].span.end], "lint/x/one");
    let fixed = apply_fixes(s, &[&unused[0].fix]).source;
    assert_eq!(
        fixed,
        "<?php\n$a; // @pun-ignore lint/x/two,lint/x/three -- why\n"
    );
    // The last ID takes the comma before it.
    let unused = ignores.unused(s, |id| id == "lint/x/three");
    let fixed = apply_fixes(s, &[&unused[0].fix]).source;
    assert_eq!(
        fixed,
        "<?php\n$a; // @pun-ignore lint/x/one, lint/x/two -- why\n"
    );
}

#[test]
fn an_unused_comment_goes_with_its_line_or_its_spaces() {
    let s = "<?php\n    // @pun-ignore a/b, a/c\n$a;\n$b; /* @pun-ignore a/b */\n";
    let ignores = Ignores::parse(s);
    let unused = ignores.unused(s, |_| true);
    assert_eq!(unused.len(), 3);
    // One comment's IDs share the deletion: applying all of them applies it once.
    let fixes: Vec<_> = unused.iter().map(|u| &u.fix).collect();
    let round = apply_fixes(s, &fixes);
    assert_eq!(round.source, "<?php\n$a;\n$b;\n");
    assert_eq!(round.applied, [0, 2]);
    assert_eq!(round.deferred, [1]);
}

#[test]
fn malformed_and_lookalike_directives() {
    let s = "<?php\n// @pun-ignore\n$a; # @pun-ignore -- no id\n// @pun-ignored x\n// @pun-ignore-end\n$b = '@pun-ignore x';\n";
    let ignores = Ignores::parse(s);
    let m: Vec<(&str, &str)> = (ignores.malformed.iter())
        .map(|m| (&s[m.span.start..m.span.end], m.message.as_str()))
        .collect();
    assert_eq!(
        m,
        [
            ("@pun-ignore", "`@pun-ignore` needs one or more rule IDs"),
            ("@pun-ignore", "`@pun-ignore` needs one or more rule IDs"),
        ]
    );
    // `@pun-ignored`, a stray end and a string are not directives.
    assert!(!ignores.is_empty());
    assert_eq!(kept(s, "lint/x/y").len(), LineIndex::new(s).line_count());
    assert!(Ignores::parse("<?php\n$a;\n").is_empty());
}

/// Only a `-file` directive covers a whole-file rule, and using it keeps it from being reported unused; a line
/// directive for the same rule does not cover it.
#[test]
fn file_directives_cover_whole_file_rules() {
    let source = "<?php\n// @pun-ignore-file lint/style\n$a = 1; // @pun-ignore lint/arrays/x\n";
    let mut ignores = Ignores::parse(source);
    assert!(ignores.ignores_file("lint/style/y"));
    assert!(!ignores.ignores_file("lint/arrays/x"));
    let unused: Vec<String> = ignores
        .unused(source, |_| true)
        .into_iter()
        .map(|u| u.id)
        .collect();
    assert_eq!(unused, ["lint/arrays/x"]);
}
