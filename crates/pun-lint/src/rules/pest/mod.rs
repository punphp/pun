//! The `pest` group: rules ported from pestphp/pest-plugin-rector at 1a4b6b7 (src/Rules/...), each in its own file,
//! and what they share: `AbstractRector`'s chain helpers, the `ExpectChainValidation` trait, `PestFunctionDetector`
//! and `PestChainAnalyzer`.
//!
//! The plugin's rules build new nodes and php-parser's format-preserving printer writes them: a new method call is
//! pretty-printed (`$var->name(arg, arg)`), and the nodes kept from the old code keep their own text. When a rule only
//! changes a node it keeps (a new name, a new first argument), the printer rewrites that part alone. The rules here
//! write the same text with edits on those parts.

mod no_focused_test;
mod no_negated_boolean_matcher;
mod no_static_test_closure;
mod prefer_comparison_matchers;
mod prefer_defer;
mod prefer_not_modifier;
mod prefer_strict_equality_matchers;
mod prefer_to_be_alpha;
mod prefer_to_be_alpha_numeric;
mod prefer_to_be_between;
mod prefer_to_be_digits;
mod prefer_to_be_directory;
mod prefer_to_be_empty;
mod prefer_to_be_file;
mod prefer_to_be_in;
mod prefer_to_be_infinite;
mod prefer_to_be_instance_of;
mod prefer_to_be_json;
mod prefer_to_be_list;
mod prefer_to_be_lowercase;
mod prefer_to_be_nan;
mod prefer_to_be_true_false;
mod prefer_to_be_truthy_falsy;
mod prefer_to_be_uppercase;
mod prefer_to_be_url;
mod prefer_to_be_uuid;
mod prefer_to_contain;
mod prefer_to_contain_equal;
mod prefer_to_contain_only_instances_of;
mod prefer_to_end_with;
mod prefer_to_equal_canonicalizing;
mod prefer_to_equal_with_delta;
mod prefer_to_have_count;
mod prefer_to_have_key;
mod prefer_to_have_length;
mod prefer_to_have_method_on_class;
mod prefer_to_have_property;
mod prefer_to_have_same_size;
mod prefer_to_match;
mod prefer_to_start_with;
mod prefer_type_matchers;

