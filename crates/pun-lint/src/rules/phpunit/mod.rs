//! The `phpunit` group: rules ported from rector-phpunit (rectorphp/rector-phpunit at e68085d, rules/...), each in its
//! own file, and what they share: `TestsNodeAnalyzer`, `IdentifierManipulator` and `ArgumentMover`.
//!
//! Rector asks PHPStan whether a class is a `PHPUnit\Framework\TestCase` and what type a call's receiver has. Pun reads
//! the class-like a node is in from its declaration and its ancestors from the codebase ([`facts`]), which lists a
//! parent class by name even when its file is not known; a receiver other than `$this`, `self`, `static` and `parent`
//! takes the type Mago gives it.

mod add_depended_test_return_type;
mod annotations;
mod no_assert_contains_only;
mod no_assert_is_type;
mod no_assert_same_type_mismatch;
mod no_at_matcher;
mod no_bare_mock_assign;
mod no_bare_mock_builder;
mod no_empty_test;
mod no_expects_any;
mod no_final_constructor_override;
mod no_flipped_assertion;
mod no_float_delta_parameter;
mod no_negated_assertion;
mod no_setup_override_attribute;
mod no_test_constructor;
mod no_unused_mock;
mod no_value_object_mock;
mod prefer_assert;
mod prefer_assert_array_has_key;
mod prefer_assert_count;
mod prefer_assert_instanceof;
mod prefer_assert_instances_of;
mod prefer_assert_null_true_false;
mod prefer_assert_string_contains;
mod prefer_assert_true_false;
mod prefer_comparison_assertion;
mod prefer_create_stub_in_coalesce;
mod prefer_merged_will_return_callback;
mod prefer_named_data_provider_keys;
mod prefer_narrow_will_return_callback;
mod prefer_narrow_with_consecutive;
mod prefer_same_for_scalars;
mod prefer_simple_callback_assert;
mod prefer_simple_mock_expectation;
mod prefer_simple_with_matcher;
mod prefer_specific_true_false_assertion;
mod prefer_specific_will_method;
mod prefer_standalone_asserts_over_callback;
mod prefer_test_prefix;
mod prefer_this_call;
mod prefer_ticket_attribute;
mod prefer_with_is_instance_of;
mod ptype;
mod require_data_provider_newlines;
mod require_strict_types_in_tests;
mod require_string_assert_argument;

pub use add_depended_test_return_type::AddDependedTestReturnType;
pub use no_assert_contains_only::NoAssertContainsOnly;
pub use no_assert_is_type::NoAssertIsType;
pub use no_assert_same_type_mismatch::NoAssertSameTypeMismatch;
pub use no_at_matcher::NoAtMatcher;
pub use no_bare_mock_assign::NoBareMockAssign;
pub use no_bare_mock_builder::NoBareMockBuilder;
pub use no_empty_test::NoEmptyTest;
pub use no_expects_any::NoExpectsAny;
pub use no_final_constructor_override::NoFinalConstructorOverride;
pub use no_flipped_assertion::NoFlippedAssertion;
pub use no_float_delta_parameter::NoFloatDeltaParameter;
pub use no_negated_assertion::NoNegatedAssertion;
pub use no_setup_override_attribute::NoSetupOverrideAttribute;
pub use no_test_constructor::NoTestConstructor;
pub use no_unused_mock::NoUnusedMock;
pub use no_value_object_mock::NoValueObjectMock;
pub use prefer_assert::PreferAssert;
pub use prefer_assert_array_has_key::PreferAssertArrayHasKey;
pub use prefer_assert_count::PreferAssertCount;
pub use prefer_assert_instanceof::PreferAssertInstanceof;
pub use prefer_assert_instances_of::PreferAssertInstancesOf;
pub use prefer_assert_null_true_false::PreferAssertNullTrueFalse;
pub use prefer_assert_string_contains::PreferAssertStringContains;
pub use prefer_assert_true_false::PreferAssertTrueFalse;
pub use prefer_comparison_assertion::PreferComparisonAssertion;
pub use prefer_create_stub_in_coalesce::PreferCreateStubInCoalesce;
pub use prefer_merged_will_return_callback::PreferMergedWillReturnCallback;
pub use prefer_named_data_provider_keys::PreferNamedDataProviderKeys;
pub use prefer_narrow_will_return_callback::PreferNarrowWillReturnCallback;
pub use prefer_narrow_with_consecutive::PreferNarrowWithConsecutive;
pub use prefer_same_for_scalars::PreferSameForScalars;
pub use prefer_simple_callback_assert::PreferSimpleCallbackAssert;
pub use prefer_simple_mock_expectation::PreferSimpleMockExpectation;
pub use prefer_simple_with_matcher::PreferSimpleWithMatcher;
pub use prefer_specific_true_false_assertion::PreferSpecificTrueFalseAssertion;
pub use prefer_specific_will_method::PreferSpecificWillMethod;
pub use prefer_standalone_asserts_over_callback::PreferStandaloneAssertsOverCallback;
pub use prefer_test_prefix::PreferTestPrefix;
pub use prefer_this_call::PreferThisCall;
pub use prefer_ticket_attribute::PreferTicketAttribute;
pub use prefer_with_is_instance_of::PreferWithIsInstanceOf;
pub use require_data_provider_newlines::RequireDataProviderNewlines;
pub use require_strict_types_in_tests::RequireStrictTypesInTests;
pub use require_string_assert_argument::RequireStringAssertArgument;

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{
    Argument, ArgumentList, ClassLikeMemberSelector, Expression, Identifier, Method, Modifier,
    Node, Variable,
};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::rules::laravel::is_a;
use crate::rules::types::class_scope;
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// The group's rules, in a fixed order.
pub(crate) fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(NoEmptyTest),
        Box::new(NoNegatedAssertion),
        Box::new(PreferAssertNullTrueFalse),
        Box::new(PreferAssertTrueFalse),
        Box::new(PreferAssertStringContains),
        Box::new(NoAssertIsType),
        Box::new(NoAssertContainsOnly),
        Box::new(PreferComparisonAssertion),
        Box::new(PreferAssertInstanceof),
        Box::new(NoFlippedAssertion),
        Box::new(NoFloatDeltaParameter),
        Box::new(PreferSpecificTrueFalseAssertion),
        Box::new(PreferAssertArrayHasKey),
        Box::new(RequireStringAssertArgument),
        Box::new(PreferAssertInstancesOf),
        Box::new(NoAssertSameTypeMismatch),
        Box::new(PreferSameForScalars),
        Box::new(PreferAssertCount),
        Box::new(NoExpectsAny),
        Box::new(PreferSimpleWithMatcher),
        Box::new(PreferSpecificWillMethod),
        Box::new(PreferSimpleMockExpectation),
        Box::new(PreferNarrowWithConsecutive),
        Box::new(NoFinalConstructorOverride),
        Box::new(NoSetupOverrideAttribute),
        Box::new(RequireStrictTypesInTests),
        Box::new(PreferThisCall),
        Box::new(NoUnusedMock),
        Box::new(NoAtMatcher),
        Box::new(NoBareMockBuilder),
        Box::new(PreferCreateStubInCoalesce),
        Box::new(RequireDataProviderNewlines),
        Box::new(PreferWithIsInstanceOf),
        Box::new(PreferSimpleCallbackAssert),
        Box::new(PreferNarrowWillReturnCallback),
        Box::new(PreferAssert),
        Box::new(PreferMergedWillReturnCallback),
        Box::new(AddDependedTestReturnType),
        Box::new(NoTestConstructor),
        Box::new(PreferNamedDataProviderKeys),
        Box::new(PreferTestPrefix),
        Box::new(NoValueObjectMock),
        Box::new(NoBareMockAssign),
        Box::new(PreferStandaloneAssertsOverCallback),
        Box::new(PreferTicketAttribute),
    ]
}

