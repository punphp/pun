//! `lint/types/add-param-type-arrow-dim-fetch`, ported from Rector 2.8.4's `AddArrowFunctionParamArrayWhereDimFetchRector`
//! (rules/TypeDeclaration/Rector/FuncCall): in a call of `array_walk()`, `array_map()`, `usort()` or
//! `array_filter()` with two arguments, an untyped parameter of the closure or arrow function that its body reads
//! with a key (`$item['key']`) gets the `array` type, unless the body checks it with `is_array()` or `instanceof`,
//! its native type is a string, or the array holds objects.

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{Argument, BinaryOperator, Expression, Node, NodeKind, Variable};
use pun_problem::Safety;

use super::add_param_type_array_function::ARRAY_AND_CALLBACK_POSITIONS;
use super::facts::facts;
use super::param::{ArrayShape, Flow, add_hint, array_shape_of, traverse};
use crate::rules::lint_b_helpers::value;
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PhpVersionFeature::SCALAR_TYPES`.
const SCALAR_TYPES: u32 = 70000;

pub struct AddParamTypeArrowDimFetch;

impl Rule for AddParamTypeArrowDimFetch {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-arrow-dim-fetch"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
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
        let Node::FunctionCall(call) = node else {
            return;
        };
        let args: Vec<&Argument<'_>> = call.argument_list.arguments.iter().collect();
        if args.len() != 2 {
            return;
        }
        let Some(&(_, array_at, callback_at)) = ARRAY_AND_CALLBACK_POSITIONS
            .iter()
            .find(|(name, _, _)| named(call, name))
        else {
            return;
        };
        let (params, scope) = match unparenthesize(value(args[callback_at])) {
            Expression::Closure(c) => (&c.parameter_list, Node::Block(&c.body)),
            Expression::ArrowFunction(a) => (&a.parameter_list, Node::Expression(a.expression)),
            _ => return,
        };
        let facts = facts(ctx);
        // resolveIsArrayVariables, resolveInstanceofVariables and resolveDimFetchVariableNames
        let mut skipped: Vec<Vec<u8>> = Vec::new();
        let mut fetched: Vec<Vec<u8>> = Vec::new();
        let mut look = |n: Node<'_, '_>| {
            match n {
                Node::FunctionCall(f) if named(f, "is_array") => {
                    if let Some(Expression::Variable(Variable::Direct(v))) = f
                        .argument_list
                        .arguments
                        .first()
                        .map(|a| unparenthesize(value(a)))
                    {
                        skipped.push(v.name.to_vec());
                    }
                }
                Node::Binary(b) if matches!(b.operator, BinaryOperator::Instanceof(_)) => {
                    if let Expression::Variable(Variable::Direct(v)) = unparenthesize(b.lhs) {
                        skipped.push(v.name.to_vec());
                    }
                }
                Node::ArrayAccess(a) => {
                    if let Expression::Variable(Variable::Direct(v)) = unparenthesize(a.array) {
                        let native = facts.native_type(node_span(a.array));
                        if !native.is_some_and(|t| {
                            !t.types.is_empty()
                                && t.types
                                    .iter()
                                    .all(|t| matches!(t, TAtomic::Scalar(s) if s.is_any_string()))
                        }) {
                            fetched.push(v.name.to_vec());
                        }
                    }
                }
                _ => {}
            }
            Flow::Continue
        };
        traverse(scope, &mut look);
        let objects = matches!(
            array_shape_of(&facts, ancestors, value(args[array_at])),
            Some(ArrayShape::General(Some(item)))
                if !item.types.is_empty() && item.types.iter().all(|t| matches!(t, TAtomic::Object(_)))
        );
        let edits: Vec<_> = params
            .parameters
            .iter()
            .filter(|p| {
                p.hint.is_none()
                    && !skipped.iter().any(|n| n == p.variable.name)
                    && fetched.iter().any(|n| n == p.variable.name)
                    && !objects
            })
            .map(|p| add_hint(p, "array"))
            .collect();
        if edits.is_empty() {
            return;
        }
        out.push(Report::fixed(
            node_span(call),
            "Add the `array` type to callback parameters read with a key.",
            Safety::Unsafe,
            edits,
        ));
    }
}
