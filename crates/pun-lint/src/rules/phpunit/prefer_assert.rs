//! `lint/phpunit/prefer-assert`, ported from rector-phpunit's `AssertFuncCallToPHPUnitAssertRector`
//! (rules/CodeQuality/Rector/FuncCall).
//!
//! In the methods of a test class or a Behat context, `assert()` becomes the PHPUnit assertion it checks:
//! `assert($a == $b)` `assertEquals($b, $a)`, `===` `assertSame()`, `!== null` `assertNotNull()`, a `(bool)` cast or a
//! `method_exists()` call `assertTrue()`, and `instanceof` of a class name `assertInstanceOf(X::class, ...)` (the name as written); a second
//! argument stays as the message. The call is `$this->...()`, or `\PHPUnit\Framework\Assert::...()` in a static
//! method or a Behat context, and from the first static closure or anonymous class of a method on (Rector's flag is
//! never reset within the method).

use mago_syntax::cst::{
    BinaryOperator, Call, Expression, Method, MethodBody, Modifier, Node, NodeKind,
    UnaryPrefixOperator,
};
use pun_problem::{Edit, Safety};

use super::{class_body, expr_text, is_in_test_class, own_class_reflection};
use crate::rules::laravel::is_a;
use crate::rules::rector_batch5::imported_function;
use crate::rules::rector_code_quality::is_constant;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const BEHAT_CONTEXT: &str = "Behat\\Behat\\Context\\Context";

pub struct PreferAssert;

impl Rule for PreferAssert {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-assert"
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
        let behat = is_behat_context(ctx, &scope);
        if !behat && !is_in_test_class(ctx, &scope, None) {
            return;
        }
        for member in members.iter() {
            let mago_syntax::cst::ClassLikeMember::Method(method) = member else {
                continue;
            };
            let MethodBody::Concrete(block) = &method.body else {
                continue;
            };
            let mut use_static = is_static(method) || behat;
            let mut path = scope.clone();
            path.push(Node::Method(method));
            let mut stack: Vec<(Node, usize)> = Vec::new();
            let mut children = Vec::new();
            Node::Block(block).visit_children(|c| children.push(c));
            let base = path.len();
            stack.extend(children.into_iter().rev().map(|c| (c, base)));
            while let Some((node, depth)) = stack.pop() {
                path.truncate(depth);
                match node {
                    Node::Closure(c) if c.r#static.is_some() => use_static = true,
                    Node::AnonymousClass(_) => use_static = true,
                    Node::FunctionCall(call) => {
                        if let Some(text) = assert_call(ctx, &path, call, use_static) {
                            let span = node_span(call);
                            out.push(Report::fixed(
                                span,
                                "Use a PHPUnit assertion instead of `assert()`.",
                                Safety::Unsafe,
                                vec![Edit::new(span, text)],
                            ));
                        }
                    }
                    _ => {}
                }
                path.push(node);
                let mut children = Vec::new();
                node.visit_children(|c| children.push(c));
                let depth = path.len();
                stack.extend(children.into_iter().rev().map(|c| (c, depth)));
            }
        }
    }
}

fn is_static(method: &Method<'_>) -> bool {
    method
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Static(_)))
}

/// `isBehatContext`: the class's reflection is a `Behat\Behat\Context\Context`.
fn is_behat_context(ctx: &Context<'_>, scope: &[Node<'_, '_>]) -> bool {
    own_class_reflection(ctx, scope).is_some_and(|name| is_a(ctx, &name, &[BEHAT_CONTEXT]))
}

/// `NodeNameResolver::isName` on a function call: the name as written (an imported name resolved), without a leading
/// `\`, against `name`.
fn is_function(
    ancestors: &[Node<'_, '_>],
    call: &mago_syntax::cst::FunctionCall<'_>,
    name: &str,
) -> bool {
    let Expression::Identifier(id) = call.function else {
        return false;
    };
    let written = id.value();
    let resolved = match (written.starts_with(b"\\"), written.contains(&b'\\')) {
        (false, false) => imported_function(ancestors, written)
            .unwrap_or_else(|| String::from_utf8_lossy(written).into_owned()),
        _ => String::from_utf8_lossy(written.strip_prefix(b"\\").unwrap_or(written)).into_owned(),
    };
    resolved.eq_ignore_ascii_case(name)
}

/// The new assertion call for `assert(...)`, as php-parser prints it.
fn assert_call(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    call: &mago_syntax::cst::FunctionCall<'_>,
    use_static: bool,
) -> Option<String> {
    if !is_function(ancestors, call, "assert") {
        return None;
    }
    let args: Vec<_> = call.argument_list.arguments.iter().collect();
    let compared = unparenthesize(args.first()?.value());
    let text = |e: &Expression<'_>| expr_text(ctx, e).to_string();
    let (method, mut exprs) = match compared {
        Expression::Binary(b) => match b.operator {
            BinaryOperator::Equal(_) => ("assertEquals", vec![text(b.rhs), text(b.lhs)]),
            BinaryOperator::Identical(_) => ("assertSame", vec![text(b.rhs), text(b.lhs)]),
            BinaryOperator::NotIdentical(_) if is_constant(b.rhs, "null") => {
                ("assertNotNull", vec![text(b.lhs)])
            }
            BinaryOperator::Instanceof(_) => {
                let class_name @ Expression::Identifier(_) = unparenthesize(b.rhs) else {
                    return None;
                };
                (
                    "assertInstanceOf",
                    vec![format!("{}::class", text(class_name)), text(b.lhs)],
                )
            }
            _ => return None,
        },
        Expression::UnaryPrefix(u)
            if matches!(
                u.operator,
                UnaryPrefixOperator::BoolCast(..) | UnaryPrefixOperator::BooleanCast(..)
            ) =>
        {
            ("assertTrue", vec![text(compared)])
        }
        Expression::Call(Call::Function(f)) if is_function(ancestors, f, "method_exists") => {
            ("assertTrue", vec![text(compared)])
        }
        _ => return None,
    };
    if let Some(message) = args.get(1) {
        exprs.push(text(message.value()));
    }
    let receiver = if use_static {
        "\\PHPUnit\\Framework\\Assert::"
    } else {
        "$this->"
    };
    Some(format!("{receiver}{method}({})", exprs.join(", ")))
}
