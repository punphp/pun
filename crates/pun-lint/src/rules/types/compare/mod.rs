//! Rector 2.6.1's type comparison, for the rules that ask whether a PHPDoc type says more than the native type:
//! `TypeComparator` (src/NodeTypeResolver/TypeComparator), its `ScalarTypeComparator`, `TypeHasher`
//! (src/NodeTypeResolver/PHPStan/TypeHasher.php), `TypeFactory::createMixedPassedOrUnionType[AndKeepConstant]` and the
//! `StaticTypeMapper` that turns a native type declaration or a PHPDoc type node into a PHPStan type ([`mapper`]).
//!
//! A type is an [`RType`]: a PHPStan type of pun-phpstan's model, or one of the `ObjectType` subclasses Rector's mappers
//! build (`FullyQualifiedObjectType`, `ShortenedObjectType`, `AliasedObjectType`, `NonExistingObjectType`, ...), whose
//! `equals()` and hash differ from a plain `ObjectType`'s, or a union or intersection of them that Rector built itself
//! (`new UnionType($types)`, in the order given, unlike `TypeCombinator::union()`). PHPStan's three-valued
//! `isSuperTypeOf()` is pun-phpstan's, run with the project's classes known ([`classes::with_classes`]).

pub(crate) mod classes;
pub(crate) mod mapper;
pub(crate) mod tags;

use pun_phpstan::types::{Accessory, Level, Tri, Type, is_super_type_of, mixed, union};

/// A PHPStan type as Rector's type mappers build it.
#[derive(Clone, Debug)]
pub(crate) enum RType {
    /// A type of PHPStan's own model (an `ObjectType` here is a plain one, as `TypeNodeResolver` or the scope gives).
    T(Type),
    /// One of Rector's `ObjectType` (or `StaticType`) subclasses.
    Object(Object),
    /// `new UnionType($types)`, its members in this order.
    Union(Vec<RType>),
    /// `new IntersectionType($types)`.
    Intersection(Vec<RType>),
}

/// Rector's object types (src/StaticTypeMapper/ValueObject/Type).
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Object {
    FullyQualified(String),
    /// A name an import shortens: the name as written and the full one.
    Shortened {
        short: String,
        full: String,
    },
    /// A name an import aliases: the alias and the full name.
    Aliased {
        alias: String,
        full: String,
    },
    NonExisting(String),
    /// `SelfObjectType`: `self` in a PHPDoc.
    SelfObject(String),
    /// `SelfStaticType`, a `ThisType`: `self` as a native type.
    SelfStatic(String),
    /// `ParentStaticType`, a `StaticType`: `parent` of a class with a parent.
    ParentStatic(String),
    /// `ParentObjectWithoutClassType`: `parent` of a class without one.
    ParentWithoutClass,
}

impl Object {
    /// `getClassName()`.
    fn class_name(&self) -> Option<&str> {
        match self {
            Object::FullyQualified(n)
            | Object::NonExisting(n)
            | Object::SelfObject(n)
            | Object::SelfStatic(n)
            | Object::ParentStatic(n) => Some(n),
            Object::Shortened { short, .. } => Some(short),
            Object::Aliased { alias, .. } => Some(alias),
            Object::ParentWithoutClass => None,
        }
    }

    /// `instanceof ObjectType`.
    fn is_object_type(&self) -> bool {
        !matches!(
            self,
            Object::SelfStatic(_) | Object::ParentStatic(_) | Object::ParentWithoutClass
        )
    }
}

impl RType {
    pub(crate) fn mixed() -> RType {
        RType::T(mixed())
    }

    /// The members of a union (`UnionType`, a benevolent one too), `None` for any other type.
    pub(crate) fn union_members(&self) -> Option<Vec<RType>> {
        match self {
            RType::Union(types) => Some(types.clone()),
            RType::T(Type::Union(types) | Type::Benevolent(types)) => {
                Some(types.iter().cloned().map(RType::T).collect())
            }
            _ => None,
        }
    }

    fn intersection_members(&self) -> Option<Vec<RType>> {
        match self {
            RType::Intersection(types) => Some(types.clone()),
            RType::T(Type::Intersection(types)) => {
                Some(types.iter().cloned().map(RType::T).collect())
            }
            _ => None,
        }
    }

    /// `instanceof UnionType`.
    pub(crate) fn is_union(&self) -> bool {
        self.union_members().is_some()
    }

