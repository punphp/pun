//! `lint/style/prefer-property-over-local-getter`, ported from Rector 2.6.1's `PrivatizeLocalGetterToPropertyRector`
//! (rules/Privatization/Rector/MethodCall).
//!
//! `$this->getSome()` becomes `$this->some` when `getSome` is a method of the same class, private or of a final
//! class, without parameters, whose only statement returns a property fetch. The fetch is copied as the getter writes
//! it.
//!
//! Rector walks the whole class, the bodies of nested anonymous classes included, and matches their `$this` calls
//! against the outer class. There `$this` is another object, so Pun stops at a nested class (which gets its own
//! visit). Rector also drops the arguments of a call to the getter (`$this->getSome($x)`); Pun leaves such a call
//! alone, since the arguments may have effects.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::text;
use super::rector_stmts::block_list;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferPropertyOverLocalGetter;

impl Rule for PreferPropertyOverLocalGetter {
    fn id(&self) -> &'static str {
        "lint/style/prefer-property-over-local-getter"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let is_final = matches!(node, Node::Class(class)
            if class.modifiers.iter().any(|m| matches!(m, Modifier::Final(_))));
        let mut members = Vec::new();
        node.visit_children(|child| {
            if let Node::ClassLikeMember(member) = child {
                members.push(member);
            }
        });
        let methods: Vec<&Method> = members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(m) => Some(m),
                _ => None,
            })
            .collect();
        let mut edits = Vec::new();
        for member in members {
            node_calls(Node::ClassLikeMember(member), &mut |call| {
                if let Some(fetch) = getter_fetch(ctx, call, &methods, is_final) {
                    edits.push(Edit::new(node_span(call), text(ctx, fetch).to_string()));
                }
            });
        }
        let Some(first) = edits.first() else { return };
        out.push(Report {
            span: first.span,
            message: "Read the property instead of calling its local getter.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}

/// Every method call under `node`, outside nested classes; a call replaced is not searched further (its arguments
/// are none).
fn node_calls<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    f: &mut impl FnMut(&'ast MethodCall<'arena>),
) {
    match node {
        Node::Class(_)
        | Node::AnonymousClass(_)
        | Node::Interface(_)
        | Node::Trait(_)
        | Node::Enum(_) => return,
        Node::MethodCall(call) => f(call),
        _ => {}
    }
    node.visit_children(|child| node_calls(child, f));
}

/// The property fetch `call` turns into: `$this->name()` without arguments, where `name` is a method of the class
/// that is private (or the class is final) and only returns a property fetch.
fn getter_fetch<'ast>(
    ctx: &Context<'_>,
    call: &MethodCall<'_>,
    methods: &[&'ast Method<'ast>],
    is_final: bool,
) -> Option<&'ast Expression<'ast>> {
    let Expression::Variable(Variable::Direct(var)) = call.object else {
        return None;
    };
    if !var.name.eq_ignore_ascii_case(b"$this") || !call.argument_list.arguments.is_empty() {
        return None;
    }
    let ClassLikeMemberSelector::Identifier(name) = &call.method else {
        return None;
    };
    // `Class_::getMethod`: the first method of that name, ignoring ASCII case.
    let method = methods
        .iter()
        .find(|m| m.name.value.eq_ignore_ascii_case(name.value))?;
    let private = method
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Private(_)));
    if !private && !is_final {
        return None;
    }
    local_property_fetch(ctx, method)
}

/// `matchLocalPropertyFetchInGetterMethod`: no parameters and one statement, `return <property fetch>;`.
fn local_property_fetch<'ast>(
    ctx: &Context<'_>,
    method: &'ast Method<'ast>,
) -> Option<&'ast Expression<'ast>> {
    if !method.parameter_list.parameters.is_empty() {
        return None;
    }
    let MethodBody::Concrete(block) = &method.body else {
        return None;
    };
    let list = block_list(ctx.source, block);
    if list.len() != 1 {
        return None;
    }
    let Statement::Return(Return {
        value: Some(value), ..
    }) = list.stmts.first()?
    else {
        return None;
    };
    let value = unparenthesize(value);
    matches!(value, Expression::Access(Access::Property(_))).then_some(value)
}
