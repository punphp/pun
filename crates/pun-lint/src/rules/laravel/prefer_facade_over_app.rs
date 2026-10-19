//! `HelperFuncCallToFacadeClassRector` from rector-laravel 2.5.0.

use mago_syntax::cst::{Access, Call, Expression, Identifier, Node, NodeKind, Variable};

use super::{argument_texts, call, is_name, member_name, replace};
use crate::rules::rector_batch5::imported_function;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferFacadeOverApp;

impl Rule for PreferFacadeOverApp {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-facade-over-app"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(function) = node else {
            return;
        };
        if function.argument_list.arguments.is_empty() {
            return;
        }
        // Upstream asks for the name of the callee, not of the FuncCall: `$app()` matches too.
        let function_name = unparenthesize(function.function);
        let identifier = match function_name {
            Expression::Identifier(id) => Some(id),
            Expression::ConstantAccess(c) => Some(&c.name),
            Expression::Call(Call::Function(f)) => match f.function {
                Expression::Identifier(id) => Some(id),
                _ => None,
            },
            _ => None,
        };
        let name = match (identifier, function_name) {
            (Some(Identifier::Local(id)), _) => imported_function(ancestors, id.value)
                .unwrap_or_else(|| String::from_utf8_lossy(id.value).into_owned()),
            (Some(id), _) => {
                let value = String::from_utf8_lossy(id.value());
                if value.starts_with("namespace\\") {
                    if ancestors
                        .iter()
                        .any(|n| matches!(n, Node::Namespace(ns) if ns.name.is_some()))
                    {
                        return;
                    }
                    value[10..].to_string()
                } else {
                    value.trim_start_matches('\\').to_string()
                }
            }
            (None, Expression::Variable(Variable::Direct(v))) => {
                String::from_utf8_lossy(&v.name[1..]).into_owned()
            }
            (None, Expression::Access(Access::Property(p))) => {
                let Some(name) = member_name(&p.property) else {
                    return;
                };
                String::from_utf8_lossy(name).into_owned()
            }
            (None, Expression::Access(Access::NullSafeProperty(p))) => {
                let Some(name) = member_name(&p.property) else {
                    return;
                };
                String::from_utf8_lossy(name).into_owned()
            }
            (None, Expression::Access(Access::StaticProperty(p))) => {
                let Variable::Direct(v) = &p.property else {
                    return;
                };
                String::from_utf8_lossy(&v.name[1..]).into_owned()
            }
            _ => return,
        };
        if !is_name(name.as_bytes(), "app") && !is_name(name.as_bytes(), "resolve") {
            return;
        }
        out.push(replace(
            node_span(function),
            "Use `App::make()` instead of the container helper.".to_string(),
            call(
                "\\Illuminate\\Support\\Facades\\App::make",
                &argument_texts(ctx, &function.argument_list),
            ),
        ));
    }
}
