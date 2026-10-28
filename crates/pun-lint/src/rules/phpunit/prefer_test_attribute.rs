//! `lint/phpunit/prefer-test-attribute`, ported from Mago 1.57.0's `prefer-test-attribute` (mago-linter
//! src/rule/best_practices/prefer_test_attribute.rs).
//!
//! A method whose name starts with `test` or `Test`, followed by nothing, `_` or an ASCII capital, and that has no
//! `#[PHPUnit\Framework\Attributes\Test]`, is reported on its name. The fix renames it without the prefix and adds the
//! attribute before the method's first attribute or modifier, at the method's indentation. Like Mago, the rule does not
//! ask whether the class is a test case (Mago only runs it with its PHPUnit integration on; Pun turns the `phpunit`
//! group on from composer.lock).

use mago_span::HasSpan;
use mago_syntax::cst::{Method, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::rules::lint_b_names::resolve_class;
use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct PreferTestAttribute;

impl Rule for PreferTestAttribute {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-test-attribute"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else {
            return;
        };
        let name = method.name.value;
        let Ok(name) = std::str::from_utf8(name) else {
            return;
        };
        if !name.starts_with("test") && !name.starts_with("Test") {
            return;
        }
        if name[4..]
            .bytes()
            .next()
            .is_some_and(|first| first != b'_' && !first.is_ascii_uppercase())
        {
            return;
        }
        if has_test_attribute(ancestors, method) {
            return;
        }
        let new_name = new_name(name);
        let start = method.span().start.offset as usize;
        let line_start = ctx.source[..start].rfind('\n').map_or(0, |pos| pos + 1);
        let indent: String = ctx.source[line_start..start]
            .chars()
            .take_while(|c| *c == ' ' || *c == '\t')
            .collect();
        out.push(Report {
            span: span(method.name.span()),
            message: format!("Use `#[Test]` attribute instead of `test` prefix on method `{name}`."),
            help: Some(format!(
                "Rename to `{new_name}` and add `#[Test]` attribute."
            )),
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![
                    Edit::new(span(method.name.span()), new_name),
                    Edit::new(
                        Span::new(start, start),
                        format!("#[\\PHPUnit\\Framework\\Attributes\\Test]\n{indent}"),
                    ),
                ],
            }),
        });
    }
}

fn has_test_attribute(ancestors: &[Node<'_, '_>], method: &Method<'_>) -> bool {
    method.attribute_lists.iter().any(|list| {
        list.attributes.iter().any(|attribute| {
            resolve_class(ancestors, &attribute.name)
                .eq_ignore_ascii_case(b"PHPUnit\\Framework\\Attributes\\Test")
        })
    })
}

/// Mago's `compute_new_name`: the name without `test`, `Test`, `test_` or `Test_`, its first letter lowered after
/// `test`; `test`, `test_` and their capitalized forms stay.
fn new_name(name: &str) -> String {
    let after_test = &name[4..];
    if after_test.is_empty() || after_test == "_" {
        return name.to_string();
    }
    if let Some(rest) = after_test.strip_prefix('_') {
        return rest.to_string();
    }
    if name.starts_with("test") {
        // the byte after the prefix is an ASCII capital
        return after_test[..1].to_ascii_lowercase() + &after_test[1..];
    }
    after_test.to_string()
}
