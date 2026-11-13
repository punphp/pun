//! PHPStan 2.3.5's bleeding-edge `featureToggles.unresolvedTemplateArguments` (src/Analyser/Generics).
//!
//! Inside a function-like body, a generic call or `new` leaves each template argument of its result open: the
//! observation pass hands out a marker keyed by the call site (UnresolvedTemplateArgumentType), records what the body
//! sends the object to and passes into it (TemplateArgumentObserver), and the solver decides every marker
//! (TemplateArgumentSolver). Pun walks a file twice, as StatementsHandler walks a body twice: the second pass reads
//! the resolutions instead of handing out markers.
//!
//! A marker is a [`TGenericParameter`] whose defining entity names the site, so it flows through variables, unions
//! and method calls like the template it stands for; its constraint is its delegate, the type it behaves as.

use std::collections::HashMap;
use std::sync::Arc;

use indexmap::IndexMap;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::misc::GenericParent;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::generic::TGenericParameter;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator;
use mago_codex::ttype::get_array_parameters;
use mago_codex::ttype::get_never;
use mago_codex::ttype::get_specialized_template_type;
use mago_codex::ttype::template::variance::Variance;
use mago_codex::ttype::union::TUnion;
use mago_span::Span;
use mago_word::Word;
use mago_word::word;

/// The defining entity's first part of every marker.
const SITE: &str = "\0site";

/// What the analyser does with the template arguments of generic calls and `new`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TemplateArgumentMode {
    /// PHPStan without the toggle: an inferred argument is generalized (`new Foo(1)` is `Foo<int>`).
    #[default]
    Off,
    /// The observation pass: markers are handed out and observed.
    Observe,
    /// The second pass: the observation pass's resolutions, an exact inferred argument otherwise.
    Resolve(Arc<Resolutions>),
}

impl TemplateArgumentMode {
    #[must_use]
    pub fn is_on(&self) -> bool {
        !matches!(self, TemplateArgumentMode::Off)
    }
}

/// A template argument of a site: the call or `new` expression's span and the template's name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SiteKey {
    pub start: u32,
    pub end: u32,
    pub template: Word,
}

impl SiteKey {
    #[must_use]
    pub fn new(site: Span, template: Word) -> Self {
        SiteKey { start: site.start.offset, end: site.end.offset, template }
    }
}

pub type Resolutions = HashMap<SiteKey, TUnion>;

/// A marker as its parts.
#[derive(Clone, Debug)]
pub struct Marker {
    pub key: SiteKey,
    /// What the call's arguments inferred, `None` when nothing was.
    pub initial: Option<TUnion>,
    /// The template is declared covariant.
    pub covariant: bool,
    /// The type the marker behaves as: the initial type, else the template's default or bound.
    pub delegate: TUnion,
}

/// A marker for the template argument `template` of `site`.
#[must_use]
pub fn marker(site: Span, template: Word, initial: Option<TUnion>, unconstrained: TUnion, covariant: bool) -> TAtomic {
    let flags = format!(
        "{}:{}:{}{}",
        site.start.offset,
        site.end.offset,
        if initial.is_some() { 'i' } else { 'n' },
        if covariant { 'c' } else { 'x' }
    );
    TAtomic::GenericParameter(TGenericParameter {
        parameter_name: template,
        constraint: Arc::new(initial.unwrap_or(unconstrained)),
        defining_entity: GenericParent::FunctionLike((word(SITE), word(&flags))),
        intersection_types: None,
    })
}

/// The marker an atomic is, if it is one.
#[must_use]
pub fn as_marker(atomic: &TAtomic) -> Option<Marker> {
    let TAtomic::GenericParameter(parameter) = atomic else {
        return None;
    };
    let GenericParent::FunctionLike((tag, flags)) = parameter.defining_entity else {
        return None;
    };
    if tag.as_bytes() != SITE.as_bytes() {
        return None;
    }
    let flags = std::str::from_utf8(flags.as_bytes()).ok()?;
    let mut parts = flags.split(':');
    let start = parts.next()?.parse().ok()?;
    let end = parts.next()?.parse().ok()?;
    let rest = parts.next()?.as_bytes();
    let delegate = (*parameter.constraint).clone();
    Some(Marker {
        key: SiteKey { start, end, template: parameter.parameter_name },
        initial: (rest.first() == Some(&b'i')).then(|| delegate.clone()),
        covariant: rest.get(1) == Some(&b'c'),
        delegate,
    })
}

fn single_marker(t: &TUnion) -> Option<Marker> {
    match t.types.as_ref() {
        [atomic] => as_marker(atomic),
        _ => None,
    }
}

/// Whether a marker occurs anywhere in the type.
#[must_use]
pub fn contains_marker(t: &TUnion) -> bool {
    t.types.iter().any(atomic_contains_marker)
}

fn atomic_contains_marker(atomic: &TAtomic) -> bool {
    if as_marker(atomic).is_some() {
        return true;
    }
    let mut found = false;
    visit_children(atomic, &mut |child| found = found || contains_marker(child));
    found
}

