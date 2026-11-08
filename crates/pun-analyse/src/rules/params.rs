//! FunctionCallParametersCheck: shared argument ordering, counts, binding and named-argument policy. Type,
//! reference and @no-named-arguments checks run from level 5, after the level-0 binding checks.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::TType;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use pun_phpstan::Key;
use pun_phpstan::symbols::{ByRef, ConstantReflection, ParameterAllowedConstants};
use pun_phpstan::types::{Tri, Type, is_super_type_of};

use super::Checker;
use crate::bitset::BitSet;

/// A ParametersAcceptor, as far as counting and naming go.
#[derive(Clone, Debug, PartialEq)]
pub struct Acceptor {
    /// `isOptional()` per parameter.
    pub optional: BitSet,
    /// `isVariadic()`: a variadic parameter, or a body calling `func_get_args()`.
    pub variadic: bool,
    /// The return type is `void` (`getKeepVoidType()`).
    pub void: bool,
    /// Per parameter: the name, the type's description and whether it is variadic.
    pub names: Vec<String>,
    pub types: Vec<String>,
    pub variadics: BitSet,
    /// A built-in's (no named argument goes to its variadic parameter).
    pub builtin: bool,
    /// `acceptsNamedArguments()`, retained when acceptors are selected or combined.
    pub accepts_named_arguments: Tri,
    /// Per parameter: its declared type (see [`ParamType`]) and `passedByReference()`.
    pub param_types: Vec<ParamType>,
    pub by_ref: Vec<ByRef>,
    /// Whether `getClosureThisType()` is non-null, retained through selection and combination.
    pub closure_this: BitSet,
    /// A method's reflection context, which `self`/`static` in its parameter types name. A trait method's context
    /// is the consuming class, not its storage owner.
    pub class: Option<String>,
    pub callee: Option<UserCallee>,
}

/// A user's storage identity stays separate from the reflected class and method name (trait aliases included).
#[derive(Clone, Debug, PartialEq)]
pub struct UserCallee {
    pub class: Option<String>,
    pub name: String,
    pub reflected_name: String,
}

/// A parameter's type as its reflection declares it, converted when an argument type is checked against it.
#[derive(Clone, Debug, PartialEq)]
pub enum ParamType {
    /// A built-in's PHPDoc type string and allowed-constant policy (data/symbols.json).
    Builtin(String, Option<ParameterAllowedConstants>),
    /// A user parameter, typed from its function's reflection (`declared::codex_params()`).
    User,
    /// A callable type's parameter.
    Typed(pun_phpstan::types::Type),
}

/// The source reflection's policy, also for a first-class callable whose signature has lost its callee identity.
pub(super) fn accepts_named(c: &Checker, f: &FunctionLikeMetadata, class: Option<&ClassLikeMetadata>) -> Tri {
    let yes = match class {
        Some(class) => super::named_argument::accepts(c, class, f),
        None => !super::declared::doc_at(c, f.span)
            .is_some_and(|(_, entry, _)| super::declarations::has_tag(&entry.names, &["@no-named-arguments"])),
    };
    Tri::from(yes)
}

impl Acceptor {
    /// Resolve a user reflection's PHPDoc before selection can discard its callee identity. Methods share the
    /// inheritance and declaring-class contract used by the named-argument collector.
    pub(super) fn accepts_named(&self, c: &Checker) -> Tri {
        let accepts = self.user(c).zip(self.callee.as_ref()).map(|((f, class), callee)| match class {
            Some(class) => if super::declared::reflected_accepts_named(c, class, &callee.reflected_name) {
                Tri::Yes
            } else {
                Tri::No
            },
            None => accepts_named(c, f, None),
        });
        accepts.unwrap_or(self.accepts_named_arguments)
    }

