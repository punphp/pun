//! `lint/dead-code/no-unused-private-property`, ported from Rector 2.8.4's `RemoveUnusedPrivatePropertyRector`
//! (rules/DeadCode/Rector/Property), with `PropertyWriteonlyAnalyzer`, `PropertyFetchFinder` and the
//! `AssignedToNodeVisitor` marks it reads.
//!
//! A private, non-static property declared alone, without attributes or a Doctrine annotation, goes when the class
//! never reads it: every fetch of it is the target of a plain assignment. The assignments go with it: an assignment
//! statement is removed (its value too), `return $this->p = value;` returns the value, and an argument
//! `f($this->p = value)` passes the value. A class that uses a trait, has a `loadMetadata()` method, fetches a
//! property by a dynamic name or calls `get_object_vars($this)` is skipped.
//!
//! Rector reads a fetch as the class's own by its type. Pun reads `$this->p`, `$this?->p`, `self::$p` and the like as
//! the class's, `$this->q->p` as another object's when `q` is declared with a class type, and any other fetch of that
//! name (`$other->p`, a fetch in a nested class) as a possible read that keeps the property. Rector also removes a
//! property whose assignment it cannot remove (`if ($this->p = f())`), leaving an assignment to an undeclared
//! property; Pun keeps such a property.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_batch5::{
    doc_comment, imported, member_from, merge, remove_members, remove_statements,
};
use super::rector_code_quality::{named, text};
use super::rector_stmts::{List, enclosing};
use crate::syntax::is_this;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUnusedPrivateProperty;