/// Calls `f` with each type directly nested in the atomic.
fn visit_children(atomic: &TAtomic, f: &mut impl FnMut(&TUnion)) {
    match atomic {
        TAtomic::Object(TObject::Named(named)) => {
            for parameter in named.type_parameters.iter().flat_map(|p| p.iter()) {
                f(parameter);
            }
        }
        TAtomic::Array(TArray::List(list)) => {
            f(&list.element_type);
            for (_, (_, element)) in list.known_elements.iter().flat_map(|k| k.iter()) {
                f(element);
            }
        }
        TAtomic::Array(TArray::Keyed(keyed)) => {
            if let Some((key, value)) = &keyed.parameters {
                f(key);
                f(value);
            }
            for (_, (_, item)) in keyed.known_items.iter().flat_map(|k| k.iter()) {
                f(item);
            }
        }
        TAtomic::Iterable(iterable) => {
            f(&iterable.key_type);
            f(&iterable.value_type);
        }
        TAtomic::Callable(TCallable::Signature(signature)) => {
            for parameter in signature.get_parameters() {
                if let Some(t) = parameter.get_type_signature() {
                    f(t);
                }
            }
            if let Some(t) = signature.get_return_type() {
                f(t);
            }
        }
        _ => {}
    }
}

/// The type with every marker replaced by `replace`'s answer for it (itself when `None`).
#[must_use]
pub fn map_markers(t: &TUnion, codebase: &CodebaseMetadata, replace: &mut impl FnMut(&Marker) -> Option<TUnion>) -> TUnion {
    map_markers_in(t, codebase, true, replace)
}

/// [`map_markers()`], leaving the arguments of objects alone unless `into_objects`.
fn map_markers_in(
    t: &TUnion,
    codebase: &CodebaseMetadata,
    into_objects: bool,
    replace: &mut impl FnMut(&Marker) -> Option<TUnion>,
) -> TUnion {
    if !contains_marker(t) {
        return t.clone();
    }
    let mut types = Vec::with_capacity(t.types.len());
    for atomic in t.types.iter() {
        if let Some(m) = as_marker(atomic) {
            match replace(&m) {
                Some(replacement) => types.extend(replacement.types.iter().cloned()),
                None => types.push(atomic.clone()),
            }
            continue;
        }
        if !into_objects && matches!(atomic, TAtomic::Object(_)) {
            types.push(atomic.clone());
            continue;
        }
        types.push(map_atomic(atomic, codebase, into_objects, replace));
    }
    if types.is_empty() {
        return get_never();
    }
    let combined = mago_codex::ttype::combiner::combine(types, codebase, CombinerOptions::default());
    t.clone_with_types(combined)
}

fn map_atomic(
    atomic: &TAtomic,
    codebase: &CodebaseMetadata,
    into_objects: bool,
    replace: &mut impl FnMut(&Marker) -> Option<TUnion>,
) -> TAtomic {
    if !atomic_contains_marker(atomic) {
        return atomic.clone();
    }
    let mut map = |t: &TUnion| map_markers_in(t, codebase, into_objects, replace);
    let mut atomic = atomic.clone();
    match &mut atomic {
        TAtomic::Object(TObject::Named(named)) => {
            if let Some(parameters) = named.get_type_parameters_mut() {
                for parameter in parameters {
                    *parameter = map(parameter);
                }
            }
        }
        TAtomic::Array(TArray::List(list)) => {
            *Arc::make_mut(&mut list.element_type) = map(&list.element_type);
            if let Some(known) = list.known_elements.as_mut().map(Arc::make_mut) {
                for (_, element) in known.values_mut() {
                    *element = map(element);
                }
            }
        }
        TAtomic::Array(TArray::Keyed(keyed)) => {
            if let Some((key, value)) = &mut keyed.parameters {
                *Arc::make_mut(key) = map(key);
                *Arc::make_mut(value) = map(value);
            }
            if let Some(known) = keyed.known_items.as_mut().map(Arc::make_mut) {
                for (_, item) in known.values_mut() {
                    *item = map(item);
                }
            }
        }
        TAtomic::Iterable(iterable) => {
            let key = map(iterable.get_key_type());
            *iterable.get_key_type_mut() = key;
            let value = map(iterable.get_value_type());
            *iterable.get_value_type_mut() = value;
        }
        TAtomic::Callable(TCallable::Signature(signature)) => {
            for parameter in signature.get_parameters_mut() {
                if let Some(t) = parameter.get_type_signature_mut() {
                    *t = map(t);
                }
            }
            if let Some(t) = signature.get_return_type_mut() {
                *t = map(t);
            }
        }
        _ => {}
    }
    atomic
}

/// `UnresolvedTemplateArgumentType::unwrapBare()`: the markers outside an object's arguments as their delegates. A
/// value read out of an object is derived; it never constrains the site.
#[must_use]
pub fn unwrap_bare(t: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
    map_markers_in(t, codebase, false, &mut |m| Some(unwrap_bare(&m.delegate, codebase)))
}

