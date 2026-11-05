//! PHPStan 2.3.1's regex array shapes: `$matches` of `preg_match()` and `preg_match_all()` typed from a constant
//! pattern (`PregMatchParameterOutTypeExtension`, `PregMatchTypeSpecifyingExtension`, `RegexArrayShapeMatcher`).

mod groups;
mod hoa;
mod matcher;
mod types;

use mago_analyzer::plugin::PluginRegistry;
use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::assertion::{FunctionAssertionProvider, InvocationAssertions};
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::assertion::Assertion;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{ArrayAccess, BinaryOperator, Call, Expression, Literal, Variable};
use pun_phpstan::types::Tri;

use matcher::{FlagsArg, Php};

/// Registers the extensions for the analysed PHP version (`PHP_VERSION_ID`).
pub(super) fn register(registry: &mut PluginRegistry, php_version: u32) {
    let php = Php {
        unmatched_as_null: php_version >= 70400,
        capture_only_named: php_version >= 80200,
    };
    registry.register_function_provider(PregMatch(php));
    registry.register_function_assertion_provider(PregMatch(php));
    registry.register_function_provider(PregReplaceCallback(php));
}

static META: ProviderMeta =
    ProviderMeta::new("pun::pcre", "PCRE shapes", "PHPStan's regex array shapes");

/// `PregReplaceCallbackClosureTypeExtension`.
struct PregReplaceCallback(Php);

impl Provider for PregReplaceCallback {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for PregReplaceCallback {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"preg_replace_callback")
    }

    fn get_return_type(
        &self,
        _: &ProviderContext<'_, '_, '_>,
        _: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        None
    }

    fn get_parameter_closure_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
        parameter: &[u8],
    ) -> Option<TUnion> {
        use std::sync::Arc;

        use mago_codex::ttype::atomic::TAtomic;
        use mago_codex::ttype::atomic::callable::{
            TCallable, TCallableSignature, parameter::TCallableParameter,
        };

        let metadata = invocation.inner().target.get_function_like_metadata()?;
        (parameter == b"$callback" && metadata.flags.is_built_in()).then_some(())?;
        let patterns = pattern_strings(context, invocation.get_argument(0, &[b"pattern"])?)?;
        let flags = flags_argument(context, invocation.get_argument(5, &[b"flags"]));
        let matches = matcher::match_pattern(self.0, &patterns, flags, Tri::Yes, false)?;
        let signature = TCallableSignature::new(false, true)
            .with_parameters(vec![TCallableParameter::new(
                Some(Arc::new(matches)),
                false,
                false,
                false,
            )])
            .with_return_type(Some(Arc::new(mago_codex::ttype::get_string())));
        Some(TUnion::from_atomic(TAtomic::Callable(
            TCallable::Signature(signature),
        )))
    }
}

/// The flags argument's constant integer, if any.
fn flags_argument(
    context: &ProviderContext<'_, '_, '_>,
    flags: Option<&Expression<'_>>,
) -> FlagsArg {
    match flags {
        None => FlagsArg::Absent,
        Some(flags) => context
            .get_expression_type(flags)
            .and_then(TUnion::get_single_literal_int_value)
            .map_or(FlagsArg::Unknown, FlagsArg::Constant),
    }
}

struct PregMatch(Php);

impl Provider for PregMatch {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

const TARGETS: FunctionTarget = FunctionTarget::ExactMultiple(&[b"preg_match", b"preg_match_all"]);

impl PregMatch {
    /// The call's builtin function, as `preg_match_all` or not.
    fn matches_all(invocation: &InvocationInfo<'_, '_, '_>) -> Option<bool> {
        let metadata = invocation.inner().target.get_function_like_metadata()?;
        metadata.flags.is_built_in().then(|| {
            invocation
                .function_name()
                .eq_ignore_ascii_case("preg_match_all")
        })
    }

    /// `RegexArrayShapeMatcher::match(All)Expr()`.
    fn matched(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
        was_matched: Tri,
    ) -> Option<TUnion> {
        let matches_all = Self::matches_all(invocation)?;
        let patterns = pattern_strings(context, invocation.get_argument(0, &[b"pattern"])?);
        let flags = flags_argument(context, invocation.get_argument(3, &[b"flags"]));
        matcher::match_pattern(self.0, &patterns?, flags, was_matched, matches_all)
    }
}

impl FunctionReturnTypeProvider for PregMatch {
    fn targets() -> FunctionTarget {
        TARGETS
    }

    fn get_return_type(
        &self,
        _: &ProviderContext<'_, '_, '_>,
        _: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        None
    }

    /// `PregMatchParameterOutTypeExtension`.
    fn get_parameter_out_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
        parameter: &[u8],
    ) -> Option<TUnion> {
        // the parameter is named different, depending on PHP version.
        matches!(parameter, b"$matches" | b"$subpatterns").then_some(())?;
        invocation.get_argument(2, &[b"matches"])?;
        if Self::matches_all(invocation)? {
            return self.matched(context, invocation, Tri::Maybe);
        }
        let matched = self.matched(context, invocation, Tri::Yes)?;
        Some(matcher::with_empty_array(matched))
    }
}