    /// The stored declaration and consuming-class context used by PHPStan's method reflection.
    pub(super) fn user<'c>(&self, c: &'c Checker) -> Option<(&'c FunctionLikeMetadata, Option<&'c ClassLikeMetadata>)> {
        let callee = self.callee.as_ref()?;
        let f = match &callee.class {
            Some(class) => c.codebase.get_method(class.as_bytes(), callee.name.as_bytes())?,
            None => c.codebase.get_function(callee.name.as_bytes())?,
        };
        Some((f, self.class.as_ref().and_then(|name| c.codebase.get_class_like(name.as_bytes()))))
    }

    fn min(&self) -> usize {
        self.optional.iter().filter(|o| !o).count()
    }

    fn accepts(&self, n: usize) -> bool {
        n >= self.min() && (self.variadic || n <= self.optional.len())
    }

    /// A user-defined function-like (Mago's metadata): a parameter is optional when it and every later one has a
    /// default or is variadic, as `ReflectionParameter::isOptional()` says.
    pub fn of(f: &FunctionLikeMetadata) -> Acceptor {
        let mut optional = BitSet::new(f.parameters.len());
        let mut tail = true;
        for (i, p) in f.parameters.iter().enumerate().rev() {
            tail &= p.flags.has_default() || p.flags.is_variadic();
            optional.set(i, tail);
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
            accepts_named_arguments: Tri::Yes,
            param_types: vec![ParamType::User; f.parameters.len()],
            // PhpParameterReflection: a by-reference parameter of user code creates a new variable
            by_ref: f
                .parameters
                .iter()
                .map(|p| if p.flags.is_by_reference() { ByRef::CreatesNewVariable } else { ByRef::No })
                .collect(),
            closure_this: f.parameters.iter().map(|p| p.closure_this_type.is_some()).collect(),
            class: None,
            callee: Some(UserCallee {
                class: None,
                name: f.name.to_string(),
                reflected_name: f.name.to_string(),
            }),
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
            accepts_named_arguments: Tri::Yes,
            param_types: v.params.iter().map(|p| ParamType::Builtin(p.ty.clone(), p.allowed_constants.clone())).collect(),
            by_ref: v.params.iter().map(|p| p.by_ref).collect(),
            closure_this: BitSet::new(v.params.len()),
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

/// `ParametersAcceptorSelector::selectFromArgs()`: named variants, arity filtering and type-based winners.
pub fn select(c: &Checker, variants: Variants, counts: &Counts) -> Acceptor {
    select_in(c, variants, counts, None)
}

/// ArgumentsHandler's count-stable structural metadata selection. Ordinary arguments consume reference,
/// name and variadic policy before their values are evaluated, so every written argument contributes mixed.
pub(super) fn select_metadata(c: &Checker, variants: Variants, counts: &Counts) -> Acceptor {
    let types: Vec<_> = counts.args.iter().enumerate().map(|(i, arg)| {
        (arg.name.clone().map_or(Key::Int(i as i64), Key::Str), Type::Mixed { explicit: false })
    }).collect();
    select_from_types(c, variants, counts, Some(&types))
}

/// A dynamic method's acceptor is selected using that name's filtered argument types.
pub(super) fn select_in(c: &Checker, variants: Variants, counts: &Counts, scope: Option<&super::infer::NamedCallScope>) -> Acceptor {
    // selectFromArgs builds an ordered int|string keyed map, not the checker's bound argument list. All constant
    // alternatives (including optional keys) contribute, and numeric indexes start at the written argument index.
    let types = (|| {
        let mut types: Vec<(Key, Type)> = Vec::new();
        for (i, arg) in counts.args.iter().enumerate() {
            let ty = match scope {
                Some(scope) => c.phpstan_type_at_in(arg.value, &scope.types),
                None => c.phpstan_type_at_in(arg.value, &c.types),
            }?;
            let index = arg.name.clone().map_or(Key::Int(i as i64), Key::Str);
            let mut union_existing = false;
            let values = if arg.unpack.is_some() {
                let arrays = ty.constant_arrays();
                if arrays.is_empty() {
                    vec![(index, ty.iterable_value_type())]
                } else {
                    union_existing = true;
                    arrays.into_iter().flat_map(|array| array.keys.into_iter().zip(array.values).enumerate().map(move |(j, (key, value))| {
                        let index = match key {
                            Type::ConstString(key) => Key::Str(String::from_utf8_lossy(&key).into_owned()),
                            _ => Key::Int((i + j) as i64),
                        };
                        (index, value)
                    })).collect()
                }
            } else {
                vec![(index, ty)]
            };
            for (index, ty) in values {
                if let Some((_, previous)) = types.iter_mut().find(|(key, _)| *key == index) {
                    *previous = if union_existing { pun_phpstan::types::union(vec![previous.clone(), ty]) } else { ty };
                } else {
                    types.push((index, ty));
                }
            }
        }
        Some(types)
    })();
    select_from_types(c, variants, counts, types.as_deref())
}

fn select_from_types(c: &Checker, variants: Variants, counts: &Counts, types: Option<&[(Key, Type)]>) -> Acceptor {
    let named = types.map_or(counts.named, |types| types.iter().any(|(key, _)| matches!(key, Key::Str(_))));
    let variants = match variants.named {
        Some(variants) if named => variants,
        _ => variants.all,
    };
    let variants: Vec<_> = variants.into_iter().map(|mut v| {
        v.accepts_named_arguments = v.accepts_named(c);
        if let (Some((f, class)), Some(callee)) = (v.user(c), v.callee.as_ref()) {
            let bindings = super::declared::reflected_closure_this(c, f, class, &callee.reflected_name);
            v.closure_this = v.names.iter().map(|name| bindings.contains_key(name)).collect();
        }
        v
    }).collect();
    if variants.len() == 1 {
        return variants.into_iter().next().expect("one");
    }
    let n = counts.positional + counts.names;
    // selectFromTypes bypasses count filtering for every unpack, including literal constant arrays.
    let unpack = counts.args.iter().any(|arg| arg.unpack.is_some());
    let acceptable: Vec<&Acceptor> = variants.iter().filter(|v| unpack || v.accepts(n)).collect();
    match acceptable.as_slice() {
        [] => combine(&variants.iter().collect::<Vec<_>>()),
        [one] => (*one).clone(),
        // Missing selected operands remain unknown; never recover them from the global map.
        [first, ..] => match types.map(|types| winning(c, &acceptable, types, unpack)) {
            Some(w) if w.is_empty() => combine(&acceptable),
            Some(w) => combine(&w),
            None => (*first).clone(),
        },
    }
}

/// `selectFromTypes()`'s winning acceptors: those whose parameters are supertypes of the arguments' types with the
/// highest certainty. An unpack repeats the last inserted type when a parameter has no positional key.
fn winning<'v>(c: &Checker, acceptable: &[&'v Acceptor], types: &[(Key, Type)], unpack: bool) -> Vec<&'v Acceptor> {
    let rank = |t: Tri| [Tri::No, Tri::Maybe, Tri::Yes].iter().position(|x| *x == t);
    let scored: Vec<_> = acceptable
        .iter()
        .map(|a| {
            let mut sup = Tri::Yes;
            for i in 0..a.names.len() {
                let entry = types.iter().find(|(key, _)| *key == Key::Int(i as i64))
                    .or_else(|| if unpack { types.last() } else { None });
                let Some((_, ty)) = entry else { break };
                // GenericParametersAcceptorResolver resolves a template from the argument, an uninferred one to its
                // bound (bug 7048); other late types are taken as yes
                let resolved = super::types::declared_type(c, a, i).map(|p| {
                    super::declared::with_classes(c, || {
                        let templates = std::cell::RefCell::new(Vec::new());
                        fn collect(t: &Type, out: &std::cell::RefCell<Vec<pun_phpstan::types::Template>>) {
                            if let Type::Template(tt) = t { out.borrow_mut().push((**tt).clone()); }
                            t.traverse(&|i| { collect(i, out); i.clone() });
                        }
                        collect(&p, &templates);
                        let templates = templates.into_inner();
                        if templates.is_empty() {
                            return p;
                        }
                        let map = pun_phpstan::types::infer_template_types(&p, ty)
                            .with_errors(&templates.iter().collect::<Vec<_>>());
                        pun_phpstan::types::resolve_template_types(&p, &map)
                    })
                });
                sup = sup.and(match resolved {
                    None | Some(Type::Mixed { .. }) => Tri::Maybe,
                    Some(p) if super::types::is_late(&p) => Tri::Yes,
                    Some(p) => super::declared::with_classes(c, || is_super_type_of(&p, ty)),
                });
            }
            (rank(sup), *a)
        })
        .filter(|(r, _)| *r > rank(Tri::No))
        .collect();
    let best = scored.iter().map(|(r, _)| *r).max();
    scored.into_iter().filter(|(r, _)| Some(*r) == best).map(|(_, a)| a).collect()
}

