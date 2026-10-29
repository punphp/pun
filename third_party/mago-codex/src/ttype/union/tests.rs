use std::hash::{DefaultHasher, Hash, Hasher};

use super::*;
use crate::ttype::{get_int, get_list, get_nullable_int, get_string};

fn assert_equal_hash(left: &TUnion, right: &TUnion) {
    assert_eq!(left, right);
    let hash = |union: &TUnion| {
        let mut hasher = DefaultHasher::new();
        union.hash(&mut hasher);
        hasher.finish()
    };
    assert_eq!(hash(left), hash(right));
}

#[test]
fn union_hash_matches_borrowed_and_reordered_nullable_types() {
    let borrowed = get_nullable_int();
    let mut reordered = borrowed.types.to_vec();
    reordered.reverse();
    let reordered = TUnion::from_vec(reordered);
    assert_ne!(borrowed.types, reordered.types);
    assert_equal_hash(&borrowed, &reordered);
}

#[test]
fn union_hash_matches_equal_duplicate_layouts() {
    let int = get_int().get_single().clone();
    let string = get_string().get_single().clone();
    let left = TUnion::new(Cow::Owned(vec![int.clone(), int.clone(), string.clone()]));
    let right = TUnion::new(Cow::Owned(vec![string.clone(), int, string]));
    assert_equal_hash(&left, &right);
}

#[test]
fn union_hash_matches_nested_equal_nullable_types() {
    let borrowed = get_nullable_int();
    let mut reordered = borrowed.types.to_vec();
    reordered.reverse();
    let reordered = TUnion::from_vec(reordered);
    assert_equal_hash(&get_list(borrowed), &get_list(reordered));
}

#[test]
fn union_hash_ignores_bookkeeping_flags_at_each_depth() {
    let original = get_nullable_int();
    let mut marked = original.clone();
    marked
        .flags
        .insert(UnionFlags::POPULATED.union(UnionFlags::NULLSAFE_NULL));
    assert_equal_hash(&original, &marked);
    assert_equal_hash(&get_list(original), &get_list(marked));
}

#[test]
fn union_construction_preserves_member_order_but_not_in_printed_ids() {
    let first = TAtomic::Object(TObject::Named(TNamedObject::new(word("ZReceiver"))));
    let second = TAtomic::Object(TObject::Named(TNamedObject::new(word("AReceiver"))));
    let forward = TUnion::from_vec(vec![first.clone(), TAtomic::Never, second.clone()]);
    let reverse = TUnion::from_vec(vec![second.clone(), first.clone()]);

    assert_eq!(forward.types.as_ref(), &[first.clone(), second.clone()]);
    assert_eq!(reverse.types.as_ref(), &[second, first]);
    assert_equal_hash(&forward, &reverse);
    assert_eq!(forward.get_id().as_str(), Some("AReceiver|ZReceiver"));
    assert_eq!(reverse.get_id().as_str(), Some("AReceiver|ZReceiver"));
}

#[test]
fn nonexistent_parent_is_a_nameless_truthy_leaf_not_a_named_parent() {
    use crate::ttype::cast::{can_atomic_be_callable, cast_atomic_to_callable};
    use crate::ttype::{get_iterable_parameters, get_named_object};

    // PHPStan 2.3.1 NonexistentParentClassType and JustNullableTypeTrait.
    let codebase = CodebaseMetadata::default();
    let parent = TAtomic::Object(TObject::NonexistentParent);
    let named = get_named_object(word("parent"), None);
    assert_ne!(parent, *named.get_single());
    assert_eq!(parent.get_id(), word("parent"));
    assert_eq!(parent.get_pretty_id_with_indent(2), word("parent"));
    assert!(parent.is_object_type());
    assert!(parent.is_truthy());
    assert!(!parent.is_falsy());
    assert!(!parent.is_literal());
    assert!(!parent.can_be_callable());
    assert!(!can_atomic_be_callable(&parent, &codebase));
    assert!(can_atomic_be_callable(named.get_single(), &codebase));
    assert!(cast_atomic_to_callable(&parent, &codebase, None).is_none());
    assert!(get_iterable_parameters(&parent, &codebase).is_none());
    assert!(!parent.is_traversable(&codebase));
    assert!(!parent.is_array_accessible_with_string_key());
    assert!(!parent.is_array_accessible_with_int_or_string_key());
    assert!(parent.get_object_or_enum_name().is_none());
    assert!(parent.get_all_object_names().is_empty());
    assert_eq!(named.get_single().get_all_object_names(), [word("parent")]);
    assert!(TObject::NonexistentParent.get_name().is_none());
    assert!(TObject::NonexistentParent.get_type_parameters().is_none());
    assert!(!parent.needs_population());
    assert!(!parent.is_expandable());
    assert!(!parent.is_complex());
    assert!(!parent.can_be_intersected());
    let mut children = Vec::new();
    parent.append_child_nodes(&mut children);
    assert!(children.is_empty());
}

