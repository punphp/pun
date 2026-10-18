//! `lint/laravel/prefer-request-server`, ported from rector-laravel 2.5.0's `ServerVariableToRequestFacadeRector`
//! (src/Rector/ArrayDimFetch).
//!
//! Reading `$_SERVER['NAME']` becomes `\Illuminate\Support\Facades\Request::server('NAME')`. Items inside an
//! assignment to a superglobal, `isset()`, `unset()` or an interpolated string are left alone
//! (`ArrayDimFetchContextNodeVisitor::IS_IN_SUPERGLOBAL_ASSIGN`), and so is any other place a call cannot be written to.

use mago_syntax::cst::{Node, NodeKind};

use super::{in_string, in_superglobal_assign, is_variable, is_written, replace};
use crate::rules::rector_code_quality::text;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferRequestServer;

impl Rule for PreferRequestServer {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-request-server"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ArrayAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ArrayAccess(access) = node else {
            return;
        };
        if !is_variable(access.array, "_SERVER") || in_superglobal_assign(ancestors) {
            return;
        }
        let span = node_span(access);
        if is_written(span, ancestors) || in_string(ancestors) {
            return;
        }
        out.push(replace(
            span,
            "Use `Request::server()` instead of reading `$_SERVER`.".to_string(),
            format!(
                "\\Illuminate\\Support\\Facades\\Request::server({})",
                text(ctx, access.index)
            ),
        ));
    }
}
