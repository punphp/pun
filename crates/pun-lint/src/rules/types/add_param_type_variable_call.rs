//! `lint/types/add-param-type-variable-call`, ported from Rector 2.6.1's `AddClosureParamTypeFromVariableCallRector`
//! (rules/TypeDeclaration/Rector/FunctionLike): a closure assigned to a variable (`$print = function ($item) {...}`)
//! that the same body calls (`$print(new Item())`) gets, for each untyped parameter, the class of the objects every
//! call passes at its position.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Argument, AssignmentOperator, Closure, Expression, MethodBody, Node, NodeKind, Variable,
};
use pun_problem::Safety;

use super::facts::{Facts, facts};
use super::param::{Flow, add_hint, param_declaration, traverse};
use crate::rules::class_method::type_of;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddParamTypeVariableCall;

impl Rule for AddParamTypeVariableCall {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-variable-call"
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
        let body = match node {
            Node::Method(m) => {
                let MethodBody::Concrete(body) = &m.body else { return };
                Node::Block(body)
            }
            Node::Function(f) => Node::Block(&f.body),
            _ => {
                let Node::Closure(c) = node else { return };
                Node::Block(&c.body)
            }
        };
        // collectAssignedClosures: the last closure each variable is assigned, in the order the variables come.
        let mut closures: Vec<(&[u8], &Closure<'_>)> = Vec::new();
        traverse(body, &mut |node| {
            if let Node::Assignment(assign) = node
                && matches!(assign.operator, AssignmentOperator::Assign(_))
                && let Expression::Variable(Variable::Direct(variable)) = unparenthesize(assign.lhs)
                && let Expression::Closure(closure) = unparenthesize(assign.rhs)
            {
                match closures.iter_mut().find(|(name, _)| *name == variable.name) {
                    Some(entry) => entry.1 = closure,
                    None => closures.push((variable.name, closure)),
                }
            }
            Flow::Continue
        });
        if closures.is_empty() {
            return;
        }
        let facts = facts(ctx);
        let mut edits = Vec::new();
        for (name, closure) in closures {
            let types = call_argument_types(&facts, body, name);
            for (position, param) in closure.parameter_list.parameters.iter().enumerate() {
                let Some(Some(object)) = types.get(position) else {
                    continue;
                };
                if param.hint.is_some() || param.ellipsis.is_some() {
                    continue;
                }
                let object = TUnion::from_vec(vec![object.clone()]);
                let Some(declaration) = param_declaration(&object, &facts.codebase, ctx.php_version) else { continue };
                edits.push(add_hint(param, &declaration));
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report::fixed(
            node_span(&node),
            "Add the class of the objects a closure variable is called with to its parameters.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// What one position of the calls holds: not yet seen, an object type, or blocked.
#[derive(Clone)]
enum Position {
    Unseen,
    Object(TAtomic, Vec<u8>),
    Blocked,
}

/// collectCallArgumentTypes: for each argument position of the calls `$name(...)` in `body` (not in nested functions
/// and classes), the object type (`ObjectType`) every call passes there; `None` where a call unpacks, passes another
/// type, or passes an object of another class.
fn call_argument_types(facts: &Facts, body: Node<'_, '_>, name: &[u8]) -> Vec<Option<TAtomic>> {
    let mut positions: Vec<Position> = Vec::new();
    traverse(body, &mut |node| {
        let Node::FunctionCall(call) = node else {
            return Flow::Continue;
        };
        if !matches!(unparenthesize(call.function), Expression::Variable(Variable::Direct(v)) if v.name == name) {
            return Flow::Continue;
        }
        for (position, arg) in call.argument_list.arguments.iter().enumerate() {
            if positions.len() <= position {
                positions.resize(position + 1, Position::Unseen);
            }
            let value = match arg {
                Argument::Positional(p) if p.ellipsis.is_some() => None,
                Argument::Positional(p) => Some(p.value),
                Argument::Named(n) => Some(n.value),
            };
            let object = value.and_then(|v| object_type(facts, v));
            positions[position] = match (&positions[position], object) {
                (Position::Blocked, _) | (_, None) => Position::Blocked,
                (Position::Object(_, seen), Some((_, class))) if *seen != class => Position::Blocked,
                (_, Some((object, class))) => Position::Object(object, class),
            };
        }
        Flow::Continue
    });
    positions
        .into_iter()
        .map(|p| match p {
            Position::Object(object, _) => Some(object),
            _ => None,
        })
        .collect()
}

/// `getType($expr) instanceof ObjectType`: one object of a named class or an enum (not `static`, `$this` or an
/// intersection), with its class name (`getClassName()`).
fn object_type(facts: &Facts, expr: &Expression<'_>) -> Option<(TAtomic, Vec<u8>)> {
    let t = type_of(facts, expr)?;
    let [atomic] = t.types.as_ref() else {
        return None;
    };
    let class = match atomic {
        TAtomic::Object(TObject::Named(named))
            if !named.is_static && named.intersection_types.is_none() =>
        {
            named.name.as_bytes().to_vec()
        }
        TAtomic::Object(TObject::Enum(e)) => e.get_name().as_bytes().to_vec(),
        _ => return None,
    };
    Some((atomic.clone(), class))
}
