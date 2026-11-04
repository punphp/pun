//! Function, method and static calls.

use mago_span::HasSpan;
use mago_syntax::cst::*;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::visibility::Visibility;

use super::class_names::{self, Pair};
use super::members;
use super::params;
use super::reflection::Lookup;
use super::{Checker, DISCOVERING_SYMBOLS_TIP};

/// CallToNonExistentFunctionRule (level 0: no name-case check), then CallToFunctionParametersRule.
pub fn function_call<'arena>(c: &mut Checker<'_, 'arena>, call: &FunctionCall<'arena>) {
    if let Expression::Identifier(id) = call.function {
        let first_level = c.is_top_level(call.span());
        named_call(c, id, params::count(&call.argument_list), first_level, call.span());
    }
}

/// `$x |> f(...)`: PHPStan analyses it as `f($x)`.
pub fn pipe<'arena>(c: &mut Checker<'_, 'arena>, pipe: &Pipe<'arena>) {
    if let Expression::PartialApplication(PartialApplication::Function(f)) = pipe.callable
        && let Expression::Identifier(id) = f.function
        && matches!(
            f.argument_list
                .arguments
                .iter()
                .collect::<Vec<_>>()
                .as_slice(),
            [PartialArgument::VariadicPlaceholder(_)]
        )
    {
        let first_level = c.is_top_level(pipe.span());
        named_call(
            c,
            id,
            params::Counts {
                positional: 1,
                ..params::Counts::default()
            },
            first_level,
            pipe.span(),
        );
    }
}

fn named_call<'arena>(
    c: &mut Checker<'_, 'arena>,
    id: &Identifier<'arena>,
    counts: params::Counts,
    first_level: bool,
    call_span: mago_span::Span,
) {
    let fallback = match id {
        Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
        _ => None,
    };
    let found = std::iter::once(c.names.get(&id.span()))
        .chain(fallback)
        .find_map(|n| c.function(n));
    if let Some((display, variants)) = found {
        let written = c.written(id.span()).trim_start_matches('\\').to_ascii_lowercase();
        let variants = super::selector::select_from_args(c, &written, &counts, variants);
        let all = variants.all.clone();
        let selected = params::select(c, variants, &counts);
        let acceptor = params::closest_variant(&all, &counts, &selected).unwrap_or(selected);
        params::check(
            c,
            &acceptor,
            &counts,
            id,
            params::Call {
                kind: params::CallKind::Function,
                subject: &format!("Function {display}"),
                scope: Some(call_span),
                void_message: Some(format!("Result of function {display} (void) is used.")),
            },
            first_level,
        );
        return;
    }
    let name = c.written_or_resolved(id);
    if c.in_function_exists(&name) {
        return;
    }
    c.report(
        id,
        "function.notFound",
        format!("Function {name} not found."),
        Some(DISCOVERING_SYMBOLS_TIP),
    );
}

/// class_exists(), interface_exists(), trait_exists() or enum_exists() (`isInClassExists()` holds in its arguments).
pub fn is_class_exists_family(c: &Checker, call: &FunctionCall) -> bool {
    let Expression::Identifier(id) = call.function else {
        return false;
    };
    let name = c.written_or_resolved(id).to_ascii_lowercase();
    matches!(
        name.as_str(),
        "class_exists" | "interface_exists" | "trait_exists" | "enum_exists"
    )
}

/// CallMethodsRule with level 0's `checkThisOnly`: calls on `$this` only.
pub fn method_call<'arena>(c: &mut Checker<'_, 'arena>, call: &MethodCall<'arena>) {
    check_this_method(c, call.object, &call.method, Some(call));
}

