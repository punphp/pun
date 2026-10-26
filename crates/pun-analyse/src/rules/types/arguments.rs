//! FunctionCallParametersCheck's checks of level 5 (`checkFunctionArgumentTypes`, `checkArgumentsPassedByReference`):
//! each argument's type against its parameter's (`argument.type`), by-reference parameters given no variable
//! (`argument.byRef`) and unpacked non-iterables (`argument.unpackNonIterable`).

use mago_span::Span;
use pun_phpstan::symbols::ByRef;
use pun_phpstan::types::{
    Conditional, NeverKind, Subject, Template, TemplateMap, Type, infer_template_types, mixed, recommended_level,
    resolve_template_types, union,
};

use super::super::params::{Acceptor, ArgKind, Call, CallKind, Entry, ParamType};
use super::super::{Checker, declared};
use super::returns::strict_types;

pub const IDENTIFIERS: &[&str] =
    &["argument.type", "argument.byRef", "argument.unpackNonIterable", "argument.templateType", "argument.named",
      "argument.invalidConstant", "argument.exclusiveConstants", "argument.bitmaskNotAllowed",
      "argument.staticClosure", "argument.unresolvableType"];

/// `describeParameter()`: `Parameter #1 $a`, `Parameter $a` (named), `Named argument b for variadic parameter ...$a`.
fn describe(acceptor: &Acceptor, p: usize, position_or_name: Option<Result<usize, &str>>) -> String {
    let variadic = acceptor.variadics[p];
    let head = match position_or_name {
        Some(Ok(i)) => format!("Parameter #{}", i + 1),
        Some(Err(name)) if variadic => format!("Named argument {name} for variadic parameter"),
        _ => "Parameter".to_string(),
    };
    let name = &acceptor.names[p];
    if name.is_empty() {
        return head;
    }
    let dots = if variadic { "..." } else { "" };
    format!("{head} {dots}${name}")
}

/// The parameter's declared type, `None` without one (an implicit `mixed`) or for variants combined with types Pun
/// does not join.
pub(in super::super) fn declared_type(c: &Checker, acceptor: &Acceptor, p: usize) -> Option<Type> {
    match &acceptor.param_types[p] {
        ParamType::Builtin(s, _) => Some(pun_phpstan::resolve::resolve_str(s, acceptor.class.as_deref())),
        ParamType::Typed(t) => Some(t.clone()),
        ParamType::User => {
            let (f, declaring) = acceptor.user(c)?;
            // ponytail: computed per argument, not cached per callee
            declared::reflected_params(c, f, declaring, &acceptor.callee.as_ref()?.reflected_name).into_iter().nth(p)
        }
    }
}

/// `getReturnType()` retains native and implicit magic return types when deciding the PHPDoc type.
fn return_type(c: &Checker, acceptor: &Acceptor) -> Option<Type> {
    let (f, declaring) = acceptor.user(c)?;
    Some(declared::codex_return_type(c, f, declaring, &acceptor.callee.as_ref()?.reflected_name))
}

/// A type only the call resolves: a template type (ResolvedFunctionVariant), `static`/`$this` or a conditional type.
pub(in super::super) fn is_late(t: &Type) -> bool {
    let late =
        std::cell::Cell::new(matches!(t, Type::Template(_) | Type::Static(..) | Type::This(_) | Type::Conditional(_)));
    t.traverse(&|i| {
        if matches!(i, Type::Template(_) | Type::Static(..) | Type::This(_) | Type::Conditional(_)) {
            late.set(true);
        }
        i.clone()
    });
    late.get()
}

/// The parameter's type for the check, `None` when it is not checked.
// ponytail: the types resolved per call in PHPStan (see [`is_late()`]) other than the callee's own templates are skipped
fn parameter_type(resolved: &[Option<Type>], p: usize) -> Option<Type> {
    resolved[p].clone().filter(|t| !is_late(t) && *t != mixed())
}

/// `GenericParametersAcceptorResolver::resolve()`: the declared parameter types, the callee's templates in them, the
/// type passed to each parameter and the templates inferred from those.
// ponytail: no default values or unpassed variadics take part, nor callables' type predicates
struct Inference {
    declared: Vec<Option<Type>>,
    templates: Vec<Template>,
    passed: Vec<Option<Type>>,
    map: TemplateMap,
}

