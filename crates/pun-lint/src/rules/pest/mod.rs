//! The `pest` group: rules ported from pestphp/pest-plugin-rector at 1a4b6b7 (src/Rules/...), each in its own file,
//! and what they share: `AbstractRector`'s chain helpers, the `ExpectChainValidation` trait, `PestFunctionDetector`
//! and `PestChainAnalyzer`.
//!
//! The plugin's rules build new nodes and php-parser's format-preserving printer writes them: a new method call is
//! pretty-printed (`$var->name(arg, arg)`), and the nodes kept from the old code keep their own text. When a rule only
//! changes a node it keeps (a new name, a new first argument), the printer rewrites that part alone. The rules here
//! write the same text with edits on those parts.

mod no_before_all_in_describe;
mod no_expect_exception;
mod no_focused_test;
mod no_invalid_repeat;
mod no_negated_boolean_matcher;
mod no_redundant_type_expectation;
mod no_static_test_closure;
mod prefer_chained_expect;
mod prefer_comparison_matchers;
mod prefer_defer;
mod prefer_each_modifier;
mod prefer_expect;
mod prefer_filesystem_matchers;
mod prefer_not_modifier;
mod prefer_sequence;
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
mod prefer_to_have_keys;
mod prefer_to_have_length;
mod prefer_to_have_method_on_class;
mod prefer_to_have_properties;
mod prefer_to_have_property;
mod prefer_to_have_same_size;
mod prefer_to_match;
mod prefer_to_match_array;
mod prefer_to_match_object;
mod prefer_to_start_with;
mod prefer_to_throw;
mod prefer_type_matchers;
mod require_type_matcher_first;

pub use no_before_all_in_describe::NoBeforeAllInDescribe;
pub use no_expect_exception::NoExpectException;
pub use no_focused_test::NoFocusedTest;
pub use no_invalid_repeat::NoInvalidRepeat;
pub use no_negated_boolean_matcher::NoNegatedBooleanMatcher;
pub use no_redundant_type_expectation::NoRedundantTypeExpectation;
pub use no_static_test_closure::NoStaticTestClosure;
pub use prefer_chained_expect::PreferChainedExpect;
pub use prefer_comparison_matchers::PreferComparisonMatchers;
pub use prefer_defer::PreferDefer;
pub use prefer_each_modifier::PreferEachModifier;
pub use prefer_expect::PreferExpect;
pub use prefer_filesystem_matchers::PreferFilesystemMatchers;
pub use prefer_not_modifier::PreferNotModifier;
pub use prefer_sequence::PreferSequence;
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
pub use prefer_to_have_keys::PreferToHaveKeys;
pub use prefer_to_have_length::PreferToHaveLength;
pub use prefer_to_have_method_on_class::PreferToHaveMethodOnClass;
pub use prefer_to_have_properties::PreferToHaveProperties;
pub use prefer_to_have_property::PreferToHaveProperty;
pub use prefer_to_have_same_size::PreferToHaveSameSize;
pub use prefer_to_match::PreferToMatch;
pub use prefer_to_match_array::PreferToMatchArray;
pub use prefer_to_match_object::PreferToMatchObject;
pub use prefer_to_start_with::PreferToStartWith;
pub use prefer_to_throw::PreferToThrow;
pub use prefer_type_matchers::PreferTypeMatchers;
pub use require_type_matcher_first::RequireTypeMatcherFirst;

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{
    Access, Argument, ArgumentList, ArrowFunction, Call, ClassLikeMemberSelector, Closure,
    Expression, FunctionCall, Identifier, Keyword, MethodCall, Node, NodeKind, PropertyAccess,
    Statement,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_code_quality::{is_constant, text};
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// The group's rules, in a fixed order.
pub(crate) fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(NoBeforeAllInDescribe),
        Box::new(NoExpectException),
        Box::new(NoFocusedTest),
        Box::new(NoInvalidRepeat),
        Box::new(NoNegatedBooleanMatcher),
        Box::new(NoRedundantTypeExpectation),
        Box::new(NoStaticTestClosure),
        Box::new(PreferChainedExpect::default()),
        Box::new(PreferComparisonMatchers),
        Box::new(PreferDefer),
        Box::new(PreferEachModifier),
        Box::new(PreferExpect),
        Box::new(PreferFilesystemMatchers),
        Box::new(PreferNotModifier),
        Box::new(PreferSequence),
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
        Box::new(PreferToHaveKeys),
        Box::new(PreferToHaveLength),
        Box::new(PreferToHaveMethodOnClass),
        Box::new(PreferToHaveProperties),
        Box::new(PreferToHaveProperty),
        Box::new(PreferToHaveSameSize),
        Box::new(PreferToMatch),
        Box::new(PreferToMatchArray),
        Box::new(PreferToMatchObject),
        Box::new(PreferToStartWith),
        Box::new(PreferToThrow),
        Box::new(PreferTypeMatchers),
        Box::new(RequireTypeMatcherFirst),
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

/// php-parser's `getComments()` of a node that starts at `start`, when no node around it starts there too (php-parser
/// gives a comment to the outermost node only): a comment right before it, with only whitespace between.
pub(crate) fn has_comments(ctx: &Context<'_>, start: usize) -> bool {
    super::php_parser::tokens(ctx.source)
        .iter()
        .rev()
        .find(|t| t.end <= start && t.kind != mago_syntax::token::TokenKind::Whitespace)
        .is_some_and(|t| t.kind.is_comment())
}

/// `PestChainAnalyzer::getRootFuncCall`: the call under the chain's method calls, and then its property fetches.
pub(crate) fn root_func_call<'a>(call: &'a MethodCall<'a>) -> Option<&'a FunctionCall<'a>> {
    let mut current: &Expression<'_> = call.object;
    while let Some(m) = method_call(current) {
        current = m.object;
    }
    while let Some(p) = property_fetch(current) {
        current = p.object;
    }
    func_call(current)
}

