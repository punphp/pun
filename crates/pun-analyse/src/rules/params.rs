//! FunctionCallParametersCheck, the part level 0 runs: argument order, counts and names, and void results used.

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::TType;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

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
pub fn select(variants: Variants, counts: &Counts) -> Acceptor {
    let variants = match variants.named {
        Some(named) if counts.named => named,
        _ => variants.all,
    };
    if variants.len() == 1 {
        return variants.into_iter().next().expect("one");
    }
    let n = counts.positional + counts.names;
    let acceptable: Vec<&Acceptor> = variants.iter().filter(|v| counts.unpack || v.accepts(n)).collect();
    if let Some(first) = acceptable.first() {
        // ponytail: several acceptable variants are told apart by argument types in PHPStan; the first wins here
        return (*first).clone();
    }
    let min = variants.iter().map(Acceptor::min).min().unwrap_or(0);
    let mut combined = Acceptor {
        optional: Vec::new(),
        variadic: false,
        void: variants.iter().all(|v| v.void),
        names: Vec::new(),
        types: Vec::new(),
        variadics: Vec::new(),
        builtin: variants.iter().any(|v| v.builtin),
    };
    for v in &variants {
        combined.variadic |= v.variadic;
        for i in 0..v.names.len() {
            if i == combined.names.len() {
                combined.names.push(v.names[i].clone());
                combined.types.push(v.types[i].clone());
                combined.variadics.push(v.variadics[i]);
                combined.optional.push(i + 1 > min);
                continue;
            }
            // ponytail: the union of the types prints both when they differ, without TypeCombinator's normalization
            if combined.names[i] != v.names[i] {
                combined.names[i] = format!("{}|{}", combined.names[i], v.names[i]);
            }
            if combined.types[i] != v.types[i] {
                combined.types[i] = format!("{}|{}", combined.types[i], v.types[i]);
            }
            let variadic = combined.variadics[i] || v.variadics[i];
            combined.variadics[i] = variadic;
            if variadic {
                // the variadic parameter swallows every parameter behind it, and their types
                let dropped: Vec<String> = combined.types[i + 1..]
                    .iter()
                    .chain(&v.types[i + 1..])
                    .cloned()
                    .collect();
                for t in dropped {
                    if !combined.types[i].split('|').any(|m| m == t) {
                        combined.types[i] = format!("{}|{}", combined.types[i], t);
                    }
                }
                combined.names.truncate(i + 1);
                combined.types.truncate(i + 1);
                combined.variadics.truncate(i + 1);
                combined.optional.truncate(i + 1);
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
        self.named = true;
        self.names += 1;
        self.args.push(Arg {
            name: Some(String::from_utf8_lossy(n.name.value).into_owned()),
            unpack: None,
            span: n.span(),
        });
    }

    fn positional(&mut self, p: &PositionalArgument) {
        let unpack = p.ellipsis.map(|_| unpacked(p.value));
        match &unpack {
            None => self.positional += 1,
            Some(Unpacked::Keys(keys)) => {
                self.positional += keys.iter().filter(|k| k.is_none()).count();
                self.named |= keys.iter().any(Option::is_some);
            }
            Some(Unpacked::Unknown) => self.unpack = true,
        }
        self.args.push(Arg { name: None, unpack, span: p.span() });
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
}

/// The messages of one call kind: `subject` is e.g. `Function foo`, `Method Foo::bar()`, `Class Foo constructor`.
pub struct Call<'s> {
    pub kind: CallKind,
    pub subject: &'s str,
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
    // the arguments after unpacking constant arrays: (name, unpacks a non-constant array, where)
    let mut arguments: Vec<(Option<&str>, bool, Span)> = Vec::new();
    let (mut has_named, mut has_unpacked) = (false, false);
    for arg in &counts.args {
        has_named |= arg.name.is_some();
        if has_named && arg.unpack.is_some() {
            let message = "Named argument cannot be followed by an unpacked (...) argument.".to_string();
            c.report_non_ignorable(arg.span, "argument.unpackAfterNamed", message, None);
        }
        // PHP 8.1 allows a named argument after an unpacked one
        if has_unpacked && arg.unpack.is_none() && (arg.name.is_none() || c.php_version < 80100) {
            let message = "Unpacked argument (...) cannot be followed by a non-unpacked argument.".to_string();
            c.report_non_ignorable(arg.span, "argument.nonUnpackAfterUnpacked", message, None);
        }
        match &arg.unpack {
            None => arguments.push((arg.name.as_deref(), false, arg.span)),
            Some(Unpacked::Keys(keys)) => {
                has_unpacked = true;
                has_named |= keys.iter().any(Option::is_some);
                arguments.extend(keys.iter().map(|k| (k.as_deref(), false, arg.span)));
            }
            Some(Unpacked::Unknown) => {
                has_unpacked = true;
                arguments.push((None, true, arg.span));
            }
        }
    }
    if has_named && c.php_version < 80000 && call.kind != CallKind::Attribute {
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
        // ponytail: level 0 has checkExtraArguments off, so only too few arguments are counted
        if invoked < min {
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
            _ => "staticMethod.void",
        };
        c.report(at, identifier, void, None);
    }
    process_arguments(c, acceptor, &arguments, has_named, at, &call);
}

/// `processArguments()`: each argument's parameter, by position or by name.
fn process_arguments(
    c: &mut Checker,
    acceptor: &Acceptor,
    arguments: &[(Option<&str>, bool, Span)],
    has_named: bool,
    at: impl HasSpan,
    call: &Call,
) {
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
    for (i, (name, unpack, span)) in arguments.iter().enumerate() {
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
            c.report(*span, "argument.duplicate", message, None);
            continue;
        }
        unused.retain(|p| *p != parameter);
    }
    if has_named {
        for p in unused.into_iter().filter(|p| !acceptor.optional[*p]) {
            let message =
                format!("Missing parameter ${} ({}) in call to {callee}.", acceptor.names[p], acceptor.types[p]);
            c.report(at.span(), "argument.missing", message, None);
        }
    }
}
