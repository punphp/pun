//! `lint/refactor/replace-static-with-method`, ported from Rector 2.6.1's `StaticCallToMethodCallRector`
//! (rules/Transform/Rector/StaticCall): `items = [{ static-class = "Nette\Utils\FileSystem", static-method = "write",
//! class-type = "App\SmartFileSystem", method-name = "dumpFile" }]`, `*` for any method (the call's own name).
//!
//! In each method of a class, a static call whose class resolves to `static-class` (as written after the imports,
//! with case) and whose method is `static-method` (with case) becomes a call of `method-name` on an object of
//! `class-type`: in a static method `(new \ClassType())->method(...)`, elsewhere on what the shared injection
//! ([`crate::rules::class_dependency`]) gives, the call left alone where it gives nothing. A first-class callable
//! (`Foo::bar(...)`) stays one.

use mago_syntax::cst::{ClassLikeMemberSelector, Expression, Method, Modifier, Node, NodeKind};

use super::{assert_class, assert_method, items, object, string};
use crate::rules::class_dependency::{Outcome, call_args, inject, is_first_class, placeholder};
use crate::rules::rector_batch5::resolve_class_name;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// One `StaticCallToMethodCall`.
pub struct StaticToMethod {
    pub static_class: String,
    pub static_method: String,
    pub class_type: String,
    pub method_name: String,
}

#[derive(Default)]
pub struct ReplaceStaticWithMethod {
    pub items: Vec<StaticToMethod>,
}

const KEYS: [&str; 4] = ["static-class", "static-method", "class-type", "method-name"];

impl Rule for ReplaceStaticWithMethod {
    fn id(&self) -> &'static str {
        "lint/refactor/replace-static-with-method"
    }

    fn once(&self) -> bool {
        true
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        let mut out = Vec::new();
        for item in items(options)? {
            let map = object(item, &KEYS, &KEYS)?;
            let item = StaticToMethod {
                static_class: string(map, "static-class")?,
                static_method: string(map, "static-method")?,
                class_type: string(map, "class-type")?,
                method_name: string(map, "method-name")?,
            };
            assert_class(&item.static_class)?;
            if item.static_method != "*" {
                assert_method(&item.static_method)?;
            }
            assert_class(&item.class_type)?;
            if item.method_name != "*" {
                assert_method(&item.method_name)?;
            }
            out.push(item);
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
            "Call the method on an injected service instead of the static method.",
            &|_| true,
            &|injector, n, full, method, scope| {
                let (class, selector, args) = match n {
                    Node::StaticMethodCall(c) => (c.class, &c.method, call_args(&c.argument_list)),
                    Node::StaticMethodPartialApplication(p) if is_first_class(&p.argument_list) => {
                        (p.class, &p.method, placeholder())
                    }
                    _ => return Outcome::Keep,
                };
                let (Expression::Identifier(id), ClassLikeMemberSelector::Identifier(name)) = (class, selector)
                else {
                    return Outcome::Keep;
                };
                let resolved = resolve_class_name(full, id);
                let written = String::from_utf8_lossy(name.value);
                let Some(item) = self.items.iter().find(|i| {
                    i.static_class == resolved && (i.static_method == "*" || i.static_method == written)
                }) else {
                    return Outcome::Keep;
                };
                let method_name = match item.method_name.as_str() {
                    "*" => written.into_owned(),
                    other => other.to_string(),
                };
                let receiver = if is_static(method) {
                    format!("(new \\{}())", item.class_type)
                } else {
                    match injector.provide(method, &item.class_type, scope) {
                        Some(receiver) => receiver,
                        None => return Outcome::Keep,
                    }
                };
                Outcome::Rewrite {
                    span: node_span(&n),
                    head: format!("{receiver}->{method_name}"),
                    args: Some(args),
                }
            },
            out,
        );
    }
}

/// `ClassMethod::isStatic()`.
fn is_static(method: &Method<'_>) -> bool {
    method.modifiers.iter().any(|m| matches!(m, Modifier::Static(_)))
}
