//! CallMethodsRule's MethodCallCheck once `checkThisOnly` is off (level 2): a call on any expression, through
//! RuleLevelHelper's `findTypeToCheck()` (unions from level 7, nullables from level 8, explicit `mixed` from 9).
//! Calls on `$this` stay with calls.rs.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{Level, Tri, Type, intersect, is_super_type_of};

use super::super::reflection::Lookup;
use super::super::{Checker, calls, declared, members, params};

pub const IDENTIFIERS: &[&str] = &["method.nonObject", "staticMethod.nonObject"];

fn all(results: impl IntoIterator<Item = Tri>) -> Tri {
    // `TrinaryLogic::extremeIdentity()`: yes or no when all agree, else maybe
    let mut it = results.into_iter();
    let Some(first) = it.next() else { return Tri::No };
    it.fold(first, |a, b| if a == b { a } else { Tri::Maybe })
}

fn any(results: impl IntoIterator<Item = Tri>) -> Tri {
    results.into_iter().fold(Tri::No, Tri::or)
}

impl<'a> Checker<'a, '_> {
    /// Restore the literal method accessory Mago does not carry. Called by `phpstan_type()` after conversion;
    /// the existing branch guards determine where it holds, rather than the debug rule's rendering.
    pub(in super::super) fn refine_method_guards(&self, e: &Expression, mut ty: Type) -> Type {
        let Some(receiver) = super::super::scope::method_receiver(e, &|id| self.resolved(id)) else { return ty; };
        let prefix = format!("{receiver}::");
        let original = self.guards.iter().find_map(|g| match g {
            super::super::scope::Guard::MethodSource(key, span) if key.starts_with(&prefix) => self.types.get(span),
            _ => None,
        }).map(|t| self.to_phpstan(t));
        if let Some(original) = &original {
            // Undo only Mago's method_exists-specific promotions/erasures, not a later instanceof/is_string
            // refinement. In particular a named subclass must not be widened back to the pre-guard object.
            if (matches!(ty, Type::ClassString(None)) && original.is_string().yes())
                || matches!((&ty, original), (Type::ObjectWithoutClass, Type::Mixed { .. }))
            {
                ty = original.clone();
            }
        }
        let classes = match &ty {
            Type::ClassString(Some(t)) => t.object_class_names(),
            _ => ty.object_class_names(),
        };
        for guard in &self.guards {
            let super::super::scope::Guard::Method(key) = guard else { continue; };
            let Some(method) = key.strip_prefix(&prefix) else { continue; };
            // PHPStan does not narrow @method/mixin-only methods: the guard promises a native method.
            if classes.iter().any(|n| {
                matches!(self.method_of(n, method.as_bytes()), Ok(Some(_)))
                    && self.class_like(n).found().is_some_and(|k| members::method(self, k, method.as_bytes()).is_none())
            }) { continue; }
            ty = declared::with_classes(self, || {
                let specified = original.as_ref().unwrap_or(&ty).clone().require_method(method);
                intersect(vec![specified, ty])
            });
        }
        ty
    }

    /// `Type::canCallMethods()`; `strict` says a `mixed` is the StrictMixedType findTypeToCheck() leaves.
    pub(super) fn can_call_methods(&self, t: &Type, strict: bool) -> Tri {
        self.capable(t, strict, true)
    }

    /// `Type::canAccessProperties()`.
    pub(super) fn can_access_properties(&self, t: &Type, strict: bool) -> Tri {
        self.capable(t, strict, false)
    }

    fn capable(&self, t: &Type, strict: bool, methods: bool) -> Tri {
        match t {
            // ObjectType::canCallMethods(): not on a plain `stdClass`
            Type::Object(n, _) if methods && n.eq_ignore_ascii_case("stdClass") => Tri::No,
            Type::Object(..)
            | Type::Static(..)
            | Type::This(_)
            | Type::EnumCase(..)
            | Type::ObjectWithoutClass
            | Type::ObjectShape(_)
            | Type::Closure(_) => Tri::Yes,
            Type::Union(ts) => all(ts.iter().map(|t| self.capable(t, strict, methods))),
            Type::Benevolent(ts) | Type::Intersection(ts) => any(ts.iter().map(|t| self.capable(t, strict, methods))),
            Type::Template(tt) => self.capable(&tt.bound, strict, methods),
            Type::Mixed { .. } | Type::Callable(None) | Type::Iterable(..) | Type::Error => Tri::Maybe,
            _ => Tri::No,
        }
    }

    /// The class a receiver type names, for its methods.
    fn class_of<'t>(t: &'t Type) -> Option<&'t str> {
        match t {
            Type::Object(n, _) | Type::Static(n, _) | Type::This(n) | Type::EnumCase(n, _) => Some(n),
            t => matches!(t, Type::Closure(_)).then_some("Closure"),
        }
    }

    /// The method `name` of class `n`, with `@method` tags; `Err` when PHPStan may find one Pun does not see
    /// (`@mixin`, `@phpstan-require-extends`, a built-in Mago lacks).
    fn method_of(&self, n: &str, name: &[u8]) -> Result<Option<members::Method<'a>>, ()> {
        let Lookup::Found(k) = self.class_like(n) else { return Err(()) };
        if let Some(m) = members::method(self, k, name).or_else(|| members::annotation_method(self, k, name)) {
            return Ok(Some(m));
        }
        // RequireExtendsMethodsClassReflectionExtension
        if let Some(m) = self.required(k).into_iter().find_map(|r| members::method(self, r, name)) {
            return Ok(Some(m));
        }
        // MixinMethodsClassReflectionExtension; ponytail: a mixin other than a class (`@mixin T`) is not resolved
        let mut found = Ok(None);
        for t in self.mixins(k) {
            match t {
                Type::Object(n, _) => {
                    if let Some(m) = self.class_like(&n).found().and_then(|m| members::method(self, m, name)) {
                        return Ok(Some(m));
                    }
                }
                _ => found = Err(()),
            }
        }
        found
    }

    /// `Type::hasMethod()`.
    fn has_method(&self, t: &Type, name: &[u8]) -> Tri {
        match t {
            Type::Union(ts) => all(ts.iter().map(|t| self.has_method(t, name))),
            Type::Benevolent(ts) | Type::Intersection(ts) => any(ts.iter().map(|t| self.has_method(t, name))),
            Type::Template(tt) => self.has_method(&tt.bound, name),
            // (`Callable(None)` as a nested pattern leaves an uncovered region on the `match` line)
            t if matches!(t, Type::Mixed { .. } | Type::ObjectWithoutClass | Type::Iterable(..) | Type::Error)
                || matches!(t, Type::Callable(None)) =>
            {
                Tri::Maybe
            }
            // (what Pun does not see is taken as there: the call is not checked further)
            t => Self::class_of(t).map_or(Tri::No, |n| if matches!(self.method_of(n, name), Ok(None)) { Tri::No } else { Tri::Yes }),
        }
    }

    /// How many of the type's classes have method `name`.
    fn classes_with(&self, t: &Type, name: &[u8]) -> usize {
        t.object_class_names().iter().filter(|n| matches!(self.method_of(n, name), Ok(Some(_)))).count()
    }

    /// `Type::getMethod()` of a type that has it.
    // ponytail: a union's UnionTypeMethodReflection is its first member's method
    fn get_method(&self, t: &Type, name: &[u8]) -> Option<members::Method<'a>> {
        match t {
            Type::Union(ts) | Type::Benevolent(ts) | Type::Intersection(ts) => {
                ts.iter().find_map(|t| self.get_method(t, name))
            }
            Type::Template(tt) => self.get_method(&tt.bound, name),
            t => self.method_of(Self::class_of(t)?, name).ok().flatten(),
        }
    }
}

