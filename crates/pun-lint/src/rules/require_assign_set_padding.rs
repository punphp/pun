//! `lint/style/require-assign-set-padding`, ported from Rector 2.8.4's `NewlineBeforeNewAssignSetRector`
//! (rules/CodingStyle/Rector/ClassMethod).
//!
//! In the body of a function, method or closure, after two statements that set or call on the same variable, a
//! statement on another variable gets a blank line before it, unless the statement before it starts two or more lines
//! above. A set is an assignment or a method call; `$this->a->b` counts as `this->a` (not the variable `$a`), `$a->b->c` as
//! `a`. Rector skips a file
//! with inline HTML.

use mago_syntax::cst::{
    Access, AssignmentOperator, Call, ClassLikeMemberSelector, Expression, MethodBody, Node,
    NodeKind, Statement, Variable,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::lint_b_helpers::{contains_html, lines_between};
use super::rector_stmts::block_list;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct RequireAssignSetPadding;

impl Rule for RequireAssignSetPadding {
    fn id(&self) -> &'static str {
        "lint/style/require-assign-set-padding"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method, NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let block = match node {
            Node::Function(f) => &f.body,
            Node::Method(m) => match &m.body {
                MethodBody::Concrete(b) => b,
                MethodBody::Abstract(_) => return,
            },
            Node::Closure(c) => &c.body,
            _ => return,
        };
        let list = block_list(ctx.source, block);
        let (mut previous, mut before_previous): (Option<String>, Option<String>) = (None, None);
        let mut edits = Vec::new();
        for (key, stmt) in list.stmts.iter().enumerate() {
            let current = subject(stmt);
            // `isNewVariableThanBefore` and `isPrecededByEmptyLine` (start lines).
            if let (Some(c), Some(p), Some(pp)) = (&current, &previous, &before_previous)
                && p == pp
                && p != c
            {
                let prev = node_span(list.stmts[key - 1]);
                if lines_between(ctx.source, prev.start, node_span(*stmt).start) < 2 {
                    // Rector prints the new empty statement as an empty line.
                    edits.push(Edit::new(Span::new(prev.end, prev.end), "\n"));
                }
            }
            before_previous = previous;
            previous = current;
        }
        let Some(first) = edits.first().map(|e| e.span) else {
            return;
        };
        if contains_html(node, ancestors) {
            return;
        }
        out.push(Report {
            span: first,
            message: "Add a blank line before the statement on a new variable.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `resolveCurrentStmtVariableName`: the name of what an assignment sets or a method call calls on.
fn subject(stmt: &Statement<'_>) -> Option<String> {
    let Statement::Expression(e) = stmt else {
        return None;
    };
    let var = match e.expression {
        Expression::Assignment(a) if matches!(a.operator, AssignmentOperator::Assign(_)) => a.lhs,
        Expression::Call(Call::Method(m)) => m.object,
        _ => return None,
    };
    if is_this(var)
        || matches!(
            var,
            Expression::Call(Call::Method(_) | Call::StaticMethod(_))
        )
    {
        return None;
    }
    // Down a chain of property fetches to what it starts on; on `$this`, the first property.
    let mut node = var;
    if let Expression::Access(Access::Property(_)) = var {
        let mut previous = var;
        while let Expression::Access(Access::Property(p)) = node {
            previous = node;
            node = p.object;
        }
        // `$this->property`, namespaced so it does not collide with a local variable of the same name.
        if is_this(node) {
            return name(previous).map(|n| format!("this->{n}"));
        }
    }
    name(node)
}

fn is_this(expr: &Expression<'_>) -> bool {
    matches!(expr, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
}

/// `NodeNameResolver::getName()` for what can start a set: a variable, a property fetch, a function call.
fn name(expr: &Expression<'_>) -> Option<String> {
    let bytes: &[u8] = match expr {
        Expression::Variable(Variable::Direct(v)) => &v.name[1..],
        Expression::Access(Access::Property(p)) => member(&p.property)?,
        Expression::Access(Access::NullSafeProperty(p)) => member(&p.property)?,
        Expression::Access(Access::StaticProperty(p)) => match &p.property {
            Variable::Direct(v) => &v.name[1..],
            _ => return None,
        },
        Expression::Call(Call::Function(f)) => match f.function {
            Expression::Identifier(id) => id.value().strip_prefix(b"\\").unwrap_or(id.value()),
            _ => return None,
        },
        _ => return None,
    };
    Some(String::from_utf8_lossy(bytes).into_owned())
}

fn member<'a>(selector: &'a ClassLikeMemberSelector<'a>) -> Option<&'a [u8]> {
    match selector {
        ClassLikeMemberSelector::Identifier(id) => Some(id.value),
        _ => None,
    }
}
