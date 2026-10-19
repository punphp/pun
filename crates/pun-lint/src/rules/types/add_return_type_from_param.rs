//! `lint/types/add-return-type-from-param`, ported from Rector 2.6.1's `ReturnTypeFromStrictParamRector`
//! (rules/TypeDeclaration/Rector/ClassMethod) and its `AddReturnTypeFromParam`: a function or method whose last
//! `return` returns a typed parameter that the body never assigns, and whose returns have one native type (`null`
//! aside), gets that parameter's type, as written, as its return type. Mago does not narrow an `int` by truthiness
//! as PHPStan does (`int<min, -1>|int<1, max>`), so `if ($b) { return $b; } return $a;` with two `int` parameters
//! is typed by Pun and left alone by Rector.

use mago_syntax::cst::{
    Expression, FunctionLikeParameterList, Node, NodeKind, Statement, UnaryPrefixOperator, Variable,
};

use super::facts::facts;
use super::native::{Kind, returned_constants, returned_kinds};
use super::silent_void::Place;
use super::{add_return_type, any_scoped, class_scope, returns_scoped, untyped};
use crate::rules::rector_code_quality::text;
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromParam;

/// `PhpVersionFeature::NULLABLE_TYPE`.
const NULLABLE_TYPE: u32 = 70100;

impl Rule for AddReturnTypeFromParam {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-param"
    }

    fn min_php_version(&self) -> u32 {
        NULLABLE_TYPE
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let parameters = match node {
            Node::Function(f) => &f.parameter_list,
            Node::Method(m) => &m.parameter_list,
            _ => return,
        };
        let Some(f) = untyped(node, ancestors, ctx) else {
            return;
        };
        // shouldSkipNode: the returns' native types, without null, must be one type
        let scope = class_scope(ancestors);
        let facts = facts(ctx);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let returns = returns_scoped(f.body);
        let Some(kinds) = returned_kinds(&place, f.body, &returns) else {
            return;
        };
        if kinds.iter().filter(|k| **k != Kind::Null).count() > 1
            || returned_constants(&place, &returns).is_none_or(|keys| keys.len() > 1)
        {
            return;
        }
        let Some(name) = current_scope_return(f.body.statements.iter()) else {
            return;
        };
        let Some(hint) = typed_parameter(parameters, name) else {
            return;
        };
        if is_assigned(f.body, name) {
            return;
        }
        // the parameter's type node, printed as written
        out.push(add_return_type(f.span, f.at, text(ctx, hint)));
    }
}

/// `findCurrentScopeReturn`: the variable the last `return` returns, walking the body in order. A `return` of
/// anything else ends the walk with none, and so does, until a later `return`, a nested function or class.
fn current_scope_return<'a>(
    statements: impl Iterator<Item = &'a Statement<'a>>,
) -> Option<&'a [u8]> {
    fn walk<'a>(node: Node<'a, 'a>, found: &mut Option<&'a [u8]>) -> bool {
        match node {
            Node::Function(_)
            | Node::Closure(_)
            | Node::ArrowFunction(_)
            | Node::Method(_)
            | Node::Class(_)
            | Node::AnonymousClass(_) => {
                *found = None;
                false
            }
            Node::Return(r) => match r.value {
                Some(Expression::Variable(Variable::Direct(v))) => {
                    *found = Some(v.name);
                    false
                }
                _ => {
                    *found = None;
                    true
                }
            },
            _ => {
                let mut stop = false;
                node.visit_children(|child| stop = stop || walk(child, found));
                stop
            }
        }
    }
    let mut found = None;
    for statement in statements {
        if walk(Node::Statement(statement), &mut found) {
            break;
        }
    }
    found
}

/// The type of the parameter `$name`, when it has one.
fn typed_parameter<'a>(
    parameters: &'a FunctionLikeParameterList<'a>,
    name: &[u8],
) -> Option<&'a mago_syntax::cst::Hint<'a>> {
    parameters
        .parameters
        .iter()
        .find(|p| p.variable.name == name)
        .and_then(|p| p.hint.as_ref())
}

/// `shouldSkipParam`: the body assigns to `$name`, or takes a reference to it (`$x = &$name`).
fn is_assigned(body: &mago_syntax::cst::Block<'_>, name: &[u8]) -> bool {
    let is_name = |e: &Expression<'_>| matches!(e, Expression::Variable(Variable::Direct(v)) if v.name == name);
    any_scoped(Node::Block(body), &mut |n| match n {
        Node::Assignment(a) => {
            is_name(a.lhs)
                || matches!(a.rhs, Expression::UnaryPrefix(p)
                    if matches!(p.operator, UnaryPrefixOperator::Reference(_)) && is_name(p.operand))
        }
        _ => false,
    })
}
