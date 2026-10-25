//! FunctionCallParametersCheck, the part level 0 runs: argument order, counts and names, and void results used.

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::TType;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use pun_phpstan::symbols::ByRef;
use pun_phpstan::types::{Tri, Type, is_super_type_of};

use super::Checker;

/// A ParametersAcceptor, as far as counting and naming go.
#[derive(Clone, Debug, PartialEq)]
pub struct Acceptor {
    /// `isOptional()` per parameter.
    pub optional: Vec<bool>,
    /// `isVariadic()`: a variadic parameter, or a body calling `func_get_args()`.
    pub variadic: bool,
    /// The return type is `void` (`getKeepVoidType()`).
    pub void: bool,
    /// Per parameter: the name, the type's description and whether it is variadic.
    pub names: Vec<String>,
    pub types: Vec<String>,
    pub variadics: Vec<bool>,
    /// A built-in's (no named argument goes to its variadic parameter).
    pub builtin: bool,
    /// Per parameter: its declared type (see [`ParamType`]) and `passedByReference()`.
    pub param_types: Vec<ParamType>,
    pub by_ref: Vec<ByRef>,
    /// A built-in method's class, which `self`/`static` in its parameter types name.
    pub class: Option<String>,
    /// A user function-like's codex entry, to read its parameter types as PHPStan does: the declaring class-like of a
    /// method (`None` for a function) and the function-like's name.
    pub callee: Option<(Option<String>, String)>,
}

/// A parameter's type as its reflection declares it, converted when an argument type is checked against it.
#[derive(Clone, Debug, PartialEq)]
pub enum ParamType {
    /// A built-in's PHPDoc type string (data/symbols.json).
    Builtin(String),
    /// A user parameter, typed from its function's reflection (`declared::codex_params()`).
    User,
    /// A callable type's parameter.
    Typed(pun_phpstan::types::Type),
}

impl Acceptor {
    fn min(&self) -> usize {
        self.optional.iter().filter(|o| !**o).count()
    }

    fn accepts(&self, n: usize) -> bool {
        n >= self.min() && (self.variadic || n <= self.optional.len())
    }

    /// A user-defined function-like (Mago's metadata): a parameter is optional when it and every later one has a
    /// default or is variadic, as `ReflectionParameter::isOptional()` says.
    pub fn of(f: &FunctionLikeMetadata) -> Acceptor {
        let mut optional = vec![false; f.parameters.len()];
        let mut tail = true;
        for (i, p) in f.parameters.iter().enumerate().rev() {
            tail &= p.flags.has_default() || p.flags.is_variadic();
            optional[i] = tail;
        }
        Acceptor {
            optional,
            variadic: f.parameters.iter().any(|p| p.flags.is_variadic())
                || f.flags.uses_func_get_args(),
            void: f
                .return_type_metadata
                .as_ref()
                .is_some_and(|t| t.type_union.is_void()),
            names: f
                .parameters
                .iter()
                .map(|p| String::from_utf8_lossy(&p.name.0.as_bytes()[1..]).into_owned())
                .collect(),
            // ponytail: Mago's id of the parameter's type, read as a PHPDoc type, stands in for PHPStan's
            types: f
                .parameters
                .iter()
                .map(|p| p.type_metadata.as_ref().map_or_else(|| "mixed".to_string(), |t| describe(&t.type_union.get_id().to_string())))
                .collect(),
            variadics: f.parameters.iter().map(|p| p.flags.is_variadic()).collect(),
            builtin: false,
            param_types: vec![ParamType::User; f.parameters.len()],
            // PhpParameterReflection: a by-reference parameter of user code creates a new variable
            by_ref: f
                .parameters
                .iter()
                .map(|p| if p.flags.is_by_reference() { ByRef::CreatesNewVariable } else { ByRef::No })
                .collect(),
            class: None,
            callee: Some((None, String::from_utf8_lossy(f.name.as_bytes()).into_owned())),
        }
    }