pub(crate) const TEST_CASE: &str = "PHPUnit\\Framework\\TestCase";
pub(crate) const TEST_CASE_LEGACY: &str = "PHPUnit_Framework_TestCase";
pub(crate) const ASSERT: &str = "PHPUnit\\Framework\\Assert";

/// `TestsNodeAnalyzer::isInTestClass` for a node whose ancestors are `ancestors`; `method` is the node itself when it
/// is a method (a trait's test methods are told apart by it).
pub(crate) fn is_in_test_class(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    method: Option<&Method<'_>>,
) -> bool {
    let Some(at) = ancestors.iter().rposition(|a| {
        matches!(
            a,
            Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
        )
    }) else {
        return false;
    };
    let Some(scope) = class_scope(&ancestors[..=at]) else {
        return false;
    };
    if matches!(ancestors[at], Node::Trait(_)) {
        let name = scope.name.unwrap_or_default();
        let in_tests_namespace = name
            .rsplit_once('\\')
            .is_some_and(|(ns, _)| ns.split('\\').any(|part| part == "Test" || part == "Tests"));
        let Some(method) = method else {
            return in_tests_namespace;
        };
        if !is_public(method)
            || method
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Static(_)))
        {
            return false;
        }
        return in_tests_namespace || method.name.value.starts_with(b"test");
    }
    class_is(ctx, &ancestors[..=at], &[TEST_CASE, TEST_CASE_LEGACY])
}

/// `ClassReflection::is()` for the class-like `ancestors` ends in, against any of `targets`.
fn class_is(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], targets: &[&str]) -> bool {
    let Some(scope) = class_scope(ancestors) else {
        return false;
    };
    match &scope.name {
        Some(name) => is_a(ctx, name, targets),
        None => {
            let Some(Node::AnonymousClass(c)) = ancestors.last() else {
                return false;
            };
            let name = facts(ctx).anonymous_class_name(mago_span::HasSpan::span(*c));
            is_a(ctx, &name, targets) || scope.parents.iter().any(|p| is_a(ctx, p, targets))
        }
    }
}

/// Whether a method is public (no visibility modifier counts as public).
pub(crate) fn is_public(method: &Method<'_>) -> bool {
    !method
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Private(_) | Modifier::Protected(_)))
}

/// A method or static method call, as php-parser's `MethodCall` and `StaticCall` (a first-class callable is a
/// partial application in Mago, never one of these).
#[derive(Clone, Copy)]
pub(crate) struct AnyCall<'a> {
    pub span: Span,
    /// The object of a method call, or the class of a static one.
    pub receiver: &'a Expression<'a>,
    pub is_static: bool,
    pub method: &'a ClassLikeMemberSelector<'a>,
    pub args: &'a ArgumentList<'a>,
}