fn infer(c: &Checker, acceptor: &Acceptor, args: &[Entry], parameters: &[Option<usize>]) -> Inference {
    let declared: Vec<Option<Type>> = (0..acceptor.names.len()).map(|p| declared_type(c, acceptor, p)).collect();
    let templates = std::cell::RefCell::new(Vec::new());
    fn collect(t: &Type, out: &std::cell::RefCell<Vec<Template>>) {
        if let Type::Template(tt) = t
            && is_callees(tt)
            && !out.borrow().contains(&**tt)
        {
            out.borrow_mut().push((**tt).clone());
        }
        t.traverse(&|i| {
            collect(i, out);
            i.clone()
        });
    }
    declared.iter().flatten().for_each(|t| collect(t, &templates));
    let templates = templates.into_inner();
    let mut passed: Vec<Option<Type>> = vec![None; declared.len()];
    for (e, p) in args.iter().zip(parameters) {
        if let Some(p) = *p
            && let Some(t) = e.ty.clone().or_else(|| c.type_at(e.value))
        {
            let t = if e.unpack { t.iterable_value_type() } else { t };
            passed[p] = Some(passed[p].take().map_or(t.clone(), |o| union(vec![o, t])));
        }
    }
    // an unpassed parameter takes its default value, a variadic one an explicit `never`
    let callee = acceptor.user(c);
    for (p, passed) in passed.iter_mut().enumerate().filter(|(_, t)| t.is_none()) {
        let default = callee.and_then(|(f, _)| f.parameters.get(p)?.default_type.as_ref());
        *passed = match default {
            Some(d) => Some(c.to_phpstan(&d.type_union)),
            None if acceptor.variadics[p] => Some(Type::Never(NeverKind::Explicit)),
            None => None,
        };
    }
    let map = if templates.is_empty() {
        TemplateMap::default()
    } else {
        let map = declared.iter().zip(&passed).fold(TemplateMap::default(), |map, (d, a)| match (d, a) {
            (Some(d), Some(a)) => map.union(declared::with_classes(c, || infer_template_types(d, a))),
            _ => map,
        });
        // a return type conditional on a passed parameter infers from its target too
        match return_type(c, acceptor) {
            Some(Type::Conditional(cond))
                if !cond.negated
                    && let Subject::Parameter(name) = &cond.subject
                    && let Some(Some(a)) = acceptor.names.iter().position(|n| name[1..] == **n).map(|i| &passed[i]) =>
            {
                map.union(declared::with_classes(c, || infer_template_types(&cond.target, a)))
            }
            _ => map,
        }
    };
    let map = map.with_errors(&templates.iter().collect::<Vec<_>>());
    Inference { declared, templates, passed, map }
}

/// A template of the called function-like itself (its class's templates stay unresolved).
pub(super) fn is_callees(t: &Template) -> bool {
    t.scope.starts_with("function ") || t.scope.starts_with("method ")
}

/// The parameters' types with the callee's templates resolved (an uninferred one by its default or bound).
fn resolved_types(acceptor: &Acceptor, inference: &Inference) -> Vec<Option<Type>> {
    let Inference { declared, passed, map, .. } = inference;
    let argument = |name: &str| passed[acceptor.names.iter().position(|n| name[1..] == **n)?].clone();
    declared.iter().map(|d| d.as_ref().map(|d| late_parameter_type(&resolve_template_types(d, map), &argument))).collect()
}

/// `TypeUtils::resolveLateResolvableTypes()` after template substitution. The resolver retains a template's
/// key-of/value-of as an object-shaped placeholder; use its existing type operations once that template is bound.
fn late_parameter_type(t: &Type, argument: &dyn Fn(&str) -> Option<Type>) -> Type {
    let t = if let Type::Conditional(cond) = t
        && let Subject::Parameter(name) = &cond.subject
        && let Some(value) = argument(name)
    {
        Type::Conditional(Box::new(Conditional { subject: Subject::Type(value), ..(**cond).clone() }))
    } else {
        t.clone()
    };
    let t = t.traverse(&|t| late_parameter_type(t, argument));
    let t = match &t {
        Type::Object(name, args) if matches!(name.as_str(), "key-of" | "value-of")
            && args.len() == 1 && !is_late(&args[0]) => {
                pun_phpstan::resolve::key_or_value_of(&args[0], name == "value-of")
            }
        Type::Conditional(cond) if cond.is_resolvable() => return late_parameter_type(&cond.resolve(), argument),
        _ => t,
    };
    pun_phpstan::resolve::resolve_late(&t)
}

