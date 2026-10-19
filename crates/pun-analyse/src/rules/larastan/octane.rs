//! OctaneCompatibilityRule: `singleton()` and `bind()` on the application with a closure that captures the
//! container (`$this->app`, or `new X($app)`, `new X($app['request'])`, `new X($app['config'])`).

use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::super::Checker;
use super::{is_a, object_names};

const APPLICATION: &[u8] = b"illuminate\\contracts\\foundation\\application";

const TIP: &str = "See: https://laravel.com/docs/octane#dependency-injection-and-octane";

fn unparenthesized<'ast, 'arena>(mut e: &'ast Expression<'arena>) -> &'ast Expression<'arena> {
    while let Expression::Parenthesized(p) = e {
        e = p.expression;
    }
    e
}

/// `->singleton()` or `->bind()` with a closure as its second argument.
pub(super) fn is_binding(call: &MethodCall) -> bool {
    binding_closure(call).is_some()
}

fn binding_closure<'ast, 'arena>(call: &'ast MethodCall<'arena>) -> Option<&'ast Closure<'arena>> {
    let ClassLikeMemberSelector::Identifier(m) = &call.method else {
        return None;
    };
    if m.value != b"singleton" && m.value != b"bind" {
        return None;
    }
    match unparenthesized(call.argument_list.arguments.get(1)?.value()) {
        Expression::Closure(f) => Some(f),
        _ => None,
    }
}

pub(super) fn check<'arena>(c: &mut Checker<'_, 'arena>, call: &MethodCall<'arena>) {
    let Some(closure) = binding_closure(call) else {
        return;
    };
    let names = c
        .type_of(call.object)
        .and_then(object_names)
        .unwrap_or_default();
    let [name] = names.as_slice() else {
        return;
    };
    if !is_a(c, name, APPLICATION) {
        return;
    }
    let statements = closure.body.statements.as_slice();
    let found = match closure.parameter_list.parameters.first() {
        None => {
            let mut finder = ThisApp(Vec::new());
            statements
                .iter()
                .for_each(|s| finder.walk_statement(s, &mut ()));
            finder.0
        }
        // using `$app` with `bind` is fine
        Some(_) if matches!(&call.method, ClassLikeMemberSelector::Identifier(m) if m.value == b"bind") =>
        {
            return;
        }
        Some(p) => {
            let mut finder = ContainerPassed {
                container: p.variable.name,
                found: Vec::new(),
            };
            statements
                .iter()
                .for_each(|s| finder.walk_statement(s, &mut ()));
            finder.found
        }
    };
    for span in found {
        c.report(
            span,
            "larastan.octaneCompatibility",
            "Consider using bind method instead or pass a closure.".to_string(),
            Some(TIP),
        );
    }
}

/// `$this->app`.
struct ThisApp(Vec<mago_span::Span>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for ThisApp {
    fn walk_in_property_access(&mut self, a: &'ast PropertyAccess<'arena>, _: &mut ()) {
        if matches!(a.object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
            && matches!(&a.property, ClassLikeMemberSelector::Identifier(p) if p.value == b"app")
        {
            self.0.push(a.span());
        }
    }
}

/// `new X($container)`, `new X($container['request'])`, `new X($container['config'])`.
struct ContainerPassed<'arena> {
    container: &'arena [u8],
    found: Vec<mago_span::Span>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for ContainerPassed<'arena> {
    fn walk_in_instantiation(&mut self, new: &'ast Instantiation<'arena>, _: &mut ()) {
        let Some(first) = new.argument_list.as_ref().and_then(|l| l.arguments.first()) else {
            return;
        };
        let passed = match first.value() {
            Expression::Variable(Variable::Direct(v)) => v.name == self.container,
            Expression::ArrayAccess(a) => {
                matches!(a.array, Expression::Variable(Variable::Direct(v)) if v.name == self.container)
                    && matches!(a.index, Expression::Literal(Literal::String(s))
                        if matches!(s.value, Some(b"request" | b"config")))
            }
            _ => false,
        };
        if passed {
            self.found.push(new.span());
        }
    }
}
