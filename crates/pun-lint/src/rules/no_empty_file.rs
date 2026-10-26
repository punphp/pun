//! `lint/complexity/no-empty-file`, ported from Mago 1.57.0's `no-redundant-file`
//! (mago-linter src/rule/redundancy/no_redundant_file.rs).
//!
//! Reports a file with no statement that does anything: only tags, whitespace, `use`, `declare` without a body that
//! does something, and expression statements made of literals, variables and magic constants.

use mago_syntax::cst::{ArrayElement, DeclareBody, Expression, Node, NodeKind, Statement};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoEmptyFile;

impl Rule for NoEmptyFile {
    fn id(&self) -> &'static str {
        "lint/complexity/no-empty-file"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        if program.statements.iter().any(is_statement_useful) {
            return;
        }
        out.push(Report {
            span: node_span(program),
            message: "Redundant file with no executable code or declarations.".into(),
            help: Some("Remove the file to simplify the project.".into()),
            fix: None,
        });
    }
}

fn is_statement_useful(statement: &Statement<'_>) -> bool {
    match statement {
        Statement::Inline(inline) => inline.value.iter().any(|b| !b.is_ascii_whitespace()),
        Statement::Namespace(namespace) => namespace.statements().iter().any(is_statement_useful),
        Statement::Block(block) => block.statements.iter().any(is_statement_useful),
        Statement::Declare(declare) => match &declare.body {
            DeclareBody::Statement(statement) => is_statement_useful(statement),
            DeclareBody::ColonDelimited(body) => body.statements.iter().any(is_statement_useful),
        },
        Statement::Try(r#try) => {
            r#try.block.statements.iter().any(is_statement_useful)
                || r#try
                    .catch_clauses
                    .iter()
                    .any(|catch| catch.block.statements.iter().any(is_statement_useful))
                || r#try
                    .finally_clause
                    .iter()
                    .any(|finally| finally.block.statements.iter().any(is_statement_useful))
        }
        Statement::Expression(statement) => is_expression_useful(statement.expression),
        Statement::Foreach(_)
        | Statement::For(_)
        | Statement::While(_)
        | Statement::DoWhile(_)
        | Statement::Continue(_)
        | Statement::Break(_)
        | Statement::Switch(_)
        | Statement::If(_)
        | Statement::Echo(_)
        | Statement::EchoTag(_)
        | Statement::HaltCompiler(_)
        | Statement::Unset(_)
        | Statement::Class(_)
        | Statement::Interface(_)
        | Statement::Trait(_)
        | Statement::Enum(_)
        | Statement::Constant(_)
        | Statement::Function(_)
        | Statement::Return(_) => true,
        _ => false,
    }
}

fn is_expression_useful(expression: &Expression<'_>) -> bool {
    let element = |element: &ArrayElement<'_>| match element {
        ArrayElement::KeyValue(el) => {
            is_expression_useful(el.key) || is_expression_useful(el.value)
        }
        ArrayElement::Value(el) => is_expression_useful(el.value),
        ArrayElement::Variadic(el) => is_expression_useful(el.value),
        ArrayElement::Missing(_) => false,
    };
    match expression {
        Expression::Binary(binary) => {
            is_expression_useful(binary.lhs) || is_expression_useful(binary.rhs)
        }
        Expression::UnaryPrefix(unary) => is_expression_useful(unary.operand),
        Expression::UnaryPostfix(unary) => is_expression_useful(unary.operand),
        Expression::Parenthesized(p) => is_expression_useful(p.expression),
        Expression::Literal(_) | Expression::MagicConstant(_) | Expression::Variable(_) => false,
        Expression::Array(array) => array.elements.iter().any(element),
        Expression::List(list) => list.elements.iter().any(element),
        Expression::LegacyArray(array) => array.elements.iter().any(element),
        _ => true,
    }
}
