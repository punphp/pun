//! `lint/laravel/prefer-constructor-injection`, ported from rector-laravel 2.7.3's
//! `ArgumentFuncCallToMethodCallRector` (src/Rector/FuncCall): `items = [{ function = "view", class =
//! "Illuminate\Contracts\View\Factory", method-if-args = "make", method-if-no-args = "..." }, { function = "config",
//! class = "Illuminate\Contracts\Config\Repository", array-method = "set", non-array-method = "get" }]` (rector-laravel's
//! `ArgumentFuncCallToMethodCall` and `ArrayFuncCallToMethodCall`, told apart by their keys).
//!
//! In each method of a class that is neither static nor abstract, a call of a configured function (by name, without
//! case, the first matching item taken) is replaced by what the shared injection
//! ([`crate::rules::class_dependency`]) gives, or a call on it, the call left alone where it gives nothing:
//! without arguments the bare expression, or `method-if-no-args()` on it; with arguments `method-if-args(...)`, or for
//! an array item `array-method(...)` when the first argument's native type is an array, else `non-array-method(...)`.
//! Where Rector stops with an error the class is left alone: a call with arguments and no `method-if-args`, and a
//! first-class callable (`view(...)`).

use mago_syntax::cst::{Expression, Method, MethodBody, Modifier, Node, NodeKind};

use crate::rules::class_dependency::{Outcome, call_args, inject, is_first_class};
use crate::rules::refactor::{
    assert_class, assert_function, assert_method, function_name, is_name, items, object, string,
};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// One `ArgumentFuncCallToMethodCall` or `ArrayFuncCallToMethodCall`.
pub enum Injection {
    Argument {
        function: String,
        class: String,
        method_if_args: Option<String>,
        method_if_no_args: Option<String>,
    },
    Array {
        function: String,
        class: String,
        array_method: String,
        non_array_method: String,
    },
}

impl Injection {
    fn function(&self) -> &str {
        match self {
            Injection::Argument { function, .. } | Injection::Array { function, .. } => function,
        }
    }

    fn class(&self) -> &str {
        match self {
            Injection::Argument { class, .. } | Injection::Array { class, .. } => class,
        }
    }
}

#[derive(Default)]
pub struct PreferConstructorInjection {
    pub items: Vec<Injection>,
}

const ARGUMENT: [&str; 4] = ["function", "class", "method-if-args", "method-if-no-args"];
const ARRAY: [&str; 4] = ["function", "class", "array-method", "non-array-method"];

impl Rule for PreferConstructorInjection {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-constructor-injection"
    }

    fn once(&self) -> bool {
        true
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        let mut out = Vec::new();
        for item in items(options)? {
            let array = item.as_object().is_some_and(|m| {
                m.contains_key("array-method") || m.contains_key("non-array-method")
            });
            let injection = if array {
                let map = object(item, &ARRAY, &ARRAY)?;
                Injection::Array {
                    function: string(map, "function")?,
                    class: string(map, "class")?,
                    array_method: string(map, "array-method")?,
                    non_array_method: string(map, "non-array-method")?,
                }
            } else {
                let map = object(item, &ARGUMENT, &ARGUMENT[..2])?;
                let optional = |key: &str| map.get(key).map(|_| string(map, key)).transpose();
                Injection::Argument {
                    function: string(map, "function")?,
                    class: string(map, "class")?,
                    method_if_args: optional("method-if-args")?,
                    method_if_no_args: optional("method-if-no-args")?,
                }
            };
            assert_class(injection.class())?;
            assert_function(injection.function())?;
            if let Injection::Array {
                array_method,
                non_array_method,
                ..
            } = &injection
            {
                assert_method(array_method)?;
                assert_method(non_array_method)?;
            }
            out.push(injection);
        }
        self.items = out;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if self.items.is_empty() {
            return;
        }
        inject(
            node,
            ancestors,
            ctx,
            "Call the injected service instead of the helper function.",
            &visited,
            &|injector, n, full, method, scope| {
                let (function, list) = match n {
                    Node::FunctionCall(c) => (c.function, Some(&c.argument_list)),
                    Node::FunctionPartialApplication(p) if is_first_class(&p.argument_list) => {
                        (p.function, None)
                    }
                    _ => return Outcome::Keep,
                };
                let Expression::Identifier(id) = function else {
                    return Outcome::Keep;
                };
                let name = function_name(ctx, full, id);
                let Some(item) = self.items.iter().find(|i| is_name(&name, i.function())) else {
                    return Outcome::Keep;
                };
                // Rector stops with an error on a first-class callable.
                let Some(list) = list else {
                    return Outcome::Fail;
                };
                let Some(receiver) = injector.provide(method, item.class(), scope) else {
                    return Outcome::Keep;
                };
                let rewrite = |head: String, args| Outcome::Rewrite {
                    span: node_span(&n),
                    head,
                    args,
                };
                let first = list
                    .arguments
                    .iter()
                    .next()
                    .map(mago_syntax::cst::Argument::value);
                match (item, first) {
                    (
                        Injection::Argument {
                            method_if_no_args: Some(m),
                            ..
                        },
                        None,
                    ) => rewrite(format!("{receiver}->{m}"), Some(call_args(list))),
                    (_, None) => rewrite(receiver, None),
                    (
                        Injection::Argument {
                            method_if_args: Some(m),
                            ..
                        },
                        Some(_),
                    ) => rewrite(format!("{receiver}->{m}"), Some(call_args(list))),
                    // `ShouldNotHappenException`
                    (Injection::Argument { .. }, Some(_)) => Outcome::Fail,
                    (
                        Injection::Array {
                            array_method,
                            non_array_method,
                            ..
                        },
                        Some(value),
                    ) => {
                        let method = match is_array(ctx, value) {
                            true => array_method,
                            false => non_array_method,
                        };
                        rewrite(format!("{receiver}->{method}"), Some(call_args(list)))
                    }
                }
            },
            out,
        );
    }
}

/// The methods the rule walks: neither static nor abstract.
fn visited(method: &Method<'_>) -> bool {
    !method
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Static(_)))
        && matches!(method.body, MethodBody::Concrete(_))
}

/// `ArrayTypeAnalyzer::isArrayType`: the native type of `expr` is an array.
fn is_array(ctx: &Context<'_>, expr: &Expression<'_>) -> bool {
    use mago_codex::ttype::atomic::TAtomic;
    facts(ctx)
        .native_type(node_span(unparenthesize(expr)))
        .is_some_and(|t| {
            !t.types.is_empty() && t.types.iter().all(|a| matches!(a, TAtomic::Array(_)))
        })
}