/// `ClosureSignatureInference` (bleeding-edge `closureSignaturesFromUsages`): a closure or arrow function written
/// where nothing types it carries a marker per parameter in the observation pass, named after the parameter and
/// sited at the closure, with the declared type as its delegate. Invocations put their arguments, and the callable
/// types the closure is sent to their parameter types, as lower bounds; the second pass walks the body with the
/// resolved types (intersected with the declared ones, as for a closure passed to a callable parameter).
#[must_use]
pub fn closure_signature_markers(closure: TUnion, site: Span, observed: &mut Observed) -> TUnion {
    let mut closure = closure;
    for atomic in closure.types.to_mut() {
        let TAtomic::Callable(TCallable::Signature(signature)) = atomic else {
            continue;
        };
        for parameter in signature.parameters.iter_mut() {
            // a variadic parameter collects arguments of different positions into one list
            if parameter.is_by_reference() || parameter.is_variadic() {
                continue;
            }
            let Some(name) = parameter.get_name() else {
                continue;
            };
            let declared = parameter.get_type_signature().cloned().unwrap_or_else(mago_codex::ttype::get_mixed);
            let marker = marker(site, name.0, None, declared.clone(), false);
            if let Some(m) = as_marker(&marker) {
                observed.site(&m, declared);
            }
            *parameter = parameter.clone().with_type_signature(Some(Arc::new(TUnion::from_atomic(marker))));
        }
    }
    closure
}

/// The second pass's parameter types of a closure written where nothing types it, by position.
#[must_use]
pub fn closure_signature_parameters(
    resolutions: &Resolutions,
    site: Span,
    names: impl Iterator<Item = Word>,
) -> Option<HashMap<usize, TUnion>> {
    let resolved: HashMap<usize, TUnion> = names
        .enumerate()
        .filter_map(|(i, name)| Some((i, resolutions.get(&SiteKey::new(site, name))?.clone())))
        .collect();
    (!resolved.is_empty()).then_some(resolved)
}

/// The second pass's type of a closure written where nothing types it: each parameter the observation pass resolved
/// has the resolved type, intersected with its declared type as the body sees it.
#[must_use]
pub fn closure_signature_resolved(
    closure: TUnion,
    resolved: &HashMap<usize, TUnion>,
    codebase: &CodebaseMetadata,
) -> TUnion {
    let mut closure = closure;
    for atomic in closure.types.to_mut() {
        let TAtomic::Callable(TCallable::Signature(signature)) = atomic else {
            continue;
        };
        for (i, parameter) in signature.parameters.iter_mut().enumerate() {
            let Some(resolution) = resolved.get(&i) else {
                continue;
            };
            let narrower = parameter
                .get_type_signature()
                .is_none_or(|declared| declared.is_mixed() || is_super_type(codebase, declared, resolution));
            if narrower {
                *parameter = parameter.clone().with_type_signature(Some(Arc::new(resolution.clone())));
            }
        }
    }
    closure
}

/// A marker of a closure's parameter ([`closure_signature_markers()`]).
fn is_closure_marker(m: &Marker) -> bool {
    m.key.template.as_bytes().first() == Some(&b'$')
}

/// What the observation pass learnt about one template argument of a site.
#[derive(Clone, Debug)]
struct Observation {
    marker: Marker,
    /// The declared types the argument was sent to, with the variance of the position.
    sends: Vec<(TUnion, Variance)>,
    /// What flowed into the argument (`add(T $x)` on the object).
    lower_bounds: Vec<TUnion>,
    /// Sent where anything is accepted (a `mixed` parameter): the object is in use.
    unconstraining: bool,
    /// The template's default or bound, which the marker's delegate hides when something (never) was inferred.
    unconstrained: Option<TUnion>,
}

/// The observation pass's constraints (TemplateArgumentConstraints), in observation order.
#[derive(Clone, Debug, Default)]
pub struct Observed {
    sites: IndexMap<SiteKey, Observation>,
}