/// MethodCallCheck shared with MethodCallableRule. A virtual callable supplies no argument check.
pub(super) fn check_this_method<'arena>(
    c: &mut Checker<'_, 'arena>,
    object: &Expression<'arena>,
    selector: &ClassLikeMemberSelector<'arena>,
    call: Option<&MethodCall<'arena>>,
) {
    let Expression::Variable(Variable::Direct(v)) = object else {
        return;
    };
    let ClassLikeMemberSelector::Identifier(name) = selector else {
        return;
    };
    if v.name != b"$this" {
        return;
    }
    let Some(class) = c.class_scope() else {
        return;
    };
    // a call on `never` reports nothing
    if c.this_is_never(class) {
        return;
    }
    // MethodExistsTypeSpecifyingExtension retains an accessory when neither lookup knows the method.
    // A final class cannot acquire that method in a subclass, so its successful branch is `never` instead.
    let refined = c.guards.iter().any(|guard| {
        let super::scope::Guard::Method(key) = guard else {
            return false;
        };
        key.strip_prefix("$this::").is_some_and(|method| {
            c.has_method_guard(key) && members::method(c, class, method.as_bytes()).is_none()
                && members::annotation_method(c, class, method.as_bytes()).is_none()
        })
    });
    if refined && class.flags.is_final() {
        return;
    }
    let method_name = String::from_utf8_lossy(name.value).into_owned();
    let method = if call.is_none() {
        match c.method_of(&String::from_utf8_lossy(class.name.as_bytes()), name.value) {
            Ok(method) => method,
            Err(()) => return,
        }
    } else {
        members::method(c, class, name.value).or_else(|| members::annotation_method(c, class, name.value))
    };
    let Some(method) = method else {
        // reportMagicMethods is off at level 0: a native __call() answers every name. ponytail: with Larastan,
        // whose method extensions are not all ported, a class with __call stays unchecked
        let key = format!("$this::{method_name}");
        if c.has_method_guard(&key)
            || (c.level == 0 || call.is_some() && c.larastan.is_some()) && members::method(c, class, b"__call").is_some()
            || class
                .pseudo_methods
                .iter()
                .any(|m| m.as_bytes().eq_ignore_ascii_case(name.value))
            // the class reflection extensions: a mixin's or a required class's method
            || call.is_some() && !matches!(c.method_of(&String::from_utf8_lossy(class.name.as_bytes()), name.value), Ok(None))
            // `$this instanceof Foo` makes `$this` an intersection with Foo, which has Foo's methods
            || c.guards.iter().any(|g| matches!(g, super::scope::Guard::ThisInstanceOf(n)
                if !matches!(c.method_of(n, name.value), Ok(None))))
        {
            return;
        }
        // ThisType's description keeps the class's template types
        let display = c.describe_object(class);
        // MethodCallCheck unwraps a top-level ThisType, not the ThisType inside an accessory intersection.
        let display = if refined {
            format!("$this({display})")
        } else {
            display
        };
        c.report(
            name,
            "method.notFound",
            format!("Call to an undefined method {display}::{method_name}()."),
            None,
        );
        return;
    };
    let declaring = method.declaring_display(c);
    let visibility = method.visibility();
    if !can_call(c, Some(class), &method.declaring_name(), visibility) {
        let v = super::restricted(visibility);
        c.report(
            name,
            // `$this` is an instance of the scope class, so only a parent's private method is out of reach
            "method.private",
            format!(
                "Call to {v} method {}() of class {declaring}.",
                method.name()
            ),
            None,
        );
    }
    if let Some(call) = call {
        arguments(c, call, name, &method);
    } else {
        method_name_case(c, name, &method, &declaring, false);
    }
}

/// MethodCallCheck/StaticMethodCallCheck's independent `checkFunctionNameCase` switch.
pub(super) fn method_name_case(
    c: &mut Checker,
    name: &LocalIdentifier,
    method: &members::Method,
    declaring: &str,
    static_call: bool,
) {
    let declared = method.name();
    if c.check_function_name_case && declared.as_bytes() != name.value
        && (static_call || declared.as_bytes().eq_ignore_ascii_case(name.value))
    {
        let kind = if static_call && method.is_static() { "static method" } else { "method" };
        let identifier = if static_call { "staticMethod.nameCase" } else { "method.nameCase" };
        let written = String::from_utf8_lossy(name.value);
        c.report(name, identifier, format!("Call to {kind} {declaring}::{declared}() with incorrect case: {written}"), None);
    }
}

