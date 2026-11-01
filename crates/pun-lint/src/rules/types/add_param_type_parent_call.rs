//! `lint/types/add-param-type-parent-call`, ported from Rector 2.6.1's `ParamTypeByParentCallTypeRector`
//! (rules/TypeDeclaration/Rector/ClassMethod) with `CallerParamMatcher::matchParentParam`: an untyped parameter that a
//! method passes to `parent::<same method>()` gets the type the parent method declares for that position, copied
//! from the parent's source (as Rector's `AstResolver` reads it), class names fully qualified.

use mago_syntax::cst::{
    ClassLikeMember, ClassLikeMemberSelector, Expression, Hint, Method, MethodBody, Node, NodeKind,
    StaticMethodCall, Variable,
};
use pun_problem::Safety;

use super::facts::{Facts, facts};
use super::names::resolve_written_class_name;
use super::param::add_hint;
use crate::rules::class_method::{qualified, walk};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddParamTypeParentCall;

impl Rule for AddParamTypeParentCall {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-parent-call"
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
        let (members, has_parent) = match node {
            Node::Class(c) => (&c.members, c.extends.is_some()),
            _ => {
                let Node::AnonymousClass(c) = node else {
                    return;
                };
                (&c.members, c.extends.is_some())
            }
        };
        // no parent calls available
        if !has_parent {
            return;
        }
        let facts = facts(ctx);
        let class = match node {
            Node::Class(c) => qualified(ancestors, c.name.value),
            _ => facts.anonymous_class_name(mago_span::HasSpan::span(&node)),
        };
        let mut edits = Vec::new();
        for member in members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let MethodBody::Concrete(body) = &method.body else {
                continue;
            };
            // hasAtLeastOneParamWithoutType
            if !method
                .parameter_list
                .parameters
                .iter()
                .any(|p| p.ellipsis.is_none() && p.hint.is_none())
            {
                continue;
            }
            let Some(call) = find_parent_static_call(Node::Block(body), method.name.value) else {
                continue;
            };
            for param in method.parameter_list.parameters.iter() {
                if param.hint.is_some() || param.ellipsis.is_some() {
                    continue;
                }
                let name = param.variable.name;
                if is_used_in_spread_arg(Node::Block(body), name) {
                    continue;
                }
                let Some(position) = call.argument_list.arguments.iter().position(|a| {
                    matches!(a.value(), Expression::Variable(Variable::Direct(v)) if v.name == name)
                }) else {
                    continue;
                };
                if let Some(hint) = parent_param_hint(&facts, &class, method.name.value, position) {
                    edits.push(add_hint(param, &hint));
                }
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report::fixed(
            node_span(&node),
            "Add the parameter types of the parent method the call passes them to.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// `findParentStaticCall`: the first `parent::<method>()` call anywhere in the method.
fn find_parent_static_call<'ast, 'arena>(
    body: Node<'ast, 'arena>,
    method: &[u8],
) -> Option<&'ast StaticMethodCall<'arena>> {
    let mut found = None;
    walk(body, &mut |node, _| {
        if found.is_some() {
            return false;
        }
        if let Node::StaticMethodCall(call) = node
            && matches!(call.class, Expression::Parent(_))
            && matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(method))
        {
            found = Some(call);
            return false;
        }
        true
    });
    found
}

/// `isParamUsedInSpreadArg`: the parameter is unpacked into a call (`...$name`), outside nested functions.
fn is_used_in_spread_arg(body: Node<'_, '_>, name: &[u8]) -> bool {
    super::any_scoped(body, &mut |node| {
        matches!(node, Node::PositionalArgument(a) if a.ellipsis.is_some()
            && matches!(unparenthesize(a.value), Expression::Variable(Variable::Direct(v)) if v.name == name))
    })
}

/// `resolveParentMethodParam`: the type, as source, of the parameter at `position` of `method` in the nearest parent
/// class that has it, read from the source of the class that declares it.
fn parent_param_hint(facts: &Facts, class: &str, method: &[u8], position: usize) -> Option<String> {
    let codebase = &facts.codebase;
    let own = codebase.get_class_like(class.as_bytes())?;
    // `getParents()`: the parent classes, nearest first
    let mut next = own.direct_parent_class.clone();
    while let Some(name) = next.take() {
        let Some(parent) = codebase.get_class_like(name.as_bytes()) else {
            break;
        };
        next = parent.direct_parent_class.clone();
        let lower = method.to_ascii_lowercase();
        let Some(declaring) = parent
            .declaring_method_ids
            .iter()
            .find(|(m, _)| m.as_bytes() == lower.as_slice())
            .map(|(_, id)| id.get_class_name())
        else {
            continue;
        };
        let Some(declaring) = codebase.get_class_like(declaring.as_bytes()) else {
            continue;
        };
        let Some(source) = facts.source_of(declaring.span.file_id) else {
            continue;
        };
        // the declaring class's source has the method (a class-like the codebase says declares it)
        return param_hint_in(
            &source,
            &String::from_utf8_lossy(declaring.original_name.as_bytes()),
            method,
            position,
        )
        .flatten();
    }
    None
}

/// In `source`, the class-like named `class`'s method `method`: `Some(hint)` of its parameter at `position`
/// (`Some(None)` for none or an untyped one), `None` when the method is not there.
fn param_hint_in(
    source: &str,
    class: &str,
    method: &[u8],
    position: usize,
) -> Option<Option<String>> {
    let arena = mago_allocator::LocalArena::new();
    let file = mago_database::file::File::ephemeral(
        std::borrow::Cow::Borrowed(b"parent.php"),
        std::borrow::Cow::Owned(source.as_bytes().to_vec()),
    );
    let program = mago_syntax::parser::parse_file(&arena, &file);
    let mut result = None;
    walk(Node::Program(program), &mut |node, path| {
        if result.is_some() {
            return false;
        }
        // the declaring class-like is a class or a trait
        let (name, members) = match node {
            Node::Class(c) => (c.name.value, &c.members),
            Node::Trait(t) => (t.name.value, &t.members),
            _ => return true,
        };
        if !qualified(path, name).eq_ignore_ascii_case(class) {
            return false;
        }
        let found = members.iter().find_map(|m| match m {
            ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(method) => Some(m),
            _ => None,
        });
        if let Some(m) = found {
            result = Some(hint_of(source, path, m, position));
        }
        false
    });
    result
}

/// The parameter at `position` of `method`, its type printed as php-parser prints a copied type node.
fn hint_of(
    source: &str,
    path: &[Node<'_, '_>],
    method: &Method<'_>,
    position: usize,
) -> Option<String> {
    let param = method.parameter_list.parameters.get(position)?;
    print_hint(source, path, param.hint.as_ref()?)
}

/// A type as php-parser prints it: names fully qualified, `?`, `|`, `&` and parentheses without spaces.
fn print_hint(source: &str, path: &[Node<'_, '_>], hint: &Hint<'_>) -> Option<String> {
    Some(match hint {
        Hint::Identifier(id) => format!(
            "\\{}",
            resolve_written_class_name(path, &String::from_utf8_lossy(id.value()))
        ),
        Hint::Nullable(n) => format!("?{}", print_hint(source, path, n.hint)?),
        Hint::Parenthesized(p) => format!("({})", print_hint(source, path, p.hint)?),
        Hint::Union(u) => format!(
            "{}|{}",
            print_hint(source, path, u.left)?,
            print_hint(source, path, u.right)?
        ),
        Hint::Intersection(i) => format!(
            "{}&{}",
            print_hint(source, path, i.left)?,
            print_hint(source, path, i.right)?
        ),
        other => {
            let span = node_span(other);
            source.get(span.start..span.end)?.to_string()
        }
    })
}