impl Observed {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sites.is_empty()
    }

    fn entry(&mut self, m: &Marker) -> &mut Observation {
        self.sites.entry(m.key).or_insert_with(|| Observation {
            marker: m.clone(),
            sends: Vec::new(),
            lower_bounds: Vec::new(),
            unconstraining: false,
            unconstrained: None,
        })
    }

    /// A site was created with the template's default or bound; it resolves even when nothing is observed.
    pub fn site(&mut self, m: &Marker, unconstrained: TUnion) {
        self.entry(m).unconstrained = Some(unconstrained);
    }

    fn send(&mut self, m: &Marker, declared: TUnion, variance: Variance) {
        self.entry(m).sends.push((declared, variance));
    }

    fn lower_bound(&mut self, m: &Marker, received: TUnion) {
        self.entry(m).lower_bounds.push(received);
    }

    fn unconstraining_send(&mut self, m: &Marker) {
        self.entry(m).unconstraining = true;
    }

    /// `collectSites()`: every marker in the type, with those its initial types carry.
    fn collect_sites(&mut self, t: &TUnion, out: &mut Vec<Marker>) {
        for atomic in t.types.iter() {
            if let Some(m) = as_marker(atomic) {
                if let Some(initial) = &m.initial {
                    self.collect_sites(initial, out);
                }
                out.push(m);
                continue;
            }
            visit_children(atomic, &mut |child| self.collect_sites(child, out));
        }
    }

    /// `collectSend()`: `actual` flows into `declared`, a declared return, property or `@var` type.
    pub fn collect_send(&mut self, codebase: &CodebaseMetadata, declared: &TUnion, actual: &TUnion) {
        self.observe_send(codebase, declared, actual, false);
    }

    /// `collectArgument()`: an argument was passed to a parameter of this declared type (the receiver's class-level
    /// arguments in place, the call's own templates still templates). `resolved` is the parameter type with the
    /// call's templates inferred: a closure argument is invoked with its parameters.
    pub fn collect_argument(
        &mut self,
        codebase: &CodebaseMetadata,
        parameter: &TUnion,
        resolved: &TUnion,
        argument: &TUnion,
        pure: bool,
    ) {
        // a pure consumer accepting anything cannot initialize an empty object
        if pure && is_plain_mixed(parameter) {
            return;
        }
        if !contains_marker(parameter) && !contains_marker(argument) {
            return;
        }
        for sent in argument.types.iter() {
            for target in resolved.types.iter() {
                self.observe_closure_invocation(codebase, sent, target, false);
            }
        }
        self.observe_send(codebase, parameter, argument, true);
        self.observe_lower_bound(codebase, parameter, argument);
    }

    fn observe_send(&mut self, codebase: &CodebaseMetadata, declared: &TUnion, actual: &TUnion, call_argument: bool) {
        if !contains_marker(actual) {
            return;
        }
        // a template bounded by mixed is mixed to a call argument: the argument is used, not constrained
        if call_argument && (is_plain_mixed(declared) || is_mixed_template(declared)) {
            let mut sites = Vec::new();
            self.collect_sites(actual, &mut sites);
            for m in &sites {
                self.unconstraining_send(m);
            }
            return;
        }
        if is_template(declared) {
            return;
        }
        if actual.types.len() > 1 {
            for member in actual.types.iter() {
                self.observe_send(codebase, declared, &TUnion::from_atomic(member.clone()), call_argument);
            }
            return;
        }
        if declared.types.len() > 1 {
            for member in declared.types.iter() {
                self.observe_send(codebase, &TUnion::from_atomic(member.clone()), actual, call_argument);
            }
            return;
        }
        let (Some(actual_atomic), Some(declared_atomic)) = (actual.types.first(), declared.types.first()) else {
            return;
        };
        // a closure sent to a callable type is invoked with that type's parameters; a call argument's parameter
        // that mentions the call's templates is read with them inferred, in collect_argument()
        if let (TAtomic::Callable(TCallable::Signature(_)), TAtomic::Callable(TCallable::Signature(_))) =
            (actual_atomic, declared_atomic)
        {
            self.observe_closure_invocation(codebase, actual_atomic, declared_atomic, call_argument);
            return;
        }
        // a bare marker is a derived value (Foo<T>::get()) and never constrains; never holds no markers
        if as_marker(actual_atomic).is_some() || matches!(actual_atomic, TAtomic::Never) {
            return;
        }
        if let TAtomic::Object(TObject::Named(object)) = actual_atomic {
            let TAtomic::Object(TObject::Named(declared_object)) = declared_atomic else {
                return;
            };
            // the declared type names an ancestor: its arguments map onto the object's through @extends/@implements
            let Some(declared_arguments) = declared_object.get_type_parameters() else {
                return;
            };
            let (Some(declared_class), Some(object_class)) = (
                codebase.get_class_like(declared_object.name.as_bytes()),
                codebase.get_class_like(object.name.as_bytes()),
            ) else {
                return;
            };
            if !codebase.is_instance_of(object.name.as_bytes(), declared_object.name.as_bytes()) {
                return;
            }
            for (i, (template, _)) in declared_class.template_types.iter().enumerate() {
                let Some(declared_argument) = declared_arguments.get(i) else {
                    continue;
                };
                let Some(argument) = get_specialized_template_type(
                    codebase,
                    *template,
                    declared_class.name,
                    object_class,
                    object.get_type_parameters(),
                ) else {
                    continue;
                };
                let variance = variance_at(declared_object, declared_class, i);
                self.observe_argument_send(codebase, declared_argument, &argument, variance, call_argument);
            }
            return;
        }
        if matches!(actual_atomic, TAtomic::Object(_)) {
            return;
        }
        // ConstantArrayType::inferTemplateTypes() and accepts(): a shape sent to a shape, offset by offset
        if let (TAtomic::Array(TArray::Keyed(actual_shape)), TAtomic::Array(TArray::Keyed(declared_shape))) =
            (actual_atomic, declared_atomic)
            && let (Some(actual_items), Some(declared_items)) = (&actual_shape.known_items, &declared_shape.known_items)
        {
            for (key, (_, actual_value)) in actual_items.iter() {
                if let Some((_, declared_value)) = declared_items.get(key) {
                    self.observe_send(codebase, declared_value, actual_value, call_argument);
                }
            }
            return;
        }
        let (Some((actual_key, actual_value)), Some((declared_key, declared_value))) =
            (iterable_parameters(actual_atomic, codebase), iterable_parameters(declared_atomic, codebase))
        else {
            return;
        };
        if !(!call_argument && is_plain_mixed(&declared_key)) {
            self.observe_send(codebase, &declared_key, &actual_key, call_argument);
        }
        if !(!call_argument && is_plain_mixed(&declared_value)) {
            self.observe_send(codebase, &declared_value, &actual_value, call_argument);
        }
    }

    /// A closure `sent` to the callable type `target`: each parameter marker of the closure gets the parameter type
    /// it is invoked with, without closure markers, and a template still open in it as its bound. `skip_templates` leaves out the parameter types
    /// that mention templates.
    fn observe_closure_invocation(
        &mut self,
        codebase: &CodebaseMetadata,
        sent: &TAtomic,
        target: &TAtomic,
        skip_templates: bool,
    ) {
        let (TAtomic::Callable(TCallable::Signature(sent)), TAtomic::Callable(TCallable::Signature(target))) =
            (sent, target)
        else {
            return;
        };
        for (i, parameter) in sent.parameters.iter().enumerate() {
            if let Some(m) = parameter.get_type_signature().and_then(single_marker)
                && is_closure_marker(&m)
            {
                let received = target
                    .parameters
                    .get(i)
                    .and_then(|p| p.get_type_signature().cloned())
                    .unwrap_or_else(mago_codex::ttype::get_mixed);
                if skip_templates && has_templates(&received) {
                    continue;
                }
                // a template the call inferred from this closure holds its own parameter markers
                let received = map_markers(&received, codebase, &mut |m| is_closure_marker(m).then(get_never));
                if received.is_never() {
                    continue;
                }
                self.lower_bound(&m, resolve_to_bounds(&received, codebase));
            }
        }
    }

    /// A template argument of an object sent to the declared argument at the same position. A union of several sites'
    /// markers is split: every marker in it gets the declared argument as an upper bound.
    fn observe_argument_send(
        &mut self,
        codebase: &CodebaseMetadata,
        declared: &TUnion,
        argument: &TUnion,
        variance: Variance,
        call_argument: bool,
    ) {
        if let Some(m) = single_marker(argument) {
            self.observe_marker_send(codebase, declared, &m, variance, call_argument);
            return;
        }
        if argument.types.len() <= 1 {
            self.observe_send(codebase, declared, argument, call_argument);
            return;
        }
        let upper = matches!(variance, Variance::Invariant | Variance::Covariant).then_some(Variance::Covariant);
        for member in argument.types.iter() {
            match as_marker(member) {
                None => self.observe_send(codebase, declared, &TUnion::from_atomic(member.clone()), call_argument),
                Some(m) => {
                    if let Some(upper) = upper {
                        self.observe_marker_send(codebase, declared, &m, upper, call_argument);
                    }
                }
            }
        }
    }

    fn observe_marker_send(
        &mut self,
        codebase: &CodebaseMetadata,
        declared: &TUnion,
        m: &Marker,
        variance: Variance,
        call_argument: bool,
    ) {
        let nothing_inferred = m.initial.as_ref().is_none_or(TUnion::is_never);
        let declared = if call_argument && nothing_inferred && has_templates(declared) {
            resolve_to_bounds(declared, codebase)
        } else {
            declared.clone()
        };
        // Foo<mixed> accepts every Foo<X>, and a declared argument with unresolved template types is no target yet
        if is_plain_mixed(&declared) || has_templates(&declared) {
            // an unresolved call parameter, like mixed, uses the object without constraining it
            if (call_argument && has_templates(&declared)) || is_plain_mixed(&declared) {
                self.unconstraining_send(m);
            }
            return;
        }
        self.send(m, declared.clone(), variance);
        // a site whose inferred argument itself carries markers: an upper bound of the site bounds those too
        if let Some(initial) = &m.initial {
            self.observe_argument_send(codebase, &declared, initial, variance, call_argument);
        }
    }

    /// A parameter type carrying the receiver's markers (`add(T $x)` on `Foo<unresolved>`) puts the argument as a
    /// lower bound on them.
    fn observe_lower_bound(&mut self, codebase: &CodebaseMetadata, parameter: &TUnion, argument: &TUnion) -> bool {
        if !contains_marker(parameter) {
            return false;
        }
        if let Some(m) = single_marker(parameter) {
            self.lower_bound(&m, argument.clone());
            return true;
        }
        if parameter.is_never() || is_template(parameter) || parameter.types.iter().any(is_callable) {
            // callable parameters put the template in a contravariant position: an upper bound, not an inflow
            return false;
        }
        if parameter.types.len() > 1 {
            // UnionType::inferTemplateTypes(): an argument member a sibling takes does not flow into a naked marker
            let mut changed = false;
            for argument_member in argument.types.iter() {
                let argument_member = TUnion::from_atomic(argument_member.clone());
                let mut taken = false;
                for member in parameter.types.iter() {
                    if as_marker(member).is_some() {
                        continue;
                    }
                    let member = TUnion::from_atomic(member.clone());
                    if !contains_marker(&member) {
                        taken = taken || is_super_type(codebase, &member, &argument_member);
                        continue;
                    }
                    if self.observe_lower_bound(codebase, &member, &argument_member) {
                        taken = true;
                        changed = true;
                    }
                }
                if taken {
                    continue;
                }
                for member in parameter.types.iter() {
                    if let Some(m) = as_marker(member) {
                        self.lower_bound(&m, argument_member.clone());
                        changed = true;
                    }
                }
            }
            return changed;
        }
        if argument.types.len() > 1 {
            let mut changed = false;
            for member in argument.types.iter() {
                changed |= self.observe_lower_bound(codebase, parameter, &TUnion::from_atomic(member.clone()));
            }
            return changed;
        }
        let (Some(parameter_atomic), Some(argument_atomic)) = (parameter.types.first(), argument.types.first()) else {
            return false;
        };
        if let TAtomic::Object(TObject::Named(parameter_object)) = parameter_atomic {
            let Some(parameter_class) = codebase.get_class_like(parameter_object.name.as_bytes()) else {
                return false;
            };
            let Some(parameter_arguments) = parameter_object.get_type_parameters() else {
                return false;
            };
            let TAtomic::Object(TObject::Named(argument_object)) = argument_atomic else {
                return false;
            };
            let Some(argument_class) = codebase.get_class_like(argument_object.name.as_bytes()) else {
                return false;
            };
            if !codebase.is_instance_of(argument_object.name.as_bytes(), parameter_object.name.as_bytes()) {
                return false;
            }
            let mut changed = false;
            for (i, (template, _)) in parameter_class.template_types.iter().enumerate() {
                let Some(parameter_argument) = parameter_arguments.get(i) else {
                    continue;
                };
                let Some(ancestor_argument) = get_specialized_template_type(
                    codebase,
                    *template,
                    parameter_class.name,
                    argument_class,
                    argument_object.get_type_parameters(),
                ) else {
                    continue;
                };
                if let Some(m) = single_marker(parameter_argument) {
                    match variance_at(parameter_object, parameter_class, i) {
                        Variance::Invariant => {
                            self.send(&m, ancestor_argument, Variance::Invariant);
                            changed = true;
                            continue;
                        }
                        Variance::Contravariant => {
                            self.send(&m, ancestor_argument, Variance::Covariant);
                            changed = true;
                            continue;
                        }
                        Variance::Bivariant => continue,
                        Variance::Covariant => {}
                    }
                }
                changed |= self.observe_lower_bound(codebase, parameter_argument, &ancestor_argument);
            }
            return changed;
        }
        if matches!(parameter_atomic, TAtomic::Object(_)) {
            return false;
        }
        let (Some((parameter_key, parameter_value)), Some((argument_key, argument_value))) =
            (iterable_parameters(parameter_atomic, codebase), iterable_parameters(argument_atomic, codebase))
        else {
            return false;
        };
        let key = self.observe_lower_bound(codebase, &parameter_key, &argument_key);
        let value = self.observe_lower_bound(codebase, &parameter_value, &argument_value);
        key || value
    }

    /// `TemplateArgumentSolver::solve()`.
    #[must_use]
    pub fn solve(&self, codebase: &CodebaseMetadata) -> Resolutions {
        let mut solver = Solver {
            codebase,
            observations: self.sites.clone(),
            representatives: HashMap::new(),
            resolutions: HashMap::new(),
            resolving: Vec::new(),
        };
        solver.merge_equal_arguments();
        let keys: Vec<SiteKey> = self.sites.keys().copied().collect();
        for key in &keys {
            let resolved = solver.resolve_key(*key);
            solver.resolutions.insert(*key, resolved);
        }
        let mut resolutions = Resolutions::new();
        for key in keys {
            let representative = solver.representative(key);
            if let Some(resolved) = solver.resolutions.get(&representative).cloned() {
                resolutions.insert(key, resolved);
            }
        }
        resolutions
    }
}

