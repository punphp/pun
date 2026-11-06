//! `lint/dead-code/no-typed-property-instanceof`, ported from Rector 2.6.1's `RemoveTypedPropertyDeadInstanceOfRector`
//! (rules/DeadCode/Rector/If_).
//!
//! In a class's methods, an `if` without `elseif` or `else` whose condition is `$this->property instanceof SomeClass`
//! where the property's type is already that class (or a subclass of it) always runs: its statements take its place;
//! `!$this->property instanceof SomeClass` never runs, so the `if` goes. The property must be promoted, assigned in
//! the constructor, or declared in the class with a native type. Rector stops at the first loop it meets walking the
//! methods in order (`STOP_TRAVERSAL`), so no `if` inside or after a loop changes; an `if` it changes is not walked
//! into.
//!
//! The property's type is PHPStan's with docblocks read; for a private property without a type or `@var` tag,
//! PHPStan (as Rector configures it, `inferPrivatePropertyTypeFromConstructor`) takes the type of the last value the
//! constructor's own statements assign to it.
//!
//! php-parser's printer decides the bytes: see [`super::stmt_splice`].

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Access, AssignmentOperator, BinaryOperator, ClassLikeMember, ClassLikeMemberSelector,
    Expression, Extends, If, MethodBody, Modifier, Node, NodeKind, Property, Sequence, Statement,
    UnaryPrefixOperator, Variable,
};
use pun_problem::{Fix, Safety};

use super::class_method::walk;
use super::no_redundant_property_default::is_assigned_in_constructor;
use super::php_parser::{Token, tokens};
use super::rector_batch5::{method, resolve_class_name};
use super::rector_conditions::if_parts;
use super::stmt_splice::{Change, NewStatement, run_edits};
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoTypedPropertyInstanceof;

impl Rule for NoTypedPropertyInstanceof {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-typed-property-instanceof"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::If])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::If(r#if) = node else { return };
        let Some(class) = ancestors.iter().enumerate().rev().find_map(|(i, a)| {
            let (members, left_brace, extends) = match a {
                Node::Class(c) => (&c.members, c.left_brace, c.extends.as_ref()),
                Node::AnonymousClass(c) => (&c.members, c.left_brace, c.extends.as_ref()),
                _ => return None,
            };
            Some(Class {
                node: *a,
                ancestors: &ancestors[..i],
                members,
                left_brace,
                extends,
            })
        }) else {
            return;
        };
        let source = ctx.source;
        let tokens = tokens(ctx);
        let facts = facts(ctx);
        let visited = visited_ifs(ctx, &tokens, &facts, &class);
        if !visited.contains(&node_span(r#if).start) {
            return;
        }
        let edits = run_edits(source, &tokens, ancestors, &|chain| {
            let Node::Statement(Statement::If(sibling)) = chain[chain.len() - 1] else {
                return None;
            };
            if !visited.contains(&node_span(sibling).start) {
                return None;
            }
            change(ctx, &tokens, &facts, &class, sibling, chain)
        });
        if let Some(edits) = edits {
            out.push(Report {
                span: node_span(r#if),
                message: "This `instanceof` is always true for the property's type.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits,
                }),
            });
        }
    }
}

/// The class Rector refactors, and the nodes above it.
struct Class<'c, 'ast, 'arena> {
    node: Node<'ast, 'arena>,
    ancestors: &'c [Node<'ast, 'arena>],
    members: &'ast Sequence<'arena, ClassLikeMember<'arena>>,
    left_brace: mago_span::Span,
    extends: Option<&'ast Extends<'arena>>,
}

