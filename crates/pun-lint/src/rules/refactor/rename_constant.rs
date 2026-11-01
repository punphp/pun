//! `lint/refactor/rename-constant`, ported from Rector 2.6.1's `RenameConstantRector`
//! (rules/Renaming/Rector/ConstFetch).
//!
//! Options: a map of old constant names to new ones, `{ MYSQL_ASSOC = "MYSQLI_ASSOC" }`; both must be plain constant
//! names (`RectorAssert::constantName`). A constant fetch whose name, as PHPStan's name resolution leaves it (see
//! [`super::name_of`]), is an old one compared without case gets the new name as written.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{Kind, assert_constant, is_name, name_of, string_map};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

#[derive(Default)]
pub struct RenameConstant {
    pub renames: Vec<(String, String)>,
}

impl Rule for RenameConstant {
    fn id(&self) -> &'static str {
        "lint/refactor/rename-constant"
    }

    /// Rector runs the rule once per file; its fix could match again on what it wrote.
    fn once(&self) -> bool {
        true
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        let renames = string_map(options)?;
        for (old, new) in &renames {
            assert_constant(old)?;
            assert_constant(new)?;
        }
        self.renames = renames;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ConstantAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ConstantAccess(constant) = node else {
            return;
        };
        let resolved = name_of(ancestors, &constant.name, Kind::Constant);
        let Some((old, new)) = self.renames.iter().find(|(old, _)| is_name(&resolved, old)) else {
            return;
        };
        // Renaming a constant to its own name would only change its case, again in every fix round.
        if resolved == *new {
            return;
        }
        out.push(Report::fixed(
            node_span(constant),
            format!("Rename the constant `{old}` to `{new}`."),
            Safety::Unsafe,
            vec![Edit::new(node_span(&constant.name), new.clone())],
        ));
    }
}
