//! Function, method and static calls.

use mago_span::HasSpan;
use mago_syntax::cst::*;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::visibility::Visibility;

use super::members;
use super::params;
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
        named_call(c, id, (1, false, false), first_level);
    }
}

fn named_call<'arena>(
    c: &mut Checker<'_, 'arena>,
    id: &Identifier<'arena>,
    counts: (usize, bool, bool),
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
        let acceptor = params::select(variants, counts.0, counts.2);
        params::check(
            c,
            &acceptor,
            counts,
            id,
            params::Call {
                kind: "function",
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
    let method_name = String::from_utf8_lossy(name.value).into_owned();
    let Some(method) = members::method(c.codebase, class, name.value) else {
        // reportMagicMethods is off at level 0: a native __call() answers every name
        if members::method(c.codebase, class, b"__call").is_some()
            || class
                .pseudo_methods
                .iter()
                .any(|m| m.as_bytes().eq_ignore_ascii_case(name.value))
        {
            return;
        }
        let display = String::from_utf8_lossy(class.original_name.as_bytes()).into_owned();
        c.report(
            name,
            "method.notFound",
            format!("Call to an undefined method {display}::{method_name}()."),
            None,
        );
        return;
    };
    let subject = format!("{}::{}()", method.declaring_display(), method.name());
    let first_level = c.is_top_level(call.span());
    let counts = params::count(&call.argument_list);
    params::check(
        c,
        &params::select(method.variants(), counts.0, counts.2),
        counts,
        name,
        params::Call {
            kind: "method",
            subject: &format!("Method {subject}"),
            void_message: Some(format!("Result of method {subject} (void) is used.")),
        },
        first_level,
    );
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
    let in_class = !c.classes.is_empty();
    let scope_class = c.class_scope();
    let is_static_keyword = matches!(call.class, Expression::Static(_));
    let class = match call.class {
        Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_) if !in_class => {
            let identifier = match call.class {
                Expression::Self_(_) => "outOfClass.self",
                Expression::Static(_) => "outOfClass.static",
                _ => "outOfClass.parent",
            };
            let written = written(c, call.class);
            c.report(
                name,
                identifier,
                format!("Calling {written}::{method_name}() outside of class scope."),
                None,
            );
            return;
        }
        Expression::Self_(_) | Expression::Static(_) => match scope_class {
            Some(k) => k,
            None => return, // ponytail: traits and anonymous classes, see Checker::classes
        },
        Expression::Parent(_) => {
            let Some(scope) = scope_class else {
                return;
            };
            let parent = scope
                .direct_parent_class
                .and_then(|p| c.class_like(&String::from_utf8_lossy(p.as_bytes())));
            match parent {
                Some(p) => p,
                None => {
                    let display =
                        String::from_utf8_lossy(scope.original_name.as_bytes()).into_owned();
                    let function = c
                        .functions
                        .last()
                        .map(|f| f.name.clone())
                        .unwrap_or_default();
                    c.report(
                        name,
                        "class.noParent",
                        format!(
                            "{display}::{function}() calls parent::{method_name}() but {display} does not extend any class."
                        ),
                        None,
                    );
                    return;
                }
            }
        }
        Expression::Identifier(id) => {
            let class_name = c.resolved(id);
            match c.class_like(&class_name) {
                Some(k) => k,
                None if c.in_class_exists(&class_name) => return,
                None => {
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
    let display = String::from_utf8_lossy(class.original_name.as_bytes()).into_owned();
    let Some(method) = members::method(c.codebase, class, name.value) else {
        if members::method(c.codebase, class, b"__callStatic").is_some()
            || class
                .static_pseudo_methods
                .iter()
                .any(|m| m.as_bytes().eq_ignore_ascii_case(name.value))
        {
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
    let declaring = method.declaring_display();
    let declared = method.name();
    let is_static = method.is_static();
    if !is_static {
        let function_static = c.functions.last().and_then(|f| f.method_static);
        let in_subclass = scope_class.is_some_and(|s| {
            s.name
                .as_bytes()
                .eq_ignore_ascii_case(class.name.as_bytes())
                || c.codebase
                    .is_instance_of(s.name.as_bytes(), class.name.as_bytes())
        });
        if function_static != Some(false) || !in_subclass {
            if !(declared.eq_ignore_ascii_case("loadhtml") && declaring == "DOMDocument") {
                c.report(
                    name,
                    "method.staticCall",
                    format!("Static call to instance method {declaring}::{declared}()."),
                    None,
                );
            }
        }
    }
    let kind = if is_static { "static method" } else { "method" };
    let visibility = method.visibility();
    if !can_call(c, scope_class, &declaring, visibility) {
        let v = if visibility.is_private() {
            "private"
        } else {
            "protected"
        };
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
    if method.is_abstract() && !is_static_keyword {
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
    let subject = format!("{declaring}::{declared}()");
    let display_kind = if is_static { "Static method" } else { "Method" };
    let first_level = c.is_top_level(call.span());
    let counts = params::count(&call.argument_list);
    params::check(
        c,
        &params::select(method.variants(), counts.0, counts.2),
        counts,
        name,
        params::Call {
            kind: "staticMethod",
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