/// `ClassReflection::getDisplayName()` of the method's declaring class as the receiver's type resolves it: a generic
/// object of that class shows its type arguments (`Foo<int,string>`).
// ponytail: an ancestor's resolved template map (a method inherited from a generic parent) is not shown
fn generic_display(c: &Checker, t: &Type, method: &members::Method) -> String {
    let display = method.declaring_display(c);
    match t {
        Type::Object(n, args) if !args.is_empty() && n.eq_ignore_ascii_case(&method.declaring_name()) => {
            let args: Vec<String> = declared::with_classes(c, || args.iter().map(|a| a.describe(Level::TypeOnly)).collect());
            format!("{display}<{}>", args.join(","))
        }
        _ => display,
    }
}

/// `$type instanceof StaticType ? $type->getStaticObjectType() : $type`, described.
fn describe(c: &Checker, t: &Type) -> String {
    let t = match t {
        Type::Static(n, args) => Type::Object(n.clone(), args.clone()),
        Type::This(n) => Type::Object(n.clone(), vec![]),
        t => t.clone(),
    };
    declared::with_classes(c, || t.describe(Level::TypeOnly))
}

/// `NonNullabilityHelper::ensureShallowNonNullability()`: a nullsafe receiver without `null`, unless it is only
/// `null`.
pub(super) fn non_null(t: Type) -> Type {
    if t.is_null().yes() { t } else { t.remove_null() }
}