/// CallMethodsRule's FunctionCallParametersCheck.
fn arguments<'arena>(
    c: &mut Checker<'_, 'arena>,
    call: &MethodCall<'arena>,
    name: &LocalIdentifier<'arena>,
    method: &members::Method,
) {
    let declaring = declaring_display(c, method, true);
    let subject = format!("{declaring}::{}()", method.name());
    let first_level = c.is_top_level(call.span());
    let counts = params::count(&call.argument_list);
    let mut variants = method.variants();
    with_scope_arguments(c, method, &mut variants);
    params::check(
        c,
        &params::select(c, variants, &counts),
        &counts,
        name,
        params::Call {
            kind: params::CallKind::Method,
            subject: &format!("Method {subject}"),
            scope: Some(call.span()),
            void_message: Some(format!("Result of method {subject} (void) is used.")),
        },
        first_level,
    );
}

/// The declaring class's template names and their arguments on a call through the scope class (`$this->`, `self::`,
/// `static::`, `parent::`): the scope's ClassReflection carries its own templates as TemplateTypeArgumentStrategy
/// arguments (TemplateTypeHelper::toArgument()), resolved through the ancestors (`@extends C<int, T>`).
fn scope_arguments(c: &Checker, method: &members::Method) -> Option<(String, Vec<String>, Vec<pun_phpstan::types::Type>)> {
    use pun_phpstan::types::{Template, Type};
    let (Some(scope), members::Method::User { declaring, .. }) = (c.class_scope(), method) else {
        return None;
    };
    let names: Vec<String> = super::declared::class_templates(c, declaring).into_iter().map(|t| t.name).collect();
    if names.is_empty() {
        return None;
    }
    let own: Vec<Type> = super::declared::class_templates(c, scope)
        .into_iter()
        .map(|t| Type::Template(Box::new(Template { argument: true, ..t })))
        .collect();
    let class = super::declared::original_name(declaring);
    let args = if scope.name.as_bytes().eq_ignore_ascii_case(declaring.name.as_bytes()) {
        own
    } else {
        let name = super::declared::original_name(scope);
        super::declared::with_classes(c, || {
            pun_phpstan::symbols::lookup(&name).and_then(|k| pun_phpstan::types::ancestor_args(&k, &class, &own))
        })?
    };
    Some((format!("class {class}"), names, args))
}

/// The variants' parameter types with [`scope_arguments()`] substituted.
fn with_scope_arguments(c: &Checker, method: &members::Method, variants: &mut params::Variants) {
    let Some((scope, names, args)) = scope_arguments(c, method) else {
        return;
    };
    super::declared::with_classes(c, || {
        for v in &mut variants.all {
            for i in 0..v.names.len() {
                let Some(declared) = super::types::declared_type(c, v, i) else { continue };
                // Template equality ignores the argument strategy, so look for the scope's templates instead
                fn mentions(t: &pun_phpstan::types::Type, scope: &str) -> bool {
                    let found = std::cell::Cell::new(matches!(t, pun_phpstan::types::Type::Template(tt) if tt.scope == scope));
                    t.traverse(&|i| {
                        found.set(found.get() || mentions(i, scope));
                        i.clone()
                    });
                    found.get()
                }
                if mentions(&declared, &scope) {
                    v.param_types[i] = params::ParamType::Typed(super::types::properties::with_args(&declared, &scope, &names, &args));
                }
            }
        }
    });
}

