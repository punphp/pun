//! `lint/types/add-param-type-array-reduce`, ported from Rector 2.6.1's `AddClosureParamTypeForArrayReduceRector`
//! (rules/TypeDeclaration/Rector/FunctionLike): the closure passed to the native `array_reduce()` gets the array's
//! item type on its second (value) parameter, and on its first (carry) parameter the closure's return type, united
//! with the initial value's type when one is passed.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    AssignmentOperator, Closure, Expression, FunctionLikeParameter, Identifier, Node, NodeKind,
};
use pun_problem::Edit;

use super::add_param_type_array_dim_fetch::is_variable;
use super::facts::{Facts, facts};
use super::namespace_of;
use super::param::{
    Flow, add_hint, array_shape_of, doc_order, is_mixed, item_type, param_declaration_unsorted, report,
    traverse,
};
use crate::rules::class_method::type_of;
use crate::rules::lint_b_helpers::value;
use crate::rules::rector_code_quality::{args, named};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddParamTypeArrayReduce;

impl Rule for AddParamTypeArrayReduce {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-array-reduce"
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
        if !named(call, "array_reduce") {
            return;
        }
        let facts = facts(ctx);
        if !is_native(&facts, ancestors, call.function) {
            return;
        }
        let args = args(call);
        let Some(Expression::Closure(closure)) = args.get(1).map(|a| unparenthesize(value(a))) else {
            return;
        };
        let Some(closure_type) = type_of(&facts, value(args[1])) else {
            return;
        };
        let [TAtomic::Callable(TCallable::Signature(signature))] = closure_type.types.as_ref() else {
            return;
        };
        let Some(returned) = signature.return_type.as_deref() else {
            return;
        };
        let mut carry: Option<Vec<TAtomic>> = Some(returned.types.to_vec());
        if returns_untyped_param(closure) {
            carry = None;
        } else if let Some(initial) = args.get(2) {
            carry = type_of(&facts, value(initial)).and_then(|initial| combine(initial, returned));
        }
        let value_type = array_shape_of(&facts, value(args[0])).as_ref().and_then(item_type);
        let params: Vec<&FunctionLikeParameter<'_>> = closure.parameter_list.parameters.iter().collect();
        let mut edits: Vec<Edit> = Vec::new();
        if let (Some(param), Some(t)) = (params.get(1), value_type) {
            // PHPStan keeps a docblock union in the order written, where Mago sorts it
            let order = doc_order(ctx.source, ancestors);
            let position = |a: &TAtomic| {
                let name = param_declaration_unsorted(std::slice::from_ref(a), &facts.codebase, ctx.php_version)
                    .unwrap_or_default();
                let short = name.rsplit('\\').next().unwrap_or(&name).to_ascii_lowercase();
                order.iter().position(|o| *o == short).unwrap_or(order.len())
            };
            let mut types = t.types.to_vec();
            types.sort_by_key(|a| position(a));
            edits.extend(refactor_parameter(&facts, param, &types, ctx.php_version));
        }
        if let (Some(param), Some(t)) = (params.first(), carry) {
            edits.extend(refactor_parameter(&facts, param, &t, ctx.php_version));
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            node_span(call),
            "Add the item and carry types to the array_reduce() closure's parameters.",
            edits,
        ));
    }
}

/// Whether the closure returns one of its untyped parameters, never reassigned: PHPStan's return type is then `mixed`
/// (the parameter is `mixed`, and writing an offset keeps it so), where Mago types the parameter from
/// `array_reduce()`'s template.
fn returns_untyped_param(closure: &Closure<'_>) -> bool {
    let untyped: Vec<&[u8]> = closure
        .parameter_list
        .parameters
        .iter()
        .filter(|p| p.hint.is_none())
        .map(|p| p.variable.name)
        .collect();
    let mut returned: Vec<&[u8]> = Vec::new();
    let mut reassigned: Vec<&[u8]> = Vec::new();
    traverse(Node::Block(&closure.body), &mut |node| {
        match node {
            Node::Return(r) => returned.extend(r.value.and_then(|v| {
                untyped.iter().copied().find(|name| is_variable(v, name))
            })),
            Node::Assignment(a) if matches!(a.operator, AssignmentOperator::Assign(_)) => {
                reassigned.extend(untyped.iter().copied().find(|name| is_variable(a.lhs, name)));
            }
            _ => {}
        }
        Flow::Continue
    });
    returned.iter().any(|name| !reassigned.contains(name))
}

/// `resolveFunctionLikeReflectionFromCall() instanceof NativeFunctionReflection`: `array_reduce` is PHP's own, not a
/// function of the current namespace that shadows it.
fn is_native(facts: &Facts, ancestors: &[Node<'_, '_>], function: &Expression<'_>) -> bool {
    if matches!(function, Expression::Identifier(Identifier::FullyQualified(_))) {
        return true;
    }
    namespace_of(ancestors)
        .is_none_or(|ns| !facts.codebase.function_exists(format!("{ns}\\array_reduce").as_bytes()))
}

/// `refactorParameter()`: the type for an untyped parameter, unless it is `mixed` or has no declaration.
fn refactor_parameter(
    facts: &Facts,
    param: &FunctionLikeParameter<'_>,
    types: &[TAtomic],
    php_version: u32,
) -> Option<Edit> {
    if param.hint.is_some() || is_mixed(&TUnion::from_vec(types.to_vec())) {
        return None;
    }
    let declaration = param_declaration_unsorted(types, &facts.codebase, php_version)?;
    Some(add_hint(param, &declaration))
}

/// `combineTypes([$initial, $returned])`: one type when the two are equal, else the members of both in PHPStan's
/// `UnionTypeHelper::sortTypes` order; `None` when a union holds an intersection.
fn combine(initial: &TUnion, returned: &TUnion) -> Option<Vec<TAtomic>> {
    if initial == returned {
        return Some(initial.types.to_vec());
    }
    let mut types: Vec<TAtomic> = Vec::new();
    for t in [initial, returned] {
        if t.types.len() > 1
            && t.types.iter().any(|a| matches!(a, TAtomic::Object(TObject::Named(n)) if n.intersection_types.is_some()))
        {
            return None;
        }
        types.extend(t.types.iter().cloned());
    }
    types.sort_by(|a, b| sort_key(a).cmp(&sort_key(b)));
    Some(types)
}

/// `UnionTypeHelper::sortTypes`: `null` last, constant booleans then other constant scalars (by value) first, the
/// rest by name.
fn sort_key(a: &TAtomic) -> (u8, i128, String) {
    match a {
        TAtomic::Null => (4, 0, String::new()),
        TAtomic::Scalar(TScalar::Bool(b)) if !b.is_general() => (0, 0, String::new()),
        TAtomic::Scalar(s) if s.is_literal_value() => (1, s.get_literal_int_value().map_or(0, i128::from), String::new()),
        other => (2, 0, mago_codex::ttype::TType::get_id(other).to_string()),
    }
}
