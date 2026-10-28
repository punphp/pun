//! `lint/laravel/prefer-dispatch-helper`, ported from rector-laravel 2.5.0's `DispatchToHelperFunctionsRector`
//! (src/Rector/StaticCall).
//!
//! `Job::dispatch(...)` and `Job::dispatchSync(...)` on a class that uses `Illuminate\Foundation\Bus\Dispatchable`
//! become `dispatch(new \Job(...))` and `dispatch_sync(new \Job(...))`; `Event::dispatch(...)` on a class that uses
//! `Illuminate\Foundation\Events\Dispatchable` becomes `event(new \Event(...))`. The class must be one the codebase
//! knows; `self`, `static` and `parent` stay as they are written.

use mago_syntax::cst::{Expression, Node, NodeKind};

use super::object_type::{name_class, special_class};
use super::print::kept_arguments;
use super::{is_name, member_name, replace};
use crate::rules::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferDispatchHelper;

impl Rule for PreferDispatchHelper {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-dispatch-helper"
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
        let Node::StaticMethodCall(call) = node else {
            return;
        };
        let Some(name) = member_name(&call.method) else {
            return;
        };
        let sync = is_name(name, "dispatchSync");
        if !sync && !is_name(name, "dispatch") {
            return;
        }
        let (class, written) = match call.class {
            Expression::Identifier(id) => (
                name_class(ancestors, id),
                format!("\\{}", super::names::resolve_class_name(ancestors, id)),
            ),
            special @ (Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) => (
                special_class(ancestors, special),
                ctx.source[node_span(special).start..node_span(special).end].to_string(),
            ),
            _ => return,
        };
        let Some(class) = class else { return };
        let facts = facts(ctx);
        let Some(metadata) = facts.codebase.get_class_like(class.as_bytes()) else {
            return;
        };
        let uses = |t: &str| {
            (metadata.used_traits.iter()).any(|u| u.as_bytes().eq_ignore_ascii_case(t.as_bytes()))
        };
        let helper = if uses("Illuminate\\Foundation\\Bus\\Dispatchable") {
            if sync { "dispatch_sync" } else { "dispatch" }
        } else if uses("Illuminate\\Foundation\\Events\\Dispatchable") {
            "event"
        } else {
            return;
        };
        let args = kept_arguments(ctx.source, &call.argument_list, node_span(call).start).join(", ");
        out.push(replace(
            node_span(call),
            format!("Use the `{helper}()` helper instead of the static `dispatch` method."),
            format!("{helper}(new {written}({args}))"),
        ));
    }
}