/// `source[span]` as `TokenStream::getTokenCode` gives it with an indent adjustment of `delta`: in every token but a
/// string's, each newline gains `delta` spaces, or loses exactly `-delta` spaces that follow it.
pub(crate) fn shift(ctx: &Context<'_>, span: Span, delta: isize) -> String {
    let text = &ctx.source[span.start..span.end];
    if delta == 0 || !text.contains('\n') {
        return text.to_string();
    }
    let (from, to) = if delta < 0 {
        (
            format!("\n{}", " ".repeat(delta.unsigned_abs())),
            "\n".to_string(),
        )
    } else {
        (
            "\n".to_string(),
            format!("\n{}", " ".repeat(delta as usize)),
        )
    };
    let mut out = String::with_capacity(text.len());
    let mut at = span.start;
    for t in super::php_parser::tokens(ctx.source) {
        if t.end <= span.start || t.start >= span.end {
            continue;
        }
        let end = t.end.min(span.end);
        let token = &ctx.source[at..end];
        at = end;
        use mago_syntax::token::TokenKind as K;
        if matches!(
            t.kind,
            K::LiteralString | K::PartialLiteralString | K::StringPart
        ) {
            out.push_str(token);
        } else {
            out.push_str(&token.replace(&from, &to));
        }
    }
    out
}

/// The indent of the line that holds `pos`, in columns as php-parser counts them (a tab is one).
pub(crate) fn indent_at(ctx: &Context<'_>, pos: usize) -> usize {
    super::php_parser::line_indent(ctx.source, pos).len()
}

/// A node kept from the old code, printed where the printer stands at `indent`: its text moved from the indent of
/// its line.
pub(crate) fn kept_text(ctx: &Context<'_>, span: Span, indent: usize) -> String {
    shift(
        ctx,
        span,
        indent as isize - indent_at(ctx, span.start) as isize,
    )
}

/// The comments php-parser gives a node that starts at `start` (when it is the outermost node there): every comment
/// after the token before it.
pub(crate) fn comment_spans(ctx: &Context<'_>, start: usize) -> Vec<Span> {
    let tokens = super::php_parser::tokens(ctx.source);
    let before = tokens
        .iter()
        .rev()
        .find(|t| t.end <= start && !t.kind.is_trivia())
        .map_or(0, |t| t.end);
    tokens
        .iter()
        .filter(|t| t.start >= before && t.end <= start && t.kind.is_comment())
        .map(|t| Span::new(t.start, t.end))
        .collect()
}

