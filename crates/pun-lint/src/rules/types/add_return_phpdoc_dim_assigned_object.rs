//! `lint/types/add-return-phpdoc-dim-assigned-object`, ported from Rector 2.6.1's
//! `AddReturnDocblockForArrayDimAssignedObjectRector` (rules/TypeDeclarationDocblocks/Rector/ClassMethod): a function
//! or method that returns a variable it only fills with `$items[] = new Item(...)` gets `@return Item[]`.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_phpstan::types::{Type, mixed};

use super::class_scope;
use super::docblock::{FunctionLike, class_name, doc_report, variable_name};
use super::facts::facts;
use super::silent_void::Place;
use crate::rules::phpdoc_writer::type_mapper::{is_mixed, is_useful_array_tag};
use crate::rules::phpdoc_writer::{DocTarget, decorate_return, expr_type};
use crate::{Context, Report, Rule};

pub struct AddReturnPhpdocDimAssignedObject;

impl Rule for AddReturnPhpdocDimAssignedObject {
    fn id(&self) -> &'static str {
        "lint/types/add-return-phpdoc-dim-assigned-object"
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
        let Some(f) = FunctionLike::of(node) else {
            return;
        };
        if f.has_non_array_return_type() {
            return;
        }
        let target = f.target(ctx);
        let mut info = target.info(ctx.source);
        if let Some(tag) = info.return_tag()
            && let Type::Array(_, item) = DocTarget::doc_type(tag, ancestors, class_name(ancestors).as_deref())
            && !is_mixed(&item)
        {
            return;
        }
        if is_useful_array_tag(info.return_tag().map(|t| &t.ty)) {
            return;
        }
        let facts = facts(ctx);
        let scope = class_scope(ancestors);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let Some(returned) = f.only_return_with_expr(&place).and_then(|r| r.value) else {
            return;
        };
        let Some(name) = variable_name(returned) else {
            return;
        };
        let mut inner = ancestors.to_vec();
        inner.push(node);
        // (a method with a `return` has a body)
        let body = f.body.expect("a body");
        if !is_exclusively_dim_assigned(ctx, &inner, Node::Block(body), name) {
            return;
        }
        // `matchArrayObjectType()`: an `ArrayType`, which PHPStan gives a variable appended to in a loop (appended to
        // only outside loops it is a constant array, which Mago's list type does not tell)
        let Some(Type::Array(_, item)) =
            expr_type(ctx, &inner, returned).filter(|_| appended_in_loop(Node::Block(body), name, false))
        else {
            return;
        };
        let Type::Object(class, args, _) = *item else {
            return;
        };
        if decorate_return(&mut info, &Type::array(mixed(), Type::Object(class, args, false))) {
            out.extend(doc_report(
                f.name,
                "Add the `@return` array type of the objects appended to the returned variable.",
                ctx.source,
                &target,
                &info,
            ));
        }
    }
}

/// `isVariableExclusivelyArrayDimAssigned()`: `$name` is only appended objects of a known class, assigned `[]`, and
/// returned.
fn is_exclusively_dim_assigned(
    ctx: &Context<'_>,
    scope: &[Node<'_, '_>],
    body: Node<'_, '_>,
    name: &str,
) -> bool {
    let mut exclusive = true;
    visit(body, &mut |n| {
        match n {
            Node::Assignment(a) if matches!(a.operator, mago_syntax::cst::AssignmentOperator::Assign(_)) => {
                let dim = match a.lhs {
                    Expression::ArrayAccess(d) => Some((d.array, true)),
                    Expression::ArrayAppend(d) => Some((d.array, false)),
                    _ => None,
                };
                if let Some((var, has_dim)) = dim {
                    if variable_name(var).is_none() {
                        exclusive = false;
                        return true;
                    }
                    if variable_name(var) == Some(name) {
                        if has_dim {
                            exclusive = false;
                        }
                        let assigned = expr_type(ctx, scope, a.rhs);
                        let known = |class: &str| facts_has_class(ctx, class);
                        if !matches!(&assigned, Some(Type::Object(class, _, _)) if known(class)) {
                            exclusive = false;
                        }
                        // ignore lower value
                        return false;
                    }
                }
                if variable_name(a.lhs) == Some(name)
                    && let Expression::Array(arr) = a.rhs
                {
                    if arr.elements.is_empty() {
                        // ignore empty array assignment
                        return false;
                    }
                    exclusive = false;
                }
                true
            }
            Node::Return(r) if r.value.is_some_and(|v| variable_name(v).is_some()) => {
                if variable_name(r.value.unwrap()) == Some(name) {
                    return false;
                }
                exclusive = false;
                true
            }
            Node::DirectVariable(v) if &v.name[1..] == name.as_bytes() => {
                exclusive = false;
                true
            }
            _ => true,
        }
    });
    exclusive
}

/// Whether `$name[] = ...` is in a loop under `node` (`in_loop`: `node` is in one).
fn appended_in_loop(node: Node<'_, '_>, name: &str, in_loop: bool) -> bool {
    let mut found = false;
    node.visit_children(|child| {
        let in_loop = in_loop || matches!(child, Node::Foreach(_) | Node::For(_) | Node::While(_) | Node::DoWhile(_));
        found |= in_loop
            && matches!(child, Node::Assignment(a) if matches!(a.lhs, Expression::ArrayAppend(d) if variable_name(d.array) == Some(name)));
        found |= appended_in_loop(child, name, in_loop);
    });
    found
}

/// `NonExistingObjectType` is a class the reflection does not know.
fn facts_has_class(ctx: &Context<'_>, class: &str) -> bool {
    facts(ctx).codebase.get_class_like(class.as_bytes()).is_some()
}

/// `traverseNodesWithCallable()` over a function's statements: `f` says whether to go into a node's children.
fn visit<'ast, 'arena>(node: Node<'ast, 'arena>, f: &mut dyn FnMut(Node<'ast, 'arena>) -> bool) {
    node.visit_children(|child| {
        if f(child) {
            visit(child, f);
        }
    });
}
