use std::collections::BTreeMap;
use std::sync::Arc;

use mago_word::{Word, concat_word};

use crate::metadata::CodebaseMetadata;
use crate::ttype::TType;
use crate::ttype::TypeRef;
use crate::ttype::atomic::TAtomic;
use crate::ttype::atomic::array::key::ArrayKey;
use crate::ttype::atomic::array::keyed::TKeyedArray;
use crate::ttype::atomic::array::list::TList;
use crate::ttype::get_arraykey;
use crate::ttype::get_int;
use crate::ttype::get_mixed;
use crate::ttype::union::TUnion;
use crate::ttype::{get_class_string, get_never, get_object, get_truthy_string, intersect_union_types};

pub mod key;
pub mod keyed;
pub mod list;

/// Represents the type of a PHP array, distinguishing between list-like and keyed/associative usage.
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Clone, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TArray {
    /// Represents an array used as a list (sequential, zero-based integer keys). `list<T>`.
    List(TList),
    /// Represents an array used as a map (string keys or non-standard integer keys). `array<Tk, Tv>`.
    Keyed(TKeyedArray),
}

impl PartialEq for TArray {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if std::ptr::eq(self, other) {
            return true;
        }

        match (self, other) {
            (TArray::List(a), TArray::List(b)) => a == b,
            (TArray::Keyed(a), TArray::Keyed(b)) => a == b,
            _ => false,
        }
    }
}

impl TArray {
    /// The callable conjunct belongs to this value, not to its declaration or its tuple view.
    #[must_use]
    pub fn callable(&self) -> Option<&TAtomic> {
        match self {
            Self::Keyed(array) => array.callable.as_deref(),
            Self::List(array) => array.callable.as_deref(),
        }
    }

    pub fn callable_mut(&mut self) -> Option<&mut TAtomic> {
        match self {
            Self::Keyed(array) => array.callable.as_mut().map(Arc::make_mut),
            Self::List(array) => array.callable.as_mut().map(Arc::make_mut),
        }
    }

    /// PHPStan IntersectionType::getConstantArrays(): project the two offsets without
    /// replacing the generic array or confusing an ordinary tuple with a callable.
    pub fn callable_array_view(&self, codebase: &CodebaseMetadata) -> Option<TKeyedArray> {
        self.callable()?;
        let items = [
            (0, crate::ttype::combine_union_types(&get_class_string(), &get_object(), codebase, Default::default())),
            (1, get_truthy_string()),
        ]
        .into_iter()
        .map(|(index, constraint)| {
            let value = match self {
                Self::Keyed(array) => array
                    .known_items
                    .as_ref()
                    .and_then(|items| items.get(&ArrayKey::Integer(index)))
                    .map(|(_, value)| value.clone())
                    .unwrap_or_else(|| {
                        array.parameters.as_ref().map_or_else(get_never, |(_, value)| (**value).clone())
                    }),
                Self::List(array) => array
                    .known_elements
                    .as_ref()
                    .and_then(|items| items.get(&(index as usize)))
                    .map(|(_, value)| value.clone())
                    .unwrap_or_else(|| (*array.element_type).clone()),
            };
            let value = intersect_union_types(&value, &constraint, codebase).unwrap_or_else(get_never);
            (ArrayKey::Integer(index), (false, value))
        })
        .collect::<BTreeMap<_, _>>();
        Some(TKeyedArray::new().with_known_items(items).with_non_empty(true))
    }

    /// IntersectionType::getOffsetValueType also refines unknown and out-of-range offsets;
    /// getConstantArrays() alone cannot answer those queries.
    pub fn refine_callable_offset(&self, index: &TUnion, value: TUnion, codebase: &CodebaseMetadata) -> TUnion {
        if self.callable().is_none() {
            return value;
        }
        let index = index.get_single_literal_int_value().or_else(|| {
            index
                .get_single_literal_string_value()
                .and_then(|value| ArrayKey::from_string(mago_word::word(value)).get_integer())
        });
        let constraint = match index {
            Some(0) => {
                crate::ttype::combine_union_types(&get_class_string(), &get_object(), codebase, Default::default())
            }
            Some(1) => get_truthy_string(),
            _ => crate::ttype::combine_union_types(&get_object(), &get_truthy_string(), codebase, Default::default()),
        };
        intersect_union_types(&value, &constraint, codebase).unwrap_or_else(get_never)
    }

