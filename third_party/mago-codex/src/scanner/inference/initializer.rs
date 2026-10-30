//! PHPStan InitializerExprTypeResolver's finite named-selector reflection query.

use mago_word::{Word, word};

use crate::metadata::CodebaseMetadata;
use crate::ttype::atomic::TAtomic;
use crate::ttype::atomic::object::TObject;
use crate::ttype::atomic::object::r#enum::TEnum;
use crate::ttype::atomic::scalar::TScalar;
use crate::ttype::atomic::scalar::class_like_string::TClassLikeString;
use crate::ttype::expander::{TypeExpansionOptions, expand_class_constant_value};
use crate::ttype::initializer::{expanded, union};
use crate::ttype::union::TUnion;
use crate::ttype::{get_error, get_mixed, get_named_object};

fn class_string_object(codebase: &CodebaseMetadata, atomic: &TAtomic) -> Option<TAtomic> {
    match atomic {
        TAtomic::Scalar(TScalar::ClassLikeString(string)) => Some(string.get_object_type(codebase)),
        TAtomic::Scalar(TScalar::String(string)) => {
            let name = string.get_known_literal_value()?;
            let class = codebase.get_class_like(name.strip_prefix(b"\\").unwrap_or(name))?;
            Some(get_named_object(class.name, None).get_single_owned())
        }
        _ => None,
    }
}

/// Dependency discovery uses the same class-string classification as the value query, before reads
/// or falsey/null filtering. Only the existing service may load/evaluate a selected declaration.
pub fn class_names(codebase: &CodebaseMetadata, holder: &TUnion) -> Vec<Word> {
    holder
        .types
        .iter()
        .flat_map(|atomic| {
            class_string_object(codebase, atomic).unwrap_or_else(|| atomic.clone()).get_all_object_names()
        })
        .collect()
}

pub(super) fn class_constant(codebase: &CodebaseMetadata, holder: &TUnion, name: &[u8], mut is_object: bool) -> TUnion {
    let class_strings: Option<Vec<_>> =
        holder.types.iter().map(|atomic| class_string_object(codebase, atomic)).collect();
    let object_type = if let Some(objects) = class_strings {
        if holder.types.iter().all(|atomic| {
            matches!(
                atomic,
                TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { .. }) | TScalar::String(_))
            )
        }) {
            is_object = false;
        }
        expanded(objects, codebase)
    } else {
        expanded(holder.types.to_vec(), codebase)
    };
    // UnionType::getObjectClassNames requires every member to contribute object names. It does not
    // silently discard strings/scalars to invent a surviving object alternative.
    let names: Option<Vec<_>> = object_type
        .types
        .iter()
        .map(|atomic| {
            let names = atomic.get_all_object_names();
            (!names.is_empty()).then_some(names)
        })
        .collect();
    let mut value = None;
    for class in names.into_iter().flatten().flatten() {
        let Some(metadata) = codebase.get_class_like_by_name(class) else {
            continue;
        };
        if let Some(case) = metadata.enum_cases.get(&word(name)) {
            let ty =
                TUnion::from_atomic(TAtomic::Object(TObject::Enum(TEnum::new_case(metadata.original_name, case.name))));
            value = Some(value.map_or_else(|| ty.clone(), |previous| union(&previous, &ty, codebase)));
            continue;
        }
        let Some(constant) = metadata.constants.get(&word(name)) else {
            continue;
        };
        let late = is_object && !metadata.flags.is_final() && !metadata.kind.is_enum();
        if late && !constant.flags.is_final() && constant.type_metadata.is_none() {
            return get_mixed();
        }
        let ty = expand_class_constant_value(
            codebase,
            constant,
            late,
            &TypeExpansionOptions { preserve_initializer_union_order: true, ..Default::default() },
        );
        value = Some(value.map_or_else(|| ty.clone(), |previous| union(&previous, &ty, codebase)));
    }
    if let Some(value) = value {
        return value;
    }
    // MixedType has a dummy constant of mixed. Heterogeneous UnionType falls back to the first
    // definite constant reflection only when every alternative definitely has that constant.
    let mut first = None;
    for atomic in object_type.types.iter() {
        let ty = if matches!(atomic, TAtomic::Mixed(_)) {
            get_mixed()
        } else {
            let object = class_string_object(codebase, atomic).unwrap_or_else(|| atomic.clone());
            let Some(class) = object.get_object_or_enum_name().and_then(|name| codebase.get_class_like_by_name(name))
            else {
                return get_error();
            };
            let Some(constant) = class.constants.get(&word(name)) else {
                return get_error();
            };
            constant
                .type_metadata
                .as_ref()
                .map(|ty| ty.type_union.clone())
                .or_else(|| constant.inferred_type.clone())
                .unwrap_or_else(get_mixed)
        };
        first.get_or_insert(ty);
    }
    first.expect("a holder union is nonempty")
}
