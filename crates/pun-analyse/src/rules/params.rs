//! FunctionCallParametersCheck, the part level 0 runs: argument counts and void results used.

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::Checker;

/// A ParametersAcceptor, as far as counting goes.
#[derive(Clone, Debug, PartialEq)]
pub struct Acceptor {
    /// `isOptional()` per parameter.
    pub optional: Vec<bool>,
    pub variadic: bool,
    /// The return type is `void` (`getKeepVoidType()`).
    pub void: bool,
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
        }
    }

    /// A built-in, as PHPStan reflects it (data/symbols.json).
    pub fn of_builtin(v: &pun_phpstan::symbols::Variant) -> Acceptor {
        Acceptor {
            optional: v.params.iter().map(|p| p.optional).collect(),
            variadic: v.params.iter().any(|p| p.variadic),
            void: v.ret == "void",
        }
    }
}

/// `ParametersAcceptorSelector::selectFromArgs()` as far as the count goes: the one variant that accepts `n`
/// arguments, else `combineAcceptors()` of all.
pub fn select(variants: Vec<Acceptor>, n: usize, unpack: bool) -> Acceptor {
    if variants.len() == 1 {
        return variants.into_iter().next().expect("one");
    }
    let acceptable: Vec<&Acceptor> = variants.iter().filter(|v| unpack || v.accepts(n)).collect();
    if let Some(first) = acceptable.first() {
        // ponytail: several acceptable variants are told apart by argument types in PHPStan; the first wins here
        return (*first).clone();
    }
    let min = variants.iter().map(Acceptor::min).min().unwrap_or(0);
    let len = variants.iter().map(|v| v.optional.len()).max().unwrap_or(0);
    Acceptor {
        optional: (0..len).map(|i| i + 1 > min).collect(),
        variadic: variants.iter().any(|v| v.variadic),
        void: variants.iter().all(|v| v.void),
    }
}

/// How an argument list counts.
#[derive(Clone, Copy, Default)]
pub struct Counts {
    pub positional: usize,
    /// Has named arguments (or unpacks an array with string keys).
    pub named: bool,
    /// Unpacks an array that is not a constant one.
    pub unpack: bool,
    /// Has any argument.
    pub any: bool,
}

pub fn count(args: &ArgumentList) -> Counts {
    let mut counts = Counts {
        any: !args.arguments.is_empty(),
        ..Counts::default()
    };
    for arg in args.arguments.iter() {
        match arg {
            Argument::Named(_) => counts.named = true,
            Argument::Positional(p) => counts.positional(p),
        }
    }
    counts
}

/// [`count()`] of an anonymous class's constructor arguments.
pub fn count_partial(args: &PartialArgumentList) -> Counts {
    let mut counts = Counts {
        any: !args.arguments.is_empty(),
        ..Counts::default()
    };
    for arg in args.arguments.iter() {
        match arg {
            PartialArgument::Positional(p) => counts.positional(p),
            // named arguments; placeholders (`?`) are no PHP 8.5 syntax, so php-parser rejects them
            _ => counts.named = true,
        }
    }
    counts
}

impl Counts {
    fn positional(&mut self, p: &PositionalArgument) {
        if p.ellipsis.is_none() {
            self.positional += 1;
            return;
        }
        match unpacked(p.value) {
            Unpacked::List(k) => self.positional += k,
            Unpacked::Named => self.named = true,
            Unpacked::Unknown => self.unpack = true,
        }
    }
}

/// What unpacking an expression gives.
enum Unpacked {
    /// A constant list of this many elements.
    List(usize),
    /// A constant array with string keys: named arguments.
    Named,
    /// Not a constant array.
    Unknown,
}

fn unpacked(e: &Expression) -> Unpacked {
    let elements: Vec<&ArrayElement> = match e {
        Expression::Array(a) => a.elements.iter().collect(),
        Expression::LegacyArray(a) => a.elements.iter().collect(),
        _ => return Unpacked::Unknown,
    };
    let mut named = false;
    for el in &elements {
        match el {
            ArrayElement::Value(_) => {}
            ArrayElement::KeyValue(kv) => {
                named |= matches!(kv.key, Expression::Literal(Literal::String(_)));
            }
            _ => return Unpacked::Unknown,
        }
    }
    if named {
        Unpacked::Named
    } else {
        Unpacked::List(elements.len())
    }
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
    counts: Counts,
    at: impl HasSpan + Copy,
    call: Call,
    first_level: bool,
) {
    let Counts {
        positional: n,
        named,
        unpack,
        ..
    } = counts;
    if !named {
        let min = acceptor.min();
        let max = (!acceptor.variadic).then_some(acceptor.optional.len());
        let invoked = if unpack {
            max.map_or(min, |m| m.max(min))
        } else {
            n
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
    if let Some(void) = call.void_message
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
}