    /// `instanceof IntersectionType`.
    pub(crate) fn is_intersection(&self) -> bool {
        self.intersection_members().is_some()
    }

    /// `instanceof MixedType`.
    pub(crate) fn is_mixed(&self) -> bool {
        matches!(
            self,
            RType::T(Type::Mixed { .. } | Type::SubtractedMixed { .. })
        )
    }

    /// `instanceof ObjectType`.
    pub(crate) fn is_object_type(&self) -> bool {
        match self {
            RType::Object(o) => o.is_object_type(),
            RType::T(t) => matches!(t, Type::Object(..) | Type::EnumCase(..)),
            _ => false,
        }
    }

    /// `instanceof StaticType` (a `ThisType` too).
    fn is_static_type(&self) -> bool {
        matches!(
            self,
            RType::T(Type::Static(..) | Type::This(_))
                | RType::Object(Object::SelfStatic(_) | Object::ParentStatic(_))
        )
    }

    /// `instanceof ThisType`.
    fn is_this_type(&self) -> bool {
        matches!(
            self,
            RType::T(Type::This(_)) | RType::Object(Object::SelfStatic(_))
        )
    }

    /// `instanceof TemplateType`.
    pub(crate) fn is_template(&self) -> bool {
        matches!(self, RType::T(Type::Template(_)))
    }

    /// `getClassName()` of an object type.
    pub(crate) fn class_name(&self) -> Option<&str> {
        match self {
            RType::Object(o) => o.class_name(),
            RType::T(Type::Object(n, _, _) | Type::Static(n, _) | Type::This(n)) => Some(n),
            _ => None,
        }
    }

    /// The type PHPStan's own methods see on the receiving side of `isSuperTypeOf()` (and in `isString()` and the
    /// like): a `ShortenedObjectType` asks its full class (its override), the others their class name.
    pub(crate) fn to_super(&self) -> Type {
        self.to_phpstan(true)
    }

    /// The type as the argument of `isSuperTypeOf()`: PHPStan reads its `getClassName()`, the short name of a
    /// `ShortenedObjectType`.
    pub(crate) fn to_sub(&self) -> Type {
        self.to_phpstan(false)
    }

    fn to_phpstan(&self, supertype: bool) -> Type {
        match self {
            RType::T(t) => t.clone(),
            RType::Object(o) => match o {
                Object::Shortened { short, full } => {
                    Type::Object(if supertype { full } else { short }.clone(), vec![], false)
                }
                Object::FullyQualified(n) | Object::NonExisting(n) | Object::SelfObject(n) => {
                    Type::Object(n.clone(), vec![], false)
                }
                Object::Aliased { alias, .. } => Type::Object(alias.clone(), vec![], false),
                Object::SelfStatic(n) => Type::This(n.clone()),
                Object::ParentStatic(n) => Type::Static(n.clone(), vec![]),
                Object::ParentWithoutClass => Type::ObjectWithoutClass,
            },
            RType::Union(types) => {
                Type::Union(types.iter().map(|t| t.to_phpstan(supertype)).collect())
            }
            RType::Intersection(types) => {
                Type::Intersection(types.iter().map(|t| t.to_phpstan(supertype)).collect())
            }
        }
    }
}

/// `$a->isSuperTypeOf($b)`.
pub(crate) fn is_super(a: &RType, b: &RType) -> Tri {
    is_super_type_of(&a.to_super(), &b.to_sub())
}

/// `$a->equals($b)`, with the overrides of Rector's object types.
pub(crate) fn equals(a: &RType, b: &RType) -> bool {
    if let (Some(xs), Some(ys)) = (a.union_members(), b.union_members()) {
        // a benevolent union is a class of its own
        let benevolent = |t: &RType| matches!(t, RType::T(Type::Benevolent(_)));
        return benevolent(a) == benevolent(b) && same_members(&xs, &ys);
    }
    if let (Some(xs), Some(ys)) = (a.intersection_members(), b.intersection_members()) {
        return same_members(&xs, &ys);
    }
    match (a, b) {
        (RType::Object(o), _) => object_equals(o, b),
        (RType::T(_), RType::Object(_)) => false,
        (RType::T(x), RType::T(y)) => type_equals(x, y),
        _ => false,
    }
}

