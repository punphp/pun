//! From a PHPStan type to the phpdoc type node Rector writes: `StaticTypeMapper::mapPHPStanTypeToPHPStanPhpDocTypeNode()`
//! (PHPStan's `Type::toPhpDocNode()` with Rector's `styleTypeNode()` on top, since Rector 2.8.4), and the steps before
//! it that the docblock rules share: `TypeNormalizer::generalizeConstantTypes()`, `TypeFactory::uniquateTypes()`,
//! `TypeHasher`, `ConstantArrayTypeGeneralizer` and `NodeDocblockTypeDecorator`'s checks.
//!
//! The types are pun-phpstan's port of PHPStan's; a printed union has its members in `UnionTypeHelper::sortTypes()`'s
//! order.

use std::cell::Cell;

use pun_phpstan::types::{Accessory, ConstArray, Level, ShapeBuilder, Type, intersect, mixed, sort_types, to_phpdoc, union};

use super::type_node::TypeNode;

/// `MixedType` (with `ErrorType`, which extends it).
pub fn is_mixed(t: &Type) -> bool {
    matches!(
        t,
        Type::Mixed { .. } | Type::SubtractedMixed { .. } | Type::Error
    )
}

/// `IntegerType` (with `ConstantIntegerType` and `IntegerRangeType`, which extend it).
pub fn is_integer_type(t: &Type) -> bool {
    matches!(t, Type::Int | Type::ConstInt(_) | Type::IntRange(..))
}


/// `ConstantArrayType::getKeyType()`.
pub fn const_array_key_type(a: &ConstArray) -> Type {
    Type::ConstArray(Box::new(a.clone())).iterable_key_type()
}

/// `ConstantArrayType::getItemType()`.
pub fn const_array_item_type(a: &ConstArray) -> Type {
    Type::ConstArray(Box::new(a.clone())).iterable_value_type()
}

/// `StaticTypeMapper::mapPHPStanTypeToPHPStanPhpDocTypeNode()`: the type with its integers plain (explicit values and
/// ranges are too detailed for a docblock), PHPStan's `toPhpDocNode()`, then Rector's docblock style.
pub fn map_to_phpdoc(t: &Type) -> TypeNode {
    style(Mapper { short_objects: false }.node(&plain_integers(t)))
}

/// `TypeTraverser::map()` of the integers to `IntegerType`, entering only unions and general arrays.
fn plain_integers(t: &Type) -> Type {
    plain_integers_changed(t).0
}

/// [`plain_integers`] and whether it replaced anything: every integer is a new `IntegerType`, and a union is only
/// normalized again (`TypeCombinator::union()`, which merges `int[]|string[]`) when a member changed, so
/// `list<int>|list<string>` (intersections, not entered) stays as it is.
fn plain_integers_changed(t: &Type) -> (Type, bool) {
    let members = |types: &[Type]| {
        let mapped: Vec<(Type, bool)> = types.iter().map(plain_integers_changed).collect();
        let changed = mapped.iter().any(|(_, c)| *c);
        (changed, mapped.into_iter().map(|(t, _)| t).collect::<Vec<_>>())
    };
    match t {
        Type::Int | Type::ConstInt(_) | Type::IntRange(..) => (Type::Int, true),
        Type::Union(types) => match members(types) {
            (true, mapped) => (union(mapped), true),
            (false, _) => (t.clone(), false),
        },
        Type::Benevolent(types) => match members(types) {
            (true, mapped) => match union(mapped) {
                Type::Union(types) => (Type::Benevolent(types), true),
                other => (other, true),
            },
            (false, _) => (t.clone(), false),
        },
        Type::Array(key, item) => {
            let (key, k) = plain_integers_changed(key);
            let (item, i) = plain_integers_changed(item);
            if k || i { (Type::array(key, item), true) } else { (t.clone(), false) }
        }
        _ => (t.clone(), false),
    }
}

struct Mapper {
    /// Objects as `ShortenedObjectType`s, printed by their short name (`RelativeNameNode`).
    short_objects: bool,
}

