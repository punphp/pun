//! What `prefer-serialize-method` and `prefer-unserialize-method` share: php-parser's `Class_::getMethod()` and the
//! edits Rector's printer makes to a method's name, return type and parameters.

use mago_syntax::cst::{ClassLikeMember, Method, Node};
use pun_problem::{Edit, Span};

use super::lint_b_helpers::append_item;
use crate::syntax::node_span;

/// `Class_::getMethod($name)` on the visited class or anonymous class (both are `Class_`): the first method so named,
/// ignoring case.
pub(super) fn class_method<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a Method<'a>> {
    let mut found = None;
    node.visit_children(|child| {
        if let Node::ClassLikeMember(ClassLikeMember::Method(m)) = child
            && found.is_none()
            && m.name.value.eq_ignore_ascii_case(name.as_bytes())
        {
            found = Some(m);
        }
    });
    found
}

/// The method renamed to `name`, as an edit of its name.
pub(super) fn rename(method: &Method<'_>, name: &str) -> Edit {
    Edit::new(node_span(&method.name), name)
}

/// `$classMethod->returnType = new Identifier($type)`: the old type's code goes, or `: type` follows the `)`.
pub(super) fn set_return_type(method: &Method<'_>, r#type: &str) -> Edit {
    match &method.return_type_hint {
        Some(hint) => Edit::new(node_span(&hint.hint), r#type),
        None => {
            let at = method.parameter_list.right_parenthesis.end.offset as usize;
            Edit::new(Span::new(at, at), format!(": {type}"))
        }
    }
}

/// `$classMethod->params[] = $param`: the parameter `code` after the last one.
pub(super) fn append_parameter(source: &str, method: &Method<'_>, code: &str) -> Edit {
    let list = &method.parameter_list;
    let spans: Vec<Span> = list.parameters.iter().map(node_span).collect();
    append_item(
        source,
        &spans,
        list.left_parenthesis.end.offset as usize,
        code,
    )
}
