//! `lint/types/add-param-phpdoc-from-callable-call`, ported from Rector 2.6.1's
//! `AddParamArrayDocblockBasedOnCallableNativeFuncCallRector` (rules/TypeDeclaration/Rector/ClassMethod): a parameter
//! declared `array` that is passed to `array_walk()`, `array_map()`, `usort()` or `array_filter()` with a closure
//! whose parameter is typed gets `@param Type[]`.

use mago_syntax::cst::{Argument, Call, Expression, FunctionLikeParameterList, Hint, Node, NodeKind};
use pun_phpstan::types::{Type, mixed, union};

use super::docblock::{FunctionLike, class_name, doc_report, is_array_hint, variable_name};
use super::names::resolve_class_name;
use crate::rules::phpdoc_writer::type_mapper::{create_mixed_passed_or_union_type, is_mixed};
use crate::rules::phpdoc_writer::{change_param_type, native_expr_type};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// `NativeFuncCallPositions::ARRAY_AND_CALLBACK_POSITIONS`: the function, its array's and its callback's position.
const POSITIONS: [(&str, usize, usize); 4] = [
    ("array_walk", 0, 1),
    ("array_map", 1, 0),
    ("usort", 0, 1),
    ("array_filter", 0, 1),
];

pub struct AddParamPhpdocFromCallableCall;

impl Rule for AddParamPhpdocFromCallableCall {
    fn id(&self) -> &'static str {
        "lint/types/add-param-phpdoc-from-callable-call"
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
        let Some(f) = FunctionLike::of(node) else {
            return;
        };
        let Some(body) = f.body else {
            return;
        };
        if f.parameters.parameters.is_empty() {
            return;
        }
        let target = f.target(ctx.source);
        let mut info = target.info(ctx.source);
        // `collectVariableNamesWithArrayType()`: the `array` parameters without a `@param` tag
        let names: Vec<String> = f
            .parameters
            .parameters
            .iter()
            .filter(|p| p.hint.as_ref().is_some_and(is_array_hint))
            .map(|p| String::from_utf8_lossy(&p.variable.name[1..]).into_owned())
            .filter(|name| info.param_tag(name).is_none())
            .collect();
        if names.is_empty() {
            return;
        }
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let mut params_with_type: Vec<(String, Vec<Type>)> = Vec::new();
        visit(Node::Block(body), &mut |n| {
            let Node::Call(Call::Function(call)) = n else {
                return;
            };
            let Expression::Identifier(id) = call.function else {
                return;
            };
            let written = String::from_utf8_lossy(id.value());
            let short = written.rsplit('\\').next().unwrap_or(&written).to_ascii_lowercase();
            if written.trim_start_matches('\\').contains('\\') {
                return;
            }
            let Some(&(_, array_at, callback_at)) = POSITIONS.iter().find(|p| p.0 == short) else {
                return;
            };
            let args: Vec<&Argument<'_>> = call.argument_list.arguments.iter().collect();
            if args.len() < 2 {
                return;
            }
            let Some(array) = get_arg(&args, "array", array_at) else {
                return;
            };
            let Some(name) = variable_name(array).filter(|n| names.iter().any(|m| m == n)) else {
                return;
            };
            // type changed, eg: by reassign
            if !native_expr_type(ctx, array).is_some_and(|t| t.is_array().yes()) {
                return;
            }
            let params = match get_arg(&args, "callback", callback_at) {
                Some(Expression::ArrowFunction(a)) => &a.parameter_list,
                Some(Expression::Closure(c)) => &c.parameter_list,
                _ => return,
            };
            let Some(param_type) = callback_param_type(ctx, &scope, params) else {
                return;
            };
            match params_with_type.iter_mut().find(|(n, _)| n == name) {
                Some((_, types)) => {
                    if !types.contains(&param_type) {
                        types.push(param_type);
                    }
                }
                None => params_with_type.push((name.to_string(), vec![param_type])),
            }
        });
        if params_with_type.is_empty() {
            return;
        }
        for (name, types) in params_with_type {
            let t = if types.len() > 1 { union(types) } else { types.into_iter().next().unwrap() };
            let variadic = f
                .parameters
                .parameters
                .iter()
                .find(|p| &p.variable.name[1..] == name.as_bytes())
                .is_some_and(|p| p.ellipsis.is_some());
            change_param_type(&mut info, &Type::array(mixed(), t), &name, variadic);
        }
        out.extend(doc_report(
            f.name,
            "Add the `@param` array type from the callback's parameter type.",
            ctx.source,
            &target,
            &info,
        ));
    }
}