impl Mapper {
    /// PHPStan's `Type::toPhpDocNode()`: a class name is a `FullyQualifiedNameNode` (`\Foo`), a union's members are in
    /// `UnionTypeHelper::sortTypes()`'s order.
    fn node(&self, t: &Type) -> TypeNode {
        let implicit_mixed = |t: &Type| matches!(t, Type::Mixed { explicit: false });
        match t {
            Type::Union(types) | Type::Benevolent(types) => TypeNode::Union {
                types: sort_types(types).iter().map(|t| self.node(t)).collect(),
                wrapped: false,
            },
            Type::Array(key, item) if implicit_mixed(key) && implicit_mixed(item) => TypeNode::ident("array"),
            Type::Array(key, item) if implicit_mixed(key) => TypeNode::Generic {
                base: "array".to_string(),
                args: vec![self.node(item)],
            },
            Type::Array(key, item) => TypeNode::Generic {
                base: "array".to_string(),
                args: vec![self.node(key), self.node(item)],
            },
            Type::Object(name, args, _) if args.is_empty() => self.object_name(name),
            Type::Object(name, args, _) => TypeNode::Generic {
                base: self.object_name(name).to_string(),
                args: args.iter().map(|a| self.node(a)).collect(),
            },
            Type::ClassString(Some(inner)) => TypeNode::Generic {
                base: "class-string".to_string(),
                args: vec![self.node(inner)],
            },
            // `list<T>`, `non-empty-list<T>`, `non-empty-array<K, T>`
            Type::Intersection(types)
                if let Some(Type::Array(key, item)) = types.iter().find(|t| matches!(t, Type::Array(..))) =>
            {
                let has = |a: Accessory| types.contains(&Type::Accessory(a));
                let list = has(Accessory::List);
                let non_empty = has(Accessory::NonEmptyArray);
                if types.len() != 1 + usize::from(list) + usize::from(non_empty) {
                    return self.fallback(t);
                }
                let base = match (list, non_empty) {
                    (true, true) => "non-empty-list",
                    (true, false) => "list",
                    (false, true) => "non-empty-array",
                    (false, false) => return self.fallback(t),
                };
                let args = if list || implicit_mixed(key) {
                    if implicit_mixed(item) {
                        return TypeNode::ident(base);
                    }
                    vec![self.node(item)]
                } else {
                    vec![self.node(key), self.node(item)]
                };
                TypeNode::Generic { base: base.to_string(), args }
            }
            Type::Intersection(types) if types.iter().all(|t| matches!(t, Type::Object(..))) => {
                TypeNode::Intersection(sort_types(types).iter().map(|t| self.node(t)).collect())
            }
            _ => self.fallback(t),
        }
    }

    fn object_name(&self, name: &str) -> TypeNode {
        if self.short_objects {
            TypeNode::Identifier(name.rsplit('\\').next().unwrap_or(name).to_string())
        } else {
            TypeNode::Identifier(format!("\\{name}"))
        }
    }

    /// Any other type: PHPStan's printed node, its class names fully qualified.
    fn fallback(&self, t: &Type) -> TypeNode {
        let printed = to_phpdoc(&if self.short_objects { t.clone() } else { fully_qualified(t) });
        pun_phpstan::phpdoc::parse(&printed)
            .ok()
            .and_then(|parsed| TypeNode::from_parsed(&parsed))
            .unwrap_or(TypeNode::Raw(printed))
    }
}

/// `StaticTypeMapper::styleTypeNode()`: a union's styled members flattened (repeats dropped), `array` as `mixed[]`,
/// and an `array<A>` or `array<int, A>` of items only as `A[]` (`A[]|B[]` for a union of items).
fn style(node: TypeNode) -> TypeNode {
    match node {
        TypeNode::Union { types, wrapped } => {
            let mut styled: Vec<TypeNode> = Vec::new();
            for t in types {
                let members = match style(t) {
                    TypeNode::Union { types, .. } => types,
                    single => vec![single],
                };
                for member in members {
                    let printed = member.to_string();
                    match styled.iter_mut().find(|s| s.to_string() == printed) {
                        Some(existing) => *existing = member,
                        None => styled.push(member),
                    }
                }
            }
            TypeNode::Union { types: styled, wrapped }
        }
        TypeNode::Intersection(types) => TypeNode::Intersection(types.into_iter().map(style).collect()),
        TypeNode::Identifier(name) if name == "array" => TypeNode::Array {
            inner: Box::new(TypeNode::ident("mixed")),
            generic_parent: false,
        },
        TypeNode::Generic { base, args } => {
            let args: Vec<TypeNode> = args.into_iter().map(style).collect();
            if base == "array" && is_array_of_items_only(&args) {
                let array_of = |inner: TypeNode| TypeNode::Array {
                    inner: Box::new(inner),
                    generic_parent: false,
                };
                return match args.into_iter().last().expect("an item type") {
                    TypeNode::Union { types, .. } => TypeNode::Union {
                        types: types.into_iter().map(array_of).collect(),
                        wrapped: false,
                    },
                    item => array_of(item),
                };
            }
            TypeNode::Generic { base, args }
        }
        other => other,
    }
}

