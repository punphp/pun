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
use mago_syntax::cst::{
    ClassLikeMemberSelector, Expression, Identifier, Node, NodeKind, PartialArgument,
    PartialArgumentList, Variable,
};

use super::phpstan_describe::describe;
use super::rector_batch5::resolve_class_name;
use super::types::class_scope;
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// The classes PHPStan lets any property be read on: its `universalObjectCratesClasses` default and the rule's own
/// `SimpleXMLElement` check.
const CRATES: &[&str] = &["stdClass", "SimpleXMLElement"];

fn dynamic(selector: &ClassLikeMemberSelector<'_>) -> bool {
    matches!(
        selector,
        ClassLikeMemberSelector::Variable(_) | ClassLikeMemberSelector::Expression(_)
    )
}

/// Whether a partial application is a first-class callable, `(...)`.
fn first_class(list: &PartialArgumentList<'_>) -> bool {
    list.arguments.len() == 1
        && matches!(
            list.arguments.get(0),
            Some(PartialArgument::VariadicPlaceholder(_))
        )
}

/// `$scope->getType($expr)->describe(VerbosityLevel::typeOnly())`. A nullsafe access is analysed as the plain access
/// on its receiver without `null` (`TypeCombinator::removeNull`), so `nullsafe` drops it.
fn type_of(ctx: &Context<'_>, expr: &Expression<'_>, nullsafe: bool) -> String {
    let Some(mut t) = facts(ctx).full_type(node_span(unparenthesize(expr))).cloned() else {
        return "mixed".into();
    };
    if nullsafe && t.types.len() > 1 {
        t.types = t.types.iter().filter(|a| !a.is_null()).cloned().collect();
    }
    let described = describe(&t);
    // An anonymous class is `class@anonymous/<path>:<line>` in PHPStan; Mago names it
    // `{anonymous-class:<path>:<line>:<column>}`.
    match described
        .strip_prefix("{anonymous-class:")
        .and_then(|rest| rest.strip_suffix('}'))
        .and_then(|rest| rest.rsplitn(3, ':').nth(1))
    {
        Some(line) => format!(
            "class@anonymous/{}:{line}",
            ctx.path.display().to_string().trim_start_matches("./")
        ),
        None => described,
    }
}

/// What a static member is accessed on: `Scope::resolveName` for a name, the expression's type otherwise.
fn class_of(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], class: &Expression<'_>) -> String {
    let enclosing = || class_scope(ancestors).and_then(|c| c.name);
    match unparenthesize(class) {
        Expression::Self_(_) | Expression::Static(_) => {
            enclosing().unwrap_or_else(|| ctx.source[node_span(class).start..node_span(class).end].to_string())
        }
        Expression::Parent(_) => class_scope(ancestors)
            .and_then(|c| c.parent_class)
            .unwrap_or_else(|| "parent".into()),
        Expression::Identifier(id) => {
            let written = String::from_utf8_lossy(id.value());
            match (id, written.to_ascii_lowercase().as_str()) {
                (Identifier::Local(_), "self" | "static") => {
                    enclosing().unwrap_or_else(|| written.into_owned())
                }
                (Identifier::Local(_), "parent") => class_scope(ancestors)
                    .and_then(|c| c.parent_class)
                    .unwrap_or_else(|| written.into_owned()),
                _ => resolve_class_name(ancestors, id),
            }
        }
        other => type_of(ctx, other, false),
    }
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
        if dynamic(method) {
            let on = type_of(ctx, object, nullsafe);
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
        if first_class(&c.argument_list) && dynamic(&c.method) {
            let on = type_of(ctx, c.object, false);
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
        if dynamic(&c.method) {
            let on = class_of(ctx, ancestors, c.class);
            out.push(report(node, format!("Variable static method call on {on}.")));
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
        if first_class(&c.argument_list) && dynamic(&c.method) {
            let on = class_of(ctx, ancestors, c.class);
            out.push(report(node, format!("Variable static method call on {on}.")));
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
        if !matches!(p.property, Variable::Direct(_)) {
            let on = class_of(ctx, ancestors, p.class);
            out.push(report(
                node,
                format!("Variable static property access on {on}."),
            ));
        }
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
        let name = match property {
            ClassLikeMemberSelector::Variable(v) => node_span(v),
            ClassLikeMemberSelector::Expression(e) => node_span(unparenthesize(e.expression)),
            _ => return,
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
        let crate_class = facts.full_type(node_span(unparenthesize(object))).is_some_and(|t| {
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
        let on = type_of(ctx, object, nullsafe);
        out.push(report(node, format!("Variable property access on {on}.")));
    }
}