struct Solver<'c> {
    codebase: &'c CodebaseMetadata,
    observations: IndexMap<SiteKey, Observation>,
    representatives: HashMap<SiteKey, SiteKey>,
    resolutions: HashMap<SiteKey, TUnion>,
    resolving: Vec<SiteKey>,
}

impl Solver<'_> {
    fn representative(&mut self, key: SiteKey) -> SiteKey {
        let representative = self.representatives.get(&key).copied().unwrap_or(key);
        if representative == key {
            return key;
        }
        let root = self.representative(representative);
        self.representatives.insert(key, root);
        root
    }

    /// Invariant arguments sent to each other share one variable until all their bounds are known.
    fn merge_equal_arguments(&mut self) {
        let mut ranks: HashMap<SiteKey, u32> = HashMap::new();
        let keys: Vec<SiteKey> = self.observations.keys().copied().collect();
        for key in &keys {
            let sends = self.observations[key].sends.clone();
            for (sent, variance) in sends {
                if variance != Variance::Invariant {
                    continue;
                }
                let Some(other) = single_marker(&sent) else {
                    continue;
                };
                if !self.observations.contains_key(&other.key) {
                    continue;
                }
                let left = self.representative(*key);
                let right = self.representative(other.key);
                if left == right {
                    continue;
                }
                let left_rank = ranks.get(&left).copied().unwrap_or(0);
                let right_rank = ranks.get(&right).copied().unwrap_or(0);
                if left_rank < right_rank {
                    self.representatives.insert(left, right);
                } else {
                    self.representatives.insert(right, left);
                    if left_rank == right_rank {
                        ranks.insert(left, left_rank + 1);
                    }
                }
            }
        }
        if self.representatives.is_empty() {
            return;
        }
        let mut merged: IndexMap<SiteKey, Observation> = IndexMap::new();
        let observations = std::mem::take(&mut self.observations);
        for (key, mut observation) in observations {
            let representative = self.representative(key);
            let codebase = self.codebase;
            if let Some(initial) = &observation.marker.initial {
                let representative_of = |m: &Marker| self.representatives.get(&m.key).copied().unwrap_or(m.key);
                // a send to itself is no bound
                let without_self = map_markers(initial, codebase, &mut |m| {
                    (representative_of(m) == representative).then(get_never)
                });
                observation.marker.initial = Some(without_self);
            }
            let mut sends = Vec::new();
            for (sent, variance) in observation.sends {
                if let Some(m) = single_marker(&sent)
                    && self.representative(m.key) == representative
                {
                    continue;
                }
                sends.push((sent, variance));
            }
            observation.sends = sends;
            match merged.get_mut(&representative) {
                None => {
                    merged.insert(representative, observation);
                }
                Some(into) => {
                    if let Some(initial) = observation.marker.initial {
                        into.marker.initial = Some(match &into.marker.initial {
                            Some(existing) => union(self.codebase, existing, &initial),
                            None => initial,
                        });
                    }
                    into.sends.extend(observation.sends);
                    into.lower_bounds.extend(observation.lower_bounds);
                    into.unconstraining |= observation.unconstraining;
                    if into.unconstrained.is_none() {
                        into.unconstrained = observation.unconstrained;
                    }
                }
            }
        }
        self.observations = merged;
    }

    fn resolve_key(&mut self, key: SiteKey) -> TUnion {
        let key = self.representative(key);
        if let Some(resolved) = self.resolutions.get(&key) {
            return resolved.clone();
        }
        let Some(observation) = self.observations.get(&key).cloned() else {
            return get_never();
        };
        if self.resolving.contains(&key) {
            // an inferred argument that refers back to itself through another site: the inferred type stands
            return observation.marker.delegate;
        }
        self.resolving.push(key);
        let resolved = self.resolve_observation(&observation);
        self.resolving.pop();
        self.resolutions.insert(key, resolved.clone());
        resolved
    }

    /// The markers of observed sites in a type replaced by their resolutions.
    fn substitute(&mut self, t: &TUnion) -> TUnion {
        let codebase = self.codebase;
        map_markers(t, codebase, &mut |m| {
            let key = self.representative(m.key);
            if self.observations.contains_key(&key) {
                Some(self.resolve_key(key))
            } else {
                Some(self.substitute(&m.delegate))
            }
        })
    }

    fn resolve_observation(&mut self, observation: &Observation) -> TUnion {
        let initial = observation.marker.initial.as_ref().map(|t| self.substitute(t));
        let mut lower_bounds: Vec<TUnion> =
            observation.lower_bounds.iter().map(|b| self.substitute(&unwrap_bare(b, self.codebase))).collect();
        // nothing inferred, or never (an empty array): every send accepts it
        let accepts_anything = initial.as_ref().is_none_or(TUnion::is_never);
        // a covariant template already accepts every subtype - a known initial type is never clamped
        if !observation.marker.covariant || accepts_anything {
            let mut covariant_fallback = None;
            let mut marker_fallback = None;
            for (sent, variance) in &observation.sends {
                if *variance == Variance::Covariant {
                    // an upper bound; with nothing inferred it is the best information there is
                    if !accepts_anything {
                        continue;
                    }
                    if single_marker(sent).is_some() {
                        // another site's variable is an edge, not information
                        if marker_fallback.is_none() {
                            marker_fallback = Some(sent.clone());
                        }
                        continue;
                    }
                    if covariant_fallback.is_none() {
                        covariant_fallback = Some(self.substitute(sent));
                    }
                    continue;
                }
                let sent = self.substitute(sent);
                if *variance == Variance::Contravariant {
                    // Foo<contravariant int> accepts Foo<X> for every X wider than int
                    lower_bounds.push(sent);
                    continue;
                }
                if *variance != Variance::Invariant {
                    continue;
                }
                // invariant: the first send that accepts what was inferred resolves the argument
                if !accepts_anything && !is_super_type(self.codebase, &sent, initial.as_ref().expect("inferred")) {
                    continue;
                }
                return sent;
            }
            if covariant_fallback.is_none()
                && let Some(fallback) = marker_fallback
            {
                covariant_fallback = Some(self.substitute(&fallback));
            }
            if let Some(fallback) = covariant_fallback {
                return fallback;
            }
        }
        let mut parts = lower_bounds;
        if let Some(initial) = &initial
            && !initial.is_never()
        {
            parts.push(initial.clone());
        }
        if parts.is_empty() {
            if observation.unconstraining {
                // sent where anything is accepted: the template's default or bound describes the argument
                return observation.unconstrained.clone().unwrap_or_else(|| observation.marker.delegate.clone());
            }
            if let Some(initial) = initial {
                return initial;
            }
            return observation.marker.delegate.clone();
        }
        let mut resolved = parts[0].clone();
        for part in &parts[1..] {
            resolved = union(self.codebase, &resolved, part);
        }
        resolved
    }
}