    /// A built-in, as PHPStan reflects it (data/symbols.json).
    pub fn of_builtin(v: &pun_phpstan::symbols::Variant) -> Acceptor {
        Acceptor {
            optional: v.params.iter().map(|p| p.optional).collect(),
            variadic: v.params.iter().any(|p| p.variadic),
            void: v.ret == "void",
            names: v.params.iter().map(|p| p.name.clone()).collect(),
            types: v.params.iter().map(|p| describe(&p.ty)).collect(),
            variadics: v.params.iter().map(|p| p.variadic).collect(),
            builtin: true,
            param_types: v.params.iter().map(|p| ParamType::Builtin(p.ty.clone())).collect(),
            by_ref: v.params.iter().map(|p| p.by_ref).collect(),
            class: None,
            callee: None,
        }
    }
}

fn describe(ty: &str) -> String {
    pun_phpstan::resolve::resolve_str(ty, None).describe(pun_phpstan::types::Level::TypeOnly)
}

/// A function-like's variants, and a built-in's variants for calls with named arguments (PHP's parameter names
/// where PHPStan's stubs use others).
pub struct Variants {
    pub all: Vec<Acceptor>,
    pub named: Option<Vec<Acceptor>>,
}

impl Variants {
    pub fn of(f: &FunctionLikeMetadata) -> Variants {
        Variants { all: vec![Acceptor::of(f)], named: None }
    }

    pub fn of_builtin(f: &pun_phpstan::symbols::Function) -> Variants {
        let list = |v: &[pun_phpstan::symbols::Variant]| v.iter().map(Acceptor::of_builtin).collect();
        Variants { all: list(&f.variants), named: f.named.as_deref().map(list) }
    }
}

/// `ParametersAcceptorSelector::selectFromArgs()` as far as the count goes: the named-argument variants for a call
/// with names, then the one variant that accepts the arguments, else `combineAcceptors()` of all.
pub fn select(c: &Checker, variants: Variants, counts: &Counts) -> Acceptor {
    let variants = match variants.named {
        Some(named) if counts.named => named,
        _ => variants.all,
    };
    if variants.len() == 1 {
        return variants.into_iter().next().expect("one");
    }
    let n = counts.positional + counts.names;
    let acceptable: Vec<&Acceptor> = variants.iter().filter(|v| counts.unpack || v.accepts(n)).collect();
    match acceptable.as_slice() {
        [] => combine(&variants.iter().collect::<Vec<_>>()),
        [one] => (*one).clone(),
        // ponytail: without the arguments' types (below level 5, named or unpacked arguments) the first wins
        [first, ..] => match winning(c, &acceptable, counts) {
            Some(w) if w.is_empty() => combine(&acceptable),
            Some(w) => combine(&w),
            None => (*first).clone(),
        },
    }
}

/// `selectFromTypes()`'s winning acceptors: those whose parameters are supertypes of the arguments' types with the
/// highest certainty (`None` when an argument's type is unknown).
fn winning<'v>(c: &Checker, acceptable: &[&'v Acceptor], counts: &Counts) -> Option<Vec<&'v Acceptor>> {
    if counts.args.iter().any(|a| a.unpack.is_some()) {
        return None;
    }
    // a named argument's type is keyed by its name, where the loop over the parameters' indexes stops
    let types = counts
        .args
        .iter()
        .take_while(|a| a.name.is_none())
        .map(|a| c.types.get(&a.value).map(|t| c.to_phpstan(t)))
        .collect::<Option<Vec<_>>>()?;
    let rank = |t: Tri| [Tri::No, Tri::Maybe, Tri::Yes].iter().position(|x| *x == t);
    let scored: Vec<_> = acceptable
        .iter()
        .map(|a| {
            let sup = types.iter().enumerate().take(a.names.len()).fold(Tri::Yes, |sup, (i, ty)| {
                // ponytail: GenericParametersAcceptorResolver resolves a template from the argument; taken as yes
                sup.and(match super::types::declared_type(c, a, i) {
                    None | Some(Type::Mixed { .. }) => Tri::Maybe,
                    Some(p) if super::types::is_late(&p) => Tri::Yes,
                    Some(p) => super::declared::with_classes(c, || is_super_type_of(&p, ty)),
                })
            });
            (rank(sup), *a)
        })
        .filter(|(r, _)| *r > rank(Tri::No))
        .collect();
    let best = scored.iter().map(|(r, _)| *r).max();
    Some(scored.into_iter().filter(|(r, _)| Some(*r) == best).map(|(_, a)| a).collect())
}

