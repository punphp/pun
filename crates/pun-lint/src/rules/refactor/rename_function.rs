//! `lint/refactor/rename-function`, ported from Rector 2.8.4's `RenameFunctionRector` (rules/Renaming/Rector/FuncCall).
//!
//! Options: a map of old function names to new ones, `{ view = "Laravel\\Templating\\render" }`. A call (or a
//! first-class callable) whose name is an old one, compared without case as `FuncCallNameResolver` resolves it, gets the
//! new name, fully qualified when it holds a `\`.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{called, fully_qualified, function_name, is_name, string_map};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

#[derive(Default)]
pub struct RenameFunction {
    pub renames: Vec<(String, String)>,
}

impl Rule for RenameFunction {
    fn id(&self) -> &'static str {
        "lint/refactor/rename-function"
    }

    /// Rector runs the rule once per file; its fix could match again on what it wrote.
    fn once(&self) -> bool {
        true
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        self.renames = string_map(options)?;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall, NodeKind::FunctionPartialApplication])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(Expression::Identifier(name)) = called(node) else {
            return;
        };
        let resolved = function_name(ctx, ancestors, name);
        let Some((old, new)) = self.renames.iter().find(|(old, _)| is_name(&resolved, old)) else {
            return;
        };
        let text = match new.contains('\\') {
            true => fully_qualified(new),
            false => new.clone(),
        };
        // A rename to the name already written (`a` to `A` matches again without case) would repeat in every round.
        if text == String::from_utf8_lossy(name.value()) {
            return;
        }
        out.push(Report::fixed(
            node_span(&node),
            format!("Rename the function `{old}()` to `{new}()`."),
            Safety::Unsafe,
            vec![Edit::new(node_span(name), text)],
        ));
    }
}