/// `UnionType::equals()`: the same number of members, each matching one of the other's.
fn same_members(xs: &[RType], ys: &[RType]) -> bool {
    if xs.len() != ys.len() {
        return false;
    }
    let mut others: Vec<&RType> = ys.iter().collect();
    for x in xs {
        match others.iter().position(|y| equals(x, y)) {
            Some(i) => {
                others.remove(i);
            }
            None => return false,
        }
    }
    others.is_empty()
}

/// pun-phpstan's equality with unions and intersections compared in any order, as PHPStan's `equals()` does.
fn type_equals(a: &Type, b: &Type) -> bool {
    match (a, b) {
        (Type::Union(xs), Type::Union(ys))
        | (Type::Benevolent(xs), Type::Benevolent(ys))
        | (Type::Intersection(xs), Type::Intersection(ys)) => {
            xs.len() == ys.len() && {
                let mut others: Vec<&Type> = ys.iter().collect();
                xs.iter()
                    .all(|x| match others.iter().position(|y| type_equals(x, y)) {
                        Some(i) => {
                            others.remove(i);
                            true
                        }
                        None => false,
                    })
            }
        }
        _ => a == b,
    }
}

/// `ClassNameFromObjectTypeResolver::resolve()`: the one class `getObjectClassNames()` gives.
fn only_class_name(t: &RType) -> Option<String> {
    match t {
        RType::Object(Object::ParentWithoutClass) => None,
        RType::Object(Object::Shortened { short, .. }) => Some(short.clone()),
        RType::Object(o) => o.class_name().map(str::to_string),
        _ => match t.to_sub().object_class_names().as_slice() {
            [one] => Some(one.clone()),
            _ => None,
        },
    }
}

/// The `equals()` of Rector's object types.
fn object_equals(o: &Object, b: &RType) -> bool {
    // `ObjectType::equals()`: the same class and name
    let parent = |b: &RType| match b {
        RType::Object(p) => {
            std::mem::discriminant(o) == std::mem::discriminant(p) && {
                o.class_name() == p.class_name()
            }
        }
        _ => false,
    };
    // `$type instanceof self || $type::class === ObjectType::class`
    let self_or_plain = |b: &RType| match b {
        RType::Object(p) => std::mem::discriminant(o) == std::mem::discriminant(p),
        RType::T(Type::Object(_, args, _)) => args.is_empty(),
        _ => false,
    };
    match o {
        Object::FullyQualified(name) | Object::NonExisting(name) => {
            parent(b) || (self_or_plain(b) && b.class_name() == Some(name))
        }
        Object::Shortened { full, .. } => {
            parent(b) || (self_or_plain(b) && b.class_name() == Some(full))
        }
        Object::Aliased { full, .. } => {
            if let Some(class) = only_class_name(b) {
                if matches!(b, RType::Object(Object::Aliased { full: f, .. }) if f == full) {
                    return true;
                }
                if *full == class {
                    return true;
                }
            }
            parent(b)
        }
        Object::SelfObject(_) | Object::SelfStatic(_) | Object::ParentStatic(_) => parent(b),
        Object::ParentWithoutClass => matches!(
            b,
            RType::Object(Object::ParentWithoutClass) | RType::T(Type::ObjectWithoutClass)
        ),
    }
}

/// `TypeHasher::createTypeHash()`.
pub(crate) fn type_hash(t: &RType) -> String {
    match t {
        RType::T(Type::Mixed { explicit } | Type::SubtractedMixed { explicit, .. }) => {
            let t = t.to_sub();
            format!(
                "{}{}",
                t.describe(Level::Precise),
                if *explicit { "1" } else { "" }
            )
        }
        RType::T(Type::Array(key, value)) => format!(
            "{}{}{}[]",
            type_hash(&RType::T((**value).clone())),
            type_hash(&RType::T((**key).clone())),
            value.describe(Level::Precise)
        ),
        RType::T(Type::Object(_, args, _)) if !args.is_empty() => t.to_sub().describe(Level::Precise),
        RType::Object(Object::Shortened { full, .. } | Object::Aliased { full, .. }) => {
            full.clone()
        }
        RType::Object(o) if o.class_name().is_some() => {
            o.class_name().unwrap_or_default().to_string()
        }
        RType::T(
            Type::Object(n, _, _) | Type::Static(n, _) | Type::This(n) | Type::EnumCase(n, _),
        ) => n.clone(),
        RType::T(Type::Closure(_)) => "Closure".to_string(),
        _ if is_constant_value(t) => php_class(t).to_string(),
        _ => normalized(t).describe(Level::Value),
    }
}