pub use no_focused_test::NoFocusedTest;
pub use no_negated_boolean_matcher::NoNegatedBooleanMatcher;
pub use no_static_test_closure::NoStaticTestClosure;
pub use prefer_comparison_matchers::PreferComparisonMatchers;
pub use prefer_defer::PreferDefer;
pub use prefer_not_modifier::PreferNotModifier;
pub use prefer_strict_equality_matchers::PreferStrictEqualityMatchers;
pub use prefer_to_be_alpha::PreferToBeAlpha;
pub use prefer_to_be_alpha_numeric::PreferToBeAlphaNumeric;
pub use prefer_to_be_between::PreferToBeBetween;
pub use prefer_to_be_digits::PreferToBeDigits;
pub use prefer_to_be_directory::PreferToBeDirectory;
pub use prefer_to_be_empty::PreferToBeEmpty;
pub use prefer_to_be_file::PreferToBeFile;
pub use prefer_to_be_in::PreferToBeIn;
pub use prefer_to_be_infinite::PreferToBeInfinite;
pub use prefer_to_be_instance_of::PreferToBeInstanceOf;
pub use prefer_to_be_json::PreferToBeJson;
pub use prefer_to_be_list::PreferToBeList;
pub use prefer_to_be_lowercase::PreferToBeLowercase;
pub use prefer_to_be_nan::PreferToBeNan;
pub use prefer_to_be_true_false::PreferToBeTrueFalse;
pub use prefer_to_be_truthy_falsy::PreferToBeTruthyFalsy;
pub use prefer_to_be_uppercase::PreferToBeUppercase;
pub use prefer_to_be_url::PreferToBeUrl;
pub use prefer_to_be_uuid::PreferToBeUuid;
pub use prefer_to_contain::PreferToContain;
pub use prefer_to_contain_equal::PreferToContainEqual;
pub use prefer_to_contain_only_instances_of::PreferToContainOnlyInstancesOf;
pub use prefer_to_end_with::PreferToEndWith;
pub use prefer_to_equal_canonicalizing::PreferToEqualCanonicalizing;
pub use prefer_to_equal_with_delta::PreferToEqualWithDelta;
pub use prefer_to_have_count::PreferToHaveCount;
pub use prefer_to_have_key::PreferToHaveKey;
pub use prefer_to_have_length::PreferToHaveLength;
pub use prefer_to_have_method_on_class::PreferToHaveMethodOnClass;
pub use prefer_to_have_property::PreferToHaveProperty;
pub use prefer_to_have_same_size::PreferToHaveSameSize;
pub use prefer_to_match::PreferToMatch;
pub use prefer_to_start_with::PreferToStartWith;
pub use prefer_type_matchers::PreferTypeMatchers;

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{
    Access, Argument, ArgumentList, ArrowFunction, Call, ClassLikeMemberSelector, Closure,
    Expression, FunctionCall, Identifier, Keyword, MethodCall, Node, PropertyAccess,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_code_quality::{is_constant, text};
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// The group's rules, in a fixed order.
pub(crate) fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(NoFocusedTest),
        Box::new(NoNegatedBooleanMatcher),
        Box::new(NoStaticTestClosure),
        Box::new(PreferComparisonMatchers),
        Box::new(PreferDefer),
        Box::new(PreferNotModifier),
        Box::new(PreferStrictEqualityMatchers),
        Box::new(PreferToBeAlpha),
        Box::new(PreferToBeAlphaNumeric),
        Box::new(PreferToBeBetween),
        Box::new(PreferToBeDigits),
        Box::new(PreferToBeDirectory),
        Box::new(PreferToBeEmpty),
        Box::new(PreferToBeFile),
        Box::new(PreferToBeIn),
        Box::new(PreferToBeInfinite),
        Box::new(PreferToBeInstanceOf),
        Box::new(PreferToBeJson),
        Box::new(PreferToBeList),
        Box::new(PreferToBeLowercase),
        Box::new(PreferToBeNan),
        Box::new(PreferToBeTrueFalse),
        Box::new(PreferToBeTruthyFalsy),
        Box::new(PreferToBeUppercase),
        Box::new(PreferToBeUrl),
        Box::new(PreferToBeUuid),
        Box::new(PreferToContain),
        Box::new(PreferToContainEqual),
        Box::new(PreferToContainOnlyInstancesOf),
        Box::new(PreferToEndWith),
        Box::new(PreferToEqualCanonicalizing),
        Box::new(PreferToEqualWithDelta),
        Box::new(PreferToHaveCount),
        Box::new(PreferToHaveKey),
        Box::new(PreferToHaveLength),
        Box::new(PreferToHaveMethodOnClass),
        Box::new(PreferToHaveProperty),
        Box::new(PreferToHaveSameSize),
        Box::new(PreferToMatch),
        Box::new(PreferToStartWith),
        Box::new(PreferTypeMatchers),
    ]
}

/// The method call `expr` is, without its parentheses (php-parser keeps none).
pub(crate) fn method_call<'a>(expr: &'a Expression<'a>) -> Option<&'a MethodCall<'a>> {
    match unparenthesize(expr) {
        Expression::Call(Call::Method(call)) => Some(call),
        _ => None,
    }
}

/// The property fetch `expr` is, without its parentheses.
pub(crate) fn property_fetch<'a>(expr: &'a Expression<'a>) -> Option<&'a PropertyAccess<'a>> {
    match unparenthesize(expr) {
        Expression::Access(Access::Property(fetch)) => Some(fetch),
        _ => None,
    }
}

/// The function call `expr` is, without its parentheses.
pub(crate) fn func_call<'a>(expr: &'a Expression<'a>) -> Option<&'a FunctionCall<'a>> {
    match unparenthesize(expr) {
        Expression::Call(Call::Function(call)) => Some(call),
        _ => None,
    }
}

