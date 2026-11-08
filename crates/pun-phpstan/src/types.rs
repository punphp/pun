//! PHPStan's type representation (the subset built-in signatures use), with `describe()` at each
//! `VerbosityLevel`, `isSuperTypeOf()` and `TypeCombinator::union()` / `intersect()`.
//!
//! Each variant is one of PHPStan 2.3.5's `Type` classes; methods keep upstream's control flow so a PHP and a
//! Rust function can be read side by side. Class knowledge (canonical names, ancestors) comes from
//! [`crate::symbols`]: PHPStan's own reflection of built-in classes.

use std::cmp::Ordering;

use crate::symbols;

// PHP descriptions are byte strings. Concatenation must not go through Display or UTF-8
// substitution: UnionType compares these bytes, including inside shapes and callable nodes.
macro_rules! description {
    ($($part:expr),+ $(,)?) => {
        [$(AsRef::<[u8]>::as_ref(&$part)),+].concat()
    };
}

mod accepts;
mod casts;
mod extraction;
mod larastan;
mod late_shape;
mod mixed;
mod reasons;
mod sort;
mod templates;
mod unresolvable;
mod verbosity;
pub use accepts::{StrictMixed, accepts, accepts_with};
pub use casts::{Cast, cast_is_error};
pub use larastan::{StringQuery, with_larastan_strings};
pub use late_shape::LateArrayShape;
pub use reasons::accepts_reasons;
pub use templates::{TemplateMap, infer_template_types, remove_final_by_keyword_overrides, resolve_template_types};
pub use unresolvable::{PropertyReflection, with_property_reflections};
pub use verbosity::recommended_level;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tri {
    No = 0,
    Maybe = 1,
    Yes = 3,
}