/// `TypeHasher::normalizeObjectType()`: each object type as a fully qualified one; a union with one changed becomes
/// `TypeCombinator::union()` of its members.
fn normalized(t: &RType) -> Type {
    fn plain(t: &Type) -> Type {
        match t {
            Type::Object(n, args, _) if args.is_empty() && n != "Iterator" && n != "iterable" => {
                t.clone()
            }
            _ => t.traverse(&plain),
        }
    }
    match t {
        RType::T(t) => plain(t),
        RType::Object(Object::Shortened { full, .. } | Object::Aliased { full, .. }) => {
            Type::Object(full.clone(), vec![], false)
        }
        RType::Object(_) => plain(&t.to_sub()),
        RType::Union(types) => {
            let changed = types.iter().any(|t| t.is_object_type());
            let members: Vec<Type> = types.iter().map(normalized).collect();
            if changed {
                union(members)
            } else {
                Type::Union(members)
            }
        }
        RType::Intersection(types) => Type::Intersection(types.iter().map(normalized).collect()),
    }
}

/// `isConstantValue()->yes()`.
fn is_constant_value(t: &RType) -> bool {
    fn constant(t: &Type) -> bool {
        match t {
            Type::ConstBool(_)
            | Type::ConstInt(_)
            | Type::ConstFloat(_)
            | Type::ConstString(_)
            | Type::Null => true,
            Type::ConstArray(a) => a.unsealed.is_none(),
            Type::Union(types) => types.iter().all(constant),
            _ => false,
        }
    }
    match t {
        RType::T(t) => constant(t),
        RType::Union(types) => types.iter().all(is_constant_value),
        _ => false,
    }
}

/// `$type::class`, as far as the comparators tell PHPStan's classes apart.
fn php_class(t: &RType) -> &'static str {
    match t {
        RType::Object(o) => match o {
            Object::FullyQualified(_) => "FullyQualifiedObjectType",
            Object::Shortened { .. } => "ShortenedObjectType",
            Object::Aliased { .. } => "AliasedObjectType",
            Object::NonExisting(_) => "NonExistingObjectType",
            Object::SelfObject(_) => "SelfObjectType",
            Object::SelfStatic(_) => "SelfStaticType",
            Object::ParentStatic(_) => "ParentStaticType",
            Object::ParentWithoutClass => "ParentObjectWithoutClassType",
        },
        RType::Union(_) => "UnionType",
        RType::Intersection(_) => "IntersectionType",
        RType::T(t) => match t {
            Type::Mixed { .. } | Type::SubtractedMixed { .. } => "MixedType",
            Type::Never(_) => "NeverType",
            Type::Void => "VoidType",
            Type::Null => "NullType",
            Type::Bool => "BooleanType",
            Type::ConstBool(_) => "ConstantBooleanType",
            Type::Int => "IntegerType",
            Type::ConstInt(_) => "ConstantIntegerType",
            Type::IntRange(..) => "IntegerRangeType",
            Type::Float => "FloatType",
            Type::ConstFloat(_) => "ConstantFloatType",
            Type::String => "StringType",
            Type::ConstString(_) => "ConstantStringType",
            Type::ClassString(None) => "ClassStringType",
            Type::ClassString(Some(_)) => "GenericClassStringType",
            Type::Union(_) => "UnionType",
            Type::Benevolent(_) => "BenevolentUnionType",
            Type::Intersection(_) => "IntersectionType",
            Type::Accessory(_) => "AccessoryType",
            Type::Array(..) => "ArrayType",
            Type::ConstArray(_) => "ConstantArrayType",
            Type::Object(_, args, _) if args.is_empty() => "ObjectType",
            Type::Object(..) => "GenericObjectType",
            Type::ObjectWithoutClass => "ObjectWithoutClassType",
            Type::Static(..) => "StaticType",
            Type::This(_) => "ThisType",
            Type::Template(_) => "TemplateType",
            _ => "Type",
        },
    }
}