/// `isArrayOfItemsOnly()`: "array<A>" or "array<int, A>" can be printed as "A[]", unless that would lose the int key
/// of nested arrays, or turn "array<A<X>|A<Y>>" into "A<X>[]|A<Y>[]".
fn is_array_of_items_only(args: &[TypeNode]) -> bool {
    if args.len() == 2 && args[0].to_string() != "int" {
        return false;
    }
    let item = args.last().expect("an item type");
    let items: Vec<&TypeNode> = match item {
        TypeNode::Union { types, .. } => types.iter().collect(),
        single => vec![single],
    };
    items.iter().all(|single| {
        !(matches!(single, TypeNode::Generic { .. }) && items.len() > 1)
            && !(args.len() == 2 && matches!(single, TypeNode::Array { .. } | TypeNode::Generic { .. }))
    })
}

/// Class names fully qualified, for PHPStan's printer (`FullyQualifiedNameNode`).
fn fully_qualified(t: &Type) -> Type {
    match t {
        Type::Object(name, args, generic) => {
            Type::Object(format!("\\{name}"), args.iter().map(fully_qualified).collect(), *generic)
        }
        // `EnumCaseObjectType::toPhpDocNode()`: a `ConstFetchNode` of the fully qualified class (PHPStan 2.3)
        Type::EnumCase(name, case) => Type::EnumCase(format!("\\{name}"), case.clone()),
        t => t.traverse(&fully_qualified),
    }
}

/// `TypeNormalizer::generalizeConstantTypes()`.
pub fn generalize_constant_types(t: &Type) -> Type {
    let deep = Cell::new(0);
    generalize_step(t, &deep)
}

/// The callback `generalizeConstantTypes()` gives `TypeTraverser::map()`; `deep` counts its calls.
fn generalize_step(t: &Type, deep: &Cell<u32>) -> Type {
    deep.set(deep.get() + 1);
    let traverse = |t: &Type| t.traverse(&|inner| generalize_step(inner, deep));
    match t {
        Type::Accessory(
            Accessory::NonFalsyString | Accessory::LiteralString | Accessory::NonEmptyString,
        ) => Type::String,
        Type::ConstBool(_) => Type::Bool,
        Type::ConstString(_) => Type::String,
        Type::ConstFloat(_) => Type::Float,
        Type::ConstInt(_) => Type::Int,
        Type::ConstArray(a) => {
            // is relevant int constantArrayType?
            let key = if is_implicit_numbered_list_key_type(a) {
                if deep.get() == 1 { mixed() } else { Type::Int }
            } else {
                generalize_constant_types(&const_array_key_type(a))
            };
            let mut item = traverse(&const_array_item_type(a));
            if matches!(item, Type::ConstString(_)) {
                item = Type::String;
            }
            if matches!(item, Type::ConstArray(_)) {
                item = generalize_constant_types(&item);
            }
            Type::array(key, item)
        }
        Type::Union(types) | Type::Benevolent(types) => generalize_union(types),
        t => {
            let converted = traverse(t);
            if matches!(converted, Type::Never(_)) {
                return mixed();
            }
            converted
        }
    }
}

const MAX_PRINTED_UNION_DOC_LENGTH: usize = 77;

/// `generalizeConstantTypes()` of a `UnionType`.
fn generalize_union(types: &[Type]) -> Type {
    let mut generalized: Vec<Type> = Vec::new();
    'types: for unioned in types {
        let g = generalize_constant_types(unioned);
        if let Type::Array(key, item) = &g {
            let key_hash = type_hash(key);
            for existing in generalized.iter_mut() {
                let Type::Array(existing_key, existing_item) = &*existing else {
                    continue;
                };
                if type_hash(existing_key) != key_hash {
                    continue;
                }
                let unique = uniquate_types(&[(**existing_item).clone(), (**item).clone()]);
                if unique.len() != 1 {
                    continue;
                }
                *existing = Type::array((**existing_key).clone(), unique[0].clone());
                continue 'types;
            }
        }
        generalized.push(g);
    }
    let mut unique = uniquate_types(&generalized);
    if unique.len() > 1 {
        let union = Type::Union(unique.clone());
        let short = style(Mapper { short_objects: true }.node(&plain_integers(&union))).to_string();
        if short.len() > MAX_PRINTED_UNION_DOC_LENGTH && !avoid_printed_docblock_trimming(&union, &unique) {
            if let Some(array) = narrow_to_always_known_array_type(&unique) {
                return array;
            }
            return mixed();
        }
        return union;
    }
    unique.remove(0)
}

