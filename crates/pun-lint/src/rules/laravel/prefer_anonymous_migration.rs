//! `lint/laravel/prefer-anonymous-migration`, ported from Mago 1.57.0's `prefer-anonymous-migration` (mago-linter
//! src/rule/best_practices/prefer_anonymous_migration.rs).
//!
//! A named class that extends `Illuminate\Database\Migrations\Migration` (the resolved name, in any case) is
//! reported: migrations are anonymous classes (`return new class extends Migration { ... };`).

use mago_syntax::cst::{Node, NodeKind};

use crate::rules::lint_b_names::resolve_class;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferAnonymousMigration;

impl Rule for PreferAnonymousMigration {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-anonymous-migration"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Class(class) = node else {
            return;
        };
        let Some(parent) = class.extends.as_ref().and_then(|e| e.types.first()) else {
            return;
        };
        if resolve_class(ancestors, parent)
            .eq_ignore_ascii_case(b"Illuminate\\Database\\Migrations\\Migration")
        {
            out.push(Report {
                span: node_span(class),
                message: "Use anonymous classes for migrations instead of named classes."
                    .to_string(),
                help: Some(
                    "Refactor the migration to use an anonymous class by removing the class name."
                        .to_string(),
                ),
                fix: None,
            });
        }
    }
}