impl<'a> AnyCall<'a> {
    pub(crate) fn of(node: Node<'a, 'a>) -> Option<AnyCall<'a>> {
        match node {
            Node::MethodCall(m) => Some(AnyCall {
                span: node_span(m),
                receiver: m.object,
                is_static: false,
                method: &m.method,
                args: &m.argument_list,
            }),
            Node::StaticMethodCall(s) => Some(AnyCall {
                span: node_span(s),
                receiver: s.class,
                is_static: true,
                method: &s.method,
                args: &s.argument_list,
            }),
            _ => None,
        }
    }

    /// The method name when written as an identifier.
    pub(crate) fn name(&self) -> Option<&'a [u8]> {
        match self.method {
            ClassLikeMemberSelector::Identifier(id) => Some(id.value),
            _ => None,
        }
    }

    /// The span of the method name.
    pub(crate) fn name_span(&self) -> Span {
        node_span(self.method)
    }

    /// `NodeNameResolver::isNames` on the method name: ASCII case-insensitive.
    pub(crate) fn is_name(&self, names: &[&str]) -> bool {
        self.name()
            .is_some_and(|n| names.iter().any(|w| n.eq_ignore_ascii_case(w.as_bytes())))
    }

    /// `getArgs()`.
    pub(crate) fn args(&self) -> Vec<&'a Argument<'a>> {
        self.args.arguments.iter().collect()
    }

    /// The receiver and its `->` or `::` as php-parser's pretty printer writes them for a call node it prints anew
    /// (`pDereferenceLhs($node->var) . '->'`): the kept receiver's own text, without what stood between it and the
    /// operator.
    pub(crate) fn receiver_text(&self, ctx: &Context<'_>) -> String {
        let operator = if self.is_static { "::" } else { "->" };
        format!("{}{operator}", text(ctx, self.receiver))
    }
}

/// `TestsNodeAnalyzer::isPHPUnitTestCaseCall`: the receiver's type is a `TestCase` or an `Assert` (`ObjectType`
/// only, so not a union, an intersection or a trait's `$this`).
pub(crate) fn is_test_case_call(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    call: &AnyCall<'_>,
) -> bool {
    receiver_is(ctx, ancestors, call, &[TEST_CASE, ASSERT])
}

/// `TestsNodeAnalyzer::isPHPUnitMethodCallNames`.
pub(crate) fn is_test_case_call_named(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    call: &AnyCall<'_>,
    names: &[&str],
) -> bool {
    call.is_name(names) && is_test_case_call(ctx, ancestors, call)
}

/// Whether the receiver of `call` is an `ObjectType` of one of `targets` or a subclass.
pub(crate) fn receiver_is(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    call: &AnyCall<'_>,
    targets: &[&str],
) -> bool {
    let receiver = unparenthesize(call.receiver);
    if call.is_static {
        return match receiver {
            Expression::Self_(_) | Expression::Static(_) => {
                own_class(ancestors).is_some_and(|c| is_a(ctx, &c, targets))
            }
            // A union of every parent class unless there is only one.
            Expression::Parent(_) => own_class(ancestors).is_some_and(|own| {
                let parents = class_parents(ctx, &own);
                parents.len() == 1 && is_a(ctx, &parents[0], targets)
            }),
            Expression::Identifier(id) => is_a(ctx, &resolve(ancestors, id), targets),
            _ => false,
        };
    }
    if let Expression::Variable(Variable::Direct(v)) = receiver
        && v.name == b"$this"
    {
        return own_class_like(ancestors).is_some_and(|c| class_is(ctx, c, targets));
    }
    object_class(ctx, receiver).is_some_and(|c| is_a(ctx, &c, targets))
}

/// The class of an `ObjectType` (a single named object, not a union or an intersection) that Mago gives `expr`.
pub(crate) fn object_class(ctx: &Context<'_>, expr: &Expression<'_>) -> Option<String> {
    let facts = facts(ctx);
    let t = facts.full_type(node_span(expr))?;
    match t.types.as_ref() {
        [TAtomic::Object(TObject::Named(named))] if named.intersection_types.is_none() => {
            Some(named.name.to_string())
        }
        _ => None,
    }
}

/// The parent classes of `class`, nearest first.
pub(crate) fn class_parents(ctx: &Context<'_>, class: &str) -> Vec<String> {
    let facts = facts(ctx);
    let mut out = Vec::new();
    let mut at = class.to_string();
    while let Some(c) = facts.codebase.get_class_like(at.as_bytes()) {
        let Some(parent) = c.direct_parent_class else {
            break;
        };
        let name = facts
            .codebase
            .get_class_like(parent.as_bytes())
            .map_or(parent.to_string(), |p| p.original_name.to_string());
        if out.contains(&name) {
            break;
        }
        out.push(name.clone());
        at = name;
    }
    out
}

/// The nodes up to the innermost class-like (not a trait) of `ancestors`.
fn own_class_like<'n, 'ast, 'arena>(
    ancestors: &'n [Node<'ast, 'arena>],
) -> Option<&'n [Node<'ast, 'arena>]> {
    let at = ancestors.iter().rposition(|a| {
        matches!(
            a,
            Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
        )
    })?;
    (!matches!(ancestors[at], Node::Trait(_))).then(|| &ancestors[..=at])
}

/// The fully qualified name of the named class-like (not a trait or an anonymous class) `ancestors` lead into.
pub(crate) fn own_class(ancestors: &[Node<'_, '_>]) -> Option<String> {
    let nodes = own_class_like(ancestors)?;
    class_scope(nodes)?.name
}