/// `isImplicitNumberedListKeyType()`: the keys are a union of `0`, `1`, ... in order.
fn is_implicit_numbered_list_key_type(a: &ConstArray) -> bool {
    a.keys.len() > 1
        && a.keys
            .iter()
            .enumerate()
            .all(|(i, k)| *k == Type::ConstInt(i as i64))
}

/// `narrowToAlwaysKnownArrayType()`.
fn narrow_to_always_known_array_type(types: &[Type]) -> Option<Type> {
    if !types.iter().all(|t| t.is_array().yes()) {
        return None;
    }
    match shared_array_structure(types) {
        shared @ Type::Array(..) => Some(shared),
        _ => None,
    }
}

/// `ArrayTypeLeastCommonDenominatorResolver::sharedArrayStructure()`.
fn shared_array_structure(types: &[Type]) -> Type {
    let mut arrays: Vec<(&Type, &Type)> = Vec::new();
    for t in types {
        let Type::Array(key, item) = t else {
            return mixed();
        };
        arrays.push((key, item));
    }
    // (never empty: a union's arrays, or their items)
    let ((first_key, _), rest) = arrays.split_first().expect("an array");
    let mut key = (*first_key).clone();
    for (other, _) in rest {
        key = intersect(vec![key, (*other).clone()]);
    }
    if matches!(key, Type::Never(_)) {
        key = mixed();
    }
    let items: Vec<Type> = arrays.iter().map(|(_, item)| (*item).clone()).collect();
    Type::array(key, shared_array_structure(&items))
}

/// `avoidPrintedDocblockTrimming()`: a union of objects only (no constant scalars or arrays).
fn avoid_printed_docblock_trimming(union: &Type, types: &[Type]) -> bool {
    if types.iter().any(|t| {
        matches!(
            t,
            Type::ConstBool(_)
                | Type::ConstInt(_)
                | Type::ConstFloat(_)
                | Type::ConstString(_)
                | Type::Null
                | Type::ConstArray(_)
        )
    }) {
        return false;
    }
    !union.object_class_names().is_empty()
}

/// `TypeFactory::createMixedPassedOrUnionType()` (`createMixedPassedOrUnionTypeAndKeepConstant()` with
/// `keep_constant`): the types unwrapped and unique, as a `UnionType` of them as they are (not
/// `TypeCombinator::union()`); `mixed` when there is none or any is `mixed`.
pub fn create_mixed_passed_or_union_type(types: &[Type], keep_constant: bool) -> Type {
    let mut unwrapped = Vec::new();
    for t in types.iter().flat_map(flatten_types) {
        match t {
            Type::ConstArray(a) => unwrapped.extend(unwrap_constant_array_types(&a)),
            t => unwrapped.push(t),
        }
    }
    let mut types = uniquate(&unwrapped, keep_constant);
    // `createUnionOrSingleType()`
    match types.len() {
        0 => mixed(),
        1 => types.remove(0),
        _ if types.iter().any(is_mixed) => mixed(),
        _ => Type::Union(types),
    }
}

/// `TypeUtils::flattenTypes()`: a union's members, a constant array as each of its arrays.
fn flatten_types(t: &Type) -> Vec<Type> {
    match t {
        Type::Union(types) | Type::Benevolent(types) => types.iter().flat_map(flatten_types).collect(),
        // too many variants: the array itself
        Type::ConstArray(a) if a.optional.len() > 14 => vec![t.clone()],
        Type::ConstArray(a) => all_arrays(a),
        t => vec![t.clone()],
    }
}

