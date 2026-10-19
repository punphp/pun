//! `lint/security/no-ffi`, ported from Mago 1.57.0's linter rule `no-ffi` (rule/safety/no_ffi.rs).
//!
//! A static call on, a constant of, a `new` of, or a type hint naming `FFI`, `FFI\CData` or `FFI\CType` (resolved
//! through the file's namespace and imports, ignoring case) is reported on the name.

use mago_syntax::cst::{Expression, Hint, Identifier, Node, NodeKind};

use super::lint_b_names::resolve_class;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoFfi;

const FFI_CLASSES: [&[u8]; 3] = [b"FFI", b"FFI\\Cdata", b"FFI\\Ctype"];

impl Rule for NoFfi {
    fn id(&self) -> &'static str {
        "lint/security/no-ffi"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::StaticMethodCall,
            NodeKind::ClassConstantAccess,
            NodeKind::Instantiation,
            NodeKind::Hint,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let class = match node {
            Node::StaticMethodCall(call) => call.class,
            Node::ClassConstantAccess(access) => access.class,
            Node::Instantiation(new) => new.class,
            Node::Hint(Hint::Identifier(identifier)) => {
                return report(ancestors, identifier, out);
            }
            _ => return,
        };
        if let Expression::Identifier(identifier) = class {
            report(ancestors, identifier, out);
        }
    }
}

fn report(ancestors: &[Node<'_, '_>], identifier: &Identifier<'_>, out: &mut Vec<Report>) {
    let name = resolve_class(ancestors, identifier);
    if !FFI_CLASSES
        .iter()
        .any(|ffi| ffi.eq_ignore_ascii_case(&name))
    {
        return;
    }
    out.push(Report {
        span: node_span(identifier),
        message: format!(
            "Potentially unsafe use of FFI class `{}`.",
            String::from_utf8_lossy(&name)
        ),
        help: Some(
            "If possible, consider using alternative solutions within PHP to avoid relying on FFI"
                .to_string(),
        ),
        fix: None,
    });
}