/// `combineAcceptors()`.
pub(super) fn combine(variants: &[&Acceptor]) -> Acceptor {
    let min = variants.iter().map(|v| v.min()).min().unwrap_or(0);
    let mut combined = Acceptor {
        optional: BitSet::default(),
        variadic: false,
        void: variants.iter().all(|v| v.void),
        names: Vec::new(),
        types: Vec::new(),
        variadics: BitSet::default(),
        builtin: variants.iter().any(|v| v.builtin),
        accepts_named_arguments: variants.iter().fold(Tri::No, |accepts, v| accepts.or(v.accepts_named_arguments)),
        param_types: Vec::new(),
        by_ref: Vec::new(),
        closure_this: BitSet::default(),
        class: variants.first().and_then(|v| v.class.clone()),
        callee: None,
    };
    for v in variants {
        combined.variadic |= v.variadic;
        for i in 0..v.names.len() {
            if i == combined.names.len() {
                combined.names.push(v.names[i].clone());
                combined.types.push(v.types[i].clone());
                combined.variadics.push(v.variadics.get(i));
                combined.optional.push(i + 1 > min);
                combined.param_types.push(v.param_types[i].clone());
                combined.by_ref.push(v.by_ref[i]);
                combined.closure_this.push(v.closure_this.get(i));
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
                (ParamType::Builtin(a, allowed), ParamType::Builtin(b, other)) => ParamType::Builtin(
                    if a == b { a.clone() } else { format!("{a}|{b}") },
                    allowed.clone().filter(|_| allowed == other),
                ),
                (ParamType::Builtin(a, _), _) => ParamType::Builtin(a.clone(), None),
                (a, _) => a.clone(),
            };
            // A combined parameter keeps a binding only if both parameters have one (the types are unioned).
            let bound = combined.closure_this.get(i) && v.closure_this.get(i);
            combined.closure_this.set(i, bound);
            // `PassedByReference::combine()`: the stronger
            let rank = |b: ByRef| [ByRef::No, ByRef::ReadsArgument, ByRef::CreatesNewVariable].iter().position(|x| *x == b);
            combined.by_ref[i] = std::cmp::max_by_key(combined.by_ref[i], v.by_ref[i], |b| rank(*b));
            let variadic = combined.variadics.get(i) || v.variadics.get(i);
            combined.variadics.set(i, variadic);
            if variadic {
                // the variadic parameter swallows every parameter behind it
                // ponytail: upstream also merges their types into its type; a variadic parameter is never missing
                combined.names.truncate(i + 1);
                combined.types.truncate(i + 1);
                combined.variadics.truncate(i + 1);
                combined.optional.truncate(i + 1);
                combined.param_types.truncate(i + 1);
                combined.by_ref.truncate(i + 1);
                combined.closure_this.truncate(i + 1);
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
    /// RuleLevelHelper::isThis() before argument normalization loses the expression.
    pub is_this: bool,
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
    /// A constant fetch, or a disjunction whose every leaf is a constant fetch.
    Constants(Vec<ConstantFetch>),
    /// A syntactically static anonymous function or arrow, not a variable containing one.
    StaticClosure,
    Other,
}

#[derive(Clone, Debug)]
pub enum ConstantFetch {
    Global { name: Span, unqualified: bool },
    Class { class: Span, named: bool, name: String },
}

/// `resolveConstantReflections()`'s syntax contract, before resolving the leaves in the call's scope.
fn constant_fetches(e: &Expression) -> Option<Vec<ConstantFetch>> {
    match e {
        Expression::Parenthesized(p) => constant_fetches(p.expression),
        Expression::ConstantAccess(a) => Some(vec![ConstantFetch::Global {
            name: a.name.span(),
            unqualified: matches!(a.name, Identifier::Local(_)),
        }]),
        Expression::Access(Access::ClassConstant(a)) => {
            let ClassLikeConstantSelector::Identifier(id) = &a.constant else { return None };
            if !matches!(a.class, Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) {
                return None;
            }
            Some(vec![ConstantFetch::Class {
                class: a.class.span(),
                named: matches!(a.class, Expression::Identifier(_)),
                name: String::from_utf8_lossy(id.value).into_owned(),
            }])
        }
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::BitwiseOr(_)) => {
            let mut left = constant_fetches(b.lhs)?;
            left.extend(constant_fetches(b.rhs)?);
            Some(left)
        }
        _ => None,
    }
}

fn selector(s: &ClassLikeMemberSelector) -> Option<Vec<u8>> {
    match s {
        ClassLikeMemberSelector::Identifier(i) => Some(i.value.to_vec()),
        _ => None,
    }
}

fn kind(e: &Expression) -> ArgKind {
    if let Some(constants) = constant_fetches(e) {
        return ArgKind::Constants(constants);
    }
    if super::assign::contains_nullsafe(e) {
        return ArgKind::Nullsafe;
    }
    match e {
        Expression::Parenthesized(p) => kind(p.expression),
        Expression::Closure(f) if f.r#static.is_some() => ArgKind::StaticClosure,
        Expression::ArrowFunction(f) if f.r#static.is_some() => ArgKind::StaticClosure,
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
            is_this: Checker::is_this_expression(value),
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
            is_this: Checker::is_this_expression(value),
            elements,
        });
    }
}

