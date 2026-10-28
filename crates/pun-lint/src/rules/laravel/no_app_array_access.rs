//! `lint/laravel/no-app-array-access`, ported from rector-laravel 2.5.0's `CallOnAppArrayAccessToStandaloneAssignRector`
//! (src/Rector/Assign).
//!
//! `$x = $app['validator']->method(...);`, where `$app` is an `Illuminate\Contracts\Foundation\Application` and the
//! key's value (`ValueResolver::getValue`) is `validator`, gets the service in a typed variable first:
//! `/** @var \Illuminate\Validation\Factory $validationFactory */ $validationFactory = $app['validator'];`, and the call
//! goes on `$validationFactory`. Only an expression statement that is a plain assignment of a method call counts.
//!
//! php-parser prints the new statements (a `Nop`, then the assignment with its doc) right after the token before the
//! statement, each on a new line at the statement's indentation; what stood between that token and the statement
//! (its comments, or the rest of a line) follows them.

use mago_syntax::cst::{AssignmentOperator, Expression, ExpressionStatement, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::object_type::is_object_type;
use super::prefer_latest_oldest::{Got, get_value};
use super::print::kept_nodes;
use crate::rules::rector_conditions::indentation;
use crate::rules::rector_stmts::previous_token_end;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoAppArrayAccess;

impl Rule for NoAppArrayAccess {
    fn id(&self) -> &'static str {
        "lint/laravel/no-app-array-access"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ExpressionStatement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ExpressionStatement(
            statement @ ExpressionStatement {
                expression: Expression::Assignment(assign),
                ..
            },
        ) = node
        else {
            return;
        };
        let (AssignmentOperator::Assign(_), Expression::Call(mago_syntax::cst::Call::Method(call))) =
            (&assign.operator, assign.rhs)
        else {
            return;
        };
        let Expression::ArrayAccess(access) = unparenthesize(call.object) else {
            return;
        };
        if !is_object_type(
            ctx,
            ancestors,
            access.array,
            "Illuminate\\Contracts\\Foundation\\Application",
        ) || !matches!(get_value(ctx, ancestors, access.index), Got::Str(s, _) if s == "validator")
        {
            return;
        }
        let source = ctx.source;
        let start = node_span(statement).start;
        let outer = (ancestors.iter().rev().map(|a| node_span(a).start))
            .find(|&s| s < start)
            .unwrap_or(0);
        let at = previous_token_end(source, outer, start);
        let indent = " ".repeat(indentation(source, start));
        let fetch = kept_nodes(source, std::iter::once(node_span(access)), start).concat();
        let inserted = format!(
            "\n\n{indent}/**\n{indent} * @var \\Illuminate\\Validation\\Factory $validationFactory\n{indent} */\n\
             {indent}$validationFactory = {fetch};"
        );
        out.push(Report {
            span: node_span(access),
            message: "Assign the `validator` service to a typed variable instead of calling on the array access."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![
                    Edit::new(Span::new(at, at), inserted),
                    Edit::new(node_span(access), "$validationFactory"),
                ],
            }),
        });
    }
}