fn union(codebase: &CodebaseMetadata, a: &TUnion, b: &TUnion) -> TUnion {
    mago_codex::ttype::combine_union_types(a, b, codebase, CombinerOptions::default())
}

/// `$a->isSuperTypeOf($b)->yes()`.
fn is_super_type(codebase: &CodebaseMetadata, a: &TUnion, b: &TUnion) -> bool {
    union_comparator::is_contained_by(codebase, b, a, false, false, false, &mut ComparisonResult::new())
}

fn is_plain_mixed(t: &TUnion) -> bool {
    t.is_mixed()
}

/// A template type that is not a marker.
fn is_template(t: &TUnion) -> bool {
    matches!(t.types.as_ref(), [atomic @ TAtomic::GenericParameter(_)] if as_marker(atomic).is_none())
}

/// A template that is not a marker, bounded by mixed (PHPStan's TemplateMixedType).
fn is_mixed_template(t: &TUnion) -> bool {
    matches!(t.types.as_ref(), [atomic @ TAtomic::GenericParameter(p)] if as_marker(atomic).is_none() && p.constraint.is_mixed())
}

/// Whether a template that is not a marker occurs in the type.
fn has_templates(t: &TUnion) -> bool {
    t.types.iter().any(|atomic| match atomic {
        TAtomic::GenericParameter(_) => as_marker(atomic).is_none(),
        _ => {
            let mut found = false;
            visit_children(atomic, &mut |child| found = found || has_templates(child));
            found
        }
    })
}