/// The type of the first parameter of a callback with one or two typed parameters of the same type.
fn callback_param_type(
    ctx: &Context<'_>,
    scope: &[Node<'_, '_>],
    params: &FunctionLikeParameterList<'_>,
) -> Option<Type> {
    let params: Vec<_> = params.parameters.iter().collect();
    if params.is_empty() || params.len() > 2 {
        return None;
    }
    let hints: Vec<&Hint<'_>> = params.iter().map(|p| p.hint.as_ref()).collect::<Option<_>>()?;
    if let [first, second] = hints.as_slice() {
        let text = |h: &Hint<'_>| {
            let span = node_span(h);
            ctx.source[span.start..span.end].to_ascii_lowercase()
        };
        if text(first) != text(second) {
            return None;
        }
    }
    let t = hint_type(scope, hints[0])?;
    (!is_mixed(&t)).then_some(t)
}

/// `StaticTypeMapper::mapPhpParserNodePHPStanType()` of a hint.
fn hint_type(scope: &[Node<'_, '_>], hint: &Hint<'_>) -> Option<Type> {
    Some(match hint {
        Hint::Identifier(id) => Type::Object(resolve_class_name(scope, id), vec![]),
        Hint::Integer(_) => Type::Int,
        Hint::String(_) => Type::String,
        Hint::Float(_) => Type::Float,
        Hint::Bool(_) => Type::Bool,
        Hint::Array(_) => Type::array(mixed(), mixed()),
        // (`void`, `never` and `static` are no parameter types)
        Hint::Mixed(_) | Hint::Void(_) | Hint::Never(_) | Hint::Static(_) => Type::Mixed { explicit: true },
        Hint::Null(_) => Type::Null,
        Hint::True(_) => Type::ConstBool(true),
        Hint::False(_) => Type::ConstBool(false),
        Hint::Iterable(_) => Type::Iterable(Box::new(mixed()), Box::new(mixed())),
        Hint::Callable(_) => Type::Callable(None),
        Hint::Parenthesized(p) => hint_type(scope, p.hint)?,
        // `IntersectionTypeNodeMapper`
        Hint::Intersection(i) => Type::Intersection(vec![hint_type(scope, i.left)?, hint_type(scope, i.right)?]),
        Hint::Object(_) => Type::ObjectWithoutClass,
        Hint::Nullable(n) => union(vec![hint_type(scope, n.hint)?, Type::Null]),
        // `UnionTypeNodeMapper`
        Hint::Union(u) => {
            create_mixed_passed_or_union_type(&[hint_type(scope, u.left)?, hint_type(scope, u.right)?], true)
        }
        // `NameNodeMapper`: `self` of a class is a `SelfStaticType`, printed `$this`
        Hint::Self_(_) => Type::This(class_name(scope)?),
        // `ParentStaticType`, printed `static`
        Hint::Parent(_) => Type::Static(class_name(scope)?, vec![]),
    })
}

/// `CallLike::getArg($name, $position)`.
fn get_arg<'a>(args: &[&'a Argument<'a>], name: &str, position: usize) -> Option<&'a Expression<'a>> {
    args.iter().enumerate().find_map(|(i, a)| match a {
        Argument::Named(n) if n.name.value == name.as_bytes() => Some(n.value),
        Argument::Positional(p) if i == position && p.ellipsis.is_none() => Some(p.value),
        _ => None,
    })
}

/// `traverseNodesWithCallable()` that does not go into classes and functions.
fn visit<'ast, 'arena>(node: Node<'ast, 'arena>, f: &mut dyn FnMut(Node<'ast, 'arena>)) {
    node.visit_children(|child| {
        if matches!(child, Node::Class(_) | Node::AnonymousClass(_) | Node::Function(_)) {
            return;
        }
        f(child);
        visit(child, f);
    });
}
