//! `lint/dead-code/no-unused-expression`, ported from Rector 2.8.4's `RemoveDeadStmtRector`
//! (rules/DeadCode/Rector/Expression) and its `LivingCodeManipulator`.
//!
//! An expression statement whose value is thrown away goes, or keeps only the parts of it that may do something (the
//! calls inside a concatenation, an array key, a dynamic name). Without types Pun is careful where Rector reflects:
//! - a property fetch statement goes only for `$this->p`, `self::$p` or `static::$p` whose class declares `p` (Rector
//!   keeps any fetch whose property it cannot find, as `__get` may run);
//! - `$v[...]` and `clone $v` are kept whole when `$v` may be an object (`$this`, a parameter with a class type, or a
//!   variable the function assigns a `new` to), where Rector asks whether the type is `ArrayAccess` or declares
//!   `__clone`;
//! - a `(void)` cast is kept (Rector keeps it for a `#[NoDiscard]` call).
//!
//! Rector writes the kept parts as statements in the order last-but-first, then first; Pun keeps their order, which
//! keeps the order they run in.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_batch5::doc_comment;
use super::rector_stmts::{
    Frame, HOLDERS, block_list, body_list, holder_list, line_indent, remove,
};
use super::side_effect::any_node;
use crate::syntax::is_this;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUnusedExpression;