/// `combineAcceptors()`.
pub(super) fn combine(variants: &[&Acceptor]) -> Acceptor {
    let min = variants.iter().map(|v| v.min()).min().unwrap_or(0);
    let mut combined = Acceptor {
        optional: Vec::new(),
        variadic: false,
        void: variants.iter().all(|v| v.void),
        names: Vec::new(),
        types: Vec::new(),
        variadics: Vec::new(),
        builtin: variants.iter().any(|v| v.builtin),
        param_types: Vec::new(),
        by_ref: Vec::new(),
        class: variants.first().and_then(|v| v.class.clone()),
        callee: None,
    };
    for v in variants {
        combined.variadic |= v.variadic;
        for i in 0..v.names.len() {
            if i == combined.names.len() {
                combined.names.push(v.names[i].clone());
                combined.types.push(v.types[i].clone());
                combined.variadics.push(v.variadics[i]);
                combined.optional.push(i + 1 > min);
                combined.param_types.push(v.param_types[i].clone());
                combined.by_ref.push(v.by_ref[i]);
                continue;
            }
            // ponytail: the union of the types prints both when they differ, without TypeCombinator's normalization
            if combined.names[i] != v.names[i] {
                combined.names[i] = format!("{}|{}", combined.names[i], v.names[i]);
            }
            if combined.types[i] != v.types[i] {
                combined.types[i] = format!("{}|{}", combined.types[i], v.types[i]);
            }
            combined.param_types[i] = match (&combined.param_types[i], &v.param_types[i]) {
                (ParamType::Builtin(a), ParamType::Builtin(b)) if a != b => ParamType::Builtin(format!("{a}|{b}")),
                (a, _) => a.clone(),
            };
            // `PassedByReference::combine()`: the stronger
            let rank = |b: ByRef| [ByRef::No, ByRef::ReadsArgument, ByRef::CreatesNewVariable].iter().position(|x| *x == b);
            combined.by_ref[i] = std::cmp::max_by_key(combined.by_ref[i], v.by_ref[i], |b| rank(*b));
            let variadic = combined.variadics[i] || v.variadics[i];
            combined.variadics[i] = variadic;
            if variadic {
                // the variadic parameter swallows every parameter behind it
                // ponytail: upstream also merges their types into its type; a variadic parameter is never missing
                combined.names.truncate(i + 1);
                combined.types.truncate(i + 1);
                combined.variadics.truncate(i + 1);
                combined.optional.truncate(i + 1);
                combined.param_types.truncate(i + 1);
                combined.by_ref.truncate(i + 1);
                break;
            }
        }
    }
    combined
}

/// `CallToFunctionParametersRule::getVariantForArgumentsCount()`: when no variant accepts the argument count but
/// their combination does, the count is checked against the variant closest to it.
pub fn closest_variant(
    variants: &[Acceptor],
    counts: &Counts,
    selected: &Acceptor,
) -> Option<Acceptor> {
    if variants.len() < 2
        || counts
            .args
            .iter()
            .any(|a| a.unpack.is_some() || a.name.is_some())
    {
        return None;
    }
    let n = counts.args.len();
    if !selected.accepts(n) || variants.iter().any(|v| v.accepts(n)) {
        return None;
    }
    let mut closest: Option<(&Acceptor, usize)> = None;
    for v in variants {
        let insufficient = n < v.min();
        let distance = if insufficient {
            v.min() - n
        } else {
            n - v.optional.len()
        };
        if closest.is_some_and(|(_, d)| distance > d || (distance == d && !insufficient)) {
            continue;
        }
        closest = Some((v, distance));
    }
    closest.map(|(v, _)| v.clone())
}

