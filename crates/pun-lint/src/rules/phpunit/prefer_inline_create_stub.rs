//! `lint/phpunit/prefer-inline-create-stub`, ported from rector-phpunit's `InlineStubPropertyToCreateStubMethodCallRector`
//! (rules/CodeQuality/Rector/Class_), with `StubPropertyResolver`, `PropertyFetchUsageFinder` and
//! `PropertyFetchFinder::findLocalPropertyFetchesByName`.
//!
//! In a test class with a `setUp()` method, a property declared alone with a `Stub` type, that `setUp()` assigns
//! `createStub(X::class)` to, and that is otherwise only passed as an argument of calls or put in arrays, goes with its
//! assignments in `setUp()`. In each method, a single fetch of it becomes `$this->createStub(\X::class)`; several
//! become a variable (`$nameStub`, or `$name` when the name ends in `stub`) assigned that call before the first
//! statement that uses it.

use mago_syntax::cst::{
    Access, ArrayElement, Call, ClassLikeMember, ClassLikeMemberSelector, Expression, Hint,
    MethodBody, Node, NodeKind, PartialArgument, Property,
};
use pun_problem::{Edit, Safety, Span};

use super::no_bare_mock_assign::scoped;
use super::no_unused_mock_property::local_property_fetches;
use super::{
    AnyCall, arg_value, class_body, class_string, is_in_test_class, remove_class_members, report,
    resolve,
};
use crate::rules::laravel::is_a;
use crate::rules::rector_batch5::remove_statements;
use crate::rules::rector_stmts::{assign, block_list, expression, first_comment, line_indent};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PHPUnitClassName::STUB`.
const STUB: &str = "PHPUnit\\Framework\\MockObject\\Stub";

pub struct PreferInlineCreateStub;

impl Rule for PreferInlineCreateStub {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-inline-create-stub"
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
        let (members, _, _) = class_body(node).unwrap();
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let methods: Vec<_> = members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(m) => Some(m),
                _ => None,
            })
            .collect();
        let Some(set_up) = methods
            .iter()
            .position(|m| m.name.value.eq_ignore_ascii_case(b"setUp"))
        else {
            return;
        };
        let lists: Vec<_> = methods
            .iter()
            .map(|m| match &m.body {
                MethodBody::Concrete(body) => Some(block_list(ctx.source, body)),
                MethodBody::Abstract(_) => None,
            })
            .collect();
        // resolveFromClassMethod: each property's last createStub() class, in the order of its first
        let mut stubs: Vec<(&[u8], Option<String>)> = Vec::new();
        for statement in lists[set_up].iter().flat_map(|l| l.stmts.iter()) {
            let Some(assign) = expression(statement).and_then(assign) else {
                continue;
            };
            let (Some(name), Expression::Call(Call::Method(call))) =
                (property_fetch_name(assign.lhs), unparenthesize(assign.rhs))
            else {
                continue;
            };
            let call = AnyCall::of(Node::MethodCall(call)).unwrap();
            if !call.is_name(&["createStub"]) {
                continue;
            }
            let class = call
                .args()
                .first()
                .and_then(|a| class_string(&scope, arg_value(a)))
                .filter(|c| !c.is_empty() && !c.starts_with('\\'));
            match stubs.iter_mut().find(|(n, _)| *n == name) {
                Some(entry) => entry.1 = class,
                None => stubs.push((name, class)),
            }
        }
        if stubs.is_empty() {
            return;
        }
        let method_roots: Vec<Node<'_, '_>> = methods.iter().map(|m| Node::Method(m)).collect();
        let mut removed_members = vec![false; members.len()];
        let mut removed_stmts: Vec<Vec<bool>> = lists
            .iter()
            .map(|l| vec![false; l.as_ref().map_or(0, |l| l.stmts.len())])
            .collect();
        let mut inserted: Vec<Vec<String>> = lists
            .iter()
            .map(|l| vec![String::new(); l.as_ref().map_or(0, |l| l.stmts.len())])
            .collect();
        let mut edits = Vec::new();
        let mut first = None;
        for (at, member) in members.iter().enumerate() {
            let ClassLikeMember::Property(Property::Plain(property)) = member else {
                continue;
            };
            // matchSoleStubPropertyItem
            if property.items.len() != 1
                || !property
                    .hint
                    .as_ref()
                    .is_some_and(|h| is_stub(ctx, &scope, h))
            {
                continue;
            }
            let name = &property.items.first().unwrap().variable().name[1..];
            let Some((_, class)) = stubs.iter().find(|(n, _)| *n == name) else {
                continue;
            };
            // are there more uses than simple passing to a new instance?
            if local_property_fetches(ctx, &scope, members.iter(), name).saturating_sub(1)
                != in_call_likes_and_arrays(&method_roots, name)
            {
                continue;
            }
            // Rector fails on the file when the class is no string
            let Some(class) = class else {
                return;
            };
            first.get_or_insert(node_span(member));
            // 1. remove property
            removed_members[at] = true;
            // 2. remove property assign in setUp()
            let list = lists[set_up].as_ref().unwrap();
            for (i, statement) in list.stmts.iter().enumerate() {
                if expression(statement).and_then(assign).is_some_and(|a| {
                    property_fetch_name(a.lhs).is_some_and(|p| p.eq_ignore_ascii_case(name))
                }) {
                    removed_stmts[set_up][i] = true;
                }
            }
            // 3. replace property fetch calls, with createStub()
            let create_stub = format!("$this->createStub(\\{class}::class)");
            for m in 0..methods.len() {
                let Some(list) = &lists[m] else {
                    continue;
                };
                let roots: Vec<Node<'_, '_>> = list
                    .stmts
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !removed_stmts[m][*i])
                    .map(|(_, s)| Node::Statement(s))
                    .collect();
                let all = fetches(&roots, name);
                if all.is_empty() {
                    continue;
                }
                // single use → inline directly
                let fetches: Vec<Span> = all
                    .iter()
                    .filter(|f| {
                        !all.iter()
                            .any(|o| o != *f && o.start <= f.start && f.end <= o.end)
                    })
                    .copied()
                    .collect();
                if all.len() == 1 {
                    edits.push(Edit::new(fetches[0], create_stub.clone()));
                    continue;
                }
                // multiple uses → assign to a local variable above first use and reuse it
                let variable = if String::from_utf8_lossy(name)
                    .to_lowercase()
                    .ends_with("stub")
                {
                    String::from_utf8_lossy(name).into_owned()
                } else {
                    format!("{}Stub", String::from_utf8_lossy(name))
                };
                for span in &fetches {
                    edits.push(Edit::new(*span, format!("${variable}")));
                }
                if let Some(i) = list.stmts.iter().enumerate().position(|(i, s)| {
                    !removed_stmts[m][i]
                        && (uses_variable(Node::Statement(s), variable.as_bytes())
                            || fetches.iter().any(|f| {
                                let span = node_span(*s);
                                span.start <= f.start && f.end <= span.end
                            }))
                }) {
                    let start = node_span(list.stmts[i]).start;
                    let indent = line_indent(ctx.source, start);
                    inserted[m][i].push_str(&format!("${variable} = {create_stub};\n{indent}"));
                }
            }
        }
        let Some(first) = first else {
            return;
        };
        edits.extend(remove_class_members(ctx.source, node, &removed_members));
        for (m, list) in lists.iter().enumerate() {
            let Some(list) = list else {
                continue;
            };
            for (i, text) in inserted[m].iter().enumerate() {
                if !text.is_empty() {
                    let start = node_span(list.stmts[i]).start;
                    let at = first_comment(ctx.source, list.before(ctx.source, i), start)
                        .unwrap_or(start);
                    edits.push(Edit::new(Span::new(at, at), text.clone()));
                }
            }
            if removed_stmts[m].contains(&true) {
                edits.extend(remove_statements(ctx.source, list, &removed_stmts[m]));
            }
        }
        out.push(report(
            first,
            "Create the stub where it is used instead of in a property.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `isObjectType($property->type, new ObjectType(Stub))` for a declared type.
fn is_stub(ctx: &Context<'_>, scope: &[Node<'_, '_>], hint: &Hint<'_>) -> bool {
    match hint {
        Hint::Identifier(id) => is_a(ctx, &resolve(scope, id), &[STUB]),
        Hint::Nullable(n) => is_stub(ctx, scope, n.hint),
        Hint::Parenthesized(p) => is_stub(ctx, scope, p.hint),
        Hint::Intersection(i) => is_stub(ctx, scope, i.left) || is_stub(ctx, scope, i.right),
        Hint::Union(u) => {
            let mut members = Vec::new();
            let mut stack = vec![u.left, u.right];
            while let Some(h) = stack.pop() {
                match h {
                    Hint::Union(u) => stack.extend([u.left, u.right]),
                    Hint::Null(_) => {}
                    other => members.push(other),
                }
            }
            !members.is_empty() && members.into_iter().all(|h| is_stub(ctx, scope, h))
        }
        _ => false,
    }
}

/// The name of a `PropertyFetch` (`$any->name`, not `?->`) with an identifier name.
fn property_fetch_name<'a>(expr: &'a Expression<'a>) -> Option<&'a [u8]> {
    match expr {
        Expression::Access(Access::Property(a)) => match &a.property {
            ClassLikeMemberSelector::Identifier(id) => Some(id.value),
            _ => None,
        },
        _ => None,
    }
}

/// `PropertyFetchUsageFinder::findInCallLikes` and `findInArrays`: the arguments of the calls and `new`s, and the
/// array items' values, that are a fetch of `name`, outside closures.
fn in_call_likes_and_arrays(roots: &[Node<'_, '_>], name: &[u8]) -> usize {
    let is_fetch =
        |e: &Expression<'_>| property_fetch_name(e).is_some_and(|p| p.eq_ignore_ascii_case(name));
    let mut count = 0;
    scoped(roots, &mut |node| {
        let values: Vec<&Expression<'_>> = match node {
            Node::FunctionCall(call) => {
                call.argument_list.arguments.iter().map(arg_value).collect()
            }
            Node::MethodCall(call) => call.argument_list.arguments.iter().map(arg_value).collect(),
            Node::NullSafeMethodCall(call) => {
                call.argument_list.arguments.iter().map(arg_value).collect()
            }
            Node::StaticMethodCall(call) => {
                call.argument_list.arguments.iter().map(arg_value).collect()
            }
            Node::Instantiation(new) => new
                .argument_list
                .iter()
                .flat_map(|l| l.arguments.iter())
                .map(arg_value)
                .collect(),
            Node::AnonymousClass(new) => new
                .argument_list
                .iter()
                .flat_map(|l| l.arguments.iter())
                .filter_map(PartialArgument::value)
                .collect(),
            Node::Array(array) => array.elements.iter().filter_map(element_value).collect(),
            Node::LegacyArray(array) => array.elements.iter().filter_map(element_value).collect(),
            _ => Vec::new(),
        };
        count += values.into_iter().filter(|v| is_fetch(v)).count();
    });
    count
}

/// An array item's value.
fn element_value<'a>(element: &'a ArrayElement<'a>) -> Option<&'a Expression<'a>> {
    match element {
        ArrayElement::KeyValue(kv) => Some(kv.value),
        ArrayElement::Value(v) => Some(v.value),
        ArrayElement::Variadic(v) => Some(v.value),
        ArrayElement::Missing(_) => None,
    }
}

/// The spans of the fetches of `name` (`$any->name`) under `roots`, closures included.
fn fetches(roots: &[Node<'_, '_>], name: &[u8]) -> Vec<Span> {
    let mut found = Vec::new();
    let mut stack: Vec<Node<'_, '_>> = roots.iter().rev().copied().collect();
    while let Some(node) = stack.pop() {
        if let Node::PropertyAccess(a) = node
            && matches!(&a.property, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
        {
            found.push(node_span(&node));
        }
        let mut children = Vec::new();
        node.visit_children(|child| children.push(child));
        stack.extend(children.into_iter().rev());
    }
    found
}

/// Whether a variable named `name` is under `node`, closures included.
fn uses_variable(node: Node<'_, '_>, name: &[u8]) -> bool {
    let mut stack = vec![node];
    while let Some(node) = stack.pop() {
        if let Node::DirectVariable(v) = node
            && v.name[1..].eq_ignore_ascii_case(name)
        {
            return true;
        }
        node.visit_children(|child| stack.push(child));
    }
    false
}
