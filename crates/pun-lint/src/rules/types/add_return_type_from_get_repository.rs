//! `lint/types/add-return-type-from-get-repository`, ported from Rector 2.6.1's
//! `ReturnTypeFromGetRepositoryDocblockRector` (rules/TypeDeclaration/Rector/ClassMethod): a method without a return
//! type whose one `return` returns an entity manager's `getRepository()` call (directly or through the variable it
//! was assigned to) returns the existing class its `@return` names, and the `@return` goes.

use mago_syntax::cst::{
    Assignment, AssignmentOperator, Call, ClassLikeMemberSelector, Expression, MethodCall, Node,
    NodeKind,
};
use pun_phpstan::types::Type;
use pun_problem::{Edit, Fix, Safety, Span};

use super::compare::classes::with_classes;
use super::compare::mapper::{TypeScope, expr_type};
use super::compare::tags::{Kind, removed_by_type};
use super::compare::{Object, RType};
use super::docblock::{FunctionLike, variable_name};
use super::facts::{Facts, facts};
use super::{returns_scoped, untyped};
use crate::rules::lint_b_doc::Doc;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromGetRepository;

const ENTITY_MANAGER_INTERFACE: &str = "Doctrine\\ORM\\EntityManagerInterface";

impl Rule for AddReturnTypeFromGetRepository {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-get-repository"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
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
        if f.return_type.is_some() {
            return;
        }
        let facts = facts(ctx);
        if !is_get_repository_return_only(&facts, &f, node, ancestors) {
            return;
        }
        if untyped(node, ancestors, ctx).is_none() {
            return;
        }
        let Some(doc) = Doc::last_of(ctx.source, ancestors, node_span(&node).start) else {
            return;
        };
        let target = f.target(ctx.source);
        let info = target.info(ctx.source);
        let Some(parsed) = info.return_tag().and_then(|tag| tag.parsed.as_ref()) else {
            return;
        };
        let mut chain = ancestors.to_vec();
        chain.push(node);
        let scope = TypeScope::new(&chain, facts.documented());
        let returned = with_classes(facts.documented(), || scope.map_doc(parsed));
        // `mapPHPStanTypeToPhpParserNode()` of an `ObjectType`: a fully qualified name
        let class = match returned {
            RType::Object(Object::FullyQualified(name)) => {
                name.trim_start_matches('\\').to_string()
            }
            RType::Object(Object::Shortened { full, .. } | Object::Aliased { full, .. }) => full,
            RType::T(Type::Object(name, _)) => name,
            _ => return,
        };
        // must be an existing repository class
        if facts.codebase.get_class_like(class.as_bytes()).is_none() {
            return;
        }
        let at = node_span(f.parameters).end;
        let removed = removed_by_type(&doc, Kind::Return, "");
        out.push(Report {
            span: f.name,
            message: format!(
                "Add the `{class}` return type of the repository `getRepository()` returns."
            ),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![
                    doc.remove_last(ctx.source, &removed),
                    Edit::new(Span::new(at, at), format!(": \\{class}")),
                ],
            }),
        });
    }
}

/// `isGetRepositoryReturnOnly()`.
fn is_get_repository_return_only(
    facts: &Facts,
    f: &FunctionLike<'_, '_>,
    node: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
) -> bool {
    let Some(body) = f.body else {
        return false;
    };
    let [only] = returns_scoped(body)[..] else {
        return false;
    };
    let Some(value) = only.value.map(unparenthesize) else {
        return false;
    };
    match value {
        // direct return: return $this->em->getRepository(...);
        Expression::Call(Call::Method(call)) => {
            is_get_repository_method_call(facts, ancestors, call)
        }
        // indirect return: $repo = $this->em->getRepository(...); ...; return $repo;
        _ => {
            let Some(name) = variable_name(value) else {
                return false;
            };
            find_variable_assign_method_call(node, name)
                .is_some_and(|call| is_get_repository_method_call(facts, ancestors, call))
        }
    }
}

/// `isGetRepositoryMethodCall()`: `->getRepository()` on an entity manager (`getType()` of the caller an
/// `ObjectType` of `EntityManagerInterface` or a class that implements it).
fn is_get_repository_method_call(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    call: &MethodCall<'_>,
) -> bool {
    if !matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"getRepository"))
    {
        return false;
    }
    let Type::Object(class, _) = expr_type(facts, ancestors, call.object, false) else {
        return false;
    };
    class == ENTITY_MANAGER_INTERFACE
        || (facts.codebase.get_class_like(class.as_bytes()).is_some()
            && facts
                .codebase
                .is_instance_of(class.as_bytes(), ENTITY_MANAGER_INTERFACE.as_bytes()))
}

/// `findVariableAssignMethodCall()`: the method call the last assignment of `$name` in the method assigns, `None`
/// when an assignment of it assigns anything else.
fn find_variable_assign_method_call<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    name: &str,
) -> Option<&'ast MethodCall<'arena>> {
    let mut assigned = Vec::new();
    collect_assignments(node, &mut assigned);
    let mut call = None;
    for a in assigned {
        if variable_name(a.lhs) != Some(name) {
            continue;
        }
        // reassigned to something else, cannot be sure of the type
        let Expression::Call(Call::Method(c)) = unparenthesize(a.rhs) else {
            return None;
        };
        call = Some(c);
    }
    call
}

/// `NodeFinder::findInstanceOf($node, Assign::class)`: every `=` assignment under `node`, in source order.
fn collect_assignments<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    out: &mut Vec<&'ast Assignment<'arena>>,
) {
    node.visit_children(|child| {
        if let Node::Assignment(a) = child
            && matches!(a.operator, AssignmentOperator::Assign(_))
        {
            out.push(a);
        }
        collect_assignments(child, out);
    });
}
