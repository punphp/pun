//! `lint/complexity/prefer-property-default`, ported from Rector 2.6.1's `InlineConstructorDefaultToPropertyRector`
//! (rules/CodeQuality/Rector/Class_).
//!
//! A constructor statement `$this->name = <value known at compile time>;` becomes the default value of the property
//! `name`, and goes from the constructor. The statement's comments move above the property, after its own. Rector
//! looks at the constructor's statements in order and stops at the first `if`, which may break the flow; what it
//! changed before stays. A readonly property or class, or a property a property hook reads, is left alone.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::is_dynamic_expr;
use super::rector_batch5::{merge, method, remove_statements};
use super::rector_stmts::{block_list, line_indent, reindent, tokens};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferPropertyDefault;

impl Rule for PreferPropertyDefault {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-property-default"
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
        let (members, modifiers) = match node {
            Node::Class(c) => (&c.members, &c.modifiers),
            Node::AnonymousClass(c) => (&c.members, &c.modifiers),
            _ => return,
        };
        let source = ctx.source;
        let Some(constructor) = method(members, b"__construct") else {
            return;
        };
        let MethodBody::Concrete(block) = &constructor.body else {
            return;
        };
        if modifiers.iter().any(|m| matches!(m, Modifier::Readonly(_))) {
            return;
        }
        let list = block_list(source, block);
        // Per property item: the new default and the comments to move, in order.
        let mut moved: Vec<(&PropertyItem, &Property, String, Vec<Span>)> = Vec::new();
        let mut removed = vec![false; list.stmts.len()];
        for (index, statement) in list.stmts.iter().enumerate() {
            // Code that may break the flow.
            if matches!(statement, Statement::If(_)) {
                break;
            }
            let Statement::Expression(e) = statement else {
                continue;
            };
            let Expression::Assignment(assign) = unparenthesize(e.expression) else {
                continue;
            };
            if !matches!(assign.operator, AssignmentOperator::Assign(_))
                || matches!(assign.rhs, Expression::UnaryPrefix(u) if u.operator.is_reference())
            {
                continue;
            }
            let Some(name) = assigned_property(assign.lhs) else {
                continue;
            };
            if is_dynamic_expr(assign.rhs) || read_in_hooks(members, name) {
                continue;
            }
            let Some((property, item)) = find_property(members, name) else {
                continue;
            };
            let from = list.before(ctx, index);
            let comments: Vec<Span> = tokens(source, from, node_span(*statement).start)
                .into_iter()
                .filter(|t| t.kind.is_comment())
                .map(|t| Span::new(t.start, t.end))
                .collect();
            let code = reindent(
                source,
                node_span(unparenthesize(assign.rhs)),
                line_indent(source, node_span(*statement).start),
                line_indent(source, node_span(property).start),
            );
            match moved.iter_mut().find(|(i, ..)| std::ptr::eq(*i, item)) {
                Some(entry) => {
                    entry.2 = code;
                    entry.3.extend(comments);
                }
                None => moved.push((item, property, code, comments)),
            }
            removed[index] = true;
        }
        if moved.is_empty() {
            return;
        }
        let mut edits = remove_statements(ctx, &list, &removed);
        // The comments, per property declaration.
        let mut comments: Vec<(&Property, Vec<Span>)> = Vec::new();
        for (item, property, code, item_comments) in &moved {
            edits.push(match item {
                PropertyItem::Concrete(c) => Edit::new(node_span(c.value), code.clone()),
                PropertyItem::Abstract(a) => {
                    let end = node_span(&a.variable).end;
                    Edit::new(Span::new(end, end), format!(" = {code}"))
                }
            });
            match comments
                .iter_mut()
                .find(|(p, _)| std::ptr::eq(*p, *property))
            {
                Some((_, list)) => list.extend(item_comments.iter().copied()),
                None => comments.push((property, item_comments.clone())),
            }
        }
        for (property, list) in comments {
            if list.is_empty() {
                continue;
            }
            let start = node_span(property).start;
            let indent = line_indent(source, start);
            let text: String = list
                .iter()
                .map(|c| {
                    format!(
                        "{}\n{indent}",
                        reindent(source, *c, line_indent(source, c.start), indent)
                    )
                })
                .collect();
            edits.push(Edit::new(Span::new(start, start), text));
        }
        let edits = merge(edits);
        if edits.windows(2).any(|w| w[0].span.end > w[1].span.start) {
            return;
        }
        out.push(Report {
            span: node_span(constructor),
            message: "Make these constant constructor assignments the properties' default values."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `matchAssignedLocalPropertyName`: `$this->name`.
fn assigned_property<'a>(lhs: &'a Expression<'a>) -> Option<&'a [u8]> {
    let Expression::Access(Access::Property(fetch)) = unparenthesize(lhs) else {
        return None;
    };
    let Expression::Variable(Variable::Direct(var)) = fetch.object else {
        return None;
    };
    let ClassLikeMemberSelector::Identifier(name) = &fetch.property else {
        return None;
    };
    (var.name.eq_ignore_ascii_case(b"$this")).then_some(name.value)
}

/// `isFoundInAnyPropertyHooks`: a `$this->name` fetch in a property hook.
fn read_in_hooks(members: &Sequence<'_, ClassLikeMember<'_>>, name: &[u8]) -> bool {
    members.iter().any(|m| match m {
        ClassLikeMember::Property(Property::Hooked(p)) => {
            fetches(Node::PropertyHookList(&p.hook_list), name)
        }
        _ => false,
    })
}

fn fetches(node: Node<'_, '_>, name: &[u8]) -> bool {
    if let Node::PropertyAccess(a) = node
        && assigned_property_access(a, name)
    {
        return true;
    }
    let mut found = false;
    node.visit_children(|child| found = found || fetches(child, name));
    found
}

fn assigned_property_access(a: &PropertyAccess<'_>, name: &[u8]) -> bool {
    matches!(a.object, Expression::Variable(Variable::Direct(v)) if v.name.eq_ignore_ascii_case(b"$this"))
        && matches!(&a.property, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
}

/// The first property item named `name` in a declaration that is not readonly.
fn find_property<'a>(
    members: &'a Sequence<'a, ClassLikeMember<'a>>,
    name: &[u8],
) -> Option<(&'a Property<'a>, &'a PropertyItem<'a>)> {
    members.iter().find_map(|m| {
        let ClassLikeMember::Property(property) = m else {
            return None;
        };
        let (modifiers, items): (_, Vec<&PropertyItem>) = match property {
            Property::Plain(p) => (&p.modifiers, p.items.iter().collect()),
            Property::Hooked(p) => (&p.modifiers, vec![&p.item]),
        };
        // A readonly property cannot have a default value.
        if modifiers.iter().any(|m| matches!(m, Modifier::Readonly(_))) {
            return None;
        }
        items
            .into_iter()
            .find(|i| i.variable().name[1..].eq_ignore_ascii_case(name))
            .map(|i| (property, i))
    })
}
