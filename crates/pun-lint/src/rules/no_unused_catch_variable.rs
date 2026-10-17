//! `lint/php-8.0/no-unused-catch-variable`, ported from Rector 2.6.1's `RemoveUnusedVariableInCatchRector`
//! (rules/Php80/Rector/Catch_), with `StmtsManipulator::isVariableUsedInNextStmt` and `ExprUsedInNodeAnalyzer`.
//!
//! For each `try` in a statement list, a catch variable is unused when neither its catch block, the `finally` block,
//! nor the statements after the `try` in the same list use it. "Use" is Rector's: a variable of that name (ASCII case
//! insensitive, as `NodeNameResolver::isName`), any variable variable, any `include`/`require`, or a `compact()`
//! argument naming it.

use mago_syntax::cst::{
    Argument, ArrayElement, Expression, Identifier, Literal, Node, NodeKind, Variable,
};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::syntax::{node_span, statements, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUnusedCatchVariable;

impl Rule for NoUnusedCatchVariable {
    fn id(&self) -> &'static str {
        "lint/php-8.0/no-unused-catch-variable"
    }

    /// PhpVersionFeature::NON_CAPTURING_CATCH.
    fn min_php_version(&self) -> u32 {
        80000
    }

    /// Rector's `refactor` on every node of `NodeGroup::STMTS_AWARE` (here, every node with statement children),
    /// for each `try` among its statements. Pun visits the `try` itself and finds that list two levels up: under it
    /// is the `try`'s statement, and above it the list's own parent. Only a `try` costs any work.
    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Try])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Try(r#try) = node else { return };
        // A `try` is always a statement, and the statement's parent holds the list.
        let n = ancestors.len();
        let (holder, this) = (ancestors[n - 2], ancestors[n - 1]);
        let parent = n.checked_sub(3).map(|i| ancestors[i]);
        let stmts = statements(holder);
        let key = stmts
            .iter()
            .position(|s| matches!(this, Node::Statement(t) if std::ptr::eq(*s, t)))
            .expect("a try is one of its parent's statements");
        // isVariableUsedInNextStmt: when the list is a try block, the try's catches and finally come after it.
        let mut after_list: Vec<Node> = Vec::new();
        if let (Node::Block(block), Some(Node::Try(outer))) = (holder, parent)
            && std::ptr::eq(block, &outer.block)
        {
            after_list.extend(outer.catch_clauses.iter().map(Node::TryCatchClause));
            after_list.extend(outer.finally_clause.iter().map(|f| Node::Block(&f.block)));
        }
        for catch in r#try.catch_clauses.iter() {
            let Some(var) = &catch.variable else {
                continue;
            };
            let name = var.name;
            let in_catch = uses(Node::Block(&catch.block), None, name)
                || r#try
                    .finally_clause
                    .iter()
                    .any(|f| uses(Node::Block(&f.block), None, name));
            let in_next = stmts[key + 1..]
                .iter()
                .map(|s| Node::Statement(s))
                .chain(after_list.iter().copied())
                .any(|n| uses(n, None, name));
            if in_catch || in_next {
                continue;
            }
            // php-parser's removal map strips the whitespace token left of the variable (`Stmt_Catch->var`).
            let var_span = node_span(var);
            let bytes = ctx.source.as_bytes();
            let mut start = var_span.start;
            while start > 0 && matches!(bytes[start - 1], b' ' | b'\t' | b'\n' | b'\r') {
                start -= 1;
            }
            out.push(Report {
                span: var_span,
                message: format!(
                    "`{}` is caught but never used.",
                    String::from_utf8_lossy(name)
                ),
                help: Some("Since PHP 8.0 a catch needs no variable; remove it.".to_string()),
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: vec![Edit::new(Span::new(start, var_span.end), "")],
                }),
            });
        }
    }
}

/// `ExprUsedInNodeAnalyzer::isUsed` on `node` and everything under it. `name` includes the `$`.
fn uses(node: Node<'_, '_>, parent: Option<Node<'_, '_>>, name: &[u8]) -> bool {
    match node {
        // php-parser names a property declaration with an identifier, not a variable.
        Node::DirectVariable(v) => {
            return !matches!(
                parent,
                Some(Node::PropertyAbstractItem(_) | Node::PropertyConcreteItem(_))
            ) && v.name.eq_ignore_ascii_case(name);
        }
        // A variable variable may be any variable.
        Node::IndirectVariable(_) | Node::NestedVariable(_) => return true,
        Node::IncludeConstruct(_)
        | Node::IncludeOnceConstruct(_)
        | Node::RequireConstruct(_)
        | Node::RequireOnceConstruct(_) => return true,
        // `A::$e` names a property (php-parser: VarLikeIdentifier); `A::$$e` and `A::${e}` hold an expression.
        Node::StaticPropertyAccess(access) => {
            return uses(Node::Expression(access.class), Some(node), name)
                || match &access.property {
                    Variable::Direct(_) => false,
                    Variable::Nested(n) => uses(Node::Variable(n.variable), Some(node), name),
                    Variable::Indirect(i) => uses(Node::Expression(i.expression), Some(node), name),
                };
        }
        Node::FunctionCall(call) if is_compact(call.function) => {
            let arguments = call.argument_list.arguments.iter().map(|a| match a {
                Argument::Positional(a) => a.value,
                Argument::Named(a) => a.value,
            });
            if arguments
                .into_iter()
                .any(|v| names_in_compact(v, &name[1..]))
            {
                return true;
            }
        }
        _ => {}
    }
    let mut found = false;
    node.visit_children(|child| found = found || uses(child, Some(node), name));
    found
}

/// `NodeNameResolver::isName($funcCall, 'compact')`: an unqualified or fully qualified `compact`.
fn is_compact(function: &Expression<'_>) -> bool {
    match function {
        Expression::Identifier(Identifier::Local(id)) => id.value.eq_ignore_ascii_case(b"compact"),
        Expression::Identifier(Identifier::FullyQualified(id)) => {
            id.value.eq_ignore_ascii_case(b"\\compact")
        }
        _ => false,
    }
}

/// `CompactFuncCallAnalyzer::isInArgOrArrayItemNodes` for one argument or array item value.
fn names_in_compact(value: &Expression<'_>, name: &[u8]) -> bool {
    match unparenthesize(value) {
        Expression::Literal(Literal::String(s)) => s.value == Some(name),
        Expression::Array(a) => a.elements.iter().any(|e| element_names(e, name)),
        Expression::LegacyArray(a) => a.elements.iter().any(|e| element_names(e, name)),
        _ => false,
    }
}

fn element_names(element: &ArrayElement<'_>, name: &[u8]) -> bool {
    match element {
        ArrayElement::KeyValue(e) => names_in_compact(e.value, name),
        ArrayElement::Value(e) => names_in_compact(e.value, name),
        ArrayElement::Variadic(e) => names_in_compact(e.value, name),
        ArrayElement::Missing(_) => false,
    }
}