/// `$node->name instanceof Identifier ? $node->name->name : null`: a name written as a plain identifier.
pub(crate) fn identifier<'a>(selector: &ClassLikeMemberSelector<'a>) -> Option<&'a [u8]> {
    match selector {
        ClassLikeMemberSelector::Identifier(id) => Some(id.value),
        _ => None,
    }
}

/// `NodeNameResolver::isName` on a method or property name: a plain identifier, ignoring ASCII case.
pub(crate) fn is_name(selector: &ClassLikeMemberSelector<'_>, name: &str) -> bool {
    identifier(selector).is_some_and(|n| n.eq_ignore_ascii_case(name.as_bytes()))
}

/// `Name::toString()` of a called function written as a name: the name without a leading `\`. Rector's name resolver
/// leaves an unqualified function name as written.
pub(crate) fn function_name<'a>(call: &FunctionCall<'a>) -> Option<&'a [u8]> {
    match call.function {
        Expression::Identifier(Identifier::FullyQualified(id)) => Some(&id.value[1..]),
        Expression::Identifier(id) => Some(id.value()),
        _ => None,
    }
}

/// `getExpectChainRoot` when the root is a function call: the call under the method calls and property fetches of
/// `call`'s object.
pub(crate) fn chain_root<'a>(call: &'a MethodCall<'a>) -> Option<&'a FunctionCall<'a>> {
    let mut current = call.object;
    loop {
        current = match unparenthesize(current) {
            Expression::Call(Call::Method(m)) => m.object,
            Expression::Access(Access::Property(p)) => p.object,
            Expression::Call(Call::Function(f)) => return Some(f),
            _ => return None,
        };
    }
}

/// `getExpectFuncCall`: the chain's root when it is a call of `expect()`.
pub(crate) fn expect_call<'a>(call: &'a MethodCall<'a>) -> Option<&'a FunctionCall<'a>> {
    chain_root(call).filter(|root| super::rector_code_quality::named(root, "expect"))
}

/// `isExpectChain`: the chain's root is a call of `expect()`.
pub(crate) fn is_expect_chain(call: &MethodCall<'_>) -> bool {
    expect_call(call).is_some()
}

/// `getExpectArgument`: the value of the first argument of the chain's `expect()`.
pub(crate) fn expect_argument<'a>(call: &'a MethodCall<'a>) -> Option<&'a Expression<'a>> {
    expect_call(call).and_then(|e| first_value(&e.argument_list))
}

/// The value of the first argument of a list.
pub(crate) fn first_value<'a>(list: &'a ArgumentList<'a>) -> Option<&'a Expression<'a>> {
    list.arguments.first().map(argument_value)
}

/// An argument's value (`Arg::$value`).
pub(crate) fn argument_value<'a>(argument: &'a Argument<'a>) -> &'a Expression<'a> {
    match argument {
        Argument::Positional(p) => p.value,
        Argument::Named(n) => n.value,
    }
}

/// What `getMatcherSubjectHolder` finds: the `expect()` call or the `->and()` call whose first argument a matcher
/// checks.
#[derive(Clone, Copy)]
pub(crate) struct Holder<'a> {
    pub span: Span,
    pub arguments: &'a ArgumentList<'a>,
    /// The object of `->and()`, and its name; none for `expect()`.
    method: Option<(&'a Expression<'a>, &'a [u8])>,
    /// The end of the function's name, for `expect()`.
    name_end: usize,
}

impl<'a> Holder<'a> {
    fn function(call: &'a FunctionCall<'a>) -> Self {
        Holder {
            span: node_span(call),
            arguments: &call.argument_list,
            method: None,
            name_end: node_span(call.function).end,
        }
    }

    fn method(call: &'a MethodCall<'a>) -> Self {
        Holder {
            span: node_span(call),
            arguments: &call.argument_list,
            method: Some((call.object, identifier(&call.method).unwrap_or_default())),
            name_end: 0,
        }
    }

