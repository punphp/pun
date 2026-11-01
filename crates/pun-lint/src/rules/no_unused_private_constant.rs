//! `lint/dead-code/no-unused-private-constant`, ported from Rector 2.6.1's `RemoveUnusedPrivateClassConstantRector`
//! (rules/DeadCode/Rector/ClassConst) and `ClassConstManipulator::hasClassConstFetch` (src/NodeManipulator).
//!
//! A private constant declared alone (`private const A = 1;`) in a class, an anonymous class or an enum is removed
//! when no `self::A`, `static::A`, `ClassName::A` or fetch on an object of the class reads it in the class or its
//! ancestors. Pun reads the ancestors declared in the same file; a class that uses a trait declared elsewhere is
//! skipped, as the trait may read the constant. A parent class can never read a private constant of its child, so
//! one declared elsewhere changes nothing. A class whose parent's name ends in `Enum` is skipped (`hasParentClassOfEnumSuffix`).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::class_method::qualified;
use super::rector_batch5::{remove_members, resolve_class_name};
use super::side_effect::any_node;
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUnusedPrivateConstant;

impl Rule for NoUnusedPrivateConstant {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-private-constant"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass, NodeKind::Enum])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (members, left_brace, right_brace, name, extends) = match node {
            Node::Class(c) => (
                &c.members,
                c.left_brace,
                c.right_brace,
                Some(c.name.value),
                c.extends.as_ref(),
            ),
            Node::AnonymousClass(c) => (
                &c.members,
                c.left_brace,
                c.right_brace,
                None,
                c.extends.as_ref(),
            ),
            Node::Enum(e) => (
                &e.members,
                e.left_brace,
                e.right_brace,
                Some(e.name.value),
                None,
            ),
            _ => return,
        };
        let class = name.map(|n| qualified(ancestors, n));
        let parent = extends
            .and_then(|e| e.types.first())
            .map(|id| resolve_class_name(ancestors, id));
        if parent.as_ref().is_some_and(|p| p.ends_with("Enum")) {
            return;
        }
        // `getAncestors()`: the class, its parents and the traits it uses, as far as this file declares them.
        let mut scopes = vec![node];
        let mut next = parent;
        while let Some(name) = next.take() {
            let Some(Node::Class(c)) = declared(ancestors[0], &name) else {
                break;
            };
            next = c
                .extends
                .as_ref()
                .and_then(|e| e.types.first())
                .map(|id| resolve_class_name(ancestors, id));
            scopes.push(Node::Class(c));
        }
        for member in members.iter() {
            let ClassLikeMember::TraitUse(r#use) = member else {
                continue;
            };
            for name in r#use.trait_names.iter() {
                match declared(ancestors[0], &resolve_class_name(ancestors, name)) {
                    Some(t) => scopes.push(t),
                    None => return,
                }
            }
        }
        for (index, member) in members.iter().enumerate() {
            let ClassLikeMember::Constant(constant) = member else {
                continue;
            };
            if !constant
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Private(_)))
                || constant.items.len() != 1
            {
                continue;
            }
            let constant_name = constant.items.first().unwrap().name.value;
            let fetched = scopes.iter().any(|scope| {
                any_node(*scope, &|n| match n {
                    Node::ClassConstantAccess(fetch) => {
                        is_name_match(ctx, ancestors, fetch, class.as_deref(), constant_name)
                    }
                    _ => false,
                })
            });
            if fetched {
                continue;
            }
            let mut removed = vec![false; members.len()];
            removed[index] = true;
            out.push(Report {
                span: node_span(constant),
                message: format!(
                    "Remove the private constant `{}`, which nothing reads.",
                    String::from_utf8_lossy(constant_name)
                ),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: remove_members(ctx.source, members, left_brace, right_brace, &removed),
                }),
            });
        }
    }
}

/// `isNameMatch`: `self::NAME`, `static::NAME` or `Class::NAME` (names compare without case), or any constant of an
/// expression of the class's type (`$this::NAME`, `$this::{$name}`).
fn is_name_match(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    fetch: &ClassConstantAccess<'_>,
    class: Option<&str>,
    name: &[u8],
) -> bool {
    let named = match &fetch.constant {
        ClassLikeConstantSelector::Identifier(id) => Some(id.value.eq_ignore_ascii_case(name)),
        _ => None,
    };
    match fetch.class {
        Expression::Self_(_) | Expression::Static(_) => named.unwrap_or(true),
        Expression::Identifier(id) => {
            named.unwrap_or(true)
                && class.is_some_and(|c| resolve_class_name(ancestors, id).eq_ignore_ascii_case(c))
        }
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => named.unwrap_or(true),
        Expression::Parent(_) => false,
        object => {
            // `isObjectType($fetch->class, new ObjectType($className))` on the native type of the object.
            let Some(class) = class else { return false };
            let facts = facts(ctx);
            let is_class = facts.native_type(node_span(object)).is_some_and(|t| {
                t.types.iter().all(|a| {
                    matches!(a, TAtomic::Object(TObject::Named(n))
                        if facts.codebase.is_instance_of(n.name.as_bytes(), class.as_bytes()))
                })
            });
            is_class && named.unwrap_or(true)
        }
    }
}

/// The class or trait the file declares under the fully qualified name `name`.
fn declared<'ast, 'arena>(program: Node<'ast, 'arena>, name: &str) -> Option<Node<'ast, 'arena>> {
    let mut found = None;
    find(program, &mut Vec::new(), name, &mut found);
    found
}

fn find<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    path: &mut Vec<Node<'ast, 'arena>>,
    name: &str,
    found: &mut Option<Node<'ast, 'arena>>,
) {
    let declared = match node {
        Node::Class(c) => Some(c.name.value),
        Node::Trait(t) => Some(t.name.value),
        _ => None,
    };
    if let Some(declared) = declared
        && qualified(path, declared).eq_ignore_ascii_case(name)
    {
        *found = Some(node);
        return;
    }
    path.push(node);
    node.visit_children(|child| {
        if found.is_none() {
            find(child, path, name, found);
        }
    });
    path.pop();
}
