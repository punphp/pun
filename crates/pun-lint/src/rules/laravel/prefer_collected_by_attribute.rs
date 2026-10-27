//! `lint/laravel/prefer-collected-by-attribute`, ported from rector-laravel 2.5.0's
//! `CollectedByPropertyToCollectedByAttributeRector` (src/Rector/Class_).
//!
//! A model (or anonymous model) without `#[CollectedBy]` whose `newCollection()` method has one statement,
//! `return new Name(...);`, loses the method and gets `#[\Illuminate\Database\Eloquent\Attributes\CollectedBy(Name::class)]`,
//! the name as written. `new static(...)` is left alone: Rector writes `static::class`, which PHP rejects in an
//! attribute.

use mago_syntax::cst::{Expression, MethodBody, Node, NodeKind, Statement};

use super::property_attribute::{ClassView, MODEL, report, rewrite};
use crate::rules::rector_stmts::block_list;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const COLLECTED_BY: &str = "Illuminate\\Database\\Eloquent\\Attributes\\CollectedBy";

pub struct PreferCollectedByAttribute;

impl Rule for PreferCollectedByAttribute {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-collected-by-attribute"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let class = ClassView::of(node).unwrap();
        if !class.is_object_type(ctx, ancestors, MODEL)
            || class.has_attribute(ancestors, COLLECTED_BY)
        {
            return;
        }
        let Some((index, method)) = class.method("newCollection") else {
            return;
        };
        let Some(value) = collected_by_value(ctx.source, method) else {
            return;
        };
        let group = format!("#[\\{COLLECTED_BY}({value})]");
        let Some(edits) = rewrite(ctx.source, &class, &[group], &[], &[index]) else {
            return;
        };
        out.push(report(
            &class,
            "Changes model newCollection method to use the CollectedBy attribute.",
            edits,
        ));
    }
}

/// `resolveCollectedByValue()`: `Name::class` for a body that is only `return new Name(...);`.
fn collected_by_value(source: &str, method: &mago_syntax::cst::Method<'_>) -> Option<String> {
    let MethodBody::Concrete(block) = &method.body else {
        return None;
    };
    let list = block_list(source, block);
    if list.len() != 1 {
        return None;
    }
    let Statement::Return(r) = list.stmts[0] else {
        return None;
    };
    let Some(Expression::Instantiation(new)) = r.value.map(unparenthesize) else {
        return None;
    };
    let name = match new.class {
        Expression::Identifier(id) => node_span(id),
        Expression::Self_(k) | Expression::Parent(k) => node_span(k),
        _ => return None,
    };
    Some(format!("{}::class", &source[name.start..name.end]))
}