/// `TemplateTypeHelper::resolveToDefaults()` of a call parameter: its templates as their bounds.
fn resolve_to_bounds(t: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
    let mut types = Vec::with_capacity(t.types.len());
    for atomic in t.types.iter() {
        match atomic {
            TAtomic::GenericParameter(parameter) if as_marker(atomic).is_none() => {
                types.extend(resolve_to_bounds(&parameter.constraint, codebase).types.iter().cloned());
            }
            TAtomic::Object(TObject::Named(named)) if named.type_parameters.is_some() => {
                let mut named = named.clone();
                if let Some(parameters) = named.get_type_parameters_mut() {
                    for parameter in parameters {
                        *parameter = resolve_to_bounds(parameter, codebase);
                    }
                }
                types.push(TAtomic::Object(TObject::Named(named)));
            }
            TAtomic::Iterable(iterable) => {
                let mut iterable = iterable.clone();
                let key = resolve_to_bounds(iterable.get_key_type(), codebase);
                *iterable.get_key_type_mut() = key;
                let value = resolve_to_bounds(iterable.get_value_type(), codebase);
                *iterable.get_value_type_mut() = value;
                types.push(TAtomic::Iterable(iterable));
            }
            TAtomic::Array(TArray::Keyed(keyed)) if keyed.parameters.is_some() => {
                let mut keyed = keyed.clone();
                if let Some((key, value)) = &mut keyed.parameters {
                    *Arc::make_mut(key) = resolve_to_bounds(key, codebase);
                    *Arc::make_mut(value) = resolve_to_bounds(value, codebase);
                }
                types.push(TAtomic::Array(TArray::Keyed(keyed)));
            }
            TAtomic::Array(TArray::List(list)) => {
                let mut list = list.clone();
                *Arc::make_mut(&mut list.element_type) = resolve_to_bounds(&list.element_type, codebase);
                types.push(TAtomic::Array(TArray::List(list)));
            }
            other => types.push(other.clone()),
        }
    }
    if types.is_empty() {
        return get_never();
    }
    t.clone_with_types(types)
}

fn is_callable(atomic: &TAtomic) -> bool {
    matches!(atomic, TAtomic::Callable(_))
}

/// The effective variance of the declared object's argument `i`: its call-site variance, else the template's own.
fn variance_at(
    object: &mago_codex::ttype::atomic::object::named::TNamedObject,
    class: &mago_codex::metadata::class_like::ClassLikeMetadata,
    i: usize,
) -> Variance {
    let call_site = object.variances.as_ref().and_then(|v| v.get(i)).copied().unwrap_or(Variance::Invariant);
    if call_site == Variance::Invariant {
        class.template_variance.get(i).copied().unwrap_or(Variance::Invariant)
    } else {
        call_site
    }
}

/// The key and value types of an array or `iterable`.
fn iterable_parameters(atomic: &TAtomic, codebase: &CodebaseMetadata) -> Option<(TUnion, TUnion)> {
    match atomic {
        TAtomic::Array(array) => Some(get_array_parameters(array, codebase)),
        TAtomic::Iterable(iterable) => Some((iterable.get_key_type().clone(), iterable.get_value_type().clone())),
        _ => None,
    }
}
