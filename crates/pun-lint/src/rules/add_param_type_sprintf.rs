//! `lint/types/add-param-type-sprintf`, ported from Rector 2.6.1's `AddParamStringTypeFromSprintfUseRector`
//! (rules/TypeDeclaration/Rector/ClassMethod) with `VariableInSprintfMaskMatcher` (rules/TypeDeclaration/NodeAnalyzer)
//! and `ParamTypeAddGuard` (rules/TypeDeclaration/Guard).
//!
//! An untyped parameter (without a default, or with a string default) that a function's only `sprintf()` passes first
//! to a format whose first `%s`/`%d` is `%s` gets the `string` type. A parameter that is assigned, read in an `if`
//! condition or in a ternary is left alone, as is a method a parent class, interface or trait also declares.

use mago_syntax::cst::{
    ArrowFunction, AssignmentOperator, Expression, FunctionLikeParameterList, Literal, MatchArm,
    MethodBody, Modifier, Node, NodeKind, UnaryPrefixOperator, Variable,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::lint_b_helpers::{Constant, constant, value};
use super::rector_code_quality::named;
use super::types::class_scope;
use super::types::facts::facts;
use super::types::has_parent_class_method;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddParamTypeSprintf;

impl Rule for AddParamTypeSprintf {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-sprintf"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::Method,
            NodeKind::Function,
            NodeKind::Closure,
            NodeKind::ArrowFunction,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (params, body): (&FunctionLikeParameterList<'_>, Node<'_, '_>) = match node {
            Node::Method(m) => match &m.body {
                MethodBody::Concrete(b) => (&m.parameter_list, Node::Block(b)),
                MethodBody::Abstract(_) => return,
            },
            Node::Function(f) => (&f.parameter_list, Node::Block(&f.body)),
            Node::Closure(c) => (&c.parameter_list, Node::Block(&c.body)),
            Node::ArrowFunction(ArrowFunction {
                parameter_list,
                expression,
                ..
            }) => (parameter_list, Node::Expression(expression)),
            _ => return,
        };
        if params.parameters.is_empty() {
            return;
        }
        if let Node::Method(m) = node
            && !m
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Private(_)))
            && class_scope(ancestors)
                .is_some_and(|scope| has_parent_class_method(&facts(ctx), &scope, m.name.value))
        {
            return;
        }
        let mut sprintfs = Vec::new();
        scoped_sprintfs(body, &mut sprintfs);
        let [sprintf] = sprintfs.as_slice() else {
            return;
        };
        // The scopes a variable is looked up in: the function's own and the ones around it.
        let scope: Vec<Node<'_, '_>> = ancestors.iter().copied().chain([node]).collect();
        let edits: Vec<Edit> = params
            .parameters
            .iter()
            .filter(|param| {
                param.hint.is_none()
                    && param
                        .default_value
                        .as_ref()
                        .is_none_or(|d| matches!(d.value, Expression::Literal(Literal::String(_))))
                    && is_legal(body, param.variable.name)
                    && matches_mask(&scope, sprintf, param.variable.name)
                    && !narrowed(body, sprintf, param.variable.name)
            })
            .map(|param| {
                let at = param
                    .ampersand
                    .map(|a| a.start.offset)
                    .or(param.ellipsis.map(|e| e.start.offset))
                    .unwrap_or(param.variable.span.start.offset) as usize;
                Edit::new(Span::new(at, at), "string ")
            })
            .collect();
        if edits.is_empty() {
            return;
        }
        out.push(Report {
            span: node_span(params),
            message: "Add the `string` type to parameters passed to `sprintf()` as `%s`."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}

/// `findInstancesOfScoped`: the `sprintf()` calls in `node`, not in a nested function, closure or class.
fn scoped_sprintfs<'a>(node: Node<'a, 'a>, out: &mut Vec<&'a mago_syntax::cst::FunctionCall<'a>>) {
    match node {
        Node::Function(_)
        | Node::Closure(_)
        | Node::ArrowFunction(_)
        | Node::Class(_)
        | Node::AnonymousClass(_)
        | Node::Interface(_)
        | Node::Trait(_)
        | Node::Enum(_) => return,
        Node::FunctionCall(call) if named(call, "sprintf") => out.push(call),
        _ => {}
    }
    node.visit_children(|child| scoped_sprintfs(child, out));
}

/// `VariableInSprintfMaskMatcher::matchMask` with the mask `%s`: `Strings::match` gives only the first `%s` or `%d`
/// of the format, so only the first value argument can match.
fn matches_mask(
    ancestors: &[Node<'_, '_>],
    sprintf: &mago_syntax::cst::FunctionCall<'_>,
    name: &[u8],
) -> bool {
    let args = sprintf.argument_list.arguments.as_slice();
    if args.len() < 2 {
        return false;
    }
    let Some(Constant::String(format)) =
        constant(ancestors, value(&args[0]), node_span(sprintf).start)
    else {
        return false;
    };
    let first = format
        .windows(2)
        .find(|w| w[0] == b'%' && (w[1] == b's' || w[1] == b'd'));
    first == Some(b"%s")
        && matches!(unparenthesize(value(&args[1])), Expression::Variable(Variable::Direct(v)) if v.name == name)
}

/// `ParamTypeAddGuard::isLegal`: the variable is not assigned with `=`, not in an `if` condition and not in a
/// ternary, anywhere in the body.
pub(crate) fn is_legal(body: Node<'_, '_>, name: &[u8]) -> bool {
    fn has_variable(node: Node<'_, '_>, name: &[u8]) -> bool {
        super::side_effect::any_node(
            node,
            &|n| matches!(n, Node::Variable(Variable::Direct(v)) if v.name == name),
        )
    }
    !super::side_effect::any_node(body, &|node| match node {
        Node::Assignment(a) => {
            matches!(a.operator, AssignmentOperator::Assign(_))
                && !matches!(a.rhs, Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Reference(_)))
                && matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(v)) if v.name == name)
        }
        Node::If(i) => has_variable(Node::Expression(i.condition), name),
        Node::Conditional(_) => has_variable(node, name),
        _ => false,
    })
}

/// Whether PHPStan's native type of the variable at the `sprintf()` call is `mixed` without a union of values (skipped
/// by `matchMask`): the call is in an arm of a `match` on the variable after arms of two conditions or more.
// ponytail: only match subjects narrow here; PHPStan narrows through every condition.
fn narrowed(body: Node<'_, '_>, sprintf: &mago_syntax::cst::FunctionCall<'_>, name: &[u8]) -> bool {
    let at = node_span(sprintf);
    super::side_effect::any_node(body, &|node| {
        let Node::Match(m) = node else {
            return false;
        };
        let span = node_span(m);
        let on_variable = matches!(
            unparenthesize(m.expression),
            Expression::Variable(Variable::Direct(v)) if v.name == name
        );
        let before: usize = m
            .arms
            .iter()
            .filter(|arm| node_span(*arm).end <= at.start)
            .map(|arm| match arm {
                MatchArm::Expression(e) => e.conditions.len(),
                MatchArm::Default(_) => 0,
            })
            .sum();
        span.start <= at.start && at.end <= span.end && on_variable && before >= 2
    })
}