/// `ConstantArrayType::getAllArrays()`: the array with each combination of its optional keys present.
fn all_arrays(a: &ConstArray) -> Vec<Type> {
    let optional = &a.optional;
    let n = optional.len();
    let combinations: Vec<Vec<usize>> = if n <= 10 {
        // `powerSet()`
        (0..1usize << n)
            .map(|i| (0..n).filter(|j| (i >> (n - 1 - j)) & 1 == 1).map(|j| optional[j]).collect())
            .collect()
    } else {
        vec![vec![], vec![optional[0]], vec![optional[n - 1]], optional.clone()]
    };
    let required: Vec<usize> = (0..a.keys.len()).filter(|i| !optional.contains(i)).collect();
    let mut arrays = Vec::new();
    for combination in combinations {
        let mut keys: Vec<usize> = required.iter().chain(&combination).copied().collect();
        keys.sort_unstable();
        // (a list's optional keys are its last ones, so every combination is a list)
        let mut builder = ShapeBuilder::new();
        for i in keys {
            builder.set(Some(a.keys[i].clone()), a.values[i].clone(), false);
        }
        arrays.push(builder.get());
    }
    arrays
}

/// `unwrapConstantArrayTypes()`: an `array<key, item>` for each item type, by position.
fn unwrap_constant_array_types(a: &ConstArray) -> Vec<Type> {
    let keys = flatten_types(&const_array_key_type(a));
    let mut unwrapped = Vec::new();
    for (position, item) in flatten_types(&const_array_item_type(a)).into_iter().enumerate() {
        let key = keys.get(position).cloned().unwrap_or_else(mixed);
        match item {
            Type::ConstArray(inner) => {
                for inner in unwrap_constant_array_types(&inner) {
                    // preserve outer array -> inner array structure: array<outerKey, innerArray>
                    unwrapped.push(Type::array(key.clone(), inner));
                }
            }
            item => unwrapped.push(Type::array(key, item)),
        }
    }
    unwrapped
}

/// `TypeFactory::uniquateTypes()` (constants not kept): the types with their values removed, the later of two with
/// the same hash in the place of the first.
pub fn uniquate_types(types: &[Type]) -> Vec<Type> {
    uniquate(types, false)
}

/// `TypeFactory::uniquateTypes()`; with `keep_constant`, a constant is kept unless its type without the value is
/// there too.
fn uniquate(types: &[Type], keep_constant: bool) -> Vec<Type> {
    let (mut has_false, mut has_true) = (false, false);
    let mut unique: Vec<(String, Type)> = Vec::new();
    let mut constant_hashes: Vec<(String, String)> = Vec::new();
    for t in types {
        // `normalizeBooleanType()`
        let mut t = t.clone();
        if t == Type::ConstBool(true) {
            has_true = true;
        }
        if t == Type::ConstBool(false) {
            has_false = true;
        }
        if has_false && has_true && matches!(t, Type::ConstBool(_)) {
            t = Type::Bool;
        }
        // `removeValueFromConstantType()`
        let t_before = t.clone();
        let removed = match t {
            Type::ConstFloat(_) => Type::Float,
            Type::ConstString(_) => Type::String,
            Type::ConstInt(_) => Type::Int,
            Type::ConstBool(_) => Type::Bool,
            t => t,
        };
        let removed_hash = type_hash(&removed);
        let (hash, t) = if keep_constant && removed != t_before {
            let hash = type_hash(&t_before);
            constant_hashes.push((hash.clone(), removed_hash));
            (hash, t_before)
        } else {
            (removed_hash, removed)
        };
        match unique.iter_mut().find(|(h, _)| *h == hash) {
            Some(slot) => slot.1 = t,
            None => unique.push((hash, t)),
        }
    }
    for (constant, removed) in &constant_hashes {
        if unique.iter().any(|(h, _)| h == removed) {
            unique.retain(|(h, _)| h != constant);
        }
    }
    unique.into_iter().map(|(_, t)| t).collect()
}