/// `Comment::getReformattedText`.
fn reformat_comment(text: &str) -> String {
    let text = text.replace("\r\n", "\n");
    let text = text.trim_end_matches('\n');
    let lines: Vec<&str> = text.split('\n').collect();
    if lines.len() == 1 {
        return text.to_string();
    }
    // Every line after the first starts with blanks and a `*`: those blanks become one space.
    if lines[1..].iter().all(|l| {
        let t = l.trim_start();
        t.starts_with('*') && t.len() < l.len()
    }) {
        return std::iter::once(lines[0].to_string())
            .chain(lines[1..].iter().map(|l| format!(" {}", l.trim_start())))
            .collect::<Vec<_>>()
            .join("\n");
    }
    // `/*` alone on its line and `*/` after blanks on the last: those blanks come off every line.
    let last = lines[lines.len() - 1];
    if lines[0].trim_end() == "/*" || lines[0].trim_end() == "/**" {
        let blanks = &last[..last.len() - last.trim_start().len()];
        if last.trim_start() == "*/" {
            return lines
                .iter()
                .map(|l| l.strip_prefix(blanks).unwrap_or(l))
                .collect::<Vec<_>>()
                .join("\n");
        }
    }
    text.to_string()
}

/// `pComments`: `comments` reformatted and joined by newlines at `indent`.
pub(crate) fn p_comments(ctx: &Context<'_>, comments: &[Span], indent: usize) -> String {
    let nl = format!("\n{}", " ".repeat(indent));
    comments
        .iter()
        .map(|c| reformat_comment(&ctx.source[c.start..c.end]).replace('\n', &nl))
        .collect::<Vec<_>>()
        .join(&nl)
}

/// `pMaybeMultiline($args)` for kept arguments printed at `indent`: joined by `, `, or, when one of them owns a
/// comment, one a line with its comments, one level deeper, and the `)` on a line of its own.
pub(crate) fn print_args(ctx: &Context<'_>, list: &ArgumentList<'_>, indent: usize) -> String {
    let args = list.arguments.as_slice();
    if !args.iter().any(|a| has_comments(ctx, node_span(a).start)) {
        return args
            .iter()
            .map(|a| kept_text(ctx, node_span(a), indent))
            .collect::<Vec<_>>()
            .join(", ");
    }
    let inner = indent + 4;
    let nl = format!("\n{}", " ".repeat(inner));
    let mut out = String::new();
    for (i, a) in args.iter().enumerate() {
        let start = node_span(a).start;
        if has_comments(ctx, start) {
            out.push_str(&nl);
            out.push_str(&p_comments(ctx, &comment_spans(ctx, start), inner));
        }
        out.push_str(&nl);
        out.push_str(&kept_text(ctx, node_span(a), inner));
        if i + 1 < args.len() {
            out.push(',');
        }
    }
    out.push('\n');
    out.push_str(&" ".repeat(indent));
    out
}

/// `pObjectProperty`: a method or property name as the printer writes it, `{expr}` for anything but an identifier.
pub(crate) fn print_member(
    ctx: &Context<'_>,
    selector: &ClassLikeMemberSelector<'_>,
    indent: usize,
) -> String {
    match selector {
        ClassLikeMemberSelector::Expression(e) => {
            format!("{{{}}}", kept_text(ctx, node_span(e.expression), indent))
        }
        ClassLikeMemberSelector::Variable(v) => format!("{{{}}}", text(ctx, v)),
        _ => text(ctx, selector).to_string(),
    }
}

/// One link of `collectChainMethods`: a method call or a property fetch of an expect chain.
#[derive(Clone, Copy)]
pub(crate) enum Link<'a> {
    Method(&'a MethodCall<'a>),
    Property(&'a PropertyAccess<'a>),
}

/// `collectChainMethods`: the method calls and property fetches of `call`'s chain, innermost first, down to (not
/// into) the function call at its root.
pub(crate) fn chain_links<'a>(call: &'a MethodCall<'a>) -> Vec<Link<'a>> {
    let mut links = vec![Link::Method(call)];
    let mut next = call.object;
    loop {
        match unparenthesize(next) {
            Expression::Call(Call::Method(m)) => {
                links.push(Link::Method(m));
                next = m.object;
            }
            Expression::Access(Access::Property(p)) => {
                links.push(Link::Property(p));
                next = p.object;
            }
            _ => break,
        }
    }
    links.reverse();
    links
}

/// `rebuildMethodChain` printed: `links` written again after `base`, each `->name` or `->name(args)`, on one line
/// at `indent`.
pub(crate) fn print_links(ctx: &Context<'_>, links: &[Link<'_>], indent: usize) -> String {
    links
        .iter()
        .map(|link| match link {
            Link::Method(m) => format!(
                "->{}({})",
                print_member(ctx, &m.method, indent),
                print_args(ctx, &m.argument_list, indent)
            ),
            Link::Property(p) => format!("->{}", print_member(ctx, &p.property, indent)),
        })
        .collect()
}