/// Where the `if`s the walk of the class's methods changes start: it stops at the first loop and does not go into an
/// `if` it changes.
fn visited_ifs(
    ctx: &Context<'_>,
    tokens: &[Token],
    facts: &Facts,
    class: &Class<'_, '_, '_>,
) -> Vec<usize> {
    let mut changed = Vec::new();
    let mut stopped = false;
    for member in class.members.iter() {
        let ClassLikeMember::Method(m) = member else {
            continue;
        };
        let mut chain = class.ancestors.to_vec();
        chain.push(class.node);
        chain.push(Node::ClassLikeMember(member));
        walk(Node::Method(m), &mut |node, path| {
            if stopped {
                return false;
            }
            match node {
                Node::While(_) | Node::Foreach(_) | Node::For(_) | Node::DoWhile(_) => {
                    stopped = true;
                    false
                }
                Node::Statement(Statement::If(r#if)) => {
                    let mut full = chain.clone();
                    full.extend_from_slice(path);
                    full.push(node);
                    if change(ctx, tokens, facts, class, r#if, &full).is_some() {
                        changed.push(node_span(r#if).start);
                        return false;
                    }
                    true
                }
                _ => true,
            }
        });
        if stopped {
            break;
        }
    }
    changed
}

/// `refactorStmtAndInstanceof` for the `if` statement `chain` ends with.
fn change(
    ctx: &Context<'_>,
    tokens: &[Token],
    facts: &Facts,
    class: &Class<'_, '_, '_>,
    r#if: &If<'_>,
    chain: &[Node<'_, '_>],
) -> Option<Change> {
    let source = ctx.source;
    let parts = if_parts(tokens, r#if);
    if parts.has_else || parts.has_elseifs {
        return None;
    }
    let (instanceof, negated) = match unparenthesize(r#if.condition) {
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
            match unparenthesize(u.operand) {
                Expression::Binary(b) if matches!(b.operator, BinaryOperator::Instanceof(_)) => {
                    (b, true)
                }
                _ => return None,
            }
        }
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::Instanceof(_)) => (b, false),
        _ => return None,
    };
    // `$this->property` only
    let Expression::Access(Access::Property(fetch)) = unparenthesize(instanceof.lhs) else {
        return None;
    };
    if !matches!(unparenthesize(fetch.object), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
    {
        return None;
    }
    let Expression::Identifier(name) = instanceof.rhs else {
        return None;
    };
    let ClassLikeMemberSelector::Identifier(property) = &fetch.property else {
        return None;
    };
    let wanted = resolve_class_name(chain, name);
    let t = property_type(source, facts, class, node_span(fetch), property.value)?;
    let own = match t.types.as_ref() {
        [TAtomic::Object(TObject::Named(n))] => n.name,
        [TAtomic::Object(TObject::Enum(e))] => e.get_name(),
        _ => return None,
    };
    let same = own.as_bytes().eq_ignore_ascii_case(wanted.as_bytes())
        || (facts.codebase.get_class_like(own.as_bytes()).is_some()
            && facts
                .codebase
                .is_instance_of(own.as_bytes(), wanted.as_bytes()));
    if !same {
        return None;
    }
    if !is_promoted(class, property.value) && is_skipped(ctx, class, property.value) {
        return None;
    }
    if negated || parts.stmts.is_empty() {
        return Some(Change::Remove);
    }
    Some(Change::Replace(NewStatement::all(
        source,
        tokens,
        &parts.stmts,
    )))
}

/// `getType($this->property)`: the type with docblocks read; for a private property without a type or `@var` tag in
/// a class with its own constructor, the type of the value the constructor last assigns to it in a statement of its
/// own (`inferPrivatePropertyTypeFromConstructor`).
fn property_type(
    source: &str,
    facts: &Facts,
    class: &Class<'_, '_, '_>,
    fetch: pun_problem::Span,
    name: &[u8],
) -> Option<TUnion> {
    let members = class.members;
    let declared = members.iter().find_map(|m| match m {
        ClassLikeMember::Property(Property::Plain(p))
            if p.items.iter().any(|i| &i.variable().name[1..] == name) =>
        {
            Some(p)
        }
        _ => None,
    });
    let inferred = declared.is_some_and(|p| {
        p.hint.is_none()
            && p.modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Private(_)))
            && !has_var_tag(source, node_span(p).start, members, class)
    });
    if !inferred {
        return facts
            .full_type(fetch)
            .or_else(|| facts.native_type(fetch))
            .cloned();
    }
    let constructor = method(members, b"__construct")?;
    let MethodBody::Concrete(body) = &constructor.body else {
        return None;
    };
    let mut found = None;
    for statement in body.statements.iter() {
        let Statement::Expression(e) = statement else {
            continue;
        };
        let Expression::Assignment(a) = e.expression else {
            continue;
        };
        if !matches!(a.operator, AssignmentOperator::Assign(_)) {
            continue;
        }
        let Expression::Access(Access::Property(target)) = a.lhs else {
            continue;
        };
        let this = matches!(target.object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this");
        if this
            && matches!(&target.property, ClassLikeMemberSelector::Identifier(id) if id.value == name)
        {
            let span = node_span(unparenthesize(a.rhs));
            if let Some(t) = facts.full_type(span).or_else(|| facts.native_type(span)) {
                found = Some(t.clone());
            }
        }
    }
    found
}

/// Whether the doc comment before the property at `start` has a `@var` tag.
fn has_var_tag(
    source: &str,
    start: usize,
    members: &mago_syntax::cst::Sequence<'_, ClassLikeMember<'_>>,
    class: &Class<'_, '_, '_>,
) -> bool {
    let index = members
        .iter()
        .position(|m| node_span(m).start == start)
        .unwrap_or(0);
    super::rector_batch5::doc_comment(
        source,
        super::rector_batch5::member_from(members, class.left_brace, index),
        start,
    )
    .is_some_and(|d| source[d.start..d.end].contains("@var"))
}

/// `isInPropertyPromotedParams`: a promoted parameter of the constructor is named like the property.
fn is_promoted(class: &Class<'_, '_, '_>, name: &[u8]) -> bool {
    method(class.members, b"__construct").is_some_and(|m| {
        m.parameter_list
            .parameters
            .iter()
            .any(|p| !p.modifiers.is_empty() && &p.variable.name[1..] == name)
    })
}

/// `isSkippedPropertyFetch`: not assigned in the constructor, and not declared in the class with a native type.
fn is_skipped(ctx: &Context<'_>, class: &Class<'_, '_, '_>, name: &[u8]) -> bool {
    let mut path = class.ancestors.to_vec();
    path.push(class.node);
    if is_assigned_in_constructor(
        ctx,
        class.members,
        class.left_brace,
        class.extends,
        &path,
        name,
    ) {
        return false;
    }
    !class.members.iter().any(|m| {
        matches!(m, ClassLikeMember::Property(Property::Plain(p))
            if p.hint.is_some() && p.items.iter().any(|i| &i.variable().name[1..] == name))
    })
}
