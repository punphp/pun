//! Object projections from PHPStan's GetClassDynamicReturnTypeExtension.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeStringKind;
use mago_codex::ttype::combiner::combine;
use mago_codex::ttype::union::TUnion;

use crate::plugin::context::InvocationInfo;
use crate::plugin::context::ProviderContext;
use crate::plugin::provider::Provider;
use crate::plugin::provider::ProviderMeta;
use crate::plugin::provider::function::FunctionReturnTypeProvider;
use crate::plugin::provider::function::FunctionTarget;

#[derive(Default)]
pub struct GetClassProvider;

impl Provider for GetClassProvider {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "php::object::get_class",
            "get_class",
            "Preserves the class-string projection of object arguments",
        );
        &META
    }
}

impl FunctionReturnTypeProvider for GetClassProvider {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"get_class")
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let argument = invocation.get_argument(0, &[b"object"])?;
        let ty = context.get_expression_type(argument)?;
        let mut projections = Vec::with_capacity(ty.types.len());
        for atomic in ty.types.iter() {
            let projection = match atomic {
                TAtomic::Object(TObject::Named(_) | TObject::Enum(_)) => {
                    TClassLikeString::of_type(TClassLikeStringKind::Class, atomic.clone())
                }
                TAtomic::Object(_) => TClassLikeString::any(TClassLikeStringKind::Class),
                _ => return None,
            };
            projections.push(TAtomic::Scalar(TScalar::ClassLikeString(projection)));
        }
        Some(TUnion::from_vec(combine(
            projections,
            context.codebase(),
            Default::default(),
        )))
    }
}