/// A class name as written at the node `ancestors` lead to, resolved.
pub(crate) fn resolve(ancestors: &[Node<'_, '_>], id: &Identifier<'_>) -> String {
    crate::rules::rector_batch5::resolve_class_name(ancestors, id)
}

/// A report on `span` with a fix made of `edits`.
pub(crate) fn report(span: Span, message: &str, safety: Safety, edits: Vec<Edit>) -> Report {
    Report {
        span,
        message: message.to_string(),
        help: None,
        fix: Some(Fix { safety, edits }),
    }
}

/// The members of a class-like and the spans of its braces.
pub(crate) fn class_body<'a>(
    node: Node<'a, 'a>,
) -> Option<(
    &'a mago_syntax::cst::Sequence<'a, mago_syntax::cst::ClassLikeMember<'a>>,
    mago_span::Span,
    mago_span::Span,
)> {
    match node {
        Node::Class(c) => Some((&c.members, c.left_brace, c.right_brace)),
        Node::AnonymousClass(c) => Some((&c.members, c.left_brace, c.right_brace)),
        Node::Trait(t) => Some((&t.members, t.left_brace, t.right_brace)),
        Node::Interface(i) => Some((&i.members, i.left_brace, i.right_brace)),
        Node::Enum(e) => Some((&e.members, e.left_brace, e.right_brace)),
        _ => None,
    }
}

/// The class-like node kinds.
pub(crate) const CLASS_LIKES: &[mago_syntax::cst::NodeKind] = &[
    mago_syntax::cst::NodeKind::Class,
    mago_syntax::cst::NodeKind::AnonymousClass,
    mago_syntax::cst::NodeKind::Trait,
    mago_syntax::cst::NodeKind::Interface,
    mago_syntax::cst::NodeKind::Enum,
];

/// An argument's value.
pub(crate) fn arg_value<'a>(arg: &'a Argument<'a>) -> &'a Expression<'a> {
    match arg {
        Argument::Positional(p) => p.value,
        Argument::Named(n) => n.value,
    }
}

/// The source text of a node.
pub(crate) fn text<'s>(ctx: &Context<'s>, node: &impl mago_span::HasSpan) -> &'s str {
    let s = node_span(node);
    &ctx.source[s.start..s.end]
}

/// The text php-parser prints for an expression node it keeps: the node without the parentheses around it.
pub(crate) fn expr_text<'s>(ctx: &Context<'s>, expr: &Expression<'_>) -> &'s str {
    text(ctx, unparenthesize(expr))
}

/// The call's arguments from `from` on, kept.
pub(crate) fn old_args(call: &AnyCall<'_>, from: usize) -> impl Iterator<Item = NewArg> {
    (from..call.args.arguments.len()).map(NewArg::Old)
}

/// `ArgumentMover::removeFirstArg` with the call renamed to `name`.
pub(crate) fn remove_first_arg(ctx: &Context<'_>, call: &AnyCall<'_>, name: &str) -> Vec<Edit> {
    let new: Vec<NewArg> = old_args(call, 1).collect();
    rewrite_args(ctx, call, Some(name), &new)
}

/// `IdentifierManipulator::renameNodeWithMap`: the new name for the call's name as written, a key of `map`.
pub(crate) fn renamed(call: &AnyCall<'_>, map: &[(&str, &'static str)]) -> Option<&'static str> {
    let name = call.name()?;
    map.iter()
        .find(|(k, _)| k.as_bytes() == name)
        .map(|(_, v)| *v)
}

/// The call printed anew as php-parser prints `new MethodCall($node->var, $name, $args)` (or a `StaticCall`): the
/// kept receiver, the name and the arguments joined with `, `.
pub(crate) fn new_call(ctx: &Context<'_>, call: &AnyCall<'_>, name: &str, args: &[&str]) -> String {
    format!("{}{name}({})", call.receiver_text(ctx), args.join(", "))
}

/// Whether the expression is the constant `true`, `false` or `null` named `name` (ASCII case-insensitive).
pub(crate) fn is_constant(expr: &Expression<'_>, name: &str) -> bool {
    use mago_syntax::cst::Literal;
    match unparenthesize(expr) {
        Expression::Literal(Literal::True(_)) => name == "true",
        Expression::Literal(Literal::False(_)) => name == "false",
        Expression::Literal(Literal::Null(_)) => name == "null",
        Expression::ConstantAccess(c) => {
            let n = c.name.value();
            n.strip_prefix(b"\\")
                .unwrap_or(n)
                .eq_ignore_ascii_case(name.as_bytes())
        }
        _ => false,
    }
}

/// `ValueResolver::getValue` for a string: a string literal.
pub(crate) fn string_value(expr: &Expression<'_>) -> Option<String> {
    match crate::rules::rector_batch5::value(expr)? {
        crate::rules::rector_batch5::Value::Str(s) => Some(s),
        _ => None,
    }
}