/// `isScalar()->yes()`.
fn is_scalar(t: &Type) -> bool {
    match t {
        Type::Bool
        | Type::ConstBool(_)
        | Type::Int
        | Type::ConstInt(_)
        | Type::IntRange(..)
        | Type::Float
        | Type::ConstFloat(_)
        | Type::String
        | Type::ConstString(_)
        | Type::ClassString(_) => true,
        Type::Accessory(a) => !matches!(a, Accessory::List | Accessory::NonEmptyArray),
        Type::Union(types) | Type::Benevolent(types) => types.iter().all(is_scalar),
        Type::Intersection(types) => types.iter().any(is_scalar),
        _ => false,
    }
}

/// `isClassString()->yes()`.
fn is_class_string(t: &Type) -> bool {
    match t {
        Type::ClassString(_) => true,
        Type::Union(types) => types.iter().all(is_class_string),
        Type::Intersection(types) => types.iter().any(is_class_string),
        _ => false,
    }
}

/// `ScalarTypeComparator::areEqualScalar()`.
fn are_equal_scalar(first: &RType, second: &RType) -> bool {
    let (a, b) = (first.to_super(), second.to_super());
    if a.is_string().yes() && b.is_string().yes() {
        // prevents "class-string" vs "string"
        return php_class(first) == php_class(second);
    }
    if a.is_integer().yes() && b.is_integer().yes() {
        // prevents "int<min, max>" vs "int"
        return php_class(first) == php_class(second);
    }
    if a.is_float().yes() && b.is_float().yes() {
        return true;
    }
    a.is_boolean().yes() && b.is_boolean().yes()
}

/// `ScalarTypeComparator::areDifferentScalarTypes()`.
fn are_different_scalar_types(first: &RType, second: &RType) -> bool {
    let (a, b) = (first.to_super(), second.to_super());
    if !is_scalar(&a) || !is_scalar(&b) {
        return false;
    }
    // treat class-string and string the same
    if a.is_string().yes() && b.is_string().yes() {
        return false;
    }
    if a.is_integer().yes() && b.is_integer().yes() {
        return false;
    }
    if !a.is_string().yes() || !is_class_string(&b) {
        return php_class(first) != php_class(second);
    }
    false
}

/// `normalizeTemplateType()`: a template is its bound.
fn template_bound(t: &RType) -> RType {
    match t {
        RType::T(Type::Template(tpl)) => RType::T(tpl.bound.clone()),
        _ => t.clone(),
    }
}

/// `TypeComparator::areTypesEqual()`.
pub(crate) fn are_types_equal(first: &RType, second: &RType) -> bool {
    let (first, second) = (template_bound(first), template_bound(second));
    if type_hash(&first) == type_hash(&second) {
        return true;
    }
    if are_equal_scalar(&first, &second) {
        return true;
    }
    // aliases and types
    if are_aliased_object_matching_fqn_object(&first, &second) {
        return true;
    }
    // is template of
    are_array_type_with_single_object_child_to_parent(&first, &second)
}

fn are_aliased_object_matching_fqn_object(first: &RType, second: &RType) -> bool {
    match (first, second) {
        (RType::Object(Object::Aliased { full, .. }), b) if b.is_object_type() => {
            b.class_name() == Some(full)
        }
        (a, RType::Object(Object::Aliased { full, .. })) if a.is_object_type() => {
            a.class_name() == Some(full)
        }
        _ => false,
    }
}

/// `A[]` and `B[]` with one class the other's child.
fn are_array_type_with_single_object_child_to_parent(first: &RType, second: &RType) -> bool {
    let (RType::T(Type::Array(_, a)), RType::T(Type::Array(_, b))) = (first, second) else {
        return false;
    };
    let (a, b) = (RType::T((**a).clone()), RType::T((**b).clone()));
    a.is_object_type() && b.is_object_type() && (is_super(&a, &b).yes() || is_super(&b, &a).yes())
}

/// `TypeComparator::isSubtype()`: `$mainType->isSuperTypeOf($checkedType)` (`ArrayTypeComparator` gives the same for
/// two general arrays, the only arrays it gets), never under `mixed`.
pub(crate) fn is_subtype(checked: &RType, main: &RType) -> bool {
    let (checked, main) = (template_bound(checked), template_bound(main));
    !main.is_mixed() && is_super(&main, &checked).yes()
}