impl Rule for NoUnusedPrivateProperty {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-private-property"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (members, left_brace, right_brace, own) = match node {
            Node::Class(class) => (
                &class.members,
                class.left_brace,
                class.right_brace,
                Some(class.name.value),
            ),
            Node::AnonymousClass(class) => {
                (&class.members, class.left_brace, class.right_brace, None)
            }
            _ => return,
        };
        if skip_class(node, members) {
            return;
        }
        let source = ctx.source;
        let mut path = ancestors.to_vec();
        path.push(node);
        let typed = typed_properties(members, own);
        let mut removed = vec![false; members.len()];
        let mut writes: Vec<Write> = Vec::new();
        for (index, member) in members.iter().enumerate() {
            let ClassLikeMember::Property(property) = member else {
                continue;
            };
            let Some(name) =
                removable_property(source, &path, members, left_brace, index, property)
            else {
                continue;
            };
            let mut fetches = Fetches {
                source,
                name,
                typed: &typed,
                writes: Vec::new(),
                read: false,
            };
            let mut stack = path.clone();
            for m in members {
                if matches!(
                    m,
                    ClassLikeMember::Method(_) | ClassLikeMember::Property(Property::Hooked(_))
                ) {
                    stack.push(Node::ClassLikeMember(m));
                    fetches.walk(&mut stack, false);
                    stack.pop();
                }
                if fetches.read {
                    break;
                }
            }
            if fetches.read {
                continue;
            }
            removed[index] = true;
            writes.extend(fetches.writes);
        }
        if !removed.contains(&true) {
            return;
        }
        let mut edits = remove_members(source, members, left_brace, right_brace, &removed);
        // The statements to remove, grouped by the list that holds them.
        let mut lists: Vec<(List, Vec<usize>)> = Vec::new();
        for write in writes {
            match write {
                Write::Statement(index, list) => {
                    match lists.iter_mut().find(|(l, _)| l.inner == list.inner) {
                        Some((_, indices)) => indices.push(index),
                        None => lists.push((list, vec![index])),
                    }
                }
                Write::Value(assignment) => {
                    edits.push(Edit::new(
                        node_span(assignment),
                        text(ctx, unparenthesize(assignment.rhs)).to_string(),
                    ));
                }
            }
        }
        for (list, indices) in &lists {
            let flags: Vec<bool> = (0..list.stmts.len())
                .map(|i| indices.contains(&i))
                .collect();
            edits.extend(remove_statements(ctx, list, &flags));
        }
        let edits = merge(edits);
        if edits.windows(2).any(|w| w[0].span.end > w[1].span.start) {
            return;
        }
        out.push(Report {
            span: node_span(&node),
            message: "Remove the private properties this class never reads.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `shouldSkipClass`: a `loadMetadata()` method, a trait, a fetch by a dynamic name or `get_object_vars($this)`.
fn skip_class(node: Node<'_, '_>, members: &Sequence<'_, ClassLikeMember<'_>>) -> bool {
    members.iter().any(|m| match m {
        ClassLikeMember::TraitUse(_) => true,
        ClassLikeMember::Method(m) => m.name.value.eq_ignore_ascii_case(b"loadMetadata"),
        _ => false,
    }) || dynamic(node)
}

/// `hasClassDynamicPropertyNames`.
fn dynamic(node: Node<'_, '_>) -> bool {
    let selector_dynamic =
        |s: &ClassLikeMemberSelector<'_>| !matches!(s, ClassLikeMemberSelector::Identifier(_));
    match node {
        Node::PropertyAccess(a) if selector_dynamic(&a.property) => return true,
        Node::NullSafePropertyAccess(a) if selector_dynamic(&a.property) => return true,
        Node::FunctionCall(call) if named(call, "get_object_vars") => {
            if let Some(Argument::Positional(first)) = call.argument_list.arguments.first()
                && matches!(first.value, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
            {
                return true;
            }
        }
        _ => {}
    }
    let mut found = false;
    node.visit_children(|child| found = found || dynamic(child));
    found
}

/// The declared properties with a class type other than the class itself: `$this->q->p` fetches another object's
/// `p` when `q` is one of them.
fn typed_properties<'a>(
    members: &'a Sequence<'a, ClassLikeMember<'a>>,
    own: Option<&[u8]>,
) -> Vec<&'a [u8]> {
    let mut out = Vec::new();
    for member in members {
        let ClassLikeMember::Property(Property::Plain(p)) = member else {
            continue;
        };
        let Some(Hint::Identifier(id)) = &p.hint else {
            continue;
        };
        let value = id.value();
        let short = value.rsplit(|b| *b == b'\\').next().unwrap_or(value);
        if own.is_some_and(|own| own.eq_ignore_ascii_case(short)) {
            continue;
        }
        for item in &p.items {
            out.push(&item.variable().name[1..]);
        }
    }
    out
}

/// `shouldSkipProperty`'s checks; the property's name (without `$`) when it may go.
fn removable_property<'a>(
    source: &str,
    path: &[Node<'_, '_>],
    members: &Sequence<'_, ClassLikeMember<'_>>,
    left_brace: mago_span::Span,
    index: usize,
    property: &'a Property<'a>,
) -> Option<&'a [u8]> {
    let (attributes, modifiers, item) = match property {
        Property::Plain(p) => {
            if p.items.len() != 1 {
                return None;
            }
            (&p.attribute_lists, &p.modifiers, p.items.first()?)
        }
        Property::Hooked(p) => (&p.attribute_lists, &p.modifiers, &p.item),
    };
    if !attributes.is_empty()
        || !modifiers.iter().any(|m| matches!(m, Modifier::Private(_)))
        || modifiers.iter().any(|m| matches!(m, Modifier::Static(_)))
    {
        return None;
    }
    let start = node_span(property).start;
    if let Some(doc) = doc_comment(source, member_from(members, left_brace, index), start)
        && has_doctrine_tag(path, &source[doc.start..doc.end])
    {
        return None;
    }
    Some(&item.variable().name[1..])
}

/// Whether a doc comment has a tag Rector reads as a Doctrine annotation (`DoctrineAnnotationDecorator`): a tag whose
/// name, resolved through the `use` imports, is a namespaced class, or `@Target`.
fn has_doctrine_tag(path: &[Node<'_, '_>], doc: &str) -> bool {
    doc.lines().any(|line| {
        let line = line
            .trim_start()
            .trim_start_matches("/**")
            .trim_start_matches('*')
            .trim_start();
        let Some(tag) = line.strip_prefix('@') else {
            return false;
        };
        let name: String = tag
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '\\' || *c == '-')
            .collect();
        name.contains('\\')
            || name == "Target"
            || (name.starts_with(|c: char| c.is_ascii_uppercase())
                && imported(path, &name).is_some_and(|full| full.contains('\\')))
    })
}

/// An assignment that goes with the property: a statement of a list, or an assignment whose value stays.
enum Write<'a> {
    Statement(usize, List<'a>),
    Value(&'a Assignment<'a>),
}

