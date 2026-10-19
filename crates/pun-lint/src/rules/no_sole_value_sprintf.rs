//! `lint/complexity/no-sole-value-sprintf`, ported from Rector 2.6.1's `RemoveSoleValueSprintfRector`
//! (rules/CodeQuality/Rector/FuncCall).
//!
//! `sprintf('%s', $value)` with a string `$value` is `$value`. Rector asks PHPStan whether the value is a string;
//! without types Pun takes a string literal, an interpolated string, a concatenation, or a local variable that every
//! assignment in its function sets to one of those, or a `string` parameter never set to anything else.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_batch5::called_function_name;
use super::side_effect::any_node;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoSoleValueSprintf;

impl Rule for NoSoleValueSprintf {
    fn id(&self) -> &'static str {
        "lint/complexity/no-sole-value-sprintf"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if !called_function_name(ancestors, call.function)
            .is_some_and(|n| n.eq_ignore_ascii_case("sprintf"))
        {
            return;
        }
        let args: Vec<&Argument> = call.argument_list.arguments.iter().collect();
        let [Argument::Positional(mask), Argument::Positional(value)] = args.as_slice() else {
            return;
        };
        if mask.ellipsis.is_some() || value.ellipsis.is_some() {
            return;
        }
        let Expression::Literal(Literal::String(mask)) = mask.value else {
            return;
        };
        if mask.value != Some(b"%s".as_slice()) {
            return;
        }
        if !is_string(ancestors, value.value) {
            return;
        }
        let source = ctx.source;
        let span = node_span(value.value);
        out.push(Report {
            span: node_span(call),
            message: "`sprintf('%s', ...)` of a string is the string itself.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(node_span(call), &source[span.start..span.end])],
            }),
        });
    }
}

fn is_string_syntax(expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Access(Access::ClassConstant(c)) => {
            matches!(&c.constant, ClassLikeConstantSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"class"))
        }
        Expression::Literal(Literal::String(_)) => true,
        Expression::CompositeString(s) => !matches!(s, CompositeString::ShellExecute(_)),
        Expression::Binary(b) => matches!(b.operator, BinaryOperator::StringConcat(_)),
        _ => false,
    }
}

/// Whether `expr` is a string to the syntax: a string literal, an interpolated string, a concatenation, `X::class`,
/// or a variable that is a `string` parameter or a local, every write of which in its function is one of those.
pub(crate) fn is_string(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    if is_string_syntax(expr) {
        return true;
    }
    let Expression::Variable(Variable::Direct(v)) = unparenthesize(expr) else {
        return false;
    };
    let Some((body, params)) = ancestors.iter().rev().find_map(|n| match n {
        Node::Function(f) => Some((n, &f.parameter_list)),
        Node::Method(m) => Some((n, &m.parameter_list)),
        Node::Closure(c) => Some((n, &c.parameter_list)),
        _ => None,
    }) else {
        return false;
    };
    let param = params.parameters.iter().find(|p| p.variable.name == v.name);
    if param.is_some_and(|p| !matches!(p.hint, Some(Hint::String(_)))) {
        return false;
    }
    if let Node::Closure(c) = body
        && c.use_clause
            .as_ref()
            .is_some_and(|u| u.variables.iter().any(|x| x.variable.name == v.name))
    {
        return false;
    }
    let name = v.name;
    let other_write = any_node(*body, &|n| match n {
        Node::Assignment(a) => {
            matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(l)) if l.name == name)
                && !(matches!(a.operator, AssignmentOperator::Assign(_)) && is_string_syntax(a.rhs))
        }
        // `foreach (... as $v)`, `list($v)`, `&$v`, `global $v`, `static $v`: any other way to set it.
        Node::Foreach(_) | Node::Global(_) | Node::Static(_) | Node::List(_) => any_node(
            n,
            &|m| matches!(m, Node::DirectVariable(d) if d.name == name),
        ),
        Node::UnaryPrefix(u) => {
            matches!(u.operator, UnaryPrefixOperator::Reference(_))
                && matches!(u.operand, Expression::Variable(Variable::Direct(d)) if d.name == name)
        }
        _ => false,
    });
    if other_write {
        return false;
    }
    param.is_some()
        || any_node(*body, &|n| {
            matches!(n, Node::Assignment(a)
            if matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(l)) if l.name == name))
        })
}