impl Rule for NoUnusedExpression {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-expression"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ExpressionStatement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (Node::ExpressionStatement(statement), [.., grand, Node::Statement(stmt)]) =
            (node, ancestors)
        else {
            return;
        };
        let source = ctx.source;
        let expr = statement.expression;
        if is_property_fetch(expr) && !declared_property(ancestors, expr) {
            return;
        }
        if matches!(expr, Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::VoidCast(..)))
        {
            return;
        }
        let mut living = Vec::new();
        keep_living(ancestors, expr, &mut living);
        // php-parser has no node for parentheses, so `(foo());` is the call itself.
        if living.len() == 1 && std::ptr::eq(living[0], unparenthesize(expr)) {
            return;
        }
        // The statement's place: an item of a statement list, or the one statement of a control structure's body.
        let (list, key) = if HOLDERS.contains(&grand.kind()) {
            // Rector removes the statement wherever it is, in a property hook's body too, where `holder_list` has none.
            let list = match grand {
                Node::Block(block) => block_list(source, block),
                _ => holder_list(source, *grand, &ancestors[..ancestors.len() - 2]).unwrap(),
            };
            let key = list
                .stmts
                .iter()
                .position(|s| std::ptr::eq(*s, *stmt))
                .unwrap();
            (list, key)
        } else {
            if living.len() > 1 {
                return;
            }
            (body_list(source, stmt), 0)
        };
        let edits = if living.is_empty() {
            let before = list.before(ctx, key);
            match doc_comment(source, before, node_span(*stmt).start) {
                // php-parser's Nop keeps the doc comment, and prints an empty line for itself.
                Some(doc) if list.frame != Frame::Bare => {
                    let span = list.with_comments(ctx, key);
                    vec![Edit::new(
                        pun_problem::Span::new(span.start, node_span(*stmt).end),
                        format!("{}\n", &source[doc.start..doc.end]),
                    )]
                }
                _ => remove(ctx, &list, &[key]),
            }
        } else {
            let indent = line_indent(source, node_span(*stmt).start);
            let text = living
                .iter()
                .map(|e| {
                    let s = node_span(*e);
                    &source[s.start..s.end]
                })
                .collect::<Vec<_>>()
                .join(&format!(";\n{indent}"));
            vec![Edit::new(node_span(expr), text)]
        };
        out.push(Report {
            span: node_span(*stmt),
            message: if living.is_empty() {
                "This statement does nothing; remove it.".to_string()
            } else {
                "Only part of this statement does something; keep that part.".to_string()
            },
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `PropertyFetchAnalyzer::isPropertyFetch`.
fn is_property_fetch(expr: &Expression<'_>) -> bool {
    matches!(
        unparenthesize(expr),
        Expression::Access(
            Access::Property(_) | Access::NullSafeProperty(_) | Access::StaticProperty(_)
        )
    )
}

/// Whether the fetch reads a property the enclosing class declares: Pun's stand-in for Rector's reflection.
fn declared_property(scope: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    let name: &[u8] = match unparenthesize(expr) {
        Expression::Access(Access::Property(p)) if is_this(p.object) => match &p.property {
            ClassLikeMemberSelector::Identifier(id) => id.value,
            _ => return false,
        },
        Expression::Access(Access::StaticProperty(p))
            if matches!(p.class, Expression::Self_(_) | Expression::Static(_)) =>
        {
            match &p.property {
                Variable::Direct(v) => &v.name[1..],
                _ => return false,
            }
        }
        _ => return false,
    };
    let Some(members) = scope.iter().rev().find_map(|n| match n {
        // A trait's methods run without a class in Rector's scope, so it keeps their fetches.
        Node::Class(c) => Some(&c.members),
        Node::Enum(e) => Some(&e.members),
        Node::AnonymousClass(c) => Some(&c.members),
        _ => None,
    }) else {
        return false;
    };
    members.iter().any(|member| match member {
        ClassLikeMember::Property(Property::Plain(p)) => {
            p.items.iter().any(|i| &i.variable().name[1..] == name)
        }
        ClassLikeMember::Property(Property::Hooked(p)) => &p.item.variable().name[1..] == name,
        ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => m
            .parameter_list
            .parameters
            .iter()
            .any(|p| p.is_promoted_property() && &p.variable.name[1..] == name),
        _ => false,
    })
}

/// `LivingCodeManipulator::keepLivingCodeFromExpr`: the parts of `expr` that may do something, in source order.
fn keep_living<'a>(
    scope: &[Node<'_, '_>],
    expr: &'a Expression<'a>,
    out: &mut Vec<&'a Expression<'a>>,
) {
    match expr {
        Expression::Parenthesized(p) => keep_living(scope, p.expression, out),
        Expression::Closure(_)
        | Expression::Literal(_)
        | Expression::MagicConstant(_)
        | Expression::ConstantAccess(_)
        | Expression::Identifier(_)
        | Expression::Self_(_)
        | Expression::Static(_)
        | Expression::Parent(_) => {}
        Expression::CompositeString(s) if !matches!(s, CompositeString::ShellExecute(_)) => {}
        Expression::Clone(c) => {
            if may_be_object(scope, c.object) {
                out.push(expr);
            } else {
                keep_living(scope, c.object, out);
            }
        }
        Expression::UnaryPrefix(u)
            if matches!(
                u.operator,
                UnaryPrefixOperator::ArrayCast(..)
                    | UnaryPrefixOperator::BoolCast(..)
                    | UnaryPrefixOperator::BooleanCast(..)
                    | UnaryPrefixOperator::DoubleCast(..)
                    | UnaryPrefixOperator::RealCast(..)
                    | UnaryPrefixOperator::FloatCast(..)
                    | UnaryPrefixOperator::IntCast(..)
                    | UnaryPrefixOperator::IntegerCast(..)
                    | UnaryPrefixOperator::ObjectCast(..)
                    | UnaryPrefixOperator::UnsetCast(..)
                    | UnaryPrefixOperator::StringCast(..)
                    | UnaryPrefixOperator::BinaryCast(..)
                    | UnaryPrefixOperator::BitwiseNot(_)
                    | UnaryPrefixOperator::Not(_)
                    | UnaryPrefixOperator::Plus(_)
                    | UnaryPrefixOperator::Negation(_)
            ) =>
        {
            keep_living(scope, u.operand, out)
        }
        Expression::Construct(Construct::Empty(e)) => keep_living(scope, e.value, out),
        Expression::Construct(Construct::Isset(i)) => {
            for value in i.values.iter() {
                keep_living(scope, value, out);
            }
        }
        Expression::Variable(v) => variable(scope, v, out),
        Expression::Access(Access::Property(p)) => {
            keep_living(scope, p.object, out);
            member(scope, &p.property, out);
        }
        Expression::ArrayAccess(a) => {
            if may_be_object(scope, a.array) {
                out.push(expr);
            } else {
                keep_living(scope, a.array, out);
                keep_living(scope, a.index, out);
            }
        }
        Expression::Access(Access::ClassConstant(c)) => {
            keep_living(scope, c.class, out);
            if let ClassLikeConstantSelector::Expression(s) = &c.constant {
                keep_living(scope, s.expression, out);
            }
        }
        Expression::Access(Access::StaticProperty(p)) => {
            keep_living(scope, p.class, out);
            variable(scope, &p.property, out);
        }
        Expression::Binary(b)
            if !matches!(
                b.operator,
                BinaryOperator::And(_)
                    | BinaryOperator::Or(_)
                    | BinaryOperator::LowAnd(_)
                    | BinaryOperator::LowOr(_)
                    | BinaryOperator::NullCoalesce(_)
            ) =>
        {
            keep_living(scope, b.lhs, out);
            keep_living(scope, b.rhs, out);
        }
        _ => out.push(expr),
    }
}

/// A variable's name: nothing for `$a`, the name's living code for `${expr}` and `$$a`.
fn variable<'a>(scope: &[Node<'_, '_>], v: &'a Variable<'a>, out: &mut Vec<&'a Expression<'a>>) {
    match v {
        Variable::Direct(_) => {}
        Variable::Indirect(i) => keep_living(scope, i.expression, out),
        Variable::Nested(n) => variable(scope, n.variable, out),
    }
}