    /// `$holder->args = [new Arg($value)]`: the arguments become `value` alone. One argument is replaced where it
    /// stands; when arguments go, php-parser prints the whole call again (`expect(value)`, `$var->and(value)`).
    pub(crate) fn set_arguments(&self, ctx: &Context<'_>, value: &Expression<'_>) -> Vec<Edit> {
        let value = value_text(ctx, value);
        match (self.arguments.arguments.as_slice(), self.method) {
            ([only], _) => vec![Edit::new(node_span(only), value)],
            (_, None) => vec![Edit::new(
                Span::new(self.name_end, self.span.end),
                format!("({value})"),
            )],
            (_, Some((object, name))) => reprint_from_object(
                object,
                self.span.end,
                format!("->{}({value})", String::from_utf8_lossy(name)),
            ),
        }
    }
}

/// `getMatcherSubjectHolder`: down the chain from `call`'s object, the first `->and()` call, or the `expect()` call
/// at its root.
pub(crate) fn subject_holder<'a>(call: &'a MethodCall<'a>) -> Option<Holder<'a>> {
    let mut current = call.object;
    loop {
        current = match unparenthesize(current) {
            Expression::Call(Call::Method(m)) if is_name(&m.method, "and") => {
                return Some(Holder::method(m));
            }
            Expression::Call(Call::Method(m)) => m.object,
            Expression::Access(Access::Property(p)) => p.object,
            Expression::Call(Call::Function(f))
                if super::rector_code_quality::named(f, "expect") =>
            {
                return Some(Holder::function(f));
            }
            _ => return None,
        };
    }
}

/// `getMatcherSubject`: the value of the holder's first argument.
pub(crate) fn matcher_subject<'a>(call: &'a MethodCall<'a>) -> Option<&'a Expression<'a>> {
    subject_holder(call).and_then(|h| first_value(h.arguments))
}

/// `isMatcherAppliedDirectly`: `call` is called on its holder, or on the holder's `->not`.
pub(crate) fn applied_directly(call: &MethodCall<'_>) -> bool {
    let Some(holder) = subject_holder(call) else {
        return false;
    };
    let mut var = unparenthesize(call.object);
    if let Some(not) = property_fetch(var).filter(|p| is_name(&p.property, "not")) {
        var = unparenthesize(not.object);
    }
    node_span(var) == holder.span
}

/// `hasNotModifier`: `call` is called on a `->not`.
pub(crate) fn has_not(call: &MethodCall<'_>) -> bool {
    property_fetch(call.object).is_some_and(|p| is_name(&p.property, "not"))
}

/// The source of an expression as php-parser's printer keeps it: without its own parentheses.
pub(crate) fn value_text<'s>(ctx: &Context<'s>, expr: &Expression<'_>) -> &'s str {
    text(ctx, unparenthesize(expr))
}

/// `$holder->args[0] = new Arg($value)`: the first argument (its name or `...` too) becomes `value`.
pub(crate) fn set_first_argument(
    ctx: &Context<'_>,
    list: &ArgumentList<'_>,
    value: &Expression<'_>,
) -> Option<Edit> {
    let first = list.arguments.first()?;
    Some(Edit::new(node_span(first), value_text(ctx, value)))
}

/// `$node->args = [new Arg(...), ...]` on a method call, with its name `name` (new or not) and `values` the printed
/// arguments: an argument replaced by one is replaced where it stands, arguments put in an empty list go after its
/// `(`, and any other change makes php-parser print the call again from its object on (`$var->name(args)`).
pub(crate) fn set_call_arguments(
    call: &MethodCall<'_>,
    name: &str,
    values: &[String],
) -> Vec<Edit> {
    let old = call.argument_list.arguments.as_slice();
    if old.len() == values.len() {
        let mut edits: Vec<Edit> = (old.iter().zip(values))
            .map(|(a, v)| Edit::new(node_span(a), v.clone()))
            .collect();
        edits.push(rename(call, name));
        return edits;
    }
    if old.is_empty() {
        let at = call.argument_list.left_parenthesis.end.offset as usize;
        return vec![
            rename(call, name),
            Edit::new(Span::new(at, at), values.join(", ")),
        ];
    }
    reprint_from_object(
        call.object,
        node_span(call).end,
        format!("->{name}({})", values.join(", ")),
    )
}