/// `UnresolvableTypeHelper`: direct siblings still contribute reasons after the first error, but later containers
/// are not descended into. Keep the distinction between no error and an error without an explanation.
fn unresolvable(t: &Type) -> Option<Vec<String>> {
    fn visit(t: &Type, found: &std::cell::Cell<bool>, reasons: &std::cell::RefCell<Vec<String>>) {
        if t == &Type::Error || t == &Type::Never(NeverKind::Implicit) {
            found.set(true);
            if let Some(reason) = t.unresolvable_reason()
                && !reasons.borrow().iter().any(|r| r == reason)
            {
                reasons.borrow_mut().push(reason.to_string());
            }
        }
        if !found.get() {
            t.traverse(&|t| {
                visit(t, found, reasons);
                t.clone()
            });
        }
    }
    let found = std::cell::Cell::new(false);
    let reasons = std::cell::RefCell::new(Vec::new());
    visit(t, &found, &reasons);
    found.get().then(|| reasons.into_inner())
}

/// PropertyReflectionFinder's public/readable instance-property facts for the actual argument object types.
fn property_reflections(c: &Checker, inference: &Inference) -> Vec<(String, pun_phpstan::types::Tri, Vec<pun_phpstan::types::PropertyReflection>)> {
    let mut names = Vec::new();
    for t in inference.passed.iter().flatten() {
        super::super::type_aliases::each(t, &mut |t| {
            for n in t.object_class_names() {
                if !names.contains(&n) { names.push(n); }
            }
        });
    }
    names.into_iter().filter_map(|name| {
        let k = c.codebase.get_class_like(name.as_bytes())?;
        // UniversalObjectCratesClassReflectionExtension answers maybe even for a declared property.
        if c.codebase.is_instance_of(k.name.as_bytes(), b"stdClass") { return None; }
        let missing = if k.flags.is_final() && !k.flags.allows_dynamic_properties() {
            pun_phpstan::types::Tri::No
        } else {
            pun_phpstan::types::Tri::Maybe
        };
        let properties = k.declaring_property_ids.iter().filter_map(|(name, declaring)| {
            let p = c.codebase.get_property(declaring.as_bytes(), name.as_bytes())?;
            if p.flags.is_static() { return None; }
            let declaring = c.codebase.get_class_like(declaring.as_bytes())?;
            Some(pun_phpstan::types::PropertyReflection {
                name: String::from_utf8_lossy(name.as_bytes()).trim_start_matches('$').to_string(),
                declaring_class: declared::original_name(declaring),
                public: p.read_visibility.is_public(),
                readable: !p.flags.is_virtual_property() || p.hooks.contains_key(&mago_word::word("get")),
                ty: p.type_metadata.as_ref().map_or_else(mixed, |t| c.to_phpstan(&t.type_union)),
            })
        }).collect();
        Some((name, missing, properties))
    }).collect()
}

