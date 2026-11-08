//! `OptionalToNullsafeOperatorRector` from rector-laravel 2.7.3.

use mago_syntax::cst::{
    Access, ClassLikeMemberSelector, CompositeString, Expression, Node, NodeKind,
};
use pun_problem::{Edit, Span};

use super::{is_name, is_written, member_name, replace};
use crate::rules::rector_code_quality::{Prec, call_to, is_constant, prec, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

#[derive(Default)]
pub struct PreferNullsafe {
    pub exclude_methods: Vec<String>,
}

impl Rule for PreferNullsafe {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-nullsafe"
    }

    fn min_php_version(&self) -> u32 {
        80000
    }

    /// `configure`: `exclude-methods` (rector-laravel's `exclude_methods`), the method names a call to keeps
    /// `optional()`.
    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        match options {
            serde_json::Value::Null => Ok(()),
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    let names = (key == "exclude-methods")
                        .then(|| value.as_array())
                        .flatten()
                        .and_then(|a| {
                            a.iter()
                                .map(|v| v.as_str().map(str::to_string))
                                .collect::<Option<Vec<_>>>()
                        });
                    match names {
                        Some(names) => self.exclude_methods = names,
                        None => {
                            return Err(format!(
                                "unknown option `{key}` or a value of the wrong type"
                            ));
                        }
                    }
                }
                Ok(())
            }
            _ => Err("options must be a table".into()),
        }
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::PropertyAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (object, arrow, selector) = if let Node::MethodCall(m) = node {
            if member_name(&m.method).is_some_and(|name| {
                self.exclude_methods
                    .iter()
                    .any(|excluded| is_name(name, excluded))
            }) {
                return;
            }
            (m.object, m.arrow, &m.method)
        } else {
            let Node::PropertyAccess(p) = node else {
                return;
            };
            (p.object, p.arrow, &p.property)
        };
        let Some(optional) = call_to(object, "optional") else {
            return;
        };
        let args = &optional.argument_list.arguments;
        let Some(first) = args.get(0) else { return };
        if args.get(1).is_some_and(|a| !is_constant(a.value(), "null")) {
            return;
        }
        // Unlike a call, a nullsafe property is legal inside isset(). Still reject an assignment within it.
        let ancestors = ancestors
            .iter()
            .rposition(|n| matches!(n, Node::IssetConstruct(_)))
            .map_or(ancestors, |i| &ancestors[i + 1..]);
        let value = unparenthesize(first.value());
        if matches!(
            value,
            Expression::Literal(_)
                | Expression::CompositeString(
                    CompositeString::Interpolated(_) | CompositeString::Document(_)
                )
                | Expression::ConstantAccess(_)
                | Expression::Array(_)
                | Expression::LegacyArray(_)
                | Expression::Access(Access::ClassConstant(_))
                | Expression::MagicConstant(_)
        ) || is_written(node_span(&node), ancestors)
        {
            return;
        }
        let code = text(ctx, value);
        let code = if prec(value) != Prec::None
            || matches!(
                value,
                Expression::Instantiation(_) | Expression::CompositeString(_)
            ) {
            format!("({code})")
        } else {
            code.to_string()
        };
        let mut report = replace(
            node_span(&node),
            "Use the nullsafe operator instead of `optional()`.".to_string(),
            code.clone(),
        );
        report.fix.as_mut().unwrap().edits = vec![
            Edit::new(node_span(object), code),
            Edit::new(
                Span::new(arrow.start.offset as usize, arrow.end.offset as usize),
                "?->",
            ),
        ];
        if let ClassLikeMemberSelector::Variable(variable) = selector {
            let span = node_span(variable);
            report.fix.as_mut().unwrap().edits.push(Edit::new(
                span,
                format!("{{{}}}", &ctx.source[span.start..span.end]),
            ));
        }
        out.push(report);
    }
}
