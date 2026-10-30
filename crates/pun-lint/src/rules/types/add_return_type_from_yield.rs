//! `lint/types/add-return-type-from-yield`, ported from Rector 2.6.1's `AddReturnTypeDeclarationFromYieldsRector`
//! (rules/TypeDeclaration/Rector/FunctionLike, with `YieldNodeFinder` and `YieldTypeResolver`): a function or method
//! whose body is only expression statements, at least one of them yielding, returns a `\Generator`. A declared
//! `Iterator`, `Generator`, `Traversable` or `iterable` stays; another declared type is replaced (a class name by
//! itself, fully qualified).

use mago_syntax::cst::{
    Block, Expression, FunctionLikeReturnTypeHint, Hint, Method, MethodBody, Node, NodeKind,
    Statement,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::facts::facts;
use super::names::resolve_class_name;
use super::never::should_skip_class_method;
use super::{SCALAR_TYPES, class_scope, statement_count};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromYield;

impl Rule for AddReturnTypeFromYield {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-yield"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (name, at, body, hint, method): (
            Span,
            usize,
            &Block<'_>,
            Option<&FunctionLikeReturnTypeHint<'_>>,
            Option<&Method<'_>>,
        ) = match node {
            Node::Function(f) => (
                node_span(&f.name),
                node_span(&f.parameter_list).end,
                &f.body,
                f.return_type_hint.as_ref(),
                None,
            ),
            Node::Method(
                m @ Method {
                    body: MethodBody::Concrete(body),
                    ..
                },
            ) => (
                node_span(&m.name),
                node_span(&m.parameter_list).end,
                body,
                m.return_type_hint.as_ref(),
                Some(m),
            ),
            // an abstract method
            _ => return,
        };
        if !has_only_yielding_expressions(ctx.source, body) {
            return;
        }
        let edit = match hint.map(|h| &h.hint) {
            None => Edit::new(Span::new(at, at), ": \\Generator".to_string()),
            Some(Hint::Iterable(_) | Hint::Self_(_) | Hint::Static(_) | Hint::Parent(_)) => return,
            Some(Hint::Identifier(id)) => {
                let class = resolve_class_name(ancestors, id);
                let skipped = ["Iterator", "Generator", "Traversable", "iterable"];
                if skipped.iter().any(|s| s.eq_ignore_ascii_case(&class)) {
                    return;
                }
                let text = format!("\\{class}");
                if text.as_bytes() == id.value() {
                    return;
                }
                Edit::new(node_span(id), text)
            }
            Some(other) => Edit::new(node_span(other), "\\Generator".to_string()),
        };
        if let Some(m) = method
            && should_skip_class_method(&facts(ctx), class_scope(ancestors).as_ref(), m)
        {
            return;
        }
        out.push(Report {
            span: name,
            message: "Add the `\\Generator` return type.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![edit],
            }),
        });
    }
}

/// `YieldNodeFinder::find` is not empty: every statement of the body is an expression statement (php-parser's `Nop`
/// for a trailing comment counts as another statement), and one of them yields outside nested functions and classes.
fn has_only_yielding_expressions(source: &str, body: &Block<'_>) -> bool {
    if statement_count(source, body) != body.statements.len() {
        return false;
    }
    let mut yields = false;
    for statement in body.statements.iter() {
        let Statement::Expression(e) = statement else {
            return false;
        };
        yields |= super::any_scoped(Node::Expression(e.expression), &mut |n| {
            matches!(n, Node::Yield(_))
        }) || matches!(e.expression, Expression::Yield(_));
    }
    yields
}