/// The printed `new Arg($value)` of each value: its source.
pub(crate) fn texts(ctx: &Context<'_>, values: &[&Expression<'_>]) -> Vec<String> {
    values
        .iter()
        .map(|v| value_text(ctx, v).to_string())
        .collect()
}

/// php-parser's printer writing a method call again: its object as it was (without parentheses, which an object of a
/// chain never needs) and then `rest` up to `end`.
pub(crate) fn reprint_from_object(object: &Expression<'_>, end: usize, rest: String) -> Vec<Edit> {
    let outer = node_span(object);
    let inner = node_span(unparenthesize(object));
    let mut edits = vec![Edit::new(Span::new(inner.end, end), rest)];
    if outer.start != inner.start {
        edits.insert(0, Edit::new(Span::new(outer.start, inner.start), ""));
    }
    edits
}

/// `new Arg($value)` printed: the value's source, joined by `, ` as php-parser prints a list of arguments.
pub(crate) fn arguments_text(ctx: &Context<'_>, values: &[&Expression<'_>]) -> String {
    values
        .iter()
        .map(|v| value_text(ctx, v))
        .collect::<Vec<_>>()
        .join(", ")
}

/// A report on `span` whose fix makes `edits`.
pub(crate) fn report(span: Span, message: String, safety: Safety, edits: Vec<Edit>) -> Report {
    Report {
        span,
        message,
        help: None,
        fix: Some(Fix { safety, edits }),
    }
}

/// The edit that renames the method of `call` (`$node->name = new Identifier($name)`).
pub(crate) fn rename(call: &MethodCall<'_>, name: &str) -> Edit {
    Edit::new(node_span(&call.method), name)
}

/// What `extractFunctionFromExpect` returns: the holder, the function call that is its first argument, and the
/// matcher's name.
pub(crate) struct Extracted<'a> {
    pub holder: Holder<'a>,
    pub function: &'a FunctionCall<'a>,
    pub method: &'a [u8],
}

/// `ExpectChainValidation::extractFunctionFromExpect`: `call` is a matcher of `valid_methods` (matched exactly)
/// called directly on its holder, whose first argument is a call of one of `functions` (by its name as written,
/// exactly).
pub(crate) fn extract_function<'a>(
    call: &'a MethodCall<'a>,
    functions: &[&str],
    valid_methods: &[&str],
) -> Option<Extracted<'a>> {
    if !is_expect_chain(call) {
        return None;
    }
    let method = identifier(&call.method)?;
    if !valid_methods.iter().any(|m| m.as_bytes() == method) {
        return None;
    }
    let holder = subject_holder(call)?;
    if !applied_directly(call) {
        return None;
    }
    let function = func_call(first_value(holder.arguments)?)?;
    let name = function_name(function)?;
    if !functions.iter().any(|f| f.as_bytes() == name) {
        return None;
    }
    Some(Extracted {
        holder,
        function,
        method,
    })
}

/// `ExpectChainValidation::calculateNeedsNot`: the new matcher needs `->not` when the old one was `toBeFalse`, or
/// was `toBeTrue` after a `->not`.
pub(crate) fn needs_not(method: &[u8], call: &MethodCall<'_>) -> bool {
    (method == b"toBeFalse") != has_not(call)
}

