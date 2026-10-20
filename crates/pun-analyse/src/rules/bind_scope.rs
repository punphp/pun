//! The class a rebound closure's body is checked in (StaticCallHandler's `Closure::bind()` scope factory,
//! MethodCallHandler's `->call()`, MutatingScope::enterClosureBind()/enterClosureCall()): the bound `$this`'s class.

use mago_syntax::cst::*;

use super::Checker;

impl<'arena> Checker<'_, 'arena> {
    /// The class of `Closure::bind($closure, $newThis, $newScope)`'s `$this`: `$newScope`'s class when it is given,
    /// else `$newThis`'s.
    // ponytail: classes read from the written expressions (`X::class`, `'X'`, `new X`, `$this`), not their types; the
    // bound scope classes are not kept apart from `$this`'s class
    pub(super) fn bind_class(&self, args: &ArgumentList<'arena>) -> Option<String> {
        let arg = |i: usize| args.arguments.get(i).map(|a| a.value());
        match arg(2) {
            Some(scope) => self.bound_class_of(scope, true),
            None => self.bound_class_of(arg(1)?, false),
        }
    }

    /// The class of `->call($newThis)`'s `$this`.
    pub(super) fn call_class(&self, args: &ArgumentList<'arena>) -> Option<String> {
        self.bound_class_of(args.arguments.first()?.value(), false)
    }

    /// The class an object expression is an instance of, or (`class_string`) the class a class string names.
    fn bound_class_of(&self, e: &Expression<'arena>, class_string: bool) -> Option<String> {
        let current = || {
            self.class_scope()
                .map(|k| String::from_utf8_lossy(k.name.as_bytes()).into_owned())
        };
        match e {
            Expression::Parenthesized(p) => self.bound_class_of(p.expression, class_string),
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => current(),
            Expression::Instantiation(new) => match new.class {
                Expression::Identifier(id) => Some(self.resolved(id)),
                Expression::Self_(_) | Expression::Static(_) => current(),
                _ => None,
            },
            Expression::Access(Access::ClassConstant(k))
                if class_string
                    && matches!(&k.constant, ClassLikeConstantSelector::Identifier(i) if i.value.eq_ignore_ascii_case(b"class")) =>
            {
                match k.class {
                    Expression::Identifier(id) => Some(self.resolved(id)),
                    Expression::Self_(_) | Expression::Static(_) => current(),
                    object => self.bound_class_of(object, false),
                }
            }
            Expression::Literal(Literal::String(s)) if class_string => {
                let name = String::from_utf8_lossy(s.value?).replace("\\\\", "\\");
                if name.eq_ignore_ascii_case("static") {
                    current()
                } else {
                    Some(name.trim_start_matches('\\').to_string())
                }
            }
            _ => None,
        }
    }
}
