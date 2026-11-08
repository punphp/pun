//! PHPStan 2.3.5's `RegexArrayShapeMatcher` (with `RegexGroupList`), building the types of `$matches`.

use std::sync::Arc;

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::get_int_range;
use mago_codex::ttype::union::TUnion;
use pun_phpstan::types::Tri;

use super::groups::{self, Capturing, Parent};
use super::types::{Atom, Flags, RType, is_super_type};

pub(super) const PREG_PATTERN_ORDER: i64 = 1;
pub(super) const PREG_SET_ORDER: i64 = 2;
pub(super) const PREG_OFFSET_CAPTURE: i64 = 256;
pub(super) const PREG_UNMATCHED_AS_NULL: i64 = 512;
/// `RegexArrayShapeMatcher::PREG_UNMATCHED_AS_NULL_ON_72_73`.
const PREG_UNMATCHED_AS_NULL_ON_72_73: i64 = 2048;

/// A `$matches` entry's value: `ty`, as `array{ty, int<-1, max>}` with `offset` (`PREG_OFFSET_CAPTURE`), in a
/// `list<...>` (or a `non-empty-list<...>` with `Some(true)`) with `list`. One call's patterns share the flags, so
/// the values at an offset are all of one kind.
#[derive(Clone, Debug, PartialEq)]
struct Value {
    ty: RType,
    offset: bool,
    list: Option<bool>,
    /// `non-empty-array<int<0, max>, ty>`: preg_match_all()'s marks by the index of the match that set them.
    by_match: bool,
}

impl Value {
    fn str(ty: RType) -> Self {
        Self {
            ty,
            offset: false,
            list: None,
            by_match: false,
        }
    }