/// One argument as written.
#[derive(Clone, Debug)]
pub struct Arg {
    pub name: Option<String>,
    /// `...$value`: what unpacking it gives.
    pub unpack: Option<Unpacked>,
    pub span: Span,
    /// The value expression, and what passing it by reference takes.
    pub value: Span,
    pub kind: ArgKind,
    /// A constant array unpacked: its elements' value expressions.
    pub elements: Vec<Span>,
}

/// What an argument expression is, as the by-reference check reads it.
#[derive(Clone, Debug)]
pub enum ArgKind {
    /// A variable, an offset or a property fetch.
    Writable,
    /// Holds a nullsafe fetch or call.
    Nullsafe,
    /// A call to a function named by this identifier (its by-reference return counts).
    Function(Span),
    /// A method call: the object, and the method's name (`None`: a dynamic name).
    MethodCall(Span, Option<Vec<u8>>),
    /// A static call on a named class, `self` or `static` (the class's identifier span; `None` for the keywords).
    StaticCall(Option<Span>, Option<Vec<u8>>),
    /// Another static call (ponytail: taken to return by reference).
    OtherCall,
    Other,
}

fn selector(s: &ClassLikeMemberSelector) -> Option<Vec<u8>> {
    match s {
        ClassLikeMemberSelector::Identifier(i) => Some(i.value.to_vec()),
        _ => None,
    }
}

fn kind(e: &Expression) -> ArgKind {
    if super::assign::contains_nullsafe(e) {
        return ArgKind::Nullsafe;
    }
    match e {
        Expression::Parenthesized(p) => kind(p.expression),
        Expression::Variable(_)
        | Expression::ArrayAccess(_)
        | Expression::ArrayAppend(_)
        | Expression::Access(Access::Property(_) | Access::StaticProperty(_)) => ArgKind::Writable,
        Expression::Call(mago_syntax::cst::Call::Function(f)) => match f.function {
            Expression::Identifier(id) => ArgKind::Function(id.span()),
            _ => ArgKind::Other,
        },
        Expression::Call(mago_syntax::cst::Call::Method(m)) => ArgKind::MethodCall(m.object.span(), selector(&m.method)),
        Expression::Call(mago_syntax::cst::Call::StaticMethod(m)) => match m.class {
            Expression::Identifier(id) => ArgKind::StaticCall(Some(id.span()), selector(&m.method)),
            Expression::Self_(_) | Expression::Static(_) => ArgKind::StaticCall(None, selector(&m.method)),
            _ => ArgKind::OtherCall,
        },
        _ => ArgKind::Other,
    }
}

/// How an argument list counts.
#[derive(Clone, Debug, Default)]
pub struct Counts {
    pub positional: usize,
    /// How many named arguments.
    pub names: usize,
    /// Has named arguments (or unpacks an array with string keys).
    pub named: bool,
    /// Unpacks an array that is not a constant one.
    pub unpack: bool,
    /// Has any argument.
    pub any: bool,
    pub args: Vec<Arg>,
}

pub fn count(args: &ArgumentList) -> Counts {
    let mut counts = Counts {
        any: !args.arguments.is_empty(),
        ..Counts::default()
    };
    for arg in args.arguments.iter() {
        match arg {
            Argument::Named(n) => counts.named(n),
            Argument::Positional(p) => counts.positional(p),
        }
    }
    counts
}

/// [`count()`] of an anonymous class's constructor arguments or an attribute's.
pub fn count_partial(args: &PartialArgumentList) -> Counts {
    let mut counts = Counts {
        any: !args.arguments.is_empty(),
        ..Counts::default()
    };
    for arg in args.arguments.iter() {
        // placeholders (`?`) are no PHP 8.5 syntax here, so php-parser rejects them
        if let PartialArgument::Positional(p) = arg {
            counts.positional(p);
        } else if let PartialArgument::Named(n) = arg {
            counts.named(n);
        }
    }
    counts
}

impl Counts {
    fn named(&mut self, n: &NamedArgument) {
        self.push_named(String::from_utf8_lossy(n.name.value).into_owned(), n.value, n.span());
    }

    fn positional(&mut self, p: &PositionalArgument) {
        self.push_positional(p.value, p.ellipsis.is_some(), p.span());
    }

