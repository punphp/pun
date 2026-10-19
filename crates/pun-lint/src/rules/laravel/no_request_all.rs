//! `lint/laravel/no-request-all`, ported from Mago 1.57.0's `no-request-all` (mago-linter
//! src/rule/safety/no_request_all.rs).
//!
//! In a function, method or closure, `$request->all()` (the variable named `$request`, in any case), `request()->all()`
//! and `Request::all()` (the `Request` facade, `Illuminate\Http\Request`, or any class written `Request`) are reported,
//! as `mago_syntax`'s `find_method_references_in_block` finds them: not inside a nested closure, arrow function or
//! class, which are checked on their own if at all.

use mago_syntax::cst::*;
use mago_syntax::utils::reference::{MethodReference, find_method_references_in_block};

use crate::rules::lint_b_names::{function_call_matches, resolve_class};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoRequestAll;

impl Rule for NoRequestAll {
    fn id(&self) -> &'static str {
        "lint/laravel/no-request-all"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method, NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let block = match node {
            Node::Function(f) => &f.body,
            Node::Method(Method {
                body: MethodBody::Concrete(block),
                ..
            }) => block,
            Node::Closure(c) => &c.body,
            _ => return,
        };
        let references = find_method_references_in_block(block, &|reference| {
            let ClassLikeMemberSelector::Identifier(method) = reference.get_selector() else {
                return false;
            };
            if !method.value.eq_ignore_ascii_case(b"all") {
                return false;
            }
            match reference {
                MethodReference::MethodCall(call) => match call.object {
                    Expression::Variable(Variable::Direct(variable)) => {
                        variable.name.eq_ignore_ascii_case(b"$request")
                    }
                    Expression::Call(Call::Function(call))
                        if call.argument_list.arguments.is_empty() =>
                    {
                        function_call_matches(ancestors, call, "request")
                    }
                    _ => false,
                },
                MethodReference::StaticMethodCall(call) => {
                    let Expression::Identifier(identifier) = call.class else {
                        return false;
                    };
                    let class = resolve_class(ancestors, identifier);
                    class.eq_ignore_ascii_case(b"Illuminate\\Support\\Facades\\Request")
                        || class.eq_ignore_ascii_case(b"Illuminate\\Http\\Request")
                        || identifier.value().eq_ignore_ascii_case(b"Request")
                }
                _ => false,
            }
        });
        for reference in references {
            out.push(Report {
                span: node_span(&reference),
                message: "Avoid using `$request->all()` or `Request::all()`.".to_string(),
                help: Some(
                    "Use `$request->only([...])` to specify the inputs you need explicitly, ensuring better security and validation."
                        .to_string(),
                ),
                fix: None,
            });
        }
    }
}