/// The `argument.templateType` errors of `checkMissingTypehints` (level 6): a template the call leaves unresolved
/// (not absorbed by a union member) that both a parameter and the return type use.
fn unresolvable_templates(c: &Checker, acceptor: &Acceptor, inference: &Inference) -> Vec<String> {
    let Inference { templates, passed, map, .. } = inference;
    let Some(ret) = return_type(c, acceptor) else {
        return vec![];
    };
    let resolved_to = |t: &Template| {
        map.types().into_iter().find(|(n, s, _)| *n == t.name && *s == t.scope).map(|(_, _, t)| t.clone())
    };
    // `getReturnTypeWithUnresolvableTemplateTypes()`: the resolved templates replaced, the parameters' conditional
    // types made conditional on the arguments
    fn resolve(t: &Type, f: &dyn Fn(&Type) -> Option<Type>) -> Type {
        f(t).unwrap_or_else(|| t.clone()).traverse(&|i| resolve(i, f))
    }
    let names = &acceptor.names;
    let ret = declared::with_classes(c, || {
        let ret = resolve(&ret, &|t| match t {
            Type::Template(tt) if is_callees(tt) => resolved_to(tt).filter(|r| *r != Type::Error),
            _ => None,
        });
        resolve(&ret, &|t| match t {
            Type::Conditional(cond) if let Subject::Parameter(p) = &cond.subject => {
                let subject = passed[names.iter().position(|n| p[1..] == **n)?].clone()?;
                Some(Type::Conditional(Box::new(Conditional { subject: Subject::Type(subject), ..(**cond).clone() })))
            }
            _ => None,
        })
    });
    let in_return = std::cell::RefCell::new(Vec::new());
    fn collect(t: &Type, out: &std::cell::RefCell<Vec<String>>) {
        let mut t = t.clone();
        while let Type::Conditional(cond) = &t
            && cond.is_resolvable()
        {
            t = cond.resolve();
        }
        if let Type::Template(tt) = &t
            && tt.default.is_none()
        {
            out.borrow_mut().push(tt.name.clone());
            return;
        }
        t.traverse(&|i| {
            collect(i, out);
            i.clone()
        });
    }
    declared::with_classes(c, || collect(&ret, &in_return));
    let in_return = in_return.into_inner();
    templates
        .iter()
        .filter(|t| t.default.is_none() && in_return.contains(&t.name))
        // (an implicit `never` is resolved, so it is in no return type)
        .filter(|t| resolved_to(t) == Some(Type::Error) && !map.is_absorbed(&t.name, &t.scope))
        .map(|t| t.name.clone())
        .collect()
}

/// `$methodReflection->returnsByReference()->yes()` of method `name` of class `class`.
fn returns_by_ref(c: &Checker, class: Option<&str>, name: &[u8]) -> bool {
    class.and_then(|n| c.class_like(n).found()).is_some_and(|k| {
        matches!(super::super::members::method(c, k, name), Some(super::super::members::Method::User { f, .. }) if f.flags.is_by_reference())
    })
}

impl Checker<'_, '_> {
    fn type_at(&self, span: Span) -> Option<Type> {
        self.types.get(&span).map(|t| self.to_phpstan(t))
    }
}