/// `ClassReflection::getDisplayName()` retains the declaring class's templates within its own scope, and a call
/// through the scope class (`through_scope`) the declaring ancestor's [`scope_arguments()`].
fn declaring_display(c: &Checker, method: &members::Method, through_scope: bool) -> String {
    if through_scope && let Some((_, _, args)) = scope_arguments(c, method) {
        let args: Vec<String> = super::declared::with_classes(c, || {
            args.iter().map(|t| t.describe(pun_phpstan::types::Level::TypeOnly)).collect()
        });
        return format!("{}<{}>", method.declaring_display(c), args.join(","));
    }
    if let Some(k) = c.class_scope()
        && !k.template_types.is_empty()
        && k.name
            .as_bytes()
            .eq_ignore_ascii_case(method.declaring_name().as_bytes())
    {
        let args: Vec<String> = super::declared::with_classes(c, || {
            super::declared::class_templates(c, k)
                .into_iter()
                .map(|t| {
                    pun_phpstan::types::Type::Template(Box::new(t))
                        .describe(pun_phpstan::types::Level::TypeOnly)
                })
                .collect()
        });
        return format!("{}<{}>", method.declaring_display(c), args.join(","));
    }
    method.declaring_display(c)
}

/// CallStaticMethodsRule (StaticMethodCallCheck, then the parameters check).
pub fn static_method_call<'arena>(c: &mut Checker<'_, 'arena>, call: &StaticMethodCall<'arena>) {
    check_static_method(c, call.class, &call.method, Some(call));
}

