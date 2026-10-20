//! CallToMethodStatementWithNoDiscardRule and CallToStaticMethodStatementWithNoDiscardRule (PHP 8.5).

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::super::{Checker, members};

struct Target<'e, 'arena> {
    receiver: &'e Expression<'arena>,
    name: &'e [u8],
    is_static: bool,
    nullsafe: bool,
    void: bool,
}

pub(super) fn unparen<'e, 'arena>(mut e: &'e Expression<'arena>) -> &'e Expression<'arena> {
    while let Expression::Parenthesized(p) = e {
        e = p.expression;
    }
    e
}

fn is_void(e: &Expression) -> bool {
    matches!(unparen(e), Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::VoidCast(..)))
}

/// The discarded expression, whether its result is cast to void, and whether it is a pipe's callable.
pub(super) fn expression<'e, 'arena>(
    s: &'e ExpressionStatement<'arena>,
) -> (&'e Expression<'arena>, bool, bool) {
    let mut e = unparen(s.expression);
    let mut void = is_void(e);
    if let Expression::UnaryPrefix(u) = e
        && void
    {
        e = unparen(u.operand);
    }
    let mut piped = false;
    if let Expression::Pipe(p) = e {
        // PHP's `(void) $value |> f(...)` casts the result of the pipe, not its input.
        void |= is_void(p.input);
        e = unparen(p.callable);
        if let Expression::ArrowFunction(f) = e {
            e = unparen(f.expression);
        } else {
            piped = true;
        }
    }
    (e, void, piped)
}

fn target<'e, 'arena>(s: &'e ExpressionStatement<'arena>) -> Option<Target<'e, 'arena>> {
    let (e, void, piped) = expression(s);
    let (receiver, selector, is_static) = match e {
        Expression::Call(Call::Method(m)) if !piped => (m.object, &m.method, false),
        Expression::Call(Call::NullSafeMethod(m)) if !piped => (m.object, &m.method, false),
        Expression::Call(Call::StaticMethod(m)) if !piped => (m.class, &m.method, true),
        Expression::PartialApplication(PartialApplication::Method(m)) if piped => {
            (m.object, &m.method, false)
        }
        Expression::PartialApplication(PartialApplication::StaticMethod(m)) if piped => {
            (m.class, &m.method, true)
        }
        _ => return None,
    };
    let ClassLikeMemberSelector::Identifier(name) = selector else {
        return None;
    };
    Some(Target {
        receiver,
        name: name.value,
        is_static,
        nullsafe: matches!(e, Expression::Call(Call::NullSafeMethod(_))),
        void,
    })
}

pub(crate) fn typed(s: &ExpressionStatement, level: u8, php_version: u32) -> Option<Span> {
    let receiver = target(s)?.receiver;
    if level < 2
        || php_version < 80500
        || matches!(
            unparen(receiver),
            Expression::Identifier(_)
                | Expression::Self_(_)
                | Expression::Static(_)
                | Expression::Parent(_)
        )
        || matches!(unparen(receiver), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
    {
        return None;
    }
    Some(receiver.span())
}

fn named_class<'a>(c: &Checker<'a, '_>, e: &Expression) -> Option<&'a ClassLikeMetadata> {
    match unparen(e) {
        Expression::Identifier(id) => c.codebase.get_class_like(c.names.get(&id.span())),
        Expression::Self_(_) | Expression::Static(_) => c.class_scope(),
        Expression::Parent(_) => c
            .class_scope()?
            .direct_parent_class
            .and_then(|p| c.codebase.get_class_like(p.as_bytes())),
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => c.class_scope(),
        _ => None,
    }
}

pub(crate) fn statement(c: &mut Checker, s: &ExpressionStatement) {
    if c.php_version < 80500 {
        return;
    }
    let Some(target) = target(s) else { return };
    // RuleLevelHelper::findTypeToCheck checks only `$this` below level 2; a named static receiver bypasses it.
    let named_static = target.is_static
        && matches!(
            unparen(target.receiver),
            Expression::Identifier(_)
                | Expression::Self_(_)
                | Expression::Static(_)
                | Expression::Parent(_)
        );
    let this = matches!(unparen(target.receiver), Expression::Variable(Variable::Direct(v)) if v.name == b"$this");
    if c.level < 2 && !named_static && !this {
        return;
    }
    let mut classes = Vec::new();
    if let Some(class) = named_class(c, target.receiver) {
        classes.push(vec![class]);
    } else if let Some(t) = c.type_of(target.receiver) {
        for atomic in t.types.iter() {
            let names = atomic.get_all_object_names();
            if !names.is_empty() {
                let intersection: Vec<_> = names
                    .iter()
                    .filter_map(|name| c.codebase.get_class_like(name.as_bytes()))
                    .collect();
                if c.level < 7
                    && t.types.len() > 1
                    && !intersection
                        .iter()
                        .any(|class| members::method(c, class, target.name).is_some())
                {
                    continue;
                }
                classes.push(intersection);
            } else if !(atomic.is_null() && (c.level < 8 || target.nullsafe)
                || c.level < 7 && t.types.len() > 1)
            {
                return;
            }
        }
    }
    if classes.is_empty() {
        return;
    }
    let union = classes.len() > 1;
    let mut found = Vec::new();
    for intersection in classes {
        let methods: Vec<_> = intersection
            .iter()
            .filter_map(|class| members::method(c, class, target.name))
            .collect();
        if methods.is_empty() {
            return;
        }
        // IntersectionTypeMethodReflection uses any NoDiscard attribute and the widest signature's class.
        let must_use = methods.iter().any(|method| match method {
            members::Method::User { f, .. } => f
                .attributes
                .iter()
                .any(|a| a.name.as_bytes().eq_ignore_ascii_case(b"NoDiscard")),
            members::Method::Builtin { declaring, f } => c
                .codebase
                .get_method(declaring.name.as_bytes(), f.name.as_bytes())
                .is_some_and(|f| {
                    f.attributes
                        .iter()
                        .any(|a| a.name.as_bytes().eq_ignore_ascii_case(b"NoDiscard"))
                }),
        });
        let mut selected = 0;
        let mut max_parameters = 0;
        for (i, method) in methods.iter().enumerate() {
            for variant in method.variants().all {
                if variant.optional.len() > max_parameters {
                    max_parameters = variant.optional.len();
                    selected = i;
                }
            }
        }
        let method = &methods[selected];
        found.push((
            must_use,
            methods.iter().any(members::Method::is_static),
            method.declaring_display(c),
            if union || methods.len() > 1 {
                String::from_utf8_lossy(target.name).into_owned()
            } else {
                method.name()
            },
        ));
    }
    let must_use = found.iter().all(|method| method.0);
    let allows_discard = found.iter().all(|method| !method.0);
    let subject = format!(
        "{} {}::{}()",
        if found.iter().all(|method| method.1) {
            "static method"
        } else {
            "method"
        },
        found[0].2,
        found[0].3,
    );
    let kind = if target.is_static {
        "staticMethod"
    } else {
        "method"
    };
    if target.void && allows_discard {
        c.report(
            s,
            &format!("{kind}.inVoidCast"),
            format!("Call to {subject} in (void) cast but method allows discarding return value."),
            None,
        );
    } else if !target.void && must_use {
        c.report_non_ignorable(
            s,
            &format!("{kind}.resultDiscarded"),
            format!("Call to {subject} on a separate line discards return value."),
            None,
        );
    }
}
