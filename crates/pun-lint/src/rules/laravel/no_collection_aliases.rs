//! `lint/laravel/no-collection-aliases`, ported from rector-laravel 2.5.0's `UnaliasCollectionMethodsRector`
//! (src/Rector/MethodCall).
//!
//! On an `Illuminate\Support\Enumerable`, `some()` becomes `contains()`, `average()` `avg()`, `unlessEmpty()`
//! `whenNotEmpty()` and `unlessNotEmpty()` `whenEmpty()`.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::object_type::is_object_type;
use super::{is_name, member_name};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoCollectionAliases;

impl Rule for NoCollectionAliases {
    fn id(&self) -> &'static str {
        "lint/laravel/no-collection-aliases"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else { return };
        let Some(name) = member_name(&call.method) else {
            return;
        };
        let replacement = [
            ("some", "contains"),
            ("average", "avg"),
            ("unlessEmpty", "whenNotEmpty"),
            ("unlessNotEmpty", "whenEmpty"),
        ]
        .into_iter()
        .find(|(alias, _)| is_name(name, alias));
        let Some((alias, replacement)) = replacement else {
            return;
        };
        if !is_object_type(
            ctx,
            ancestors,
            call.object,
            "Illuminate\\Support\\Enumerable",
        ) {
            return;
        }
        let span = node_span(&call.method);
        out.push(Report::fixed(
            span,
            format!("Use `{replacement}()` instead of its alias `{alias}()`."),
            Safety::Unsafe,
            vec![Edit::new(span, replacement)],
        ));
    }
}
