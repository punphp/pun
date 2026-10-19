//! The engine: one parse, problems, the fix loop and its errors. Expected values are derived by hand.

use std::path::{Path, PathBuf};

use mago_span::HasSpan;
use mago_syntax::cst::Node;
use pun_lint::{Context, LintError, Linter, MAX_ROUNDS, Report, Rule, rules};
use pun_problem::{Edit, Fix, Level, Safety, Span};

#[test]
fn check_reports_each_problem_once_with_rule_path_span_and_fix() {
    let linter = Linter::new(rules::all(), 80400);
    let code = "<?php\n$a = $a + 1;\n";
    let problems = linter.check(Path::new("src/a.php"), code).unwrap();
    assert_eq!(problems.len(), 1);

    let compound = &problems[0];
    assert_eq!(compound.rule, "lint/style/prefer-compound-assignment");
    assert_eq!(compound.path, PathBuf::from("src/a.php"));
    assert_eq!(compound.span, Span::new(6, 17)); // `$a = $a + 1`
    assert_eq!(compound.level, Level::Error);
    assert_eq!(
        compound.message,
        "Use `+=` instead of repeating the assigned variable."
    );
    assert_eq!(compound.help, None);
    let fix = compound.fix.as_ref().unwrap();
    assert_eq!(fix.safety, Safety::Unsafe);
    assert_eq!(fix.edits, vec![Edit::new(Span::new(8, 16), " += ")]);
}

#[test]
fn rules_below_their_php_version_do_not_run() {
    let rules = || -> Vec<Box<dyn Rule>> { vec![Box::new(ReportOnly), Box::new(NeedsPhp84)] };
    let both = vec!["lint/test/report-only", "lint/test/needs-php-8.4"];
    assert_eq!(Linter::new(rules(), 80400).rule_ids(), both);
    assert_eq!(
        Linter::new(rules(), 80399).rule_ids(),
        vec!["lint/test/report-only"]
    );
}

#[test]
fn fix_counts_rounds_and_keeps_problems_without_a_fix() {
    // Round 1 unwraps the outer parentheses; the inner ones overlap it and wait. Round 2 unwraps them. ReportOnly's
    // problem stays.
    let rules: Vec<Box<dyn Rule>> = vec![Box::new(ReportOnly), Box::new(Unwrap)];
    let linter = Linter::new(rules, 80400);
    let code = "<?php\n$a = ((1));\ntry { } finally { }\n";
    let fixed = linter.fix(Path::new("a.php"), code).unwrap();
    assert_eq!(fixed.source, "<?php\n$a = 1;\ntry { } finally { }\n");
    assert_eq!(fixed.rounds, 2);
    assert_eq!(fixed.applied.len(), 2);
    assert_eq!(fixed.applied[0].span, Span::new(11, 16)); // `((1))`, in the original source
    assert_eq!(fixed.applied[1].span, Span::new(11, 14)); // `(1)`, in round 2's source
    assert_eq!(fixed.remaining.len(), 1);
    assert_eq!(fixed.remaining[0].rule, "lint/test/report-only");
}

#[test]
fn nothing_to_fix_is_zero_rounds() {
    let linter = Linter::new(rules::all(), 80400);
    let fixed = linter.fix(Path::new("a.php"), "<?php\n$a += 1;\n").unwrap();
    assert_eq!(fixed.source, "<?php\n$a += 1;\n");
    assert_eq!(fixed.rounds, 0);
    assert!(fixed.applied.is_empty() && fixed.remaining.is_empty());
}

#[test]
fn a_file_that_does_not_parse_is_refused() {
    let linter = Linter::new(rules::all(), 80400);
    let err = linter
        .check(Path::new("a.php"), "<?php\n$a = ;\n")
        .unwrap_err();
    let LintError::Syntax { span, .. } = &err else {
        panic!("{err:?}");
    };
    assert_eq!(span.start, 11); // the `;`
    assert!(err.to_string().starts_with("syntax error: "), "{err}");
    assert_eq!(
        linter
            .fix(Path::new("a.php"), "<?php\n$a = ;\n")
            .unwrap_err(),
        err
    );
}