fn member<'a>(
    scope: &[Node<'_, '_>],
    selector: &'a ClassLikeMemberSelector<'a>,
    out: &mut Vec<&'a Expression<'a>>,
) {
    match selector {
        ClassLikeMemberSelector::Variable(v) => variable(scope, v, out),
        ClassLikeMemberSelector::Expression(s) => keep_living(scope, s.expression, out),
        _ => {}
    }
}

/// Whether `expr` may hold an object: `$this`, a parameter with a class type, or a variable the enclosing function
/// (or file) assigns a `new` to.
fn may_be_object(scope: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    let Expression::Variable(Variable::Direct(v)) = unparenthesize(expr) else {
        return false;
    };
    if v.name == b"$this" {
        return true;
    }
    // The statement's function, or the file (an arrow function holds no statement).
    let body = scope
        .iter()
        .rev()
        .find(|n| {
            matches!(
                n,
                Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::Program(_)
            )
        })
        .unwrap();
    let parameters = match body {
        Node::Function(f) => Some(&f.parameter_list),
        Node::Method(m) => Some(&m.parameter_list),
        Node::Closure(c) => Some(&c.parameter_list),
        _ => None,
    };
    if parameters.is_some_and(|list| {
        list.parameters
            .iter()
            .any(|p| p.variable.name == v.name && p.hint.as_ref().is_some_and(|h| class_hint(h)))
    }) {
        return true;
    }
    any_node(*body, &|n| match n {
        Node::Assignment(a) => {
            matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(l)) if l.name == v.name)
                && matches!(unparenthesize(a.rhs), Expression::Instantiation(_))
        }
        _ => false,
    })
}

/// A type that may name a class: anything but the built-in scalar and array types.
fn class_hint(hint: &Hint<'_>) -> bool {
    match hint {
        Hint::Identifier(_)
        | Hint::Object(_)
        | Hint::Mixed(_)
        | Hint::Iterable(_)
        | Hint::Self_(_)
        | Hint::Static(_)
        | Hint::Parent(_) => true,
        Hint::Nullable(n) => class_hint(n.hint),
        Hint::Union(u) => class_hint(u.left) || class_hint(u.right),
        Hint::Intersection(_) => true,
        Hint::Parenthesized(p) => class_hint(p.hint),
        _ => false,
    }
}
