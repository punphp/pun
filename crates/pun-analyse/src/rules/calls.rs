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
        named_call(c, id, params::count(&call.argument_list), first_level);
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
        );
    }
}

/// FunctionCallableRule: `f(...)`.
pub fn function_callable<'arena>(
    c: &mut Checker<'_, 'arena>,
    f: &FunctionPartialApplication<'arena>,
) {
    let Expression::Identifier(id) = f.function else {
        return;
    };
    let whole = f.function.span().join(f.argument_list.span());
    if c.piped.contains(&whole)
        || !matches!(
            f.argument_list
                .arguments
                .iter()
                .collect::<Vec<_>>()
                .as_slice(),
            [PartialArgument::VariadicPlaceholder(_)]
        )
    {
        return;
    }
    let fallback = match id {
        Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
        _ => None,
    };
    if std::iter::once(c.names.get(&id.span()))
        .chain(fallback)
        .any(|n| c.function(n).is_some())
    {
        return;
    }
    let name = c.written_or_resolved(id);
    if !c.in_function_exists(&name) {
        c.report(
            id,
            "function.notFound",
            format!("Function {name} not found."),
            None,
        );
    }
}

fn named_call<'arena>(
    c: &mut Checker<'_, 'arena>,
    id: &Identifier<'arena>,
    counts: params::Counts,
    first_level: bool,
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
    let Expression::Variable(Variable::Direct(v)) = call.object else {
        return;
    };
    let ClassLikeMemberSelector::Identifier(name) = &call.method else {
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
    let method_name = String::from_utf8_lossy(name.value).into_owned();
    let Some(method) = members::method(c, class, name.value) else {
        if let Some(method) = members::annotation_method(c, class, name.value) {
            // an annotation method is public: only its arguments are checked
            arguments(c, call, name, &method);
            return;
        }
        // reportMagicMethods is off at level 0: a native __call() answers every name
        let guard =
            super::scope::Guard::Method(format!("$this::{method_name}").to_ascii_lowercase());
        if c.guards.contains(&guard)
            || members::method(c, class, b"__call").is_some()
            || class
                .pseudo_methods
                .iter()
                .any(|m| m.as_bytes().eq_ignore_ascii_case(name.value))
        {
            return;
        }
        let display = c.display(class);
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
    arguments(c, call, name, &method);
}

/// CallMethodsRule's FunctionCallParametersCheck.
fn arguments<'arena>(
    c: &mut Checker<'_, 'arena>,
    call: &MethodCall<'arena>,
    name: &LocalIdentifier<'arena>,
    method: &members::Method,
) {
    let declaring = declaring_display(c, method);
    let subject = format!("{declaring}::{}()", method.name());
    let first_level = c.is_top_level(call.span());
    let counts = params::count(&call.argument_list);
    params::check(
        c,
        &params::select(c, method.variants(), &counts),
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

/// `ClassReflection::getDisplayName()` retains the declaring class's templates within its own scope.
fn declaring_display(c: &Checker, method: &members::Method) -> String {
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

fn written<'arena>(c: &Checker<'_, 'arena>, e: &Expression<'arena>) -> String {
    String::from_utf8_lossy(&c.file.contents[e.span().to_range_usize()]).into_owned()
}

/// CallStaticMethodsRule (StaticMethodCallCheck, then the parameters check).
pub fn static_method_call<'arena>(c: &mut Checker<'_, 'arena>, call: &StaticMethodCall<'arena>) {
    let ClassLikeMemberSelector::Identifier(name) = &call.method else {
        return;
    };
    let method_name = String::from_utf8_lossy(name.value).into_owned();
    let scope_class = c.class_scope();
    let is_static_keyword = matches!(call.class, Expression::Static(_));
    // ClassNameCheck's errors on a written class, dropped where PHPStan returns none (`__callStatic()`)
    let mut pair = None;
    let class = match (call.class, scope_class) {
        (Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_), None) => {
            let identifier = super::out_of_class(call.class);
            let written = written(c, call.class);
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
    let Some(method) = members::method(c, class, name.value)
        .or_else(|| members::annotation_method(c, class, name.value))
    else {
        let guarded = match call.class {
            Expression::Static(_) => "static".to_string(),
            Expression::Self_(_) => "self".to_string(),
            _ => String::from_utf8_lossy(class.name.as_bytes()).into_owned(),
        };
        let key = format!("{guarded}::{method_name}").to_ascii_lowercase();
        let known = c.guards.contains(&super::scope::Guard::Method(key));
        if !known && members::method(c, class, b"__callStatic").is_some() {
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
        return;
    }
    let subject = format!("{}::{declared}()", declaring_display(c, &method));
    let display_kind = if is_static { "Static method" } else { "Method" };
    let first_level = c.is_top_level(call.span());
    let counts = params::count(&call.argument_list);
    let mut variants = method.variants();
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
