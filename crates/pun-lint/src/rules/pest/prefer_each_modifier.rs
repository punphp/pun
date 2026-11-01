//! `lint/pest/prefer-each-modifier`, ported from pest-plugin-rector's `UseEachModifierRector`: a `foreach` whose
//! only statement is `expect($item)->...` on its value becomes `expect($items)->each->...`.

use mago_syntax::cst::{
    Expression, ForeachBody, ForeachTarget, Node, NodeKind, Statement, UnaryPrefixOperator,
    Variable,
};
use pun_problem::{Edit, Safety, Span};

use super::{
    Link, chain_links, comments_start, expect_call, first_value, indent_at, kept_text, method_call,
    print_links, shift,
};
use crate::rules::php_parser::{block_list, open_list, tokens};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferEachModifier;

impl Rule for PreferEachModifier {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-each-modifier"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Foreach(foreach) = node else {
            return;
        };
        let tokens = tokens(ctx.source);
        let list = match &foreach.body {
            ForeachBody::Statement(Statement::Block(block)) => block_list(&tokens, block),
            ForeachBody::Statement(statement) => {
                super::super::php_parser::body_list(&tokens, statement)
            }
            ForeachBody::ColonDelimited(body) => open_list(
                &tokens,
                &body.statements,
                body.colon.end.offset as usize,
                body.end_foreach.span.start.offset as usize,
            ),
        };
        let ([Statement::Expression(statement)], false) = (list.stmts.as_slice(), list.nop) else {
            return;
        };
        let Some(call) = method_call(statement.expression) else {
            return;
        };
        let Some(expect) = expect_call(call) else {
            return;
        };
        let Some(subject) = first_value(&expect.argument_list).map(unparenthesize) else {
            return;
        };
        let Expression::Variable(Variable::Direct(subject)) = subject else {
            return;
        };
        let (key, value) = match &foreach.target {
            ForeachTarget::Value(t) => (None, t.value),
            ForeachTarget::KeyValue(t) => (Some(t.key), t.value),
        };
        let value = match unparenthesize(value) {
            Expression::UnaryPrefix(u)
                if matches!(u.operator, UnaryPrefixOperator::Reference(_)) =>
            {
                unparenthesize(u.operand)
            }
            v => v,
        };
        let Expression::Variable(Variable::Direct(value)) = value else {
            return;
        };
        if value.name != subject.name {
            return;
        }
        let links = chain_links(call);
        let used = |name: &[u8]| {
            links.iter().any(|link| match link {
                Link::Method(m) => m
                    .argument_list
                    .arguments
                    .iter()
                    .any(|a| uses_variable(Node::Argument(a), name)),
                Link::Property(_) => false,
            })
        };
        if used(value.name) {
            return;
        }
        if let Some(Expression::Variable(Variable::Direct(key))) = key.map(unparenthesize)
            && used(key.name)
        {
            return;
        }
        // The new statement takes the loop's place, at its indent, and the loop's comments go with it.
        let span = node_span(foreach);
        let indent = indent_at(ctx, span.start);
        let argument = node_span(&expect.argument_list.arguments.as_slice()[0]);
        let expect_span = node_span(expect);
        let delta = indent as isize - indent_at(ctx, expect_span.start) as isize;
        let printed = format!(
            "{}{}{}->each{};",
            shift(ctx, Span::new(expect_span.start, argument.start), delta),
            kept_text(ctx, node_span(unparenthesize(foreach.expression)), indent),
            shift(ctx, Span::new(argument.end, expect_span.end), delta),
            print_links(ctx, &links, indent),
        );
        out.push(Report::fixed(
            span,
            "Use the `->each` modifier instead of a `foreach` of `expect()` calls.",
            Safety::Safe,
            vec![Edit::new(
                Span::new(comments_start(ctx, span.start), span.end),
                printed,
            )],
        ));
    }
}

/// `NodeFinder::findInstanceOf($node, Variable::class)` holds a variable named `name`.
fn uses_variable(node: Node<'_, '_>, name: &[u8]) -> bool {
    if let Node::DirectVariable(v) = node
        && v.name == name
    {
        return true;
    }
    let mut found = false;
    node.visit_children(|child| found = found || uses_variable(child, name));
    found
}