/// `TypeHasher::createTypeHash()`.
pub fn type_hash(t: &Type) -> String {
    match t {
        Type::Mixed { explicit } | Type::SubtractedMixed { explicit, .. } => {
            format!(
                "{}{}",
                t.describe(Level::Precise),
                if *explicit { "1" } else { "" }
            )
        }
        Type::Error => t.describe(Level::Precise),
        Type::Array(_, item) => format!(
            "{}{}{}[]",
            type_hash(&t.iterable_value_type()),
            type_hash(&t.iterable_key_type()),
            item.describe(Level::Precise)
        ),
        Type::Object(_, args, _) if !args.is_empty() => t.describe(Level::Precise),
        Type::Object(name, _, _) | Type::Static(name, _) | Type::This(name) => name.clone(),
        Type::EnumCase(name, _) => name.clone(),
        // `isConstantValue()`: the class name
        Type::ConstBool(_) => "ConstantBooleanType".to_string(),
        Type::ConstInt(_) => "ConstantIntegerType".to_string(),
        Type::ConstFloat(_) => "ConstantFloatType".to_string(),
        Type::ConstString(_) => "ConstantStringType".to_string(),
        Type::ConstArray(_) => "ConstantArrayType".to_string(),
        Type::Null => "NullType".to_string(),
        _ => t.describe(Level::Value),
    }
}

/// `ConstantArrayTypeGeneralizer`: a constant array as `array<key, item>` (an empty one as `array{}`), three
/// levels deep at most.
pub fn generalize_constant_array(a: &ConstArray) -> TypeNode {
    let nesting = Cell::new(0);
    generalize_nested(a, true, &nesting)
}

const MAX_NESTING: u32 = 3;

fn generalize_nested(a: &ConstArray, fresh: bool, nesting: &Cell<u32>) -> TypeNode {
    if fresh {
        nesting.set(0);
    } else {
        nesting.set(nesting.get() + 1);
    }
    let key = generalize_constant_types(&const_array_key_type(a));
    let item = const_array_item_type(a);
    if matches!(item, Type::Never(_)) {
        return TypeNode::Shape {
            kind: "array".to_string(),
            items: vec![],
            sealed: true,
            unsealed: String::new(),
        };
    }
    let item_node = match &item {
        Type::ConstArray(_) if nesting.get() >= MAX_NESTING => map_to_phpdoc(&mixed()),
        Type::ConstArray(inner) => generalize_nested(inner, false, nesting),
        item => {
            let g = generalize_constant_types(item);
            // correction
            map_to_phpdoc(if matches!(g, Type::Never(_)) { &Type::Mixed { explicit: false } } else { &g })
        }
    };
    TypeNode::Generic {
        base: "array".to_string(),
        args: vec![map_to_phpdoc(&key), item_node],
    }
}

/// `NodeDocblockTypeDecorator::createTypeNode()`: the generalized type's node, `mixed[]` for `mixed`.
pub fn create_type_node(t: &Type) -> TypeNode {
    // empty array default "[]" resolves to "never[]"; widen to "mixed[]" to keep the docblock useful
    let node = map_to_phpdoc(&never_to_mixed(&generalize_constant_types(t)));
    if node == TypeNode::ident("mixed") {
        return TypeNode::Array {
            inner: Box::new(node),
            generic_parent: false,
        };
    }
    node
}

/// `TypeTraverser::map()` of each `NeverType` to `MixedType`.
fn never_to_mixed(t: &Type) -> Type {
    match t {
        Type::Never(_) => Type::Mixed { explicit: false },
        // `UnionType::traverse()` normalizes the union again only when a member changed
        Type::Union(types) | Type::Benevolent(types)
            if types.iter().all(|m| never_to_mixed(m) == *m) =>
        {
            t.clone()
        }
        t => t.traverse(&never_to_mixed),
    }
}

/// `NodeDocblockTypeDecorator::isBareMixedType()`: `mixed`, or an array of nothing or of `mixed` by int or `mixed`
/// keys.
pub fn is_bare_mixed_type(t: &Type) -> bool {
    if is_mixed(t) {
        return true;
    }
    match generalize_constant_types(t) {
        Type::Array(key, item) => {
            matches!(*item, Type::Never(_)) || (is_mixed(&item) && (is_integer_type(&key) || is_mixed(&key)))
        }
        _ => false,
    }
}

/// `UsefulArrayTagNodeAnalyzer::isUsefulArrayTag()` of a tag's type (`None`: no tag).
pub fn is_useful_array_tag(t: Option<&TypeNode>) -> bool {
    match t {
        None => false,
        Some(TypeNode::Identifier(name)) => !matches!(name.as_str(), "array" | "mixed" | "iterable"),
        Some(t) => !is_mixed_array(t),
    }
}

/// `UsefulArrayTagNodeAnalyzer::isMixedArray()`: `mixed[]`.
pub fn is_mixed_array(t: &TypeNode) -> bool {
    matches!(t, TypeNode::Array { inner, .. } if **inner == TypeNode::ident("mixed"))
}