impl Tri {
    pub fn from(b: bool) -> Tri {
        if b { Tri::Yes } else { Tri::No }
    }
    pub fn yes(self) -> bool {
        self == Tri::Yes
    }
    pub fn no(self) -> bool {
        self == Tri::No
    }
    pub fn maybe(self) -> bool {
        self == Tri::Maybe
    }
    pub fn and(self, o: Tri) -> Tri {
        self.min(o)
    }
    pub fn or(self, o: Tri) -> Tri {
        self.max(o)
    }
    pub fn negate(self) -> Tri {
        match self {
            Tri::Yes => Tri::No,
            Tri::No => Tri::Yes,
            Tri::Maybe => Tri::Maybe,
        }
    }
    /// `TrinaryLogic::extremeIdentity()`: all equal → that value, else maybe.
    pub fn extreme_identity(results: impl IntoIterator<Item = Tri>) -> Tri {
        let mut it = results.into_iter();
        let first = it.next().unwrap_or(Tri::No);
        if it.all(|r| r == first) {
            first
        } else {
            Tri::Maybe
        }
    }
    /// `TrinaryLogic::lazyMaxMin()`: any yes → yes, else the minimum.
    pub fn max_min(results: impl IntoIterator<Item = Tri>) -> Tri {
        let mut min = Tri::Yes;
        for r in results {
            if r.yes() {
                return r;
            }
            min = min.and(r);
        }
        min
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    TypeOnly,
    Value,
    Precise,
}

impl Level {
    fn handle<T>(self, type_only: impl FnOnce() -> T, value: impl FnOnce() -> T) -> T {
        match self {
            Level::TypeOnly => type_only(),
            _ => value(),
        }
    }
    fn handle3<T>(
        self,
        type_only: impl FnOnce() -> T,
        value: impl FnOnce() -> T,
        precise: impl FnOnce() -> T,
    ) -> T {
        match self {
            Level::TypeOnly => type_only(),
            Level::Value => value(),
            Level::Precise => precise(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeverKind {
    Implicit,
    Explicit,
    /// `NonAcceptingNeverType` (`never` written in PHPDoc)
    NonAccepting,
}

#[derive(Clone, Debug)]
pub enum Accessory {
    NonEmptyString,
    NonFalsyString,
    NumericString,
    LiteralString,
    LowercaseString,
    UppercaseString,
    /// `AccessoryDecimalIntegerStringType`, `inverse` is `non-decimal-int-string`
    DecimalIntString {
        inverse: bool,
    },
    List,
    NonEmptyArray,
    /// `HasMethodType`: comparison is case-insensitive, description retains the literal spelling.
    HasMethod(String),
    /// `HasOffsetValueType`: a required literal offset and its independently narrowed value.
    HasOffsetValue(Box<Type>, Box<Type>),
    /// Larastan's `GenericModelPropertyType` (`model-property<T>` with `checkModelProperties`): a property name of T.
    ModelProperty(Box<Type>),
    /// Larastan's `ViewStringType` (`view-string`): the name of a view that exists.
    ViewString,
}

impl PartialEq for Accessory {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::HasMethod(a), Self::HasMethod(b)) => a.eq_ignore_ascii_case(b),
            (Self::HasOffsetValue(ak, av), Self::HasOffsetValue(bk, bv)) => ak == bk && av == bv,
            (Self::ModelProperty(a), Self::ModelProperty(b)) => a == b,
            (Self::DecimalIntString { inverse: a }, Self::DecimalIntString { inverse: b }) => {
                a == b
            }
            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstArray {
    /// `ConstantIntegerType` / `ConstantStringType` keys
    pub keys: Vec<Type>,
    pub values: Vec<Type>,
    pub next_auto_indexes: Vec<i64>,
    pub optional: Vec<usize>,
    pub is_list: Tri,
    /// `[key, value]` of `...<K, V>`; `None` is a sealed shape outside bleeding edge (`isUnsealed()` maybe).
    pub unsealed: Option<(Type, Type)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub optional: bool,
    pub by_ref: bool,
    pub variadic: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Signature {
    pub params: Vec<Param>,
    pub ret: Type,
    /// a generic callable's own templates (`callable<T>(T): T`)
    pub templates: Vec<Template>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Subject {
    Parameter(String),
    Type(Type),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Conditional {
    pub subject: Subject,
    pub target: Type,
    pub then: Type,
    pub otherwise: Type,
    pub negated: bool,
}

#[derive(Clone, Debug)]
pub struct Template {
    pub name: String,
    /// `TemplateTypeScope::describe()`, e.g. `function preg_match()`, `class ArrayIterator`
    pub scope: String,
    pub bound: Type,
    pub default: Option<Type>,
    /// a class's own template as the argument of its `static` / `$this` type
    pub argument: bool,
}

impl PartialEq for Template {
    /// TemplateTypeTrait::equals(): the parameter/argument strategy is not part of a template's identity.
    fn eq(&self, other: &Self) -> bool {
        self.scope == other.scope
            && self.name == other.name
            && self.bound == other.bound
            && self.default == other.default
    }
}

/// A traversal callback's source decision. Replacing a node may intentionally produce an equal type.
#[derive(Debug)]
pub enum Traversal {
    Retain,
    Replace(Type),
}

impl Traversal {
    pub fn into_type(self, original: &Type) -> Type {
        match self {
            Self::Retain => original.clone(),
            Self::Replace(ty) => ty,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Mixed {
        explicit: bool,
    },
    /// RuleLevelHelper's checked mixed, never stored as a declaration or inferred scope value.
    StrictMixed,
    /// MixedType with its excluded value set. Kept separate so existing plain-mixed constructors stay plain.
    SubtractedMixed { explicit: bool, excluded: Box<Type> },
    Never(NeverKind),
    Error,
    /// ErrorType/NeverType's explanation, deliberately excluded from type identity.
    Reasoned(Box<Type>, String),
    /// `LateResolvableArrayShapeType`: shape keys that still contain templates.
    LateArrayShape(Box<LateArrayShape>),
    Void,
    Null,
    Bool,
    ConstBool(bool),
    Int,
    ConstInt(i64),
    IntRange(Option<i64>, Option<i64>),
    Float,
    ConstFloat(f64),
    String,
    /// PHP's ConstantStringType owns bytes, not a UTF-8 text value.
    ConstString(Vec<u8>),
    ClassString(Option<Box<Type>>),
    /// `__stringAndStringable` (`StringAlwaysAcceptingObjectWithToStringType`)
    StringAcceptingStringable,
    /// `__stringNotStringable` (`StringNeverAcceptingObjectWithToStringType`)
    StringNotAcceptingStringable,
    Resource,
    ObjectWithoutClass,
    /// PHPStan's `NonexistentParentClassType`: an object without a class identity or member capabilities.
    /// This is not the named `parent` retained by unresolved native type hints.
    NonexistentParent,
    /// `ObjectType` / `GenericObjectType`: written name, arguments and per-value `asFinal()` override.
    /// The override affects reflection/containment, not equals() or the printed class name.
    Object(String, Vec<Type>, bool),
    /// ObjectType's negative constraint, independent of generic/static and positive intersection identity.
    SubtractedObject {
        object: Box<Type>,
        excluded: Box<Type>,
    },
    /// `StaticType`; with arguments `GenericStaticType` (`static<T>`)
    Static(String, Vec<Type>),
    This(String),
    Array(Box<Type>, Box<Type>),
    ConstArray(Box<ConstArray>),
    Iterable(Box<Type>, Box<Type>),
    /// `CallableType`; `None` is the common `callable`
    Callable(Option<Box<Signature>>),
    /// `ClosureType` with a signature (plain `Closure` is an object type)
    Closure(Box<Signature>),
    Union(Vec<Type>),
    Benevolent(Vec<Type>),
    Intersection(Vec<Type>),
    Accessory(Accessory),
    Conditional(Box<Conditional>),
    Template(Box<Template>),
    ObjectShape(Vec<(String, Type, bool)>),
    /// `EnumCaseObjectType`
    EnumCase(String, String),
    /// a bivariant `*` generic argument (explicit `mixed`)
    Wildcard,
    /// a generic argument's call-site variance (`GenericObjectType::getVariances()`): `covariant T` (true) or
    /// `contravariant T` (false)
    Projection(bool, Box<Type>),
    /// `OffsetAccessType` (`T['k']`) of a template, resolved late; ponytail: used as an error type (accepting
    /// anything) until the scope resolves it
    OffsetAccess(Box<Type>, Box<Type>),
}

/// `TemplateTypeVariance::describe()` of a call-site variance
pub fn variance_name(covariant: bool) -> &'static str {
    if covariant { "covariant" } else { "contravariant" }
}

pub fn mixed() -> Type {
    Type::Mixed { explicit: false }
}

/// `(new BenevolentUnionType([int, string]))->toArrayKey()`
fn array_key() -> Type {
    Type::Benevolent(vec![Type::Int, Type::String])
}

impl Type {
    /// `new ArrayType($key, $item)`: a `(int|string)` benevolent key is stored as implicit mixed.
    pub fn array(key: Type, item: Type) -> Type {
        let key = if key.is_benevolent_array_key() {
            mixed()
        } else if matches!(key, Type::StrictMixed) {
            // ArrayType::__construct(): StrictMixedType keys become the PHP array-key domain.
            Type::Union(vec![Type::String, Type::Int])
        } else {
            key
        };
        Type::Array(Box::new(key), Box::new(item))
    }

    pub fn list(item: Type) -> Type {
        Type::Intersection(vec![
            Type::array(Type::IntRange(Some(0), None), item),
            Type::Accessory(Accessory::List),
        ])
    }

    fn is_benevolent_array_key(&self) -> bool {
        matches!(self, Type::Benevolent(types) if types.len() == 2 && types.contains(&Type::Int) && types.contains(&Type::String))
    }

    /// `IntegerRangeType::fromInterval()`
    pub fn int_range(min: Option<i64>, max: Option<i64>) -> Type {
        match (min, max) {
            (Some(a), Some(b)) if a > b => Type::Never(NeverKind::Implicit),
            (Some(a), Some(b)) if a == b => Type::ConstInt(a),
            (None, None) => Type::Int,
            (None, Some(i64::MIN)) => Type::ConstInt(i64::MIN),
            (Some(i64::MAX), None) => Type::ConstInt(i64::MAX),
            _ => Type::IntRange(min, max),
        }
    }

    /// An integer type's bounds (`int` is unbounded).
    fn int_bounds(&self) -> Option<(Option<i64>, Option<i64>)> {
        match self {
            Type::Int => Some((None, None)),
            Type::IntRange(min, max) => Some((*min, *max)),
            Type::ConstInt(v) => Some((Some(*v), Some(*v))),
            _ => None,
        }
    }

    fn is_implicit_never(&self) -> bool {
        matches!(self.unreasoned(), Type::Never(NeverKind::Implicit))
    }

    pub fn is_never(&self) -> bool {
        matches!(self.unreasoned(), Type::Never(_))
    }

    /// `$t instanceof MixedType && $t->describe(precise) === 'mixed' && !$t->isExplicitMixed()`
    fn is_plain_implicit_mixed(&self) -> bool {
        matches!(self, Type::Mixed { explicit: false })
    }

    fn is_compound(&self) -> bool {
        matches!(
            self,
            Type::Mixed { .. }
                | Type::StrictMixed
                | Type::SubtractedMixed { .. }
                | Type::Error
                | Type::Never(_)
                | Type::Reasoned(..)
                | Type::LateArrayShape(_)
                | Type::IntRange(..)
                | Type::Iterable(..)
                | Type::Callable(_)
                | Type::Union(_)
                | Type::Benevolent(_)
                | Type::Intersection(_)
                | Type::Accessory(_)
                | Type::Conditional(_)
                | Type::Template(_)
        )
    }

    fn query(&self, f: &dyn Fn(&Type) -> Tri) -> Tri {
        self.query_mixed(f, Tri::Maybe)
    }

    /// Member capabilities/existence use unionResults too, but MixedType answers Yes rather than Maybe.
    /// The visitor supplies reflection-dependent leaf answers; StrictMixedType remains an ordinary No leaf.
    pub fn query_members(&self, f: &dyn Fn(&Type) -> Tri) -> Tri {
        self.query_mixed(f, Tri::Yes)
    }

    fn query_mixed(&self, f: &dyn Fn(&Type) -> Tri, mixed: Tri) -> Tri {
        match self {
            Type::Reasoned(t, _) => t.query_mixed(f, mixed),
            Type::SubtractedObject { object, .. } => object.query_mixed(f, mixed),
            Type::LateArrayShape(s) => s.resolve().query_mixed(f, mixed),
            Type::Union(types) => Tri::extreme_identity(types.iter().map(|t| t.query_mixed(f, mixed))),
            Type::Benevolent(types) => types.iter().map(|t| t.query_mixed(f, mixed)).fold(Tri::No, Tri::or),
            Type::Intersection(types) => Tri::max_min(types.iter().map(|t| t.query_mixed(f, mixed))),
            Type::Template(t) => t.bound.query_mixed(f, mixed),
            // LateResolvableTypeTrait
            Type::Conditional(c) => c.resolve().query_mixed(f, mixed),
            Type::Mixed { .. } | Type::SubtractedMixed { .. } | Type::Error => mixed,
            Type::Never(_) => Tri::No,
            _ => f(self),
        }
    }

    /// `query()` for the questions a common `CallableType` answers maybe (it may be a string, an array or an object).
    fn query_c(&self, f: &dyn Fn(&Type) -> Tri) -> Tri {
        self.query(&|t| {
            if matches!(t, Type::Callable(_)) {
                Tri::Maybe
            } else {
                f(t)
            }
        })
    }

    /// MixedType excludes a query only when its subtracted type contains the complete domain.
    /// `benevolent` selects UnionType::unionResults instead of notBenevolentUnionResults.
    fn query_with_mixed_domain(
        &self,
        query: fn(&Type) -> Tri,
        domain: fn() -> Type,
        benevolent: bool,
        f: &dyn Fn(&Type) -> Tri,
    ) -> Tri {
        match self {
            Type::Reasoned(t, _) => query(t),
            Type::SubtractedObject { object, .. } => query(object),
            Type::LateArrayShape(s) => query(&s.resolve()),
            Type::Union(types) => Tri::extreme_identity(types.iter().map(query)),
            Type::Benevolent(types) if benevolent => types.iter().map(query).fold(Tri::No, Tri::or),
            Type::Benevolent(types) => Tri::extreme_identity(types.iter().map(query)),
            Type::Intersection(types) => Tri::max_min(types.iter().map(query)),
            // TemplateMixedType inherits plain MixedType queries, not the bound's excluded set.
            Type::Template(t) if matches!(&t.bound, Type::SubtractedMixed { .. }) => Tri::Maybe,
            Type::Template(t) => query(&t.bound),
            Type::Conditional(c) => query(&c.resolve()),
            Type::SubtractedMixed { excluded, .. } => {
                if is_super_type_of(excluded, &domain()).yes() { Tri::No } else { Tri::Maybe }
            }
            Type::Mixed { .. } | Type::Error => Tri::Maybe,
            _ => f(self),
        }
    }

    fn query_c_with_mixed_domain(
        &self,
        query: fn(&Type) -> Tri,
        domain: fn() -> Type,
        benevolent: bool,
        f: &dyn Fn(&Type) -> Tri,
    ) -> Tri {
        self.query_with_mixed_domain(query, domain, benevolent, &|t| {
            if matches!(t, Type::Callable(_)) { Tri::Maybe } else { f(t) }
        })
    }

    /// A callable array (`callable&array`): `array{class-string|object, non-falsy-string}`.
    fn is_callable_array(&self) -> bool {
        matches!(self, Type::Intersection(_)) && self.is_callable().yes() && self.is_array().yes()
    }

    pub fn is_string(&self) -> Tri {
        // UnionType::isString uses notBenevolentUnionResults, unlike canAccessConstants/isObject.
        match self {
            Type::Union(ts) | Type::Benevolent(ts) => {
                return Tri::extreme_identity(ts.iter().map(Type::is_string));
            }
            Type::Intersection(ts) => return Tri::max_min(ts.iter().map(Type::is_string)),
            Type::Template(t) if matches!(&t.bound, Type::SubtractedMixed { .. }) => return Tri::Maybe,
            Type::Template(t) => return t.bound.is_string(),
            Type::Conditional(c) => return c.resolve().is_string(),
            _ => {}
        }
        self.query_c_with_mixed_domain(Self::is_string, || Type::String, false, &|t| {
            if matches!(t, Type::Accessory(Accessory::HasMethod(_) | Accessory::HasOffsetValue(..))) {
                return Tri::Maybe;
            }
            Tri::from(matches!(
                t,
                Type::String
                    | Type::ConstString(_)
                    | Type::ClassString(_)
                    | Type::StringAcceptingStringable
                    | Type::StringNotAcceptingStringable
                    | Type::Accessory(
                        Accessory::NonEmptyString
                            | Accessory::NonFalsyString
                            | Accessory::NumericString
                            | Accessory::LiteralString
                            | Accessory::LowercaseString
                            | Accessory::UppercaseString
                            | Accessory::DecimalIntString { .. }
                            | Accessory::ModelProperty(_)
                            | Accessory::ViewString
                    )
            ))
        })
    }

    pub fn is_integer(&self) -> Tri {
        self.query(&|t| {
            Tri::from(matches!(
                t,
                Type::Int | Type::ConstInt(_) | Type::IntRange(..)
            ))
        })
    }

    pub fn is_float(&self) -> Tri {
        self.query_with_mixed_domain(Self::is_float, || Type::Float, false, &|t| Tri::from(matches!(t, Type::Float | Type::ConstFloat(_))))
    }

    pub fn is_boolean(&self) -> Tri {
        self.query_with_mixed_domain(Self::is_boolean, || Type::Bool, false, &|t| Tri::from(matches!(t, Type::Bool | Type::ConstBool(_))))
    }

    /// `isObject()`, not `ObjectWithoutClassType::isSuperTypeOf()`: the latter rejects nonexistent parent.
    pub fn is_object(&self) -> Tri {
        self.query_c_with_mixed_domain(Self::is_object, || Type::ObjectWithoutClass, true, &|t| match t {
            Type::NonexistentParent
            | Type::ObjectWithoutClass
            | Type::Object(..)
            | Type::Static(..)
            | Type::This(_)
            | Type::EnumCase(..)
            | Type::ObjectShape(_)
            | Type::Closure(_) => Tri::Yes,
            Type::Iterable(..)
            | Type::Accessory(Accessory::HasMethod(_) | Accessory::HasOffsetValue(..)) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    fn is_class_string(&self) -> Tri {
        self.query_c(&|t| match t {
            Type::ClassString(_) => Tri::Yes,
            Type::Accessory(Accessory::HasMethod(_)) => Tri::Maybe,
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::Accessory(
                Accessory::NonEmptyString
                | Accessory::NonFalsyString
                | Accessory::LiteralString
                | Accessory::ModelProperty(_)
                | Accessory::ViewString,
            ) => Tri::Maybe,
            Type::ConstString(s) => Tri::from(
                std::str::from_utf8(s)
                    .ok()
                    .and_then(symbols::lookup)
                    .is_some(),
            ),
            _ => Tri::No,
        })
    }

    pub fn is_array(&self) -> Tri {
        self.query_c_with_mixed_domain(Self::is_array, || Type::array(mixed(), mixed()), false, &|t| {
            if matches!(t, Type::Iterable(..)) { return Tri::Maybe; }
            if matches!(t, Type::Accessory(Accessory::HasOffsetValue(..))) {
                return Tri::Maybe;
            }
            Tri::from(matches!(
                t,
                Type::Array(..)
                    | Type::ConstArray(_)
                    | Type::Accessory(Accessory::List | Accessory::NonEmptyArray)
            ))
        })
    }

    pub fn is_constant_array(&self) -> Tri {
        if self.is_callable_array() {
            return Tri::Yes;
        }
        self.query_c(&|t| match t {
            Type::ConstArray(_) => Tri::Yes,
            Type::Array(..) | Type::Accessory(Accessory::List | Accessory::NonEmptyArray | Accessory::HasOffsetValue(..)) => {
                Tri::Maybe
            }
            _ => Tri::No,
        })
    }

    fn is_constant_value(&self) -> Tri {
        self.query(&|t| {
            Tri::from(matches!(
                t,
                Type::ConstBool(_)
                    | Type::ConstInt(_)
                    | Type::ConstFloat(_)
                    | Type::ConstString(_)
                    | Type::Null
                    | Type::ConstArray(_)
            ))
        })
    }

    fn is_constant_scalar(&self) -> bool {
        match self.scalar_class_type() {
            Type::ConstBool(_)
            | Type::ConstInt(_)
            | Type::ConstFloat(_)
            | Type::ConstString(_)
            | Type::Null => true,
            // UnionType's query aggregates its members, but it does not implement ConstantScalarType.
            Type::Union(parts) | Type::Benevolent(parts) => parts.iter().all(Self::is_constant_scalar),
            _ => false,
        }
    }

    /// TemplateTypeFactory's scalar subclasses, for instanceof dispatch only. A direct constant string/int
    /// bound creates a constant subclass; using that template as another bound creates StringType/IntegerType.
    /// NullType stays constant through nested bounds, while BooleanType never becomes ConstantBooleanType.
    fn scalar_class_type(&self) -> &Type {
        match self {
            Type::Template(t) => match &t.bound {
                bound @ (Type::ConstInt(_) | Type::ConstString(_) | Type::Null) => bound,
                bound @ (Type::Union(_) | Type::Benevolent(_)) => bound,
                Type::String => &Type::String,
                Type::Int | Type::IntRange(..) => &Type::Int,
                Type::Bool | Type::ConstBool(_) => &Type::Bool,
                Type::Template(_) => match t.bound.scalar_class_type() {
                    Type::String | Type::ConstString(_) => &Type::String,
                    Type::Int | Type::ConstInt(_) => &Type::Int,
                    Type::Bool => &Type::Bool,
                    Type::Null => &Type::Null,
                    bound @ (Type::Union(_) | Type::Benevolent(_)) => bound,
                    _ => self,
                },
                _ => self,
            },
            _ => self,
        }
    }

    pub fn is_list(&self) -> Tri {
        if self.is_callable_array() {
            return Tri::Yes;
        }
        self.query_c_with_mixed_domain(Self::is_list, || Type::list(mixed()), false, &|t| match t {
            Type::Accessory(Accessory::List) => Tri::Yes,
            Type::Accessory(Accessory::HasOffsetValue(key, _)) => {
                if key.is_string().yes() { Tri::No } else { Tri::Maybe }
            }
            Type::ConstArray(a) => a.is_list,
            // `ArrayType::isList()`: no when no key can be a non-negative int
            Type::Array(key, _) if is_super_type_of(&Type::int_range(Some(0), None), key).no() => {
                Tri::No
            }
            // (MaybeArrayTypeTrait)
            Type::Array(..) | Type::Accessory(Accessory::NonEmptyArray) | Type::Iterable(..) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    pub fn is_iterable(&self) -> Tri {
        if self.is_never() {
            return Tri::Yes;
        }
        self.query_c_with_mixed_domain(Self::is_iterable, || Type::Iterable(Box::new(mixed()), Box::new(mixed())), true, &|t| match t {
            Type::Array(..)
            | Type::ConstArray(_)
            | Type::Iterable(..)
            | Type::Accessory(Accessory::List | Accessory::NonEmptyArray) => Tri::Yes,
            Type::Object(class, _, _) | Type::Static(class, _) | Type::This(class) => {
                is_instance_of(class, "Traversable")
            }
            Type::ObjectWithoutClass
            | Type::ObjectShape(_)
            | Type::Accessory(Accessory::HasMethod(_) | Accessory::HasOffsetValue(..)) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    /// `isCloneable()`: an object, `mixed` and `never`; a common callable or iterable maybe.
    pub fn is_cloneable(&self) -> Tri {
        match self {
            Type::Reasoned(t, _) => t.is_cloneable(),
            Type::LateArrayShape(s) => s.resolve().is_cloneable(),
            Type::Mixed { .. } | Type::SubtractedMixed { .. } | Type::Error | Type::Never(_) => Tri::Yes,
            Type::Template(t) => t.bound.is_cloneable(),
            _ => self.query(&|t| match t {
                Type::Object(..)
                | Type::ObjectWithoutClass
                | Type::ObjectShape(_)
                | Type::Static(..)
                | Type::This(_)
                | Type::EnumCase(..)
                | Type::Closure(_) => Tri::Yes,
                Type::Callable(_)
                | Type::Iterable(..)
                | Type::Accessory(Accessory::HasMethod(_) | Accessory::HasOffsetValue(..)) => Tri::Maybe,
                _ => Tri::No,
            }),
        }
    }

    pub fn is_iterable_at_least_once(&self) -> Tri {
        match self {
            _ if self.is_callable_array() => Tri::Yes,
            Type::Intersection(types) => {
                let iterable: Vec<&Type> = types.iter().filter(|t| !t.is_iterable().no()).collect();
                if iterable.is_empty() {
                    return Tri::No;
                }
                Tri::max_min(iterable.into_iter().map(Type::is_iterable_at_least_once))
            }
            _ => self.query_with_mixed_domain(Self::is_iterable_at_least_once, || Type::Iterable(Box::new(mixed()), Box::new(mixed())), true, &|t| match t {
                Type::Never(_) => Tri::Maybe,
                Type::ConstArray(a) => a.is_iterable_at_least_once(),
                Type::Accessory(Accessory::NonEmptyArray | Accessory::HasOffsetValue(..)) => Tri::Yes,
                Type::Array(..) | Type::Iterable(..) | Type::Accessory(Accessory::List) => {
                    Tri::Maybe
                }
                // StaticType asks its static object type
                Type::Object(..) | Type::Static(..) | Type::This(_) | Type::ObjectWithoutClass | Type::Callable(_) => Tri::Maybe,
                _ => Tri::No,
            }),
        }
    }

    fn is_non_empty_string(&self) -> Tri {
        self.query_c(&|t| match t {
            Type::Accessory(
                Accessory::NonEmptyString
                | Accessory::NonFalsyString
                | Accessory::NumericString
                | Accessory::DecimalIntString { inverse: false },
            ) => Tri::Yes,
            Type::ConstString(s) => Tri::from(!s.is_empty()),
            Type::ClassString(_) => Tri::Yes,
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::Accessory(_) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    fn is_non_falsy_string(&self) -> Tri {
        self.query_c(&|t| match t {
            Type::Accessory(Accessory::NonFalsyString) => Tri::Yes,
            Type::ConstString(s) => Tri::from(!s.is_empty() && s.as_slice() != b"0"),
            Type::ClassString(_) => Tri::Yes,
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::Accessory(_) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    pub fn is_numeric_string(&self) -> Tri {
        self.query(&|t| match t {
            Type::Accessory(Accessory::NumericString)
            | Type::Accessory(Accessory::DecimalIntString { inverse: false }) => Tri::Yes,
            Type::ConstString(s) => Tri::from(
                std::str::from_utf8(s)
                    .ok()
                    .and_then(crate::value::numeric)
                    .is_some(),
            ),
            Type::ClassString(_) => Tri::No,
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::Accessory(_) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    /// `isDecimalIntegerString()`
    pub fn is_decimal_int_string(&self) -> Tri {
        self.query(&|t| match t {
            Type::Accessory(Accessory::DecimalIntString { inverse }) => Tri::from(!inverse),
            Type::ConstString(s) => Tri::from(
                std::str::from_utf8(s)
                    .ok()
                    .and_then(|s| s.parse::<i64>().ok())
                    .is_some_and(|i| i.to_string().as_bytes() == s),
            ),
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::Accessory(
                Accessory::NonEmptyString
                | Accessory::NonFalsyString
                | Accessory::NumericString
                | Accessory::LiteralString
                | Accessory::LowercaseString
                | Accessory::UppercaseString
                | Accessory::HasMethod(_)
                | Accessory::ModelProperty(_)
                | Accessory::ViewString,
            ) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    fn is_literal_string(&self) -> Tri {
        self.query_c(&|t| match t {
            Type::Accessory(Accessory::LiteralString) | Type::ConstString(_) => Tri::Yes,
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::ClassString(_)
            | Type::Accessory(_) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    fn is_lowercase_string(&self) -> Tri {
        self.query_c(&|t| match t {
            Type::Accessory(Accessory::LowercaseString)
            | Type::Accessory(Accessory::DecimalIntString { inverse: false }) => Tri::Yes,
            Type::ConstString(s) => Tri::from(s.to_ascii_lowercase() == *s),
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::ClassString(_)
            | Type::Accessory(_) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    fn is_uppercase_string(&self) -> Tri {
        self.query_c(&|t| match t {
            Type::Accessory(Accessory::UppercaseString)
            | Type::Accessory(Accessory::DecimalIntString { inverse: false }) => Tri::Yes,
            Type::ConstString(s) => Tri::from(s.to_ascii_uppercase() == *s),
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::ClassString(_)
            | Type::Accessory(_) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    pub fn is_callable(&self) -> Tri {
        self.query_with_mixed_domain(Self::is_callable, || Type::Callable(None), true, &|t| match t {
            Type::Callable(_) | Type::Closure(_) => Tri::Yes,
            Type::Accessory(Accessory::HasMethod(n)) if n.eq_ignore_ascii_case("__invoke") => {
                Tri::Yes
            }
            Type::Accessory(Accessory::NumericString)
            | Type::Accessory(Accessory::DecimalIntString { inverse: false }) => Tri::No,
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::ClassString(_)
            | Type::Accessory(_)
            | Type::Iterable(..)
            | Type::ObjectWithoutClass
            | Type::ObjectShape(_) => Tri::Maybe,
            Type::ConstString(s) => string_is_callable(s),
            Type::Array(_, item) => Tri::Maybe.and(item.is_string()),
            Type::ConstArray(a) => a.is_callable(),
            Type::Object(class, _, _) | Type::Static(class, _) | Type::This(class) => {
                match symbols::lookup(class) {
                    Some(c) if has_method(&c, "__invoke") => Tri::Yes,
                    Some(c) if c.is_final || matches!(t, Type::Object(_, _, true)) => Tri::No,
                    _ => Tri::Maybe,
                }
            }
            _ => Tri::No,
        })
    }

    /// `Type::hasMethod()`, using the same class metadata as the other shared type queries.
    pub fn has_method(&self, name: &str) -> Tri {
        self.query(&|t| match t {
            Type::Accessory(Accessory::HasMethod(n)) => {
                if n.eq_ignore_ascii_case(name) {
                    Tri::Yes
                } else {
                    Tri::Maybe
                }
            }
            Type::Object(n, _, _) | Type::Static(n, _) | Type::This(n) | Type::EnumCase(n, _) => {
                match symbols::lookup(n) {
                    Some(c) if has_method(&c, &name.to_ascii_lowercase()) => Tri::Yes,
                    Some(c) if c.is_final || matches!(t, Type::Object(_, _, true)) => Tri::No,
                    _ => Tri::Maybe,
                }
            }
            Type::ClassString(_)
            | Type::ObjectWithoutClass
            | Type::ObjectShape(_)
            | Type::Iterable(..)
            | Type::Callable(_) => Tri::Maybe,
            Type::ConstString(_) if t.is_class_string().yes() => Tri::Maybe,
            _ => Tri::No,
        })
    }

    /// MethodExistsTypeSpecifyingExtension's truthy literal-method refinement. Plain strings are unchanged;
    /// class strings retain their string type, while mixed/object receivers gain only an object accessory.
    pub fn require_method(self, name: &str) -> Type {
        let accessory = Type::Accessory(Accessory::HasMethod(name.to_string()));
        if self.is_string().yes() {
            return if self.is_class_string().yes() {
                intersect(vec![self, accessory])
            } else {
                self
            };
        }
        intersect(vec![
            self,
            union(vec![
                intersect(vec![Type::ObjectWithoutClass, accessory]),
                Type::ClassString(None),
            ]),
        ])
    }

    /// `getObjectClassNames()`
    pub fn object_class_names(&self) -> Vec<String> {
        match self {
            Type::SubtractedObject { object, .. } => object.object_class_names(),
            Type::Object(class, _, _)
            | Type::Static(class, _)
            | Type::This(class)
            | Type::EnumCase(class, _) => vec![class.clone()],
            Type::Closure(_) => vec!["Closure".to_string()],
            // TemplateObjectType and TemplateUnionType inherit their bound's object identities.
            Type::Template(t) => t.bound.object_class_names(),
            Type::Union(types) | Type::Benevolent(types) | Type::Intersection(types) => {
                let mut names = Vec::new();
                for t in types {
                    let inner = t.object_class_names();
                    // BenevolentUnionType's pickFromTypes ignores nonobjects, not unnamed objects.
                    if inner.is_empty()
                        && (matches!(self, Type::Union(_))
                            || matches!(self, Type::Benevolent(_))
                                && t.is_object().yes())
                    {
                        return Vec::new();
                    }
                    for name in inner {
                        if !names.contains(&name) {
                            names.push(name);
                        }
                    }
                }
                names
            }
            _ => vec![],
        }
    }

    pub fn iterable_key_type(&self) -> Type {
        match self {
            Type::Reasoned(t, _) => t.iterable_key_type(),
            Type::SubtractedObject { object, .. } => object.iterable_key_type(),
            Type::LateArrayShape(s) => s.resolve().iterable_key_type(),
            Type::NonexistentParent => Type::Error,
            Type::StrictMixed => self.clone(),
            Type::Template(t) if matches!(t.bound, Type::StrictMixed) => self.clone(),
            _ if self.is_callable_array() => {
                Type::Union(vec![Type::ConstInt(0), Type::ConstInt(1)])
            }
            Type::Array(key, _) => {
                if matches!(**key, Type::Mixed { .. }) {
                    array_key()
                } else {
                    (**key).clone()
                }
            }
            Type::ConstArray(a) => a.iterable_key_type(),
            Type::Iterable(key, _) => (**key).clone(),
            Type::Accessory(Accessory::List) => Type::int_range(Some(0), None),
            Type::Intersection(types) => {
                intersect(types.iter().map(Type::iterable_key_type).collect())
            }
            Type::Union(types) => union(types.iter().map(Type::iterable_key_type).collect()),
            Type::Template(t) => t.bound.iterable_key_type(),
            Type::Object(class, args, _) => iterable_arg(class, args, true),
            Type::Static(class, args) => static_of(class, args).iterable_key_type(),
            Type::This(class) => static_object(class).iterable_key_type(),
            Type::Never(_) => Type::Never(NeverKind::Implicit),
            _ => mixed(),
        }
    }

    pub fn iterable_value_type(&self) -> Type {
        match self {
            Type::Reasoned(t, _) => t.iterable_value_type(),
            Type::SubtractedObject { object, .. } => object.iterable_value_type(),
            Type::LateArrayShape(s) => s.resolve().iterable_value_type(),
            Type::NonexistentParent => Type::Error,
            Type::StrictMixed => self.clone(),
            Type::Template(t) if matches!(t.bound, Type::StrictMixed) => self.clone(),
            Type::Intersection(types) if self.is_callable_array() => intersect(vec![
                intersect(types.iter().map(Type::iterable_value_type).collect()),
                Type::Union(vec![
                    Type::ObjectWithoutClass,
                    Type::Intersection(vec![
                        Type::String,
                        Type::Accessory(Accessory::NonFalsyString),
                    ]),
                ]),
            ]),
            Type::Array(_, value) | Type::Iterable(_, value) => (**value).clone(),
            Type::ConstArray(a) => a.iterable_value_type(),
            Type::Intersection(types) => {
                intersect(types.iter().map(Type::iterable_value_type).collect())
            }
            Type::Union(types) => union(types.iter().map(Type::iterable_value_type).collect()),
            Type::Template(t) => t.bound.iterable_value_type(),
            Type::Object(class, args, _) => {
                let value = iterable_arg(class, args, false);
                // an Iterator's `current()` and an IteratorAggregate's `getIterator()` resolve a missing argument
                // to its bound, an explicit `mixed` (`ObjectType::getIterableValueType()`)
                if value.is_implicit_mixed()
                    && (is_instance_of(class, "Iterator").yes()
                        || is_instance_of(class, "IteratorAggregate").yes())
                {
                    Type::Mixed { explicit: true }
                } else {
                    value
                }
            }
            Type::Static(class, args) => static_of(class, args).iterable_value_type(),
            Type::This(class) => static_object(class).iterable_value_type(),
            Type::Never(_) => Type::Never(NeverKind::Implicit),
            _ => mixed(),
        }
    }

    pub fn describe(&self, level: Level) -> String {
        display_bytes(&self.describe_bytes(level))
    }

    fn describe_bytes(&self, level: Level) -> Vec<u8> {
        self.describe_with_subtraction(level, None)
    }

    fn describe_with_subtraction(&self, level: Level, excluded: Option<&Type>) -> Vec<u8> {
        match self {
            Type::Reasoned(t, _) => t.describe_bytes(level),
            Type::LateArrayShape(s) => s.print(false),
            Type::Mixed { .. } | Type::StrictMixed => "mixed".into(),
            Type::SubtractedObject { object, excluded } => {
                object.describe_with_subtraction(level, Some(excluded))
            }
            Type::SubtractedMixed { excluded, .. } => level.handle3(
                || "mixed".into(),
                || "mixed".into(),
                || {
                    let description = excluded.describe_bytes(level);
                    // SubstractableTypeTrait groups unions and types with their own subtraction, not intersections.
                    // Template subclasses inherit the union class or delegate getSubtractedType() to their bound.
                    let mut class = excluded.unreasoned();
                    while let Type::Template(t) = class {
                        class = t.bound.unreasoned();
                    }
                    if matches!(
                        class,
                        Type::Union(_)
                            | Type::Benevolent(_)
                            | Type::SubtractedMixed { .. }
                            | Type::SubtractedObject { .. }
                    ) {
                        description!("mixed~(", description, ")")
                    } else {
                        description!("mixed~", description)
                    }
                },
            ),
            Type::Error => level.handle3(|| "mixed".into(), || "mixed".into(), || "*ERROR*".into()),
            Type::Never(NeverKind::NonAccepting) => "never".into(),
            Type::Never(_) => "*NEVER*".into(),
            Type::Void => "void".into(),
            Type::Null => "null".into(),
            Type::Bool => "bool".into(),
            Type::ConstBool(b) => (if *b { "true" } else { "false" }).into(),
            Type::Int => "int".into(),
            Type::ConstInt(i) => level.handle(|| "int".into(), || i.to_string().into_bytes()),
            Type::IntRange(min, max) => format!(
                "int<{}, {}>",
                min.map_or("min".to_string(), |v| v.to_string()),
                max.map_or("max".to_string(), |v| v.to_string())
            )
            .into_bytes(),
            Type::Float => "float".into(),
            Type::ConstFloat(f) => {
                level.handle(|| "float".into(), || float_to_string(*f).into_bytes())
            }
            Type::String | Type::StringAcceptingStringable | Type::StringNotAcceptingStringable => {
                "string".into()
            }
            Type::ConstString(s) => level.handle3(
                || "string".into(),
                || {
                    if std::str::from_utf8(s)
                        .ok()
                        .and_then(symbols::lookup)
                        .is_some_and(|c| c.name.as_bytes() == s)
                    {
                        export_string(s)
                    } else {
                        match std::str::from_utf8(s) {
                            Ok(s) => export_string(crate::schema::truncate(s, 20, "…").as_bytes()),
                            Err(_) if String::from_utf8_lossy(s).chars().count() <= 20 => {
                                export_string(s)
                            }
                            // Nette's UTF-8 regexp runs only when truncation is necessary. PHPStan
                            // catches that failure and appends an ellipsis after twenty original bytes.
                            Err(_) => {
                                export_string(&[&s[..s.len().min(20)], "…".as_bytes()].concat())
                            }
                        }
                    }
                },
                || export_string(s),
            ),
            Type::ClassString(None) => "class-string".into(),
            Type::ClassString(Some(t)) => {
                description!("class-string<", t.describe_bytes(level), ">")
            }
            Type::Resource => "resource".into(),
            Type::ObjectWithoutClass => "object".into(),
            Type::NonexistentParent => "parent".into(),
            Type::Object(class, args, _) => {
                let mut name = match level {
                    Level::Precise => class.clone(),
                    _ => canonical_class(class),
                }
                .into_bytes();
                if level == Level::Precise
                    && let Some(excluded) = excluded
                {
                    let description = excluded.describe_bytes(level);
                    if matches!(
                        excluded,
                        Type::Union(_)
                            | Type::Benevolent(_)
                            | Type::SubtractedMixed { .. }
                            | Type::SubtractedObject { .. }
                    ) {
                        name.extend(description!("~(", description, ")"));
                    } else {
                        name.extend(description!("~", description));
                    }
                }
                if args.is_empty() {
                    name
                } else {
                    let args: Vec<_> = args.iter().map(|a| a.describe_bytes(level)).collect();
                    description!(name, "<", args.join(b", ".as_slice()), ">")
                }
            }
            Type::Static(class, args) => {
                description!(
                    "static(",
                    static_of(class, args).describe_with_subtraction(level, excluded),
                    ")"
                )
            }
            Type::This(class) => description!(
                "$this(",
                static_object(class).describe_with_subtraction(level, excluded),
                ")"
            ),
            Type::EnumCase(class, case) => format!("{class}::{case}").into_bytes(),
            Type::Wildcard => "*".into(),
            // `TypeProjectionHelper::describe()`
            Type::Projection(c, t) => description!(variance_name(*c), " ", t.describe_bytes(level)),
            // (`Printer::print($this->toPhpDocNode())`)
            Type::OffsetAccess(..) => to_phpdoc_bytes(self),
            Type::Array(key, item) => {
                let mixed_key = key.is_plain_implicit_mixed();
                let mixed_item = item.is_plain_implicit_mixed();
                if mixed_key || (level != Level::Precise && key.is_never()) {
                    if mixed_item || (level != Level::Precise && item.is_never()) {
                        return "array".into();
                    }
                    return description!("array<", item.describe_bytes(level), ">");
                }
                description!(
                    "array<",
                    key.describe_bytes(level),
                    ", ",
                    item.describe_bytes(level),
                    ">"
                )
            }
            Type::ConstArray(a) => a.describe(level),
            Type::Iterable(key, item) => {
                if matches!(**key, Type::Mixed { .. }) {
                    if matches!(**item, Type::Mixed { .. }) {
                        return "iterable".into();
                    }
                    return description!("iterable<", item.describe_bytes(level), ">");
                }
                description!(
                    "iterable<",
                    key.describe_bytes(level),
                    ", ",
                    item.describe_bytes(level),
                    ">"
                )
            }
            Type::Callable(sig) => level.handle(
                || "callable".into(),
                || {
                    print_callable(
                        "callable",
                        sig.as_deref().unwrap_or(&Signature {
                            params: vec![],
                            ret: mixed(),
                            templates: vec![],
                        }),
                    )
                },
            ),
            Type::Closure(sig) => {
                level.handle(|| "Closure".into(), || print_callable("Closure", sig))
            }
            Type::Union(types) => describe_union(types, level),
            Type::Benevolent(types) => description!("(", describe_union(types, level), ")"),
            Type::Intersection(types) => match level {
                Level::TypeOnly => describe_intersection_type(self, types, level),
                _ => describe_intersection_itself(self, types, level),
            },
            Type::Accessory(a) => a.describe(level),
            Type::Conditional(c) => {
                let subject = match &c.subject {
                    Subject::Parameter(p) => p.as_bytes().to_vec(),
                    Subject::Type(t) => t.describe_bytes(level),
                };
                description!(
                    "(",
                    subject,
                    " ",
                    if c.negated { "is not" } else { "is" },
                    " ",
                    c.target.describe_bytes(level),
                    " ? ",
                    c.then.describe_bytes(level),
                    " : ",
                    c.otherwise.describe_bytes(level),
                    ")"
                )
            }
            Type::Template(t) => {
                let mut basic = t.name.as_bytes().to_vec();
                if !t.bound.is_implicit_or_explicit_plain_mixed() {
                    basic.extend_from_slice(b" of ");
                    basic.extend(t.bound.describe_bytes(level));
                }
                if let Some(default) = &t.default {
                    basic.extend_from_slice(b" = ");
                    basic.extend(default.describe_bytes(level));
                }
                if level == Level::Precise {
                    description!(
                        basic,
                        " (",
                        t.scope,
                        ", ",
                        if t.argument { "argument" } else { "parameter" },
                        ")"
                    )
                } else {
                    basic
                }
            }
            Type::ObjectShape(props) => {
                let items: Vec<_> = props
                    .iter()
                    .map(|(name, ty, optional)| {
                        description!(
                            name,
                            if *optional { "?" } else { "" },
                            ": ",
                            ty.describe_bytes(level)
                        )
                    })
                    .collect();
                description!("object{", items.join(b", ".as_slice()), "}")
            }
        }
    }

    /// A template bound that `TemplateTypeTrait::describe()` leaves out (`MixedType` without subtraction).
    fn is_implicit_or_explicit_plain_mixed(&self) -> bool {
        matches!(self, Type::Mixed { .. })
    }

    /// `generalize(GeneralizePrecision::lessSpecific())`
    pub fn generalize(&self) -> Type {
        match self {
            Type::ConstInt(_) | Type::IntRange(..) => Type::Int,
            Type::ConstFloat(_) => Type::Float,
            Type::ConstBool(_) => Type::Bool,
            Type::ConstString(_) => Type::String,
            Type::Accessory(a) if a.is_string_accessory() => Type::String,
            Type::ConstArray(a) => a.generalize(),
            Type::EnumCase(class, _) => Type::Object(class.clone(), vec![], false),
            _ => self.traverse(&Type::generalize),
        }
    }

    /// `traverse()`: the same type with `f` applied to its direct inner types.
    pub fn traverse(&self, f: &dyn Fn(&Type) -> Type) -> Type {
        self.map_children(f)
    }

    /// Traverse direct children without guessing source retention from structural equality.
    pub fn traverse_source(&self, f: &dyn Fn(&Type) -> Traversal) -> Traversal {
        let replaced = std::cell::Cell::new(false);
        let mapped = self.map_children(&|inner| match f(inner) {
            Traversal::Retain => inner.clone(),
            Traversal::Replace(ty) => {
                replaced.set(true);
                ty
            }
        });
        if replaced.get() { Traversal::Replace(mapped) } else { Traversal::Retain }
    }

    fn map_children(&self, f: &dyn Fn(&Type) -> Type) -> Type {
        let sig = |s: &Signature| Signature {
            params: s
                .params
                .iter()
                .map(|p| Param {
                    ty: f(&p.ty),
                    ..p.clone()
                })
                .collect(),
            ret: f(&s.ret),
            templates: s.templates.clone(),
        };
        match self {
            Type::Reasoned(t, reason) => f(t).with_unresolvable_reason(reason.clone()),
            Type::SubtractedObject { object, excluded } => Type::SubtractedObject {
                object: Box::new(f(object)),
                excluded: Box::new(f(excluded)),
            },
            Type::LateArrayShape(s) => s.traverse(f),
            Type::Array(k, v) => Type::array(f(k), f(v)),
            Type::Accessory(Accessory::HasOffsetValue(k, v)) => {
                Type::Accessory(Accessory::HasOffsetValue(k.clone(), Box::new(f(v))))
            }
            Type::Accessory(Accessory::ModelProperty(t)) => Type::Accessory(Accessory::ModelProperty(Box::new(f(t)))),
            Type::Iterable(k, v) => Type::Iterable(Box::new(f(k)), Box::new(f(v))),
            Type::Union(types) => union(types.iter().map(f).collect()),
            Type::Benevolent(types) => to_benevolent(union(types.iter().map(f).collect())),
            Type::Intersection(types) => {
                let mapped: Vec<Type> = types.iter().map(f).collect();
                if mapped == *types {
                    Type::Intersection(mapped)
                } else {
                    intersect(mapped)
                }
            }
            Type::Callable(Some(s)) => Type::Callable(Some(Box::new(sig(s)))),
            Type::Closure(s) => Type::Closure(Box::new(sig(s))),
            Type::Object(class, args, exact) if !args.is_empty() => {
                Type::Object(class.clone(), args.iter().map(f).collect(), *exact)
            }
            Type::Projection(c, t) => Type::Projection(*c, Box::new(f(t))),
            Type::OffsetAccess(t, o) => Type::OffsetAccess(Box::new(f(t)), Box::new(f(o))),
            Type::ClassString(Some(g)) => Type::ClassString(Some(Box::new(f(g)))),
            Type::Static(class, args) if !args.is_empty() => {
                Type::Static(class.clone(), args.iter().map(f).collect())
            }
            Type::Template(t) => {
                let mut bound = f(&t.bound);
                let default = t.default.as_ref().map(f);
                // TemplateTypeTrait::traverse() rebuilds through TemplateTypeFactory only when changed.
                // The factory has no StrictMixedType case: that bound falls back to explicit MixedType.
                if matches!(bound, Type::StrictMixed) && (bound != t.bound || default != t.default) {
                    bound = Type::Mixed { explicit: true };
                }
                Type::Template(Box::new(Template { bound, default, ..(**t).clone() }))
            }
            Type::Conditional(c) => Type::Conditional(Box::new(Conditional {
                subject: match &c.subject {
                    Subject::Type(t) => Subject::Type(f(t)),
                    p => p.clone(),
                },
                target: f(&c.target),
                then: f(&c.then),
                otherwise: f(&c.otherwise),
                negated: c.negated,
            })),
            // `ConstantArrayType::traverse()`: the values and the unsealed `...<K, V>`, not the keys
            Type::ConstArray(a) => {
                let mut a = a.clone();
                a.values = a.values.iter().map(f).collect();
                a.unsealed = a.unsealed.map(|(k, v)| (f(&k), f(&v)));
                Type::ConstArray(a)
            }
            Type::ObjectShape(props) => Type::ObjectShape(
                props
                    .iter()
                    .map(|(n, t, o)| (n.clone(), f(t), *o))
                    .collect(),
            ),
            _ => self.clone(),
        }
    }
}

impl Accessory {
    fn describe(&self, level: Level) -> Vec<u8> {
        match self {
            Accessory::NonEmptyString => "non-empty-string",
            Accessory::NonFalsyString => "non-falsy-string",
            Accessory::NumericString => "numeric-string",
            Accessory::LiteralString => "literal-string",
            Accessory::LowercaseString => "lowercase-string",
            Accessory::UppercaseString => "uppercase-string",
            Accessory::DecimalIntString { inverse: false } => "decimal-int-string",
            Accessory::DecimalIntString { inverse: true } => "non-decimal-int-string",
            Accessory::List => "list",
            Accessory::NonEmptyArray => "non-empty-array",
            Accessory::HasMethod(name) => return format!("hasMethod({name})").into_bytes(),
            Accessory::ModelProperty(t) => return description!("model property of ", t.describe_bytes(level)),
            Accessory::ViewString => "view-string",
            Accessory::HasOffsetValue(key, value) => {
                return description!(
                    "hasOffsetValue(",
                    key.describe_bytes(level),
                    ", ",
                    value.describe_bytes(level),
                    ")"
                );
            }
        }
        .into()
    }

    fn is_string_accessory(&self) -> bool {
        !matches!(
            self,
            Accessory::List | Accessory::NonEmptyArray | Accessory::HasMethod(_) | Accessory::HasOffsetValue(..)
        )
    }
}

/// `StaticType::getStaticObjectType()`: the class, with its own templates as arguments when it is generic.
fn static_object(class: &str) -> Type {
    // a class the lookup does not know (outside `declared::with_classes()`) has no templates to pass on
    let Some(c) = symbols::lookup(class) else {
        return Type::Object(class.to_string(), Vec::new(), false);
    };
    let args = c
        .templates
        .iter()
        .map(|t| {
            Type::Template(Box::new(Template {
                name: t.name.clone(),
                scope: format!("class {}", c.name),
                bound: crate::resolve::resolve_str(&t.bound, Some(&c.name)),
                default: t
                    .default
                    .as_ref()
                    .map(|d| crate::resolve::resolve_str(d, Some(&c.name))),
                argument: true,
            }))
        })
        .collect();
    Type::Object(c.name.clone(), args, false)
}

/// `getStaticObjectType()` of a `static` (`GenericStaticType`'s is the class with its arguments).
fn static_of(class: &str, args: &[Type]) -> Type {
    if args.is_empty() {
        static_object(class)
    } else {
        Type::Object(class.to_string(), args.to_vec(), false)
    }
}

/// `getIterableKeyType()` (`key`) or `getIterableValueType()` of a class with these template arguments (an error
/// for a class that is not `Traversable`).
fn iterable_arg(class: &str, args: &[Type], key: bool) -> Type {
    let Some(c) = symbols::lookup(class) else {
        return Type::Error;
    };
    c.iterable
        .as_ref()
        .map(|(k, v)| if key { k } else { v })
        .map_or(Type::Error, |s| {
            with_class_args(&crate::resolve::resolve_str(s, Some(&c.name)), &c, args)
        })
}

/// The type arguments `c<args>` gives its generic ancestor `ancestor` (`getAncestorWithClassName()`), `None` when
/// `c` does not name them.
pub fn ancestor_args(c: &symbols::Class, ancestor: &str, args: &[Type]) -> Option<Vec<Type>> {
    let generic = c.generics.get(ancestor)?;
    Some(
        generic
            .iter()
            .map(|s| with_class_args(&crate::resolve::resolve_str(s, Some(&c.name)), c, args))
            .collect(),
    )
}

/// `t` with the class's own templates replaced by its arguments (by their bounds when missing).
fn with_class_args(t: &Type, c: &symbols::Class, args: &[Type]) -> Type {
    if let Type::Template(tpl) = t
        && tpl.scope == format!("class {}", c.name)
        && let Some(i) = c.templates.iter().position(|x| x.name == tpl.name)
    {
        return match args.get(i) {
            // the bound (a `mixed` one implicit, as `ObjectType::getIterableKeyType()` resolves it)
            Some(Type::Wildcard) | None if matches!(tpl.bound, Type::Mixed { .. }) => mixed(),
            Some(Type::Wildcard) | None => tpl.bound.clone(),
            // ponytail: a call-site variance is not applied to the position (`resolveTemplateTypes()`'s
            // call-site variance map); the argument stands for its template
            Some(Type::Projection(_, arg)) => (**arg).clone(),
            Some(arg) => arg.clone(),
        };
    }
    t.traverse(&|x| with_class_args(x, c, args))
}

/// `ObjectType::isInstanceOf()`
fn is_instance_of(class: &str, target: &str) -> Tri {
    match symbols::lookup(class) {
        Some(c) if c.name == target || c.ancestors.iter().any(|a| a == target) => Tri::Yes,
        Some(c) if c.is_interface => Tri::Maybe,
        Some(_) => Tri::No,
        None => Tri::Maybe,
    }
}

/// `ConstantStringType::isCallable()`: a function's name, or `Class::method`.
fn string_is_callable(bytes: &[u8]) -> Tri {
    let s = String::from_utf8_lossy(bytes);
    let s = s.as_ref();
    if s.is_empty() {
        return Tri::No;
    }
    if symbols::has_function(s) {
        return Tri::Yes;
    }
    let Some((class, method)) = s.split_once("::") else {
        return Tri::No;
    };
    // `#^([a-zA-Z_\x7f-\xff\\][a-zA-Z0-9_\x7f-\xff\\]*)::([a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*)\z#`
    let word = |w: &str, backslash: bool| {
        let ok = |ch: char| ch == '_' || !ch.is_ascii() || ch.is_ascii_alphanumeric() || (backslash && ch == '\\');
        w.chars().next().is_some_and(|ch| ok(ch) && !ch.is_ascii_digit()) && w.chars().all(ok)
    };
    if !word(class, true) || !word(method, false) {
        return Tri::No;
    }
    match symbols::lookup(class) {
        None => Tri::Maybe,
        Some(c) if has_method(&c, &method.to_ascii_lowercase()) => Tri::Yes,
        Some(c) if !c.is_final => Tri::Maybe,
        Some(_) => Tri::No,
    }
}

/// `ClassReflection::hasNativeMethod()`: declared or inherited.
fn has_method(c: &symbols::Class, name: &str) -> bool {
    std::iter::once(c.name.as_str())
        .chain(c.ancestors.iter().map(String::as_str))
        .any(|n| symbols::lookup(n).is_some_and(|a| a.methods.contains_key(name)))
}

fn canonical_class(class: &str) -> String {
    symbols::lookup(class).map_or_else(|| class.to_string(), |c| c.name.clone())
}

/// `ConstantFloatType::castFloatToString()` (precision -1).
pub fn float_to_string(f: f64) -> String {
    if f.is_nan() {
        return "NAN".to_string();
    }
    if f.is_infinite() {
        return (if f > 0.0 { "INF" } else { "-INF" }).to_string();
    }
    let s = format!("{f:?}");
    let s = s.strip_suffix(".0").map_or(s.clone(), |x| format!("{x}.0"));
    if let Some((mantissa, exp)) = s.split_once('e') {
        let exp: i32 = exp.parse().expect("exponent");
        let mantissa = if mantissa.contains('.') {
            mantissa.to_string()
        } else {
            format!("{mantissa}.0")
        };
        return format!(
            "{mantissa}E{}{}",
            if exp < 0 { "-" } else { "+" },
            exp.abs()
        );
    }
    s
}

/// `ConstantStringType::export()`
fn export_string(value: &[u8]) -> Vec<u8> {
    let double = value.iter().any(|b| *b < 0x20);
    let quote = if double { b'"' } else { b'\'' };
    let mut out = vec![quote];
    for &byte in value {
        match byte {
            b'\\' => out.extend_from_slice(b"\\\\"),
            b if b == quote => out.extend_from_slice(&[b'\\', b]),
            b'\n' if double => out.extend_from_slice(b"\\n"),
            b'\t' if double => out.extend_from_slice(b"\\t"),
            b'\r' if double => out.extend_from_slice(b"\\r"),
            0x0b if double => out.extend_from_slice(b"\\v"),
            0x0c if double => out.extend_from_slice(b"\\f"),
            0x07 if double => out.extend_from_slice(b"\\a"),
            0x08 if double => out.extend_from_slice(b"\\b"),
            b if b < 0x20 => out.extend_from_slice(format!("\\{b:03o}").as_bytes()),
            b => out.push(b),
        }
    }
    out.push(quote);
    out
}

fn display_bytes(out: &[u8]) -> String {
    // Only the public presentation boundary needs UTF-8; nested descriptions retain bytes.
    // json_encode(JSON_INVALID_UTF8_SUBSTITUTE) groups malformed sequences like php_next_utf8_char,
    // not Rust's UTF-8 decoder: a surrogate or overlong three-byte sequence gets one replacement.
    let mut display = String::new();
    let mut bytes = out;
    while !bytes.is_empty() {
        match std::str::from_utf8(bytes) {
            Ok(valid) => {
                display.push_str(valid);
                break;
            }
            Err(error) => {
                let valid = error.valid_up_to();
                display.push_str(std::str::from_utf8(&bytes[..valid]).unwrap());
                bytes = &bytes[valid..];
                let width = match bytes[0] {
                    0xc2..=0xdf => 2,
                    0xe0..=0xef => 3,
                    0xf0..=0xf4 => 4,
                    _ => 1,
                };
                let consumed = 1 + bytes
                    .iter()
                    .skip(1)
                    .take(width - 1)
                    .take_while(|&&byte| !(byte < 0x80 || (0xc2..=0xf4).contains(&byte)))
                    .count();
                display.push('�');
                bytes = &bytes[consumed..];
            }
        }
    }
    display
}

/// `UnionTypeHelper::sortTypes()`
pub fn sort_types(types: &[Type]) -> Vec<Type> {
    if types.len() > 1024 {
        return types.to_vec();
    }
    sort::types(types)
}

fn compare_strings(a: impl AsRef<[u8]>, b: impl AsRef<[u8]>) -> Ordering {
    let (a, b) = (a.as_ref(), b.as_ref());
    let cmp = a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase());
    if cmp != Ordering::Equal {
        cmp
    } else {
        a.cmp(b)
    }
}

fn compare_in_union(a: &Type, b: &Type) -> Ordering {
    let a_class = a.scalar_class_type();
    let b_class = b.scalar_class_type();
    if matches!(a_class, Type::Null) {
        return Ordering::Greater;
    } else if matches!(b_class, Type::Null) {
        return Ordering::Less;
    }
    match (a, b) {
        (Type::Accessory(_), Type::Accessory(_)) => {
            return compare_strings(
                a.describe_bytes(Level::Value),
                b.describe_bytes(Level::Value),
            );
        }
        (Type::Accessory(_), _) => return Ordering::Greater,
        (_, Type::Accessory(_)) => return Ordering::Less,
        _ => {}
    }
    let a_bool = matches!(a, Type::ConstBool(_));
    let b_bool = matches!(b, Type::ConstBool(_));
    if a_bool != b_bool {
        return a_bool.cmp(&b_bool);
    }
    let constant_class = |t: &Type| {
        matches!(
            t,
            Type::ConstBool(_) | Type::ConstInt(_) | Type::ConstFloat(_) | Type::ConstString(_)
        )
    };
    let a_scalar = constant_class(a_class);
    let b_scalar = constant_class(b_class);
    if a_scalar != b_scalar {
        return b_scalar.cmp(&a_scalar);
    }
    let number = |t: &Type| match t.scalar_class_type() {
        Type::ConstInt(i) => Some(*i as f64),
        Type::ConstFloat(f) => Some(*f),
        _ => None,
    };
    if let (Some(x), Some(y)) = (number(a), number(b)) {
        let cmp = x.partial_cmp(&y).unwrap_or(Ordering::Equal);
        if cmp != Ordering::Equal {
            return cmp;
        }
        // an int before the equal float (two equal numbers of one kind are one type)
        return matches!(b_class, Type::ConstInt(_)).cmp(&matches!(a_class, Type::ConstInt(_)));
    }
    match (a_class, b_class) {
        (Type::IntRange(a_min, _), Type::IntRange(b_min, _)) => {
            return a_min.unwrap_or(i64::MIN).cmp(&b_min.unwrap_or(i64::MIN));
        }
        (Type::IntRange(..), Type::Int | Type::ConstInt(_)) => return Ordering::Greater,
        (Type::Int | Type::ConstInt(_), Type::IntRange(..)) => return Ordering::Less,
        (Type::ConstString(x), Type::ConstString(y)) => {
            return x
                .to_ascii_lowercase()
                .cmp(&y.to_ascii_lowercase())
                .then_with(|| x.cmp(y));
        }
        _ => {}
    }
    let callable = |t: &Type| matches!(t, Type::Callable(_) | Type::Closure(_));
    if callable(a) && callable(b) {
        return compare_strings(
            a.describe_bytes(Level::Value),
            b.describe_bytes(Level::Value),
        );
    }
    if a.is_constant_array().yes() && b.is_constant_array().yes() {
        // `array{}` first
        let empty = |t: &Type| t.is_iterable_at_least_once().no();
        return empty(b).cmp(&empty(a)).then_with(|| {
            compare_strings(
                a.describe_bytes(Level::Value),
                b.describe_bytes(Level::Value),
            )
        });
    }
    if a.is_string().yes() && b.is_string().yes() {
        return compare_strings(
            a.describe_bytes(Level::Precise),
            b.describe_bytes(Level::Precise),
        );
    }
    compare_strings(
        a.describe_bytes(Level::TypeOnly),
        b.describe_bytes(Level::TypeOnly),
    )
}

impl Type {
    /// TypeCombinator's array/iterable instanceof checks include the corresponding Template* subclasses.
    /// Only dispatch uses this view: supertype reductions must still compare the original template identities.
    fn array_class(&self) -> &Type {
        match self {
            Type::Template(t)
                if matches!(
                    t.bound,
                    Type::Array(..) | Type::ConstArray(_) | Type::Iterable(..)
                ) =>
            {
                &t.bound
            }
            _ => self,
        }
    }
}

fn describe_union(types: &[Type], level: Level) -> Vec<u8> {
    let join = |types: &[Type]| -> Vec<u8> {
        let mut names = Vec::new();
        for (i, t) in types.iter().enumerate() {
            let d = t.describe_bytes(level);
            let wrap = match t {
                Type::Closure(_) | Type::Callable(_) => true,
                Type::Template(_) if matches!(t.scalar_class_type(), Type::Union(_)) => true,
                Type::Template(tpl) => {
                    i + 1 < types.len()
                        && level != Level::Precise
                        && !matches!(tpl.bound, Type::Mixed { .. })
                }
                Type::Intersection(_) => d.contains(&b'&'),
                _ => false,
            };
            names.push(if wrap { description!("(", d, ")") } else { d });
        }
        // UnionType::describe(): number every occurrence of a duplicated precise description.
        if level == Level::Precise {
            let mut counts = std::collections::HashMap::new();
            for name in &names {
                *counts.entry(name.clone()).or_insert(0) += 1;
            }
            let mut indices = std::collections::HashMap::new();
            for name in &mut names {
                if counts[name] > 1 {
                    let index = indices.entry(name.clone()).or_insert(0);
                    *index += 1;
                    name.extend(format!("#{index}").into_bytes());
                }
            }
        } else {
            let mut seen = std::collections::HashSet::new();
            names.retain(|n| seen.insert(n.clone()));
        }
        if names.len() > 1024 {
            names.truncate(1024);
            names.push("…".into());
        }
        names.join(b"|".as_slice())
    };
    let sorted = sort_types(types);
    if level != Level::TypeOnly {
        return join(&sorted);
    }
    let generalized = union(
        sorted
            .iter()
            .map(|t| {
                if t.is_constant_value().yes() && !matches!(t, Type::ConstBool(_)) {
                    match t.scalar_class_type() {
                        scalar @ (Type::ConstString(_) | Type::ConstInt(_)) => scalar.generalize(),
                        Type::Bool | Type::String | Type::Int if matches!(t, Type::Template(_)) => {
                            t.clone()
                        }
                        _ => t.generalize(),
                    }
                } else {
                    t.clone()
                }
            })
            .collect(),
    );
    match generalized {
        Type::Union(types) => join(&sort_types(&types)),
        other => join(&[other]),
    }
}

/// Larastan's string types are StringType subclasses with their own description.
fn larastan_string(types: &[Type]) -> Option<&Accessory> {
    types.iter().find_map(|t| match t {
        Type::Accessory(a @ (Accessory::ModelProperty(_) | Accessory::ViewString)) => Some(a),
        _ => None,
    })
}

fn describe_intersection_type(this: &Type, types: &[Type], level: Level) -> Vec<u8> {
    if let Some(a) = larastan_string(types) {
        return a.describe(level);
    }
    let is_list = this.is_list().yes();
    let mut names = Vec::new();
    let mut value_type = None;
    for t in sort_types(types) {
        if is_list {
            if matches!(t, Type::Array(..) | Type::ConstArray(_)) {
                value_type = Some(t.iterable_value_type());
                continue;
            }
            if matches!(t, Type::Accessory(Accessory::NonEmptyArray)) {
                continue;
            }
        }
        if matches!(t, Type::Accessory(_)) {
            continue;
        }
        names.push(t.generalize().describe_bytes(level));
    }
    if is_list {
        let inner = match value_type {
            Some(v) if !v.is_plain_implicit_mixed() => {
                description!("<", v.describe_bytes(level), ">")
            }
            _ => Vec::new(),
        };
        names.push(description!("list", inner));
    }
    names.sort_by(|a, b| compare_strings(a, b));
    names.join(b"&".as_slice())
}

fn describe_intersection_itself(this: &Type, types: &[Type], level: Level) -> Vec<u8> {
    if let Some(a) = larastan_string(types) {
        return a.describe(level);
    }
    let mut base: Vec<(usize, &Type)> = Vec::new();
    let mut to_describe: Vec<(usize, Type)> = Vec::new();
    let mut skip_names: Vec<&[u8]> = Vec::new();
    let is_list = this.is_list().yes();
    let is_array = this.is_array().yes();
    let is_non_empty_array = this.is_iterable_at_least_once().yes();
    // IntersectionType::describeItself(): a template array describes itself, so its refinements cannot
    // disappear into the ordinary array/list prefix that is only emitted for concrete ArrayType members.
    let has_template_array = types
        .iter()
        .any(|t| matches!(t, Type::Template(t) if matches!(t.bound, Type::Array(..))));
    let mut described = Vec::new();
    let sorted = sort_types(types);
    for (i, t) in sorted.iter().enumerate() {
        if level != Level::Precise
            && matches!(
                t,
                Type::Accessory(Accessory::HasMethod(_) | Accessory::HasOffsetValue(..))
            )
        {
            continue;
        }
        if let Type::Accessory(a) = t
            && a.is_string_accessory()
        {
            if matches!(a, Accessory::LowercaseString | Accessory::UppercaseString)
                && level != Level::Precise
            {
                continue;
            }
            to_describe.push((i, t.clone()));
            skip_names.push(b"string");
            continue;
        }
        if is_list || is_array {
            match t {
                Type::Array(key, value) => {
                    if is_list {
                        let inner = if value.is_plain_implicit_mixed() {
                            Vec::new()
                        } else {
                            description!("<", value.describe_bytes(level), ">")
                        };
                        described.push((
                            i,
                            description!(
                                if is_non_empty_array {
                                    "non-empty-list"
                                } else {
                                    "list"
                                },
                                inner
                            ),
                        ));
                    } else {
                        let inner = if !key.is_plain_implicit_mixed() {
                            description!(
                                "<",
                                key.describe_bytes(level),
                                ", ",
                                value.describe_bytes(level),
                                ">"
                            )
                        } else if !value.is_plain_implicit_mixed() {
                            description!("<", value.describe_bytes(level), ">")
                        } else {
                            Vec::new()
                        };
                        let kind = if is_non_empty_array {
                            "non-empty-array"
                        } else {
                            "array"
                        };
                        described.push((i, description!(kind, inner)));
                    }
                    continue;
                }
                Type::ConstArray(a) => {
                    let d = a.describe(level);
                    let rest = d
                        .strip_prefix(b"list")
                        .unwrap_or_else(|| &d["array".len()..]);
                    let mut begin = String::from(if is_list { "list" } else { "array" });
                    if is_non_empty_array && !t.is_iterable_at_least_once().yes() {
                        begin = format!("non-empty-{begin}");
                    }
                    described.push((i, description!(begin, rest)));
                    continue;
                }
                Type::Accessory(Accessory::NonEmptyArray | Accessory::List) => {
                    if has_template_array {
                        described.push((i, t.describe_bytes(level)));
                    }
                    continue;
                }
                _ => {}
            }
        }
        if matches!(t, Type::Callable(None)) {
            to_describe.push((i, t.clone()));
            skip_names.push(b"object");
            skip_names.push(b"string");
            continue;
        }
        // Every accessory was handled above (string ones always, array ones as `is_array`).
        base.push((i, t));
    }
    'base: for (i, t) in base {
        let d = t.describe_bytes(level);
        if (d == b"object" || d == b"string")
            && skip_names.contains(&d.as_slice())
            && let Some(pos) = to_describe
                .iter()
                .position(|(_, x)| matches!(x, Type::Callable(None)))
        {
            described.push((i, description!("callable-", d)));
            to_describe.remove(pos);
            continue 'base;
        }
        if skip_names.contains(&d.as_slice()) {
            continue;
        }
        described.push((i, d));
    }
    for (i, t) in to_describe {
        described.push((i, t.describe_bytes(level)));
    }
    described.sort_by_key(|(i, _)| *i);
    described
        .into_iter()
        .map(|(_, d)| d)
        .collect::<Vec<_>>()
        .join(b"&".as_slice())
}

/// `Printer::print()` of `CallableType::toPhpDocNode()` / `ClosureType` with parameter names dropped.
fn print_callable(ident: &str, sig: &Signature) -> Vec<u8> {
    let params: Vec<_> = sig
        .params
        .iter()
        .map(|p| {
            description!(
                to_phpdoc_bytes(&p.ty),
                if p.variadic { " ..." } else { "" },
                if p.optional && !p.variadic { "=" } else { "" }
            )
        })
        .collect();
    let ret = to_phpdoc_bytes(&sig.ret);
    let ret = if matches!(
        sig.ret,
        Type::Union(_)
            | Type::Benevolent(_)
            | Type::Intersection(_)
            | Type::Callable(Some(_))
            | Type::Closure(_)
    ) && ret.iter().any(|b| b"|&(".contains(b))
    {
        description!("(", ret, ")")
    } else {
        ret
    };
    description!(
        ident,
        print_templates(sig),
        "(",
        params.join(b", ".as_slice()),
        "): ",
        ret
    )
}

/// A callable's `<T of Bound = Default, ...>` (its TemplateTagValueNodes, the bound always printed).
fn print_templates(sig: &Signature) -> Vec<u8> {
    if sig.templates.is_empty() {
        return Vec::new();
    }
    let tags: Vec<_> = sig
        .templates
        .iter()
        // (a TemplateTagValueNode of the bound only)
        .map(|t| description!(t.name, " of ", to_phpdoc_bytes(&t.bound)))
        .collect();
    description!("<", tags.join(b", ".as_slice()), ">")
}

/// `Printer::print($type->toPhpDocNode())` up to PHPStan 2.3.3 (what Rector's PHPStan prints): class names as written.
pub fn to_phpdoc(t: &Type) -> String {
    display_bytes(&to_phpdoc_bytes(t))
}

thread_local! {
    static NAME_NODES: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// `Printer::print($type->toPhpDocNode())` from PHPStan 2.3.4 on: object, generic object, `Closure` and enum case
/// types are fully qualified name nodes, printed with a leading backslash.
pub fn to_phpdoc_fully_qualified(t: &Type) -> String {
    NAME_NODES.set(true);
    let printed = display_bytes(&to_phpdoc_bytes(t));
    NAME_NODES.set(false);
    printed
}

/// A class name as `toPhpDocNode()` prints it.
fn phpdoc_class_name(class: &str) -> Vec<u8> {
    if NAME_NODES.get() {
        description!("\\", class)
    } else {
        class.as_bytes().to_vec()
    }
}

fn to_phpdoc_bytes(t: &Type) -> Vec<u8> {
    match t {
        Type::Reasoned(t, _) => to_phpdoc_bytes(t),
        Type::LateArrayShape(s) => s.print(true),
        Type::Accessory(Accessory::HasMethod(_) | Accessory::HasOffsetValue(..)) => Vec::new(),
        Type::Intersection(types)
            if types
                .iter()
                .any(|t| matches!(t, Type::Accessory(Accessory::HasOffsetValue(..)))) =>
        {
            to_phpdoc_bytes(&intersect(
                types
                    .iter()
                    .filter(|t| !matches!(t, Type::Accessory(Accessory::HasOffsetValue(..))))
                    .cloned()
                    .collect(),
            ))
        }
        Type::Mixed { .. } | Type::StrictMixed | Type::SubtractedMixed { .. } | Type::Error => {
            "mixed".into()
        }
        Type::Never(_) => "never".into(),
        Type::ConstInt(i) => i.to_string().into_bytes(),
        Type::ConstFloat(f) => float_to_string(*f).into_bytes(),
        Type::ConstString(s) => {
            let mut out = vec![b'\''];
            for &byte in s {
                if matches!(byte, b'\\' | b'\'') {
                    out.push(b'\\');
                }
                out.push(byte);
            }
            out.push(b'\'');
            out
        }
        Type::IntRange(min, max) => format!(
            "int<{}, {}>",
            min.map_or("min".to_string(), |v| v.to_string()),
            max.map_or("max".to_string(), |v| v.to_string())
        )
        .into_bytes(),
        Type::ClassString(Some(g)) => description!("class-string<", to_phpdoc_bytes(g), ">"),
        Type::Object(class, args, _) if !args.is_empty() => {
            description!(
                phpdoc_class_name(class),
                "<",
                args.iter()
                    .map(to_phpdoc_bytes)
                    .collect::<Vec<_>>()
                    .join(b", ".as_slice()),
                ">"
            )
        }
        Type::Object(class, _, _) => phpdoc_class_name(class),
        Type::EnumCase(class, case) if NAME_NODES.get() => description!("\\", class, "::", case),
        Type::Static(..) => "static".into(),
        Type::This(_) => "$this".into(),
        Type::Array(key, item) => {
            if key.is_plain_implicit_mixed() {
                if item.is_plain_implicit_mixed() {
                    return "array".into();
                }
                return description!("array<", to_phpdoc_bytes(item), ">");
            }
            description!(
                "array<",
                to_phpdoc_bytes(key),
                ", ",
                to_phpdoc_bytes(item),
                ">"
            )
        }
        Type::ConstArray(array) => array.to_phpdoc(),
        Type::Iterable(key, item) => {
            if matches!(**key, Type::Mixed { .. }) {
                if matches!(**item, Type::Mixed { .. }) {
                    return "iterable".into();
                }
                return description!("iterable<", to_phpdoc_bytes(item), ">");
            }
            description!(
                "iterable<",
                to_phpdoc_bytes(key),
                ", ",
                to_phpdoc_bytes(item),
                ">"
            )
        }
        Type::Union(types) | Type::Benevolent(types) => join_phpdoc(&sort_types(types), "|"),
        // `list<T>`, `non-empty-list<T>`, `non-empty-array<K, T>`
        Type::Intersection(types)
            if let Some(Type::Array(key, item)) =
                types.iter().find(|t| matches!(t, Type::Array(..))) =>
        {
            let has = |a: Accessory| types.contains(&Type::Accessory(a));
            match (
                types.len(),
                has(Accessory::List),
                has(Accessory::NonEmptyArray),
            ) {
                (2, true, false) if !item.is_plain_implicit_mixed() => {
                    description!("list<", to_phpdoc_bytes(item), ">")
                }
                (3, true, true) if !item.is_plain_implicit_mixed() => {
                    description!("non-empty-list<", to_phpdoc_bytes(item), ">")
                }
                (2, false, true)
                    if !key.is_plain_implicit_mixed() && !item.is_plain_implicit_mixed() =>
                {
                    description!(
                        "non-empty-array<",
                        to_phpdoc_bytes(key),
                        ", ",
                        to_phpdoc_bytes(item),
                        ">"
                    )
                }
                _ => t.describe_bytes(Level::Precise),
            }
        }
        Type::Intersection(types)
            if types
                .iter()
                .any(|t| matches!(t, Type::Accessory(Accessory::HasMethod(_)))) =>
        {
            let printable: Vec<_> = types
                .iter()
                .filter(|t| !matches!(t, Type::Accessory(Accessory::HasMethod(_))))
                .cloned()
                .collect();
            to_phpdoc_bytes(&intersect(printable))
        }
        Type::Intersection(types)
            if types.iter().any(|t| {
                matches!(t, Type::StrictMixed)
                    || matches!(t, Type::Template(t) if matches!(t.bound, Type::StrictMixed))
            }) =>
        {
            join_phpdoc(&sort_types(types), "&")
        }
        Type::Intersection(_) => t.describe_bytes(Level::Precise),
        Type::Template(tpl) => tpl.name.as_bytes().to_vec(),
        Type::Callable(None) => "callable".into(),
        Type::Callable(Some(sig)) => print_callable_named("callable", sig),
        Type::Closure(sig) if NAME_NODES.get() => print_callable_named("\\Closure", sig),
        Type::Closure(sig) => print_callable_named("Closure", sig),
        Type::Projection(c, inner) => description!(variance_name(*c), " ", to_phpdoc_bytes(inner)),
        Type::OffsetAccess(t, o) => {
            let on = to_phpdoc_bytes(t);
            // (Printer's parentheses around a union, intersection or callable)
            let wrap = matches!(
                **t,
                Type::Union(_) | Type::Benevolent(_) | Type::Callable(Some(_)) | Type::Closure(_)
            ) || (matches!(**t, Type::Intersection(_)) && on.contains(&b'&'));
            if wrap {
                description!("(", on, ")[", to_phpdoc_bytes(o), "]")
            } else {
                description!(on, "[", to_phpdoc_bytes(o), "]")
            }
        }
        _ => t.describe_bytes(Level::Precise),
    }
}

fn join_phpdoc(types: &[Type], sep: &str) -> Vec<u8> {
    types
        .iter()
        .map(|t| {
            let s = to_phpdoc_bytes(t);
            if matches!(
                t,
                Type::Union(_) | Type::Benevolent(_) | Type::Intersection(_)
            ) && s.iter().any(|b| b"|&".contains(b))
            {
                description!("(", s, ")")
            } else {
                s
            }
        })
        .collect::<Vec<_>>()
        .join(sep.as_bytes())
}

fn print_callable_named(ident: &str, sig: &Signature) -> Vec<u8> {
    let params: Vec<_> = sig
        .params
        .iter()
        .map(|p| {
            let name = if p.name.is_empty() {
                String::new()
            } else {
                format!("${}", p.name)
            };
            let s = description!(
                to_phpdoc_bytes(&p.ty),
                " ",
                if p.by_ref { "&" } else { "" },
                if p.variadic { "..." } else { "" },
                name
            );
            description!(s.trim_ascii(), if p.optional { "=" } else { "" })
        })
        .collect();
    let ret = match &sig.ret {
        r @ (Type::Union(_)
        | Type::Benevolent(_)
        | Type::Intersection(_)
        | Type::Callable(Some(_))
        | Type::Closure(_)) => description!("(", to_phpdoc_bytes(r), ")"),
        r => to_phpdoc_bytes(r),
    };
    description!(
        ident,
        print_templates(sig),
        "(",
        params.join(b", ".as_slice()),
        "): ",
        ret
    )
}

impl ConstArray {
    pub fn empty() -> ConstArray {
        ConstArray {
            keys: vec![],
            values: vec![],
            next_auto_indexes: vec![0],
            optional: vec![],
            is_list: Tri::Yes,
            unsealed: None,
        }
    }

    fn is_optional_key(&self, i: usize) -> bool {
        self.optional.contains(&i)
    }

    fn is_unsealed(&self) -> Tri {
        match &self.unsealed {
            None => Tri::Maybe,
            Some((key, _)) => Tri::from(*key != Type::Never(NeverKind::Explicit)),
        }
    }

    fn is_iterable_at_least_once(&self) -> Tri {
        if self.keys.is_empty() {
            return if self.is_unsealed().yes() { Tri::Maybe } else { Tri::No };
        }
        if self.optional.len() < self.keys.len() {
            Tri::Yes
        } else {
            Tri::Maybe
        }
    }

    /// The constant-array branch of `TypeCombinator::intersect()` against an `ArrayType`: each key the array may
    /// have, with its value intersected with the array's item type.
    fn intersect_array(&self, array: &Type) -> Type {
        let (array_key, item) = (array.iterable_key_type(), array.iterable_value_type());
        let mut b = ShapeBuilder::new();
        if self.is_unsealed().yes()
            && let Some((k, v)) = &self.unsealed
        {
            let k = intersect(vec![k.clone(), array_key.clone()]);
            let v = intersect(vec![v.clone(), item.clone()]);
            if !k.is_never() && !v.is_never() {
                b.make_unsealed(k, v);
            }
        }
        for (i, (key, value)) in self.keys.iter().zip(&self.values).enumerate() {
            // `ArrayType::hasOffsetValueType()` of a constant key: no when the key type excludes it, else maybe
            if is_super_type_of(&array_key, key).no() {
                continue;
            }
            b.set(
                Some(intersect(vec![key.clone(), array_key.clone()])),
                intersect(vec![value.clone(), item.clone()]),
                self.is_optional_key(i),
            );
        }
        b.get()
    }

    /// `isCallable()`: only a `[class-string|object, method]` shape (its missing slots possibly in the unsealed
    /// extras) can be; a sealed one answers from the class's methods (`doFindTypeAndMethodNames()`).
    // ponytail: an unsealed shape answers maybe; `supportsCallableInstanceMethods()` (PHP < 8) is not modelled
    fn is_callable(&self) -> Tri {
        if self.unsealed.is_none() || !self.is_unsealed().yes() {
            if let Some(t) = self.callable_methods() {
                return t;
            }
        }
        self.callable_shape()
    }

    /// The sealed `[class, methods]` shape's certainty; `None` when its class or methods are not constant.
    /// `[class, 'method']`: the class (and whether it is a class-string) and the constant method names.
    fn callable_parts(&self) -> Option<(&str, bool, Vec<&str>)> {
        if self.keys != [Type::ConstInt(0), Type::ConstInt(1)] {
            return None;
        }
        let methods: Vec<&str> = match &self.values[1] {
            Type::ConstString(m) => vec![std::str::from_utf8(m).ok()?],
            Type::Union(ts) => ts
                .iter()
                .map(|t| {
                    if let Type::ConstString(m) = t {
                        std::str::from_utf8(m).ok()
                    } else {
                        None
                    }
                })
                .collect::<Option<_>>()?,
            _ => return None,
        };
        match &self.values[0] {
            Type::ConstString(n) => Some((std::str::from_utf8(n).ok()?, true, methods)),
            Type::Object(n, _, _) | Type::Static(n, _) | Type::This(n) => Some((n, false, methods)),
            _ => None,
        }
    }

    fn callable_methods(&self) -> Option<Tri> {
        let (class, string, methods) = self.callable_parts()?;
        let class = symbols::lookup(class)?;
        // ponytail: PHP 8 (`supportsCallableInstanceMethods()` is false): a class-string's instance method is no callable
        let instance = |m: &str| {
            std::iter::once(class.name.as_str())
                .chain(class.ancestors.iter().map(String::as_str))
                .find_map(|n| symbols::lookup(n)?.methods.get(m).map(|f| !f.is_static))
                .unwrap_or(false)
        };
        let has: Vec<Tri> = methods
            .iter()
            .map(|m| m.to_ascii_lowercase())
            .filter(|m| !(string && has_method(&class, m) && instance(m)))
            .map(|m| match has_method(&class, &m) {
                true => Tri::Yes,
                false if class.is_final => Tri::No,
                false => Tri::Maybe,
            })
            .collect();
        if has.is_empty() {
            return Some(Tri::No);
        }
        let found: Vec<Tri> = has.iter().copied().filter(|t| !t.no()).collect();
        if found.is_empty() {
            return Some(Tri::No);
        }
        let optional = if self.optional.is_empty() { Tri::Yes } else { Tri::Maybe };
        let missing = if found.len() < has.len() { Tri::Maybe } else { Tri::Yes };
        Some(found.into_iter().fold(Tri::Yes, Tri::and).and(optional).and(missing))
    }

    fn callable_shape(&self) -> Tri {
        let unsealed = self.unsealed.as_ref().filter(|_| self.is_unsealed().yes());
        if self.keys.len() > 2
            || (unsealed.is_none() && self.keys.len() != 2)
            || self
                .keys
                .iter()
                .any(|k| !matches!(k, Type::ConstInt(0 | 1)))
        {
            return Tri::No;
        }
        let slot = |n: i64, expected: Type| {
            self.keys.contains(&Type::ConstInt(n))
                || unsealed.is_some_and(|(k, v)| {
                    !is_super_type_of(k, &Type::ConstInt(n)).no()
                        && !is_super_type_of(&expected, v).no()
                })
        };
        Tri::from(
            slot(
                0,
                Type::Union(vec![Type::ObjectWithoutClass, Type::ClassString(None)]),
            ) && slot(
                1,
                Type::Intersection(vec![
                    Type::String,
                    Type::Accessory(Accessory::NonFalsyString),
                ]),
            ),
        )
        .and(Tri::Maybe)
    }

    fn iterable_key_type(&self) -> Type {
        let mut key = match self.keys.len() {
            0 => Type::Never(NeverKind::Explicit),
            1 => self.keys[0].clone(),
            _ => Type::Union(self.keys.clone()),
        };
        if self.is_unsealed().yes()
            && let Some((unsealed_key, _)) = &self.unsealed
        {
            let k = if matches!(unsealed_key, Type::Mixed { .. }) {
                array_key()
            } else {
                unsealed_key.clone()
            };
            key = union(vec![key, k]);
        }
        key
    }

    fn iterable_value_type(&self) -> Type {
        let mut value = if self.values.is_empty() {
            Type::Never(NeverKind::Explicit)
        } else {
            union(self.values.clone())
        };
        if self.is_unsealed().yes()
            && let Some((_, v)) = &self.unsealed
        {
            value = union(vec![value, v.clone()]);
        }
        value
    }

    fn should_be_described_as_a_list(&self) -> bool {
        if !self.is_list.yes() || self.optional.is_empty() {
            return false;
        }
        self.optional.len() > 1 || self.optional[0] != self.keys.len() - 1
    }

    fn describe(&self, level: Level) -> Vec<u8> {
        let name = if self.should_be_described_as_a_list() {
            "list"
        } else {
            "array"
        };
        if level == Level::TypeOnly {
            if self.is_iterable_at_least_once().no() {
                return name.into();
            }
            let key = self.iterable_key_type();
            if key.is_benevolent_array_key() {
                return description!(
                    name,
                    "<",
                    self.iterable_value_type().describe_bytes(level),
                    ">"
                );
            }
            return description!(
                name,
                "<",
                key.describe_bytes(level),
                ", ",
                self.iterable_value_type().describe_bytes(level),
                ">"
            );
        }
        let truncate = level == Level::Value;
        let mut items = Vec::new();
        let mut values = Vec::new();
        let mut values_only = true;
        for (i, key) in self.keys.iter().enumerate() {
            if *key != Type::ConstInt(i as i64) {
                values_only = false;
            }
            let optional = self.is_optional_key(i);
            if optional {
                values_only = false;
            }
            let key = match key {
                Type::ConstString(s) if s.contains(&b'"') => {
                    description!("'", s, "'")
                }
                Type::ConstString(s) if s.contains(&b'\'') => {
                    description!("\"", s, "\"")
                }
                Type::ConstString(s) if !is_valid_identifier(s) => {
                    description!("'", s, "'")
                }
                Type::ConstString(s) => s.clone(),
                // the only other key type, a `ConstantIntegerType`
                k => to_phpdoc_bytes(k),
            };
            let value = self.values[i].describe_bytes(level);
            items.push(description!(
                key,
                if optional { "?" } else { "" },
                ": ",
                value
            ));
            values.push(value);
        }
        let mut append = Vec::new();
        if truncate && items.len() > 8 {
            items.truncate(8);
            values.truncate(8);
            append = ", ...".into();
        }
        if self.is_unsealed().yes()
            && let Some((key, value)) = &self.unsealed
        {
            if !items.is_empty() {
                append.extend_from_slice(b", ");
            }
            append.extend_from_slice(b"...");
            let key_description = key.describe_bytes(Level::Precise);
            if key.is_plain_implicit_mixed()
                || (self.is_list.yes() && key_description == b"int<0, max>")
            {
                if !value.is_plain_implicit_mixed() {
                    append.extend(description!("<", value.describe_bytes(level), ">"));
                }
            } else {
                append.extend(description!(
                    "<",
                    key.describe_bytes(level),
                    ", ",
                    value.describe_bytes(level),
                    ">"
                ));
            }
        }
        description!(
            name,
            "{",
            if values_only {
                values.join(b", ".as_slice())
            } else {
                items.join(b", ".as_slice())
            },
            append,
            "}"
        )
    }

    /// `ConstantArrayType::toPhpDocNode()`: unlike `describe()`, nested values and quoted keys use PHPDoc syntax.
    fn to_phpdoc(&self) -> Vec<u8> {
        let mut name = if self.should_be_described_as_a_list() {
            "list"
        } else {
            "array"
        };
        let mut items = Vec::new();
        let mut values = Vec::new();
        let mut values_only = true;
        for (i, key) in self.keys.iter().enumerate() {
            let optional = self.is_optional_key(i);
            if *key != Type::ConstInt(i as i64) || optional {
                values_only = false;
            }
            let key = match key {
                Type::ConstString(s) if is_valid_identifier(s) => s.clone(),
                key => to_phpdoc_bytes(key),
            };
            let value = to_phpdoc_bytes(&self.values[i]);
            items.push(description!(
                key,
                if optional { "?" } else { "" },
                ": ",
                value
            ));
            values.push(value);
        }
        let mut items = if values_only { values } else { items };
        if self.is_unsealed().yes()
            && let Some((key, value)) = &self.unsealed
        {
            let tail = if key.is_plain_implicit_mixed()
                || (self.is_list.yes() && key.describe_bytes(Level::Precise) == b"int<0, max>")
            {
                if value.is_plain_implicit_mixed() {
                    "...".into()
                } else {
                    description!("...<", to_phpdoc_bytes(value), ">")
                }
            } else {
                name = "array";
                description!(
                    "...<",
                    to_phpdoc_bytes(key),
                    ", ",
                    to_phpdoc_bytes(value),
                    ">"
                )
            };
            items.push(tail);
        }
        description!(name, "{", items.join(b", ".as_slice()), "}")
    }

    fn generalize(&self) -> Type {
        if self.keys.is_empty() && !self.is_unsealed().yes() {
            return Type::ConstArray(Box::new(self.clone()));
        }
        let mut array = Type::array(
            self.iterable_key_type().generalize(),
            self.iterable_value_type().generalize(),
        );
        let mut accessories = Vec::new();
        if self.is_iterable_at_least_once().yes() {
            accessories.push(Type::Accessory(Accessory::NonEmptyArray));
        }
        if self.is_list.yes() {
            array = intersect(vec![array, Type::Accessory(Accessory::List)]);
        }
        if !accessories.is_empty() {
            accessories.insert(0, array);
            return intersect(accessories);
        }
        array
    }

    /// `ConstantArrayType::makeList()`
    fn make_list(&self) -> ConstArray {
        let mut a = self.clone();
        if !a.is_list.no() {
            a.is_list = Tri::Yes;
        }
        a
    }

    /// `ConstantArrayType::makeOffsetRequired()` for the first key.
    fn make_first_offset_required(&self) -> ConstArray {
        let mut a = self.clone();
        a.optional.retain(|&i| i != 0);
        a
    }
}

fn is_valid_identifier(s: &[u8]) -> bool {
    let mut bytes = s.iter().copied().peekable();
    let mut any = false;
    while bytes.peek().is_some() {
        if bytes.peek() == Some(&b'\\') {
            bytes.next();
        }
        match bytes.next() {
            Some(b) if b.is_ascii_alphabetic() || b == b'_' || b >= 0x80 => {}
            _ => return false,
        }
        while bytes
            .peek()
            .is_some_and(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b >= 0x80)
        {
            bytes.next();
        }
        any = true;
    }
    any
}

/// `ConstantArrayTypeBuilder` (array degradation disabled, as for PHPDoc shapes).
pub struct ShapeBuilder {
    array: ConstArray,
}

impl ShapeBuilder {
    pub fn new() -> Self {
        ShapeBuilder {
            array: ConstArray::empty(),
        }
    }

    pub fn set(&mut self, key: Option<Type>, value: Type, optional: bool) {
        let key = key.as_ref().map(crate::resolve::to_array_key);
        let a = &mut self.array;
        // a PHPDoc item without a key is never optional (`value?` is not a shape item)
        let Some(key) = key else {
            let max = *a.next_auto_indexes.iter().max().expect("auto index");
            a.keys.push(Type::ConstInt(max));
            a.values.push(value);
            a.next_auto_indexes = vec![max + 1];
            return;
        };
        if let Some(i) = a.keys.iter().position(|k| *k == key) {
            a.values[i] = if optional {
                union(vec![value, a.values[i].clone()])
            } else {
                value
            };
            if !optional {
                a.optional.retain(|&o| o != i);
            }
            return;
        }
        a.keys.push(key.clone());
        a.values.push(value);
        let non_list = if optional {
            a.is_list.and(Tri::Maybe)
        } else {
            Tri::No
        };
        match key {
            Type::ConstInt(offset) if !a.next_auto_indexes.is_empty() => {
                let min = *a.next_auto_indexes.iter().min().expect("auto index");
                let max = *a.next_auto_indexes.iter().max().expect("auto index");
                if offset >= 0 {
                    if offset > min {
                        a.is_list = if offset <= max {
                            a.is_list.and(Tri::Maybe)
                        } else {
                            non_list
                        };
                    }
                } else {
                    a.is_list = non_list;
                }
                // (past PHP_INT_MAX there is no next index)
                if offset >= max {
                    let next = offset.checked_add(1);
                    if !optional {
                        a.next_auto_indexes = next.into_iter().collect();
                    } else {
                        a.next_auto_indexes.extend(next);
                    }
                }
            }
            _ => a.is_list = non_list,
        }
        if optional {
            a.optional.push(a.keys.len() - 1);
        }
    }

    pub fn make_unsealed(&mut self, key: Type, value: Type) {
        self.array.unsealed = Some((key, value));
    }

    pub fn get(self) -> Type {
        let mut array = self.array;
        // without keys nothing was set: still an empty list
        if array.keys.is_empty() {
            if let Some((key, value)) = array.unsealed.clone()
                && key != Type::Never(NeverKind::Explicit)
            {
                return Type::array(key, value);
            }
            return Type::ConstArray(Box::new(array));
        }
        if let Some((key, _)) = &mut array.unsealed
            && key.is_benevolent_array_key()
        {
            *key = mixed();
        }
        Type::ConstArray(Box::new(array))
    }
}

/// `$a->isSuperTypeOf($b)`
pub fn is_super_type_of(a: &Type, b: &Type) -> Tri {
    use Type as T;
    if let T::Reasoned(t, _) = b {
        return is_super_type_of(a, t);
    }
    if let T::LateArrayShape(s) = b {
        return is_super_type_of(a, &s.resolve());
    }
    if let T::SubtractedObject { object, excluded } = b
        && matches!(a, T::Object(..) | T::Static(..) | T::This(_))
    {
        return if is_super_type_of(excluded, a).yes() {
            Tri::No
        } else {
            is_super_type_of(a, object)
        };
    }
    match a {
        T::Reasoned(t, _) => is_super_type_of(t, b),
        T::LateArrayShape(s) => if b.is_never() { Tri::Yes } else { is_super_type_of(&s.resolve(), b).and(Tri::Maybe) },
        // `*` is a bivariant `mixed`
        // (a call-site variance is a generic argument's only, compared in `valid_variance()`)
        T::Mixed { .. } | T::StrictMixed | T::Error | T::Wildcard | T::OffsetAccess(..) | T::Projection(..) => Tri::Yes,
        T::SubtractedMixed { excluded, .. } => mixed::is_super(excluded, b),
        T::SubtractedObject { object, excluded } => {
            if b.is_compound() { return is_sub_type_of(b, a); }
            let outside = is_super_type_of(excluded, b).negate();
            if outside.no() { return Tri::No; }
            let positive = match b {
                T::SubtractedObject { object, excluded } => {
                    if is_super_type_of(excluded, a).yes() { return Tri::No; }
                    object.as_ref()
                }
                b => b,
            };
            is_super_type_of(object, positive).and(outside)
        }
        T::Never(_) => match b {
            T::Never(_) => Tri::Yes,
            T::Template(_) => Tri::Maybe,
            _ => Tri::No,
        },
        T::Conditional(c) => conditional_super(c, b),
        T::Union(types) | T::Benevolent(types) => {
            // a TemplateBenevolentUnionType is a UnionType but not a TemplateUnionType
            if matches!(
                b,
                T::Union(_) | T::Benevolent(_) | T::Iterable(..) | T::Never(_) | T::IntRange(..)
            ) || matches!(b, T::Template(tpl) if matches!(tpl.bound, T::Benevolent(_)))
            {
                return is_sub_type_of(b, a);
            }
            let mut result = Tri::No;
            for t in types {
                let r = is_super_type_of(t, b);
                if r.yes() {
                    return r;
                }
                result = result.or(r);
            }
            // a TemplateUnionType or a conditional also answers as a whole
            let whole = matches!(b, T::Conditional(_))
                || matches!(b, T::Template(tpl) if matches!(tpl.bound, T::Union(_) | T::Benevolent(_)));
            if whole {
                result = result.or(is_sub_type_of(b, a));
            }
            result
        }
        T::Intersection(types) => {
            if matches!(b, T::Intersection(_)) && a == b {
                return Tri::Yes;
            }
            if b.is_never() {
                return Tri::Yes;
            }
            types
                .iter()
                .map(|t| is_super_type_of(t, b))
                .fold(Tri::Yes, Tri::and)
        }
        T::Template(tpl) => {
            if matches!(b, T::Template(_) | T::Intersection(_)) {
                return is_sub_type_of(b, a);
            }
            if b.is_never() {
                return Tri::Yes;
            }
            is_super_type_of(&tpl.bound, b).and(Tri::Maybe)
        }
        // IntegerRangeType is an IntegerType: integer types compare directly, not through CompoundType
        T::Int | T::ConstInt(_) | T::IntRange(..)
            if matches!(b, T::Int | T::ConstInt(_) | T::IntRange(..)) =>
        {
            match (a, b) {
                (T::Int, _) => Tri::Yes,
                (T::ConstInt(x), T::ConstInt(y)) => Tri::from(x == y),
                (T::ConstInt(x), T::IntRange(min, max)) => {
                    if min.is_none_or(|m| m <= *x) && max.is_none_or(|m| *x <= m) {
                        Tri::Maybe
                    } else {
                        Tri::No
                    }
                }
                (_, T::Int) => Tri::Maybe,
                _ => int_range_super(a, b),
            }
        }
        _ if b.is_compound() && !(matches!(a, T::Callable(_)) && matches!(b, T::Callable(_))) => {
            if let T::Accessory(acc) = a
                && (matches!(acc, Accessory::List | Accessory::NonEmptyArray))
                && a == b
            {
                return Tri::Yes;
            }
            is_sub_type_of(b, a)
        }
        T::NonexistentParent => Tri::from(matches!(b, T::NonexistentParent)),
        T::IntRange(..) | T::Int | T::ConstInt(_) => Tri::No,
        T::Null => Tri::from(matches!(b, T::Null)),
        T::Void => Tri::from(matches!(b, T::Void)),
        T::Bool => Tri::from(matches!(b, T::Bool | T::ConstBool(_))),
        T::ConstBool(x) => match b {
            T::ConstBool(y) => Tri::from(x == y),
            T::Bool => Tri::Maybe,
            _ => Tri::No,
        },
        T::Float => Tri::from(matches!(b, T::Float | T::ConstFloat(_))),
        T::ConstFloat(_) => match b {
            T::ConstFloat(_) => Tri::from(a == b),
            T::Float => Tri::Maybe,
            _ => Tri::No,
        },
        T::String | T::StringAcceptingStringable | T::StringNotAcceptingStringable => {
            Tri::from(matches!(
                b,
                T::String
                    | T::StringAcceptingStringable
                    | T::StringNotAcceptingStringable
                    | T::ConstString(_)
                    | T::ClassString(_)
            ))
        }
        T::ConstString(x) => match b {
            // the class named by the string against the generic type, its uncertainty dropped
            T::ClassString(Some(g)) if matches!(**g, T::Mixed { .. }) => Tri::Maybe,
            T::ClassString(Some(g)) => {
                let named = T::Object(String::from_utf8_lossy(x).into_owned(), vec![], false);
                let r = match &**g {
                    T::Template(tpl) => is_super_type_of(&tpl.bound, &named),
                    T::Static(c, args) => is_super_type_of(&static_of(c, args), &named),
                    g => is_super_type_of(g, &named),
                };
                if r.yes() { Tri::Maybe } else { Tri::No }
            }
            T::ClassString(None) => Tri::from(
                std::str::from_utf8(x)
                    .ok()
                    .and_then(symbols::lookup)
                    .is_some(),
            )
            .and(Tri::Maybe)
            .or(Tri::No),
            T::ConstString(y) => Tri::from(x == y),
            T::String | T::StringAcceptingStringable | T::StringNotAcceptingStringable => {
                Tri::Maybe
            }
            _ => Tri::No,
        },
        T::ClassString(None) => b.is_class_string(),
        T::ClassString(Some(g)) => match b {
            T::ConstString(s) => {
                if matches!(**g, T::Mixed { .. }) {
                    return Tri::Yes;
                }
                // a template's bound: not the uncertainty of the template itself
                let g = match &**g {
                    T::Template(t) => &t.bound,
                    g => g,
                };
                let r = is_super_type_of(
                    g,
                    &T::Object(String::from_utf8_lossy(s).into_owned(), vec![], false),
                );
                if b.is_class_string().yes() {
                    r
                } else {
                    r.and(Tri::Maybe)
                }
            }
            T::ClassString(Some(h)) => is_super_type_of(g, h),
            T::ClassString(None) => is_super_type_of(g, &T::ObjectWithoutClass),
            T::String => Tri::Maybe,
            _ => Tri::No,
        },
        T::Resource => Tri::from(matches!(b, T::Resource)),
        T::ObjectWithoutClass => match b {
            T::ObjectWithoutClass | T::ObjectShape(_) => Tri::Yes,
            _ => Tri::from(!b.object_class_names().is_empty()),
        },
        T::Object(class, args, exact) => object_super(class, args, b, true, *exact),
        T::This(class) if !matches!(b, T::This(_)) => {
            is_super_type_of(&T::Static(class.clone(), vec![]), b).and(Tri::Maybe)
        }
        T::Static(class, _) | T::This(class) => {
            let object = match a {
                T::Static(_, args) => static_of(class, args),
                _ => static_object(class),
            };
            match b {
                T::Static(..) | T::This(_) => is_super_type_of(&object, b),
                T::ObjectWithoutClass => Tri::Maybe,
                T::Object(c, _, exact) => {
                    let r = is_super_type_of(&object, b);
                    if r.yes() && (*exact || symbols::lookup(c).is_some_and(|c| c.is_final)) {
                        r
                    } else {
                        r.and(Tri::Maybe)
                    }
                }
                _ => Tri::No,
            }
        }
        T::Array(_, item) => match b {
            T::Array(..) | T::ConstArray(_) => {
                let result = is_super_type_of(item, &b.iterable_value_type()).and(
                    is_super_type_of(&a.iterable_key_type(), &b.iterable_key_type()),
                );
                // a possibly-empty constant array admits `[]`, a subtype of every array type
                if result.no() && matches!(b, T::ConstArray(_)) && !b.is_iterable_at_least_once().yes() {
                    Tri::Maybe
                } else {
                    result
                }
            }
            _ => Tri::No,
        },
        T::ConstArray(ca) => match b {
            T::ConstArray(cb) => const_array_super(ca, cb),
            T::Array(..) => {
                if ca.keys.is_empty() {
                    return Tri::Maybe;
                }
                let key = is_super_type_of(&ca.iterable_key_type(), &b.iterable_key_type());
                if key.no() {
                    return key;
                }
                Tri::Maybe.and(key).and(is_super_type_of(
                    &ca.iterable_value_type(),
                    &b.iterable_value_type(),
                ))
            }
            _ => Tri::No,
        },
        T::Callable(sig) => {
            let callable = b.is_callable();
            match sig {
                Some(ours) if !callable.no() => callable.and(signature_super(ours, b, None)),
                _ => callable,
            }
        }
        T::Closure(ours) => match b {
            T::Closure(_) => signature_super(ours, b, None),
            T::Object(c, _, _) if canonical_class(c) == "Closure" => Tri::Maybe,
            _ => object_super("Closure", &[], b, true, false),
        },
        T::EnumCase(class, _) => match b {
            T::EnumCase(..) => Tri::from(a == b),
            T::Object(c, _, _) if canonical_class(c) == canonical_class(class) => Tri::Maybe,
            T::ObjectWithoutClass => Tri::Maybe,
            _ => Tri::No,
        },
        T::ObjectShape(props) => match b {
            // NonexistentParentClassType is an object but has no instance properties, even optional ones.
            T::NonexistentParent => Tri::from(props.iter().all(|(_, _, optional)| *optional)),
            // a final class without a required property; ponytail: otherwise maybe, the property types are not
            // compared
            _ if b.object_class_names().iter().any(|c| {
                symbols::lookup(c).is_some_and(|c| {
                    c.is_final
                        && props
                            .iter()
                            .any(|(name, _, optional)| !optional && !c.properties.contains(name))
                })
            }) =>
            {
                Tri::No
            }
            T::ObjectWithoutClass => Tri::Maybe,
            T::ObjectShape(_) => Tri::from(a == b).or(Tri::Maybe),
            T::Object(..) | T::Static(..) | T::This(_) => Tri::Maybe,
            _ => Tri::No,
        },
        // (b is not compound here)
        T::Iterable(key, value) => b
            .is_iterable()
            .and(is_super_type_of(value, &b.iterable_value_type()))
            .and(is_super_type_of(key, &b.iterable_key_type())),
        T::Accessory(acc) => accessory_super(acc, b),
    }
}

impl Conditional {
    /// `LateResolvableType::resolve()`: the branch the subject takes, both when it may take either.
    /// ponytail: the subject inside the branches is not narrowed (`getNormalizedIf()`); add for `(T is int ? T : ...)`.
    pub fn resolve(&self) -> Type {
        let Subject::Type(subject) = &self.subject else {
            return union(vec![self.then.clone(), self.otherwise.clone()]);
        };
        let (yes, no) = if self.negated {
            (&self.otherwise, &self.then)
        } else {
            (&self.then, &self.otherwise)
        };
        match is_super_type_of(&self.target, subject) {
            Tri::Yes => yes.clone(),
            Tri::No => no.clone(),
            Tri::Maybe => union(vec![self.then.clone(), self.otherwise.clone()]),
        }
    }

    /// `isResolvable()`: no templates involved, or the subject's branch is certain (never for a parameter).
    pub fn is_resolvable(&self) -> bool {
        let Subject::Type(subject) = &self.subject else {
            return false;
        };
        !contains_template(subject) && !contains_template(&self.target)
            || !is_super_type_of(&self.target, subject).maybe()
    }
}

pub(crate) fn contains_template(t: &Type) -> bool {
    let found = std::cell::Cell::new(matches!(t, Type::Template(_)));
    t.traverse(&|x| {
        found.set(found.get() || contains_template(x));
        x.clone()
    });
    found.get()
}

/// `ConditionalType::isSuperTypeOf()`: branch by branch against its own kind, else as what it resolves to.
fn conditional_super(c: &Conditional, b: &Type) -> Tri {
    if let Type::Conditional(other) = b
        && std::mem::discriminant(&c.subject) == std::mem::discriminant(&other.subject)
    {
        return is_super_type_of(&c.then, &other.then)
            .and(is_super_type_of(&c.otherwise, &other.otherwise));
    }
    if b.is_never() {
        return Tri::Yes;
    }
    let b = match b {
        Type::Conditional(other) => other.resolve(),
        b => b.clone(),
    };
    let r = is_super_type_of(&c.resolve(), &b);
    if c.is_resolvable() {
        r
    } else {
        r.and(Tri::Maybe)
    }
}

/// `CallableTypeHelper::isParametersAcceptorSuperTypeOf()` over `b`'s variants (a callable without a signature, or a
/// method Pun knows no signature of, is the trivial `(...): mixed` one); `accept` is `Some(strictTypes)` for
/// `$treatMixedAsAny` (accepts() compares parameters and return with accepts()).
fn signature_super(ours: &Signature, b: &Type, accept: Option<bool>) -> Tri {
    let variants = callable_signatures(b);
    variants
        .iter()
        .map(|theirs| variant_super(ours, &specialize_signature(ours, theirs), accept))
        .reduce(Tri::or)
        .expect("a variant")
}

/// `ParametersAcceptorSelector::selectFromTypes()` of one variant: infer its templates from the accepting
/// callable's parameter types before comparing either parameters or return type.
fn specialize_signature(ours: &Signature, theirs: &Signature) -> Signature {
    let mut map = TemplateMap::default();
    for (i, param) in theirs.params.iter().enumerate() {
        let received = if param.variadic {
            Some(union(ours.params.iter().skip(i).map(|p| p.ty.clone()).collect()))
        } else {
            ours.params.get(i).map(|p| p.ty.clone())
        };
        if let Some(received) = received {
            map = map.union(infer_template_types(&param.ty, &received));
        }
    }
    let map = map.with_errors(&theirs.templates.iter().collect::<Vec<_>>());
    Signature {
        params: theirs.params.iter().map(|p| Param {
            ty: resolve_template_types(&p.ty, &map),
            ..p.clone()
        }).collect(),
        ret: resolve_template_types(&theirs.ret, &map),
        templates: theirs.templates.clone(),
    }
}

/// `getCallableParametersAcceptors()`: a callable without a signature, or a method Pun knows no signature of, is
/// the trivial `(...): mixed` one.
fn callable_signatures(b: &Type) -> Vec<Signature> {
    let of = |f: &symbols::Function| -> Vec<Signature> {
        f.variants
            .iter()
            .map(|v| Signature {
                params: v
                    .params
                    .iter()
                    .map(|p| Param {
                        name: p.name.clone(),
                        ty: crate::resolve::resolve_str(&p.ty, None),
                        optional: p.optional,
                        by_ref: p.by_ref != symbols::ByRef::No,
                        variadic: p.variadic,
                    })
                    .collect(),
                ret: crate::resolve::resolve_str(&v.ret, None),
                templates: vec![],
            })
            .collect()
    };
    let method = |class: &str, method: &str| -> Vec<Signature> {
        symbols::lookup(class)
            .and_then(|c| {
                std::iter::once(c.name.clone())
                    .chain(c.ancestors.iter().cloned())
                    .find_map(|n| symbols::lookup(&n)?.methods.get(&method.to_ascii_lowercase()).map(of))
            })
            .unwrap_or_default()
    };
    let mut variants = match b {
        Type::Callable(Some(s)) | Type::Closure(s) => vec![(**s).clone()],
        Type::ConstString(name)
            if let Some(f) = std::str::from_utf8(name).ok().and_then(symbols::function) =>
        {
            of(f)
        }
        // `'Class::method'`
        Type::ConstString(name) => std::str::from_utf8(name)
            .ok()
            .and_then(|name| name.split_once("::"))
            .map(|(c, m)| method(c, m))
            .unwrap_or_default(),
        // `[class, 'method']` (`ConstantArrayType::getCallableParametersAcceptors()`)
        Type::ConstArray(a) => a
            .callable_parts()
            .map(|(class, _, names)| names.into_iter().flat_map(|m| method(class, m)).collect())
            .unwrap_or_default(),
        _ => Vec::new(),
    };
    if variants.is_empty() {
        variants.push(Signature {
            params: vec![],
            ret: mixed(),
            templates: vec![],
        });
    }
    variants
}

fn variant_super(ours: &Signature, theirs: &Signature, accept: Option<bool>) -> Tri {
    let compare = |a: &Type, b: &Type, strict: bool| match accept {
        Some(_) => accepts::accepts(a, b, strict),
        None => is_super_type_of(a, b),
    };
    let mut their_params: Vec<&Param> = theirs.params.iter().collect();
    if let Some(last) = theirs.params.last()
        && last.variadic
    {
        while their_params.len() < ours.params.len() {
            their_params.push(last);
        }
    }
    let mut result = Tri::Yes;
    for (i, theirs) in their_params.iter().enumerate() {
        let Some(ours) = ours.params.get(i) else {
            if !theirs.optional {
                result = result.and(Tri::No);
            }
            continue;
        };
        if ours.optional && !theirs.optional {
            result = result.and(Tri::No);
        }
        result = result.and(compare(&theirs.ty, &ours.ty, accept.unwrap_or(true)));
    }
    if accept.is_none() && theirs.params.len() < ours.params.len() {
        result = result.and(Tri::Maybe);
    }
    result.and(compare(&ours.ret, &theirs.ret, true))
}

/// `TypeUtils::toBenevolentUnion()`
pub fn to_benevolent(t: Type) -> Type {
    match t {
        Type::Union(types) => Type::Benevolent(types),
        t => t,
    }
}

fn accessory_super(acc: &Accessory, b: &Type) -> Tri {
    match acc {
        Accessory::NonEmptyString => {
            if b.is_non_falsy_string().yes() {
                Tri::Yes
            } else {
                b.is_non_empty_string()
            }
        }
        Accessory::NonFalsyString => b.is_non_falsy_string(),
        Accessory::NumericString => b.is_numeric_string(),
        Accessory::LiteralString => b.is_literal_string(),
        Accessory::LowercaseString => b.is_lowercase_string(),
        Accessory::UppercaseString => b.is_uppercase_string(),
        Accessory::DecimalIntString { inverse } => decimal_int_super(*inverse, b),
        Accessory::List => b.is_list(),
        Accessory::NonEmptyArray => b.is_array().and(b.is_iterable_at_least_once()),
        Accessory::HasMethod(name) => b.has_method(name),
        Accessory::HasOffsetValue(key, value) => offset_value_relation(key, value, b, false),
        Accessory::ModelProperty(t) => larastan::model_property_super(t, b),
        Accessory::ViewString => larastan::view_string_super(b),
    }
}

/// HasOffsetValueType's offset/value comparison for the array representations used by inference.
/// Other offset-accessible kinds retain uncertainty rather than being mistaken for arrays.
fn offset_value_relation(key: &Type, value: &Type, other: &Type, reverse: bool) -> Tri {
    let (has, item) = match other {
        Type::Accessory(Accessory::HasOffsetValue(k, v)) if k.as_ref() == key => (Tri::Yes, v.as_ref().clone()),
        Type::Accessory(Accessory::HasOffsetValue(..)) => (Tri::Maybe, mixed()),
        Type::Array(k, v) => (is_super_type_of(k, key).and(Tri::Maybe), v.as_ref().clone()),
        Type::ConstArray(a) => {
            if let Some(i) = a.keys.iter().position(|k| k == key) {
                (if a.is_optional_key(i) { Tri::Maybe } else { Tri::Yes }, a.values[i].clone())
            } else if let Some((k, v)) = &a.unsealed {
                (is_super_type_of(k, key).and(Tri::Maybe), v.clone())
            } else {
                return Tri::No;
            }
        }
        _ => {
            let object = matches!(other, Type::Object(..) | Type::ObjectWithoutClass | Type::Static(..) | Type::This(_));
            return other.is_array().or(other.is_string()).or(Tri::from(object)).and(Tri::Maybe);
        }
    };
    if reverse {
        has.and(if matches!(other, Type::Accessory(Accessory::HasOffsetValue(..))) { Tri::Yes } else { Tri::Maybe })
            .and(is_super_type_of(&item, value))
    } else {
        has.and(is_super_type_of(value, &item))
    }
}

/// `AccessoryDecimalIntegerStringType::isSuperTypeOf()`
fn decimal_int_super(inverse: bool, b: &Type) -> Tri {
    let decimal = b.is_decimal_int_string();
    b.is_string()
        .and(if inverse { decimal.negate() } else { decimal })
}

fn int_range_super(a: &Type, b: &Type) -> Tri {
    let ((min, max), (bmin, bmax)) = (
        a.int_bounds().unwrap_or_default(),
        b.int_bounds().unwrap_or_default(),
    );
    let disjoint = matches!((min, bmax), (Some(x), Some(y)) if x > y)
        || matches!((max, bmin), (Some(x), Some(y)) if x < y);
    if disjoint {
        return Tri::No;
    }
    let lower = min.is_none() || bmin.is_some_and(|v| min.expect("min") <= v);
    let upper = max.is_none() || bmax.is_some_and(|v| max.expect("max") >= v);
    if lower && upper { Tri::Yes } else { Tri::Maybe }
}

fn object_super(class: &str, args: &[Type], b: &Type, strict: bool, exact: bool) -> Tri {
    let names = match b {
        Type::Closure(_) => vec!["Closure".to_string()],
        b => b.object_class_names(),
    };
    if names.is_empty() && !matches!(b, Type::ObjectWithoutClass) {
        return Tri::No;
    }
    if let Type::ObjectWithoutClass = b {
        return Tri::Maybe;
    }
    let that = &names[0];
    let input_exact = matches!(b, Type::Object(_, _, true));
    let result = match (symbols::lookup(class), symbols::lookup(that)) {
        (Some(this), Some(other)) => {
            if this.name == other.name {
                // Same written class: an override is only maybe wider than a broad reflection.
                // Differently written aliases keep ObjectType's separate canonical-name shortcut.
                if class == that && exact && !this.is_final && !input_exact { Tri::Maybe } else { Tri::Yes }
            } else if this.is_trait || other.is_trait {
                Tri::No
            } else if !exact && !this.is_final && other.ancestors.contains(&this.name) {
                Tri::Yes
            } else if (!input_exact && !other.is_final && this.ancestors.contains(&other.name))
                || (this.is_interface && !other.is_final && !input_exact)
                || (other.is_interface && !this.is_final && !exact)
            {
                Tri::Maybe
            } else {
                Tri::No
            }
        }
        _ => {
            if that == class {
                Tri::Yes
            } else {
                Tri::Maybe
            }
        }
    };
    if args.is_empty() || result.no() {
        return result;
    }
    // GenericObjectType: `$b` as this class (`getAncestorWithClassName()`), each argument a valid variance
    let Type::Object(b_class, b_args, _) = b else {
        return result;
    };
    let strict = strict && !b_args.is_empty();
    let (Some(c), class) = (symbols::lookup(b_class), canonical_class(class)) else {
        return result;
    };
    let ancestor: Vec<Type> = if c.name == class {
        if b_args.is_empty() {
            // a raw reference to the generic class itself
            return result.and(Tri::Maybe);
        }
        b_args.clone()
    } else if c.ancestors.contains(&class) {
        // ponytail: every built-in class names its generic ancestors' arguments; for user classes add PHPStan's
        // non-generic ancestor case (the class relation and maybe)
        // an ancestor named without type arguments: the class relation only
        let Some(generic) = c.generics.get(&class) else {
            return result;
        };
        generic
            .iter()
            .map(|s| with_class_args(&crate::resolve::resolve_str(s, Some(&c.name)), &c, b_args))
            .collect()
    } else {
        return result;
    };
    if ancestor.len() != args.len() {
        return Tri::No;
    }
    let declared = symbols::lookup(&class);
    args.iter()
        .zip(&ancestor)
        .enumerate()
        .map(|(i, (x, y))| valid_variance(x, y, declared.as_ref().and_then(|c| c.templates.get(i)).map_or("invariant", |t| t.variance), strict))
        .fold(Tri::Yes, Tri::and)
}

/// `TemplateTypeVariance::isValidVariance()` + `validPosition()`: call-site variance overrides the declaration;
/// ordinary mixed arguments are gradual only in an accepts context, not a strict generic subtype comparison.
fn valid_variance(a: &Type, b: &Type, declared: &str, strict: bool) -> Tri {
    match (a, b) {
        (Type::Wildcard, _) => Tri::Yes,
        (_, Type::Wildcard) => Tri::No,
        (Type::Projection(c, a), b) => {
            // (`b`'s own call-site variance must be in the same direction)
            let b = match b {
                Type::Projection(d, _) if d != c => return Tri::No,
                Type::Projection(_, b) => &**b,
                b => b,
            };
            if matches!(b, Type::Never(_)) || (!strict && (matches!(**a, Type::Mixed { .. }) || matches!(b, Type::Mixed { .. }))) {
                return Tri::Yes;
            }
            if *c { is_super_type_of(a, b) } else { is_super_type_of(b, a) }
        }
        (_, Type::Projection(..)) => Tri::No,
        (_, Type::Never(_)) => Tri::Yes,
        (Type::Mixed { .. }, _) | (_, Type::Mixed { .. }) if !strict => Tri::Yes,
        (Type::Benevolent(_), _) if !strict && !is_super_type_of(a, b).no() => Tri::Yes,
        (_, Type::Benevolent(_)) if !strict && !is_super_type_of(b, a).no() => Tri::Yes,
        _ if declared == "covariant" => is_super_type_of(a, b),
        _ if declared == "contravariant" => is_super_type_of(b, a),
        (Type::Template(a), Type::Template(b)) if a.name == b.name && a.scope == b.scope => Tri::Yes,
        (Type::Mixed { .. }, _) => Tri::Yes,
        (Type::Benevolent(_), _) if !is_super_type_of(a, b).no() => Tri::Yes,
        (_, Type::Benevolent(_)) if !is_super_type_of(b, a).no() => Tri::Maybe,
        (_, Type::Mixed { .. }) => Tri::Maybe,
        _ => Tri::from(a == b),
    }
}

fn const_array_super(a: &ConstArray, b: &ConstArray) -> Tri {
    if a.keys.is_empty() {
        return b.is_iterable_at_least_once().negate();
    }
    let mut result = Tri::Yes;
    for (i, key) in a.keys.iter().enumerate() {
        let has = match b.keys.iter().position(|k| k == key) {
            None => Tri::No,
            Some(j) if b.is_optional_key(j) => Tri::Maybe,
            Some(_) => Tri::Yes,
        };
        if has.no() {
            if !a.is_optional_key(i) {
                return Tri::No;
            }
            continue;
        } else if has.maybe() && !a.is_optional_key(i) {
            result = result.and(Tri::Maybe);
        }
        let j = b.keys.iter().position(|k| k == key).expect("offset");
        let r = is_super_type_of(&a.values[i], &b.values[j]);
        if r.no() {
            return r;
        }
        result = result.and(r);
    }
    result
}

/// `$a->isSubTypeOf($b)` for compound `$a`. `$b` is compound only for a union `$b` (and an iterable, never,
/// range, template or conditional `$a`): other compound pairs are answered by `isSuperTypeOf()` first.
fn is_sub_type_of(a: &Type, b: &Type) -> Tri {
    use Type as T;
    match a {
        T::StrictMixed => Tri::from(matches!(b, T::StrictMixed | T::Mixed { .. } | T::SubtractedMixed { .. } | T::Error)).or(Tri::Maybe),
        // (against a mixed `$b`, isSuperTypeOf() answers yes first)
        T::Mixed { .. } | T::Error => Tri::Maybe,
        T::SubtractedMixed { excluded, .. } => {
            if is_super_type_of(excluded, b).yes() { Tri::No } else { Tri::Maybe }
        }
        T::Union(types) | T::Benevolent(types) => {
            Tri::extreme_identity(types.iter().map(|t| is_super_type_of(b, t)))
        }
        T::Intersection(types) => Tri::max_min(types.iter().map(|t| is_super_type_of(b, t))),
        T::IntRange(min, max) => match b {
            T::Int | T::ConstInt(_) | T::IntRange(..) | T::Intersection(_) => {
                is_super_type_of(b, a)
            }
            // isSubTypeOfUnionWithReason(): a finite range is covered by its constants, else member by member
            T::Union(types) | T::Benevolent(types) => {
                if let (Some(min), Some(max)) = (min, max) {
                    let covered = types
                        .iter()
                        .filter(|t| matches!(t, T::ConstInt(v) if v >= min && v <= max))
                        .count();
                    if covered as i64 == max - min + 1 {
                        return Tri::Yes;
                    }
                }
                types
                    .iter()
                    .map(|t| is_sub_type_of(a, t))
                    .fold(Tri::No, Tri::or)
            }
            _ => Tri::No,
        },
        T::Iterable(..) => {
            if matches!(b, T::Intersection(_) | T::Union(_) | T::Benevolent(_)) {
                return is_super_type_of(
                    b,
                    &T::Union(vec![
                        T::array(a.iterable_key_type(), a.iterable_value_type()),
                        T::Object("Traversable".into(), vec![], false),
                    ]),
                );
            }
            if b.is_constant_array().yes() && b.is_iterable_at_least_once().no() {
                return Tri::Maybe;
            }
            let limit = if matches!(b, T::Iterable(..)) {
                Tri::Yes
            } else {
                Tri::Maybe
            };
            limit
                .and(b.is_iterable())
                .and(is_super_type_of(
                    &b.iterable_value_type(),
                    &a.iterable_value_type(),
                ))
                .and(is_super_type_of(
                    &b.iterable_key_type(),
                    &a.iterable_key_type(),
                ))
        }
        // (two callables meet in isSuperTypeOf())
        T::Callable(_) => b.is_callable().and(Tri::Maybe),
        T::Accessory(acc) => {
            let same = matches!(b, T::Accessory(x) if x == acc);
            match acc {
                Accessory::NonFalsyString
                    if matches!(b, T::Accessory(Accessory::NonEmptyString)) =>
                {
                    Tri::Yes
                }
                Accessory::DecimalIntString { inverse: false }
                    if matches!(
                        b,
                        T::Accessory(
                            Accessory::NumericString
                                | Accessory::LowercaseString
                                | Accessory::UppercaseString
                        )
                    ) =>
                {
                    Tri::Yes
                }
                _ => {
                    let q = match acc {
                        Accessory::NonEmptyString => b.is_non_empty_string(),
                        Accessory::NonFalsyString => b.is_non_falsy_string(),
                        Accessory::NumericString => b.is_numeric_string(),
                        Accessory::LiteralString => b.is_literal_string(),
                        Accessory::LowercaseString => b.is_lowercase_string(),
                        Accessory::UppercaseString => b.is_uppercase_string(),
                        Accessory::DecimalIntString { inverse } => decimal_int_super(*inverse, b),
                        Accessory::NonEmptyArray => b.is_array().and(b.is_iterable_at_least_once()),
                        Accessory::List => b.is_list(),
                        Accessory::HasMethod(name) => b.has_method(name),
                        Accessory::HasOffsetValue(key, value) => return offset_value_relation(key, value, b, true),
                        Accessory::ModelProperty(t) => {
                            return match b {
                                T::Accessory(Accessory::ModelProperty(other)) => is_super_type_of(other, t),
                                _ => b.is_string().and(Tri::Maybe),
                            };
                        }
                        Accessory::ViewString => b.is_string(),
                    };
                    q.and(if same { Tri::Yes } else { Tri::Maybe })
                }
            }
        }
        // (a union `$b` reaches here only for a union-bound template, which compares by its bound)
        T::Template(tpl) => match b {
            T::Template(other) => {
                let r = is_super_type_of(&other.bound, &tpl.bound);
                if other.scope == tpl.scope && other.name == tpl.name {
                    r
                } else {
                    r.and(Tri::Maybe)
                }
            }
            _ => is_super_type_of(b, &tpl.bound),
        },
        T::Conditional(c) => {
            let r = c.resolve();
            if r.is_compound() {
                is_sub_type_of(&r, b)
            } else {
                is_super_type_of(b, &r)
            }
        }
        // never, the remaining compound type
        _ => Tri::Yes,
    }
}

pub fn union(types: Vec<Type>) -> Type {
    let mut types = types;
    if types.len() == 1
        && !matches!(types[0], Type::Union(_) | Type::Benevolent(_))
        && !types[0].is_array().yes()
    {
        return types.pop().expect("one type");
    }
    let mut benevolent: Vec<String> = Vec::new();
    let mut flat = Vec::new();
    for t in types {
        match t {
            Type::Mixed { explicit: false } => return t,
            _ if t.is_implicit_never() => {}
            Type::Benevolent(inner) => {
                for i in &inner {
                    benevolent.push(i.describe(Level::Value));
                }
                flat.extend(inner);
            }
            Type::Union(inner) => flat.extend(inner),
            t => flat.push(t),
        }
    }
    let mut types = flat;
    if types.is_empty() {
        return Type::Never(NeverKind::Implicit);
    }
    if types.len() == 1 && !types[0].is_array().yes() {
        return types.pop().expect("one type");
    }
    let mut arrays = Vec::new();
    // Scalar constants per runtime class, including the distinct TemplateConstant* / TemplateNullType buckets.
    let mut scalars: Vec<(u8, Vec<Type>)> = Vec::new();
    let mut generic = [false; 5];
    let mut ranges = Vec::new();
    let mut rest = Vec::new();
    for t in types {
        if t.is_constant_scalar() {
            let class = scalar_class(&t);
            match scalars.iter_mut().find(|(c, _)| *c == class) {
                Some((_, items)) => {
                    if !items.iter().any(|other| {
                        if matches!(t, Type::Template(_)) {
                            other.describe(Level::Precise) == t.describe(Level::Precise)
                        } else {
                            other == &t
                        }
                    }) {
                        items.push(t);
                    }
                }
                None => scalars.push((class, vec![t])),
            }
            continue;
        }
        // only a type containing every value of the scalar (not `T of int`, `1&T`) absorbs all the constants of that
        // scalar (PHPStan 2.3.4)
        if t.is_boolean().yes() {
            generic[0] |= is_super_type_of(&t, &Type::Bool).yes();
        } else if t.is_float().yes() {
            generic[2] |= is_super_type_of(&t, &Type::Float).yes();
        } else if t.is_integer().yes() && !matches!(t, Type::IntRange(..)) {
            generic[1] |= is_super_type_of(&t, &Type::Int).yes();
        }
        // (a string that is no class-string, without accessories, is only a `mixed~string` subtraction PHPStan
        // intersects with a string, which Pun has no type for: generic[3] stays false)
        if let Type::IntRange(..) = t {
            ranges.push(t);
            continue;
        }
        if t.is_array().yes() {
            arrays.push(t);
            continue;
        }
        rest.push(t);
    }
    ranges.sort_by_key(|t| {
        let (min, max) = t.int_bounds().unwrap_or_default();
        (min.unwrap_or(i64::MIN), max.unwrap_or(i64::MAX))
    });
    let mut types = rest;
    types.extend(ranges);
    let mut kept_scalars: Vec<Type> = Vec::new();
    for (class, mut items) in scalars {
        if class < 4 && generic[class as usize] {
            continue;
        }
        if class == 0 && items.len() == 2 {
            types.push(Type::Bool);
            continue;
        }
        let mut i = 0;
        while i < types.len() {
            let mut j = 0;
            let mut removed_i = false;
            while j < items.len() {
                match compare_in_union_merge(&types[i], &items[j]) {
                    Some(Merge::A(t)) => {
                        types[i] = t;
                        items.remove(j);
                        j = 0;
                        continue;
                    }
                    Some(Merge::B(t)) => {
                        items[j] = t;
                        types.remove(i);
                        removed_i = true;
                        break;
                    }
                    None => j += 1,
                }
            }
            if !removed_i {
                i += 1;
            }
        }
        kept_scalars.extend(items);
    }
    types.extend(process_arrays(arrays));
    let mut i = 0;
    'outer: while i < types.len() {
        let mut j = i + 1;
        while j < types.len() {
            match compare_in_union_merge(&types[i], &types[j]) {
                Some(Merge::A(t)) => {
                    types[i] = t;
                    types.remove(j);
                    continue;
                }
                Some(Merge::B(t)) => {
                    types[j] = t;
                    types.remove(i);
                    continue 'outer;
                }
                None => j += 1,
            }
        }
        i += 1;
    }
    types.extend(kept_scalars);
    if types.len() == 1 {
        return types.pop().expect("one type");
    }
    if !benevolent.is_empty()
        && types
            .iter()
            .all(|t| benevolent.contains(&t.describe(Level::Value)))
    {
        return Type::Benevolent(types);
    }
    Type::Union(types)
}

fn scalar_class(t: &Type) -> u8 {
    match t {
        Type::ConstBool(_) => 0,
        Type::ConstInt(_) => 1,
        Type::ConstFloat(_) => 2,
        Type::ConstString(_) => 3,
        Type::Template(t) if matches!(t.bound, Type::ConstInt(_)) => 5,
        Type::Template(t) if matches!(t.bound, Type::ConstString(_)) => 6,
        Type::Template(_) if matches!(t.scalar_class_type(), Type::Union(_)) => 8,
        Type::Template(_) if matches!(t.scalar_class_type(), Type::Benevolent(_)) => 9,
        Type::Template(_) => 7, // TemplateNullType, including a nested null-template bound
        _ => 4,
    }
}

/// `TypeUtils::getAccessoryTypes() !== []` of a string type that is not a class string (plain `string` answers
/// maybe to that, so only intersections get here)

enum Merge {
    A(Type),
    B(Type),
}

/// `TypeCombinator::remove()` of object types (a union removes each member in turn): what a `catch` leaves of a
/// throw point. A type it cannot subtract stays as it is.
pub fn remove_object(from: &Type, removed: &Type) -> Type {
    if let Type::Union(parts) = removed {
        return parts.iter().fold(from.clone(), |from, part| remove_object(&from, part));
    }
    match is_super_type_of(removed, from) {
        Tri::Yes => Type::Never(NeverKind::Implicit),
        Tri::No => from.clone(),
        Tri::Maybe => remove_object_exclusion(from, removed).unwrap_or_else(|| from.clone()),
    }
}

/// TypeCombinator::remove() and ObjectType::tryRemove() for a named-object exclusion.
fn remove_object_exclusion(from: &Type, removed: &Type) -> Option<Type> {
    if removed.is_compound() {
        return None;
    }
    if let Type::Union(parts) | Type::Benevolent(parts) = from {
        return Some(union(
            parts.iter().map(|part| remove_object_exclusion(part, removed)).collect::<Option<_>>()?,
        ));
    }
    let (object, previous) = match from {
        Type::SubtractedObject { object, excluded } => {
            (object.as_ref(), Some(excluded.as_ref()))
        }
        from => (from, None),
    };
    let Type::Object(class, args, _) = object else { return None };
    match is_super_type_of(removed, from) {
        Tri::Yes => return Some(Type::Never(NeverKind::Implicit)),
        Tri::No => return Some(from.clone()),
        Tri::Maybe => {}
    }
    let removed_object = match removed {
        Type::SubtractedObject { object, .. } => object.as_ref(),
        removed => removed,
    };
    if let Type::Object(removed_class, _, _) = removed_object {
        // UnionType::EQUAL_UNION_CLASSES, before ObjectType's ordinary subtraction.
        let complement = match (class.as_str(), removed_class.as_str()) {
            ("Throwable", "Exception") => Some("Error"),
            ("Throwable", "Error") => Some("Exception"),
            ("DateTimeInterface", "DateTime") => Some("DateTimeImmutable"),
            ("DateTimeInterface", "DateTimeImmutable") => Some("DateTime"),
            _ => None,
        };
        if let Some(complement) = complement {
            return Some(Type::Object(complement.into(), vec![], false));
        }
    }
    if !is_super_type_of(from, removed).yes() {
        return Some(from.clone());
    }
    let excluded = match previous {
        Some(previous) => union(vec![previous.clone(), removed.clone()]),
        None => removed.clone(),
    };
    Some(Type::object_subtracted(
        Type::Object(class.clone(), args.clone(), false),
        excluded,
    ))
}

/// TypeCombinator::intersectWithSubtractedType(): restoring part of an excluded object retains its difference.
fn restore_object_exclusion(positive: &Type, excluded: &Type, other: &Type) -> Option<Type> {
    // Only a definite containment uses remove(); uncertain subtractable types intersect their exclusions.
    if !is_super_type_of(excluded, other).yes() {
        return None;
    }
    let remaining = remove_object_exclusion(excluded, other)?;
    let remaining = if remaining.is_never() || !is_super_type_of(&remaining, other).no() {
        Type::Never(NeverKind::Implicit)
    } else {
        remaining
    };
    Some(match positive {
        Type::Mixed { explicit } => Type::mixed_subtracted(*explicit, remaining),
        _ => Type::object_subtracted(positive.clone(), remaining),
    })
}

/// TypeCombinator::intersect()'s positive-object pre-check, followed by unionWithSubtractedType().
fn intersect_object_exclusion(a: &Type, b: &Type) -> Option<Type> {
    let Type::SubtractedObject { object, excluded } = a else { return None };
    if !is_super_type_of(object, b).yes() {
        return None;
    }
    mixed::intersection(&Type::SubtractedMixed { explicit: false, excluded: excluded.clone() }, b)
}

/// `TypeCombinator::compareTypesInUnion()`
fn compare_in_union_merge(a: &Type, b: &Type) -> Option<Merge> {
    // ConstantStringType/ConstantIntegerType check their nonconstant parent class before CompoundType.
    // In particular a literal is only maybe wider than a nested TemplateStringType/TemplateIntegerType,
    // even when that template's bound happens to be the same constant literal.
    let super_type = |a: &Type, b: &Type| {
        if (a.is_constant_scalar() && !matches!(a, Type::Template(_)))
            || matches!(a, Type::IntRange(..))
        {
            is_super_type_of(a, b.scalar_class_type())
        } else {
            is_super_type_of(a, b)
        }
    };
    if let Type::SubtractedMixed { explicit, excluded } = a {
        return Some(Merge::A(
            restore_object_exclusion(&Type::Mixed { explicit: *explicit }, excluded, b)
                .unwrap_or_else(|| mixed::join(*explicit, excluded, b)),
        ));
    }
    if let Type::SubtractedMixed { explicit, excluded } = b {
        return Some(Merge::B(
            restore_object_exclusion(&Type::Mixed { explicit: *explicit }, excluded, a)
                .unwrap_or_else(|| mixed::join(*explicit, excluded, a)),
        ));
    }
    if let Type::SubtractedObject { object, excluded } = a
        && is_super_type_of(object, b).yes()
        && let Some(joined) = restore_object_exclusion(object, excluded, b)
            .or_else(|| mixed::join_object(object, excluded, b))
    {
        return Some(Merge::A(joined));
    }
    if let Type::SubtractedObject { object, excluded } = b
        && is_super_type_of(object, a).yes()
        && let Some(joined) = restore_object_exclusion(object, excluded, a)
            .or_else(|| mixed::join_object(object, excluded, a))
    {
        return Some(Merge::B(joined));
    }
    if let Type::IntRange(min, max) = a
        && let Some(t) = range_try_union(*min, *max, b)
    {
        return Some(Merge::A(t));
    }
    if let Type::IntRange(min, max) = b
        && let Some(t) = range_try_union(*min, *max, a)
    {
        return Some(Merge::B(t));
    }
    if matches!(a, Type::IntRange(..)) && matches!(b, Type::IntRange(..)) {
        return None;
    }
    if a.is_constant_array().yes() && b.is_constant_array().yes() {
        return None;
    }
    if let (Type::Iterable(ak, av), Type::Iterable(bk, bv)) = (a, b) {
        return Some(Merge::A(Type::Iterable(
            Box::new(union(vec![(**ak).clone(), (**bk).clone()])),
            Box::new(union(vec![(**av).clone(), (**bv).clone()])),
        )));
    }
    // SubtractableType (object and mixed types) keeps the first of two equivalent types
    let subtractable = |t: &Type| {
        let mut t = t;
        while let Type::Template(template) = t {
            t = &template.bound;
        }
        matches!(
            t,
            Type::Object(..)
                | Type::Mixed { .. }
                | Type::ObjectWithoutClass
                | Type::Static(..)
                | Type::This(_)
                | Type::EnumCase(..)
        )
    };
    if subtractable(a) && super_type(a, b).yes() {
        return Some(Merge::A(a.clone()));
    }
    if subtractable(b) && super_type(b, a).yes() {
        return Some(Merge::B(b.clone()));
    }
    if super_type(b, a).yes() {
        return Some(Merge::B(b.clone()));
    }
    if super_type(a, b).yes() {
        return Some(Merge::A(a.clone()));
    }
    // (a constant string is always `b`: constant scalars merge into the other types one by one)
    if let Type::ConstString(s) = b
        && s.is_empty()
        && matches!(
            a.describe(Level::Value).as_str(),
            "non-empty-string" | "non-falsy-string"
        )
    {
        return Some(Merge::A(intersect(case_accessories(a))));
    }
    if let Type::ConstString(s) = b
        && s.as_slice() == b"0"
        && let Some(t) = downgrade_non_falsy(a)
    {
        return Some(Merge::A(t));
    }
    None
}

/// `downgradeNonFalsyStringToNonEmpty()`: `'0'|non-falsy-string` is `non-empty-string`.
fn downgrade_non_falsy(t: &Type) -> Option<Type> {
    let non_falsy = Type::Accessory(Accessory::NonFalsyString);
    let rest = match t {
        Type::Intersection(types) if types.contains(&non_falsy) => {
            types.iter().filter(|x| **x != non_falsy).cloned().collect()
        }
        _ => return None,
    };
    let without = intersect(rest);
    Some(if without.is_non_empty_string().yes() {
        without
    } else {
        intersect(vec![without, Type::Accessory(Accessory::NonEmptyString)])
    })
}

fn case_accessories(t: &Type) -> Vec<Type> {
    let mut v = vec![Type::String];
    if t.is_lowercase_string().yes() {
        v.push(Type::Accessory(Accessory::LowercaseString));
    }
    if t.is_uppercase_string().yes() {
        v.push(Type::Accessory(Accessory::UppercaseString));
    }
    v
}

/// `IntegerRangeType::tryUnion()`
fn range_try_union(min: Option<i64>, max: Option<i64>, other: &Type) -> Option<Type> {
    let (omin, omax) = other.int_bounds()?;
    let disjoint = matches!((min, omax), (Some(x), Some(y)) if x > y.saturating_add(1))
        || matches!((max, omin), (Some(x), Some(y)) if x.saturating_add(1) < y);
    if disjoint {
        return None;
    }
    Some(Type::int_range(
        min.zip(omin).map(|(x, y)| x.min(y)),
        max.zip(omax).map(|(x, y)| x.max(y)),
    ))
}

impl Type {
    /// `getArrays()` as (iterable key, item) pairs
    fn general_arrays(&self) -> Vec<(Type, Type)> {
        match self {
            Type::Array(_, item) => vec![(self.iterable_key_type(), (**item).clone())],
            Type::Intersection(types) => types.iter().flat_map(Type::general_arrays).collect(),
            // TemplateIntersectionType inherits IntersectionType::getArrays().
            Type::Template(t) if matches!(t.bound, Type::Intersection(_)) => t.bound.general_arrays(),
            _ => vec![],
        }
    }
}

/// `TypeCombinator::processArrayTypes()`
fn process_arrays(arrays: Vec<Type>) -> Vec<Type> {
    if arrays.is_empty() {
        return vec![];
    }
    let accessories = common_array_accessories(&arrays);
    let with = |t: Type, accessories: &[Type]| {
        let mut all = vec![t];
        all.extend(accessories.iter().cloned());
        intersect(all)
    };
    if arrays.len() == 1 {
        return vec![with(
            arrays.into_iter().next().expect("one array"),
            &accessories,
        )];
    }
    let (mut keys, mut values, mut seen) = (Vec::new(), Vec::new(), Vec::<Type>::new());
    let mut general = false;
    for a in &arrays {
        let constant = a.constant_arrays();
        if constant.is_empty() {
            for (key, item) in a.general_arrays() {
                keys.push(key);
                values.push(item);
                general = true;
            }
            continue;
        }
        for ca in constant {
            for (key, value) in ca.keys.iter().zip(&ca.values) {
                values.push(value.clone());
                if !seen.contains(key) {
                    keys.push(key.clone());
                    seen.push(key.clone());
                }
            }
        }
    }
    // (`reduceArrays($arrayTypes, false)` hands a union with a general array back unchanged, never as one array)
    if general {
        return vec![with(Type::array(union(keys), union(values)), &accessories)];
    }
    reduce_arrays(&arrays)
        .into_iter()
        .map(|a| {
            let applied: Vec<Type> = if a.is_iterable_at_least_once().no() {
                accessories
                    .iter()
                    .filter(|t| **t != Type::Accessory(Accessory::NonEmptyArray))
                    .cloned()
                    .collect()
            } else {
                accessories.clone()
            };
            with(a, &applied)
        })
        .collect()
}

/// `TypeCombinator::reduceArrays()` for shapes without unsealed extras (PHPDoc shapes outside bleeding edge).
fn reduce_arrays(arrays: &[Type]) -> Vec<Type> {
    let mut retained = Vec::new();
    let mut to_process: Vec<Option<ConstArray>> = Vec::new();
    let mut empty = None;
    // (processArrayTypes() calls it with constant arrays only)
    for a in arrays {
        if a.is_iterable_at_least_once().no() {
            empty = Some(a.clone());
            continue;
        }
        to_process.extend(a.constant_arrays().into_iter().map(Some));
    }
    // Plain empty shapes take part in merging. Template/intersection identity must
    // survive separately, as reduceArrays' $newArrays rather than $arraysToProcess.
    if let Some(empty) = empty {
        match empty {
            Type::ConstArray(c) => to_process.push(Some(*c)),
            empty => retained.push(empty),
        }
    }
    let mut per_key: Vec<(Type, Vec<usize>)> = Vec::new();
    for (i, a) in to_process.iter().enumerate() {
        for key in &a.as_ref().expect("array").keys {
            match per_key.iter_mut().find(|(k, _)| k == key) {
                Some((_, v)) => v.push(i),
                None => per_key.push((key.clone(), vec![i])),
            }
        }
    }
    let mut eligible: Vec<(usize, Vec<(usize, usize)>)> = Vec::new();
    for (_, idx) in &per_key {
        for x in 0..idx.len().saturating_sub(1) {
            for y in x + 1..idx.len() {
                let pos = match eligible.iter().position(|(i, _)| *i == idx[x]) {
                    Some(p) => p,
                    None => {
                        eligible.push((idx[x], vec![]));
                        eligible.len() - 1
                    }
                };
                match eligible[pos].1.iter_mut().find(|(j, _)| *j == idx[y]) {
                    Some((_, c)) => *c += 1,
                    None => eligible[pos].1.push((idx[y], 1)),
                }
            }
        }
    }
    for (i, others) in &eligible {
        for &(j, overlap) in others {
            let (Some(a), Some(b)) = (&to_process[*i], &to_process[j]) else {
                if to_process[*i].is_none() {
                    break;
                }
                continue;
            };
            if overlap == 1
                && a.keys.len() == 1
                && b.keys.len() == 1
                && !matches!(
                    union(vec![a.values[0].clone(), b.values[0].clone()]),
                    Type::Union(_)
                )
            {
                to_process[j] = Some(b.merge_with(a));
                to_process[*i] = None;
                break;
            }
            if overlap == a.keys.len() && b.is_keys_superset_of(a) {
                to_process[j] = Some(b.merge_with(a));
                to_process[*i] = None;
                break;
            }
            if overlap == b.keys.len() && a.is_keys_superset_of(b) {
                to_process[*i] = Some(a.merge_with(b));
                to_process[j] = None;
                continue;
            }
        }
    }
    // ponytail: reduceArrays' second pass and final collapses only apply to shapes with unsealed extras
    // (bleeding edge) or past ConstantArrayTypeBuilder::ARRAY_COUNT_LIMIT, neither of which signatures reach.
    retained
        .into_iter()
        .chain(to_process.into_iter().flatten().map(|a| Type::ConstArray(Box::new(a))))
        .collect()
}

impl ConstArray {
    /// `legacyIsKeysSupersetOf()`, called (by `reduce_arrays`) only when every key of `other` is one of ours, so
    /// its size and missing-key checks are left out.
    fn is_keys_superset_of(&self, other: &ConstArray) -> bool {
        let mut fail_on_different = self.keys.len() != other.keys.len() || self.keys.len() < 2;
        for (j, key) in other.keys.iter().enumerate() {
            let i = self
                .keys
                .iter()
                .position(|k| k == key)
                .expect("a shared key");
            if !is_super_type_of(&other.values[j], &self.values[i]).no() {
                continue;
            }
            if fail_on_different {
                return false;
            }
            fail_on_different = true;
        }
        let required = self
            .keys
            .iter()
            .enumerate()
            .filter(|(i, k)| !other.keys.contains(k) && !self.is_optional_key(*i))
            .count();
        required <= 1
    }

    /// `legacyMergeWith()`
    fn merge_with(&self, other: &ConstArray) -> ConstArray {
        let mut merged = self.clone();
        for (i, key) in self.keys.iter().enumerate() {
            match other.keys.iter().position(|k| k == key) {
                None => merged.optional.push(i),
                Some(j) => {
                    if other.is_optional_key(j) {
                        merged.optional.push(i);
                    }
                    merged.values[i] =
                        union(vec![merged.values[i].clone(), other.values[j].clone()]);
                }
            }
        }
        let mut seen = Vec::new();
        merged.optional.retain(|i| {
            !seen.contains(i) && {
                seen.push(*i);
                true
            }
        });
        merged
            .next_auto_indexes
            .extend(other.next_auto_indexes.iter().copied());
        merged.next_auto_indexes.sort_unstable();
        merged.next_auto_indexes.dedup();
        merged.is_list = self.is_list.and(other.is_list);
        merged
    }
}

/// `TypeCombinator::processArrayAccessoryTypes()`
fn common_array_accessories(arrays: &[Type]) -> Vec<Type> {
    let mut found: Vec<(Type, Vec<usize>)> = Vec::new();
    let mut add = |t: Type, i: usize| match found.iter_mut().find(|(x, _)| *x == t) {
        Some((_, idx)) => {
            if !idx.contains(&i) {
                idx.push(i);
            }
        }
        None => found.push((t, vec![i])),
    };
    for (i, a) in arrays.iter().enumerate() {
        // TemplateIntersectionType exposes its bound's immediate getTypes(), not resolved templates.
        let intersection = match a {
            Type::Template(t) => &t.bound,
            _ => a,
        };
        if let Type::Intersection(types) = intersection {
            for t in types {
                if matches!(t, Type::Template(_)) {
                    break;
                }
                if matches!(t, Type::Accessory(_) | Type::Callable(_)) {
                    add(t.clone(), i);
                }
            }
        }
        if !a.is_constant_array().yes() {
            continue;
        }
        for ca in a.constant_arrays() {
            if ca.is_list.yes() {
                add(Type::Accessory(Accessory::List), i);
            }
            if ca.is_iterable_at_least_once().yes() {
                add(Type::Accessory(Accessory::NonEmptyArray), i);
            }
        }
    }
    let mut common: Vec<Type> = found
        .into_iter()
        .filter(|(_, idx)| idx.len() == arrays.len())
        .map(|(t, _)| t)
        .collect();
    if arrays.iter().all(|a| a.is_iterable_at_least_once().yes()) {
        common.push(Type::Accessory(Accessory::NonEmptyArray));
    }
    common
}

pub fn intersect(types: Vec<Type>) -> Type {
    if types.len() <= 1 {
        return types
            .into_iter()
            .next()
            .unwrap_or(Type::Never(NeverKind::Implicit));
    }
    if let Some(never) = types.iter().find(|t| t.is_implicit_never()) {
        return never.clone();
    }
    // LateResolvableTypeTrait keeps comparisons uncertain until template keys/offsets resolve.
    if types.iter().any(|t| matches!(t, Type::LateArrayShape(_) | Type::OffsetAccess(..)) && contains_template(t)) {
        return Type::Intersection(types);
    }
    // A & (B | C) → (A & B) | (A & C); with several unions a template union goes first, then a benevolent one
    let members = |t: &Type| -> Option<Vec<Type>> {
        match t {
            Type::Union(inner) | Type::Benevolent(inner) => Some(inner.clone()),
            Type::Template(tpl) => match &tpl.bound {
                Type::Union(inner) | Type::Benevolent(inner) => Some(inner.clone()),
                _ => None,
            },
            _ => None,
        }
    };
    let unions: Vec<usize> = (0..types.len())
        .filter(|&i| members(&types[i]).is_some())
        .collect();
    let pick = if unions.len() >= 2 {
        unions
            .iter()
            .find(|&&i| matches!(types[i], Type::Template(_)))
            .or_else(|| {
                unions
                    .iter()
                    .find(|&&i| matches!(types[i], Type::Benevolent(_)))
            })
            .or(unions.first())
            .copied()
    } else {
        unions.first().copied()
    };
    if let Some(i) = pick {
        let inner = members(&types[i]).expect("union");
        let others: Vec<Type> = types
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, t)| t.clone())
            .collect();
        let parts = inner
            .into_iter()
            .map(|t| {
                let mut v = vec![t];
                v.extend(others.iter().cloned());
                intersect(v)
            })
            .collect();
        let mut u = union(parts);
        if u.is_never() {
            return u;
        }
        let benevolent = match &types[i] {
            Type::Template(tpl) => matches!(tpl.bound, Type::Benevolent(_)),
            t => matches!(t, Type::Benevolent(_)),
        };
        if benevolent {
            u = to_benevolent(u);
        }
        if let Type::Template(tpl) = &types[i] {
            return Type::Template(Box::new(Template {
                bound: template_bound(u),
                ..(**tpl).clone()
            }));
        }
        return u;
    }
    let mut flat = Vec::new();
    for t in types {
        match t {
            Type::Intersection(inner) => flat.extend(inner),
            t => flat.push(t),
        }
    }
    let mut types = flat;
    // constant arrays first
    types.sort_by_key(|t| !matches!(t.array_class(), Type::ConstArray(_)));
    let mut i = 0;
    'outer: while i < types.len() {
        let mut j = i + 1;
        'inner: while j < types.len() {
            if let Some(t) = intersect_object_exclusion(&types[j], &types[i]) {
                types[i] = t;
                types.remove(j);
                continue;
            }
            if let Some(t) = intersect_object_exclusion(&types[i], &types[j]) {
                types[j] = t;
                types.remove(i);
                continue 'outer;
            }
            if let Some(t) = mixed::intersection(&types[i], &types[j])
                .or_else(|| mixed::intersection(&types[j], &types[i]))
            {
                types[i] = t;
                types.remove(j);
                continue;
            }
            if let Type::IntRange(min, max) = types[i]
                && let Some(t) = range_try_intersect(min, max, &types[j])
            {
                types[j] = t;
                types.remove(i);
                continue 'outer;
            }
            // the SubtractableType pre-check, `MixedType::isSuperTypeOfMixed()`: an explicit mixed is no wider
            // than an implicit one (or an error)
            let mixedish = |t: &Type| matches!(t, Type::Mixed { .. } | Type::Error);
            if mixedish(&types[i]) && mixedish(&types[j]) {
                let explicit = Type::Mixed { explicit: true };
                if types[j] != explicit || types[i] == explicit {
                    types.remove(j);
                    continue;
                }
                types.remove(i);
                continue 'outer;
            }
            let (a_super, a_reasons) = if let Type::Iterable(key, value) = types[j].array_class() {
                (iterable_super_of_mixed(key, value, &types[i]), Vec::new())
            } else {
                unresolvable::compare(&types[j], &types[i])
            };
            if a_super.yes() {
                types.remove(j);
                continue;
            }
            let b_super = if let Type::Iterable(key, value) = types[i].array_class() {
                iterable_super_of_mixed(key, value, &types[j])
            } else {
                unresolvable::compare(&types[i], &types[j]).0
            };
            if b_super.maybe() {
                if let (Type::ClassString(Some(x)), Type::ClassString(Some(y))) =
                    (&types[i], &types[j])
                {
                    types[i] =
                        Type::ClassString(Some(Box::new(intersect(vec![(**x).clone(), (**y).clone()]))));
                    types.remove(j);
                    continue;
                }
                // ConstantArrayType & list / non-empty-array (constant arrays sort first, so the shape is i)
                if let Type::ConstArray(ca) = types[i].array_class() {
                    let merged = match &types[j] {
                        Type::Accessory(Accessory::List) => Some(ca.make_list()),
                        Type::Accessory(Accessory::NonEmptyArray)
                            if (ca.keys.len() == 1 || ca.is_list.yes())
                                && ca.is_optional_key(0)
                                && !ca.is_unsealed().yes() =>
                        {
                            Some(ca.make_first_offset_required())
                        }
                        _ => None,
                    };
                    if let Some(merged) = merged {
                        let bound = Type::ConstArray(Box::new(merged));
                        // TemplateConstantArrayType::recreate() preserves scope, name, strategy and default.
                        types[i] = match &types[i] {
                            Type::Template(t) => Type::Template(Box::new(Template {
                                bound,
                                ..(**t).clone()
                            })),
                            _ => bound,
                        };
                        types.remove(j);
                        continue 'inner;
                    }
                }
                // ConstantArrayType & ArrayType: key by key (constant arrays sort first, so the shape is i).
                // ponytail: two constant arrays meet only through isSuperTypeOf(), not key by key.
                // As PHPStan's `continue 2`, the merged shape is not compared with the later types again.
                if let (Type::ConstArray(ca), Type::Array(..)) =
                    (types[i].array_class(), types[j].array_class())
                {
                    types[i] = ca.intersect_array(&types[j]);
                    types.remove(j);
                    i += 1;
                    continue 'outer;
                }
                // `getKeyType()`: the raw key of an array, not its iterable key
                if matches!(
                    types[i].array_class(),
                    Type::Array(..) | Type::Iterable(..) | Type::ConstArray(_)
                ) && let Type::Array(jk, _) | Type::Iterable(jk, _) = types[j].array_class()
                {
                    let key = intersect(vec![types[i].iterable_key_type(), (**jk).clone()]);
                    let item = intersect(vec![
                        types[i].iterable_value_type(),
                        types[j].iterable_value_type(),
                    ]);
                    types[j] = if matches!(
                        (types[i].array_class(), types[j].array_class()),
                        (Type::Iterable(..), Type::Iterable(..))
                    ) {
                        Type::Iterable(Box::new(key), Box::new(item))
                    } else {
                        Type::array(key, item)
                    };
                    types.remove(i);
                    continue 'outer;
                }
                if let (Type::Array(..), Type::Accessory(Accessory::List)) = (&types[i], &types[j])
                    && !is_super_type_of(
                        &types[j].iterable_key_type(),
                        &types[i].iterable_key_type(),
                    )
                    .yes()
                {
                    let key = intersect(vec![
                        types[i].iterable_key_type(),
                        types[j].iterable_key_type(),
                    ]);
                    // (never empty: the key types overlap, or the two would not be maybe-related)
                    let item = types[i].iterable_value_type();
                    types[i] = Type::array(key, item);
                    j += 1;
                    continue;
                }
                j += 1;
                continue;
            }
            if b_super.yes() {
                types.remove(i);
                continue 'outer;
            }
            if a_super.no() {
                let never = Type::Never(NeverKind::Implicit);
                return a_reasons.into_iter().next().map_or(never.clone(), |reason| never.with_unresolvable_reason(reason));
            }
            j += 1;
        }
        i += 1;
    }
    if types.len() == 1 {
        return types.pop().expect("one type");
    }
    // ponytail: PHPStan supplies a base type for an intersection of accessories only; nothing in Pun builds one
    Type::Intersection(types)
}

/// `TemplateTypeFactory::create()`: a template keeps only the bounds it has a template type for (other bounds,
/// such as `class-string` or a constant float, make it a plain mixed template).
fn template_bound(bound: Type) -> Type {
    match bound {
        Type::Object(..)
        | Type::EnumCase(..)
        | Type::ObjectWithoutClass
        | Type::Array(..)
        | Type::ConstArray(_)
        | Type::ObjectShape(_)
        | Type::String
        | Type::ConstString(_)
        | Type::Int
        | Type::IntRange(..)
        | Type::ConstInt(_)
        | Type::Float
        | Type::Bool
        | Type::ConstBool(_)
        | Type::Mixed { .. }
        | Type::Union(_)
        | Type::Benevolent(_)
        | Type::Intersection(_)
        | Type::Iterable(..)
        | Type::Null => bound,
        _ => Type::Mixed { explicit: true },
    }
}

/// `IterableType::isSuperTypeOfMixed()`
fn iterable_super_of_mixed(key: &Type, value: &Type, b: &Type) -> Tri {
    // `isNestedTypeSuperTypeOf()`: an explicit `mixed` is only maybe wider than an implicit one
    let nested = |x: &Type, y: &Type| match (x, y) {
        (Type::Mixed { explicit: true }, Type::Mixed { explicit: false }) => Tri::Maybe,
        (Type::Mixed { .. }, Type::Mixed { .. }) => Tri::Yes,
        _ => is_super_type_of(x, y),
    };
    b.is_iterable()
        .and(nested(value, &b.iterable_value_type()))
        .and(nested(key, &b.iterable_key_type()))
}

/// `IntegerRangeType::tryIntersect()`
fn range_try_intersect(min: Option<i64>, max: Option<i64>, other: &Type) -> Option<Type> {
    let (omin, omax) = other.int_bounds()?;
    // the tighter bound of each side; a missing one is unbounded
    let bound = |x: Option<i64>, y: Option<i64>, tighter: fn(i64, i64) -> i64| match (x, y) {
        (Some(x), Some(y)) => Some(tighter(x, y)),
        (x, y) => x.or(y),
    };
    Some(Type::int_range(
        bound(min, omin, i64::max),
        bound(max, omax, i64::min),
    ))
}

/// The `Type` queries and `TypeCombinator` helpers that declared types need (TypehintHelper::decideType()).
impl Type {
    /// `isNull()`
    pub fn is_null(&self) -> Tri {
        self.query_with_mixed_domain(Self::is_null, || Type::Null, false, &|t| Tri::from(matches!(t, Type::Null)))
    }

    /// `isVoid()`
    pub fn is_void(&self) -> Tri {
        if matches!(self, Type::Intersection(_)) { return Tri::No; }
        self.query_with_mixed_domain(Self::is_void, || Type::Void, true, &|t| Tri::from(matches!(t, Type::Void)))
    }

    /// `$type instanceof MixedType && !$type->isExplicitMixed()`
    pub fn is_implicit_mixed(&self) -> bool {
        matches!(self, Type::Mixed { explicit: false } | Type::SubtractedMixed { explicit: false, .. })
    }

    /// `TypeCombinator::addNull()`: unchanged when it may already be null (`never` is not null, so it becomes null)
    pub fn add_null(self) -> Type {
        if self.is_null().no() {
            union(vec![self, Type::Null])
        } else {
            self
        }
    }

    /// `TypeCombinator::removeNull()`
    pub fn remove_null(self) -> Type {
        match self {
            Type::Null => Type::Never(NeverKind::Implicit),
            Type::Union(types) if types.iter().any(|t| matches!(t, Type::Null)) => union(
                types
                    .into_iter()
                    .filter(|t| !matches!(t, Type::Null))
                    .collect(),
            ),
            Type::Benevolent(types) if types.iter().any(|t| matches!(t, Type::Null)) => {
                match union(
                    types
                        .into_iter()
                        .filter(|t| !matches!(t, Type::Null))
                        .collect(),
                ) {
                    Type::Union(types) => Type::Benevolent(types),
                    t => t,
                }
            }
            t => t,
        }
    }

    /// `TemplateTypeHelper::resolveToBounds()`
    pub fn resolve_to_bounds(&self) -> Type {
        match self {
            Type::Template(t) => t.bound.resolve_to_bounds(),
            t => t.traverse(&Type::resolve_to_bounds),
        }
    }
}

/// `TypehintHelper::decideType()`: the PHPDoc type where it narrows the native one.
pub fn decide_type(ty: Type, phpdoc: Option<Type>) -> Type {
    let phpdoc = phpdoc.map(|p| {
        if ty.is_null().no() {
            p.remove_null()
        } else {
            p
        }
    });
    if matches!(ty, Type::Benevolent(_)) {
        return ty;
    }
    let Some(mut doc) = phpdoc.filter(|p| !matches!(p, Type::Error)) else {
        return ty;
    };
    if matches!(
        doc,
        Type::Never(NeverKind::Explicit | NeverKind::NonAccepting)
    ) {
        return doc;
    }
    if ty.is_implicit_mixed() && doc.is_void().yes() {
        return doc;
    }
    let mixed_key_array = |t: &Type| match t {
        Type::Array(key, item) if key.describe(Level::TypeOnly) == "mixed" => {
            Some(Type::Iterable(key.clone(), item.clone()))
        }
        _ => None,
    };
    if matches!(ty.clone().remove_null(), Type::Iterable(..)) {
        if let Type::Union(members) = &doc {
            doc = Type::Union(
                members
                    .iter()
                    // (a member takes its iterable key type, `array-key`, not its key type)
                    .map(|m| match mixed_key_array(m) {
                        Some(Type::Iterable(_, item)) => {
                            Type::Iterable(Box::new(m.iterable_key_type()), item)
                        }
                        _ => m.clone(),
                    })
                    .collect(),
            );
        } else if let Some(iterable) = mixed_key_array(&doc) {
            doc = iterable;
        }
    }
    let narrows = (ty.is_callable().yes() && doc.is_callable().yes())
        || ((!matches!(doc, Type::Never(_)) || ty.is_implicit_mixed())
            && is_super_type_of(&ty, &doc.resolve_to_bounds()).yes());
    let result = if narrows { doc } else { ty.clone() };
    match &ty {
        Type::Union(members) => {
            let add: Vec<Type> = members
                .iter()
                .filter(|m| is_super_type_of(m, &result).no())
                .cloned()
                .collect();
            if add.is_empty() {
                result
            } else {
                union(std::iter::once(result).chain(add).collect())
            }
        }
        // `TypeCombinator::containsNull()` of a type that is not a union
        Type::Null => result.add_null(),
        _ => result,
    }
}
