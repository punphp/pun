//! From a PHPStan type to the phpdoc type node Rector writes: `PHPStanStaticTypeMapper::mapToPHPStanPhpDocTypeNode()`
//! with its type mappers (`ArrayTypeMapper`, `UnionTypeMapper`, `ObjectTypeMapper`, ...), and the steps before it
//! that the docblock rules share: `TypeNormalizer::generalizeConstantTypes()`, `TypeFactory::uniquateTypes()`,
//! `TypeHasher`, `ConstantArrayTypeGeneralizer` and `NodeDocblockTypeDecorator`'s checks.
//!
//! The types are pun-phpstan's port of PHPStan's (`TypeCombinator::union()` decides the order of a union's members,
//! which the printed type keeps).

use std::cell::Cell;

use pun_phpstan::types::{Accessory, ConstArray, Level, ShapeBuilder, Type, intersect, mixed, to_phpdoc};

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

/// `PHPStanStaticTypeMapper::mapToPHPStanPhpDocTypeNode()`.
pub fn map_to_phpdoc(t: &Type) -> TypeNode {
    Mapper { short_objects: false }.map(t)
}

struct Mapper {
    /// Objects as `ShortenedObjectType`s, printed by their short name (`TypeNormalizer::resolveNameShortDocTypeNode`).
    short_objects: bool,
}

impl Mapper {
    fn map(&self, t: &Type) -> TypeNode {
        match t {
            Type::Array(key, item) => self.array(key, item),
            Type::Bool => TypeNode::ident("bool"),
            Type::ConstBool(b) => TypeNode::ident(if *b { "true" } else { "false" }),
            Type::ClassString(Some(inner)) => TypeNode::Generic {
                base: "class-string".to_string(),
                args: vec![match &**inner {
                    Type::Object(name, args, _) if args.is_empty() => self.object_name(name),
                    other => TypeNode::Raw(to_phpdoc(other)),
                }],
            },
            Type::ClassString(None) => TypeNode::ident("class-string"),
            Type::Float => TypeNode::ident("float"),
            Type::Int | Type::ConstInt(_) | Type::IntRange(..) => TypeNode::ident("int"),
            Type::Mixed { .. } | Type::SubtractedMixed { .. } | Type::Error => {
                TypeNode::ident("mixed")
            }
            Type::Never(_) => TypeNode::ident("never"),
            Type::Null => TypeNode::ident("null"),
            Type::Object(name, args, _) if args.is_empty() => self.object_name(name),
            Type::Object(name, args, _) => TypeNode::Generic {
                base: format!("\\{name}"),
                args: args
                    .iter()
                    .map(|a| TypeNode::Raw(to_phpdoc(&fully_qualified(a))))
                    .collect(),
            },
            Type::ObjectWithoutClass => TypeNode::ident("object"),
            Type::String => TypeNode::ident("string"),
            Type::Union(types) | Type::Benevolent(types) => self.union(types),
            // `IntersectionTypeMapper` of objects: `IntersectionTypeNode`
            Type::Intersection(types) if types.iter().all(|t| matches!(t, Type::Object(..))) => TypeNode::Raw(format!(
                "({})",
                types.iter().map(|t| self.map(t).to_string()).collect::<Vec<_>>().join(" & ")
            )),
            Type::Void => TypeNode::ident("void"),
            Type::Intersection(_) => intersection(t),
            other => TypeNode::Raw(to_phpdoc(other)),
        }
    }

    /// `ObjectTypeMapper`: a plain `ObjectType` gets a leading backslash; a `ShortenedObjectType` prints its short
    /// name.
    fn object_name(&self, name: &str) -> TypeNode {
        if self.short_objects {
            TypeNode::Identifier(name.rsplit('\\').next().unwrap_or(name).to_string())
        } else {
            TypeNode::Identifier(format!("\\{name}"))
        }
    }

    /// `UnionTypeMapper::mapToPHPStanPhpDocTypeNode()`.
    fn union(&self, types: &[Type]) -> TypeNode {
        let mut nodes = Vec::new();
        let mut existing: Vec<String> = Vec::new();
        // (an array of `never` was generalized to an array of `mixed` before)
        for t in types {
            let mut node = self.map(t);
            if let TypeNode::Array { inner, .. } = &mut node
                && let TypeNode::Union { types: inner, .. } = &mut **inner
            {
                inner.retain(|n| {
                    let printed = n.to_string();
                    if existing.contains(&printed) {
                        return false;
                    }
                    existing.push(printed);
                    true
                });
                if inner.is_empty() {
                    continue;
                }
            }
            nodes.push(node);
        }
        TypeNode::Union {
            types: nodes,
            wrapped: false,
        }
    }

    /// `ArrayTypeMapper::mapToPHPStanPhpDocTypeNode()`.
    fn array(&self, key: &Type, item: &Type) -> TypeNode {
        let is_generic = is_generic_array_candidate(key, item);
        if let Type::Union(types) | Type::Benevolent(types) = item
            && !is_generic
        {
            return self.array_from_union(types);
        }
        if let Type::Array(inner_key, inner_item) = item
            && is_generic_array_candidate(inner_key, inner_item)
        {
            return self.generic_array(key, item);
        }
        if is_generic {
            return self.generic_array(key, item);
        }
        // (`array<int, mixed>` keeps its `int` key only when it is no maybe-list, which a general array by integer keys
        // always is)
        TypeNode::Array {
            inner: Box::new(self.map(item)),
            generic_parent: false,
        }
    }

