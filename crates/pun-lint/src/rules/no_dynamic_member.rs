//! The member rules of phpstan-strict-rules 2.0.12's `VariableVariables` (src/Rules/VariableVariables), one Pun rule
//! each:
//!
//! - `lint/style/no-dynamic-method-call` (`VariableMethodCallRule`, `method.dynamicName`): `$object->$name()`, also
//!   with `?->` (PHPStan analyses a nullsafe call as the call it stands for).
//! - `lint/style/no-dynamic-method-callable` (`VariableMethodCallableRule`, `method.dynamicName`): `$object->$name(...)`.
//! - `lint/style/no-dynamic-property` (`VariablePropertyFetchRule`, `property.dynamicName`): `$object->$name`, also
//!   with `?->`, unless the name's type is a literal string or the object may be a universal object crate
//!   (`stdClass`, PHPStan's `universalObjectCratesClasses` default) or a `SimpleXMLElement`.
//! - `lint/style/no-dynamic-static-call` (`VariableStaticMethodCallRule`, `staticMethod.dynamicName`): `Foo::$name()`.
//! - `lint/style/no-dynamic-static-callable` (`VariableStaticMethodCallableRule`, `staticMethod.dynamicName`):
//!   `Foo::$name(...)`.
//! - `lint/style/no-dynamic-static-property` (`VariableStaticPropertyFetchRule`, `staticProperty.dynamicName`):
//!   `Foo::$$name`, `Foo::${expr}`.
//!
//! A member name is dynamic when it is not an identifier (php-parser's `Identifier`, `VarLikeIdentifier`): a
//! variable or any `{expr}`, `{'literal'}` included. The message names what the member is accessed on: a class name
//! as `Scope::resolveName` resolves it (`self`, `static` and `parent` the enclosing class or its parent), else the
//! type of the expression as `describe(VerbosityLevel::typeOnly())` prints it ([`describe`]), from Mago's analyser.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{ClassLikeMemberSelector, Expression, Node, NodeKind, Variable};

use super::phpstan_describe::describe;
use super::rector_batch5::resolve_class_name;
use super::types::class_scope;
use super::types::facts::facts;
use pun_problem::Span;

use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// The classes PHPStan lets any property be read on: its `universalObjectCratesClasses` default and the rule's own
/// `SimpleXMLElement` check.
const CRATES: &[&str] = &["stdClass", "SimpleXMLElement"];

/// The name of a dynamic member (a variable or an `{expr}`), `None` for an identifier.
fn dynamic(selector: &ClassLikeMemberSelector<'_>) -> Option<Span> {
    match selector {
        ClassLikeMemberSelector::Variable(v) => Some(node_span(v)),
        ClassLikeMemberSelector::Expression(e) => Some(node_span(unparenthesize(e.expression))),
        _ => None,
    }
}

/// `$scope->getType($expr)->describe(VerbosityLevel::typeOnly())`. A nullsafe access is analysed as the plain access
/// on its receiver without `null` (`TypeCombinator::removeNull`), so `nullsafe` drops it. `None` where the analyser
/// did not reach the expression: unreachable code, which PHPStan does not analyse either.
fn type_of(ctx: &Context<'_>, expr: &Expression<'_>, nullsafe: bool) -> Option<String> {
    let mut t = facts(ctx)
        .full_type(node_span(unparenthesize(expr)))
        .cloned()?;
    if nullsafe && t.types.len() > 1 {
        t.types = t.types.iter().filter(|a| !a.is_null()).cloned().collect();
    }
    let described = describe(&t);
    // An anonymous class is `class@anonymous/<path>:<line>` in PHPStan; Mago names it
    // `{anonymous-class:<path>:<line>:<column>}`.
    match described
        .strip_prefix("{anonymous-class:")
        .and_then(|rest| rest.strip_suffix('}'))
        .and_then(|rest| rest.rsplit(':').nth(1))
    {
        Some(line) => Some(format!(
            "class@anonymous/{}:{line}",
            ctx.path.display().to_string().trim_start_matches("./")
        )),
        None => Some(described),
    }
}

/// What a static member is accessed on: `Scope::resolveName` for a name, the expression's type otherwise. `None` in
/// code the analyser did not reach (it typed no member `name`), which PHPStan does not analyse.
fn class_of(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    name: Span,
    class: &Expression<'_>,
) -> Option<String> {
    facts(ctx).full_type(name)?;
    let class = unparenthesize(class);
    let written = || ctx.source[node_span(class).start..node_span(class).end].to_string();
    let scope = class_scope(ancestors);
    Some(match class {
        // Outside a class (or `parent` in one without a parent) the keyword stays as written.
        Expression::Self_(_) | Expression::Static(_) => {
            scope.and_then(|c| c.name).unwrap_or_else(written)
        }
        Expression::Parent(_) => scope.and_then(|c| c.parent_class).unwrap_or_else(written),
        Expression::Identifier(id) => resolve_class_name(ancestors, id),
        other => return type_of(ctx, other, false),
    })
}

