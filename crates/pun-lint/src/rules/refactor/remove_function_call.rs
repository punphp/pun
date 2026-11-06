//! `lint/refactor/remove-function-call`, ported from Rector 2.6.1's `RemoveFuncCallRector`
//! (rules/Removing/Rector/FuncCall).
//!
//! Options: `items = ["var_dump"]`. A statement that is only a call to one of the functions is removed. The call's name
//! is compared without case as `NodeNameResolver::getName()` reads it, which for `$name()` is the variable's name.

use mago_syntax::cst::{Expression, Node, NodeKind, Statement, Variable};
use pun_problem::Safety;

use super::{Kind, called, is_name, items, name_of};
use crate::rules::rector_stmts::{enclosing, remove};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

#[derive(Default)]
pub struct RemoveFunctionCall {
    pub functions: Vec<String>,
}

impl Rule for RemoveFunctionCall {
    fn id(&self) -> &'static str {
        "lint/refactor/remove-function-call"
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        self.functions = items(options)?
            .into_iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or("each of `items` must be a string")
            })
            .collect::<Result<_, _>>()?;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Statement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Statement(statement @ Statement::Expression(e)) = node else {
            return;
        };
        let call = match unparenthesize(e.expression) {
            Expression::Call(mago_syntax::cst::Call::Function(call)) => Node::FunctionCall(call),
            Expression::PartialApplication(mago_syntax::cst::PartialApplication::Function(p)) => {
                Node::FunctionPartialApplication(p)
            }
            _ => return,
        };
        let name = match called(call) {
            Some(Expression::Identifier(name)) => name_of(ancestors, name, Kind::Function),
            Some(Expression::Variable(Variable::Direct(v))) => {
                String::from_utf8_lossy(&v.name[1..]).into_owned()
            }
            _ => return,
        };
        let Some(function) = self.functions.iter().find(|f| is_name(&name, f)) else {
            return;
        };
        let (list, index) = enclosing(ctx.source, statement, ancestors);
        out.push(Report::fixed(
            node_span(statement),
            format!("Remove the call to `{function}()`."),
            Safety::Unsafe,
            remove(ctx, &list, &[index]),
        ));
    }
}
