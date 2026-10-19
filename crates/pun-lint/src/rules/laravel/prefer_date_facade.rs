//! `lint/laravel/prefer-date-facade`, ported from rector-laravel 2.5.0's `CarbonToDateFacadeRector`
//! (src/Rector/StaticCall).
//!
//! A static call on `Carbon\Carbon` or `Illuminate\Support\Carbon` (or a class the codebase knows extends one) becomes
//! the same call on `\Illuminate\Support\Facades\Date`.

use mago_syntax::cst::{Node, NodeKind};

use super::{argument_texts, call, class_name, is_a, replace};
use crate::rules::rector_code_quality::text;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferDateFacade;

impl Rule for PreferDateFacade {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-date-facade"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::StaticMethodCall(s) = node else {
            return;
        };
        let Some(class) = class_name(s.class, ancestors) else {
            return;
        };
        if !is_a(
            ctx,
            &class,
            &["Carbon\\Carbon", "Illuminate\\Support\\Carbon"],
        ) {
            return;
        }
        out.push(replace(
            node_span(s),
            "Use the `Date` facade instead of calling `Carbon` directly.".to_string(),
            call(
                &format!(
                    "\\Illuminate\\Support\\Facades\\Date::{}",
                    text(ctx, &s.method)
                ),
                &argument_texts(ctx, &s.argument_list),
            ),
        ));
    }
}