/// The receivers whose type the check reads: every method call's object but `$this`'s (level 2).
pub(super) fn typed(object: &Expression, spans: &mut Vec<Span>) {
    if !is_this(object) {
        spans.push(object.span());
    }
}

fn is_this(e: &Expression) -> bool {
    matches!(e, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
}

/// MethodCallCheck::check() and the parameters check of `$object->name(...)` (`nullsafe` for `?->`).
pub(in super::super) fn method_call<'arena>(
    c: &mut Checker<'_, 'arena>,
    span: Span,
    object: &Expression<'arena>,
    selector: &ClassLikeMemberSelector<'arena>,
    args: &ArgumentList<'arena>,
    nullsafe: bool,
) {
    if c.level < 2 || is_this(object) {
        return;
    }
    let ClassLikeMemberSelector::Identifier(name) = selector else {
        return; // ponytail: a dynamic name's constant strings
    };
    // NullsafeOperatorHelper: a nullsafe call is checked on the receiver without `null`
    let ty = c
        .types
        .get(&object.span())
        .map(|t| c.to_phpstan(t))
        .map(|t| if nullsafe || super::super::assign::contains_nullsafe(object) { non_null(t) } else { t });
    let found = {
        let c = &*c;
        c.find_type_to_check_of(ty, &|t| {
            c.can_call_methods(t, false).yes() && c.has_method(t, name.value).yes()
        })
    };
    let Some(t) = found else {
        return;
    };
    let method_name = String::from_utf8_lossy(name.value).into_owned();
    if !c.can_call_methods(&t, true).yes() || matches!(t, Type::ClassString(_)) {
        let message = format!("Cannot call method {method_name}() on {}.", describe(c, &t));
        c.report(name, "method.nonObject", message, None);
        return;
    }
    if !c.has_method(&t, name.value).yes() {
        // a true `method_exists($var, 'name')`
        if let Expression::Variable(Variable::Direct(v)) = object {
            let key = format!("{}::{method_name}", String::from_utf8_lossy(v.name));
            if c.guards.contains(&super::super::scope::Guard::Method(key)) {
                return;
            }
        }
        // ponytail: a parent's private method of the one class named (`method.private`) is reported as not found
        let message = format!("Call to an undefined method {}::{method_name}().", describe(c, &t));
        c.report(name, "method.notFound", message, None);
        return;
    }
    let Some(method) = c.get_method(&t, name.value) else {
        return;
    };
    let declaring = generic_display(c, &t, &method);
    let visibility = method.visibility();
    if !calls::can_call(c, c.class_scope(), &method.declaring_name(), visibility) {
        let v = super::super::restricted(visibility);
        let identifier = if visibility.is_private() { "method.private" } else { "method.protected" };
        let message = format!("Call to {v} method {}() of class {declaring}.", method.name());
        c.report(name, identifier, message, None);
    }
    // ponytail: a union's or intersection's method merges its members' variants in PHPStan; checked only when one
    // member has it
    if c.classes_with(&t, name.value) != 1 {
        return;
    }
    let subject = format!("{declaring}::{}()", method.name());
    let first_level = c.is_top_level(span);
    let counts = params::count(args);
    let mut variants = method.variants();
    // ClosureBindToVarVisitor: `$var->bindTo(...)` on a Closure
    if method.name().eq_ignore_ascii_case("bindTo")
        && !args.arguments.is_empty()
        && let Expression::Variable(Variable::Direct(v)) = object
        && declared::with_classes(c, || is_super_type_of(&Type::Object("Closure".into(), vec![]), &t).yes())
    {
        let var = String::from_utf8_lossy(&v.name[1..]);
        variants = super::super::selector::closure_bind(c, &var, span.start.offset, 0, variants);
    }
    params::check(
        c,
        &params::select(c, variants, &counts),
        &counts,
        name,
        params::Call {
            kind: params::CallKind::Method,
            subject: &format!("Method {subject}"),
            void_message: Some(format!("Result of method {subject} (void) is used.")),
        },
        first_level,
    );
}