/// A constant array element's value expression (only values and key-values make one).
pub(super) fn element(e: &ArrayElement) -> Span {
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
                Expression::Literal(Literal::String(s)) => {
                    let key = String::from_utf8_lossy(s.value.unwrap_or_default()).into_owned();
                    (!key.parse::<i64>().is_ok_and(|i| i.to_string() == key)).then_some(key)
                }
                _ => None,
            }),
            _ => return Unpacked::Unknown,
        }
    }
    Unpacked::Keys(keys)
}

/// `getConstantArrays()` for unpacking: every union alternative must be constant. Keep the shortest
/// alternative's positions, common names and the last alternative's optionality, as the upstream loop does.
fn unpacked_types(ty: &Type) -> Option<Vec<(Option<String>, Type)>> {
    fn arrays(ty: &Type) -> Option<Vec<&pun_phpstan::types::ConstArray>> {
        match ty {
            Type::ConstArray(a) => Some(vec![a]),
            Type::Union(ts) | Type::Benevolent(ts) => {
                Some(ts.iter().map(arrays).collect::<Option<Vec<_>>>()?.concat())
            }
            Type::Template(t) => arrays(&t.bound),
            _ => None,
        }
    }
    let arrays = arrays(ty)?;
    let count = arrays.iter().map(|a| {
        if a.unsealed.as_ref().is_some_and(|(key, _)| !matches!(key, Type::Never(_))) { 0 } else { a.keys.len() }
    }).min()?;
    Some((0..count).filter_map(|i| {
        let key = &arrays[0].keys[i];
        let name = match key {
            Type::ConstString(s) if arrays.iter().all(|a| a.keys[i] == *key) => Some(String::from_utf8_lossy(s).into_owned()),
            _ => None,
        };
        if name.is_none() && arrays.last().unwrap().optional.contains(&i) {
            return None;
        }
        Some((name, pun_phpstan::types::union(arrays.iter().map(|a| a.values[i].clone()).collect())))
    }).collect())
}

