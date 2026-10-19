//! `lint/functions/no-required-after-optional`, ported from Rector 2.6.1's `OptionalParametersAfterRequiredRector`
//! (rules/CodeQuality/Rector/ClassMethod).
//!
//! PHP deprecates a required parameter after an optional one. Each such parameter of a method, function or closure
//! gets a default (and with it every required parameter after it, which now follows an optional one): `null` for an
//! untyped, nullable or `null`-including parameter, a zero value for `string`, `int`, `float`, `bool`, `array`,
//! `true` or `false` (the first of a union's members that has one), and `null` with the type made nullable for the
//! rest. A variadic parameter stays.

use mago_syntax::cst::{FunctionLikeParameterList, Hint, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoRequiredAfterOptional;

impl Rule for NoRequiredAfterOptional {
    fn id(&self) -> &'static str {
        "lint/functions/no-required-after-optional"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::NULLABLE_TYPE
        70100
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method, NodeKind::Function, NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let mut list = None;
        node.visit_children(|child| {
            if let Node::FunctionLikeParameterList(l) = child {
                list = Some(l);
            }
        });
        let Some(list) = list else { return };
        let edits = edits(ctx.source, list);
        if edits.is_empty() {
            return;
        }
        out.push(Report {
            span: node_span(list),
            message: "Give the required parameters after an optional one a default value."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}

fn edits(source: &str, list: &FunctionLikeParameterList<'_>) -> Vec<Edit> {
    let mut edits = Vec::new();
    let mut previous_optional = false;
    for parameter in list.parameters.iter() {
        if parameter.default_value.is_some() || parameter.ellipsis.is_some() {
            previous_optional = parameter.default_value.is_some();
            continue;
        }
        if !previous_optional {
            continue;
        }
        // `processParam()`: from here on every parameter has a default.
        let end = node_span(&parameter.variable).end;
        let mut default = "null";
        match &parameter.hint {
            None | Some(Hint::Nullable(_)) => {}
            Some(hint @ Hint::Intersection(_)) => {
                let s = node_span(hint);
                edits.push(Edit::new(s, format!("({})|null", &source[s.start..s.end])));
            }
            Some(hint @ Hint::Union(_)) => {
                let mut members = Vec::new();
                flatten(hint, &mut members);
                if !members.iter().any(|m| matches!(m, Hint::Null(_))) {
                    match members.iter().find_map(|m| reasonable(m)) {
                        Some(value) => default = value,
                        None => {
                            let at = node_span(hint).end;
                            edits.push(Edit::new(Span::new(at, at), "|null"));
                        }
                    }
                }
            }
            Some(hint) => match reasonable(hint) {
                Some(value) => default = value,
                None if matches!(hint, Hint::Null(_) | Hint::Mixed(_)) => {}
                None => {
                    let at = node_span(hint).start;
                    edits.push(Edit::new(Span::new(at, at), "?"));
                }
            },
        }
        edits.push(Edit::new(Span::new(end, end), format!(" = {default}")));
        previous_optional = true;
    }
    edits
}

/// A union's members, in order.
fn flatten<'a>(hint: &'a Hint<'a>, out: &mut Vec<&'a Hint<'a>>) {
    match hint {
        Hint::Union(union) => {
            flatten(union.left, out);
            flatten(union.right, out);
        }
        other => out.push(other),
    }
}

/// `mapReasonableParamValue()` for one type.
fn reasonable(hint: &Hint<'_>) -> Option<&'static str> {
    match hint {
        Hint::String(_) => Some("''"),
        Hint::Integer(_) => Some("0"),
        Hint::Float(_) => Some("0.0"),
        Hint::Bool(_) | Hint::False(_) => Some("false"),
        Hint::Array(_) => Some("[]"),
        Hint::True(_) => Some("true"),
        _ => None,
    }
}