/// The class expressions of static calls the check reads (`$x::foo()`; names and keywords are calls.rs's).
pub(super) fn typed_class(class: &Expression, spans: &mut Vec<Span>) {
    if !matches!(
        class,
        Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)
    ) {
        spans.push(class.span());
    }
}

/// StaticMethodCallCheck::check() on a class expression (`$x::foo()`), then the parameters check.
pub(in super::super) fn static_call<'arena>(c: &mut Checker<'_, 'arena>, call: &StaticMethodCall<'arena>) {
    let class = call.class;
    if c.level < 2
        || matches!(
            class,
            Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)
        )
    {
        return;
    }
    let ClassLikeMemberSelector::Identifier(name) = &call.method else {
        return;
    };
    let ty = c
        .types
        .get(&class.span())
        .map(|t| c.to_phpstan(t))
        .map(|t| if super::super::assign::contains_nullsafe(class) { t.remove_null() } else { t });
    let found = {
        let c = &*c;
        c.find_type_to_check_of(ty, &|t| {
            c.can_call_methods(t, false).yes() && c.has_method(t, name.value).yes()
        })
    };
    let Some(mut t) = found else {
        return;
    };
    if let Type::ClassString(Some(g)) = &t {
        if !c.can_call_methods(g, false).yes() {
            return;
        }
        t = (**g).clone();
    } else if t.is_string().yes() {
        return;
    }
    let described = describe(c, &t);
    // TypeCombinator::remove($classType, new StringType())
    if let Type::Union(ts) = &t {
        t = pun_phpstan::types::union(ts.iter().filter(|t| !t.is_string().yes()).cloned().collect());
    }
    let method_name = String::from_utf8_lossy(name.value).into_owned();
    if !c.can_call_methods(&t, true).yes() {
        let message = format!("Cannot call static method {method_name}() on {described}.");
        c.report(name, "staticMethod.nonObject", message, None);
        return;
    }
    if !c.has_method(&t, name.value).yes() {
        let message = format!("Call to an undefined static method {described}::{method_name}().");
        c.report(name, "staticMethod.notFound", message, None);
        return;
    }
    // (a method only an unresolved `@mixin` has)
    let method = c.get_method(&t, name.value);
    let Some(method) = method else { return };
    let declaring = method.declaring_display(c);
    let declared = method.name();
    if !method.is_static() {
        let in_instance_method = c
            .functions
            .last()
            .is_some_and(|f| f.kind == super::super::FunctionScopeKind::Method { is_static: false });
        let in_class = c.class_scope().is_some_and(|s| {
            t.object_class_names()
                .iter()
                .all(|n| s.name.as_bytes().eq_ignore_ascii_case(n.as_bytes()) || c.codebase.is_instance_of(s.name.as_bytes(), n.as_bytes()))
        });
        if !in_instance_method || !in_class {
            if declared.eq_ignore_ascii_case("loadhtml") && declaring == "DOMDocument" {
                return;
            }
            let message = format!("Static call to instance method {declaring}::{declared}().");
            c.report(name, "method.staticCall", message, None);
            return;
        }
    }
    let kind = if method.is_static() { "static method" } else { "method" };
    let visibility = method.visibility();
    if !calls::can_call(c, c.class_scope(), &method.declaring_name(), visibility) {
        let v = super::super::restricted(visibility);
        let identifier = if visibility.is_private() { "staticMethod.private" } else { "staticMethod.protected" };
        let message = format!("Call to {v} {kind} {declared}() of class {declaring}.");
        c.report(name, identifier, message, None);
    }
    if c.classes_with(&t, name.value) != 1 {
        return;
    }
    let subject = format!("{declaring}::{declared}()");
    let display_kind = if method.is_static() { "Static method" } else { "Method" };
    let first_level = c.is_top_level(call.span());
    let counts = params::count(&call.argument_list);
    params::check(
        c,
        &params::select(c, method.variants(), &counts),
        &counts,
        name,
        params::Call {
            kind: params::CallKind::StaticMethod,
            subject: &format!("{display_kind} {subject}"),
            void_message: Some(format!("Result of {kind} {subject} (void) is used.")),
        },
        first_level,
    );
}