/// php-parser's comments of a statement starting at `start`: from the first comment after the token before it.
pub(crate) fn comments_start(ctx: &Context<'_>, start: usize) -> usize {
    let tokens = super::php_parser::tokens(ctx.source);
    let before = tokens
        .iter()
        .rev()
        .find(|t| t.end <= start && !t.kind.is_trivia())
        .map_or(0, |t| t.end);
    tokens
        .iter()
        .find(|t| t.start >= before && t.end <= start && t.kind.is_comment())
        .map_or(start, |t| t.start)
}

/// One statement of a list a rule rewrites (php-parser's new `stmts`).
pub(crate) enum Item<'a> {
    /// The old statement at this index, as it was.
    Old(usize),
    /// The old statement at this index (the same node), whose inside changed: its text at its old place.
    Changed(usize, String),
    /// A new statement: `print` gives its text at an indent, and it carries the comments `comments`.
    New {
        print: Box<dyn Fn(usize) -> String + 'a>,
        comments: Vec<Span>,
    },
}

impl Item<'_> {
    fn old(&self) -> Option<usize> {
        match self {
            Item::Old(i) | Item::Changed(i, _) => Some(*i),
            Item::New { .. } => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Diff {
    Keep(usize, usize),
    Remove(usize),
    Add(usize),
    Replace(usize, usize),
}

/// `Differ::diffWithReplacements` on the old statements `0..n` and `new`, a statement being equal only to itself.
fn diff(n: usize, new: &[Item<'_>]) -> Vec<Diff> {
    let m = new.len();
    let equal = |x: usize, y: usize| new[y].old() == Some(x);
    let offset = (n + m + 1) as isize;
    let at = |k: isize| (k + offset) as usize;
    let mut v = vec![0isize; 2 * (n + m) + 3];
    v[at(1)] = 0;
    let mut trace = Vec::new();
    let (mut x, mut y) = (0isize, 0isize);
    'outer: for d in 0..=(n + m) as isize {
        trace.push(v.clone());
        let mut k = -d;
        while k <= d {
            x = if k == -d || (k != d && v[at(k - 1)] < v[at(k + 1)]) {
                v[at(k + 1)]
            } else {
                v[at(k - 1)] + 1
            };
            y = x - k;
            while (x as usize) < n && (y as usize) < m && equal(x as usize, y as usize) {
                x += 1;
                y += 1;
            }
            v[at(k)] = x;
            if x as usize >= n && y as usize >= m {
                break 'outer;
            }
            k += 2;
        }
    }
    let mut result = Vec::new();
    for d in (0..trace.len() as isize).rev() {
        let v = &trace[d as usize];
        let k = x - y;
        let prev_k = if k == -d || (k != d && v[at(k - 1)] < v[at(k + 1)]) {
            k + 1
        } else {
            k - 1
        };
        let prev_x = v[at(prev_k)];
        let prev_y = prev_x - prev_k;
        while x > prev_x && y > prev_y {
            result.push(Diff::Keep(x as usize - 1, y as usize - 1));
            x -= 1;
            y -= 1;
        }
        if d == 0 {
            break;
        }
        while x > prev_x {
            result.push(Diff::Remove(x as usize - 1));
            x -= 1;
        }
        while y > prev_y {
            result.push(Diff::Add(y as usize - 1));
            y -= 1;
        }
    }
    result.reverse();
    // coalesceReplacements: a run of removals followed by as many additions is a run of replacements.
    let mut out = Vec::new();
    let mut i = 0;
    while i < result.len() {
        if !matches!(result[i], Diff::Remove(_)) {
            out.push(result[i]);
            i += 1;
            continue;
        }
        let j = i + result[i..]
            .iter()
            .take_while(|d| matches!(d, Diff::Remove(_)))
            .count();
        let k = j + result[j..]
            .iter()
            .take_while(|d| matches!(d, Diff::Add(_)))
            .count();
        if j - i == k - j {
            for n in 0..j - i {
                let (Diff::Remove(o), Diff::Add(a)) = (result[i + n], result[j + n]) else {
                    unreachable!()
                };
                out.push(Diff::Replace(o, a));
            }
        } else {
            out.extend_from_slice(&result[i..k]);
        }
        i = k;
    }
    out
}

/// An old statement of a list as the printer sees it: where its comments start, where it starts and ends.
#[derive(Clone, Copy)]
pub(crate) struct OldStmt {
    pub comments: usize,
    pub start: usize,
    pub end: usize,
    /// php-parser's `Nop` for the comments after the last statement.
    pub nop: bool,
}

/// The statements of `list` (the trailing comments' `Nop` included) with their comments.
pub(crate) fn old_statements(
    ctx: &Context<'_>,
    list: &super::php_parser::List<'_, '_>,
) -> Vec<OldStmt> {
    let mut olds: Vec<OldStmt> = list
        .stmts
        .iter()
        .map(|s| {
            let span = node_span(*s);
            OldStmt {
                comments: comments_start(ctx, span.start),
                start: span.start,
                end: span.end,
                nop: false,
            }
        })
        .collect();
    if list.nop {
        let after = olds.last().map_or(list.inner.start, |s| s.end);
        let tokens = super::php_parser::tokens(ctx.source);
        let comments: Vec<_> = tokens
            .iter()
            .filter(|t| t.start >= after && t.end <= list.inner.end && t.kind.is_comment())
            .collect();
        let (start, end) = (comments[0].start, comments[comments.len() - 1].end);
        olds.push(OldStmt {
            comments: start,
            start,
            end,
            nop: true,
        });
    }
    olds
}

/// The comments of an old statement (all of a `Nop`).
fn old_comments(ctx: &Context<'_>, old: &OldStmt) -> Vec<Span> {
    let end = if old.nop { old.end } else { old.start };
    super::php_parser::tokens(ctx.source)
        .iter()
        .filter(|t| t.start >= old.comments && t.end <= end && t.kind.is_comment())
        .map(|t| Span::new(t.start, t.end))
        .collect()
}

/// php-parser's `pArray` on a statement list whose old statements are `olds` and new ones `items`, printed from `pos`
/// on: the text that replaces `pos..end`, and `end`; none where the printer gives up and prints the list's parent
/// again.
pub(crate) fn print_list(
    ctx: &Context<'_>,
    olds: &[OldStmt],
    braces: bool,
    mut pos: usize,
    items: &[Item<'_>],
) -> Option<(String, usize)> {
    let tokens = super::php_parser::tokens(ctx.source);
    let tag_in = |from: usize, to: usize| {
        use mago_syntax::token::TokenKind as K;
        tokens.iter().any(|t| {
            t.start >= from
                && t.end <= to
                && matches!(
                    t.kind,
                    K::OpenTag | K::CloseTag | K::EchoTag | K::ShortOpenTag
                )
        })
    };
    if olds.len() == 1 && items.len() != 1 && !braces {
        return None;
    }
    let mut result = String::new();
    let mut before_first_keep = true;
    let mut skip_removed = false;
    let mut delayed: Vec<&Item<'_>> = Vec::new();
    let mut last_indent = 0;
    // p() of a statement: an old one moved to `indent` (php-parser's indent adjustment), a new one printed there.
    let print_new = |item: &Item<'_>, indent: usize| match item {
        Item::Old(i) => kept_text(ctx, Span::new(olds[*i].start, olds[*i].end), indent),
        Item::Changed(i, text) => shift_text(
            text,
            indent as isize - indent_at(ctx, olds[*i].start) as isize,
        ),
        Item::New { print, .. } => print(indent),
    };
    let comments_of = |item: &Item<'_>| match item {
        Item::New { comments, .. } => comments.clone(),
        Item::Old(i) | Item::Changed(i, _) => old_comments(ctx, &olds[*i]),
    };
    let new_comments = |item: &Item<'_>, indent: usize| {
        let comments = comments_of(item);
        (!comments.is_empty()).then(|| p_comments(ctx, &comments, indent))
    };
    for (i, d) in diff(olds.len(), items).into_iter().enumerate() {
        match d {
            Diff::Keep(o, n) | Diff::Replace(o, n) => {
                before_first_keep = false;
                let old = olds[o];
                last_indent = indent_at(ctx, old.start);
                let comment_start = if old.comments < pos {
                    old.start
                } else {
                    old.comments
                };
                if skip_removed {
                    if tag_in(pos, old.start) {
                        return None;
                    }
                } else {
                    result.push_str(&ctx.source[pos..comment_start]);
                }
                let nl = format!("\n{}", " ".repeat(last_indent));
                for item in delayed.drain(..) {
                    if let Some(c) = new_comments(item, last_indent) {
                        result.push_str(&c);
                        result.push_str(&nl);
                    }
                    result.push_str(&print_new(item, last_indent));
                    result.push_str(&nl);
                }
                // A node with the comments of the old one keeps their text; other comments are printed again.
                if matches!(d, Diff::Keep(..))
                    || (comments_of(&items[n]).is_empty() && old.comments == old.start)
                {
                    result.push_str(&ctx.source[comment_start..old.start]);
                } else if let Some(c) = new_comments(&items[n], last_indent) {
                    result.push_str(&c);
                    result.push_str(&nl);
                }
                skip_removed = false;
                result.push_str(&print_new(&items[n], last_indent));
                pos = old.end;
            }
            Diff::Add(n) => {
                if before_first_keep {
                    delayed.push(&items[n]);
                    continue;
                }
                let nl = format!("\n{}", " ".repeat(last_indent));
                result.push_str(&nl);
                if let Some(c) = new_comments(&items[n], last_indent) {
                    result.push_str(&c);
                    result.push_str(&nl);
                }
                result.push_str(&print_new(&items[n], last_indent));
            }
            Diff::Remove(o) => {
                let old = olds[o];
                if i == 0 {
                    result.push_str(&ctx.source[pos..old.comments]);
                    skip_removed = true;
                } else if tag_in(pos, old.comments) {
                    return None;
                }
                pos = old.end;
            }
        }
    }
    if skip_removed || !delayed.is_empty() {
        return None;
    }
    // Rector's `pArray`: with a `Nop` in the list, lines of blanks lose them.
    if items.iter().any(|i| i.old().is_some_and(|o| olds[o].nop)) {
        result = result
            .split('\n')
            .map(|l| {
                if l.trim_matches([' ', '\t']).is_empty() {
                    ""
                } else {
                    l
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
    }
    Some((result, pos))
}

/// `pStmts`: each statement on a line of its own at `indent`, after its comments; the old ones keep their text, moved
/// to `indent`.
pub(crate) fn p_stmts(
    ctx: &Context<'_>,
    olds: &[OldStmt],
    items: &[Item<'_>],
    indent: usize,
) -> String {
    let nl = format!("\n{}", " ".repeat(indent));
    let mut out = String::new();
    for item in items {
        let comments = match item {
            Item::New { comments, .. } => comments.clone(),
            Item::Old(i) | Item::Changed(i, _) => old_comments(ctx, &olds[*i]),
        };
        if !comments.is_empty() {
            out.push_str(&nl);
            out.push_str(&p_comments(ctx, &comments, indent));
        }
        match item {
            Item::Old(i) if olds[*i].nop => {}
            Item::Old(i) => {
                out.push_str(&nl);
                out.push_str(&kept_text(
                    ctx,
                    Span::new(olds[*i].start, olds[*i].end),
                    indent,
                ));
            }
            Item::Changed(i, text) => {
                // The changed text stands at the old statement's indent; the printer moves it as a whole.
                let delta = indent as isize - indent_at(ctx, olds[*i].start) as isize;
                out.push_str(&nl);
                out.push_str(&shift_text(text, delta));
            }
            Item::New { print, .. } => {
                out.push_str(&nl);
                out.push_str(&print(indent));
            }
        }
    }
    out
}

/// [`shift`] on text that is no longer in the file: every newline moved by `delta`.
fn shift_text(text: &str, delta: isize) -> String {
    match delta {
        0 => text.to_string(),
        d if d < 0 => text.replace(&format!("\n{}", " ".repeat(d.unsigned_abs())), "\n"),
        d => text.replace('\n', &format!("\n{}", " ".repeat(d as usize))),
    }
}

/// Rector's rewrite of the statement list `list` of `holder` (under `ancestors`) into `items`: the list printed
/// again in place, or, where php-parser's printer gives up on the list, its parent printed again. Only a closure, a
/// function, a method and a file without a namespace are printed again; elsewhere the rewrite is refused.
pub(crate) fn rewrite_list(
    ctx: &Context<'_>,
    holder: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    list: &super::php_parser::List<'_, '_>,
    items: &[Item<'_>],
) -> Option<Vec<Edit>> {
    let parent = ancestors.last().copied();
    let grandparent = ancestors.len().checked_sub(2).map(|i| ancestors[i]);
    use super::php_parser::Frame;
    let olds = old_statements(ctx, list);
    let braces = matches!(list.frame, Frame::Braces { .. });
    let program = matches!(holder, Node::Program(_));
    let count_changed = program
        && olds.iter().filter(|o| !o.nop).count()
            != items
                .iter()
                .filter(|i| i.old().is_none_or(|o| !olds[o].nop))
                .count();
    if let Some((text, end)) = print_list(ctx, &olds, braces, list.inner.start, items) {
        let mut edits = vec![Edit::new(Span::new(list.inner.start, end), text)];
        if count_changed && !ctx.source.ends_with('\n') {
            edits.push(Edit::new(
                Span::new(ctx.source.len(), ctx.source.len()),
                "\n",
            ));
        }
        return Some(edits);
    }
    let parent = match parent {
        Some(Node::MethodBody(_)) => grandparent,
        p => p,
    };
    match (holder, parent) {
        (Node::Block(_), Some(Node::Closure(closure))) => {
            let span = node_span(closure);
            let indent = indent_at(ctx, span.start);
            let mut out = String::new();
            for attributes in closure.attribute_lists.iter() {
                out.push_str(&kept_text(ctx, node_span(attributes), indent));
                out.push(' ');
            }
            if closure.r#static.is_some() {
                out.push_str("static ");
            }
            out.push_str("function ");
            if closure.ampersand.is_some() {
                out.push('&');
            }
            out.push('(');
            out.push_str(&print_params(ctx, &closure.parameter_list, indent));
            out.push(')');
            if let Some(uses) = &closure.use_clause {
                let vars: Vec<&str> = uses.variables.iter().map(|v| text(ctx, v)).collect();
                out.push_str(&format!(" use ({})", vars.join(", ")));
            }
            if let Some(hint) = &closure.return_type_hint {
                out.push_str(": ");
                out.push_str(&kept_text(ctx, node_span(&hint.hint), indent));
            }
            out.push_str(" {");
            out.push_str(&p_stmts(ctx, &olds, items, indent + 4));
            out.push('\n');
            out.push_str(&" ".repeat(indent));
            out.push('}');
            Some(vec![Edit::new(span, out)])
        }
        (Node::Block(_), Some(Node::Function(function))) => {
            let span = node_span(function);
            let indent = indent_at(ctx, span.start);
            let head = format!(
                "function {}{}",
                if function.ampersand.is_some() {
                    "&"
                } else {
                    ""
                },
                text(ctx, &function.name)
            );
            let out = print_function(
                ctx,
                &olds,
                items,
                indent,
                (&function.attribute_lists, head),
                &function.parameter_list,
                function.return_type_hint.as_ref(),
            );
            Some(vec![Edit::new(span, out)])
        }
        (Node::Block(_), Some(Node::Method(method))) => {
            let span = node_span(method);
            let indent = indent_at(ctx, span.start);
            use mago_syntax::cst::Modifier as M;
            let has = |f: fn(&M<'_>) -> bool| method.modifiers.iter().any(f);
            let mut head = String::new();
            for (present, word) in [
                (has(|m| matches!(m, M::Final(_))), "final "),
                (has(|m| matches!(m, M::Abstract(_))), "abstract "),
                (has(|m| matches!(m, M::Public(_))), "public "),
                (has(|m| matches!(m, M::Protected(_))), "protected "),
                (has(|m| matches!(m, M::Private(_))), "private "),
                (has(|m| matches!(m, M::Static(_))), "static "),
            ] {
                if present {
                    head.push_str(word);
                }
            }
            head.push_str("function ");
            if method.ampersand.is_some() {
                head.push('&');
            }
            head.push_str(text(ctx, &method.name));
            let out = print_function(
                ctx,
                &olds,
                items,
                indent,
                (&method.attribute_lists, head),
                &method.parameter_list,
                method.return_type_hint.as_ref(),
            );
            Some(vec![Edit::new(span, out)])
        }
        (Node::Program(_), _) => {
            // The whole file, from `<?php` on; a file with HTML or a closing tag before its end is left alone.
            let tokens = super::php_parser::tokens(ctx.source);
            use mago_syntax::token::TokenKind as K;
            let first = tokens.first()?;
            if first.kind != K::OpenTag || first.start != 0 {
                return None;
            }
            let tag = tokens.iter().skip(1).find(|t| {
                matches!(
                    t.kind,
                    K::CloseTag | K::InlineText | K::OpenTag | K::EchoTag | K::ShortOpenTag
                )
            });
            if let Some(t) = tag
                && (t.kind != K::CloseTag || !matches!(&ctx.source[t.end..], "" | "\n"))
            {
                return None;
            }
            let mut out = format!("<?php\n{}", p_stmts(ctx, &olds, items, 0));
            if count_changed && !out.ends_with('\n') {
                out.push('\n');
            }
            Some(vec![Edit::new(Span::new(0, ctx.source.len()), out)])
        }
        _ => None,
    }
}

/// `pParams` of a function-like printed again: its parameters' text, joined by `, `, or a line each when one owns a
/// comment.
fn print_params(
    ctx: &Context<'_>,
    list: &mago_syntax::cst::FunctionLikeParameterList<'_>,
    indent: usize,
) -> String {
    let params: Vec<Span> = list.parameters.iter().map(|p| node_span(p)).collect();
    if !params.iter().any(|p| has_comments(ctx, p.start)) {
        return params
            .iter()
            .map(|p| kept_text(ctx, *p, indent))
            .collect::<Vec<_>>()
            .join(", ");
    }
    let inner = indent + 4;
    let nl = format!("\n{}", " ".repeat(inner));
    let mut out = String::new();
    for p in &params {
        if has_comments(ctx, p.start) {
            out.push_str(&nl);
            out.push_str(&p_comments(ctx, &comment_spans(ctx, p.start), inner));
        }
        out.push_str(&nl);
        out.push_str(&kept_text(ctx, *p, inner));
        if !std::ptr::eq(p, params.last().unwrap()) {
            out.push(',');
        }
    }
    out.push('\n');
    out.push_str(&" ".repeat(indent));
    out
}

/// The nodes that hold a statement list (`NodeGroup::STMTS_AWARE`), as Mago has them: a single statement under a
/// control structure is left out, as no rule here changes a list of one.
pub(crate) const STATEMENT_LISTS: &[NodeKind] = &[
    NodeKind::Program,
    NodeKind::Block,
    NodeKind::NamespaceImplicitBody,
    NodeKind::SwitchExpressionCase,
    NodeKind::SwitchDefaultCase,
    NodeKind::IfColonDelimitedBody,
    NodeKind::IfColonDelimitedBodyElseIfClause,
    NodeKind::IfColonDelimitedBodyElseClause,
    NodeKind::WhileColonDelimitedBody,
    NodeKind::ForColonDelimitedBody,
    NodeKind::ForeachColonDelimitedBody,
    NodeKind::DeclareColonDelimitedBody,
];

/// The list `node` holds, as php-parser's `stmts`: PHP's `?>` takes the newline after it, which Mago keeps as
/// inline HTML.
pub(crate) fn statement_list<'ast, 'arena>(
    ctx: &Context<'_>,
    node: Node<'ast, 'arena>,
) -> super::php_parser::List<'ast, 'arena> {
    let tokens = super::php_parser::tokens(ctx.source);
    let mut list = super::php_parser::holder_list(ctx.source, &tokens, node);
    list.stmts.retain(|s| match s {
        Statement::Inline(inline) => {
            !(matches!(inline.value, b"\n" | b"\r\n")
                && ctx.source[..node_span(inline).start].ends_with("?>"))
        }
        _ => true,
    });
    list
}

/// `$stmt->expr` of an expression statement.
pub(crate) fn statement_expression<'a>(statement: &'a Statement<'a>) -> Option<&'a Expression<'a>> {
    match statement {
        Statement::Expression(s) => Some(s.expression),
        _ => None,
    }
}

/// `pStmt_Function` / `pStmt_ClassMethod` printed again at `indent`: the attributes a line each, `head` (modifiers,
/// `function` and the name), the parameters, the return type, and the body with its `{` on a line of its own.
fn print_function(
    ctx: &Context<'_>,
    olds: &[OldStmt],
    items: &[Item<'_>],
    indent: usize,
    (attributes, head): (
        &mago_syntax::cst::Sequence<'_, mago_syntax::cst::AttributeList<'_>>,
        String,
    ),
    params: &mago_syntax::cst::FunctionLikeParameterList<'_>,
    return_type: Option<&mago_syntax::cst::FunctionLikeReturnTypeHint<'_>>,
) -> String {
    let nl = format!("\n{}", " ".repeat(indent));
    let mut out = String::new();
    for list in attributes.iter() {
        out.push_str(&kept_text(ctx, node_span(list), indent));
        out.push_str(&nl);
    }
    out.push_str(&head);
    out.push('(');
    out.push_str(&print_params(ctx, params, indent));
    out.push(')');
    if let Some(hint) = return_type {
        out.push_str(": ");
        out.push_str(&kept_text(ctx, node_span(&hint.hint), indent));
    }
    out.push_str(&nl);
    out.push('{');
    out.push_str(&p_stmts(ctx, olds, items, indent + 4));
    out.push_str(&nl);
    out.push('}');
    out
}