/// `ValueResolver::getValue` for a class name: a string literal, or `X::class` resolved.
pub(crate) fn class_string(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> Option<String> {
    use mago_syntax::cst::{Access, ClassLikeConstantSelector};
    match unparenthesize(expr) {
        Expression::Access(Access::ClassConstant(access)) => {
            let (Expression::Identifier(class), ClassLikeConstantSelector::Identifier(constant)) =
                (access.class, &access.constant)
            else {
                return None;
            };
            constant
                .value
                .eq_ignore_ascii_case(b"class")
                .then(|| resolve(ancestors, class))
        }
        expr => string_value(expr),
    }
}

/// `FlipAssertRector::isScalarValue`: a php-parser `Scalar` (a literal, a magic constant, an interpolated string), a
/// constant or a class constant.
pub(crate) fn is_scalar_value(expr: &Expression<'_>) -> bool {
    use mago_syntax::cst::{Access, CompositeString};
    matches!(
        unparenthesize(expr),
        Expression::Literal(_)
            | Expression::MagicConstant(_)
            | Expression::CompositeString(
                CompositeString::Interpolated(_) | CompositeString::Document(_)
            )
            | Expression::ConstantAccess(_)
            | Expression::Access(Access::ClassConstant(_))
    )
}

/// Whether PHPStan's type of `expr` is a `ConstantScalarType` (a literal value or `null`) or a `ConstantArrayType`
/// (an array whose items are all known). A literal and a magic constant are, whatever type Mago gives them.
pub(crate) fn is_constant_value(ctx: &Context<'_>, expr: &Expression<'_>) -> bool {
    use mago_codex::ttype::atomic::array::TArray;
    let expr = unparenthesize(expr);
    if matches!(expr, Expression::Literal(_) | Expression::MagicConstant(_)) {
        return true;
    }
    let facts = facts(ctx);
    let Some(t) = facts.full_type(node_span(expr)) else {
        return false;
    };
    match t.types.as_ref() {
        [atomic] if atomic.is_literal() => true,
        [TAtomic::Array(TArray::Keyed(k))] => k.parameters.is_none(),
        [TAtomic::Array(TArray::List(l))] => l.element_type.is_never(),
        _ => false,
    }
}

/// An argument of a rewritten call: an argument the call keeps (by its index), or the text php-parser prints for an
/// argument node that is not one of the call's own (a new `Arg`, or an argument moved in from another call).
pub(crate) enum NewArg {
    Old(usize),
    New(String),
}

/// The edits for setting a call's arguments to `new` (and its name to `name`), as php-parser's format-preserving
/// printer writes them (`pArray` over the `args` list, with `Differ::diffWithReplacements`). Where `pArray` gives up
/// (a sole argument replaced by several, or every argument removed from the start), the call is printed anew:
/// receiver, name and the arguments joined with `, `.
pub(crate) fn rewrite_args(
    ctx: &Context<'_>,
    call: &AnyCall<'_>,
    name: Option<&str>,
    new: &[NewArg],
) -> Vec<Edit> {
    let source = ctx.source;
    let old: Vec<Span> = call.args.arguments.iter().map(node_span).collect();
    let printed = |a: &NewArg| match a {
        NewArg::Old(i) => source[old[*i].start..old[*i].end].to_string(),
        NewArg::New(text) => text.clone(),
    };
    let fallback = || {
        let name = name.map_or_else(
            || source[call.name_span().start..call.name_span().end].to_string(),
            str::to_string,
        );
        let args: Vec<String> = new.iter().map(printed).collect();
        vec![Edit::new(
            call.span,
            format!("{}{name}({})", call.receiver_text(ctx), args.join(", ")),
        )]
    };
    let keys: Vec<usize> = new
        .iter()
        .enumerate()
        .map(|(j, a)| match a {
            NewArg::Old(i) => *i,
            NewArg::New(_) => old.len() + j,
        })
        .collect();
    let diff = coalesce_replacements(diff(old.len(), &keys));
    let multiline = old.len() >= 2
        && old
            .windows(2)
            .all(|w| source[w[0].end..w[1].end].contains('\n'));
    let open = call.args.left_parenthesis.end.offset as usize;
    let mut pos = open;
    let mut result = String::new();
    let mut before_first_keep = true;
    let mut skip_removed = false;
    let mut delayed: Vec<String> = Vec::new();
    let mut indent = String::new();
    let comment_start = |from: usize, i: usize| {
        crate::rules::rector_stmts::first_comment(source, from, old[i].start)
            .unwrap_or(old[i].start)
    };
    for (index, elem) in diff.iter().enumerate() {
        match *elem {
            Elem::Keep(o, n) | Elem::Replace(o, n) => {
                before_first_keep = false;
                indent =
                    " ".repeat(crate::rules::rector_stmts::line_indent(source, old[o].start).len());
                let from = comment_start(pos, o);
                if !skip_removed {
                    result.push_str(&source[pos..from]);
                }
                for add in delayed.drain(..) {
                    result.push_str(&add);
                    if multiline {
                        result.push_str(&format!(",\n{indent}"));
                    } else {
                        result.push_str(", ");
                    }
                }
                let keep = matches!(elem, Elem::Keep(..));
                if keep {
                    result.push_str(&source[from..old[o].start]);
                }
                skip_removed = false;
                result.push_str(&printed(&new[n]));
                pos = old[o].end;
            }
            Elem::Add(n) => {
                if before_first_keep {
                    delayed.push(printed(&new[n]));
                    continue;
                }
                if multiline {
                    result.push_str(&format!(",\n{indent}"));
                } else {
                    result.push_str(", ");
                }
                result.push_str(&printed(&new[n]));
            }
            Elem::Remove(o) => {
                let start = comment_start(pos, o);
                if index == 0 {
                    result.push_str(&source[pos..start]);
                    skip_removed = true;
                }
                pos = old[o].end;
            }
        }
    }
    if skip_removed || !delayed.is_empty() && !old.is_empty() {
        return fallback();
    }
    let mut edits = Vec::new();
    if let Some(name) = name {
        edits.push(Edit::new(call.name_span(), name));
    }
    if !delayed.is_empty() {
        // An empty list: the new arguments go right after the `(`.
        edits.push(Edit::new(Span::new(open, open), delayed.join(", ")));
    } else {
        edits.push(Edit::new(Span::new(open, pos), result));
    }
    edits
}

/// An element of php-parser's `Differ`, on indexes of the old and the new list.
#[derive(Clone, Copy)]
enum Elem {
    Keep(usize, usize),
    Remove(usize),
    Add(usize),
    Replace(usize, usize),
}

/// php-parser's `Differ::diff` (Myers) from the items `0..n` to the list `new`, where `new[y] == x` means the old item
/// `x` is kept as the new item `y` (a new item has a key of `n` or above).
fn diff(n: usize, new: &[usize]) -> Vec<Elem> {
    let m = new.len();
    let max = (n + m) as isize;
    let at = |k: isize| (k + max + 1) as usize;
    let mut v = vec![0usize; 2 * max as usize + 3];
    let mut trace: Vec<Vec<usize>> = Vec::new();
    let (mut x, mut y) = (0, 0);
    'outer: for d in 0..=max {
        trace.push(v.clone());
        let mut k = -d;
        while k <= d {
            x = if k == -d || (k != d && v[at(k - 1)] < v[at(k + 1)]) {
                v[at(k + 1)]
            } else {
                v[at(k - 1)] + 1
            };
            y = (x as isize - k) as usize;
            while x < n && y < m && x == new[y] {
                x += 1;
                y += 1;
            }
            v[at(k)] = x;
            if x >= n && y >= m {
                break 'outer;
            }
            k += 2;
        }
    }
    let mut out = Vec::new();
    for d in (0..trace.len() as isize).rev() {
        let v = &trace[d as usize];
        let k = x as isize - y as isize;
        let prev_k = if k == -d || (k != d && v[at(k - 1)] < v[at(k + 1)]) {
            k + 1
        } else {
            k - 1
        };
        let prev_x = v[at(prev_k)];
        let prev_y = prev_x as isize - prev_k;
        while x > prev_x && y as isize > prev_y {
            out.push(Elem::Keep(x - 1, y - 1));
            x -= 1;
            y -= 1;
        }
        if d == 0 {
            break;
        }
        while x > prev_x {
            out.push(Elem::Remove(x - 1));
            x -= 1;
        }
        while y as isize > prev_y {
            out.push(Elem::Add(y - 1));
            y -= 1;
        }
    }
    out.reverse();
    out
}