#[test]
fn a_fix_that_breaks_the_syntax_is_an_error() {
    let linter = Linter::new(vec![Box::new(DropSemicolons)], 80400);
    let err = linter
        .fix(Path::new("a.php"), "<?php\n$a = 1;\n$b = 2;\n")
        .unwrap_err();
    let LintError::FixBrokeSyntax { rules, .. } = &err else {
        panic!("{err:?}");
    };
    assert_eq!(rules, &vec!["lint/test/drop-semicolons".to_string()]);
    assert!(
        err.to_string()
            .starts_with("the fixes of lint/test/drop-semicolons produced a syntax error: "),
        "{err}"
    );
}

#[test]
fn fixes_that_never_settle_stop_after_max_rounds() {
    let linter = Linter::new(vec![Box::new(AlwaysAppend)], 80400);
    let err = linter.fix(Path::new("a.php"), "<?php\n").unwrap_err();
    assert_eq!(
        err,
        LintError::NoConvergence {
            rules: vec!["lint/test/always-append".to_string()]
        }
    );
    assert_eq!(
        err.to_string(),
        format!("fixes did not settle after {MAX_ROUNDS} rounds (lint/test/always-append)")
    );
}

/// Reports every `try`, with no fix.
struct ReportOnly;

impl Rule for ReportOnly {
    fn id(&self) -> &'static str {
        "lint/test/report-only"
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if let (Node::Try(t), [Node::Program(_), Node::Statement(_)]) = (node, ancestors) {
            out.push(Report {
                span: pun_lint::syntax::node_span(t),
                message: "try".into(),
                help: None,
                fix: None,
            });
        }
    }
}

/// Reports nothing, and needs PHP 8.4.
struct NeedsPhp84;

impl Rule for NeedsPhp84 {
    fn id(&self) -> &'static str {
        "lint/test/needs-php-8.4"
    }

    fn min_php_version(&self) -> u32 {
        80400
    }

    fn visit<'ast, 'arena>(
        &self,
        _: Node<'ast, 'arena>,
        _: &[Node<'ast, 'arena>],
        _: &Context<'_>,
        _: &mut Vec<Report>,
    ) {
    }
}

/// Replaces each parenthesized expression with its inside.
struct Unwrap;

impl Rule for Unwrap {
    fn id(&self) -> &'static str {
        "lint/test/unwrap"
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if let Node::Parenthesized(p) = node {
            let span = pun_lint::syntax::node_span(p);
            let inner = pun_lint::syntax::node_span(p.expression);
            out.push(Report {
                span,
                message: "parentheses".into(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: vec![Edit::new(span, &ctx.source[inner.start..inner.end])],
                }),
            });
        }
    }
}

/// Deletes every statement terminator `;`.
struct DropSemicolons;

impl Rule for DropSemicolons {
    fn id(&self) -> &'static str {
        "lint/test/drop-semicolons"
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _: &[Node<'ast, 'arena>],
        _: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if let Node::Terminator(t) = node {
            let span = pun_lint::syntax::span(t.span());
            out.push(Report {
                span,
                message: "semicolon".into(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: vec![Edit::new(span, "")],
                }),
            });
        }
    }
}

/// Appends a newline at the end of the file, every time.
struct AlwaysAppend;

impl Rule for AlwaysAppend {
    fn id(&self) -> &'static str {
        "lint/test/always-append"
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if let Node::Program(_) = node {
            let end = ctx.source.len();
            out.push(Report {
                span: Span::new(end, end),
                message: "append".into(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: vec![Edit::new(Span::new(end, end), "\n")],
                }),
            });
        }
    }
}

#[test]
fn a_rule_without_options_rejects_any() {
    let mut rule = ReportOnly;
    assert_eq!(rule.configure(&serde_json::Value::Null), Ok(()));
    assert_eq!(
        rule.configure(&serde_json::json!({"a": 1})),
        Err("takes no options".to_string())
    );
}
