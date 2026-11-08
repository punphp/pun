//! What the code-quality rules ported from Rector share: php-parser's view of a function call, `ValueResolver`'s
//! literal checks, `BinaryOpManipulator`'s inverse operators, and php-parser's operator precedence, which decides where
//! the printed code needs parentheses.
//!
//! There is no printer: a rule builds the new code from the source text of the nodes it keeps. php-parser keeps no
//! parentheses, so an operand is taken without its own and gets new ones where its precedence asks for them, as
//! php-parser's `pInfixOp` and `pPrefixOp` add them when they print a node.

use mago_span::HasSpan;
use mago_syntax::cst::{
    Argument, BinaryOperator, Call, Construct, Expression, ExpressionStatement, FunctionCall,
    Identifier, Literal, Node, UnaryPrefixOperator, Yield,
};
use pun_problem::{Edit, Span};

use crate::Context;
use crate::syntax::{node_span, same_code, unparenthesize};

/// `NodeNameResolver::isName($funcCall, $name)` for a call of a global function: the bare name, unqualified or with a
/// leading backslash, in any ASCII case. An unqualified name in a namespace falls back to the global function, as PHP
/// does.
pub(super) fn call_to<'a>(expr: &'a Expression<'a>, name: &str) -> Option<&'a FunctionCall<'a>> {
    match unparenthesize(expr) {
        Expression::Call(Call::Function(call)) if named(call, name) => Some(call),
        _ => None,
    }
}

/// Whether `call` calls the global function `name`; see [`call_to`].
pub(super) fn named(call: &FunctionCall<'_>, name: &str) -> bool {
    let called = match call.function {
        Expression::Identifier(Identifier::Local(id)) => id.value,
        Expression::Identifier(Identifier::FullyQualified(id)) => &id.value[1..],
        _ => return false,
    };
    called.eq_ignore_ascii_case(name.as_bytes())
}

/// `FuncCall::getArgs()`: the arguments in order, named ones included. Mago parses `f(...)` as a partial
/// application, so a [`FunctionCall`] is never a first-class callable.
pub(super) fn args<'a>(call: &'a FunctionCall<'a>) -> Vec<&'a Argument<'a>> {
    call.argument_list.arguments.iter().collect()
}

/// `NodeComparator::areNodesEqual`: the same code, ignoring whitespace, comments and the parentheses around either
/// expression (php-parser has no node for them).
pub(super) fn equal(ctx: &Context<'_>, a: &Expression<'_>, b: &Expression<'_>) -> bool {
    same_code(
        ctx.source,
        node_span(unparenthesize(a)),
        node_span(unparenthesize(b)),
    )
}

/// `ConstFetchAnalyzer::isConstantWithLowercasedName`: `true`, `false` or `null` in any case, with or without a
/// leading backslash.
pub(super) fn is_constant(expr: &Expression<'_>, name: &str) -> bool {
    let constant: &[u8] = match unparenthesize(expr) {
        Expression::Literal(Literal::True(_)) => b"true",
        Expression::Literal(Literal::False(_)) => b"false",
        Expression::Literal(Literal::Null(_)) => b"null",
        Expression::ConstantAccess(c) => c.name.value(),
        _ => return false,
    };
    let constant = constant.strip_prefix(b"\\").unwrap_or(constant);
    constant.eq_ignore_ascii_case(name.as_bytes())
}

/// `AssignAndBinaryMap::BINARY_OP_TO_INVERSE_CLASSES`: the operator that negates a comparison. php-parser reads `<>`
/// as `!=`.
pub(super) fn inverse(op: &BinaryOperator<'_>) -> Option<&'static str> {
    Some(match op {
        BinaryOperator::Identical(_) => "!==",
        BinaryOperator::NotIdentical(_) => "===",
        BinaryOperator::Equal(_) => "!=",
        BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => "==",
        BinaryOperator::GreaterThan(_) => "<=",
        BinaryOperator::LessThan(_) => ">=",
        BinaryOperator::GreaterThanOrEqual(_) => "<",
        BinaryOperator::LessThanOrEqual(_) => ">",
        _ => return None,
    })
}

/// php-parser's `precedenceMap` entry for an expression: `Infix(precedence, precedenceLHS, precedenceRHS)` for an
/// operator printed between its operands (`pInfixOp`, and `pPostfixOp` for `instanceof`), `Prefix` for one printed
/// before its operand (`pPrefixOp`: `!`, casts, assignments, `yield`, `print`, `include`, `fn`, `throw`), `None` for
/// an expression that never needs parentheses. A lower number binds tighter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Prec {
    Infix(i32, i32, i32),
    Prefix(i32),
    None,
}