/// `Differ::coalesceReplacements`: a run of removals followed by as many additions is a run of replacements.
fn coalesce_replacements(diff: Vec<Elem>) -> Vec<Elem> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < diff.len() {
        if !matches!(diff[i], Elem::Remove(_)) {
            out.push(diff[i]);
            i += 1;
            continue;
        }
        let mut j = i;
        while j < diff.len() && matches!(diff[j], Elem::Remove(_)) {
            j += 1;
        }
        let mut k = j;
        while k < diff.len() && matches!(diff[k], Elem::Add(_)) {
            k += 1;
        }
        if j - i == k - j {
            for n in 0..j - i {
                if let (Elem::Remove(old), Elem::Add(new)) = (diff[i + n], diff[j + n]) {
                    out.push(Elem::Replace(old, new));
                }
            }
        } else {
            out.extend_from_slice(&diff[i..k]);
        }
        i = k;
    }
    out
}

/// `NodeNameResolver::getName` on an expression: a variable's name, a property fetch's property, a constant's or a
/// called function's name as written (without a leading `\`).
pub(crate) fn expr_name<'a>(expr: &'a Expression<'a>) -> Option<&'a [u8]> {
    use mago_syntax::cst::{Access, Call};
    let strip = |n: &'a [u8]| n.strip_prefix(b"\\").unwrap_or(n);
    match unparenthesize(expr) {
        Expression::Variable(Variable::Direct(v)) => Some(&v.name[1..]),
        Expression::Access(Access::Property(p)) => match &p.property {
            ClassLikeMemberSelector::Identifier(id) => Some(id.value),
            _ => None,
        },
        Expression::Access(Access::NullSafeProperty(p)) => match &p.property {
            ClassLikeMemberSelector::Identifier(id) => Some(id.value),
            _ => None,
        },
        Expression::Access(Access::StaticProperty(p)) => match &p.property {
            Variable::Direct(v) => Some(&v.name[1..]),
            _ => None,
        },
        Expression::ConstantAccess(c) => Some(strip(c.name.value())),
        Expression::Call(Call::Function(f)) => match f.function {
            Expression::Identifier(id) => Some(strip(id.value())),
            _ => None,
        },
        _ => None,
    }
}

/// `BetterNodeFinder::findFirst`: whether `node` or a node inside it matches `found`.
pub(crate) fn contains(node: Node<'_, '_>, found: &dyn Fn(Node<'_, '_>) -> bool) -> bool {
    if found(node) {
        return true;
    }
    let mut hit = false;
    node.visit_children(|child| {
        if !hit && contains(child, found) {
            hit = true;
        }
    });
    hit
}