    /// Some of a call's arguments (`call_user_func()`'s after the callback).
    pub fn of<'x, 'arena: 'x>(args: impl IntoIterator<Item = &'x Argument<'arena>>) -> Counts {
        let mut counts = Counts::default();
        for arg in args {
            counts.any = true;
            match arg {
                Argument::Named(n) => counts.named(n),
                Argument::Positional(p) => counts.positional(p),
            }
        }
        counts
    }

    /// One positional argument (a pipe's input).
    pub fn of_value(value: &Expression) -> Counts {
        let mut counts = Counts { any: true, ..Counts::default() };
        counts.push_positional(value, false, value.span());
        counts
    }

    /// `call_user_func_array()`'s constant array as arguments (`reorderCallUserFuncArrayArguments()`): a string key
    /// names one; `None` for a dynamic or empty key.
    pub fn of_array(elements: &[&ArrayElement]) -> Option<Counts> {
        let mut counts = Counts::default();
        for e in elements {
            counts.any = true;
            match e {
                ArrayElement::Value(v) => counts.push_positional(v.value, false, e.span()),
                ArrayElement::Variadic(v) => counts.push_positional(v.value, true, e.span()),
                ArrayElement::KeyValue(kv) if let Expression::Literal(Literal::Integer(_)) = kv.key => {
                    counts.push_positional(kv.value, false, e.span())
                }
                ArrayElement::KeyValue(kv)
                    if let Expression::Literal(Literal::String(s)) = kv.key
                        && let Some(key) = s.value.filter(|k| !k.is_empty()) =>
                {
                    let key = String::from_utf8_lossy(key).into_owned();
                    // an integer-like string key is an integer one (`key([$k => null])`)
                    if key.parse::<i64>().is_ok_and(|i| i.to_string() == key) {
                        counts.push_positional(kv.value, false, e.span());
                    } else {
                        counts.push_named(key, kv.value, e.span());
                    }
                }
                // a dynamic or empty key, or a missing element
                _ => return None,
            }
        }
        Some(counts)
    }

    fn push_named(&mut self, name: String, value: &Expression, span: Span) {
        self.named = true;
        self.names += 1;
        self.args.push(Arg {
            name: Some(name),
            unpack: None,
            span,
            value: value.span(),
            kind: kind(value),
            elements: Vec::new(),
        });
    }

    fn push_positional(&mut self, value: &Expression, ellipsis: bool, span: Span) {
        let unpack = ellipsis.then(|| unpacked(value));
        match &unpack {
            None => self.positional += 1,
            Some(Unpacked::Keys(keys)) => {
                self.positional += keys.iter().filter(|k| k.is_none()).count();
                self.named |= keys.iter().any(Option::is_some);
            }
            Some(Unpacked::Unknown) => self.unpack = true,
        }
        let elements = match (&unpack, value) {
            (Some(Unpacked::Keys(_)), Expression::Array(a)) => a.elements.iter().map(element).collect(),
            (Some(Unpacked::Keys(_)), Expression::LegacyArray(a)) => a.elements.iter().map(element).collect(),
            _ => Vec::new(),
        };
        self.args.push(Arg {
            name: None,
            unpack,
            span,
            value: value.span(),
            kind: kind(value),
            elements,
        });
    }
}

/// A constant array element's value expression (only values and key-values make one).
fn element(e: &ArrayElement) -> Span {
    match e {
        ArrayElement::KeyValue(kv) => kv.value.span(),
        e => e.span(),
    }
}

/// What unpacking an expression gives.
#[derive(Clone, Debug)]
pub enum Unpacked {
    /// A constant array: each element's string key (a named argument) or none.
    Keys(Vec<Option<String>>),
    /// Not a constant array.
    Unknown,
}

