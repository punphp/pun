//! `lint/laravel/prefer-class-container-key`, ported from rector-laravel 2.5.0's `ReplaceServiceContainerCallArgRector`
//! (src/Rector/MethodCall).
//!
//! The first argument of `make()` or `get()` on an `Illuminate\Contracts\Container\Container`, of a static `make()` or
//! `get()` on `Illuminate\Support\Facades\Application`, or of the `app()` or `resolve()` helper, is replaced when it
//! is a configured old service: a string (any literal php-parser reads as a `String_`, compared exactly) or a class
//! constant (`Foo::class` or `Foo::NAME`, compared by the class's resolved name alone). The replacements apply in
//! their order, each to the argument the one before left. The configuration is `items`, a list of tables with
//! `old-service` and `new-service`, each a string or a class constant as `pun migrate` writes php-parser's
//! `ClassConstFetch` (`{ class = { name = "Foo" }, name = { name = "class" } }`). Without it the rule does nothing.

use mago_syntax::cst::{
    Access, CompositeString, Expression, FunctionCall, Identifier, Literal, Node, NodeKind,
};
use pun_problem::{Edit, Fix, Safety};
use serde_json::Value;

use super::names::resolve_class_name;
use super::object_type::is_object_type;
use super::prefer_validation_rule_array::document_value;
use super::{is_name, member_name};
use crate::rules::rector_code_quality::named;
use crate::rules::types::facts::facts;
use crate::rules::types::namespace_of;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// A service: a string, or a class constant (the class's name as php-parser's `Name::toString()`, and the constant).
#[derive(Clone, Debug, PartialEq)]
pub enum ContainerService {
    Name(String),
    Constant(String, String),
}

#[derive(Default)]
pub struct PreferClassContainerKey {
    pub items: Vec<(ContainerService, ContainerService)>,
}

impl Rule for PreferClassContainerKey {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-class-container-key"
    }

    /// `configure`: `items`, the `ReplaceServiceContainerCallArg`s.
    fn configure(&mut self, options: &Value) -> Result<(), String> {
        let map = match options {
            Value::Null => return Ok(()),
            Value::Object(map) => map,
            _ => return Err("options must be a table".into()),
        };
        if let Some(key) = map.keys().find(|k| *k != "items") {
            return Err(format!("unknown option `{key}`"));
        }
        let error = "`items` must be a list of tables with `old-service` and `new-service`";
        let items = map.get("items").and_then(Value::as_array).ok_or(error)?;
        let mut parsed = Vec::new();
        for item in items {
            let item = item.as_object().ok_or(error)?;
            if item
                .keys()
                .any(|k| k != "old-service" && k != "new-service")
            {
                return Err(error.into());
            }
            let service = |key: &str| item.get(key).and_then(service).ok_or(error);
            parsed.push((service("old-service")?, service("new-service")?));
        }
        self.items = parsed;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::MethodCall,
            NodeKind::StaticMethodCall,
            NodeKind::FunctionCall,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (valid, list) = match node {
            Node::MethodCall(c) => (
                is_container_method(&c.method)
                    && is_object_type(
                        ctx,
                        ancestors,
                        c.object,
                        "Illuminate\\Contracts\\Container\\Container",
                    ),
                &c.argument_list,
            ),
            Node::StaticMethodCall(c) => (
                is_container_method(&c.method)
                    && is_object_type(
                        ctx,
                        ancestors,
                        c.class,
                        "Illuminate\\Support\\Facades\\Application",
                    ),
                &c.argument_list,
            ),
            _ => {
                let Node::FunctionCall(f) = node else { return };
                (is_helper(ctx, ancestors, f), &f.argument_list)
            }
        };
        // `$node->args[0] instanceof Arg`: unpacked and named arguments count too
        let Some(first) = list.arguments.first().filter(|_| valid) else {
            return;
        };
        let value = first.value();
        let Some(mut current) = written(ctx, ancestors, value) else {
            return;
        };
        let mut changed = false;
        for (old, new) in &self.items {
            let matches = match (&current, old) {
                (ContainerService::Constant(a, _), ContainerService::Constant(b, _)) => a == b,
                (ContainerService::Name(a), ContainerService::Name(b)) => a == b,
                _ => false,
            };
            if matches {
                current = new.clone();
                changed = true;
            }
        }
        if !changed {
            return;
        }
        let code = match &current {
            ContainerService::Name(s) => {
                format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
            }
            ContainerService::Constant(class, constant) => format!("\\{class}::{constant}"),
        };
        out.push(Report {
            span: node_span(value),
            message: "Use the configured service container key.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![Edit::new(node_span(unparenthesize(value)), code)],
            }),
        });
    }
}

/// A configured service as `pun migrate` writes it.
fn service(v: &Value) -> Option<ContainerService> {
    if let Some(s) = v.as_str() {
        return Some(ContainerService::Name(s.to_string()));
    }
    let o = v.as_object()?;
    if o.keys()
        .any(|k| !matches!(k.as_str(), "class" | "name" | "attributes"))
    {
        return None;
    }
    let name = |key: &str| o.get(key)?.get("name")?.as_str().map(str::to_string);
    Some(ContainerService::Constant(name("class")?, name("name")?))
}

/// The method is `make` or `get`, in any case.
fn is_container_method(method: &mago_syntax::cst::ClassLikeMemberSelector<'_>) -> bool {
    member_name(method).is_some_and(|n| is_name(n, "make") || is_name(n, "get"))
}

/// `isNames($node->name, ['app', 'resolve'])`: the global helper (a namespace's own function of that name is not it).
fn is_helper(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], f: &FunctionCall<'_>) -> bool {
    ["app", "resolve"].iter().any(|name| {
        named(f, name)
            && !(matches!(f.function, Expression::Identifier(Identifier::Local(_)))
                && namespace_of(ancestors).is_some_and(|ns| {
                    facts(ctx)
                        .codebase
                        .function_exists(format!("{ns}\\{name}").as_bytes())
                }))
    })
}

/// The argument as a service, when it is a string php-parser reads as a `String_` or a class constant whose class is
/// written as a name.
fn written(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    value: &Expression<'_>,
) -> Option<ContainerService> {
    let bytes = match unparenthesize(value) {
        Expression::Literal(Literal::String(s)) => s.value?.to_vec(),
        Expression::CompositeString(CompositeString::Document(d)) => document_value(ctx, d)?,
        Expression::Access(Access::ClassConstant(c)) => {
            let class = match unparenthesize(c.class) {
                Expression::Identifier(id) => resolve_class_name(ancestors, id),
                e @ (Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) => {
                    let span = node_span(e);
                    ctx.source[span.start..span.end].to_string()
                }
                _ => return None,
            };
            return Some(ContainerService::Constant(
                class,
                // `isMatchForChangeServiceContainerCallArgValue` compares the class alone
                ctx.source[node_span(&c.constant).start..node_span(&c.constant).end].to_string(),
            ));
        }
        _ => return None,
    };
    String::from_utf8(bytes).ok().map(ContainerService::Name)
}