/// Whether the node is a method call named one of `names`.
pub(crate) fn is_method_call_named(node: Node<'_, '_>, names: &[&str]) -> bool {
    matches!(node, Node::MethodCall(_)) && AnyCall::of(node).is_some_and(|c| c.is_name(names))
}

/// The text of `expr` with every method call named `equalTo` inside it replaced by its first argument's value, as
/// `traverseNodesWithCallable` leaves it: the replacement of `expr` itself is lost (the traversed array is dropped),
/// and a replacement is traversed in turn. `None` when such a call has no argument (Rector stops there).
pub(crate) fn without_equal_to(ctx: &Context<'_>, expr: &Expression<'_>) -> Option<String> {
    use mago_syntax::cst::MethodCall;
    fn collect<'a>(node: Node<'a, 'a>, root: Span, calls: &mut Vec<&'a MethodCall<'a>>) {
        if let Node::MethodCall(m) = node
            && node_span(m) != root
            && is_method_call_named(node, &["equalTo"])
        {
            calls.push(m);
            return;
        }
        node.visit_children(|child| collect(child, root, calls));
    }
    let expr = unparenthesize(expr);
    let span = node_span(expr);
    let mut calls = Vec::new();
    Node::Expression(expr).visit_children(|child| collect(child, span, &mut calls));
    let mut out = String::new();
    let mut at = span.start;
    for m in calls {
        let call = node_span(m);
        out.push_str(&ctx.source[at..call.start]);
        out.push_str(&replaced_equal_to(ctx, m)?);
        at = call.end;
    }
    out.push_str(&ctx.source[at..span.end]);
    Some(out)
}

/// The text of an `equalTo()` call's replacement: its first argument's value, with its own `equalTo()` calls
/// replaced.
fn replaced_equal_to(ctx: &Context<'_>, call: &mago_syntax::cst::MethodCall<'_>) -> Option<String> {
    inner_without_equal_to(ctx, arg_value(call.argument_list.arguments.first()?))
}

/// [`without_equal_to`] for a node inside the traversed one, which is replaced too when it is an `equalTo()` call.
pub(crate) fn inner_without_equal_to(ctx: &Context<'_>, expr: &Expression<'_>) -> Option<String> {
    use mago_syntax::cst::Call;
    let value = unparenthesize(expr);
    if let Expression::Call(Call::Method(m)) = value
        && is_method_call_named(Node::MethodCall(m), &["equalTo"])
    {
        return replaced_equal_to(ctx, m);
    }
    without_equal_to(ctx, value)
}

/// `pStmt_ClassMethod` for a method php-parser prints anew without its attributes: the modifiers in php-parser's
/// order, the parameters joined with `, ` (a line each, without a trailing comma, when one has comments), and the
/// statements of its body `block` a line each (blank lines between them go).
pub(crate) fn method_anew(
    source: &str,
    method: &Method<'_>,
    block: &mago_syntax::cst::Block<'_>,
) -> String {
    use crate::rules::rector_stmts::{block_list, line_indent};
    let indent = line_indent(source, node_span(method).start);
    let mut out = crate::rules::rector_batch5::modifiers_text(&method.modifiers);
    out.push_str("function ");
    if method.ampersand.is_some() {
        out.push('&');
    }
    out.push_str(std::str::from_utf8(method.name.value).unwrap_or_default());
    let list = &method.parameter_list;
    let params: Vec<Span> = list.parameters.iter().map(node_span).collect();
    out.push('(');
    out.push_str(&maybe_multiline(
        source,
        list.left_parenthesis.end.offset as usize,
        &params,
        indent,
    ));
    out.push(')');
    if let Some(hint) = &method.return_type_hint {
        let s = node_span(&hint.hint);
        out.push_str(": ");
        out.push_str(&source[s.start..s.end]);
    }
    out.push('\n');
    out.push_str(indent);
    out.push_str(&block_anew(source, &block_list(source, block), indent));
    out
}

/// A block php-parser prints anew at the indent `indent` (`pStmts`): `{`, each statement on a line one level deeper
/// after its comments (a line each; the blank lines between go), the comments after the last statement (the `Nop`),
/// and `}` on a line of its own.
pub(crate) fn block_anew(
    source: &str,
    list: &crate::rules::rector_stmts::List<'_>,
    indent: &str,
) -> String {
    use crate::rules::rector_stmts::{line_indent, reindent, tokens};
    let inner = format!("{indent}    ");
    let mut lines = Vec::new();
    let comments = |from: usize, to: usize, lines: &mut Vec<String>| {
        for t in tokens(source, from, to)
            .into_iter()
            .filter(|t| t.kind.is_comment())
        {
            let span = Span::new(t.start, t.end);
            lines.push(reindent(source, span, line_indent(source, t.start), &inner));
        }
    };
    for (i, statement) in list.stmts.iter().enumerate() {
        let span = node_span(*statement);
        comments(list.before(source, i), span.start, &mut lines);
        lines.push(reindent(
            source,
            span,
            line_indent(source, span.start),
            &inner,
        ));
    }
    if let Some(nop) = list.nop_span(source) {
        comments(nop.start, nop.end, &mut lines);
    }
    if lines.is_empty() {
        return format!("{{\n{indent}}}");
    }
    format!(
        "{{\n{inner}{}\n{indent}}}",
        lines.join(&format!("\n{inner}"))
    )
}