fn unpacked(e: &Expression) -> Unpacked {
    let elements: Vec<&ArrayElement> = match e {
        Expression::Array(a) => a.elements.iter().collect(),
        Expression::LegacyArray(a) => a.elements.iter().collect(),
        _ => return Unpacked::Unknown,
    };
    let mut keys = Vec::new();
    for el in &elements {
        match el {
            ArrayElement::Value(_) => keys.push(None),
            ArrayElement::KeyValue(kv) => keys.push(match kv.key {
                Expression::Literal(Literal::String(s)) => Some(String::from_utf8_lossy(s.value.unwrap_or_default()).into_owned()),
                _ => None,
            }),
            _ => return Unpacked::Unknown,
        }
    }
    Unpacked::Keys(keys)
}

/// The calls FunctionCallParametersCheck checks.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CallKind {
    Function,
    Method,
    StaticMethod,
    New,
    Attribute,
    /// CallCallablesRule's `$callable()`.
    Callable,
}

/// The messages of one call kind: `subject` is e.g. `Function foo`, `Method Foo::bar()`, `Class Foo constructor`.
pub struct Call<'s> {
    pub kind: CallKind,
    pub subject: &'s str,
    /// `Result of function foo (void) is used.`; `None` for `new`.
    pub void_message: Option<String>,
}

/// `counts` is [`count()`] of the call's arguments.
pub fn check<'a>(
    c: &mut Checker,
    acceptor: &Acceptor,
    counts: &'a Counts,
    at: impl HasSpan + Copy,
    call: Call,
    first_level: bool,
) {
    // the arguments after unpacking constant arrays
    let mut arguments: Vec<Entry> = Vec::new();
    let (mut has_named, mut has_unpacked) = (false, false);
    for arg in &counts.args {
        has_named |= arg.name.is_some();
        if has_named && arg.unpack.is_some() {
            let message = "Named argument cannot be followed by an unpacked (...) argument.".to_string();
            c.report_non_ignorable(arg.span, "argument.unpackAfterNamed", message, None);
        }
        // PHP 8.1 allows a named argument after an unpacked one
        if has_unpacked && arg.unpack.is_none() && (arg.name.is_none() || !c.surely_at_least(80100)) {
            let message = "Unpacked argument (...) cannot be followed by a non-unpacked argument.".to_string();
            c.report_non_ignorable(arg.span, "argument.nonUnpackAfterUnpacked", message, None);
        }
        let entry = |name: Option<&'a str>, unpack: bool, value: Span| Entry {
            name,
            unpack,
            span: arg.span,
            value,
            kind: &arg.kind,
        };
        match &arg.unpack {
            None => arguments.push(entry(arg.name.as_deref(), false, arg.value)),
            Some(Unpacked::Keys(keys)) => {
                has_unpacked = true;
                has_named |= keys.iter().any(Option::is_some);
                arguments.extend(
                    keys.iter()
                        .zip(&arg.elements)
                        .map(|(k, v)| entry(k.as_deref(), false, *v)),
                );
            }
            Some(Unpacked::Unknown) => {
                has_unpacked = true;
                arguments.push(entry(None, true, arg.value));
            }
        }
    }
    if has_named && !c.surely_at_least(80000) && call.kind != CallKind::Attribute {
        let message = "Named arguments are supported only on PHP 8.0 and later.".to_string();
        c.report_non_ignorable(at, "argument.namedNotSupported", message, None);
    }
    if !has_named {
        let min = acceptor.min();
        let max = (!acceptor.variadic).then_some(acceptor.optional.len());
        let invoked = if counts.unpack {
            max.map_or(min, |m| m.max(min))
        } else {
            counts.positional
        };
        // checkExtraArguments is on from level 1
        if invoked < min || (c.level >= 1 && max.is_some_and(|m| invoked > m)) {
            let s = if invoked == 1 { "" } else { "s" };
            let subject = call.subject;
            let message = match max {
                Some(m) if m == min => {
                    format!("{subject} invoked with {invoked} parameter{s}, {min} required.")
                }
                None => format!(
                    "{subject} invoked with {invoked} parameter{s}, at least {min} required."
                ),
                Some(m) => {
                    format!("{subject} invoked with {invoked} parameter{s}, {min}-{m} required.")
                }
            };
            c.report(at, "arguments.count", message, None);
        }
    }
    if let Some(void) = call.void_message.clone()
        && !first_level
        && acceptor.void
    {
        let identifier = match call.kind {
            CallKind::Function => "function.void",
            CallKind::Method => "method.void",
            CallKind::Callable => "callable.void",
            _ => "staticMethod.void",
        };
        c.report(at, identifier, void, None);
    }
    let parameters = process_arguments(c, acceptor, &arguments, has_named, at, &call);
    if c.level >= 5 {
        super::types::arguments(c, acceptor, &arguments, &parameters, at.span(), &call);
    }
}