#[test]
fn nonexistent_parent_lattice_retains_asymmetric_object_and_bottom_controls() {
    use crate::ttype::combiner::{CombinerOptions, combine};
    use crate::ttype::comparator::{ComparisonResult, union_comparator};
    use crate::ttype::{
        combine_union_types, get_mixed, get_named_object, get_never, get_object, intersect_union_types,
    };

    let codebase = CodebaseMetadata::default();
    let parent = TUnion::from_atomic(TAtomic::Object(TObject::NonexistentParent));
    let object = get_object();
    let named = get_named_object(word("parent"), None);
    let mixed = get_mixed();
    let never = get_never();
    // Each tuple is (other, parent <= other, other <= parent, union, intersection).
    // Expectations are from the pinned PHPStan TypeCombinator and type API.
    for (other, forward, reverse, union, intersection) in [
        (parent.clone(), true, true, parent.clone(), parent.clone()),
        (
            object.clone(),
            false,
            false,
            TUnion::from_vec(vec![parent.get_single().clone(), object.get_single().clone()]),
            never.clone(),
        ),
        (
            named.clone(),
            false,
            false,
            TUnion::from_vec(vec![parent.get_single().clone(), named.get_single().clone()]),
            never.clone(),
        ),
        (mixed.clone(), true, false, mixed, parent.clone()),
        (never.clone(), false, true, parent.clone(), never.clone()),
    ] {
        for (left, right, contained) in [(&parent, &other, forward), (&other, &parent, reverse)] {
            assert_eq!(
                union_comparator::is_contained_by(
                    &codebase,
                    left,
                    right,
                    false,
                    false,
                    false,
                    &mut ComparisonResult::new(),
                ),
                contained,
            );
            assert_eq!(combine_union_types(left, right, &codebase, Default::default()), union);
            assert_eq!(intersect_union_types(left, right, &codebase).unwrap_or_else(get_never), intersection);
            // Exercise the public n-ary combiner as well as the binary union API.
            let combined = combine(
                vec![left.get_single().clone(), right.get_single().clone(), parent.get_single().clone()],
                &codebase,
                CombinerOptions::default(),
            );
            assert_eq!(TUnion::from_vec(combined), union);
        }
    }
    for other in [object, named, crate::ttype::get_mixed_iterable(), get_never()] {
        for (left, right) in [(&parent, &other), (&other, &parent)] {
            assert!(!union_comparator::can_expression_types_be_identical(&codebase, left, right, false, false));
        }
    }
    assert!(union_comparator::can_expression_types_be_identical(&codebase, &parent, &parent, false, false));
    let shape = TUnion::from_atomic(TAtomic::Object(TObject::new_with_properties(false, Default::default())));
    assert!(!union_comparator::is_contained_by(
        &codebase,
        &parent,
        &shape,
        false,
        false,
        false,
        &mut ComparisonResult::new(),
    ));
}