/// `ExpectChainValidation::buildMatcherCall`: the holder's first argument becomes `value`, and `call` becomes
/// `$holder->matcher(args)` (through `->not` if `not`).
pub(crate) fn build_matcher_call(
    ctx: &Context<'_>,
    call: &MethodCall<'_>,
    holder: Holder<'_>,
    value: &Expression<'_>,
    matcher: &str,
    args: &[&Expression<'_>],
    not: bool,
) -> Option<Vec<Edit>> {
    let not = if not { "->not" } else { "" };
    Some(vec![
        set_first_argument(ctx, holder.arguments, value)?,
        Edit::new(
            Span::new(holder.span.end, node_span(call).end),
            format!("{not}->{matcher}({})", arguments_text(ctx, args)),
        ),
    ])
}

/// `ExpectChainValidation::isFalse` / `isTrue`: the constant `false` / `true`.
pub(crate) fn is_bool_constant(expr: &Expression<'_>, name: &str) -> bool {
    is_constant(expr, name)
}

/// The rules of the `extractFunctionFromExpect` family: `expect(function(args))->toBeTrue()` (or `toBeFalse()`, or
/// after `->not`) becomes `expect(subject)->matcher(rest)`. `shape` picks the subject and the matcher's arguments
/// from the call's arguments, or refuses them.
pub(crate) fn function_matcher<'a>(
    ctx: &Context<'_>,
    call: &'a MethodCall<'a>,
    function: &str,
    matcher: &str,
    shape: impl Fn(&[&'a Expression<'a>]) -> Option<(&'a Expression<'a>, Vec<&'a Expression<'a>>)>,
    out: &mut Vec<Report>,
) {
    let Some(extracted) = extract_function(call, &[function], &["toBeTrue", "toBeFalse"]) else {
        return;
    };
    let values: Vec<&Expression<'_>> = (extracted.function.argument_list.arguments.iter())
        .map(argument_value)
        .collect();
    let Some((subject, args)) = shape(&values) else {
        return;
    };
    let not = needs_not(extracted.method, call);
    let Some(edits) = build_matcher_call(ctx, call, extracted.holder, subject, matcher, &args, not)
    else {
        return;
    };
    out.push(report(
        node_span(call),
        format!("Use `{matcher}()` instead of checking `{function}()`."),
        Safety::Unsafe,
        edits,
    ));
}

/// A PHPStan `TrinaryLogic` answer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Trinary {
    Yes,
    Maybe,
    No,
}

/// The type questions the rules ask of `$this->getType($expr)`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Is {
    String,
    Integer,
    Float,
    Array,
    Object,
    Iterable,
    Boolean,
}

/// `$this->getType($expr)->is<kind>()`: the answer for each type of the union, combined as PHPStan's
/// `TrinaryLogic::extremeIdentity` (yes or no when all agree, maybe otherwise). An expression the analyser did not
/// type is `mixed`, which may be anything.
pub(crate) fn type_is(ctx: &Context<'_>, expr: &Expression<'_>, kind: Is) -> Trinary {
    let facts = facts(ctx);
    let Some(union) = facts.full_type(node_span(unparenthesize(expr))) else {
        return Trinary::Maybe;
    };
    let mut answers = union.types.iter().map(|t| atomic_is(t, kind));
    let Some(first) = answers.next() else {
        return Trinary::Maybe;
    };
    answers.fold(first, |a, b| if a == b { a } else { Trinary::Maybe })
}