/// The calls FunctionCallParametersCheck checks.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CallKind {
    Function,
    /// CallUserFuncRule uses function identifiers, but passes `isBuiltin: false` for the forwarded call.
    CallUserFunc,
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
    /// The operand-position scope's logical call, separate from the selector used for diagnostics.
    pub scope: Option<Span>,
    /// `Result of function foo (void) is used.`; `None` for `new`.
    pub void_message: Option<String>,
}

/// `counts` is [`count()`] of the call's arguments.
pub fn check(
    c: &mut Checker,
    acceptor: &Acceptor,
    counts: &Counts,
    at: impl HasSpan + Copy,
    call: Call,
    first_level: bool,
) {
    let scope = call.scope.and_then(|span| c.named_call_scopes.get(&span))
        .and_then(|scopes| scopes.iter().find(|scope| !scope.filtered)).cloned();
    check_in(c, acceptor, counts, at, call, first_level, scope.as_ref());
}

/// FunctionCallParametersCheck in one owned operand scope; absent selected operands stay absent.
pub(super) fn check_in(
    c: &mut Checker,
    acceptor: &Acceptor,
    counts: &Counts,
    at: impl HasSpan + Copy,
    call: Call,
    first_level: bool,
    scope: Option<&super::infer::NamedCallScope>,
) {
    check_bound_in(c, acceptor, counts, at, call, first_level, scope, true);
}