#[test]
fn unnamed_literal_class_string_has_provenance_but_no_value_or_bound() {
    use crate::ttype::atomic::derived::new::TNew;
    use crate::ttype::atomic::scalar::class_like_string::{TClassLikeString, TClassLikeStringKind};
    use crate::ttype::cast::{can_atomic_be_callable, cast_atomic_to_callable};

    let codebase = CodebaseMetadata::default();
    let string = TClassLikeString::LiteralAny;
    assert!(!TScalar::ClassLikeString(string.clone()).is_boring());
    let atomic = TAtomic::Scalar(TScalar::ClassLikeString(string.clone()));
    assert_eq!(string.kind(), Some(TClassLikeStringKind::Class));
    assert!(!string.has_constraint());
    assert!(!string.is_generic());
    assert!(!string.is_literal(), "this query requires a known name, not just literal provenance");
    assert!(string.literal_value().is_none());
    assert!(string.constraint().is_none());
    assert_eq!(string.get_object_type(&codebase), TAtomic::Object(TObject::Any));
    assert_eq!(atomic.get_id(), word("class-string&literal-string"));
    assert!(atomic.is_string_of_literal_origin());
    assert!(!atomic.is_literal());
    assert!(atomic.get_literal_string_value().is_none());
    assert!(atomic.get_class_string_value().is_none());
    assert!(atomic.to_array_key().is_none());
    assert!(!atomic.is_object_type());
    assert!(atomic.get_all_object_names().is_empty());
    assert!(!atomic.needs_population());
    assert!(!atomic.is_expandable());
    assert!(can_atomic_be_callable(&atomic, &codebase));
    assert!(atomic.can_be_callable());
    assert!(cast_atomic_to_callable(&atomic, &codebase, None).is_none());
    assert_eq!(TNew::get_new_targets(&[atomic.clone()], &codebase), Some(crate::ttype::get_object()));
    let mut children = Vec::new();
    atomic.append_child_nodes(&mut children);
    assert!(children.is_empty());
    let ordinary = TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::class_string()));
    let literal = TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::literal(word("stdClass"))));
    assert_ne!(atomic, ordinary);
    assert_ne!(atomic, literal);
    assert!(!ordinary.is_string_of_literal_origin());
    assert!(literal.is_string_of_literal_origin());
    assert_eq!(literal.get_class_string_value(), Some(word("stdClass")));
}

#[test]
fn nameless_leaves_survive_forced_population_without_becoming_symbols() {
    use crate::ttype::atomic::scalar::class_like_string::TClassLikeString;

    static LEAVES: &[TAtomic] = &[
        TAtomic::Object(TObject::NonexistentParent),
        TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::LiteralAny)),
    ];
    let mut union = TUnion::new(Cow::Borrowed(LEAVES));
    let mut references = SymbolReferences::default();
    populate_union_type(&mut union, &Symbols::default(), None, &mut references, true);
    assert!(matches!(union.types, Cow::Borrowed(_)), "nameless leaves need no symbol population or allocation");
    assert_eq!(union.types.as_ref(), LEAVES);
    assert!(union.flags.contains(UnionFlags::POPULATED));
}

#[test]
fn unnamed_literal_class_string_lattice_does_not_become_a_literal_parent_name() {
    use crate::ttype::atomic::scalar::class_like_string::TClassLikeString;
    use crate::ttype::atomic::scalar::string::TString;
    use crate::ttype::combiner::combine;
    use crate::ttype::comparator::{ComparisonResult, union_comparator};
    use crate::ttype::{
        combine_union_types, get_class_string, get_mixed, get_never, get_object, intersect_union_types,
    };

    let codebase = CodebaseMetadata::default();
    let unnamed = TUnion::from_atomic(TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::LiteralAny)));
    let literal =
        TUnion::from_atomic(TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::literal(word("stdClass")))));
    let literal_string = TUnion::from_atomic(TAtomic::Scalar(TScalar::String(TString::unspecified_literal(false))));
    for (other, forward, reverse, union, intersection) in [
        (get_class_string(), true, false, get_class_string(), unnamed.clone()),
        (literal_string.clone(), true, false, literal_string.clone(), unnamed.clone()),
        (literal.clone(), false, true, unnamed.clone(), literal),
        (get_string(), true, false, get_string(), unnamed.clone()),
        (
            get_object(),
            false,
            false,
            TUnion::from_vec(vec![unnamed.get_single().clone(), get_object().get_single().clone()]),
            get_never(),
        ),
        (get_mixed(), true, false, get_mixed(), unnamed.clone()),
        (get_never(), false, true, unnamed.clone(), get_never()),
    ] {
        for (left, right, contained) in [(&unnamed, &other, forward), (&other, &unnamed, reverse)] {
            assert_eq!(
                union_comparator::is_contained_by(
                    &codebase,
                    left,
                    right,
                    false,
                    false,
                    false,
                    &mut ComparisonResult::new(),
                ),
                contained,
            );
            assert_eq!(combine_union_types(left, right, &codebase, Default::default()), union);
            assert_eq!(intersect_union_types(left, right, &codebase).unwrap_or_else(get_never), intersection);
            assert_eq!(
                TUnion::from_vec(combine(
                    vec![left.get_single().clone(), right.get_single().clone()],
                    &codebase,
                    Default::default()
                )),
                union
            );
        }
    }
    // This actual intersection creates the accessory identity; no constant or bound is invented.
    for (left, right) in [(&get_class_string(), &literal_string), (&literal_string, &get_class_string())] {
        assert_eq!(intersect_union_types(left, right, &codebase), Some(unnamed.clone()));
    }
}
