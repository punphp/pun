//! What the ports of Mago 1.57.0's linter rules share: mago-linter's rule/utils/misc.rs helpers, its scope stack
//! (scope.rs) read from the ancestors, and the parsing of a rule's options, which keep Mago's (kebab-case) names.

use mago_span::HasSpan;
use mago_syntax::cst::{Access, Expression, Method, MethodBody, Node, Statement, Variable};
use pun_problem::Span;

use crate::syntax::span;

/// `get_class_like_header_span`: from the keyword to the name of a named class-like, the `class` keyword of an
/// anonymous class, and the whole node for anything else.
pub(crate) fn class_like_header_span(node: Node<'_, '_>) -> Span {
    span(match node {
        Node::Class(class) => class.class.span().join(class.name.span()),
        Node::Trait(r#trait) => r#trait.r#trait.span().join(r#trait.name.span()),
        Node::Enum(r#enum) => r#enum.r#enum.span().join(r#enum.name.span()),
        Node::Interface(interface) => interface.interface.span().join(interface.name.span()),
        Node::AnonymousClass(class) => class.class.span(),
        _ => node.span(),
    })
}

/// `is_method_setter_or_getter`: a method whose body only returns `$this->...` (and takes no parameter), or only
/// assigns one parameter to `$this->...` and, maybe, returns `$this`.
pub(crate) fn is_method_setter_or_getter(method: &Method<'_>) -> bool {
    let MethodBody::Concrete(block) = &method.body else {
        return false;
    };
    let statements_len = block.statements.len();
    if statements_len > 2 {
        return false;
    }
    let Some(statement) = block.statements.first() else {
        return false;
    };
    match statement {
        Statement::Return(r#return) if method.parameter_list.parameters.is_empty() => {
            r#return
                .value
                .is_some_and(|value| is_accessing_property_of_this(value))
                && statements_len == 1
        }
        Statement::Expression(statement) if method.parameter_list.parameters.len() == 1 => {
            let Expression::Assignment(assignment) = statement.expression else {
                return false;
            };
            if !is_accessing_property_of_this(assignment.lhs) {
                return false;
            }
            // The block has one or two statements, so it has a last one.
            match block.statements.last() {
                Some(Statement::Return(r#return)) => r#return
                    .value
                    .is_some_and(|value| is_variable_named(value, b"$this")),
                _ => false,
            }
        }
        _ => false,
    }
}

fn is_accessing_property_of_this(expression: &Expression<'_>) -> bool {
    match expression {
        Expression::Access(Access::Property(access)) => is_variable_named(access.object, b"$this"),
        _ => false,
    }
}

fn is_variable_named(expression: &Expression<'_>, name: &[u8]) -> bool {
    matches!(expression, Expression::Variable(Variable::Direct(v)) if v.name == name)
}

/// The innermost function-like (function, method, closure, arrow function) holding the node, Mago's
/// `ScopeStack::get_function_like_scope`.
pub(crate) fn function_like_scope<'ast, 'arena>(
    ancestors: &[Node<'ast, 'arena>],
) -> Option<Node<'ast, 'arena>> {
    ancestors.iter().rev().copied().find(|n| {
        matches!(
            n,
            Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_)
        )
    })
}

/// `ScopeStack::get_namespace`: the name of the namespace holding the node, without a leading `\`, empty outside
/// one (and in an unnamed one).
pub(crate) fn namespace<'arena>(ancestors: &[Node<'_, 'arena>]) -> &'arena [u8] {
    ancestors
        .iter()
        .rev()
        .find_map(|n| match n {
            Node::Namespace(ns) => Some(ns.name.as_ref().map_or(&b""[..], |n| {
                n.value().strip_prefix(b"\\").unwrap_or(n.value())
            })),
            _ => None,
        })
        .unwrap_or(b"")
}

/// Takes a rule's options, a table: `set` takes each key and value, and returns `None` when it does not know the
/// key or the value has the wrong type.
pub(crate) fn configure(
    options: &serde_json::Value,
    mut set: impl FnMut(&str, &serde_json::Value) -> Option<()>,
) -> Result<(), String> {
    match options {
        serde_json::Value::Null => Ok(()),
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if set(key, value).is_none() {
                    return Err(format!(
                        "unknown option `{key}` or a value of the wrong type"
                    ));
                }
            }
            Ok(())
        }
        _ => Err("options must be a table".into()),
    }
}

/// A non-negative integer no larger than `max` (the size of the integer type Mago reads it into).
pub(crate) fn uint(value: &serde_json::Value, max: u64) -> Option<usize> {
    value.as_u64().filter(|&n| n <= max).map(|n| n as usize)
}

/// A boolean.
pub(crate) fn boolean(value: &serde_json::Value) -> Option<bool> {
    value.as_bool()
}
