//! `lint/laravel/prefer-casts-method`, ported from Mago 1.57.0's `prefer-casts-method` (mago-linter
//! src/rule/best_practices/prefer_casts_method.rs).
//!
//! A property `$casts` (the name is case-sensitive) declared without `static` and with an array default, in any
//! class-like, is reported: Laravel 11 reads casts from a `casts()` method. Hooked properties are not looked at.

use mago_syntax::cst::{Expression, Modifier, Node, NodeKind, Property, PropertyItem};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferCastsMethod;

impl Rule for PreferCastsMethod {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-casts-method"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Property])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Property(Property::Plain(property)) = node else {
            return;
        };
        if property
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Static(_)))
        {
            return;
        }
        let casts = property.items.iter().find_map(|item| match item {
            PropertyItem::Concrete(c)
                if c.variable.name == b"$casts"
                    && matches!(c.value, Expression::Array(_) | Expression::LegacyArray(_)) =>
            {
                Some(c)
            }
            _ => None,
        });
        if let Some(casts) = casts {
            out.push(Report {
                span: node_span(&casts.variable),
                message: "Define model casts in the `casts()` method instead of the `$casts` property."
                    .to_string(),
                help: Some(
                    "Move the cast definitions into a `protected function casts(): array` method that returns the array."
                        .to_string(),
                ),
                fix: None,
            });
        }
    }
}