fn report(node: Node<'_, '_>, message: String) -> Report {
    Report {
        span: node_span(&node),
        message,
        help: None,
        fix: None,
    }
}

pub struct NoDynamicMethodCall;

impl Rule for NoDynamicMethodCall {
    fn id(&self) -> &'static str {
        "lint/style/no-dynamic-method-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::NullSafeMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (object, method, nullsafe) = match node {
            Node::MethodCall(c) => (c.object, &c.method, false),
            Node::NullSafeMethodCall(c) => (c.object, &c.method, true),
            _ => return,
        };
        if dynamic(method).is_some() {
            let Some(on) = type_of(ctx, object, nullsafe) else {
                return;
            };
            out.push(report(node, format!("Variable method call on {on}.")));
        }
    }
}

pub struct NoDynamicMethodCallable;

impl Rule for NoDynamicMethodCallable {
    fn id(&self) -> &'static str {
        "lint/style/no-dynamic-method-callable"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodPartialApplication])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodPartialApplication(c) = node else {
            return;
        };
        if c.argument_list.is_first_class_callable() && dynamic(&c.method).is_some() {
            let Some(on) = type_of(ctx, c.object, false) else {
                return;
            };
            out.push(report(node, format!("Variable method call on {on}.")));
        }
    }
}

pub struct NoDynamicStaticCall;

impl Rule for NoDynamicStaticCall {
    fn id(&self) -> &'static str {
        "lint/style/no-dynamic-static-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::StaticMethodCall(c) = node else {
            return;
        };
        if let Some(name) = dynamic(&c.method) {
            let Some(on) = class_of(ctx, ancestors, name, c.class) else {
                return;
            };
            out.push(report(
                node,
                format!("Variable static method call on {on}."),
            ));
        }
    }
}

pub struct NoDynamicStaticCallable;

impl Rule for NoDynamicStaticCallable {
    fn id(&self) -> &'static str {
        "lint/style/no-dynamic-static-callable"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::StaticMethodPartialApplication])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::StaticMethodPartialApplication(c) = node else {
            return;
        };
        if let Some(name) = dynamic(&c.method)
            && c.argument_list.is_first_class_callable()
        {
            let Some(on) = class_of(ctx, ancestors, name, c.class) else {
                return;
            };
            out.push(report(
                node,
                format!("Variable static method call on {on}."),
            ));
        }
    }
}

pub struct NoDynamicStaticProperty;

impl Rule for NoDynamicStaticProperty {
    fn id(&self) -> &'static str {
        "lint/style/no-dynamic-static-property"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::StaticPropertyAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::StaticPropertyAccess(p) = node else {
            return;
        };
        let name = match &p.property {
            Variable::Direct(_) => return,
            Variable::Indirect(i) => node_span(unparenthesize(i.expression)),
            Variable::Nested(n) => node_span(n.variable),
        };
        let Some(on) = class_of(ctx, ancestors, name, p.class) else {
            return;
        };
        out.push(report(
            node,
            format!("Variable static property access on {on}."),
        ));
    }
}

pub struct NoDynamicProperty;

impl Rule for NoDynamicProperty {
    fn id(&self) -> &'static str {
        "lint/style/no-dynamic-property"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::PropertyAccess, NodeKind::NullSafePropertyAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (object, property, nullsafe) = match node {
            Node::PropertyAccess(p) => (p.object, &p.property, false),
            Node::NullSafePropertyAccess(p) => (p.object, &p.property, true),
            _ => return,
        };
        let Some(name) = dynamic(property) else {
            return;
        };
        let facts = facts(ctx);
        // `isLiteralString()->yes()`: every member of the name's type is a string from literals.
        let literal = facts.full_type(name).is_some_and(|t| {
            !t.types.is_empty()
                && t.types.iter().all(
                    |a| matches!(a, TAtomic::Scalar(TScalar::String(s)) if s.literal.is_some()),
                )
        });
        if literal {
            return;
        }
        let crate_class = facts
            .full_type(node_span(unparenthesize(object)))
            .is_some_and(|t| {
                t.types.iter().any(|a| match a {
                    TAtomic::Object(TObject::Named(n)) => CRATES.iter().any(|c| {
                        facts
                            .codebase
                            .is_instance_of(n.name.as_bytes(), c.as_bytes())
                    }),
                    _ => false,
                })
            });
        if crate_class {
            return;
        }
        let Some(on) = type_of(ctx, object, nullsafe) else {
            return;
        };
        out.push(report(node, format!("Variable property access on {on}.")));
    }
}
