//! `lint/phpunit/prefer-this-call`, ported from rector-phpunit's `PreferPHPUnitThisCallRector`
//! (rules/CodeQuality/Rector/Class_) and `AssertMethodAnalyzer::detectTestCaseCall`.
//!
//! In a test class, `self::assert*()` and `static::assert*()` (and the matcher, mock and exception methods of
//! `NonAssertNonStaticMethods`) become `$this->...()` when the class's method of that name is declared by
//! `PHPUnit\Framework\TestCase` or `PHPUnit\Framework\Assert`. Static methods, static closures and static arrow
//! functions are skipped with what they hold. php-parser prints the new call: the arguments joined with `, ` (a line
//! each when one has comments). Rector's oracle matches `self` and `static` as written in lower case only, and leaves
//! anonymous classes alone.

use mago_syntax::cst::{ClassLikeMemberSelector, Expression, Modifier, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{ASSERT, TEST_CASE, is_in_test_class, maybe_multiline};
use crate::rules::laravel::is_a;
use crate::rules::rector_stmts::line_indent;
use crate::rules::types::class_scope;
use crate::rules::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// rector-phpunit's `NonAssertNonStaticMethods::ALL`.
pub(super) const NON_ASSERT: &[&str] = &[
    "createMock",
    "atLeast",
    "atLeastOnce",
    "once",
    "never",
    "any",
    "exactly",
    "atMost",
    "throwException",
    "expectException",
    "expectExceptionMessage",
    "expectExceptionCode",
    "expectExceptionMessageMatches",
];

pub struct PreferThisCall;

impl Rule for PreferThisCall {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-this-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let facts = facts(ctx);
        // `isObjectType(self, TestCase)`, and the class's reflection
        let Some(metadata) = class_scope(&scope)
            .and_then(|s| s.name)
            .filter(|class| is_a(ctx, class, &[TEST_CASE]))
            .and_then(|class| facts.codebase.get_class_like(class.as_bytes()))
        else {
            return;
        };
        let declared_by_phpunit = |method: &[u8]| {
            metadata
                .declaring_method_ids
                .iter()
                .find(|(m, _)| m.as_bytes().eq_ignore_ascii_case(method))
                .is_some_and(|(_, id)| {
                    let declaring = id.get_class_name();
                    [TEST_CASE, ASSERT]
                        .iter()
                        .any(|c| c.as_bytes().eq_ignore_ascii_case(declaring.as_bytes()))
                })
        };
        let mut stack = Vec::new();
        node.visit_children(|child| stack.push(child));
        while let Some(node) = stack.pop() {
            let skipped = match node {
                Node::Method(m) => m.modifiers.iter().any(|m| matches!(m, Modifier::Static(_))),
                Node::Closure(c) => c.r#static.is_some(),
                Node::ArrowFunction(f) => f.r#static.is_some(),
                Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_) => true,
                _ => false,
            };
            if skipped {
                continue;
            }
            node.visit_children(|child| stack.push(child));
            let Node::StaticMethodCall(call) = node else {
                continue;
            };
            let keyword = match call.class {
                Expression::Self_(k) | Expression::Static(k) => k.value,
                _ => continue,
            };
            let ClassLikeMemberSelector::Identifier(name) = &call.method else {
                continue;
            };
            let named = name.value.starts_with(b"assert")
                || NON_ASSERT.iter().any(|n| n.as_bytes() == name.value);
            if !matches!(keyword, b"self" | b"static") || !named || !declared_by_phpunit(name.value)
            {
                continue;
            }
            let span = node_span(call);
            let list = &call.argument_list;
            let args: Vec<Span> = list.arguments.iter().map(node_span).collect();
            let text = format!(
                "$this->{}({})",
                String::from_utf8_lossy(name.value),
                maybe_multiline(
                    ctx.source,
                    list.left_parenthesis.end.offset as usize,
                    &args,
                    line_indent(ctx.source, span.start),
                )
            );
            out.push(Report::fixed(
                span,
                "Call the PHPUnit method on `$this`.",
                Safety::Safe,
                vec![Edit::new(span, text)],
            ));
        }
    }
}
