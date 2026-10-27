//! `lint/refactor/remove-function-argument`, ported from Rector 2.6.1's `RemoveFuncCallArgRector`
//! (rules/Removing/Rector/FuncCall).
//!
//! Options: `items = [{ function = "ldap_first_attribute", argument-position = 2 }]` (`RemoveFuncCallArg`). A call to
//! the function (its `Name` as written or resolved, compared without case) loses the argument at that position,
//! counted from 0 over every argument, named ones included. A first-class callable `f(...)` has one argument, its
//! `...`.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::{Edit, Span};

use super::{Kind, assert_function, called, int, is_name, items, name_of, object, report, string};
use crate::rules::rector_batch5::list_removal;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

#[derive(Default)]
pub struct RemoveFunctionArgument {
    /// (function, argument position)
    pub removals: Vec<(String, i64)>,
}

impl Rule for RemoveFunctionArgument {
    fn id(&self) -> &'static str {
        "lint/refactor/remove-function-argument"
    }

    /// Rector runs the rule once per file; its fix could match again on what it wrote.
    fn once(&self) -> bool {
        true
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        let mut removals = Vec::new();
        for item in items(options)? {
            let map = object(
                item,
                &["function", "argument-position"],
                &["function", "argument-position"],
            )?;
            let function = string(map, "function")?;
            assert_function(&function)?;
            removals.push((function, int(map, "argument-position")?));
        }
        self.removals = removals;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall, NodeKind::FunctionPartialApplication])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(Expression::Identifier(name)) = called(node) else {
            return;
        };
        if self.removals.is_empty() {
            return;
        }
        let resolved = name_of(ancestors, name, Kind::Function);
        // The argument spans, and the span inside the parentheses.
        let (args, inner): (Vec<Span>, Span) = match node {
            Node::FunctionCall(call) => {
                let list = &call.argument_list;
                (
                    list.arguments.iter().map(|a| node_span(a)).collect(),
                    Span::new(
                        list.left_parenthesis.end.offset as usize,
                        list.right_parenthesis.start.offset as usize,
                    ),
                )
            }
            Node::FunctionPartialApplication(p) => {
                let list = &p.argument_list;
                (
                    list.arguments.iter().map(|a| node_span(a)).collect(),
                    Span::new(
                        list.left_parenthesis.end.offset as usize,
                        list.right_parenthesis.start.offset as usize,
                    ),
                )
            }
            _ => return,
        };
        let mut removed = vec![false; args.len()];
        for (function, position) in &self.removals {
            if is_name(&resolved, function)
                && let Ok(at) = usize::try_from(*position)
                && at < args.len()
            {
                removed[at] = true;
            }
        }
        if !removed.contains(&true) {
            return;
        }
        // With every argument gone, php-parser prints the call anew, with nothing between its parentheses.
        let edits = list_removal(&args, &removed).unwrap_or_else(|| vec![Edit::new(inner, "")]);
        let positions: Vec<String> = (removed.iter().enumerate())
            .filter(|(_, r)| **r)
            .map(|(i, _)| i.to_string())
            .collect();
        out.push(report(
            node_span(&node),
            format!(
                "Remove the argument at position {} of `{resolved}()`.",
                positions.join(", ")
            ),
            edits,
        ));
    }
}