impl FunctionAssertionProvider for PregMatch {
    fn targets() -> FunctionTarget {
        TARGETS
    }

    /// `PregMatchTypeSpecifyingExtension` in the truthy and falsey contexts.
    fn get_assertions(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<InvocationAssertions> {
        let pattern = invocation.get_argument(0, &[b"pattern"])?;
        let mut assertions = InvocationAssertions::new();
        let matches = invocation.get_argument(2, &[b"matches"]);
        if let Some(subject) = invocation.get_argument(1, &[b"subject"])
            && context
                .get_expression_type(subject)
                .is_some_and(TUnion::is_string)
            && !is_sub_expression_of(subject, matches)
            && let Some(subject_type) =
                pattern_strings(context, pattern).and_then(|p| matcher::match_subject(self.0, &p))
        {
            assertions.add_if_true(mago_word::word("$subject"), is_type(subject_type));
        }
        if matches.is_some() {
            if let Some(matched) = self.matched(context, invocation, Tri::Yes) {
                assertions.add_if_true(mago_word::word("$matches"), is_type(matched));
            }
            // PHPStan overwrites $matches in the falsey context (`setAlwaysOverwriteTypes()`); Mago's assertions
            // intersect, and preg_match_all()'s out type has no `array{}` to narrow to, so its falsey branch keeps it
            if !Self::matches_all(invocation)?
                && let Some(unmatched) = self.matched(context, invocation, Tri::No)
            {
                assertions.add_if_false(mago_word::word("$matches"), is_type(unmatched));
            }
        }
        (!assertions.is_empty()).then_some(assertions)
    }
}

fn is_type(t: TUnion) -> Vec<Assertion> {
    t.types.iter().cloned().map(Assertion::IsType).collect()
}

/// `isSubExprOfMatchesArg()`.
fn is_sub_expression_of(subject: &Expression<'_>, matches: Option<&Expression<'_>>) -> bool {
    let mut root = subject;
    while let Expression::ArrayAccess(ArrayAccess { array, .. }) = root {
        root = array;
    }
    match (root, matches) {
        (
            Expression::Variable(Variable::Direct(root)),
            Some(Expression::Variable(Variable::Direct(matches))),
        ) => root.name == matches.name,
        _ => false,
    }
}

/// The pattern argument's constant strings (`getPatternType()`): a concatenation through
/// `RegexExpressionHelper::resolvePatternConcat()`, which reads a `preg_quote()` call as `(?:.*)`.
fn pattern_strings(
    context: &ProviderContext<'_, '_, '_>,
    pattern: &Expression<'_>,
) -> Option<Vec<String>> {
    let strings = match pattern {
        Expression::Binary(binary)
            if matches!(binary.operator, BinaryOperator::StringConcat(_)) =>
        {
            concat(context, pattern)
        }
        pattern => constant_strings(context, pattern),
    };
    strings
        .into_iter()
        .map(|s| String::from_utf8(s).ok())
        .collect()
}

fn concat(context: &ProviderContext<'_, '_, '_>, e: &Expression<'_>) -> Vec<Vec<u8>> {
    match e {
        Expression::Parenthesized(p) => concat(context, p.expression),
        Expression::Call(Call::Function(f)) if matches!(f.function, Expression::Identifier(id) if id.value().rsplit(|b| *b == b'\\').next().is_some_and(|n| n.eq_ignore_ascii_case(b"preg_quote"))) =>
        {
            // assume preg_quote() cannot create capturing groups or contain meta characters
            vec![b"(?:.*)".to_vec()]
        }
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)) => {
            let (left, right) = (concat(context, b.lhs), concat(context, b.rhs));
            left.iter()
                .flat_map(|l| {
                    right
                        .iter()
                        .map(move |r| [l.as_slice(), r.as_slice()].concat())
                })
                .collect()
        }
        e => constant_strings(context, e),
    }
}

/// `getConstantStrings()` of an expression's type (`toString()` of a constant integer too, inside a concatenation).
fn constant_strings(context: &ProviderContext<'_, '_, '_>, e: &Expression<'_>) -> Vec<Vec<u8>> {
    if let Expression::Literal(Literal::String(s)) = e {
        return s.value.map(|v| vec![v.to_vec()]).unwrap_or_default();
    }
    // the analysed argument always has a type
    let types = context.get_expression_type(e).map_or(&[][..], |t| &t.types);
    let strings: Vec<Vec<u8>> = types
        .iter()
        .filter_map(|a| a.get_literal_string_value().map(<[u8]>::to_vec))
        .collect();
    if strings.len() == types.len() {
        strings
    } else {
        Vec::new()
    }
}