/// One argument after unpacking constant arrays.
pub struct Entry<'a> {
    pub name: Option<&'a str>,
    /// Unpacks a non-constant array.
    pub unpack: bool,
    /// The argument (its line).
    pub span: Span,
    /// The expression whose type is passed: the argument's value, or an unpacked constant array's element.
    pub value: Span,
    pub kind: &'a ArgKind,
}

/// `processArguments()`: each argument's parameter, by position or by name (`None`: no parameter is checked).
fn process_arguments(
    c: &mut Checker,
    acceptor: &Acceptor,
    arguments: &[Entry],
    has_named: bool,
    at: impl HasSpan,
    call: &Call,
) -> Vec<Option<usize>> {
    let mut parameters = vec![None; arguments.len()];
    // `in call to function foo`, `... to method Foo::bar()`, `... to Foo constructor`
    let callee = match call.kind {
        CallKind::New => call.subject.trim_start_matches("Class "),
        CallKind::Attribute => call.subject.trim_start_matches("Attribute class "),
        _ => call.subject,
    };
    let callee = match call.kind {
        CallKind::New | CallKind::Attribute => callee.to_string(),
        _ => callee[..1].to_ascii_lowercase() + &callee[1..],
    };
    let count = acceptor.names.len();
    let natively_variadic = acceptor.variadics.iter().any(|v| *v);
    let mut unused: Vec<usize> = (0..count).filter(|i| !acceptor.variadics[*i]).collect();
    let mut named_occurred = false;
    let mut named_for_variadic: Vec<&str> = Vec::new();
    for (i, e) in arguments.iter().enumerate() {
        let (name, unpack, span) = (&e.name, &e.unpack, &e.span);
        let parameter = match name {
            None if i < count => i,
            // func_get_args()
            None if !acceptor.variadic || count == 0 || !acceptor.variadics[count - 1] => break,
            None => count - 1,
            Some(name) => {
                named_occurred = true;
                match acceptor.names.iter().position(|n| n == name) {
                    Some(p) => p,
                    None if !natively_variadic || acceptor.builtin => {
                        let message = format!("Unknown parameter ${name} in call to {callee}.");
                        c.report(*span, "argument.unknown", message, None);
                        continue;
                    }
                    None => {
                        if named_for_variadic.contains(name) {
                            let message = format!("Named parameter ${name} overwrites previous argument.");
                            c.report(*span, "argument.duplicate", message, None);
                        }
                        named_for_variadic.push(name);
                        count - 1
                    }
                }
            }
        };
        if named_occurred && name.is_none() && !unpack {
            let message = "Named argument cannot be followed by a positional argument.".to_string();
            c.report_non_ignorable(*span, "argument.positionalAfterNamed", message, None);
            continue;
        }
        if has_named && !acceptor.variadics[parameter] && !unused.contains(&parameter) {
            let message = format!("Argument for parameter ${} has already been passed.", acceptor.names[parameter]);
            parameters[i] = Some(parameter);
            c.report(*span, "argument.duplicate", message, None);
            continue;
        }
        parameters[i] = Some(parameter);
        unused.retain(|p| *p != parameter);
    }
    if has_named {
        // (keyed by name: callable(int, int)'s unnamed parameters are one)
        let mut seen = std::collections::HashSet::new();
        for p in unused.into_iter().filter(|p| !acceptor.optional[*p] && seen.insert(&acceptor.names[*p])) {
            let message =
                format!("Missing parameter ${} ({}) in call to {callee}.", acceptor.names[p], acceptor.types[p]);
            c.report(at.span(), "argument.missing", message, None);
        }
    }
    parameters
}