/// The precedence of `expr` as php-parser prints it.
pub(super) fn prec(expr: &Expression<'_>) -> Prec {
    match unparenthesize(expr) {
        Expression::Binary(b) => {
            let (p, l, r) = binary(&b.operator);
            Prec::Infix(p, l, r)
        }
        Expression::UnaryPrefix(u) => unary(&u.operator).map_or(Prec::None, Prec::Prefix),
        Expression::Pipe(_) => Prec::Infix(65, 66, 65),
        Expression::Conditional(_) => Prec::Infix(150, 150, 150),
        Expression::Assignment(_) => Prec::Prefix(160),
        Expression::Yield(Yield::From(_)) => Prec::Prefix(170),
        Expression::Yield(_) => Prec::Prefix(175),
        Expression::Construct(Construct::Print(_)) => Prec::Prefix(180),
        Expression::Construct(
            Construct::Include(_)
            | Construct::IncludeOnce(_)
            | Construct::Require(_)
            | Construct::RequireOnce(_),
        ) => Prec::Prefix(220),
        Expression::ArrowFunction(_) => Prec::Prefix(230),
        Expression::Throw(_) => Prec::Prefix(240),
        _ => Prec::None,
    }
}

/// The precedence of a prefix operator: `!` is looser than a cast; `++`, `--` and `&` are never wrapped.
fn unary(op: &UnaryPrefixOperator<'_>) -> Option<i32> {
    match op {
        UnaryPrefixOperator::Not(_) => Some(30),
        UnaryPrefixOperator::PreIncrement(_)
        | UnaryPrefixOperator::PreDecrement(_)
        | UnaryPrefixOperator::Reference(_) => None,
        _ => Some(10),
    }
}

/// The `precedenceMap` entry of a binary operator: precedence, then the limits for the left and the right operand.
pub(super) fn binary(op: &BinaryOperator<'_>) -> (i32, i32, i32) {
    match op {
        BinaryOperator::Exponentiation(_) => (0, 0, 1),
        BinaryOperator::Instanceof(_) => (20, 20, 20),
        BinaryOperator::Multiplication(_)
        | BinaryOperator::Division(_)
        | BinaryOperator::Modulo(_) => (40, 41, 40),
        BinaryOperator::Addition(_)
        | BinaryOperator::Subtraction(_)
        | BinaryOperator::StringConcat(_) => (50, 51, 50),
        BinaryOperator::LeftShift(_) | BinaryOperator::RightShift(_) => (60, 61, 60),
        BinaryOperator::LessThan(_)
        | BinaryOperator::LessThanOrEqual(_)
        | BinaryOperator::GreaterThan(_)
        | BinaryOperator::GreaterThanOrEqual(_) => (70, 70, 70),
        BinaryOperator::Equal(_)
        | BinaryOperator::NotEqual(_)
        | BinaryOperator::AngledNotEqual(_)
        | BinaryOperator::Identical(_)
        | BinaryOperator::NotIdentical(_)
        | BinaryOperator::Spaceship(_) => (80, 80, 80),
        BinaryOperator::BitwiseAnd(_) => (90, 91, 90),
        BinaryOperator::BitwiseXor(_) => (100, 101, 100),
        BinaryOperator::BitwiseOr(_) => (110, 111, 110),
        BinaryOperator::And(_) => (120, 121, 120),
        BinaryOperator::Or(_) => (130, 131, 130),
        BinaryOperator::NullCoalesce(_) => (140, 140, 141),
        BinaryOperator::LowAnd(_) => (190, 191, 190),
        BinaryOperator::LowXor(_) => (200, 201, 200),
        BinaryOperator::LowOr(_) => (210, 211, 210),
    }
}

/// Whether code of precedence `child` needs parentheses where php-parser prints it with precedence `limit`:
/// `pInfixOp` wraps an operator at or above the limit. `pPrefixOp` wraps a prefix operator at or above `lhs`, the
/// limit of the left operand of the binary operator the code ends up on the left of (none on a right side).
pub(super) fn needs_parens(child: Prec, limit: i32, lhs: Option<i32>) -> bool {
    match child {
        Prec::Infix(p, ..) => p >= limit,
        Prec::Prefix(p) => lhs.is_some_and(|l| p >= l),
        Prec::None => false,
    }
}

