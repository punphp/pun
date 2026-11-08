//! `lint/laravel/prefer-closure-only-bind`, ported from rector-laravel 2.7.3's `ContainerBindConcreteWithClosureOnlyRector`
//! (src/Rector/MethodCall).
//!
//! On an `Illuminate\Contracts\Container\Container`, `bind()` or `singleton()` with a class string (`abstract`) and a
//! closure (`concrete`), positional or named, drops the class string and declares it as the closure's return type
//! instead (`bind(Foo::class, function () {...})` becomes `bind(function (): \Foo {...})`), which the container
//! reads. `bind()`'s `shared` argument follows the closure by name; any other argument leaves the call alone. The
//! closure's returned type (`ReturnTypeInferer`: the native types of its `return`s, `mixed` when it has none or only
//! bare ones) must possibly be the class. When that answer depends on whether a returned variable holds an object of
//! exactly its class (a `new`), which Mago's types do not say, Pun leaves the call.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{Argument, Closure, Expression, Node, NodeKind, Variable};
use pun_problem::{Edit, Safety, Span};

use super::object_type::{
    Tri, class_string_class, class_super_of, exact_class_super_of, is_object_type, object_class,
};
use super::{get_arg, is_name, member_name};
use crate::rules::types::facts::facts;
use crate::rules::types::returns_scoped;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferClosureOnlyBind;

impl Rule for PreferClosureOnlyBind {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-closure-only-bind"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else { return };
        // `bindIf()` and `singletonIf()` pass the abstract to `bound()`, where a closure fails.
        let Some(method) =
            member_name(&call.method).filter(|n| is_name(n, "bind") || is_name(n, "singleton"))
        else {
            return;
        };
        if !is_object_type(
            ctx,
            ancestors,
            call.object,
            "Illuminate\\Contracts\\Container\\Container",
        ) {
            return;
        }
        let list = &call.argument_list;
        let (Some(first), Some(second)) =
            (get_arg(list, "abstract", 0), get_arg(list, "concrete", 1))
        else {
            return;
        };
        // `singleton()` has no `$shared` parameter.
        let shared = is_name(method, "bind")
            .then(|| get_arg(list, "shared", 2))
            .flatten();
        // A spread or an unknown name would be dropped with the rest.
        if 2 + usize::from(shared.is_some()) != list.arguments.len() {
            return;
        }
        let abstract_expr = unparenthesize(first.value());
        let (false, Expression::Closure(closure)) = (
            matches!(abstract_expr, Expression::Variable(Variable::Direct(_))),
            unparenthesize(second.value()),
        ) else {
            return;
        };
        let facts = facts(ctx);
        let Some(class) =
            (facts.full_type(node_span(abstract_expr))).and_then(|t| class_string_class(ctx, t))
        else {
            return;
        };
        if inferred_super_of(ctx, closure, &class).is_none_or(|t| t == Tri::No) {
            return;
        }
        let hint = format!("\\{class}");
        let (type_span, type_text) = match &closure.return_type_hint {
            Some(r) => (node_span(&r.hint), hint),
            None => {
                let close = closure
                    .use_clause
                    .as_ref()
                    .map_or(closure.parameter_list.right_parenthesis, |u| {
                        u.right_parenthesis
                    });
                let at = close.end.offset as usize;
                (Span::new(at, at), format!(": {hint}"))
            }
        };
        // The closure takes the first place, unnamed; `shared` follows by name.
        let value = node_span(second.value());
        let mut args = format!(
            "{}{type_text}{}",
            &ctx.source[value.start..type_span.start],
            &ctx.source[type_span.end..value.end]
        );
        if let Some(shared) = shared {
            let span = node_span(shared.value());
            let name = if matches!(shared, Argument::Named(_)) {
                ""
            } else {
                "shared: "
            };
            let start = if name.is_empty() {
                node_span(shared).start
            } else {
                span.start
            };
            args.push_str(&format!(", {name}{}", &ctx.source[start..span.end]));
        }
        let all = Span::new(
            node_span(list.arguments.first().unwrap()).start,
            node_span(list.arguments.last().unwrap()).end,
        );
        out.push(Report::fixed(
            node_span(call),
            format!("Declare `{class}` as the closure's return type instead of passing it."),
            Safety::Safe,
            vec![Edit::new(all, args)],
        ));
    }
}

/// `$returnTypeInferer->inferFunctionLike($closure)->isSuperTypeOf(new ObjectType($class))`: yes or maybe when some
/// returned value may be an object of the class (a union answers as its best part). No `return`, or only bare ones,
/// is `mixed`; a `void` beside other types (a bare `return`, or a path that ends without one) is no object, so it
/// changes no answer. `None` when the answer cannot be told (see [`answer_of`]).
fn inferred_super_of(ctx: &Context<'_>, closure: &Closure<'_>, class: &str) -> Option<Tri> {
    let mut best: Option<Tri> = None;
    for value in returns_scoped(&closure.body).iter().filter_map(|r| r.value) {
        let answer = answer_of(ctx, value, class)?;
        best = Some(best.map_or(answer, |b| max(b, answer)));
    }
    Some(best.unwrap_or(Tri::Yes))
}

fn max(a: Tri, b: Tri) -> Tri {
    if a as u8 >= b as u8 { a } else { b }
}

/// How the native type of a returned value answers, branch by branch through a ternary. A `new` of a written class
/// is an exact object (final, see [`exact_class_super_of`]); a returned anonymous class is `object`
/// (`getNativeType`); a call's or a property's declared class is not exact. PHPStan keeps `new`'s exactness through a
/// variable, which Mago's types do not say: `None` when a returned variable's answer depends on it.
fn answer_of(ctx: &Context<'_>, value: &Expression<'_>, class: &str) -> Option<Tri> {
    let value = unparenthesize(value);
    if let Expression::Conditional(c) = value {
        // `a ?: b` gives `a`
        let then = answer_of(ctx, c.then.unwrap_or(c.condition), class)?;
        return Some(max(then, answer_of(ctx, c.r#else, class)?));
    }
    let facts = facts(ctx);
    // a value Mago has no type for is PHPStan's error type, `mixed`
    let types = facts
        .native_type(node_span(value))
        .map_or(&[][..], |t| &t.types[..]);
    let exact = match value {
        Expression::Instantiation(i) => Some(matches!(
            i.class,
            Expression::Identifier(_) | Expression::Self_(_) | Expression::Parent(_)
        )),
        Expression::Variable(_) => None,
        _ => Some(false),
    };
    let mut best = if types.is_empty() { Tri::Yes } else { Tri::No };
    for atomic in types {
        let answer = match atomic {
            TAtomic::Mixed(_) | TAtomic::Object(TObject::Any) => Tri::Yes,
            TAtomic::Callable(c) if c.get_signature().is_some_and(|s| s.is_closure) => {
                class_super_of(ctx, "Closure", class)
            }
            TAtomic::Callable(_) | TAtomic::GenericParameter(_) => Tri::Maybe,
            a => match object_class(a) {
                None => Tri::No,
                Some(own) if own.starts_with("{anonymous-class") => Tri::Yes,
                Some(own) => {
                    let answer = exact_class_super_of(ctx, &own, class, exact == Some(true));
                    if exact.is_none() && answer != exact_class_super_of(ctx, &own, class, true) {
                        return None;
                    }
                    answer
                }
            },
        };
        best = max(best, answer);
    }
    Some(best)
}