    /// Checks if this represents a list (`list<T>`).
    #[inline]
    #[must_use]
    pub const fn is_list(&self) -> bool {
        match self {
            Self::List(_) => true,
            Self::Keyed(array) => array.callable.is_some(),
        }
    }

    /// Checks if this represents a keyed array (`array<Tk, Tv>`).
    #[inline]
    #[must_use]
    pub const fn is_keyed(&self) -> bool {
        matches!(self, TArray::Keyed(_))
    }

    /// Returns a reference to the `ListArrayType` data if this is a `List` variant.
    #[inline]
    #[must_use]
    pub const fn get_list(&self) -> Option<&TList> {
        if let TArray::List(data) = self { Some(data) } else { None }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        if self.is_non_empty() {
            return false;
        }

        match &self {
            Self::Keyed(keyed_array) => {
                keyed_array.parameters.is_none()
                    && keyed_array.known_items.as_deref().is_none_or(std::collections::BTreeMap::is_empty)
            }
            Self::List(list) => {
                list.element_type.is_never()
                    && list.known_elements.as_deref().is_none_or(std::collections::BTreeMap::is_empty)
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn has_known_items(&self) -> bool {
        match &self {
            Self::Keyed(keyed_array) => keyed_array.known_items.as_deref().is_some_and(|items| !items.is_empty()),
            Self::List(list) => list.known_elements.as_deref().is_some_and(|items| !items.is_empty()),
        }
    }

    #[must_use]
    pub fn is_sealed(&self) -> bool {
        match &self {
            Self::Keyed(keyed_array) => keyed_array.parameters.is_none(),
            Self::List(list) => list.element_type.is_never(),
        }
    }

    /// Checks if the array is known to be non-empty.
    #[inline]
    #[must_use]
    pub const fn is_non_empty(&self) -> bool {
        match &self {
            Self::Keyed(keyed_array) => keyed_array.non_empty || keyed_array.callable.is_some(),
            Self::List(list) => list.non_empty || list.callable.is_some(),
        }
    }

    /// Checks if the array is a native PHP array (no known items and standard key/value types).
    #[inline]
    #[must_use]
    pub fn is_vanilla(&self) -> bool {
        match &self {
            Self::Keyed(keyed_array) => {
                if keyed_array.non_empty || keyed_array.callable.is_some() {
                    return false;
                }

                if keyed_array.known_items.is_some() {
                    return false;
                }

                let Some((key_parameter, value_parameter)) = keyed_array.parameters.as_ref() else {
                    return false;
                };

                key_parameter.is_array_key() && value_parameter.is_vanilla_mixed()
            }
            Self::List(_) => false,
        }
    }

    /// Returns the minimum size of the array based on known items or elements.
    #[must_use]
    pub fn get_minimum_size(&self) -> usize {
        if self.callable().is_some() {
            return 2;
        }
        let mut size = 0;

        match &self {
            Self::Keyed(keyed_array) => {
                if let Some(known_items) = keyed_array.known_items.as_deref() {
                    for (optional, _) in known_items.values() {
                        if !optional {
                            size += 1;
                        }
                    }
                } else if keyed_array.non_empty {
                    size = 1;
                }
            }
            Self::List(list) => {
                if let Some(count) = list.known_count {
                    size = count;
                } else if let Some(known_elements) = list.known_elements.as_deref() {
                    for (optional, _) in known_elements.values() {
                        if !optional {
                            size += 1;
                        }
                    }
                } else if list.non_empty {
                    size = 1;
                }
            }
        }

        size.max(usize::from(self.is_non_empty()))
    }

    /// ArrayType/ConstantArrayType/IntersectionType::getArraySize().
    pub fn get_size(&self) -> TUnion {
        if self.callable().is_some() {
            return crate::ttype::get_literal_int(2);
        }
        let max = match self {
            Self::List(array) => array.known_count.or_else(|| {
                array.element_type.is_never().then(|| array.known_elements.as_ref().map_or(0, |items| items.len()))
            }),
            Self::Keyed(array) => {
                array.parameters.is_none().then(|| array.known_items.as_ref().map_or(0, |items| items.len()))
            }
        };
        let min = self.get_minimum_size();
        if max == Some(min) {
            crate::ttype::get_literal_int(min as i64)
        } else {
            crate::ttype::get_int_range(Some(min as i64), max.map(|size| size as i64))
        }
    }

    /// TypeCombinator::reduceArrays() before UnionType::getArraySize(). Keep this
    /// query's optional-key projection separate from the real incoming array values.
    pub fn get_union_size(arrays: &[&Self], codebase: &CodebaseMetadata) -> TUnion {
        let mut sizes = Vec::new();
        let mut shapes = Vec::new();
        for array in arrays {
            let items = match array {
                Self::Keyed(array) if array.callable.is_none() && array.parameters.is_none() => {
                    array.known_items.as_deref().cloned().unwrap_or_default()
                }
                Self::List(array)
                    if array.callable.is_none()
                        && array.element_type.is_never()
                        && array.known_count.is_none_or(|count| {
                            array.known_elements.as_ref().is_some_and(|items| {
                                items.len() == count && items.values().all(|(optional, _)| !optional)
                            })
                        }) =>
                {
                    array
                        .known_elements
                        .iter()
                        .flat_map(|items| items.iter())
                        .map(|(key, value)| (ArrayKey::Integer(*key as i64), value.clone()))
                        .collect()
                }
                _ => {
                    sizes.extend(array.get_size().types.into_owned());
                    continue;
                }
            };
            shapes.push(Some((items, array.is_non_empty())));
        }
        // legacyIsKeysSupersetOf(): an additional required key is optional after
        // merging. Two additional required keys must not turn 1|3 into int<1, 3>.
        let is_superset = |larger: &BTreeMap<ArrayKey, (bool, TUnion)>, smaller: &BTreeMap<ArrayKey, (bool, TUnion)>| {
            let mut fail_on_different = larger.len() != smaller.len() || larger.len() < 2;
            for (key, (_, value)) in smaller {
                let Some((_, other)) = larger.get(key) else { return false };
                if crate::ttype::comparator::union_comparator::can_expression_types_be_identical(
                    codebase, value, other, false, false,
                ) {
                    continue;
                }
                if fail_on_different {
                    return false;
                }
                fail_on_different = true;
            }
            larger.iter().filter(|(key, (optional, _))| !optional && !smaller.contains_key(key)).count() <= 1
        };
        for i in 0..shapes.len() {
            for j in i + 1..shapes.len() {
                let (Some((left, _)), Some((right, _))) = (&shapes[i], &shapes[j]) else { continue };
                // reduceArrays considers pairs sharing a key, not disjoint shapes.
                if !left.keys().any(|key| right.contains_key(key)) {
                    continue;
                }
                let (keep, remove) = if is_superset(right, left) {
                    (j, i)
                } else if is_superset(left, right) {
                    (i, j)
                } else {
                    continue;
                };
                let (smaller, smaller_non_empty) = shapes[remove].take().unwrap();
                let (larger, non_empty) = shapes[keep].as_mut().unwrap();
                *non_empty &= smaller_non_empty;
                // ConstantArrayType::legacyMergeWith().
                for (key, (optional, value)) in larger {
                    if let Some((other_optional, other_value)) = smaller.get(key) {
                        *optional |= other_optional;
                        *value = crate::ttype::combine_union_types(value, other_value, codebase, Default::default());
                    } else {
                        *optional = true;
                    }
                }
                if remove == i {
                    break;
                }
            }
        }
        for (shape, non_empty) in shapes.into_iter().flatten() {
            let array = Self::Keyed(TKeyedArray::new().with_known_items(shape).with_non_empty(non_empty));
            sizes.extend(array.get_size().types.into_owned());
        }
        TUnion::from_vec(crate::ttype::combiner::combine(sizes, codebase, Default::default()))
    }

    /// Returns the key type of the array, if applicable.
    #[must_use]
    pub fn get_key_type(&self) -> Option<TUnion> {
        match self {
            Self::Keyed(keyed_array) => {
                if let Some(parameters) = keyed_array.parameters.as_ref() {
                    return Some(parameters.0.as_ref().clone());
                }

                None
            }
            Self::List(_) => Some(get_int()),
        }
    }

    /// Returns the value type of the array, if available.
    #[must_use]
    pub fn get_value_type(&self) -> Option<TUnion> {
        match self {
            Self::Keyed(keyed_array) => {
                if let Some(parameters) = keyed_array.parameters.as_ref() {
                    return Some(parameters.1.as_ref().clone());
                }

                None
            }
            Self::List(list) => Some(list.element_type.as_ref().clone()),
        }
    }

    /// Checks if the array is truthy (non-empty or contains known definite elements).
    #[inline]
    #[must_use]
    pub fn is_truthy(&self) -> bool {
        if self.callable().is_some() {
            return true;
        }
        match &self {
            Self::Keyed(keyed_array) => {
                if keyed_array.non_empty {
                    return true;
                }

                if let Some(known_items) = keyed_array.get_known_items() {
                    for (optional, _) in known_items.values() {
                        if !optional {
                            return true;
                        }
                    }
                }

                false
            }
            Self::List(list) => {
                if list.non_empty {
                    return true;
                }

                if let Some(known_elements) = list.get_known_elements() {
                    for (optional, _) in known_elements.values() {
                        if !optional {
                            return true;
                        }
                    }
                }

                false
            }
        }
    }

    /// Checks if the array is falsy (empty or contains no known elements).
    #[inline]
    #[must_use]
    pub fn is_falsy(&self) -> bool {
        if self.callable().is_some() {
            return false;
        }
        match &self {
            Self::Keyed(keyed_array) => {
                if keyed_array.known_items.is_some() {
                    return false;
                }

                if keyed_array
                    .parameters
                    .as_ref()
                    .is_some_and(|parameters| !parameters.0.is_never() && !parameters.1.is_never())
                {
                    return false;
                }

                !keyed_array.non_empty
            }
            Self::List(list) => list.known_elements.is_none() && list.element_type.is_never() && !list.non_empty,
        }
    }

    /// Returns true if any type parameter in this array is a Placeholder.
    #[inline]
    #[must_use]
    pub fn contains_placeholder(&self) -> bool {
        match self {
            Self::Keyed(keyed_array) => keyed_array
                .parameters
                .as_ref()
                .is_some_and(|p| p.0.contains_placeholder() || p.1.contains_placeholder()),
            Self::List(list) => list.element_type.contains_placeholder(),
        }
    }

    /// Removes placeholder types from the array type.
    #[inline]
    pub fn remove_placeholders(&mut self) {
        match self {
            Self::Keyed(keyed_array) => {
                if let Some(parameters) = keyed_array.parameters.as_mut() {
                    if matches!(parameters.0.get_single(), TAtomic::Placeholder) {
                        *Arc::make_mut(&mut parameters.0) = get_arraykey();
                    }

                    if matches!(parameters.1.get_single(), TAtomic::Placeholder) {
                        *Arc::make_mut(&mut parameters.1) = get_mixed();
                    }
                }
            }
            Self::List(list) => {
                if matches!(list.element_type.get_single(), TAtomic::Placeholder) {
                    *Arc::make_mut(&mut list.element_type) = get_mixed();
                }
            }
        }
    }
}

impl TType for TArray {
    fn can_be_intersected(&self) -> bool {
        true
    }

    fn get_intersection_types(&self) -> Option<&[TAtomic]> {
        self.callable().map(std::slice::from_ref)
    }

    fn has_intersection_types(&self) -> bool {
        self.callable().is_some()
    }

    fn add_intersection_type(&mut self, intersection_type: TAtomic) -> bool {
        if !matches!(intersection_type, TAtomic::Callable(_)) {
            return false;
        }
        let callable = Some(Arc::new(intersection_type));
        match self {
            Self::Keyed(array) => array.callable = callable,
            Self::List(array) => array.callable = callable,
        }
        true
    }

    fn append_child_nodes<'types>(&'types self, children: &mut Vec<TypeRef<'types>>) {
        if let Some(callable) = self.callable() {
            children.push(TypeRef::Atomic(callable));
        }
        match self {
            TArray::Keyed(keyed_array) => keyed_array.append_child_nodes(children),
            TArray::List(list) => list.append_child_nodes(children),
        }
    }

    fn needs_population(&self) -> bool {
        self.callable().is_some_and(TType::needs_population)
            || match self {
                TArray::Keyed(keyed_array) => keyed_array.needs_population(),
                TArray::List(list) => list.needs_population(),
            }
    }

    fn is_expandable(&self) -> bool {
        self.callable().is_some_and(TType::is_expandable)
            || match self {
                TArray::Keyed(keyed_array) => keyed_array.is_expandable(),
                TArray::List(list) => list.is_expandable(),
            }
    }

    fn is_complex(&self) -> bool {
        match self {
            TArray::Keyed(keyed_array) => keyed_array.is_complex(),
            TArray::List(list) => list.is_complex(),
        }
    }

    fn get_id(&self) -> Word {
        let array = match self {
            TArray::List(list_data) => list_data.get_id(),
            TArray::Keyed(keyed_data) => keyed_data.get_id(),
        };
        self.callable().map_or(array, |callable| concat_word!(array, b"&", callable.get_id()))
    }

    fn get_pretty_id_with_indent(&self, indent: usize) -> Word {
        let array = match self {
            TArray::List(list_data) => list_data.get_pretty_id_with_indent(indent),
            TArray::Keyed(keyed_data) => keyed_data.get_pretty_id_with_indent(indent),
        };
        self.callable().map_or(array, |callable| concat_word!(array, b"&", callable.get_pretty_id_with_indent(indent)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ttype::atomic::callable::TCallable;
    use crate::ttype::{get_explicit_mixed, get_literal_int, get_mixed_callable, get_string};

    #[test]
    fn callable_array_views_and_copies_retain_value_owned_constraints() {
        let codebase = CodebaseMetadata::default();
        for mut array in [
            TArray::Keyed(TKeyedArray::new_with_parameters(
                Arc::new(get_explicit_mixed()),
                Arc::new(get_explicit_mixed()),
            )),
            TArray::List(TList::new_non_empty(Arc::new(get_explicit_mixed()))),
        ] {
            assert!(!array.has_intersection_types());
            assert!(!array.add_intersection_type(get_int().get_single().clone()));
            assert!(array.add_intersection_type(get_mixed_callable().get_single().clone()));
            assert!(array.has_intersection_types());
            assert!(array.get_child_nodes().iter().any(|node| matches!(node, TypeRef::Atomic(TAtomic::Callable(_)))));
            assert_eq!(array.get_size(), get_literal_int(2));
            assert!(array.get_id().as_bytes().contains(&b'&'));
            assert!(array.get_pretty_id().as_bytes().contains(&b'&'));
            assert_eq!(array.refine_callable_offset(&get_literal_int(1), get_mixed(), &codebase), get_truthy_string());
            assert_eq!(
                array.refine_callable_offset(
                    &crate::ttype::get_literal_string(mago_word::word("1")),
                    get_mixed(),
                    &codebase
                ),
                get_truthy_string()
            );
            let view = array.callable_array_view(&codebase).unwrap();
            assert_eq!(view.known_items.as_ref().unwrap()[&ArrayKey::Integer(1)].1, get_truthy_string());
            assert!(view.callable.is_none());

            let mut copy = array.clone();
            let Some(TAtomic::Callable(TCallable::Signature(signature))) = copy.callable_mut() else {
                panic!("callable signature")
            };
            signature.return_type = Some(Arc::new(get_string()));
            let Some(TAtomic::Callable(TCallable::Signature(original))) = array.callable() else {
                panic!("callable signature")
            };
            assert_ne!(original.return_type, Some(Arc::new(get_string())));
            assert_ne!(copy, array);
            if let TArray::List(list) = copy {
                assert_eq!(list.clone_non_empty().callable, list.callable);
            }
        }
    }

    #[test]
    fn ordinary_arrays_do_not_acquire_callable_views_or_offset_constraints() {
        let codebase = CodebaseMetadata::default();
        let list = TArray::List(TList::from_known_elements(BTreeMap::from([
            (0, (false, get_int())),
            (1, (true, get_string())),
        ])));
        assert!(list.callable_array_view(&codebase).is_none());
        assert_eq!(list.refine_callable_offset(&get_literal_int(1), get_int(), &codebase), get_int());
        assert_eq!(list.get_size(), crate::ttype::get_int_range(Some(1), Some(2)));
        let keyed = TArray::Keyed(
            TKeyedArray::new().with_known_items(BTreeMap::from([(ArrayKey::Integer(0), (false, get_int()))])),
        );
        assert_eq!(keyed.get_size(), get_literal_int(1));
        assert_eq!(
            TArray::List(TList::new(Arc::new(get_string()))).get_size(),
            crate::ttype::get_int_range(Some(0), None)
        );
    }

    #[test]
    fn count_union_views_merge_one_optional_successor_but_not_missing_cardinalities() {
        let codebase = CodebaseMetadata::default();
        let one = TArray::List(TList::from_known_elements(BTreeMap::from([(0, (false, get_int()))])));
        let two = TArray::Keyed(TKeyedArray::new().with_known_items(BTreeMap::from([
            (ArrayKey::Integer(0), (false, get_int())),
            (ArrayKey::Integer(1), (false, get_string())),
        ])));
        let three = TArray::Keyed(TKeyedArray::new().with_known_items(BTreeMap::from([
            (ArrayKey::Integer(0), (false, get_int())),
            (ArrayKey::Integer(1), (false, get_string())),
            (ArrayKey::Integer(2), (false, get_string())),
        ])));
        let gap = TUnion::from_vec(vec![get_literal_int(1).get_single().clone(), get_literal_int(3).get_single().clone()]);
        for arrays in [vec![&one, &two], vec![&two, &one], vec![&two, &one, &two]] {
            // PHPStan's count-joins.php capture: a one-element conditional append is int<1, 2>.
            assert_eq!(TArray::get_union_size(&arrays, &codebase), crate::ttype::get_int_range(Some(1), Some(2)));
        }
        for arrays in [vec![&one, &three], vec![&three, &one]] {
            // There is no two-element possibility; an interval hull would invent one.
            assert_eq!(TArray::get_union_size(&arrays, &codebase), gap);
        }
        assert_eq!(TArray::get_union_size(&[&three, &two, &one], &codebase), crate::ttype::get_int_range(Some(1), Some(3)));
    }

    #[test]
    fn count_union_views_keep_disjoint_keys_values_and_callable_conjuncts() {
        let codebase = CodebaseMetadata::default();
        let shape = |items: Vec<(i64, bool, TUnion)>| TArray::Keyed(TKeyedArray::new().with_known_items(
            items.into_iter().map(|(key, optional, value)| (ArrayKey::Integer(key), (optional, value))).collect(),
        ));
        let one = shape(vec![(0, false, get_int())]);
        let different = shape(vec![(0, false, get_string()), (1, false, get_int())]);
        let partial = shape(vec![(0, false, get_int()), (2, false, get_int())]);
        let two = shape(vec![(0, false, get_int()), (1, false, get_int())]);
        let gap = TUnion::from_vec(vec![get_literal_int(1).get_single().clone(), get_literal_int(2).get_single().clone()]);
        assert_eq!(TArray::get_union_size(&[&one, &different], &codebase), gap);
        assert_eq!(TArray::get_union_size(&[&partial, &two], &codebase), get_literal_int(2));
        let disjoint = shape(vec![(3, false, get_int()), (4, false, get_int())]);
        assert_eq!(TArray::get_union_size(&[&one, &disjoint], &codebase), gap);
        let optional = shape(vec![(0, false, get_string()), (1, true, get_int())]);
        assert_eq!(TArray::get_union_size(&[&two, &optional], &codebase), crate::ttype::get_int_range(Some(1), Some(2)));
        let two_different = shape(vec![(0, false, get_string()), (1, false, get_string())]);
        assert_eq!(TArray::get_union_size(&[&two, &two_different], &codebase), get_literal_int(2));
        let empty = TArray::List(TList::new(Arc::new(get_never())));
        assert_eq!(TArray::get_union_size(&[&empty], &codebase), get_literal_int(0));
        let mut counted_empty = TList::new(Arc::new(get_never()));
        counted_empty.known_count = Some(0);
        assert_eq!(TArray::get_union_size(&[&TArray::List(counted_empty)], &codebase), get_literal_int(0));
        let mut counted_optional = TList::from_known_elements(BTreeMap::from([
            (0, (false, get_int())), (1, (true, get_int())),
        ]));
        counted_optional.known_count = Some(2);
        assert_eq!(TArray::get_union_size(&[&TArray::List(counted_optional)], &codebase), get_literal_int(2));
        let mut non_empty = shape(vec![(0, true, get_int()), (1, true, get_string())]);
        let TArray::Keyed(keyed) = &mut non_empty else { unreachable!() };
        keyed.non_empty = true;
        assert_eq!(TArray::get_union_size(&[&non_empty], &codebase), crate::ttype::get_int_range(Some(1), Some(2)));
        let mut callable = TArray::Keyed(TKeyedArray::new_with_parameters(Arc::new(get_mixed()), Arc::new(get_mixed())));
        callable.add_intersection_type(get_mixed_callable().get_single().clone());
        assert_eq!(TArray::get_union_size(&[&callable, &one], &codebase), gap);
    }

    #[test]
    fn callable_array_signatures_participate_in_population_expansion_and_template_replacement() {
        use crate::misc::GenericParent;
        use crate::reference::SymbolReferences;
        use crate::ttype::atomic::callable::TCallableSignature;
        use crate::ttype::atomic::generic::TGenericParameter;
        use crate::ttype::atomic::object::{TObject, named::TNamedObject};
        use crate::ttype::atomic::reference::TReference;
        use crate::ttype::expander::{TypeExpansionOptions, expand_union};
        use crate::ttype::template::{TemplateResult, inferred_type_replacer};
        use mago_word::word;

        let mut codebase = CodebaseMetadata::default();
        codebase.symbols.add_symbol_name(word("state"), crate::symbol::SymbolKind::Enum);
        for mut array in [
            TArray::Keyed(TKeyedArray::new_with_parameters(Arc::new(get_mixed()), Arc::new(get_mixed()))),
            TArray::List(TList::new(Arc::new(get_mixed()))),
        ] {
            let signature = |return_type| {
                TAtomic::Callable(TCallable::Signature(
                    TCallableSignature::new(false, false).with_return_type(Some(Arc::new(return_type))),
                ))
            };
            array.add_intersection_type(signature(TUnion::from_atomic(TAtomic::Reference(TReference::Symbol {
                name: word("state"),
                parameters: None,
                variances: None,
                intersection_types: None,
            }))));
            assert!(array.needs_population());
            assert!(array.is_expandable());
            assert!(array.is_list());
            assert!(array.is_non_empty());
            assert!(array.is_truthy());
            assert!(!array.is_falsy());
            assert!(!array.is_vanilla());
            assert_eq!(array.get_minimum_size(), 2);
            let mut atomic = TAtomic::Array(array);
            crate::ttype::atomic::populate_atomic_type(
                &mut atomic,
                &codebase.symbols,
                None,
                &mut SymbolReferences::new(),
                true,
            );
            let TAtomic::Array(mut array) = atomic else { panic!("array") };
            assert_eq!(
                array.callable(),
                Some(&signature(TUnion::from_atomic(TAtomic::Object(TObject::new_enum(word("state"))))))
            );

            array.add_intersection_type(signature(TUnion::from_atomic(TAtomic::Object(TObject::Named(
                TNamedObject::new(word("self")),
            )))));
            let mut expanded = TUnion::from_atomic(TAtomic::Array(array.clone()));
            expand_union(
                &codebase,
                &mut expanded,
                &TypeExpansionOptions { self_class: Some(word("example")), ..Default::default() },
            );
            let TAtomic::Array(expanded) = expanded.get_single() else { panic!("array") };
            assert_eq!(
                expanded.callable(),
                Some(&signature(TUnion::from_atomic(TAtomic::Object(TObject::Named(TNamedObject::new(word(
                    "example"
                )))))))
            );

            let parent = GenericParent::ClassLike(word("example"));
            array.add_intersection_type(signature(TUnion::from_atomic(TAtomic::GenericParameter(
                TGenericParameter::new(word("T"), Arc::new(get_mixed()), parent),
            ))));
            let mut templates = TemplateResult::default();
            templates.add_lower_bound(word("T"), parent, get_string());
            let replaced =
                inferred_type_replacer::replace(&TUnion::from_atomic(TAtomic::Array(array)), &templates, &codebase);
            let TAtomic::Array(replaced) = replaced.get_single() else { panic!("array") };
            assert_eq!(replaced.callable(), Some(&signature(get_string())));
        }
    }
}