/// `code`, of precedence `p`, in parentheses when [`needs_parens`] says so.
pub(super) fn wrap(code: &str, p: Prec, limit: i32, lhs: Option<i32>) -> String {
    if needs_parens(p, limit, lhs) {
        format!("({code})")
    } else {
        code.to_string()
    }
}

/// The source of `expr` without its parentheses, in new ones where [`needs_parens`] says so.
pub(super) fn operand(
    ctx: &Context<'_>,
    expr: &Expression<'_>,
    limit: i32,
    lhs: Option<i32>,
) -> String {
    let inner = unparenthesize(expr);
    wrap(text(ctx, inner), prec(inner), limit, lhs)
}

/// `left op right` printed by `pInfixOp` with the operator's operand limits `(l, r)`.
pub(super) fn infix(
    ctx: &Context<'_>,
    left: &Expression<'_>,
    op: &str,
    right: &Expression<'_>,
    (l, r): (i32, i32),
) -> String {
    format!(
        "{} {op} {}",
        operand(ctx, left, l, Some(l)),
        operand(ctx, right, r, None)
    )
}

/// `BinaryOpManipulator::inverseNode`: a comparison with the inverse operator, the operand of a `!`, or the
/// expression under a new `!`. Returns the code and its precedence; `lhs` is as in [`needs_parens`].
pub(super) fn inverse_node(
    ctx: &Context<'_>,
    expr: &Expression<'_>,
    lhs: Option<i32>,
) -> (String, Prec) {
    match unparenthesize(expr) {
        Expression::Binary(b) if inverse(&b.operator).is_some() => {
            let (p, l, r) = binary(&b.operator);
            let code = infix(ctx, b.lhs, inverse(&b.operator).unwrap(), b.rhs, (l, r));
            (code, Prec::Infix(p, l, r))
        }
        Expression::UnaryPrefix(u) if u.operator.is_not() => {
            let inner = unparenthesize(u.operand);
            (text(ctx, inner).to_string(), prec(inner))
        }
        inner => (
            format!("!{}", operand(ctx, inner, 30, lhs)),
            Prec::Prefix(30),
        ),
    }
}

/// `inverseBooleanAnd` and `inverseBooleanOr`: `!(a && b)` as `!a || !b` and `!(a || b)` as `!a && !b`, where `op`
/// is the new operator and `(l, r)` its operand limits.
pub(super) fn de_morgan(
    ctx: &Context<'_>,
    left: &Expression<'_>,
    right: &Expression<'_>,
    op: &str,
    (l, r): (i32, i32),
) -> String {
    let (left, lp) = inverse_node(ctx, left, Some(l));
    let (right, rp) = inverse_node(ctx, right, None);
    format!(
        "{} {op} {}",
        wrap(&left, lp, l, Some(l)),
        wrap(&right, rp, r, None)
    )
}

/// The source of a node.
pub(crate) fn text<'s>(ctx: &Context<'s>, node: &impl HasSpan) -> &'s str {
    let s = node_span(node);
    &ctx.source[s.start..s.end]
}

/// The parent of the node at `span`: the nearest ancestor that is not the node itself under another name (Mago
/// wraps a node in `Expression`, `Call` and the like).
fn parent<'n, 'ast, 'arena>(
    span: Span,
    ancestors: &'n [Node<'ast, 'arena>],
) -> Option<&'n Node<'ast, 'arena>> {
    ancestors.iter().rev().find(|a| node_span(*a) != span)
}

