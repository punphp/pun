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