/// `TypeTraverser::map()` with `true` and `false` as `bool`; a union with a member changed is
/// `TypeCombinator::union()` of its members.
fn normalize_constant_boolean(t: &RType) -> RType {
    fn plain(t: &Type) -> Type {
        match t {
            Type::ConstBool(_) => Type::Bool,
            _ => t.traverse(&plain),
        }
    }
    match t {
        RType::T(t) => RType::T(plain(t)),
        RType::Object(_) => t.clone(),
        RType::Union(types) => {
            let mapped: Vec<RType> = types.iter().map(normalize_constant_boolean).collect();
            let changed = types
                .iter()
                .any(|t| matches!(t, RType::T(Type::ConstBool(_))));
            if changed {
                combine(mapped)
            } else {
                RType::Union(mapped)
            }
        }
        RType::Intersection(types) => {
            RType::Intersection(types.iter().map(normalize_constant_boolean).collect())
        }
    }
}

/// `TypeCombinator::union()` of members that may hold Rector's object types: pun-phpstan's union when none does,
/// else the members without repeats.
pub(crate) fn combine(types: Vec<RType>) -> RType {
    if types.iter().all(|t| matches!(t, RType::T(_))) {
        return RType::T(union(types.iter().map(RType::to_super).collect()));
    }
    let mut unique: Vec<RType> = Vec::new();
    for t in types {
        if !unique.iter().any(|u| equals(u, &t)) {
            unique.push(t);
        }
    }
    match unique.len() {
        1 => unique.remove(0),
        _ => RType::Union(unique),
    }
}

/// `TypeCombinator::removeNull()`.
pub(crate) fn remove_null(t: &RType) -> RType {
    match t {
        RType::Union(types) => combine(
            types
                .iter()
                .filter(|t| !matches!(t, RType::T(Type::Null)))
                .cloned()
                .collect(),
        ),
        RType::T(t) => RType::T(t.clone().remove_null()),
        _ => t.clone(),
    }
}

/// What the comparison needs to know of the class the compared node is in (`ReflectionResolver`).
#[derive(Clone, Copy, Default)]
pub(crate) struct ClassInfo {
    /// `isClass()`: a class, not an interface, a trait or an enum.
    pub is_class: bool,
    pub is_final: bool,
}

/// `TypeComparator::arePhpParserAndPhpStanPhpDocTypesEqual()`, on the two types mapped already
/// (`mapPhpParserNodePHPStanType`, `mapPHPStanPhpDocTypeNodeToPHPStanType`); `class` is the class reflection of the
/// native node, if any.
pub(crate) fn are_native_and_doc_types_equal(
    native: &RType,
    doc: &RType,
    class: Option<ClassInfo>,
) -> bool {
    if !are_types_equal(native, doc) && is_subtype(doc, native) {
        return false;
    }
    // normalize bool union types
    let native = normalize_constant_boolean(native);
    let doc = normalize_constant_boolean(doc);
    // is scalar replace by another - remove it?
    let different_scalars = are_different_scalar_types(&native, &doc);
    if !different_scalars && !are_types_equal(&native, &doc) {
        return false;
    }
    if are_types_same_with_literal_type_in_phpdoc(different_scalars, &doc, &native) {
        return false;
    }
    let members = doc.union_members().or_else(|| doc.intersection_members());
    if let Some(members) = members
        && members.iter().any(is_template_object)
    {
        return false;
    }
    is_this_type_in_final_class(&doc, &native, class)
}

/// `instanceof TemplateObjectType`: a template bound to a class.
fn is_template_object(t: &RType) -> bool {
    matches!(t, RType::T(Type::Template(tpl)) if matches!(tpl.bound, Type::Object(..)))
}

fn are_types_same_with_literal_type_in_phpdoc(
    different_scalars: bool,
    doc: &RType,
    native: &RType,
) -> bool {
    different_scalars
        && matches!(
            doc,
            RType::T(
                Type::ConstBool(_)
                    | Type::ConstInt(_)
                    | Type::ConstFloat(_)
                    | Type::ConstString(_)
                    | Type::Null
            )
        )
        && is_super(native, doc).yes()
}

/// `$this` documented over `self`/`static` keeps its meaning (the same instance); `static` documented over `self` is
/// the same in a final class.
fn is_this_type_in_final_class(doc: &RType, native: &RType, class: Option<ClassInfo>) -> bool {
    if doc.is_this_type() && native.is_static_type() {
        return false;
    }
    if !(doc.is_static_type() && native.is_this_type()) {
        return true;
    }
    class.is_some_and(|c| c.is_class && c.is_final)
}

