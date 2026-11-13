//! PHPStan 2.3.5's RandomizerMethodReturnTypeExtension for `Random\Randomizer::shuffleArray()`: the argument's
//! `Type::shuffleArray()`, a list of its values that keeps non-emptiness.

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::method::{MethodReturnTypeProvider, MethodTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    combine_union_types, get_array_parameters, get_list, get_mixed, get_non_empty_list,
};

static META: ProviderMeta = ProviderMeta::new(
    "pun-randomizer",
    "pun",
    "PHPStan's Randomizer::shuffleArray() return type",
);

pub(super) struct Randomizer;

impl Provider for Randomizer {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl MethodReturnTypeProvider for Randomizer {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: [MethodTarget; 1] =
            [MethodTarget::exact(b"Random\\Randomizer", b"shuffleArray")];
        &TARGETS
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        _: &[u8],
        _: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let array = context.get_expression_type(invocation.get_argument(0, &[b"array"])?)?;
        let codebase = context.codebase();
        let mut result: Option<TUnion> = None;
        for atomic in array.types.iter() {
            let shuffled = match atomic {
                // ArrayType: list<value>; NonEmptyArrayType keeps itself; a constant array degrades to its values
                TAtomic::Array(array) => {
                    let (_, value) = get_array_parameters(array, codebase);
                    if array.is_non_empty() {
                        get_non_empty_list(value)
                    } else {
                        get_list(value)
                    }
                }
                // MixedType: list<mixed>
                TAtomic::Mixed(_) => get_list(get_mixed()),
                // ErrorType for a member that is no array: leave the declared return type
                _ => return None,
            };
            result = Some(match result {
                Some(previous) => {
                    combine_union_types(&previous, &shuffled, codebase, CombinerOptions::default())
                }
                None => shuffled,
            });
        }
        result
    }
}