/// php-parser's `pMaybeMultiline` over items whose source spans are `items` (the first starts after `from`), printed
/// at the indent `indent`: the items joined with `, `, or, when one has comments before it, each item on a line of its
/// own one level deeper after its comments, without a trailing comma, and a line break before the closing token.
pub(crate) fn maybe_multiline(source: &str, from: usize, items: &[Span], indent: &str) -> String {
    use crate::rules::rector_stmts::{previous_token_end, tokens};
    let mut from = from;
    let mut printed = Vec::new();
    for span in items {
        let before: Vec<&str> = tokens(
            source,
            previous_token_end(source, from, span.start),
            span.start,
        )
        .into_iter()
        .filter(|t| t.kind.is_comment())
        .map(|t| &source[t.start..t.end])
        .collect();
        printed.push((before, &source[span.start..span.end]));
        from = span.end;
    }
    if printed.iter().all(|(c, _)| c.is_empty()) {
        let texts: Vec<&str> = printed.iter().map(|(_, p)| *p).collect();
        return texts.join(", ");
    }
    let inner = format!("\n{indent}    ");
    let lines: Vec<String> = printed
        .iter()
        .map(|(comments, item)| {
            let mut line = String::new();
            for comment in comments {
                line.push_str(&inner);
                line.push_str(comment);
            }
            line.push_str(&inner);
            line.push_str(item);
            line
        })
        .collect();
    format!("{}\n{indent}", lines.join(","))
}

/// The edits for removing the members `removed` flags from a class, an anonymous class or a trait, as php-parser
/// prints them: with a member kept, [`remove_members`]; with only the comments after the last member kept
/// (php-parser's `Nop`), the members go up to those comments; with nothing kept, the class-like is printed anew
/// (`pClassCommon`): its attributes a line each, then its modifiers, name or arguments, `extends` and `implements`
/// lists on one line, and an empty body on the next. Another class-like keeps its members.
pub(crate) fn remove_class_members(
    source: &str,
    node: Node<'_, '_>,
    removed: &[bool],
) -> Vec<Edit> {
    use crate::rules::rector_batch5::{modifiers_text, remove_members};
    use crate::rules::rector_stmts::{first_comment, line_indent};
    use mago_span::HasSpan;
    use mago_syntax::cst::{AttributeList, Extends, Implements, Sequence};
    let text = |s: mago_span::Span| &source[s.start.offset as usize..s.end.offset as usize];
    let span = node_span(&node);
    let indent = line_indent(source, span.start);
    type Parts<'n, 'a> = (
        &'n Sequence<'a, AttributeList<'a>>,
        String,
        Option<&'n Extends<'a>>,
        Option<&'n Implements<'a>>,
    );
    let (lists, head, extends, implements): Parts<'_, '_> = match node {
        Node::Class(c) => (
            &c.attribute_lists,
            format!(
                "{}class {}",
                modifiers_text(&c.modifiers),
                text(c.name.span)
            ),
            c.extends.as_ref(),
            c.implements.as_ref(),
        ),
        Node::AnonymousClass(c) => {
            // Rector's oracle prints the arguments of an anonymous class without their comments, on one line
            let args: Vec<&str> = c
                .argument_list
                .iter()
                .flat_map(|a| a.arguments.iter())
                .map(|a| text(a.span()))
                .collect();
            let args = if args.is_empty() {
                String::new()
            } else {
                format!("({})", args.join(", "))
            };
            (
                &c.attribute_lists,
                format!("{}class{args}", modifiers_text(&c.modifiers)),
                c.extends.as_ref(),
                c.implements.as_ref(),
            )
        }
        Node::Trait(t) => (
            &t.attribute_lists,
            format!("trait {}", text(t.name.span)),
            None,
            None,
        ),
        _ => return Vec::new(),
    };
    let (members, left, right) = class_body(node).expect("a class-like");
    if removed.contains(&false) {
        return remove_members(source, members, left, right, removed);
    }
    let spans: Vec<Span> = members.iter().map(node_span).collect();
    let (open, close) = (left.end.offset as usize, right.start.offset as usize);
    let first = first_comment(source, open, spans[0].start).unwrap_or(spans[0].start);
    if let Some(nop) = first_comment(source, spans[spans.len() - 1].end, close) {
        return vec![Edit::new(Span::new(first, nop), "")];
    }
    // PHPStan names an anonymous class, so `pAttrGroups` puts its attributes on lines of their own too
    let mut out = String::from(if matches!(node, Node::AnonymousClass(_)) {
        "new "
    } else {
        ""
    });
    for list in lists.iter() {
        out.push_str(text(list.span()));
        out.push('\n');
        out.push_str(indent);
    }
    out.push_str(&head);
    let names = |types: &mago_syntax::cst::TokenSeparatedSequence<'_, Identifier<'_>>| {
        types
            .iter()
            .map(|t| text(t.span()))
            .collect::<Vec<_>>()
            .join(", ")
    };
    if let Some(extends) = extends {
        out.push_str(" extends ");
        out.push_str(&names(&extends.types));
    }
    if let Some(implements) = implements {
        out.push_str(" implements ");
        out.push_str(&names(&implements.types));
    }
    out.push_str(&format!("\n{indent}{{\n{indent}}}"));
    vec![Edit::new(span, out)]
}