/// The structural binding boundary for reflections whose semantic signature is not yet representable. Counts,
/// ordering, unknown/missing/duplicate names and method-level named policy use the ordinary shared consumer.
/// Do not run type/reference/template or return checks against an arbitrary combined User parameter.
pub(super) fn check_bound_in(
    c: &mut Checker,
    acceptor: &Acceptor,
    counts: &Counts,
    at: impl HasSpan + Copy,
    call: Call,
    first_level: bool,
    scope: Option<&super::infer::NamedCallScope>,
    semantic: bool,
) {
    // the arguments after unpacking constant arrays
    let unpacked: Vec<_> = counts.args.iter().map(|arg| {
        (arg.unpack.is_some() && (scope.is_some_and(|scope| scope.filtered)
            || matches!(arg.unpack, Some(Unpacked::Unknown)))).then(|| {
            match scope {
                Some(scope) => c.phpstan_type_at_in(arg.value, &scope.types),
                None => c.types.get(&arg.value).map(|t| c.to_phpstan(t)),
            }
        }).flatten()
    }).collect();
    let inferred: Vec<_> = unpacked.iter().map(|ty| ty.as_ref().and_then(unpacked_types)).collect();
    // the fix of every `named argument $x` error of the call
    let positional_fix = !acceptor.accepts_named_arguments.yes()
        && counts.args.iter().any(|a| a.name.is_some())
        && positional_arguments_fix(c, acceptor, &counts.args, call.kind);
    let mut arguments: Vec<Entry> = Vec::new();
    let (mut has_named, mut has_unpacked) = (false, false);
    for ((arg, inferred), unpacked) in counts.args.iter().zip(&inferred).zip(&unpacked) {
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
        let entry = |name, unpack, value| Entry {
            name,
            unpack,
            span: arg.span,
            value,
            ty: None,
            scope: scope.filter(|scope| scope.filtered),
            kind: &arg.kind,
            is_this: arg.is_this,
        };
        match &arg.unpack {
            None => arguments.push(entry(arg.name.as_deref(), false, arg.value)),
            Some(Unpacked::Keys(keys)) if !scope.is_some_and(|scope| scope.filtered) => {
                has_unpacked = true;
                has_named |= keys.iter().any(Option::is_some);
                arguments.extend(
                    keys.iter()
                        .zip(&arg.elements)
                        .map(|(k, v)| entry(k.as_deref(), false, *v)),
                );
            }
            Some(_) => {
                has_unpacked = true;
                if let Some(inferred) = inferred {
                    has_named |= inferred.iter().any(|(name, _)| name.is_some());
                    arguments.extend(inferred.iter().map(|(name, ty)| Entry {
                        ty: Some(ty.clone()),
                        ..entry(name.as_deref(), false, arg.value)
                    }));
                    // An open shape expands no concrete positions, but a required key still guarantees an unpack.
                    if arguments.is_empty()
                        && unpacked.as_ref().is_some_and(|ty| ty.is_iterable_at_least_once().yes())
                    {
                        arguments.push(entry(None, true, arg.value));
                    }
                } else {
                    arguments.push(entry(None, true, arg.value));
                }
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
        let invoked = if arguments.iter().any(|e| e.unpack) {
            max.map_or(min, |m| m.max(min))
        } else if inferred.iter().any(Option::is_some) {
            arguments.len()
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
    if semantic && let Some(void) = call.void_message.clone()
        && !first_level
        && call.scope.and_then(|span| c.keep_void_type_at(span)).map_or(acceptor.void, |ty| {
            // NullsafeMethodCallHandler checks the ensured plain twin, before adding short-circuit null.
            // The provenance flag excludes null declared by any actual target.
            ty.is_void() || call.kind == CallKind::Method && ty.has_nullsafe_null() && ty.remove_null().is_void()
        })
    {
        let identifier = match call.kind {
            CallKind::Function | CallKind::CallUserFunc => "function.void",
            CallKind::Method => "method.void",
            CallKind::Callable => "callable.void",
            _ => "staticMethod.void",
        };
        c.report(at, identifier, void, None);
    }
    let parameters = process_arguments(c, acceptor, &arguments, has_named, at, &call);
    if c.on(crate::rules::Switches::CHECK_ARGUMENT_TYPES) || c.on(crate::rules::Switches::CHECK_ARGUMENTS_PASSED_BY_REFERENCE) {
        if semantic {
            super::types::arguments(c, acceptor, &arguments, &parameters, at.span(), &call, positional_fix);
        } else {
            for (argument, parameter) in arguments.iter().zip(parameters) {
                if parameter.is_some() {
                    check_named(c, acceptor, argument, &call, positional_fix);
                }
            }
        }
    }
}

/// The per-bound-argument `@no-named-arguments` check. Like upstream, it runs only after processArguments and
/// only when argument type/reference checking is enabled, immediately before that argument's type checks.
/// `positional_fix`: the call's named arguments can be made positional ([`positional_arguments_fix()`]).
pub(super) fn check_named(c: &mut Checker, acceptor: &Acceptor, e: &Entry, call: &Call, positional_fix: bool) {
    if acceptor.accepts_named_arguments.yes() {
        return;
    }
    let fixable = e.name.is_some() && positional_fix;
    let argument = if let Some(name) = e.name {
        format!("named argument ${name}")
    } else if e.unpack {
        let ty = match e.scope {
            Some(_) => e.value_type(c),
            None => c.types.get(&e.value).map(|t| c.to_phpstan(t)),
        };
        let key = ty.map(|t| {
            let key = t.iterable_key_type();
            // This is a certainty query, not lenient argument acceptance. BenevolentUnionType inherits
            // UnionType::isString(), so an int|string array key is still only possibly a string.
            match key {
                Type::Benevolent(parts) => Type::Union(parts).is_string(),
                key => key.is_string(),
            }
        }).unwrap_or(Tri::Maybe);
        if key.no() {
            return;
        }
        let key = if key.yes() { "string key" } else { "possibly string key" };
        format!("unpacked array with {key}")
    } else {
        return;
    };
    let count = c.errors.len();
    c.report(e.span, "argument.named", format!(
        "{} invoked with {argument}, but it's not allowed because of @no-named-arguments.", call.subject
    ), None);
    if fixable {
        c.fixable_since(count);
    }
}

/// FunctionCallParametersCheck::createPositionalArgumentsFix() is not false: no argument is unpacked, each named
/// one names a different non-variadic parameter, and each skipped parameter before the last one passed is optional,
/// not by reference, with a default createDefaultValueExpr() can write. A call made from an attribute is not in
/// the file.
fn positional_arguments_fix(c: &Checker, acceptor: &Acceptor, args: &[Arg], kind: CallKind) -> bool {
    if kind == CallKind::Attribute {
        return false;
    }
    let mut positions: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let mut last = 0;
    for (i, arg) in args.iter().enumerate() {
        if arg.unpack.is_some() {
            return false;
        }
        let Some(name) = &arg.name else {
            positions.insert(i, i);
            last = last.max(i);
            continue;
        };
        // `$parameterPositions[$name]`: the last parameter of the name
        let Some(position) = acceptor.names.iter().rposition(|n| n == name) else {
            return false;
        };
        // a named argument that is collected by the variadic parameter keeps its string key
        if acceptor.variadics.get(position) || positions.insert(position, i).is_some() {
            return false;
        }
        last = last.max(position);
    }
    let user = acceptor.user(c);
    (0..last).filter(|p| !positions.contains_key(p)).all(|p| {
        if !acceptor.optional.get(p) || acceptor.by_ref.get(p) != Some(&ByRef::No) {
            return false;
        }
        // ponytail: a built-in's default (data/symbols.json's type string) is not read; such a call is not fixable
        let Some(default) = user.and_then(|(f, _)| f.parameters.get(p)?.default_type.as_ref()) else {
            return false;
        };
        // createDefaultValueExpr()
        let ty = c.to_phpstan(&default.type_union);
        (ty.is_constant_array().yes() && ty.is_iterable_at_least_once().no())
            || matches!(
                ty,
                Type::Null | Type::ConstBool(_) | Type::ConstInt(_) | Type::ConstFloat(_) | Type::ConstString(_)
            )
    })
}

/// The declared parameter types with a policy, for the built-in fallback check on other parameters.
pub(super) fn allowed_constants_type(c: &Checker, acceptor: &Acceptor) -> Option<Type> {
    let types: Vec<_> = acceptor.param_types.iter().enumerate().filter_map(|(i, p)| {
        matches!(p, ParamType::Builtin(_, Some(_))).then(|| super::types::declared_type(c, acceptor, i)).flatten()
    }).collect();
    (!types.is_empty()).then(|| pun_phpstan::types::union(types))
}

/// FunctionCallParametersCheck's PHP 8+ constant checks, after this argument's ordinary type check.
pub(super) fn check_constants(
    c: &mut Checker,
    acceptor: &Acceptor,
    e: &Entry,
    p: usize,
    allowed_type: Option<&Type>,
    parameter_type: Option<&Type>,
    parameter: &str,
    call: &Call,
) {
    if !c.surely_at_least(80000) {
        return;
    }
    let ArgKind::Constants(fetches) = e.kind else { return };
    let Some(constants) = fetches.iter().map(|f| resolve_constant(c, f)).collect::<Option<Vec<_>>>() else { return };
    let policy = match &acceptor.param_types[p] {
        ParamType::Builtin(_, policy) => policy.as_ref(),
        _ => None,
    };
    let of = call.subject[..1].to_ascii_lowercase() + &call.subject[1..];
    let parameter = parameter[..1].to_ascii_lowercase() + &parameter[1..];
    let invalid = |c: &mut Checker, constant: &ConstantReflection| {
        c.report(e.span, "argument.invalidConstant", format!(
            "Constant {} is not allowed for {parameter} of {of}.", constant.name
        ), None);
    };
    if let Some(policy) = policy {
        let result = policy.check(&constants);
        for constant in result.disallowed {
            invalid(c, constant);
        }
        for group in result.violated_groups {
            c.report(e.span, "argument.exclusiveConstants", format!(
                "Constants {} cannot be combined for {parameter} of {of}.", group.join(", ")
            ), None);
        }
        if result.bitmask_not_allowed {
            c.report(e.span, "argument.bitmaskNotAllowed", format!(
                "Combining constants with | is not allowed for {parameter} of {of}."
            ), None);
        }
    } else if acceptor.builtin && !matches!(call.kind, CallKind::Callable | CallKind::CallUserFunc) && allowed_type.zip(parameter_type).is_some_and(|(a, p)| {
        super::declared::with_classes(c, || is_super_type_of(a, p).yes())
    }) {
        // only a constant another parameter of the function takes is reported here
        let policies: Vec<_> = acceptor.param_types.iter().filter_map(|p| match p {
            ParamType::Builtin(_, Some(policy)) => Some(policy),
            _ => None,
        }).collect();
        for constant in constants.iter().filter(|k| k.builtin) {
            let one = std::slice::from_ref(constant);
            if policies.iter().any(|policy| policy.check(one).is_ok()) {
                invalid(c, constant);
            }
        }
    }
}

/// Resolve syntax, not inferred numeric values. A single unresolved leaf cancels the whole disjunction.
fn resolve_constant(c: &Checker, fetch: &ConstantFetch) -> Option<ConstantReflection> {
    match fetch {
        ConstantFetch::Global { name, unqualified } => {
            let resolved = c.resolved_span(*name);
            let written = c.written(*name).trim_start_matches('\\').to_string();
            let fallback = *unqualified && !c.names.is_imported(name);
            for name in std::iter::once(resolved).chain(fallback.then_some(written)) {
                if ["null", "true", "false"].iter().any(|n| name.eq_ignore_ascii_case(n)) {
                    return None;
                }
                if let Some(k) = c.codebase.get_constant(name.as_bytes()).filter(|k| !k.flags.is_built_in()) {
                    // As in ReflectionProvider, a define() inside a class-like is not a global declaration.
                    if !c.codebase.class_likes.values().any(|o| {
                        o.span.file_id == k.span.file_id && o.span.start.offset <= k.span.start.offset && k.span.end.offset <= o.span.end.offset
                    }) {
                        return Some(ConstantReflection { name, builtin: false });
                    }
                }
                if c.builtins.constants.contains_key(&name) {
                    return Some(ConstantReflection { name, builtin: true });
                }
            }
            None
        }
        ConstantFetch::Class { class, named, name } => {
            let class = if *named {
                c.resolved_span(*class)
            } else {
                let scope = c.class_scope()?;
                if c.written(*class).eq_ignore_ascii_case("parent") {
                    scope.direct_parent_class?.to_string()
                } else {
                    scope.name.to_string()
                }
            };
            use super::reflection::{Known, known};
            let builtin = match known(c, &class)? {
                Known::User(k) => {
                    let key = mago_word::word(name.as_bytes());
                    if k.constants.get(&key).is_some_and(|k| !k.flags.is_built_in()) || k.enum_cases.contains_key(&key) {
                        return Some(ConstantReflection { name: format!("{class}::{name}"), builtin: false });
                    }
                    k.all_parent_classes.iter().chain(k.all_parent_interfaces.iter())
                        .filter_map(|n| c.builtins.classes.get(&n.to_string().to_ascii_lowercase()))
                        .find(|k| k.constants.iter().any(|(n, _)| n == name))?
                }
                Known::Builtin(k) => std::iter::once(k).chain(k.ancestors.iter().filter_map(|n| c.builtins.classes.get(&n.to_ascii_lowercase())))
                    .find(|k| k.constants.iter().any(|(n, _)| n == name) || k.cases.contains(name))?,
            };
            Some(ConstantReflection { name: format!("{}::{name}", builtin.name), builtin: true })
        }
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
    /// An inferred constant array's element type (there is no separate expression for it).
    pub ty: Option<Type>,
    /// A constant dynamic method's selected scope, never recovered from the global type map.
    pub scope: Option<&'a super::infer::NamedCallScope>,
    pub kind: &'a ArgKind,
    /// Only used for an unexpanded unpack, whose expression is still the original argument value.
    pub is_this: bool,
}

impl Entry<'_> {
    pub(super) fn type_at(&self, c: &Checker, span: Span) -> Option<Type> {
        match self.scope {
            Some(scope) => c.phpstan_type_at_in(span, &scope.types),
            None => c.phpstan_type_at_in(span, &c.types),
        }
    }

    pub(super) fn value_type(&self, c: &Checker) -> Option<Type> {
        self.ty.clone().or_else(|| self.type_at(c, self.value))
    }
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
    let natively_variadic = acceptor.variadics.iter().any(|v| v);
    let mut unused: Vec<usize> = (0..count).filter(|i| !acceptor.variadics.get(*i)).collect();
    let mut named_occurred = false;
    let mut named_for_variadic: Vec<&str> = Vec::new();
    for (i, e) in arguments.iter().enumerate() {
        let (name, unpack, span) = (&e.name, &e.unpack, &e.span);
        let parameter = match name {
            None if i < count => i,
            // func_get_args()
            None if !acceptor.variadic || count == 0 || !acceptor.variadics.get(count - 1) => break,
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
        if has_named && !acceptor.variadics.get(parameter) && !unused.contains(&parameter) {
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
        for p in unused.into_iter().filter(|p| !acceptor.optional.get(*p) && seen.insert(&acceptor.names[*p])) {
            let message =
                format!("Missing parameter ${} ({}) in call to {callee}.", acceptor.names[p], acceptor.types[p]);
            c.report(at.span(), "argument.missing", message, None);
        }
    }
    parameters
}