fn atomic_is(atomic: &TAtomic, kind: Is) -> Trinary {
    use Trinary::{Maybe, No, Yes};
    match (atomic, kind) {
        (TAtomic::Mixed(_), _) => Maybe,
        (TAtomic::Scalar(scalar), _) => match (scalar, kind) {
            (TScalar::String(_) | TScalar::ClassLikeString(_), Is::String) => Yes,
            (TScalar::Integer(_), Is::Integer) => Yes,
            (TScalar::Float(_), Is::Float) => Yes,
            (TScalar::Bool(_), Is::Boolean) => Yes,
            (TScalar::Generic, Is::String | Is::Integer | Is::Float | Is::Boolean) => Maybe,
            (TScalar::Numeric, Is::String | Is::Integer | Is::Float) => Maybe,
            (TScalar::ArrayKey, Is::String | Is::Integer) => Maybe,
            _ => No,
        },
        (TAtomic::Array(_), Is::Array | Is::Iterable) => Yes,
        (TAtomic::Iterable(_), Is::Iterable) => Yes,
        (TAtomic::Iterable(_), Is::Array | Is::Object) => Maybe,
        (TAtomic::Object(_), Is::Object) => Yes,
        (TAtomic::Object(_), Is::Iterable) => Maybe,
        (TAtomic::Callable(_), Is::String | Is::Array | Is::Object) => Maybe,
        (TAtomic::Null | TAtomic::Void | TAtomic::Never | TAtomic::Resource(_), _) => No,
        (
            TAtomic::Array(_) | TAtomic::Iterable(_) | TAtomic::Object(_) | TAtomic::Callable(_),
            _,
        ) => No,
        _ => Maybe,
    }
}

/// `PestFunctionDetector::CLOSURE_FUNCTIONS`: the Pest functions that take a callback, and its argument's index.
const CLOSURE_FUNCTIONS: [(&str, usize); 7] = [
    ("it", 1),
    ("test", 1),
    ("describe", 1),
    ("beforeEach", 0),
    ("afterEach", 0),
    ("beforeAll", 0),
    ("afterAll", 0),
];

/// A Pest callback: a closure or an arrow function.
#[derive(Clone, Copy)]
pub(crate) enum Callback<'a> {
    Closure(&'a Closure<'a>),
    Arrow(&'a ArrowFunction<'a>),
}

impl<'a> Callback<'a> {
    fn node(self) -> Node<'a, 'a> {
        match self {
            Callback::Closure(c) => Node::Closure(c),
            Callback::Arrow(a) => Node::ArrowFunction(a),
        }
    }
}

/// `PestFunctionDetector::extractClosure`: the callback a Pest function (named exactly, as written) gets at its
/// callback's index, with its `function` or `fn` keyword and its `static` keyword.
pub(crate) fn pest_callback<'a>(
    call: &'a FunctionCall<'a>,
) -> Option<(Callback<'a>, Keyword<'a>, Option<Keyword<'a>>)> {
    let name = function_name(call)?;
    let (_, index) = CLOSURE_FUNCTIONS
        .iter()
        .find(|(f, _)| f.as_bytes() == name)?;
    let value = argument_value(call.argument_list.arguments.get(*index)?);
    match unparenthesize(value) {
        Expression::Closure(c) => Some((Callback::Closure(c), c.function, c.r#static)),
        Expression::ArrowFunction(a) => Some((Callback::Arrow(a), a.r#fn, a.r#static)),
        _ => None,
    }
}

/// `PestFunctionDetector::closureRequiresInstanceBinding`: a part of the callback uses `$this`, outside a nested
/// static closure (a nested arrow function counts, static or not) and outside the callbacks that nested Pest calls
/// get.
pub(crate) fn requires_instance_binding(callback: Callback<'_>) -> bool {
    let mut found = false;
    callback
        .node()
        .visit_children(|child| found = found || uses_this(child));
    found
}

/// `subNodeUsesThis`.
fn uses_this(node: Node<'_, '_>) -> bool {
    match node {
        Node::DirectVariable(v) => v.name == b"$this",
        Node::FunctionCall(call) => {
            // `funcCallUsesThis`: the arguments, without the call's own Pest callback; the name is not looked at.
            let callback = pest_callback(call).map(|(c, ..)| node_span(&c.node()));
            call.argument_list.arguments.iter().any(|a| {
                let value = argument_value(a);
                Some(node_span(unparenthesize(value))) != callback
                    && uses_this(Node::Expression(value))
            })
        }
        Node::Closure(c) => c.r#static.is_none() && requires_instance_binding(Callback::Closure(c)),
        Node::ArrowFunction(a) => requires_instance_binding(Callback::Arrow(a)),
        _ => {
            let mut found = false;
            node.visit_children(|child| found = found || uses_this(child));
            found
        }
    }
}
