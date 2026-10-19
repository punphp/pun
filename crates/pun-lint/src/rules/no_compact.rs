//! `lint/complexity/no-compact`, ported from Rector 2.6.1's `CompactToVariablesRector` (rules/CodeQuality/Rector/
//! FuncCall) and its `CompactConverter`.
//!
//! `compact('a', 'b')` is `['a' => $a, 'b' => $b]`. Rector also follows a variable whose type is a constant list of
//! strings; without types Pun follows a local variable assigned once, to an array literal of string literals, and
//! never written otherwise.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_batch5::called_function_name;
use super::side_effect::any_node;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoCompact;

impl Rule for NoCompact {
    fn id(&self) -> &'static str {
        "lint/complexity/no-compact"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if !called_function_name(ancestors, call.function)
            .is_some_and(|n| n.eq_ignore_ascii_case("compact"))
        {
            return;
        }
        let args: Vec<&Argument> = call.argument_list.arguments.iter().collect();
        let names = match all_named(&args) {
            Some(names) => names,
            None => {
                let Some(Argument::Positional(first)) = args.first() else {
                    return;
                };
                // Rector reads the first argument's type: an array literal of strings, or a variable holding one.
                let Some(names) =
                    names(first.value).or_else(|| list_variable(ancestors, first.value))
                else {
                    return;
                };
                names
            }
        };
        let items: Vec<String> = names.iter().map(|n| format!("'{n}' => ${n}")).collect();
        out.push(Report {
            span: node_span(call),
            message: "Use an array of the variables instead of `compact()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(
                    node_span(call),
                    format!("[{}]", items.join(", ")),
                )],
            }),
        });
    }
}

/// A string literal's value when it can name a variable.
fn name(expr: &Expression<'_>) -> Option<String> {
    let Expression::Literal(Literal::String(s)) = unparenthesize(expr) else {
        return None;
    };
    let value = std::str::from_utf8(s.value?).ok()?;
    let mut chars = value.chars();
    let first = chars.next()?;
    ((first.is_ascii_alphabetic() || first == '_' || !first.is_ascii())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || !c.is_ascii()))
    .then(|| value.to_string())
}

/// `CompactConverter::hasAllArgumentsNamed`: every argument a string (so `compact()` is `[]`).
fn all_named(args: &[&Argument<'_>]) -> Option<Vec<String>> {
    args.iter()
        .map(|a| match a {
            Argument::Positional(p) if p.ellipsis.is_none() => name(p.value),
            _ => None,
        })
        .collect()
}

/// The names in the array literal of strings a local variable is assigned once and never otherwise written.
fn list_variable(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> Option<Vec<String>> {
    let Expression::Variable(Variable::Direct(v)) = unparenthesize(expr) else {
        return None;
    };
    let (body, params) = ancestors.iter().rev().find_map(|n| match n {
        Node::Function(f) => Some((n, &f.parameter_list)),
        Node::Method(m) => Some((n, &m.parameter_list)),
        Node::Closure(c) => Some((n, &c.parameter_list)),
        _ => None,
    })?;
    if params.parameters.iter().any(|p| p.variable.name == v.name) {
        return None;
    }
    let target = v.name;
    let writes = std::cell::RefCell::new(Vec::new());
    let other = any_node(*body, &|n| match n {
        Node::Assignment(a) if matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(l)) if l.name == target) =>
        {
            writes.borrow_mut().push(
                matches!(a.operator, AssignmentOperator::Assign(_))
                    .then(|| names(a.rhs))
                    .flatten(),
            );
            false
        }
        // A write through `$v[...]` (reads count too: Pun can't tell them apart here).
        Node::ArrayAccess(a) => {
            matches!(unparenthesize(a.array), Expression::Variable(Variable::Direct(l)) if l.name == target)
        }
        Node::ArrayAppend(a) => {
            matches!(unparenthesize(a.array), Expression::Variable(Variable::Direct(l)) if l.name == target)
        }
        Node::Foreach(_) | Node::Global(_) | Node::Static(_) | Node::List(_) => any_node(
            n,
            &|m| matches!(m, Node::DirectVariable(d) if d.name == target),
        ),
        Node::UnaryPrefix(u) => {
            matches!(u.operator, UnaryPrefixOperator::Reference(_))
                && matches!(u.operand, Expression::Variable(Variable::Direct(d)) if d.name == target)
        }
        _ => false,
    });
    let writes = writes.into_inner();
    if other || writes.len() != 1 {
        return None;
    }
    writes.into_iter().next().flatten()
}

/// The names of an array literal of strings.
fn names(expr: &Expression<'_>) -> Option<Vec<String>> {
    let elements: Vec<&ArrayElement> = match unparenthesize(expr) {
        Expression::Array(a) => a.elements.iter().collect(),
        Expression::LegacyArray(a) => a.elements.iter().collect(),
        _ => return None,
    };
    elements
        .iter()
        .map(|e| match e {
            ArrayElement::Value(v) => name(v.value),
            _ => None,
        })
        .collect()
}