/// StaticMethodCallCheck shared with StaticMethodCallableRule, without synthetic empty arguments.
pub(super) fn check_static_method<'arena>(
    c: &mut Checker<'_, 'arena>,
    class_expr: &Expression<'arena>,
    selector: &ClassLikeMemberSelector<'arena>,
    call: Option<&StaticMethodCall<'arena>>,
) {
    let ClassLikeMemberSelector::Identifier(name) = selector else {
        return;
    };
    let method_name = String::from_utf8_lossy(name.value).into_owned();
    let scope_class = c.class_scope();
    let is_static_keyword = matches!(class_expr, Expression::Static(_));
    // ClassNameCheck's errors on a written class, dropped where PHPStan returns none (`__callStatic()`)
    let mut pair = None;
    let class = match (class_expr, scope_class) {
        (Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_), None) => {
            let identifier = super::out_of_class(class_expr);
            let written = c.written(class_expr.span());
            c.report(
                name,
                identifier,
                format!("Calling {written}::{method_name}() outside of class scope."),
                None,
            );
            return;
        }
        (Expression::Self_(_) | Expression::Static(_), Some(k)) => k,
        (Expression::Parent(_), Some(scope)) => {
            let parent = scope
                .direct_parent_class
                .map(|p| c.class_like(&String::from_utf8_lossy(p.as_bytes())));
            match parent {
                Some(Lookup::Found(p)) => p,
                Some(Lookup::Opaque) => return,
                None | Some(Lookup::NotFound) => {
                    let action = format!(
                        "{}::{}() calls parent::{method_name}()",
                        c.display(scope),
                        c.function_name()
                    );
                    c.report_no_parent(name, scope, &action);
                    return;
                }
            }
        }
        (Expression::Identifier(id), _) => {
            let class_name = c.resolved(id);
            match c.class_like(&class_name) {
                Lookup::Found(k) => {
                    pair = Some(Pair::name(class_name, id.span()));
                    k
                }
                Lookup::Opaque => return,
                Lookup::NotFound if c.in_class_exists(&class_name) => return,
                Lookup::NotFound => {
                    c.report(
                        name,
                        "class.notFound",
                        format!("Call to static method {method_name}() on an unknown class {class_name}."),
                        Some(DISCOVERING_SYMBOLS_TIP),
                    );
                    return;
                }
            }
        }
        _ => return,
    };
    let display = c.display(class);
    // a macro is a static method unless its definition names an instance method (or a callable string)
    if let Some(found) = super::larastan::macro_method(c, class, name.value) {
        let in_instance_method =
            c.functions.last().is_some_and(|f| f.kind == super::FunctionScopeKind::Method { is_static: false });
        let in_subclass = scope_class.is_some_and(|s| {
            s.name.as_bytes().eq_ignore_ascii_case(class.name.as_bytes())
                || c.codebase.is_instance_of(s.name.as_bytes(), class.name.as_bytes())
        });
        if let Some((declaring, declared)) = found.instance
            && (!in_instance_method || !in_subclass)
        {
            c.report(name, "method.staticCall", format!("Static call to instance method {declaring}::{declared}()."), None);
        }
        return;
    }
    let method = if call.is_none() {
        match c.method_of(&String::from_utf8_lossy(class.name.as_bytes()), name.value) {
            Ok(method) => method,
            Err(()) => return,
        }
    } else {
        members::method(c, class, name.value).or_else(|| members::annotation_method(c, class, name.value))
    };
    let Some(method) = method else {
        // FacadesMethodsExtension leaves a facade's method to MixinMethodsClassReflectionExtension: the `@mixin`
        // class's method as a StaticMethodReflection, whose arguments and result are checked
        if let Some(call) = call
            && c.larastan.is_some()
            && let Some(mixin) = super::larastan::facade_mixin_method(c, class, name.value, 0)
        {
            let subject = format!("{}::{}()", mixin.declaring_display(c), mixin.name());
            let counts = params::count(&call.argument_list);
            let first_level = c.is_top_level(call.span());
            params::check(
                c,
                &params::select(c, mixin.variants(), &counts),
                &counts,
                name,
                params::Call {
                    kind: params::CallKind::StaticMethod,
                    subject: &format!("Static method {subject}"),
                    scope: Some(call.span()),
                    void_message: Some(format!("Result of static method {subject} (void) is used.")),
                },
                first_level,
            );
            return;
        }
        let guarded = match class_expr {
            Expression::Static(_) => "static".to_string(),
            Expression::Self_(_) => "self".to_string(),
            _ => String::from_utf8_lossy(class.name.as_bytes()).into_owned(),
        };
        let key = format!("{guarded}::{method_name}").to_ascii_lowercase();
        let known = c.has_method_guard(&key);
        if let Some(call) = call
            && c.larastan.is_some()
            && super::larastan::is_model(c, class)
            && super::types::forwarded_static_call(c, class, name, call)
        {
            return;
        }
        // reportMagicMethods is off at level 0; ponytail: with Larastan a class with __callStatic stays unchecked
        if !known && (c.level == 0 || call.is_some() && c.larastan.is_some())
            && members::method(c, class, b"__callStatic").is_some()
            || call.is_some() && !matches!(c.method_of(&String::from_utf8_lossy(class.name.as_bytes()), name.value), Ok(None))
        {
            return;
        }
        class_names::check_at(c, pair.as_slice(), Some(&class_names::Location::StaticMethodCall(None)), true);
        if known {
            return;
        }
        c.report(
            name,
            "staticMethod.notFound",
            format!("Call to an undefined static method {display}::{method_name}()."),
            None,
        );
        return;
    };
    // ModelForwardsCallsExtension answers a model's non-public `#[Scope]` method the scope cannot call with a static
    // builder method. ponytail: its arguments (the scope's, the builder shifted off) are not checked
    if c.larastan.is_some()
        && super::larastan::is_hidden_scope(c, class, name.value, scope_class)
    {
        return;
    }
    let errors_before = c.errors.len();
    let location = class_names::Location::StaticMethodCall(Some(super::internal::method_member(c, &method)));
    class_names::check_at(c, pair.as_slice(), Some(&location), true);
    let declaring = method.declaring_display(c);
    let declared = method.name();
    let is_static = method.is_static();
    // after "Static call to instance method", PHPStan checks only the arguments
    let mut checked = false;
    if !is_static {
        let in_instance_method = c
            .functions
            .last()
            .is_some_and(|f| f.kind == super::FunctionScopeKind::Method { is_static: false });
        let in_subclass = scope_class.is_some_and(|s| {
            s.name
                .as_bytes()
                .eq_ignore_ascii_case(class.name.as_bytes())
                || c.codebase
                    .is_instance_of(s.name.as_bytes(), class.name.as_bytes())
        });
        if (!in_instance_method || !in_subclass)
            && !(declared.eq_ignore_ascii_case("loadhtml") && declaring == "DOMDocument")
        {
            checked = true;
            c.report(
                name,
                "method.staticCall",
                format!("Static call to instance method {declaring}::{declared}()."),
                None,
            );
        }
    }
    let kind = if is_static { "static method" } else { "method" };
    let visibility = method.visibility();
    if !checked && !can_call(c, scope_class, &method.declaring_name(), visibility) {
        let v = super::restricted(visibility);
        c.report(
            name,
            if visibility.is_private() {
                "staticMethod.private"
            } else {
                "staticMethod.protected"
            },
            format!("Call to {v} {kind} {declared}() of class {declaring}."),
            None,
        );
    }
    if !checked && method.is_abstract() && !is_static_keyword {
        // StaticMethodCallCheck replaces earlier class-name/visibility errors with its abstract-call error.
        if call.is_none() {
            c.errors.truncate(errors_before);
        }
        let s = if is_static { " static" } else { "" };
        c.report(
            name,
            if is_static {
                "staticMethod.callToAbstract"
            } else {
                "method.callToAbstract"
            },
            format!("Cannot call abstract{s} method {declaring}::{declared}()."),
            None,
        );
        checked = true;
    }
    let Some(call) = call else {
        if !checked {
            method_name_case(c, name, &method, &declaring, true);
        }
        return;
    };
    let through_scope = matches!(call.class, Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_));
    let subject = format!("{}::{declared}()", declaring_display(c, &method, through_scope));
    let display_kind = if is_static { "Static method" } else { "Method" };
    let first_level = c.is_top_level(call.span());
    let counts = params::count(&call.argument_list);
    let mut variants = method.variants();
    if through_scope {
        with_scope_arguments(c, &method, &mut variants);
    }
    // CalledOnTypeUnresolvedMethodPrototypeReflection: a call on a class name resolves the parameters' `static` and
    // `$this` to that class
    if matches!(call.class, Expression::Identifier(_)) && matches!(method, members::Method::User { .. }) {
        let receiver = pun_phpstan::types::Type::Object(super::declared::original_name(class), vec![], false);
        super::declared::with_classes(c, || {
            for v in &mut variants.all {
                for i in 0..v.names.len() {
                    let Some(declared) = super::types::declared_type(c, v, i) else { continue };
                    let called = super::method_reflection::called_on(&declared, &receiver);
                    if called != declared {
                        v.param_types[i] = params::ParamType::Typed(called);
                    }
                }
            }
        });
    }
    // ClosureBindArgVisitor: `Closure::bind($var, ...)`
    if let Expression::Identifier(id) = call.class
        && c.written(id.span()).trim_start_matches('\\').eq_ignore_ascii_case("closure")
        && name.value.eq_ignore_ascii_case(b"bind")
        && call.argument_list.arguments.len() > 1
        && let Some(Expression::Variable(Variable::Direct(v))) = call.argument_list.arguments.first().map(|a| a.value())
    {
        let var = String::from_utf8_lossy(&v.name[1..]);
        variants = super::selector::closure_bind(c, &var, call.span().start.offset, 1, variants);
    }
    params::check(
        c,
        &params::select(c, variants, &counts),
        &counts,
        name,
        params::Call {
            kind: params::CallKind::StaticMethod,
            subject: &format!("{display_kind} {subject}"),
            scope: Some(call.span()),
            void_message: Some(format!("Result of {kind} {subject} (void) is used.")),
        },
        first_level,
    );
}

/// `$scope->canCallMethod()`: public anywhere, private from the declaring class, protected from its hierarchy.
pub fn can_call(
    c: &Checker,
    scope: Option<&ClassLikeMetadata>,
    declaring: &str,
    visibility: Visibility,
) -> bool {
    let Some(scope) = scope else {
        return visibility == Visibility::Public;
    };
    match visibility {
        Visibility::Public => true,
        Visibility::Private => scope
            .name
            .as_bytes()
            .eq_ignore_ascii_case(declaring.as_bytes()),
        Visibility::Protected => {
            c.codebase
                .is_instance_of(scope.name.as_bytes(), declaring.as_bytes())
                || c.codebase
                    .is_instance_of(declaring.as_bytes(), scope.name.as_bytes())
        }
    }
}
