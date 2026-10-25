//! MethodCallableRule and StaticMethodCallableRule: member checks, never argument checks.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::Type;

use super::types::flow::unparen;
use super::{Checker, calls, declared, internal, scope, types};

fn is_this(e: &Expression) -> bool {
    matches!(unparen(e), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
}

/// IsCallableFunctionTypeSpecifyingExtension delegates a literal `[receiver, 'method']` to method_exists.
/// Keep the existing scope's boolean/branch handling instead of special-casing callable diagnostics.
pub(super) fn literal_guard(
    e: &Expression,
    resolve: &dyn Fn(&Identifier) -> String,
) -> Option<String> {
    let Expression::Array(array) = unparen(e) else {
        return None;
    };
    let [ArrayElement::Value(receiver), ArrayElement::Value(method)] = array.elements.as_slice()
    else {
        return None;
    };
    let Expression::Literal(Literal::String(method)) = method.value else {
        return None;
    };
    let receiver = scope::method_receiver(receiver.value, resolve)?;
    Some(format!(
        "{receiver}::{}",
        String::from_utf8_lossy(method.value?)
    ))
}

impl Checker<'_, '_> {
    /// The original receiver, and whether the condition promises the method. For `is_callable`, PHPStan only
    /// delegates to method_exists when the original constant array is not definitely non-callable.
    pub(super) fn method_guard_source(&self, key: &str) -> Option<(Type, bool)> {
        let wanted = scope::Guard::Method(key.to_string());
        let span = self.guards.iter().find_map(|g| match g {
            scope::Guard::MethodSource(k, span) if scope::Guard::Method(k.clone()) == wanted => {
                Some(span)
            }
            _ => None,
        })?;
        let source = self.to_phpstan(self.types.get(span)?);
        if let Type::ConstArray(array) = &source {
            let receiver = array.values.first()?.clone();
            let specified = !declared::with_classes(self, || source.is_callable().no());
            Some((receiver, specified))
        } else {
            Some((source, true))
        }
    }

    pub(super) fn has_method_guard(&self, key: &str) -> bool {
        self.guards.contains(&scope::Guard::Method(key.to_string()))
            && self
                .method_guard_source(key)
                .is_none_or(|(_, specified)| specified)
    }
}

/// Request the same receiver types as ordinary calls; checkThisOnly still applies below level 2.
pub(super) fn typed_expressions(program: &Program, level: u8) -> Vec<Span> {
    struct Wanted(u8, Vec<Span>);
    impl Wanted {
        fn receiver(
            &mut self,
            e: &Expression,
            selector: &ClassLikeMemberSelector,
            args: &PartialArgumentList,
        ) {
            if internal::is_callable(args)
                && matches!(selector, ClassLikeMemberSelector::Identifier(_))
                && (self.0 >= 2 || is_this(e))
            {
                self.1.push(unparen(e).span());
            }
        }
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_method_partial_application(
            &mut self,
            m: &'ast MethodPartialApplication<'arena>,
            _: &mut (),
        ) {
            self.receiver(m.object, &m.method, &m.argument_list);
        }
        fn walk_in_static_method_partial_application(
            &mut self,
            m: &'ast StaticMethodPartialApplication<'arena>,
            _: &mut (),
        ) {
            if !named_class(m.class) {
                self.receiver(m.class, &m.method, &m.argument_list);
            }
        }
    }
    let mut wanted = Wanted(level, Vec::new());
    wanted.walk_program(program, &mut ());
    wanted.1
}

pub(super) fn method<'arena>(c: &mut Checker<'_, 'arena>, m: &MethodPartialApplication<'arena>) {
    if c.piped.contains(&m.span()) || !internal::is_callable(&m.argument_list) {
        return;
    }
    let object = unparen(m.object);
    if is_this(object) && c.this_defined() {
        calls::check_this_method(c, object, &m.method, None);
    } else {
        types::check_method(c, object, &m.method, None, false);
    }
}

fn named_class(e: &Expression) -> bool {
    matches!(
        e,
        Expression::Identifier(_)
            | Expression::Self_(_)
            | Expression::Static(_)
            | Expression::Parent(_)
    )
}

pub(super) fn static_method<'arena>(
    c: &mut Checker<'_, 'arena>,
    m: &StaticMethodPartialApplication<'arena>,
) {
    if c.piped.contains(&m.span()) || !internal::is_callable(&m.argument_list) {
        return;
    }
    if named_class(m.class) {
        calls::check_static_method(c, m.class, &m.method, None);
    } else {
        types::check_static(c, unparen(m.class), &m.method, None);
    }
}