    fn plain(&self) -> bool {
        !self.offset && self.list.is_none() && !self.by_match
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Key {
    Int(i64),
    Str(String),
}

/// A constant array (`ConstantArrayType`).
#[derive(Clone, Debug, PartialEq)]
struct Shape {
    entries: Vec<(Key, Value, bool)>,
    list: bool,
}

/// A type of `$matches`.
#[derive(Clone, Debug, PartialEq)]
enum Out {
    Shape(Shape),
    /// `list<shape>` (`PREG_SET_ORDER`), non-empty or not.
    SetOrder(Vec<Shape>, bool),
}

/// The pieces of `RegexArrayShapeMatcher`'s state that come from configuration.
#[derive(Clone, Copy)]
pub(super) struct Php {
    /// `PhpVersion::supportsPregUnmatchedAsNull()`.
    pub(super) unmatched_as_null: bool,
    /// `PhpVersion::supportsPregCaptureOnlyNamedGroups()`.
    pub(super) capture_only_named: bool,
}

/// The flags argument: absent, a constant integer, or anything else (no shape then).
#[derive(Clone, Copy, Debug)]
pub(super) enum FlagsArg {
    Absent,
    Constant(i64),
    Unknown,
}

/// `matchPatternType()`: the type of `$matches` after matching each of `patterns`, or `None` for the declared one.
pub(super) fn match_pattern(
    php: Php,
    patterns: &[String],
    flags: FlagsArg,
    was_matched: Tri,
    matches_all: bool,
) -> Option<TUnion> {
    if was_matched.no() {
        return Some(get_keyed_array_empty());
    }
    if patterns.is_empty() {
        return None;
    }
    let flags = match flags {
        FlagsArg::Absent => None,
        FlagsArg::Unknown => return None,
        FlagsArg::Constant(value) => {
            let flags = value
                & (PREG_OFFSET_CAPTURE
                    | PREG_PATTERN_ORDER
                    | PREG_SET_ORDER
                    | PREG_UNMATCHED_AS_NULL
                    | PREG_UNMATCHED_AS_NULL_ON_72_73);
            // some other unsupported/unexpected flag was passed in
            if flags != value {
                return None;
            }
            Some(flags)
        }
    };
    let mut outs = Vec::new();
    for pattern in patterns {
        outs.extend(match_regex(php, pattern, flags, was_matched, matches_all)?);
    }
    Some(to_union(&union(outs)))
}

/// `matchSubjectExpr()`'s type for constant patterns.
pub(super) fn match_subject(php: Php, patterns: &[String]) -> Option<TUnion> {
    if patterns.is_empty() {
        return None;
    }
    let mut subject = Vec::new();
    for pattern in patterns {
        subject.push(RType::from(
            groups::parse_groups(pattern, php.capture_only_named)?.subject_base_type,
        ));
    }
    Some(RType::union(subject).to_union())
}

/// `TypeCombinator::union(array{}, $matched)`: preg_match()'s out type.
pub(super) fn with_empty_array(matched: TUnion) -> TUnion {
    let mut types = vec![TAtomic::Array(TArray::Keyed(TKeyedArray::new()))];
    types.extend(matched.types.iter().filter(|a| !is_empty_array(a)).cloned());
    TUnion::from_vec(types)
}

fn is_empty_array(atomic: &TAtomic) -> bool {
    matches!(atomic, TAtomic::Array(TArray::Keyed(k)) if k.known_items.is_none() && k.parameters.is_none())
}

/// `[]`.
fn get_keyed_array_empty() -> TUnion {
    TUnion::from_atomic(TAtomic::Array(TArray::Keyed(TKeyedArray::new())))
}

struct Matcher {
    php: Php,
    flags: i64,
    was_matched: Tri,
    matches_all: bool,
}

/// `matchRegex()`.
fn match_regex(
    php: Php,
    regex: &str,
    flags: Option<i64>,
    was_matched: Tri,
    matches_all: bool,
) -> Option<Vec<Out>> {
    // `None`: regex could not be parsed by RegexParser
    let walk = groups::parse_groups(regex, php.capture_only_named)?;
    let m = Matcher {
        php,
        flags: flags.unwrap_or(0),
        was_matched,
        matches_all,
    };
    let subject_base_type = if was_matched.yes() {
        RType::from(walk.subject_base_type.clone())
    } else {
        string()
    };
    let list = GroupList(walk.groups.clone());
    let trailing_optionals = list.count_trailing_optionals();
    let only_optional_top_level_group = list.only_optional_top_level_group();
    let only_top_level_alternation = list.only_top_level_alternation();

    let branches = (!matches_all && only_optional_top_level_group.is_none() && !m.contains_unmatched_as_null())
        .then_some(only_top_level_alternation)
        .flatten()
        .map(|id| &walk.alternations[id]);
    if list.has_duplicate_names_outside_of_alternation_branches(branches) {
        // which of the groups sharing a name sets the named offset is only modelled
        // for groups in different branches of a top level alternation
        return None;
    }
    if list.0.iter().any(Capturing::resets_group_counter)
        && (matches_all
            || only_optional_top_level_group.is_some()
            || only_top_level_alternation.is_none()
            || m.contains_unmatched_as_null())
    {
        // groups in a branch reset (?|...) share offsets, which is only modelled
        // when its alternation is the only top level alternation
        return None;
    }

    if !matches_all
        && was_matched.yes()
        && let Some(group) = only_optional_top_level_group
    {
        // if only one top level capturing optional group exists
        // we build a more precise tagged union of a empty-match and a match with the group
        let list = list.force_group_non_optional(&group);
        let mut outs = vec![m.build_array_type(
            &subject_base_type,
            &list,
            trailing_optionals,
            &walk.mark_verbs,
        )];
        if !m.contains_unmatched_as_null() {
            // positive match has a subject but not any capturing group
            outs.insert(0, m.subject_only(&subject_base_type));
        }
        return Some(outs);
    }
    if !matches_all
        && only_optional_top_level_group.is_none()
        && let Some(alternation_id) = only_top_level_alternation
    {
        // if only a single top level alternation exist built a more precise tagged union
        let alternation = &walk.alternations[alternation_id];
        let mut outs = Vec::new();
        let mut is_optional_alternation = false;
        for (_, combo) in &alternation.combinations {
            let mut combo_list = list.clone();
            let mut before_current_combo = true;
            for group in &list.0 {
                if combo.contains(&group.id) {
                    is_optional_alternation = group.in_optional_alternation();
                    combo_list = combo_list.force_group_alternation_branch_matched(group);
                    before_current_combo = false;
                } else if before_current_combo && !group.resets_group_counter() {
                    combo_list = combo_list
                        .force_group_type_and_non_optional(group, m.unmatched_group_type());
                } else if group.alternation == Some(alternation_id)
                    && !m.contains_unmatched_as_null()
                {
                    combo_list = combo_list.remove_group(group);
                }
            }
            outs.push(m.build_array_type(
                &subject_base_type,
                &combo_list,
                trailing_optionals,
                &walk.mark_verbs,
            ));
        }
        if !m.contains_unmatched_as_null()
            && (alternation.count != alternation.combinations.len() || is_optional_alternation)
        {
            // positive match has a subject but not any capturing group
            outs.push(m.subject_only(&subject_base_type));
        }
        return Some(outs);
    }
    // the general case, which should work in all cases but does not yield the most
    // precise result possible in some cases
    Some(vec![m.build_array_type(
        &subject_base_type,
        &list,
        trailing_optionals,
        &walk.mark_verbs,
    )])
}

fn string() -> RType {
    Atom::Str(Flags::default()).into()
}

impl Matcher {
    /// `ConstantArrayTypeBuilder` with the subject at offset 0 only.
    fn subject_only(&self, subject_base_type: &RType) -> Out {
        Out::Shape(Shape {
            entries: vec![(
                Key::Int(0),
                self.create_subject_value_type(subject_base_type),
                false,
            )],
            list: true,
        })
    }

    /// `buildArrayType()`.
    fn build_array_type(
        &self,
        subject_base_type: &RType,
        groups: &GroupList,
        trailing_optionals: usize,
        mark_verbs: &[String],
    ) -> Out {
        let mut force_list = mark_verbs.is_empty();
        let mut shape = Shape {
            entries: Vec::new(),
            list: false,
        };
        // first item in matches contains the overall match.
        set_offset(
            &mut shape,
            Key::Int(0),
            self.create_subject_value_type(subject_base_type),
            self.is_subject_optional(),
        );
        let count = groups.0.len();
        for (i, group) in groups.0.iter().enumerate() {
            let is_trailing_optional = i + trailing_optionals >= count;
            let is_last_group = i + 1 == count;
            let value = self.create_group_value_type(group, is_trailing_optional, is_last_group);
            let optional = self.is_group_optional(group, is_trailing_optional);
            if let Some(name) = &group.name {
                force_list = false;
                set_offset(&mut shape, Key::Str(name.clone()), value.clone(), optional);
            }
            set_offset(&mut shape, Key::Int(i as i64 + 1), value, optional);
        }
        if !mark_verbs.is_empty() {
            let marks = RType::union(
                mark_verbs
                    .iter()
                    .map(|m| RType::from(Atom::Const(m.clone()))),
            );
            // marks are keyed by the index of the match that set them, matches without a mark are left out
            let by_match = self.matches_all && self.contains_pattern_order();
            set_offset(&mut shape, Key::Str("MARK".into()), Value { by_match, ..Value::str(marks) }, true);
        }
        if self.matches_all && self.contains_set_order() {
            return Out::SetOrder(vec![shape], self.was_matched.yes());
        }
        shape.list = force_list;
        Out::Shape(shape)
    }

    fn is_subject_optional(&self) -> bool {
        !self.matches_all && !self.was_matched.yes()
    }

    /// `createSubjectValueType()`.
    fn create_subject_value_type(&self, base_type: &RType) -> Value {
        if !self.matches_all {
            return remove_null(self.value_type(base_type));
        }
        let value = remove_null(self.value_type(&string()));
        if self.contains_pattern_order() {
            return Value {
                list: Some(self.was_matched.yes()),
                ..value
            };
        }
        value
    }

    /// `isGroupOptional()`.
    fn is_group_optional(&self, group: &Capturing, is_trailing_optional: bool) -> bool {
        if self.matches_all {
            return is_trailing_optional
                && !self.contains_unmatched_as_null()
                && self.contains_set_order();
        }
        // preg_match()'s shapes are always for a match (PregMatchParameterOutTypeExtension adds `array{}` itself),
        // so upstream's `!$wasMatched->yes()` case does not occur here
        is_trailing_optional && !self.contains_unmatched_as_null() && group.is_optional()
    }

    /// `createGroupValueType()`.
    fn create_group_value_type(
        &self,
        group: &Capturing,
        is_trailing_optional: bool,
        is_last_group: bool,
    ) -> Value {
        let unmatched_as_null = self.contains_unmatched_as_null();
        if self.matches_all {
            let mut value = if !unmatched_as_null
                && group.is_optional()
                // an unmatched group followed by a matched one is reported as ''
                && (!self.contains_set_order() || !is_last_group)
            {
                remove_null(self.value_type(&RType::union([
                    group.ty().clone(),
                    Atom::Const(String::new()).into(),
                ])))
            } else {
                self.value_type(group.ty())
            };
            if !is_trailing_optional && unmatched_as_null && !group.is_optional() {
                value = remove_null(value);
            }
            if self.contains_pattern_order() {
                value = Value {
                    list: Some(self.was_matched.yes()),
                    ..value
                };
            }
            return value;
        }
        let mut value = if !is_last_group && !unmatched_as_null && group.is_optional() {
            self.value_type(&RType::union([
                group.ty().clone(),
                self.unmatched_group_type(),
            ]))
        } else {
            self.value_type(group.ty())
        };
        if self.was_matched.yes()
            && !is_trailing_optional
            && unmatched_as_null
            && !group.is_optional()
        {
            value = remove_null(value);
        }
        value
    }

    fn contains_offset_capture(&self) -> bool {
        self.flags & PREG_OFFSET_CAPTURE != 0
    }

    /// If no order flag is given, PREG_PATTERN_ORDER is assumed.
    fn contains_pattern_order(&self) -> bool {
        !self.contains_set_order()
    }

    fn contains_set_order(&self) -> bool {
        self.flags & PREG_SET_ORDER != 0
    }

    /// `containsUnmatchedAsNull()`: preg_match_all() with PREG_UNMATCHED_AS_NULL works consistently across php-versions.
    fn contains_unmatched_as_null(&self) -> bool {
        self.flags & PREG_UNMATCHED_AS_NULL != 0
            && (self.matches_all
                || self.flags & PREG_UNMATCHED_AS_NULL_ON_72_73 != 0
                || self.php.unmatched_as_null)
    }

    /// `getUnmatchedGroupType()`: the type of an unmatched group followed by a matched one.
    fn unmatched_group_type(&self) -> RType {
        if self.flags & PREG_UNMATCHED_AS_NULL != 0 {
            Atom::Null.into()
        } else {
            Atom::Const(String::new()).into()
        }
    }

    /// `getValueType()`; unmatched groups return -1 as offset.
    fn value_type(&self, base_type: &RType) -> Value {
        let mut value = base_type.clone();
        if self.contains_unmatched_as_null() {
            value = RType::union([value, Atom::Null.into()]);
        }
        Value {
            ty: value,
            offset: self.contains_offset_capture(),
            list: None,
            by_match: false,
        }
    }
}

/// `TypeCombinator::removeNull()` of a value.
fn remove_null(value: Value) -> Value {
    if value.plain() {
        Value::str(value.ty.remove_null())
    } else {
        value
    }
}

/// `ConstantArrayTypeBuilder::setOffsetValueType()` with a constant key.
fn set_offset(shape: &mut Shape, key: Key, value: Value, optional: bool) {
    if let Some(entry) = shape.entries.iter_mut().find(|(k, ..)| *k == key) {
        entry.1 = if optional {
            union_values(&value, &entry.1)
        } else {
            value
        };
        entry.2 &= optional;
        return;
    }
    shape.entries.push((key, value, optional));
}

/// `RegexGroupList`.
#[derive(Clone)]
struct GroupList(Vec<Capturing>);

impl GroupList {
    fn count_trailing_optionals(&self) -> usize {
        self.0.iter().rev().take_while(|g| g.is_optional()).count()
    }

    fn force_group_non_optional(&self, group: &Capturing) -> Self {
        self.clone_and_re_parent(group, None, false)
    }

    fn force_group_type_and_non_optional(&self, group: &Capturing, ty: RType) -> Self {
        self.clone_and_re_parent(group, Some(ty), false)
    }

    fn force_group_alternation_branch_matched(&self, group: &Capturing) -> Self {
        self.clone_and_re_parent(group, None, true)
    }

    /// `cloneAndReParentList()`: a group below the target gets the forced group as its direct parent.
    fn clone_and_re_parent(
        &self,
        target: &Capturing,
        ty: Option<RType>,
        alternation_branch_only: bool,
    ) -> Self {
        let mut forced = None;
        let mut groups: Vec<Capturing> = self
            .0
            .iter()
            .map(|g| {
                if g.id != target.id {
                    return g.clone();
                }
                let mut f = if alternation_branch_only {
                    g.force_alternation_branch_matched()
                } else {
                    g.force_non_optional()
                };
                if let Some(ty) = &ty {
                    f = f.force_type(ty.clone());
                }
                forced = Some(f.clone());
                f
            })
            .collect();
        let forced = forced.expect("the target is one of the list's groups");
        for group in &mut groups {
            let mut parent = group.parent().cloned();
            while let Some(p) = parent {
                if let Parent::Capturing(c) = &*p
                    && c.id == target.id
                {
                    *group = group.with_parent(forced.clone());
                }
                parent = p.parent().cloned();
            }
        }
        GroupList(groups)
    }

    fn remove_group(&self, remove: &Capturing) -> Self {
        GroupList(
            self.0
                .iter()
                .filter(|g| g.id != remove.id)
                .cloned()
                .collect(),
        )
    }

    /// `hasDuplicateNamesOutsideOfAlternationBranches()`: groups sharing a name (allowed by (?J)) are only understood
    /// when each of them is in a different branch of the given alternation.
    fn has_duplicate_names_outside_of_alternation_branches(&self, alternation: Option<&groups::Alternation>) -> bool {
        let mut by_name: Vec<(&str, Vec<i64>)> = Vec::new();
        for group in &self.0 {
            let Some(name) = &group.name else { continue };
            match by_name.iter_mut().find(|(n, _)| n == name) {
                Some((_, ids)) => ids.push(group.id),
                None => by_name.push((name, vec![group.id])),
            }
        }
        let branch_of = |id: i64| {
            alternation?.combinations.iter().rev().find(|(_, ids)| ids.contains(&id)).map(|(branch, _)| *branch)
        };
        by_name.iter().filter(|(_, ids)| ids.len() > 1).any(|(_, ids)| {
            let mut branches = Vec::new();
            ids.iter().any(|id| match branch_of(*id) {
                None => true,
                Some(branch) if branches.contains(&branch) => true,
                Some(branch) => {
                    branches.push(branch);
                    false
                }
            })
        })
    }

    fn only_optional_top_level_group(&self) -> Option<Capturing> {
        let mut found = None;
        for group in self.0.iter().filter(|g| g.is_top_level()) {
            if !group.is_optional() || found.is_some() {
                return None;
            }
            found = Some(group.clone());
        }
        found
    }

    fn only_top_level_alternation(&self) -> Option<usize> {
        let mut alternation = None;
        for group in self.0.iter().filter(|g| g.is_top_level()) {
            if group.in_optional_quantification() {
                return None;
            }
            let id = group.alternation?;
            if alternation.is_some_and(|a| a != id) {
                return None;
            }
            alternation = Some(id);
        }
        alternation
    }
}

/// `TypeCombinator::union()` of the types of `$matches`: constant arrays reduced as `reduceArrays()` (with tagged
/// unions preserved and without bleeding edge's unsealed arrays) reduces them, `list<...>` types merged.
fn union(outs: Vec<Out>) -> Vec<Out> {
    let mut shapes = Vec::new();
    let mut set_order: Option<(Vec<Shape>, bool)> = None;
    for out in outs {
        match out {
            Out::Shape(shape) => shapes.push(shape),
            Out::SetOrder(items, non_empty) => {
                let (all, all_non_empty) = set_order.get_or_insert_with(|| (Vec::new(), true));
                all.extend(items);
                *all_non_empty &= non_empty;
            }
        }
    }
    let mut out: Vec<Out> = reduce_arrays(shapes).into_iter().map(Out::Shape).collect();
    if let Some((items, non_empty)) = set_order {
        out.push(Out::SetOrder(reduce_arrays(items), non_empty));
    }
    out
}

/// `reduceArrays($constantArrays, true)` for sealed constant arrays (`legacyIsKeysSupersetOf()`, `legacyMergeWith()`).
fn reduce_arrays(shapes: Vec<Shape>) -> Vec<Shape> {
    // every shape holds offset 0, so upstream's empty array, moved last, does not occur
    let mut arrays: Vec<Option<Shape>> = Vec::new();
    for shape in shapes {
        if !arrays.iter().flatten().any(|a| *a == shape) {
            arrays.push(Some(shape));
        }
    }

    let mut per_key: Vec<(Key, Vec<usize>)> = Vec::new();
    for (i, array) in arrays.iter().enumerate() {
        for (key, ..) in &array.as_ref().expect("not merged yet").entries {
            match per_key.iter_mut().find(|(k, _)| k == key) {
                Some((_, indices)) => indices.push(i),
                None => per_key.push((key.clone(), vec![i])),
            }
        }
    }
    let mut scalars: Vec<Option<Vec<(Key, Atom)>>> = arrays
        .iter()
        .map(|a| Some(constant_scalar_values(a.as_ref().expect("not merged yet"))))
        .collect();
    let mut eligible: Vec<(usize, Vec<(usize, usize)>)> = Vec::new();
    for (_, indices) in &per_key {
        for (n, &i) in indices.iter().enumerate() {
            for &j in &indices[n + 1..] {
                let others = match eligible.iter_mut().position(|(k, _)| *k == i) {
                    Some(at) => &mut eligible[at].1,
                    None => {
                        eligible.push((i, Vec::new()));
                        &mut eligible.last_mut().expect("just pushed").1
                    }
                };
                match others.iter_mut().find(|(k, _)| *k == j) {
                    Some((_, count)) => *count += 1,
                    None => others.push((j, 1)),
                }
            }
        }
    }

    for (i, others) in eligible {
        for (j, overlapping) in others {
            let (Some(a), Some(b)) = (&arrays[i], &arrays[j]) else {
                if arrays[i].is_none() {
                    break;
                }
                continue;
            };
            // Merge two single-key arrays sharing the same key when their value types union into a single type
            if overlapping == 1
                && a.entries.len() == 1
                && b.entries.len() == 1
                && !is_union(&union_values(&a.entries[0].1, &b.entries[0].1))
            {
                arrays[j] = Some(merge_with(b, a));
                scalars[j] = None;
                arrays[i] = None;
                break;
            }
            let same_count = a.entries.len() == b.entries.len() && a.entries.len() >= 2;
            if have_disjoint_constant_scalar_values(
                scalars[i].as_deref(),
                scalars[j].as_deref(),
                same_count,
            ) {
                // neither array is a keys-superset of the other
                continue;
            }
            if overlapping == a.entries.len() && is_keys_superset_of(b, a) {
                arrays[j] = Some(merge_with(b, a));
                scalars[j] = None;
                arrays[i] = None;
                break;
            }
            if overlapping == b.entries.len() && is_keys_superset_of(a, b) {
                arrays[i] = Some(merge_with(a, b));
                scalars[i] = None;
                arrays[j] = None;
            }
        }
    }
    arrays.into_iter().flatten().collect()
}

/// `getConstantScalarValuesByKey()`.
fn constant_scalar_values(shape: &Shape) -> Vec<(Key, Atom)> {
    shape
        .entries
        .iter()
        .filter_map(|(key, value, _)| match value {
            Value {
                ty: RType(atoms), ..
            } if value.plain() && atoms.len() == 1 && !matches!(atoms[0], Atom::Str(_)) => {
                Some((key.clone(), atoms[0].clone()))
            }
            _ => None,
        })
        .collect()
}

/// `haveDisjointConstantScalarValues()`.
fn have_disjoint_constant_scalar_values(
    a: Option<&[(Key, Atom)]>,
    b: Option<&[(Key, Atom)]>,
    one_unrelated_key_allowed: bool,
) -> bool {
    let (Some(mut a), Some(mut b)) = (a, b) else {
        return false;
    };
    if a.len() > b.len() {
        (a, b) = (b, a);
    }
    let mut differing = 0;
    for (key, value) in a {
        if b.iter()
            .find(|(k, _)| k == key)
            .is_none_or(|(_, v)| v == value)
        {
            continue;
        }
        differing += 1;
        if differing >= 2 || !one_unrelated_key_allowed {
            return true;
        }
    }
    false
}

/// `legacyIsKeysSupersetOf()`; `reduce_arrays()` asks only when `this` holds every key of `other`, so upstream's
/// shorter-array and missing-key checks never fail.
fn is_keys_superset_of(this: &Shape, other: &Shape) -> bool {
    let mut fail_on_different_value_type =
        this.entries.len() != other.entries.len() || this.entries.len() < 2;
    for (key, other_value, _) in &other.entries {
        let value = this
            .entries
            .iter()
            .find(|(k, ..)| k == key)
            .map(|(_, value, _)| value);
        if !value.is_some_and(|value| values_disjoint(other_value, value)) {
            continue;
        }
        if fail_on_different_value_type {
            return false;
        }
        fail_on_different_value_type = true;
    }
    this.entries
        .iter()
        .filter(|(key, _, optional)| !optional && !other.entries.iter().any(|(k, ..)| k == key))
        .count()
        <= 1
}

/// `legacyMergeWith()`.
fn merge_with(this: &Shape, other: &Shape) -> Shape {
    let entries = this
        .entries
        .iter()
        .map(
            |(key, value, optional)| match other.entries.iter().find(|(k, ..)| k == key) {
                None => (key.clone(), value.clone(), true),
                Some((_, other_value, other_optional)) => (
                    key.clone(),
                    union_values(value, other_value),
                    *optional || *other_optional,
                ),
            },
        )
        .collect();
    Shape {
        entries,
        list: this.list && other.list,
    }
}

/// `TypeCombinator::union()` of two values.
fn union_values(a: &Value, b: &Value) -> Value {
    Value {
        ty: RType::union([a.ty.clone(), b.ty.clone()]),
        offset: a.offset,
        list: a.list.zip(b.list).map(|(a, b)| a && b),
        by_match: a.by_match,
    }
}

/// A union type, not one array holding the union.
fn is_union(value: &Value) -> bool {
    value.plain() && value.ty.0.len() > 1
}

/// `$a->isSuperTypeOf($b)->no()`: no value of one is a value of the other.
fn values_disjoint(a: &Value, b: &Value) -> bool {
    a.ty.0
        .iter()
        .all(|x| b.ty.0.iter().all(|y| atoms_disjoint(x, y)))
}

fn atoms_disjoint(a: &Atom, b: &Atom) -> bool {
    match (a, b) {
        (Atom::Null, Atom::Null) | (Atom::Str(_), Atom::Str(_)) => false,
        (Atom::Const(a), Atom::Const(b)) => a != b,
        (Atom::Const(_), Atom::Str(_)) => !is_super_type(b, a),
        (Atom::Str(_), Atom::Const(_)) => !is_super_type(a, b),
        _ => true,
    }
}

fn to_union(outs: &[Out]) -> TUnion {
    TUnion::from_vec(
        outs.iter()
            .map(|out| match out {
                Out::Shape(shape) => shape_atomic(shape),
                Out::SetOrder(items, non_empty) => {
                    let items =
                        Arc::new(TUnion::from_vec(items.iter().map(shape_atomic).collect()));
                    TAtomic::Array(TArray::List(if *non_empty {
                        TList::new_non_empty(items)
                    } else {
                        TList::new(items)
                    }))
                }
            })
            .collect(),
    )
}

fn shape_atomic(shape: &Shape) -> TAtomic {
    if shape.list {
        return TAtomic::Array(TArray::List(TList::from_known_elements(
            shape
                .entries
                .iter()
                .enumerate()
                .map(|(i, (_, value, optional))| (i, (*optional, value_union(value))))
                .collect(),
        )));
    }
    let items = shape
        .entries
        .iter()
        .map(|(key, value, optional)| {
            let key = match key {
                Key::Int(i) => ArrayKey::Integer(*i),
                Key::Str(s) => ArrayKey::String(mago_word::word(s)),
            };
            (key, (*optional, value_union(value)))
        })
        .collect();
    let non_empty = shape.entries.iter().any(|(.., optional)| !optional);
    TAtomic::Array(TArray::Keyed(
        TKeyedArray::new()
            .with_known_items(items)
            .with_non_empty(non_empty),
    ))
}

fn value_union(value: &Value) -> TUnion {
    let mut ty = value.ty.to_union();
    if value.offset {
        ty = TUnion::from_atomic(TAtomic::Array(TArray::List(TList::from_known_elements(
            [
                (0, (false, ty)),
                (1, (false, get_int_range(Some(-1), None))),
            ]
            .into_iter()
            .collect(),
        ))));
    }
    if value.by_match {
        return TUnion::from_atomic(TAtomic::Array(TArray::Keyed(
            TKeyedArray::new_with_parameters(Arc::new(get_int_range(Some(0), None)), Arc::new(ty)).with_non_empty(true),
        )));
    }
    match value.list {
        None => ty,
        Some(non_empty) => {
            let item = Arc::new(ty);
            TUnion::from_atomic(TAtomic::Array(TArray::List(if non_empty {
                TList::new_non_empty(item)
            } else {
                TList::new(item)
            })))
        }
    }
}
