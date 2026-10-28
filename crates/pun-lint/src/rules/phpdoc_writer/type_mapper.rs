//! From a PHPStan type to the phpdoc type node Rector writes: `PHPStanStaticTypeMapper::mapToPHPStanPhpDocTypeNode()`
//! with its type mappers (`ArrayTypeMapper`, `UnionTypeMapper`, `ObjectTypeMapper`, ...), and the steps before it
//! that the docblock rules share: `TypeNormalizer::generalizeConstantTypes()`, `TypeFactory::uniquateTypes()`,
//! `TypeHasher`, `ConstantArrayTypeGeneralizer` and `NodeDocblockTypeDecorator`'s checks.
//!
//! The types are pun-phpstan's port of PHPStan's (`TypeCombinator::union()` decides the order of a union's members,
//! which the printed type keeps).

use std::cell::Cell;

use pun_phpstan::types::{Accessory, ConstArray, Level, Type, intersect, mixed, to_phpdoc};

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

fn is_union(t: &Type) -> bool {
    matches!(t, Type::Union(_) | Type::Benevolent(_))
}

fn union_members(t: &Type) -> &[Type] {
    match t {
        Type::Union(types) | Type::Benevolent(types) => types,
        _ => std::slice::from_ref(t),
    }
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
            Type::Reasoned(t, _) => self.map(t),
            Type::Array(key, item) => self.array(key, item),
            Type::Bool => TypeNode::ident("bool"),
            Type::ConstBool(b) => TypeNode::ident(if *b { "true" } else { "false" }),
            Type::ClassString(Some(inner)) => TypeNode::Generic {
                base: "class-string".to_string(),
                args: vec![match &**inner {
                    Type::Object(name, args) if args.is_empty() => self.object_name(name),
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
            Type::Object(name, args) if args.is_empty() => self.object_name(name),
            Type::Object(name, args) => TypeNode::Generic {
                base: format!("\\{name}"),
                args: args
                    .iter()
                    .map(|a| TypeNode::Raw(to_phpdoc(&fully_qualified(a))))
                    .collect(),
            },
            Type::ObjectWithoutClass => TypeNode::ident("object"),
            Type::String => TypeNode::ident("string"),
            Type::Union(types) | Type::Benevolent(types) => self.union(types),
            Type::Void => TypeNode::ident("void"),
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
        for t in types {
            let t = match t {
                Type::Array(key, item) if matches!(**item, Type::Never(_)) => {
                    Type::array((**key).clone(), mixed())
                }
                t => t.clone(),
            };
            let mut node = self.map(&t);
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
        if is_union(item) && !is_generic {
            return self.array_from_union(union_members(item));
        }
        if let Type::Array(inner_key, inner_item) = item
            && is_generic_array_candidate(inner_key, inner_item)
        {
            return self.generic_array(key, item, true);
        }
        if is_generic {
            return self.generic_array(key, item, true);
        }
        // keep the `int` key of `array<int, mixed>`
        if is_integer_type(key) && !Type::array(key.clone(), item.clone()).is_list().maybe() {
            return TypeNode::Generic {
                base: "array".to_string(),
                args: vec![self.map(key), self.map(item)],
            };
        }
        TypeNode::Array {
            inner: Box::new(self.map(item)),
            generic_parent: false,
        }
    }

    /// `createArrayTypeNodeFromUnionType()`.
    fn array_from_union(&self, types: &[Type]) -> TypeNode {
        let mut nodes: Vec<(String, TypeNode)> = Vec::new();
        for t in types {
            let node = self.map(t);
            let printed = node.to_string();
            match nodes.iter_mut().find(|(p, _)| *p == printed) {
                Some(slot) => slot.1 = node,
                None => nodes.push((printed, node)),
            }
        }
        let mut nodes: Vec<TypeNode> = nodes.into_iter().map(|(_, n)| n).collect();
        let inner = if nodes.len() > 1 {
            TypeNode::Union {
                types: nodes,
                wrapped: false,
            }
        } else {
            nodes.remove(0)
        };
        TypeNode::Array {
            inner: Box::new(inner),
            generic_parent: false,
        }
    }

    /// `createGenericArrayType()`.
    fn generic_array(&self, key: &Type, item: &Type, with_key: bool) -> TypeNode {
        let item_node = self.map(item);
        let with_key = with_key && !is_class_string_array(key, item);
        let mut args = if with_key {
            let key_node = self.map(key);
            if matches!(item_node, TypeNode::Union { .. }) && is_pair_class_too_detailed(item) {
                vec![key_node, self.map(&Type::ClassString(None))]
            } else {
                vec![key_node, item_node]
            }
        } else {
            vec![item_node]
        };
        for arg in &mut args {
            if let TypeNode::Array { generic_parent, .. } = arg {
                *generic_parent = with_key;
            }
        }
        TypeNode::Generic {
            base: "array".to_string(),
            args,
        }
    }
}

/// `ObjectTypeMapper`'s traversal for a generic object's arguments: classes fully qualified.
fn fully_qualified(t: &Type) -> Type {
    match t {
        Type::Object(name, args) if args.is_empty() => Type::Object(format!("\\{name}"), vec![]),
        t => t.traverse(&fully_qualified),
    }
}

/// `ArrayTypeMapper::isGenericArrayCandidate()`.
fn is_generic_array_candidate(key: &Type, item: &Type) -> bool {
    if is_mixed(key) {
        return false;
    }
    if is_class_string_array(key, item) {
        return true;
    }
    // skip simple arrays, like "string[]", from converting to obvious "array<int, string>"
    if key.is_integer().yes() && !item.is_array().yes() {
        return false;
    }
    // a general array's keys are never a constant array
    !matches!(key, Type::Never(_))
}

/// `isClassStringArrayType()`.
fn is_class_string_array(key: &Type, item: &Type) -> bool {
    (is_mixed(key) || matches!(key, Type::ConstInt(_))) && matches!(item, Type::ClassString(Some(_)))
}

/// `isPairClassTooDetailed()`: a union of more than three generic class strings.
fn is_pair_class_too_detailed(item: &Type) -> bool {
    match item {
        Type::Union(types) | Type::Benevolent(types) => {
            types.iter().all(|t| matches!(t, Type::ClassString(Some(_)))) && types.len() > 3
        }
        _ => false,
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
        let union = Type::Union(unique);
        let short = Mapper {
            short_objects: true,
        }
        .map(&union)
        .to_string();
        if short.len() > MAX_PRINTED_UNION_DOC_LENGTH && !avoid_printed_docblock_trimming(&union) {
            if let Some(array) = narrow_to_always_known_array_type(&union) {
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
fn narrow_to_always_known_array_type(union: &Type) -> Option<Type> {
    let types = union_members(union);
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
    let Some(((first_key, _), rest)) = arrays.split_first() else {
        return mixed();
    };
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
fn avoid_printed_docblock_trimming(union: &Type) -> bool {
    let types = union_members(union);
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

/// `TypeFactory::uniquateTypes()` (constants not kept): the types with their values removed, the later of two with
/// the same hash in the place of the first.
pub fn uniquate_types(types: &[Type]) -> Vec<Type> {
    let (mut has_false, mut has_true) = (false, false);
    let mut unique: Vec<(String, Type)> = Vec::new();
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
        let removed = match t {
            Type::ConstFloat(_) => Type::Float,
            Type::ConstString(_) => Type::String,
            Type::ConstInt(_) => Type::Int,
            Type::ConstBool(_) => Type::Bool,
            t => t,
        };
        let hash = type_hash(&removed);
        match unique.iter_mut().find(|(h, _)| *h == hash) {
            Some(slot) => slot.1 = removed,
            None => unique.push((hash, removed)),
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
        Type::Object(_, args) if !args.is_empty() => t.describe(Level::Precise),
        Type::Object(name, _) | Type::Static(name, _) | Type::This(name) => name.clone(),
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