/// `TemplateTypeRemovalGuard::isLegal()`: no template, alone or in a union.
pub(crate) fn is_legal_without_template(doc: &RType) -> bool {
    match doc.union_members() {
        Some(members) => !members.iter().any(RType::is_template),
        None => !doc.is_template(),
    }
}

/// `TypeFactory::createMixedPassedOrUnionType($types, $keepConstantTypes)`: the types flattened (`unwrapUnionedTypes`,
/// a constant array becoming one `array<key, item>` per item), one per hash (`uniquateTypes`), then `mixed`, the one
/// type or their union.
pub(crate) fn create_union(types: Vec<RType>, keep_constant: bool) -> RType {
    let mut unwrapped = Vec::new();
    for t in types {
        let flat = t.union_members().unwrap_or_else(|| vec![t]);
        for t in flat {
            match t {
                RType::T(Type::ConstArray(_)) => unwrap_constant_array(&t.to_sub(), &mut unwrapped),
                t => unwrapped.push(t),
            }
        }
    }
    let mut types = uniquate(unwrapped, keep_constant);
    match types.len() {
        0 => RType::mixed(),
        1 => types.remove(0),
        _ if types.iter().any(RType::is_mixed) => RType::mixed(),
        _ => RType::Union(types),
    }
}

/// `unwrapConstantArrayTypes()`: one `array<key, item>` per item of the constant array, nested ones kept nested.
fn unwrap_constant_array(t: &Type, out: &mut Vec<RType>) {
    let flatten = |t: Type| match t {
        Type::Union(types) => types,
        t => vec![t],
    };
    let keys = flatten(t.iterable_key_type());
    for (i, item) in flatten(t.iterable_value_type()).into_iter().enumerate() {
        let key = keys.get(i).cloned().unwrap_or_else(mixed);
        if matches!(item, Type::ConstArray(_)) {
            let mut inner = Vec::new();
            unwrap_constant_array(&item, &mut inner);
            for inner in inner {
                out.push(RType::T(Type::array(key.clone(), inner.to_sub())));
            }
            continue;
        }
        out.push(RType::T(Type::array(key, item)));
    }
}

/// `uniquateTypes()`: one type per hash, the last of them in the place of the first; with `keep_constant` a constant
/// is hashed by its kind and dropped when its general type is there too, else it is its general type; `true` or
/// `false` after both were seen is `bool`.
fn uniquate(types: Vec<RType>, keep_constant: bool) -> Vec<RType> {
    let mut unique: Vec<(String, RType)> = Vec::new();
    let mut constants: Vec<(String, String)> = Vec::new();
    let (mut has_false, mut has_true) = (false, false);
    let insert = |list: &mut Vec<(String, RType)>, hash: String, t: RType| match list
        .iter_mut()
        .find(|(h, _)| *h == hash)
    {
        Some(entry) => entry.1 = t,
        None => list.push((hash, t)),
    };
    for t in types {
        let is_true = matches!(t, RType::T(Type::ConstBool(true)));
        let is_false = matches!(t, RType::T(Type::ConstBool(false)));
        has_true |= is_true;
        has_false |= is_false;
        let t = if has_true && has_false && (is_true || is_false) {
            RType::T(Type::Bool)
        } else {
            t
        };
        match without_value(&t) {
            Some(general) => {
                let general_hash = type_hash(&general);
                if keep_constant {
                    let hash = type_hash(&t);
                    match constants.iter_mut().find(|(h, _)| *h == hash) {
                        Some(entry) => entry.1 = general_hash,
                        None => constants.push((hash.clone(), general_hash)),
                    }
                    insert(&mut unique, hash, t);
                } else {
                    insert(&mut unique, general_hash, general);
                }
            }
            None => {
                let hash = type_hash(&t);
                insert(&mut unique, hash, t);
            }
        }
    }
    for (hash, general) in &constants {
        if unique.iter().any(|(h, _)| h == general) {
            unique.retain(|(h, _)| h != hash);
        }
    }
    unique.into_iter().map(|(_, t)| t).collect()
}

/// `removeValueFromConstantType()`: the general type of a constant, `None` for any other type.
fn without_value(t: &RType) -> Option<RType> {
    Some(RType::T(match t {
        RType::T(Type::ConstFloat(_)) => Type::Float,
        RType::T(Type::ConstString(_)) => Type::String,
        RType::T(Type::ConstInt(_)) => Type::Int,
        RType::T(Type::ConstBool(_)) => Type::Bool,
        _ => return None,
    }))
}