struct Fetches<'t, 'a> {
    source: &'t str,
    name: &'a [u8],
    typed: &'t [&'a [u8]],
    writes: Vec<Write<'a>>,
    read: bool,
}

impl<'a> Fetches<'_, 'a> {
    /// Walks the node `stack` ends with; `nested` inside a nested class.
    fn walk(&mut self, stack: &mut Vec<Node<'a, 'a>>, nested: bool) {
        if self.read {
            return;
        }
        let node = *stack.last().unwrap();
        let nested = nested
            || (stack.len() > 1
                && matches!(
                    node,
                    Node::Class(_)
                        | Node::AnonymousClass(_)
                        | Node::Interface(_)
                        | Node::Trait(_)
                        | Node::Enum(_)
                ));
        match node {
            Node::PropertyAccess(a) if self.names(&a.property) => {
                if nested || !is_this(a.object) {
                    self.read |= !self.other_object(a.object) || nested;
                } else {
                    self.local(stack);
                }
                return;
            }
            Node::NullSafePropertyAccess(a) if self.names(&a.property) => {
                if nested || !is_this(a.object) {
                    self.read |= !self.other_object(a.object) || nested;
                } else {
                    // `$this?->p = ...` is not an assignment target.
                    self.read = true;
                }
                return;
            }
            Node::StaticPropertyAccess(a) => {
                if matches!(&a.property, Variable::Direct(v) if v.name[1..].eq_ignore_ascii_case(self.name))
                {
                    self.read = true;
                    return;
                }
            }
            _ => {}
        }
        let mut children = Vec::new();
        node.visit_children(|child| children.push(child));
        for child in children {
            stack.push(child);
            self.walk(stack, nested);
            stack.pop();
        }
    }

    fn names(&self, selector: &ClassLikeMemberSelector<'_>) -> bool {
        matches!(selector, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(self.name))
    }

    /// Whether the object of a fetch is another class's object: `$this->q` with `q` declared with another class type.
    fn other_object(&self, object: &Expression<'_>) -> bool {
        match unparenthesize(object) {
            Expression::Access(Access::Property(q)) => {
                is_this(q.object)
                    && matches!(&q.property, ClassLikeMemberSelector::Identifier(id) if self.typed.contains(&id.value))
            }
            _ => false,
        }
    }

    /// A fetch of the class's property, at the end of `stack`: a read unless it is the target of a plain assignment
    /// (not part of a chain) that Pun can remove.
    fn local(&mut self, stack: &[Node<'a, 'a>]) {
        match self.write(stack) {
            Some(write) => self.writes.push(write),
            None => self.read = true,
        }
    }

    fn write(&self, stack: &[Node<'a, 'a>]) -> Option<Write<'a>> {
        let fetch = node_span(stack.last()?);
        // The nodes above the fetch, without Mago's wrappers.
        let above: Vec<(usize, Node<'a, 'a>)> = stack[..stack.len() - 1]
            .iter()
            .copied()
            .enumerate()
            .rev()
            .filter(|(_, n)| {
                !matches!(
                    n,
                    Node::Expression(_) | Node::Access(_) | Node::Parenthesized(_)
                )
            })
            .collect();
        let (_, Node::Assignment(assignment)) = above.first()? else {
            return None;
        };
        let plain = matches!(assignment.operator, AssignmentOperator::Assign(_))
            && !matches!(assignment.rhs, Expression::UnaryPrefix(u) if u.operator.is_reference());
        if !plain
            || node_span(unparenthesize(assignment.lhs)) != fetch
            || matches!(unparenthesize(assignment.rhs), Expression::Assignment(_))
        {
            return None;
        }
        match above.get(1)? {
            (_, Node::Assignment(_)) => None,
            (_, Node::Return(_) | Node::PositionalArgument(_) | Node::NamedArgument(_)) => {
                Some(Write::Value(assignment))
            }
            (i, Node::ExpressionStatement(_)) => {
                let Some(Node::Statement(statement)) = stack.get(i - 1) else {
                    return None;
                };
                let (list, index) = enclosing(self.source, statement, &stack[..i - 1]);
                Some(Write::Statement(index, list))
            }
            _ => None,
        }
    }
}
