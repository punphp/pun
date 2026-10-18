//! `lint/laravel/prefer-exception-class-string`, ported from rector-laravel 2.5.0's
//! `ThrowIfAndThrowUnlessExceptionsToUseClassStringRector` (src/Rector/FuncCall).
//!
//! `throw_if($condition, new SomeException($a, $b))` becomes `throw_if($condition, SomeException::class, $a, $b)`, and
//! the same for `throw_unless()`. A first argument passed by name is left alone (Rector writes a positional argument
//! after it, which PHP rejects).

use mago_syntax::cst::{Argument, Expression, Node, NodeKind};

use super::{argument_texts, replace};
use crate::rules::rector_code_quality::{named, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferExceptionClassString;

impl Rule for PreferExceptionClassString {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-exception-class-string"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(f) = node else { return };
        if !named(f, "throw_if") && !named(f, "throw_unless") {
            return;
        }
        let args = &f.argument_list.arguments;
        if args.len() != 2 || matches!(args.get(0), Some(Argument::Named(_))) {
            return;
        }
        let Some(exception) = args.get(1) else { return };
        let Expression::Instantiation(new) = unparenthesize(exception.value()) else {
            return;
        };
        if !matches!(
            new.class,
            Expression::Identifier(_)
                | Expression::Self_(_)
                | Expression::Static(_)
                | Expression::Parent(_)
        ) {
            return;
        }
        let mut code = format!("{}::class", text(ctx, new.class));
        if let Some(list) = &new.argument_list {
            for arg in argument_texts(ctx, list) {
                code.push_str(", ");
                code.push_str(arg);
            }
        }
        out.push(replace(
            node_span(exception),
            "Pass the exception's class and arguments instead of an instance.".to_string(),
            code,
        ));
    }
}
