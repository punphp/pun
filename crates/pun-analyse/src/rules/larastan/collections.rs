//! NoUnnecessaryEnumerableToArrayCallsRule: `->toArray()` on an Enumerable whose values are never Arrayable.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::get_specialized_template_type;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::*;
use mago_word::word;

use super::super::Checker;
use super::{is_a, is_object_of};

const ENUMERABLE: &[u8] = b"illuminate\\support\\enumerable";
const ARRAYABLE: &[u8] = b"illuminate\\contracts\\support\\arrayable";

/// The `->toArray()` calls whose object the rule reads.
pub(super) fn to_array<'a, 'arena>(
    call: &'a MethodCall<'arena>,
) -> Option<&'a ClassLikeMemberSelector<'arena>> {
    matches!(&call.method, ClassLikeMemberSelector::Identifier(m) if m.value == b"toArray")
        .then_some(&call.method)
}

pub(super) fn check<'arena>(c: &mut Checker<'_, 'arena>, call: &MethodCall<'arena>) {
    let Some(name) = to_array(call) else {
        return;
    };
    let Some(ty) = c.type_of(call.object) else {
        return;
    };
    if !is_object_of(c, ty, ENUMERABLE) || !value_type(c, ty).is_some_and(|v| not_arrayable(c, &v))
    {
        return;
    }
    c.report(
        name,
        "larastan.unnecessaryEnumerableToArrayCall",
        "Called [toArray()] on an Enumerable which does not contain any Arrayables.".to_string(),
        Some("Use [all()] to get the items as an array."),
    );
}

/// `getTemplateType(Enumerable::class, 'TValue')`: each part's TValue.
fn value_type(c: &Checker, ty: &TUnion) -> Option<TUnion> {
    let mut types = Vec::new();
    for t in ty.types.iter() {
        let TAtomic::Object(TObject::Named(n)) = t else {
            return None;
        };
        let k = c.codebase.get_class_like(n.name.as_bytes())?;
        let value = get_specialized_template_type(
            c.codebase,
            word("TValue"),
            word(ENUMERABLE),
            k,
            n.type_parameters.as_deref(),
        )?;
        types.extend(value.types.iter().cloned());
    }
    Some(TUnion::from_vec(types))
}

/// `(new ObjectType(Arrayable::class))->isSuperTypeOf($type)->no()`.
fn not_arrayable(c: &Checker, ty: &TUnion) -> bool {
    !ty.types.is_empty()
        && ty.types.iter().all(|t| match t {
            TAtomic::Scalar(_)
            | TAtomic::Null
            | TAtomic::Array(_)
            | TAtomic::Resource(_)
            | TAtomic::Never => true,
            TAtomic::Object(TObject::Named(n)) => {
                !is_a(c, n.name.as_bytes(), ARRAYABLE)
                    && c.codebase
                        .get_class_like(n.name.as_bytes())
                        .is_some_and(|k| k.flags.is_final())
            }
            TAtomic::Object(TObject::Enum(_)) => true,
            _ => false,
        })
}