    /// `createArrayTypeNodeFromUnionType()`: each member's node once (the members of a union print apart).
    fn array_from_union(&self, types: &[Type]) -> TypeNode {
        let mut nodes: Vec<TypeNode> = Vec::new();
        for node in types.iter().map(|t| self.map(t)) {
            if !nodes.contains(&node) {
                nodes.push(node);
            }
        }
        TypeNode::Array {
            inner: Box::new(TypeNode::Union {
                types: nodes,
                wrapped: false,
            }),
            generic_parent: false,
        }
    }

    /// `createGenericArrayType()` with the key (an array of class strings by a constant integer key, printed
    /// without, is never mapped: the types are generalized first).
    fn generic_array(&self, key: &Type, item: &Type) -> TypeNode {
        let item_node = self.map(item);
        let key_node = self.map(key);
        let mut args = if matches!(item_node, TypeNode::Union { .. }) && is_pair_class_too_detailed(item) {
            vec![key_node, self.map(&Type::ClassString(None))]
        } else {
            vec![key_node, item_node]
        };
        for arg in &mut args {
            if let TypeNode::Array { generic_parent, .. } = arg {
                *generic_parent = true;
            }
        }
        TypeNode::Generic {
            base: "array".to_string(),
            args,
        }
    }
}

/// `IntersectionTypeMapper::mapToPHPStanPhpDocTypeNode()`: PHPStan's own node (`list<T>`, `non-empty-array<K, V>`),
/// the names among the root's direct children fully qualified unless they are scalar, array or explicit `mixed`
/// types (the traversal goes no deeper than a node that is not a name).
fn intersection(t: &Type) -> TypeNode {
    let printed = to_phpdoc(t);
    let Some(mut node) = pun_phpstan::phpdoc::parse(&printed)
        .ok()
        .and_then(|parsed| TypeNode::from_parsed(&parsed))
    else {
        return TypeNode::Raw(printed);
    };
    if let TypeNode::Generic { args: children, .. } | TypeNode::Intersection(children) = &mut node {
        children.iter_mut().for_each(qualify);
    }
    node
}

/// A name the intersection's traversal prefixes with `\`: no scalar, array or explicit `mixed` type.
fn qualify(node: &mut TypeNode) {
    let TypeNode::Identifier(name) = node else {
        return;
    };
    let t = crate::rules::types::compare::mapper::scalar_string_to_type(name);
    let scalar = Type::Union(vec![Type::Int, Type::Float, Type::String, Type::Bool]);
    let kept = matches!(t, Type::Accessory(_) | Type::Mixed { explicit: true })
        || pun_phpstan::types::is_super_type_of(&scalar, &t).yes()
        || t.is_array().yes();
    if !kept {
        *name = format!("\\{}", name.trim_start_matches('\\'));
    }
}

/// `ObjectTypeMapper`'s traversal for a generic object's arguments: classes fully qualified.
fn fully_qualified(t: &Type) -> Type {
    match t {
        Type::Object(name, args, _) if args.is_empty() => Type::Object(format!("\\{name}"), vec![], false),
        t => t.traverse(&fully_qualified),
    }
}

/// `ArrayTypeMapper::isGenericArrayCandidate()`.
fn is_generic_array_candidate(key: &Type, item: &Type) -> bool {
    if is_mixed(key) {
        return false;
    }
    // (`isClassStringArrayType()` needs a constant integer key, which a generalized type never has)
    // skip simple arrays, like "string[]", from converting to obvious "array<int, string>"
    if key.is_integer().yes() && !item.is_array().yes() {
        return false;
    }
    // a general array's keys are never a constant array
    !matches!(key, Type::Never(_))
}

/// `isPairClassTooDetailed()`: a union of more than three generic class strings.
fn is_pair_class_too_detailed(item: &Type) -> bool {
    matches!(item, Type::Union(types) | Type::Benevolent(types)
        if types.iter().all(|t| matches!(t, Type::ClassString(Some(_)))) && types.len() > 3)
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
        let short = Mapper {
            short_objects: true,
        }
        .map(&union)
        .to_string();
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
    Some(Type::array(shared_array_structure(types), mixed()))
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
    let node = map_to_phpdoc(&generalize_constant_types(t));
    if node == TypeNode::ident("mixed") {
        return TypeNode::Array {
            inner: Box::new(node),
            generic_parent: false,
        };
    }
    node
}

/// `NodeDocblockTypeDecorator::isBareMixedType()`: `mixed`, or an array of nothing or of `mixed` by int keys.
pub fn is_bare_mixed_type(t: &Type) -> bool {
    if is_mixed(t) {
        return true;
    }
    match generalize_constant_types(t) {
        Type::Array(key, item) => {
            matches!(*item, Type::Never(_)) || (is_mixed(&item) && is_integer_type(&key))
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