/// `code`, the new code for the node at `span` with precedence `new`, in parentheses when its parent needs them
/// there. php-parser's format-preserving printer adds them through the parent's fixup (`FIXUP_PREC_LEFT`,
/// `FIXUP_PREC_RIGHT`, `FIXUP_PREC_UNARY`, `FIXUP_DEREF_LHS` and the like).
pub(super) fn in_parent(span: Span, ancestors: &[Node<'_, '_>], code: String, new: Prec) -> String {
    let parent = parent(span, ancestors);
    let at_start = parent.is_some_and(|p| node_span(p).start == span.start);
    let wrap = match parent {
        Some(Node::Binary(b)) => {
            let (_, l, r) = binary(&b.operator);
            if at_start {
                needs_parens(new, l, Some(l))
            } else {
                needs_parens(new, r, None)
            }
        }
        Some(Node::UnaryPrefix(u)) => {
            needs_parens(new, unary(&u.operator).unwrap_or(i32::MAX), None)
        }
        Some(Node::Conditional(c)) => {
            c.then.is_none_or(|t| node_span(t) != span)
                && needs_parens(new, 150, at_start.then_some(150))
        }
        Some(Node::Assignment(_)) => needs_parens(new, 160, None),
        Some(
            Node::ArrayAccess(_)
            | Node::ArrayAppend(_)
            | Node::PropertyAccess(_)
            | Node::NullSafePropertyAccess(_)
            | Node::MethodCall(_)
            | Node::NullSafeMethodCall(_)
            | Node::StaticMethodCall(_)
            | Node::StaticPropertyAccess(_)
            | Node::ClassConstantAccess(_)
            | Node::FunctionCall(_),
        ) => at_start && new != Prec::None,
        _ => false,
    };
    if wrap { format!("({code})") } else { code }
}

/// The edit that puts `parts`, one statement each, in place of the expression statement `statement` (Rector's
/// `refactor()` returning several statements). In a list of statements they follow each other on lines of their own.
/// A single-statement body (`if ($a) stmt;`) cannot hold them, so php-parser prints the whole `if` again with braces;
/// Pun puts the braces around the new statements. A statement that ends in `?>` instead of `;` is left alone there.
pub(super) fn statements_edit(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    statement: &ExpressionStatement<'_>,
    parts: &[String],
) -> Option<Edit> {
    let span = node_span(statement.expression);
    let indent = indent(ctx, span.start);
    if parts.len() == 1 || holds_statements(ancestors[ancestors.len() - 2]) {
        return Some(Edit::new(span, parts.join(&format!(";\n{indent}"))));
    }
    let span = node_span(statement);
    if !text(ctx, statement).ends_with(';') {
        return None;
    }
    let body: String = parts
        .iter()
        .map(|p| format!("{indent}    {p};\n"))
        .collect();
    Some(Edit::new(span, format!("{{\n{body}{indent}}}")))
}

/// `in_parent` for an expression of the file that takes the place of the node at `span`. php-parser prints a kept
/// node as it was, without the parentheses its new place may need; Pun adds them, except where they change nothing:
/// on the right of the same associative operator (`'x' . sprintf($a . $b)` is `'x' . $a . $b`).
pub(super) fn moved(
    ctx: &Context<'_>,
    span: Span,
    ancestors: &[Node<'_, '_>],
    value: &Expression<'_>,
) -> String {
    let value = unparenthesize(value);
    let code = text(ctx, value).to_string();
    if let (Some(Node::Binary(parent)), Expression::Binary(b)) = (parent(span, ancestors), value)
        && std::mem::discriminant(&parent.operator) == std::mem::discriminant(&b.operator)
        && node_span(parent.rhs) == span
        && matches!(
            b.operator,
            BinaryOperator::StringConcat(_)
                | BinaryOperator::And(_)
                | BinaryOperator::Or(_)
                | BinaryOperator::BitwiseAnd(_)
                | BinaryOperator::BitwiseOr(_)
                | BinaryOperator::BitwiseXor(_)
        )
    {
        return code;
    }
    in_parent(span, ancestors, code, prec(value))
}

/// Whether `node` holds a list of statements.
fn holds_statements(node: Node<'_, '_>) -> bool {
    matches!(
        node,
        Node::Program(_)
            | Node::Block(_)
            | Node::NamespaceImplicitBody(_)
            | Node::SwitchExpressionCase(_)
            | Node::SwitchDefaultCase(_)
            | Node::IfColonDelimitedBody(_)
            | Node::IfColonDelimitedBodyElseIfClause(_)
            | Node::IfColonDelimitedBodyElseClause(_)
            | Node::ForeachColonDelimitedBody(_)
            | Node::ForColonDelimitedBody(_)
            | Node::WhileColonDelimitedBody(_)
            | Node::DeclareColonDelimitedBody(_)
    )
}

/// The whitespace that starts the line of `offset`, which Rector's printer repeats before each new statement.
fn indent<'s>(ctx: &Context<'s>, offset: usize) -> &'s str {
    let line = ctx.source[..offset].rfind('\n').map_or(0, |i| i + 1);
    let rest = &ctx.source[line..];
    &rest[..rest.len() - rest.trim_start_matches([' ', '\t']).len()]
}
