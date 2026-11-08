//! `lint/types/add-bool-return-type`, ported from Rector 2.8.4's `BoolReturnTypeFromBooleanStrictReturnsRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): a function or method whose every `return` returns a boolean
//! expression (`!`, `empty()`, `isset()`, `instanceof`, a `(bool)` cast, a comparison, `&&`, `||`, `and`, `or`,
//! `xor`) or a call of a built-in function declared to return `bool`, and whose end cannot be reached, returns
//! `bool`. Only `true` and `false` is left to `lint/types/add-bool-return-type-from-const`.

use mago_syntax::cst::{
    BinaryOperator, Call, Construct, Expression, Identifier, Literal, Node, NodeKind,
    UnaryPrefixOperator,
};

use super::facts::{Facts, facts, is_builtin_function};
use super::silent_void::Place;
use super::{
    SCALAR_TYPES, add_return_type, class_scope, has_only_return_with_expr, namespace_of,
    returns_scoped, untyped,
};
use crate::{Context, Report, Rule};

pub struct AddBoolReturn;

impl Rule for AddBoolReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-bool-return-type"
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
        let Some(f) = untyped(node, ancestors, ctx) else {
            return;
        };
        let returns = returns_scoped(f.body);
        let scope = class_scope(ancestors);
        let facts = facts(ctx);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        if !has_only_return_with_expr(&place, f.body, &returns) {
            return;
        }
        let values: Vec<&Expression<'_>> = returns
            .iter()
            .filter_map(|r| r.value)
            .map(crate::syntax::unparenthesize)
            .collect();
        let constant = |e: &&Expression<'_>| {
            matches!(e, Expression::Literal(Literal::True(_) | Literal::False(_)))
        };
        if values.iter().all(constant) {
            return;
        }
        if values
            .iter()
            .all(|e| is_bool_expr(e) || is_native_bool_function_call(&facts, ancestors, e))
        {
            out.push(add_return_type(f.span, f.at, "bool"));
        }
    }
}

/// `ExprAnalyzer::isBoolExpr`.
fn is_bool_expr(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::UnaryPrefix(p) => matches!(
            p.operator,
            UnaryPrefixOperator::Not(_)
                | UnaryPrefixOperator::BoolCast(..)
                | UnaryPrefixOperator::BooleanCast(..)
        ),
        Expression::Construct(Construct::Empty(_) | Construct::Isset(_)) => true,
        Expression::Binary(b) => matches!(
            b.operator,
            BinaryOperator::Instanceof(_)
                | BinaryOperator::Equal(_)
                | BinaryOperator::NotEqual(_)
                | BinaryOperator::AngledNotEqual(_)
                | BinaryOperator::Identical(_)
                | BinaryOperator::NotIdentical(_)
                | BinaryOperator::GreaterThan(_)
                | BinaryOperator::GreaterThanOrEqual(_)
                | BinaryOperator::LessThan(_)
                | BinaryOperator::LessThanOrEqual(_)
                | BinaryOperator::And(_)
                | BinaryOperator::Or(_)
                | BinaryOperator::LowAnd(_)
                | BinaryOperator::LowOr(_)
                | BinaryOperator::LowXor(_)
        ),
        _ => false,
    }
}

/// `isNativeBooleanReturnTypeFuncCall`: a call of a built-in function whose native return type is `bool`.
fn is_native_bool_function_call(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
) -> bool {
    let Expression::Call(Call::Function(call)) = expr else {
        return false;
    };
    let Expression::Identifier(name) = call.function else {
        return false;
    };
    function_name(facts, ancestors, name).is_some_and(|name| {
        is_builtin_function(name.as_bytes())
            && facts
                .codebase
                .get_function(name.as_bytes())
                .and_then(|f| f.return_type_declaration_metadata.as_ref())
                .is_some_and(|t| t.type_union.is_bool())
    })
}

/// The function a call of `name` reaches: the namespaced one when the codebase has it, else the global one.
pub(crate) fn function_name(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    name: &Identifier<'_>,
) -> Option<String> {
    let written = String::from_utf8_lossy(name.value()).into_owned();
    if let Some(fq) = written.strip_prefix('\\') {
        return Some(fq.to_string());
    }
    let Some(ns) = namespace_of(ancestors) else {
        return Some(written);
    };
    let namespaced = format!("{ns}\\{written}");
    if written.contains('\\') || facts.codebase.get_function(namespaced.as_bytes()).is_some() {
        return Some(namespaced);
    }
    Some(written)
}