/// The rest of `check()` after `processArguments()`, `parameters` being each argument's parameter.
pub(in super::super) fn arguments(
    c: &mut Checker,
    acceptor: &Acceptor,
    args: &[Entry],
    parameters: &[Option<usize>],
    at: Span,
    call: &Call,
) {
    // `%s of function foo expects ...`: the subject with a lowercase first letter
    let of = call.subject[..1].to_ascii_lowercase() + &call.subject[1..];
    let strict = strict_types(c);
    let levels = c.levels();
    let inference = infer(c, acceptor, args, parameters);
    let resolved = declared::with_classes(c, || pun_phpstan::types::with_property_reflections(
        property_reflections(c, &inference), || resolved_types(acceptor, &inference)
    ));
    let allowed_type = super::super::params::allowed_constants_type(c, acceptor);
    for (i, (e, p)) in args.iter().zip(parameters).enumerate() {
        if e.unpack {
            let ty = c.type_at(e.value);
            let found = c.find_type_to_check_of(ty, &|t| t.is_iterable().yes());
            if let Some(t) = found
                && !t.is_iterable().yes()
            {
                let described = declared::with_classes(c, || t.describe(pun_phpstan::types::Level::TypeOnly));
                let message = format!("Only iterables can be unpacked, {described} given in argument #{}.", i + 1);
                c.report(e.span, "argument.unpackNonIterable", message, None);
            }
        }
        let Some(p) = *p else {
            continue;
        };
        super::super::params::check_named(c, acceptor, e, call);
        let by_ref = acceptor.by_ref[p];
        let value = e.ty.clone().or_else(|| c.type_at(e.value)).map(|t| if e.unpack { t.iterable_value_type() } else { t });
        // a built-in's by-reference parameter that creates the variable takes anything
        if (by_ref != ByRef::CreatesNewVariable || !acceptor.builtin)
            && let (Some(param), Some(value)) = (parameter_type(&resolved, p), value)
        {
            let strict = strict && (!acceptor.builtin || !param.is_callable().yes());
            let message = declared::with_classes(c, || {
                (!levels.accepts(&param, &value, strict)).then(|| {
                    let level = recommended_level(&param, Some(&value));
                    let message = format!(
                        "{} of {of} expects {}, {} given.",
                        describe(acceptor, p, Some(e.name.map_or(Ok(i), Err))),
                        param.describe(level),
                        value.describe(level)
                    );
                    (message, levels.accepts_tip(&param, &value, strict))
                })
            });
            if let Some((message, tip)) = message {
                c.report(e.span, "argument.type", message, tip.as_deref());
            }
        }
        let parameter = || describe(acceptor, p, e.name.is_none().then_some(Ok(i)));
        if let Some(reasons) = resolved[p].as_ref().and_then(unresolvable)
            && inference.declared[p].as_ref().is_some_and(|t| unresolvable(t).is_none())
        {
            // RuleErrorBuilder::addTip()/build(), as rendered by PHPStan's plain JSON formatter.
            let reasons: Vec<_> = reasons.into_iter().map(|r| r.replace("<fg=cyan>", "").replace("</>", "")).collect();
            let tip = match reasons.as_slice() {
                [] => None,
                [one] => Some(one.clone()),
                all => Some(all.iter().map(|r| format!("• {r}")).collect::<Vec<_>>().join("\n")),
            };
            c.report(e.span, "argument.unresolvableType", format!(
                "{} of {of} contains unresolvable type.", parameter()
            ), tip.as_deref());
        }
        if acceptor.closure_this[p] && matches!(e.kind, ArgKind::StaticClosure) {
            c.report(e.span, "argument.staticClosure", format!(
                "{} of {of} expects bindable closure, static closure given.", parameter()
            ), None);
        }
        super::super::params::check_constants(
            c, acceptor, e, p, allowed_type.as_ref(), resolved[p].as_ref(),
            &describe(acceptor, p, Some(e.name.map_or(Ok(i), Err))), call,
        );
        if by_ref == ByRef::No {
            continue;
        }
        let fine = match e.kind {
            ArgKind::Nullsafe | ArgKind::Other | ArgKind::Constants(_) | ArgKind::StaticClosure => false,
            ArgKind::Writable | ArgKind::OtherCall => true,
            ArgKind::Function(id) => {
                c.codebase.get_function(c.names.get(id)).is_some_and(|f| f.flags.is_by_reference())
            }
            // callReturnsByReference(): a method PHPStan does not find returns no reference
            ArgKind::MethodCall(object, Some(name)) => {
                let class = if c.written(*object) == "$this" {
                    c.class_scope().map(|k| String::from_utf8_lossy(k.name.as_bytes()).into_owned())
                } else {
                    c.type_at(*object).and_then(|t| t.object_class_names().into_iter().next())
                };
                returns_by_ref(c, class.as_deref(), name)
            }
            ArgKind::StaticCall(class, Some(name)) => {
                let class = match class {
                    Some(id) => Some(String::from_utf8_lossy(c.names.get(id)).into_owned()),
                    None => c.class_scope().map(|k| String::from_utf8_lossy(k.name.as_bytes()).into_owned()),
                };
                returns_by_ref(c, class.as_deref(), name)
            }
            ArgKind::MethodCall(_, None) | ArgKind::StaticCall(_, None) => false,
        };
        if fine {
            continue;
        }
        let parameter = describe(acceptor, p, e.name.is_none().then_some(Ok(i)));
        let message = match call.kind {
            CallKind::New => {
                format!(" {parameter} of {of} is passed by reference, so it expects variables only")
            }
            CallKind::Attribute => {
                format!("{parameter} of {of} is passed by reference, so it expects variables only")
            }
            _ => {
                format!("{parameter} of {of} is passed by reference, so it expects variables only.")
            }
        };
        c.report(e.span, "argument.byRef", message, None);
    }
    if c.level >= 6 {
        let site = match call.kind {
            CallKind::New | CallKind::Attribute => {
                format!("instantiation of {}", of.trim_end_matches(" constructor"))
            }
            _ => format!("call to {of}"),
        };
        for name in unresolvable_templates(c, acceptor, &inference) {
            let message = format!("Unable to resolve the template type {name} in {site}");
            let tip = "See: https://phpstan.org/blog/solving-phpstan-error-unable-to-resolve-template-type";
            c.report(at, "argument.templateType", message, Some(tip));
        }
    }
}
