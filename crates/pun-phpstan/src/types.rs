//! PHPStan's type representation (the subset built-in signatures use), with `describe()` at each
//! `VerbosityLevel`, `isSuperTypeOf()` and `TypeCombinator::union()` / `intersect()`.
//!
//! Each variant is one of PHPStan 2.2.7's `Type` classes; methods keep upstream's control flow so a PHP and a
//! Rust function can be read side by side. Class knowledge (canonical names, ancestors) comes from
//! [`crate::symbols`]: PHPStan's own reflection of built-in classes.

use std::cmp::Ordering;

use crate::symbols;

mod accepts;
mod casts;
mod templates;
mod verbosity;
pub use accepts::{StrictMixed, accepts, accepts_with};
pub use casts::{Cast, cast_is_error, is_numeric};
pub use templates::{TemplateMap, infer_template_types, resolve_template_types};
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
    fn extreme_identity(results: impl IntoIterator<Item = Tri>) -> Tri {
        let mut it = results.into_iter();
        let first = it.next().unwrap_or(Tri::No);
        if it.all(|r| r == first) {
            first
        } else {
            Tri::Maybe
        }
    }
    /// `TrinaryLogic::lazyMaxMin()`: any yes → yes, else the minimum.
    fn max_min(results: impl IntoIterator<Item = Tri>) -> Tri {
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
    fn handle(self, type_only: impl FnOnce() -> String, value: impl FnOnce() -> String) -> String {
        match self {
            Level::TypeOnly => type_only(),
            _ => value(),
        }
    }
    fn handle3(
        self,
        type_only: impl FnOnce() -> String,
        value: impl FnOnce() -> String,
        precise: impl FnOnce() -> String,
    ) -> String {
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct Template {
    pub name: String,
    /// `TemplateTypeScope::describe()`, e.g. `function preg_match()`, `class ArrayIterator`
    pub scope: String,
    pub bound: Type,
    pub default: Option<Type>,
    /// a class's own template as the argument of its `static` / `$this` type
    pub argument: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Mixed {
        explicit: bool,
    },
    Never(NeverKind),
    Error,
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
    ConstString(String),
    ClassString(Option<Box<Type>>),
    /// `__stringAndStringable` (`StringAlwaysAcceptingObjectWithToStringType`)
    StringAcceptingStringable,
    /// `__stringNotStringable` (`StringNeverAcceptingObjectWithToStringType`)
    StringNotAcceptingStringable,
    Resource,
    ObjectWithoutClass,
    /// `ObjectType` (no args) or `GenericObjectType`, with the class name as written
    Object(String, Vec<Type>),
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

fn bx(t: Type) -> Box<Type> {
    Box::new(t)
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
        } else {
            key
        };
        Type::Array(bx(key), bx(item))
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
        matches!(self, Type::Never(NeverKind::Implicit))
    }

    fn is_never(&self) -> bool {
        matches!(self, Type::Never(_))
    }

    /// `$t instanceof MixedType && $t->describe(precise) === 'mixed' && !$t->isExplicitMixed()`
    fn is_plain_implicit_mixed(&self) -> bool {
        matches!(self, Type::Mixed { explicit: false })
    }

    fn is_compound(&self) -> bool {
        matches!(
            self,
            Type::Mixed { .. }
                | Type::Error
                | Type::Never(_)
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
        match self {
            Type::Union(types) => Tri::extreme_identity(types.iter().map(f)),
            Type::Benevolent(types) => types.iter().map(f).fold(Tri::No, Tri::or),
            Type::Intersection(types) => Tri::max_min(types.iter().map(f)),
            Type::Template(t) => t.bound.query(f),
            Type::Mixed { .. } | Type::Error => Tri::Maybe,
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

    /// A callable array (`callable&array`): `array{class-string|object, non-falsy-string}`.
    fn is_callable_array(&self) -> bool {
        matches!(self, Type::Intersection(_)) && self.is_callable().yes() && self.is_array().yes()
    }

    pub fn is_string(&self) -> Tri {
        self.query_c(&|t| {
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
                    )
            ))
        })
    }

    fn is_integer(&self) -> Tri {
        self.query(&|t| {
            Tri::from(matches!(
                t,
                Type::Int | Type::ConstInt(_) | Type::IntRange(..)
            ))
        })
    }

    pub fn is_float(&self) -> Tri {
        self.query(&|t| Tri::from(matches!(t, Type::Float | Type::ConstFloat(_))))
    }

    pub fn is_boolean(&self) -> Tri {
        self.query(&|t| Tri::from(matches!(t, Type::Bool | Type::ConstBool(_))))
    }

    fn is_class_string(&self) -> Tri {
        self.query_c(&|t| match t {
            Type::ClassString(_) => Tri::Yes,
            Type::String | Type::StringAcceptingStringable | Type::StringNotAcceptingStringable => {
                Tri::Maybe
            }
            Type::ConstString(s) => Tri::from(symbols::lookup(s).is_some()),
            Type::Accessory(
                Accessory::NonEmptyString | Accessory::NonFalsyString | Accessory::LiteralString,
            ) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    pub fn is_array(&self) -> Tri {
        self.query_c(&|t| {
            Tri::from(matches!(
                t,
                Type::Array(..)
                    | Type::ConstArray(_)
                    | Type::Accessory(Accessory::List | Accessory::NonEmptyArray)
            ))
        })
    }

    fn is_constant_array(&self) -> Tri {
        if self.is_callable_array() {
            return Tri::Yes;
        }
        self.query_c(&|t| match t {
            Type::ConstArray(_) => Tri::Yes,
            Type::Array(..) | Type::Accessory(Accessory::List | Accessory::NonEmptyArray) => {
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
        matches!(
            self,
            Type::ConstBool(_)
                | Type::ConstInt(_)
                | Type::ConstFloat(_)
                | Type::ConstString(_)
                | Type::Null
        )
    }

    pub fn is_list(&self) -> Tri {
        if self.is_callable_array() {
            return Tri::Yes;
        }
        self.query_c(&|t| match t {
            Type::Accessory(Accessory::List) => Tri::Yes,
            Type::ConstArray(a) => a.is_list,
            // `ArrayType::isList()`: no when no key can be a non-negative int
            Type::Array(key, _) if is_super_type_of(&Type::int_range(Some(0), None), key).no() => {
                Tri::No
            }
            Type::Array(..) | Type::Accessory(Accessory::NonEmptyArray) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    pub fn is_iterable(&self) -> Tri {
        if self.is_never() {
            return Tri::Yes;
        }
        self.query_c(&|t| match t {
            Type::Array(..)
            | Type::ConstArray(_)
            | Type::Iterable(..)
            | Type::Accessory(Accessory::List | Accessory::NonEmptyArray) => Tri::Yes,
            Type::Object(class, _) | Type::Static(class, _) | Type::This(class) => {
                is_instance_of(class, "Traversable")
            }
            Type::ObjectWithoutClass | Type::ObjectShape(_) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    /// `isCloneable()`: an object, `mixed` and `never`; a common callable or iterable maybe.
    pub fn is_cloneable(&self) -> Tri {
        match self {
            Type::Mixed { .. } | Type::Error | Type::Never(_) => Tri::Yes,
            Type::Template(t) => t.bound.is_cloneable(),
            _ => self.query(&|t| match t {
                Type::Object(..)
                | Type::ObjectWithoutClass
                | Type::ObjectShape(_)
                | Type::Static(..)
                | Type::This(_)
                | Type::EnumCase(..)
                | Type::Closure(_) => Tri::Yes,
                Type::Callable(_) | Type::Iterable(..) => Tri::Maybe,
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
            _ => self.query(&|t| match t {
                Type::ConstArray(a) => a.is_iterable_at_least_once(),
                Type::Accessory(Accessory::NonEmptyArray) => Tri::Yes,
                Type::Array(..) | Type::Iterable(..) | Type::Accessory(Accessory::List) => {
                    Tri::Maybe
                }
                Type::Object(..) | Type::ObjectWithoutClass | Type::Callable(_) => Tri::Maybe,
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
            Type::ConstString(s) => Tri::from(!s.is_empty() && s != "0"),
            Type::ClassString(_) => Tri::Yes,
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::Accessory(_) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    fn is_numeric_string(&self) -> Tri {
        self.query(&|t| match t {
            Type::Accessory(Accessory::NumericString)
            | Type::Accessory(Accessory::DecimalIntString { inverse: false }) => Tri::Yes,
            Type::ConstString(s) => Tri::from(crate::value::numeric(s).is_some()),
            Type::ClassString(_) => Tri::No,
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::Accessory(_) => Tri::Maybe,
            _ => Tri::No,
        })
    }

    /// `isDecimalIntegerString()`
    fn is_decimal_int_string(&self) -> Tri {
        self.query(&|t| match t {
            Type::Accessory(Accessory::DecimalIntString { inverse }) => Tri::from(!inverse),
            Type::ConstString(s) => Tri::from(s.parse::<i64>().is_ok_and(|i| i.to_string() == *s)),
            Type::String
            | Type::StringAcceptingStringable
            | Type::StringNotAcceptingStringable
            | Type::Accessory(
                Accessory::NonEmptyString
                | Accessory::NonFalsyString
                | Accessory::NumericString
                | Accessory::LiteralString
                | Accessory::LowercaseString
                | Accessory::UppercaseString,
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
        self.query(&|t| match t {
            Type::Callable(_) | Type::Closure(_) => Tri::Yes,
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
            Type::Object(class, _) | Type::Static(class, _) | Type::This(class) => {
                match symbols::lookup(class) {
                    Some(c) if has_method(&c, "__invoke") => Tri::Yes,
                    Some(c) if c.is_final => Tri::No,
                    _ => Tri::Maybe,
                }
            }
            _ => Tri::No,
        })
    }

    /// `getObjectClassNames()`
    pub fn object_class_names(&self) -> Vec<String> {
        match self {
            Type::Object(class, _)
            | Type::Static(class, _)
            | Type::This(class)
            | Type::EnumCase(class, _) => vec![class.clone()],
            Type::Closure(_) => vec!["Closure".to_string()],
            Type::Union(types) | Type::Intersection(types) => {
                let mut names = Vec::new();
                for inner in types.iter().map(Type::object_class_names) {
                    // `pickFromTypes()`: a union member without names leaves none
                    if inner.is_empty() && matches!(self, Type::Union(_)) {
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
            // ponytail: templates are compound, so isSuperTypeOf() never asks them; add their arm when something
            // else does
            _ => vec![],
        }
    }

    pub fn iterable_key_type(&self) -> Type {
        match self {
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
            Type::Object(class, args) => iterable_arg(class, args, true),
            Type::Static(class, args) => static_of(class, args).iterable_key_type(),
            Type::This(class) => static_object(class).iterable_key_type(),
            Type::Never(_) => Type::Never(NeverKind::Implicit),
            _ => mixed(),
        }
    }

    pub fn iterable_value_type(&self) -> Type {
        match self {
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
            Type::Object(class, args) => {
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
        match self {
            Type::Mixed { .. } => "mixed".to_string(),
            Type::Error => level.handle3(|| "mixed".into(), || "mixed".into(), || "*ERROR*".into()),
            Type::Never(NeverKind::NonAccepting) => "never".to_string(),
            Type::Never(_) => "*NEVER*".to_string(),
            Type::Void => "void".to_string(),
            Type::Null => "null".to_string(),
            Type::Bool => "bool".to_string(),
            Type::ConstBool(b) => (if *b { "true" } else { "false" }).to_string(),
            Type::Int => "int".to_string(),
            Type::ConstInt(i) => level.handle(|| "int".into(), || i.to_string()),
            Type::IntRange(min, max) => format!(
                "int<{}, {}>",
                min.map_or("min".to_string(), |v| v.to_string()),
                max.map_or("max".to_string(), |v| v.to_string())
            ),
            Type::Float => "float".to_string(),
            Type::ConstFloat(f) => level.handle(|| "float".into(), || float_to_string(*f)),
            Type::String | Type::StringAcceptingStringable | Type::StringNotAcceptingStringable => {
                "string".to_string()
            }
            Type::ConstString(s) => level.handle3(
                || "string".into(),
                || {
                    if symbols::lookup(s).is_some_and(|c| c.name == *s) {
                        export_string(s)
                    } else {
                        export_string(&crate::schema::truncate(s, 20, "…"))
                    }
                },
                || export_string(s),
            ),
            Type::ClassString(None) => "class-string".to_string(),
            Type::ClassString(Some(t)) => format!("class-string<{}>", t.describe(level)),
            Type::Resource => "resource".to_string(),
            Type::ObjectWithoutClass => "object".to_string(),
            Type::Object(class, args) => {
                let name = match level {
                    Level::Precise => class.clone(),
                    _ => canonical_class(class),
                };
                if args.is_empty() {
                    name
                } else {
                    let args: Vec<String> = args.iter().map(|a| a.describe(level)).collect();
                    format!("{name}<{}>", args.join(", "))
                }
            }
            Type::Static(class, args) => {
                format!("static({})", static_of(class, args).describe(level))
            }
            Type::This(class) => format!("$this({})", static_object(class).describe(level)),
            Type::EnumCase(class, case) => format!("{class}::{case}"),
            Type::Wildcard => "*".to_string(),
            // `TypeProjectionHelper::describe()`
            Type::Projection(c, t) => format!("{} {}", variance_name(*c), t.describe(level)),
            // (`Printer::print($this->toPhpDocNode())`)
            Type::OffsetAccess(..) => to_phpdoc(self),
            Type::Array(key, item) => {
                let mixed_key = key.is_plain_implicit_mixed();
                let mixed_item = item.is_plain_implicit_mixed();
                if mixed_key || (level != Level::Precise && key.is_never()) {
                    if mixed_item || (level != Level::Precise && item.is_never()) {
                        return "array".to_string();
                    }
                    return format!("array<{}>", item.describe(level));
                }
                format!("array<{}, {}>", key.describe(level), item.describe(level))
            }
            Type::ConstArray(a) => a.describe(level),
            Type::Iterable(key, item) => {
                if matches!(**key, Type::Mixed { .. }) {
                    if matches!(**item, Type::Mixed { .. }) {
                        return "iterable".to_string();
                    }
                    return format!("iterable<{}>", item.describe(level));
                }
                format!(
                    "iterable<{}, {}>",
                    key.describe(level),
                    item.describe(level)
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
            Type::Benevolent(types) => format!("({})", describe_union(types, level)),
            Type::Intersection(types) => match level {
                Level::TypeOnly => describe_intersection_type(self, types, level),
                _ => describe_intersection_itself(self, types, level),
            },
            Type::Accessory(a) => a.describe().to_string(),
            Type::Conditional(c) => {
                let subject = match &c.subject {
                    Subject::Parameter(p) => p.clone(),
                    Subject::Type(t) => t.describe(level),
                };
                format!(
                    "({} {} {} ? {} : {})",
                    subject,
                    if c.negated { "is not" } else { "is" },
                    c.target.describe(level),
                    c.then.describe(level),
                    c.otherwise.describe(level)
                )
            }
            Type::Template(t) => {
                let mut basic = t.name.clone();
                if !t.bound.is_implicit_or_explicit_plain_mixed() {
                    basic.push_str(" of ");
                    basic.push_str(&t.bound.describe(level));
                }
                if let Some(default) = &t.default {
                    basic.push_str(" = ");
                    basic.push_str(&default.describe(level));
                }
                if level == Level::Precise {
                    format!(
                        "{basic} ({}, {})",
                        t.scope,
                        if t.argument { "argument" } else { "parameter" }
                    )
                } else {
                    basic
                }
            }
            Type::ObjectShape(props) => {
                let items: Vec<String> = props
                    .iter()
                    .map(|(name, ty, optional)| {
                        format!(
                            "{name}{}: {}",
                            if *optional { "?" } else { "" },
                            ty.describe(level)
                        )
                    })
                    .collect();
                format!("object{{{}}}", items.join(", "))
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
            Type::EnumCase(class, _) => Type::Object(class.clone(), vec![]),
            _ => self.traverse(&Type::generalize),
        }
    }

    /// `traverse()`: the same type with `f` applied to its direct inner types.
    pub fn traverse(&self, f: &dyn Fn(&Type) -> Type) -> Type {
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
            Type::Array(k, v) => Type::array(f(k), f(v)),
            Type::Iterable(k, v) => Type::Iterable(bx(f(k)), bx(f(v))),
            Type::Union(types) => union(types.iter().map(f).collect()),
            Type::Benevolent(types) => to_benevolent(union(types.iter().map(f).collect())),
            Type::Intersection(types) => {
                let mapped: Vec<Type> = types.iter().map(f).collect();
                if mapped == *types {
                    self.clone()
                } else {
                    intersect(mapped)
                }
            }
            Type::Callable(Some(s)) => Type::Callable(Some(Box::new(sig(s)))),
            Type::Closure(s) => Type::Closure(Box::new(sig(s))),
            Type::Object(class, args) if !args.is_empty() => {
                Type::Object(class.clone(), args.iter().map(f).collect())
            }
            Type::Projection(c, t) => Type::Projection(*c, bx(f(t))),
            Type::OffsetAccess(t, o) => Type::OffsetAccess(bx(f(t)), bx(f(o))),
            Type::ClassString(Some(g)) => Type::ClassString(Some(bx(f(g)))),
            Type::Static(class, args) if !args.is_empty() => {
                Type::Static(class.clone(), args.iter().map(f).collect())
            }
            Type::Template(t) => Type::Template(Box::new(Template {
                bound: f(&t.bound),
                ..(**t).clone()
            })),
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
            // `ConstantArrayType::traverse()`: the values and the unsealed `...<K, V>`'s value, not the keys
            Type::ConstArray(a) => {
                let mut a = a.clone();
                a.values = a.values.iter().map(f).collect();
                a.unsealed = a.unsealed.map(|(k, v)| (k, f(&v)));
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
    fn describe(&self) -> &'static str {
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
        }
    }

    fn is_string_accessory(&self) -> bool {
        !matches!(self, Accessory::List | Accessory::NonEmptyArray)
    }
}

/// `StaticType::getStaticObjectType()`: the class, with its own templates as arguments when it is generic.
fn static_object(class: &str) -> Type {
    // a class the lookup does not know (outside `declared::with_classes()`) has no templates to pass on
    let Some(c) = symbols::lookup(class) else {
        return Type::Object(class.to_string(), Vec::new());
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
    Type::Object(c.name.clone(), args)
}

/// `getStaticObjectType()` of a `static` (`GenericStaticType`'s is the class with its arguments).
fn static_of(class: &str, args: &[Type]) -> Type {
    if args.is_empty() {
        static_object(class)
    } else {
        Type::Object(class.to_string(), args.to_vec())
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
fn string_is_callable(s: &str) -> Tri {
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
fn export_string(value: &str) -> String {
    if value.bytes().any(|b| b < 0x20) {
        let mut out = String::from("\"");
        for c in value.chars() {
            match c {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                '\t' => out.push_str("\\t"),
                '\r' => out.push_str("\\r"),
                '\x0b' => out.push_str("\\v"),
                '\x0c' => out.push_str("\\f"),
                '\x07' => out.push_str("\\a"),
                '\x08' => out.push_str("\\b"),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\{:03o}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
        return out;
    }
    format!("'{}'", value.replace('\\', "\\\\").replace('\'', "\\'"))
}

/// `UnionTypeHelper::sortTypes()`
pub fn sort_types(types: &[Type]) -> Vec<Type> {
    let mut sorted = types.to_vec();
    if sorted.len() > 1024 {
        return sorted;
    }
    sorted.sort_by(compare_in_union);
    sorted
}

fn compare_strings(a: &str, b: &str) -> Ordering {
    let cmp = a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase());
    if cmp != Ordering::Equal {
        cmp
    } else {
        a.cmp(b)
    }
}

fn compare_in_union(a: &Type, b: &Type) -> Ordering {
    if matches!(a, Type::Null) {
        return Ordering::Greater;
    } else if matches!(b, Type::Null) {
        return Ordering::Less;
    }
    match (a, b) {
        (Type::Accessory(_), Type::Accessory(_)) => {
            return compare_strings(&a.describe(Level::Value), &b.describe(Level::Value));
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
    let a_scalar = a.is_constant_scalar() && !matches!(a, Type::Null);
    let b_scalar = b.is_constant_scalar() && !matches!(b, Type::Null);
    if a_scalar != b_scalar {
        return b_scalar.cmp(&a_scalar);
    }
    let number = |t: &Type| match t {
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
        return matches!(b, Type::ConstInt(_)).cmp(&matches!(a, Type::ConstInt(_)));
    }
    match (a, b) {
        (Type::IntRange(a_min, _), Type::IntRange(b_min, _)) => {
            return a_min.unwrap_or(i64::MIN).cmp(&b_min.unwrap_or(i64::MIN));
        }
        (Type::IntRange(..), _) if b.is_plain_integer_class() => return Ordering::Greater,
        (_, Type::IntRange(..)) if a.is_plain_integer_class() => return Ordering::Less,
        (Type::ConstString(x), Type::ConstString(y)) => return compare_strings(x, y),
        _ => {}
    }
    let callable = |t: &Type| matches!(t, Type::Callable(_) | Type::Closure(_));
    if callable(a) && callable(b) {
        return compare_strings(&a.describe(Level::Value), &b.describe(Level::Value));
    }
    if a.is_constant_array().yes() && b.is_constant_array().yes() {
        // `array{}` first
        let empty = |t: &Type| t.is_iterable_at_least_once().no();
        return empty(b)
            .cmp(&empty(a))
            .then_with(|| compare_strings(&a.describe(Level::Value), &b.describe(Level::Value)));
    }
    if a.is_string().yes() && b.is_string().yes() {
        return compare_strings(&a.describe(Level::Precise), &b.describe(Level::Precise));
    }
    compare_strings(&a.describe(Level::TypeOnly), &b.describe(Level::TypeOnly))
}

impl Type {
    /// `$t instanceof IntegerType` for `IntegerRangeType` comparisons: TemplateIntegerType /
    /// TemplateConstantIntegerType.
    fn is_plain_integer_class(&self) -> bool {
        // (an `int` or a constant int never meets a range when sorting: it was merged, or sorted as a scalar)
        matches!(self, Type::Template(t) if matches!(t.bound, Type::Int | Type::ConstInt(_) | Type::IntRange(..)))
    }
}

fn describe_union(types: &[Type], level: Level) -> String {
    let join = |types: &[Type]| -> String {
        let mut names: Vec<String> = Vec::new();
        for (i, t) in types.iter().enumerate() {
            let d = t.describe(level);
            let wrap = match t {
                Type::Closure(_) | Type::Callable(_) => true,
                Type::Template(tpl) if matches!(tpl.bound, Type::Union(_)) => true,
                Type::Template(tpl) => {
                    i + 1 < types.len()
                        && level != Level::Precise
                        && !matches!(tpl.bound, Type::Mixed { .. })
                }
                Type::Intersection(_) => d.contains('&'),
                _ => false,
            };
            names.push(if wrap { format!("({d})") } else { d });
        }
        // ponytail: at the precise level PHPStan numbers equal descriptions (`T#1|T#2`); no union of Pun's holds two
        // members that describe the same precisely, so only the other levels dedupe
        if level != Level::Precise {
            let mut seen = std::collections::HashSet::new();
            names.retain(|n| seen.insert(n.clone()));
        }
        if names.len() > 1024 {
            names.truncate(1024);
            names.push("…".to_string());
        }
        names.join("|")
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
                    t.generalize()
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

fn describe_intersection_type(this: &Type, types: &[Type], level: Level) -> String {
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
        names.push(t.generalize().describe(level));
    }
    if is_list {
        let inner = match value_type {
            Some(v) if !v.is_plain_implicit_mixed() => format!("<{}>", v.describe(level)),
            _ => String::new(),
        };
        names.push(format!("list{inner}"));
    }
    names.sort_by(|a, b| compare_strings(a, b));
    names.join("&")
}

fn describe_intersection_itself(this: &Type, types: &[Type], level: Level) -> String {
    let mut base: Vec<(usize, &Type)> = Vec::new();
    let mut to_describe: Vec<(usize, Type)> = Vec::new();
    let mut skip_names: Vec<&str> = Vec::new();
    let is_list = this.is_list().yes();
    let is_array = this.is_array().yes();
    let is_non_empty_array = this.is_iterable_at_least_once().yes();
    let mut described: Vec<(usize, String)> = Vec::new();
    let sorted = sort_types(types);
    for (i, t) in sorted.iter().enumerate() {
        if let Type::Accessory(a) = t
            && a.is_string_accessory()
        {
            if matches!(a, Accessory::LowercaseString | Accessory::UppercaseString)
                && level != Level::Precise
            {
                continue;
            }
            to_describe.push((i, t.clone()));
            skip_names.push("string");
            continue;
        }
        if is_list || is_array {
            match t {
                Type::Array(key, value) => {
                    if is_list {
                        let inner = if value.is_plain_implicit_mixed() {
                            String::new()
                        } else {
                            format!("<{}>", value.describe(level))
                        };
                        described.push((
                            i,
                            format!(
                                "{}{inner}",
                                if is_non_empty_array {
                                    "non-empty-list"
                                } else {
                                    "list"
                                }
                            ),
                        ));
                    } else {
                        let inner = if !key.is_plain_implicit_mixed() {
                            format!("<{}, {}>", key.describe(level), value.describe(level))
                        } else if !value.is_plain_implicit_mixed() {
                            format!("<{}>", value.describe(level))
                        } else {
                            String::new()
                        };
                        let kind = if is_non_empty_array {
                            "non-empty-array"
                        } else {
                            "array"
                        };
                        described.push((i, format!("{kind}{inner}")));
                    }
                    continue;
                }
                Type::ConstArray(a) => {
                    let d = a.describe(level);
                    let rest = d
                        .strip_prefix("list")
                        .unwrap_or_else(|| &d["array".len()..]);
                    let mut begin = String::from(if is_list { "list" } else { "array" });
                    if is_non_empty_array && !t.is_iterable_at_least_once().yes() {
                        begin = format!("non-empty-{begin}");
                    }
                    described.push((i, format!("{begin}{rest}")));
                    continue;
                }
                Type::Accessory(Accessory::NonEmptyArray | Accessory::List) => continue,
                _ => {}
            }
        }
        if matches!(t, Type::Callable(None)) {
            to_describe.push((i, t.clone()));
            skip_names.push("object");
            skip_names.push("string");
            continue;
        }
        // Every accessory was handled above (string ones always, array ones as `is_array`).
        base.push((i, t));
    }
    'base: for (i, t) in base {
        let d = t.describe(level);
        if (d == "object" || d == "string")
            && skip_names.contains(&d.as_str())
            && let Some(pos) = to_describe
                .iter()
                .position(|(_, x)| matches!(x, Type::Callable(None)))
        {
            described.push((i, format!("callable-{d}")));
            to_describe.remove(pos);
            continue 'base;
        }
        if skip_names.contains(&d.as_str()) {
            continue;
        }
        described.push((i, d));
    }
    for (i, t) in to_describe {
        described.push((i, t.describe(level)));
    }
    described.sort_by_key(|(i, _)| *i);
    described
        .into_iter()
        .map(|(_, d)| d)
        .collect::<Vec<_>>()
        .join("&")
}

/// `Printer::print()` of `CallableType::toPhpDocNode()` / `ClosureType` with parameter names dropped.
fn print_callable(ident: &str, sig: &Signature) -> String {
    let params: Vec<String> = sig
        .params
        .iter()
        .map(|p| {
            format!(
                "{}{}{}",
                to_phpdoc(&p.ty),
                if p.variadic { " ..." } else { "" },
                if p.optional && !p.variadic { "=" } else { "" }
            )
        })
        .collect();
    let ret = to_phpdoc(&sig.ret);
    let ret = if matches!(
        sig.ret,
        Type::Union(_)
            | Type::Benevolent(_)
            | Type::Intersection(_)
            | Type::Callable(Some(_))
            | Type::Closure(_)
    ) && ret.contains(['|', '&', '('])
    {
        format!("({ret})")
    } else {
        ret
    };
    format!("{ident}{}({}): {ret}", print_templates(sig), params.join(", "))
}

/// A callable's `<T of Bound = Default, ...>` (its TemplateTagValueNodes, the bound always printed).
fn print_templates(sig: &Signature) -> String {
    if sig.templates.is_empty() {
        return String::new();
    }
    let tags: Vec<String> = sig
        .templates
        .iter()
        // (a TemplateTagValueNode of the bound only)
        .map(|t| format!("{} of {}", t.name, to_phpdoc(&t.bound)))
        .collect();
    format!("<{}>", tags.join(", "))
}

/// `Printer::print($type->toPhpDocNode())`
pub fn to_phpdoc(t: &Type) -> String {
    match t {
        Type::Mixed { .. } | Type::Error => "mixed".to_string(),
        Type::Never(_) => "never".to_string(),
        Type::ConstInt(i) => i.to_string(),
        Type::ConstFloat(f) => float_to_string(*f),
        Type::ConstString(s) => format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'")),
        Type::IntRange(min, max) => format!(
            "int<{}, {}>",
            min.map_or("min".to_string(), |v| v.to_string()),
            max.map_or("max".to_string(), |v| v.to_string())
        ),
        Type::ClassString(Some(g)) => format!("class-string<{}>", to_phpdoc(g)),
        Type::Object(class, args) if !args.is_empty() => {
            format!(
                "{class}<{}>",
                args.iter().map(to_phpdoc).collect::<Vec<_>>().join(", ")
            )
        }
        Type::Object(class, _) => class.clone(),
        Type::Static(..) => "static".to_string(),
        Type::This(_) => "$this".to_string(),
        Type::Array(key, item) => {
            if key.is_plain_implicit_mixed() {
                if item.is_plain_implicit_mixed() {
                    return "array".to_string();
                }
                return format!("array<{}>", to_phpdoc(item));
            }
            format!("array<{}, {}>", to_phpdoc(key), to_phpdoc(item))
        }
        Type::Iterable(key, item) => {
            if matches!(**key, Type::Mixed { .. }) {
                if matches!(**item, Type::Mixed { .. }) {
                    return "iterable".to_string();
                }
                return format!("iterable<{}>", to_phpdoc(item));
            }
            format!("iterable<{}, {}>", to_phpdoc(key), to_phpdoc(item))
        }
        Type::Union(types) | Type::Benevolent(types) => join_phpdoc(&sort_types(types), "|"),
        // `list<T>`, `non-empty-list<T>`, `non-empty-array<K, T>`
        Type::Intersection(types) if let Some(Type::Array(key, item)) = types.iter().find(|t| matches!(t, Type::Array(..))) => {
            let has = |a: Accessory| types.contains(&Type::Accessory(a));
            match (types.len(), has(Accessory::List), has(Accessory::NonEmptyArray)) {
                (2, true, false) if !item.is_plain_implicit_mixed() => format!("list<{}>", to_phpdoc(item)),
                (3, true, true) if !item.is_plain_implicit_mixed() => format!("non-empty-list<{}>", to_phpdoc(item)),
                (2, false, true) if !key.is_plain_implicit_mixed() && !item.is_plain_implicit_mixed() => {
                    format!("non-empty-array<{}, {}>", to_phpdoc(key), to_phpdoc(item))
                }
                _ => t.describe(Level::Precise),
            }
        }
        Type::Intersection(_) => t.describe(Level::Precise),
        Type::Template(tpl) => tpl.name.clone(),
        Type::Callable(None) => "callable".to_string(),
        Type::Callable(Some(sig)) => print_callable_named("callable", sig),
        Type::Closure(sig) => print_callable_named("Closure", sig),
        Type::Projection(c, inner) => format!("{} {}", variance_name(*c), to_phpdoc(inner)),
        Type::OffsetAccess(t, o) => {
            let on = to_phpdoc(t);
            // (Printer's parentheses around a union, intersection or callable)
            let wrap = matches!(**t, Type::Union(_) | Type::Benevolent(_) | Type::Callable(Some(_)) | Type::Closure(_))
                || (matches!(**t, Type::Intersection(_)) && on.contains('&'));
            if wrap { format!("({on})[{}]", to_phpdoc(o)) } else { format!("{on}[{}]", to_phpdoc(o)) }
        }
        _ => t.describe(Level::Precise),
    }
}

fn join_phpdoc(types: &[Type], sep: &str) -> String {
    types
        .iter()
        .map(|t| {
            let s = to_phpdoc(t);
            if matches!(
                t,
                Type::Union(_) | Type::Benevolent(_) | Type::Intersection(_)
            ) && s.contains(['|', '&'])
            {
                format!("({s})")
            } else {
                s
            }
        })
        .collect::<Vec<_>>()
        .join(sep)
}

fn print_callable_named(ident: &str, sig: &Signature) -> String {
    let params: Vec<String> = sig
        .params
        .iter()
        .map(|p| {
            let name = if p.name.is_empty() {
                String::new()
            } else {
                format!("${}", p.name)
            };
            let s = format!(
                "{} {}{}{}",
                to_phpdoc(&p.ty),
                if p.by_ref { "&" } else { "" },
                if p.variadic { "..." } else { "" },
                name
            );
            format!("{}{}", s.trim(), if p.optional { "=" } else { "" })
        })
        .collect();
    let ret = match &sig.ret {
        r @ (Type::Union(_)
        | Type::Benevolent(_)
        | Type::Intersection(_)
        | Type::Callable(Some(_))
        | Type::Closure(_)) => format!("({})", to_phpdoc(r)),
        r => to_phpdoc(r),
    };
    format!("{ident}{}({}): {ret}", print_templates(sig), params.join(", "))
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
        // (an unsealed shape without keys is an `ArrayType`)
        if self.keys.is_empty() {
            return Tri::No;
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
            Type::ConstString(m) => vec![m.as_str()],
            Type::Union(ts) => ts.iter().map(|t| if let Type::ConstString(m) = t { Some(m.as_str()) } else { None }).collect::<Option<_>>()?,
            _ => return None,
        };
        match &self.values[0] {
            Type::ConstString(n) => Some((n, true, methods)),
            Type::Object(n, _) | Type::Static(n, _) | Type::This(n) => Some((n, false, methods)),
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

    fn describe(&self, level: Level) -> String {
        let name = if self.should_be_described_as_a_list() {
            "list"
        } else {
            "array"
        };
        if level == Level::TypeOnly {
            if self.is_iterable_at_least_once().no() {
                return name.to_string();
            }
            let key = self.iterable_key_type();
            if key.is_benevolent_array_key() {
                return format!("{name}<{}>", self.iterable_value_type().describe(level));
            }
            return format!(
                "{name}<{}, {}>",
                key.describe(level),
                self.iterable_value_type().describe(level)
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
                Type::ConstString(s) if s.contains('"') => format!("'{s}'"),
                Type::ConstString(s) if s.contains('\'') => format!("\"{s}\""),
                Type::ConstString(s) if !is_valid_identifier(s) => format!("'{s}'"),
                Type::ConstString(s) => s.clone(),
                // the only other key type, a `ConstantIntegerType`
                k => to_phpdoc(k),
            };
            let value = self.values[i].describe(level);
            items.push(format!("{key}{}: {value}", if optional { "?" } else { "" }));
            values.push(value);
        }
        let mut append = String::new();
        if truncate && items.len() > 8 {
            items.truncate(8);
            values.truncate(8);
            append = ", ...".to_string();
        }
        if self.is_unsealed().yes()
            && let Some((key, value)) = &self.unsealed
        {
            if !items.is_empty() {
                append.push_str(", ");
            }
            append.push_str("...");
            let key_description = key.describe(Level::Precise);
            if key.is_plain_implicit_mixed()
                || (self.is_list.yes() && key_description == "int<0, max>")
            {
                if !value.is_plain_implicit_mixed() {
                    append.push_str(&format!("<{}>", value.describe(level)));
                }
            } else {
                append.push_str(&format!(
                    "<{}, {}>",
                    key.describe(level),
                    value.describe(level)
                ));
            }
        }
        format!(
            "{name}{{{}{append}}}",
            if values_only {
                values.join(", ")
            } else {
                items.join(", ")
            }
        )
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

fn is_valid_identifier(s: &str) -> bool {
    let mut bytes = s.bytes().peekable();
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
        match key {
            Type::ConstInt(offset) if !a.next_auto_indexes.is_empty() => {
                let min = *a.next_auto_indexes.iter().min().expect("auto index");
                let max = *a.next_auto_indexes.iter().max().expect("auto index");
                if offset >= 0 {
                    if offset > min {
                        a.is_list = if offset <= max {
                            a.is_list.and(Tri::Maybe)
                        } else {
                            Tri::No
                        };
                    }
                } else {
                    a.is_list = Tri::No;
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
            _ => a.is_list = Tri::No,
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
    match a {
        // `*` is a bivariant `mixed`
        // (a call-site variance is a generic argument's only, compared in `valid_variance()`)
        T::Mixed { .. } | T::Error | T::Wildcard | T::OffsetAccess(..) | T::Projection(..) => Tri::Yes,
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
        T::ConstFloat(x) => match b {
            T::ConstFloat(y) => Tri::from(x == y),
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
                let named = T::Object(x.clone(), vec![]);
                let r = match &**g {
                    T::Template(tpl) => is_super_type_of(&tpl.bound, &named),
                    T::Static(c, args) => is_super_type_of(&static_of(c, args), &named),
                    g => is_super_type_of(g, &named),
                };
                if r.yes() { Tri::Maybe } else { Tri::No }
            }
            T::ClassString(None) => Tri::from(symbols::lookup(x).is_some())
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
                let r = is_super_type_of(g, &T::Object(s.clone(), vec![]));
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
        T::Object(class, args) => object_super(class, args, b),
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
                T::Object(c, _) => {
                    let r = is_super_type_of(&object, b);
                    if r.yes() && symbols::lookup(c).is_some_and(|c| c.is_final) {
                        r
                    } else {
                        r.and(Tri::Maybe)
                    }
                }
                _ => Tri::No,
            }
        }
        T::Array(_, item) => match b {
            T::Array(..) | T::ConstArray(_) => is_super_type_of(item, &b.iterable_value_type())
                .and(is_super_type_of(
                    &a.iterable_key_type(),
                    &b.iterable_key_type(),
                )),
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
            T::Object(c, _) if canonical_class(c) == "Closure" => Tri::Maybe,
            _ => object_super("Closure", &[], b),
        },
        T::EnumCase(class, _) => match b {
            T::EnumCase(..) => Tri::from(a == b),
            T::Object(c, _) if canonical_class(c) == canonical_class(class) => Tri::Maybe,
            T::ObjectWithoutClass => Tri::Maybe,
            _ => Tri::No,
        },
        T::ObjectShape(props) => match b {
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
    fn resolve(&self) -> Type {
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
    fn is_resolvable(&self) -> bool {
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
        .map(|theirs| variant_super(ours, theirs, accept))
        .reduce(Tri::or)
        .expect("a variant")
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
        Type::ConstString(name) if let Some(f) = symbols::function(name) => of(f),
        // `'Class::method'`
        Type::ConstString(name) => name.split_once("::").map(|(c, m)| method(c, m)).unwrap_or_default(),
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

fn object_super(class: &str, args: &[Type], b: &Type) -> Tri {
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
    let result = match (symbols::lookup(class), symbols::lookup(that)) {
        (Some(this), Some(other)) => {
            if this.name == other.name {
                Tri::Yes
            } else if this.is_trait || other.is_trait {
                Tri::No
            } else if other.ancestors.contains(&this.name) {
                Tri::Yes
            } else if this.ancestors.contains(&other.name)
                || (this.is_interface && !other.is_final)
                || (other.is_interface && !this.is_final)
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
    let Type::Object(b_class, b_args) = b else {
        return result;
    };
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
    args.iter()
        .zip(&ancestor)
        .map(|(x, y)| valid_variance(x, y))
        .fold(Tri::Yes, Tri::and)
}

/// `TemplateTypeVariance::isValidVariance()` + `validPosition()` of an invariant template, `*` being bivariant and
/// a `covariant` / `contravariant` argument its call-site variance.
fn valid_variance(a: &Type, b: &Type) -> Tri {
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
            if matches!(b, Type::Never(_)) || matches!(**a, Type::Mixed { .. }) || matches!(b, Type::Mixed { .. }) {
                return Tri::Yes;
            }
            if *c { is_super_type_of(a, b) } else { is_super_type_of(b, a) }
        }
        (_, Type::Projection(..)) => Tri::No,
        (_, Type::Never(_)) | (Type::Mixed { .. }, _) | (_, Type::Mixed { .. }) => Tri::Yes,
        (Type::Benevolent(_), _) if !is_super_type_of(a, b).no() => Tri::Yes,
        (_, Type::Benevolent(_)) if !is_super_type_of(b, a).no() => Tri::Yes,
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
        // (against a mixed `$b`, isSuperTypeOf() answers yes first)
        T::Mixed { .. } | T::Error => Tri::Maybe,
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
                        T::Object("Traversable".into(), vec![]),
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
    // a TemplateBenevolentUnionType is flattened too, and comes back when only its members are left
    let mut benevolent_template: Option<Box<Template>> = None;
    let mut flat = Vec::new();
    for t in types {
        match t {
            Type::Mixed { explicit: false } => return t,
            Type::Never(NeverKind::Implicit) => {}
            Type::Benevolent(inner) => {
                for i in &inner {
                    benevolent.push(i.describe(Level::Value));
                }
                flat.extend(inner);
            }
            Type::Template(tpl) => {
                if let Type::Benevolent(inner) = &tpl.bound {
                    benevolent.extend(inner.iter().map(|i| i.describe(Level::Value)));
                    flat.extend(inner.iter().cloned());
                    benevolent_template.get_or_insert(tpl);
                    continue;
                }
                flat.push(Type::Template(tpl));
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
    // scalar constants per class, in first-seen order: bool, int, float, string, null
    let mut scalars: Vec<(u8, Vec<Type>)> = Vec::new();
    let mut generic = [false; 5];
    let mut ranges = Vec::new();
    let mut rest = Vec::new();
    for t in types {
        if t.is_constant_scalar() {
            let class = scalar_class(&t);
            match scalars.iter_mut().find(|(c, _)| *c == class) {
                Some((_, items)) => {
                    if !items.contains(&t) {
                        items.push(t);
                    }
                }
                None => scalars.push((class, vec![t])),
            }
            continue;
        }
        if t.is_boolean().yes() {
            generic[0] = true;
        } else if t.is_float().yes() {
            generic[2] = true;
        } else if t.is_integer().yes() && !matches!(t, Type::IntRange(..)) {
            generic[1] = true;
        } else if t.is_string().yes() && t.is_class_string().no() && !has_accessory(&t) {
            generic[3] = true;
        }
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
        return match benevolent_template {
            Some(tpl) => Type::Template(Box::new(Template {
                bound: Type::Benevolent(types),
                ..*tpl
            })),
            None => Type::Benevolent(types),
        };
    }
    Type::Union(types)
}

fn scalar_class(t: &Type) -> u8 {
    match t {
        Type::ConstBool(_) => 0,
        Type::ConstInt(_) => 1,
        Type::ConstFloat(_) => 2,
        Type::ConstString(_) => 3,
        _ => 4,
    }
}

/// `TypeUtils::getAccessoryTypes() !== []` of a string type that is not a class string (plain `string` answers
/// maybe to that, so only intersections get here)
fn has_accessory(t: &Type) -> bool {
    matches!(t, Type::Intersection(types) if types.iter().any(|t| matches!(t, Type::Accessory(_))))
}

enum Merge {
    A(Type),
    B(Type),
}

/// `TypeCombinator::compareTypesInUnion()`
fn compare_in_union_merge(a: &Type, b: &Type) -> Option<Merge> {
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
            bx(union(vec![(**ak).clone(), (**bk).clone()])),
            bx(union(vec![(**av).clone(), (**bv).clone()])),
        )));
    }
    // SubtractableType (object and mixed types) keeps the first of two equivalent types
    let subtractable = |t: &Type| {
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
    if subtractable(a) && is_super_type_of(a, b).yes() {
        return Some(Merge::A(a.clone()));
    }
    if subtractable(b) && is_super_type_of(b, a).yes() {
        return Some(Merge::B(b.clone()));
    }
    if is_super_type_of(b, a).yes() {
        return Some(Merge::B(b.clone()));
    }
    if is_super_type_of(a, b).yes() {
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
        && s == "0"
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
    /// `getConstantArrays()`
    fn constant_arrays(&self) -> Vec<ConstArray> {
        match self {
            // `IntersectionType::getConstantArrays()` of `callable&array`: its offsets 0 and 1
            _ if self.is_callable_array() => {
                let item = self.general_arrays().pop().map_or_else(mixed, |(_, v)| v);
                vec![ConstArray {
                    keys: vec![Type::ConstInt(0), Type::ConstInt(1)],
                    values: vec![
                        intersect(vec![
                            item.clone(),
                            Type::Union(vec![Type::ClassString(None), Type::ObjectWithoutClass]),
                        ]),
                        intersect(vec![
                            item,
                            Type::Intersection(vec![
                                Type::String,
                                Type::Accessory(Accessory::NonFalsyString),
                            ]),
                        ]),
                    ],
                    next_auto_indexes: vec![2],
                    optional: vec![],
                    is_list: Tri::Yes,
                    unsealed: None,
                }]
            }
            Type::ConstArray(a) => vec![(**a).clone()],
            Type::Intersection(types) => types.iter().flat_map(Type::constant_arrays).collect(),
            _ => vec![],
        }
    }

    /// `getArrays()` as (iterable key, item) pairs
    fn general_arrays(&self) -> Vec<(Type, Type)> {
        match self {
            Type::Array(_, item) => vec![(self.iterable_key_type(), (**item).clone())],
            Type::Intersection(types) => types.iter().flat_map(Type::general_arrays).collect(),
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
    let (mut general, mut overflowed, mut filled) = (false, false, 0);
    for a in &arrays {
        let constant = a.constant_arrays();
        if constant.is_empty() || !a.is_iterable_at_least_once().no() {
            filled += 1;
        }
        if constant.is_empty() {
            for (key, item) in a.general_arrays() {
                keys.push(key);
                values.push(item);
                general = true;
            }
            continue;
        }
        'array: for ca in constant {
            for (key, value) in ca.keys.iter().zip(&ca.values) {
                values.push(value.clone());
                if seen.contains(key) {
                    continue;
                }
                keys.push(key.clone());
                seen.push(key.clone());
                // the bit index of each key would overflow an int
                if seen.len() >= 63 {
                    general = true;
                    overflowed = true;
                    break 'array;
                }
            }
        }
    }
    if general && (!overflowed || filled > 1) {
        let reduced = reduce_arrays(&arrays, false);
        if reduced.len() == 1 {
            return vec![with(
                reduced.into_iter().next().expect("one array"),
                &accessories,
            )];
        }
        return vec![with(Type::array(union(keys), union(values)), &accessories)];
    }
    reduce_arrays(&arrays, true)
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
fn reduce_arrays(arrays: &[Type], preserve_tagged_unions: bool) -> Vec<Type> {
    let mut new_arrays = Vec::new();
    let mut to_process: Vec<Option<ConstArray>> = Vec::new();
    let mut empty = None;
    for a in arrays {
        // only a general union (`preserve_tagged_unions` false) has arrays that are not constant
        if !a.is_constant_array().yes() {
            return arrays.to_vec();
        }
        if a.is_iterable_at_least_once().no() {
            empty = Some(a.clone());
            continue;
        }
        to_process.extend(a.constant_arrays().into_iter().map(Some));
    }
    if let Some(empty) = empty {
        match empty {
            Type::ConstArray(c) if preserve_tagged_unions => to_process.push(Some(*c)),
            e => new_arrays.push(e),
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
            if preserve_tagged_unions
                && overlap == 1
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
            if preserve_tagged_unions && overlap == a.keys.len() && b.is_keys_superset_of(a) {
                to_process[j] = Some(b.merge_with(a));
                to_process[*i] = None;
                break;
            }
            if preserve_tagged_unions && overlap == b.keys.len() && a.is_keys_superset_of(b) {
                to_process[*i] = Some(a.merge_with(b));
                to_process[j] = None;
                continue;
            }
            if !preserve_tagged_unions && overlap == a.keys.len() && overlap == b.keys.len() {
                to_process[j] = Some(b.merge_with(a));
                to_process[*i] = None;
                break;
            }
        }
    }
    // ponytail: reduceArrays' second pass and final collapses only apply to shapes with unsealed extras
    // (bleeding edge) or past ConstantArrayTypeBuilder::ARRAY_COUNT_LIMIT, neither of which signatures reach.
    new_arrays.extend(
        to_process
            .into_iter()
            .flatten()
            .map(|a| Type::ConstArray(Box::new(a))),
    );
    new_arrays
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
        if let Type::Intersection(types) = a {
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
    types.sort_by_key(|t| !matches!(t, Type::ConstArray(_)));
    let mut i = 0;
    'outer: while i < types.len() {
        let mut j = i + 1;
        'inner: while j < types.len() {
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
            let a_super = if let Type::Iterable(key, value) = &types[j] {
                iterable_super_of_mixed(key, value, &types[i])
            } else {
                is_super_type_of(&types[j], &types[i])
            };
            if a_super.yes() {
                types.remove(j);
                continue;
            }
            let b_super = if let Type::Iterable(key, value) = &types[i] {
                iterable_super_of_mixed(key, value, &types[j])
            } else {
                is_super_type_of(&types[i], &types[j])
            };
            if b_super.maybe() {
                if let (Type::ClassString(Some(x)), Type::ClassString(Some(y))) =
                    (&types[i], &types[j])
                {
                    types[i] =
                        Type::ClassString(Some(bx(intersect(vec![(**x).clone(), (**y).clone()]))));
                    types.remove(j);
                    continue;
                }
                // ConstantArrayType & list / non-empty-array (constant arrays sort first, so the shape is i)
                if let Type::ConstArray(ca) = &types[i] {
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
                        types[i] = Type::ConstArray(Box::new(merged));
                        types.remove(j);
                        continue 'inner;
                    }
                }
                // ConstantArrayType & ArrayType: key by key (constant arrays sort first, so the shape is i).
                // ponytail: two constant arrays meet only through isSuperTypeOf(), not key by key.
                // As PHPStan's `continue 2`, the merged shape is not compared with the later types again.
                if let (Type::ConstArray(ca), Type::Array(..)) = (&types[i], &types[j]) {
                    types[i] = ca.intersect_array(&types[j]);
                    types.remove(j);
                    i += 1;
                    continue 'outer;
                }
                // `getKeyType()`: the raw key of an array, not its iterable key
                if matches!(
                    types[i],
                    Type::Array(..) | Type::Iterable(..) | Type::ConstArray(_)
                ) && let Type::Array(jk, _) | Type::Iterable(jk, _) = &types[j]
                {
                    let key = intersect(vec![types[i].iterable_key_type(), (**jk).clone()]);
                    let item = intersect(vec![
                        types[i].iterable_value_type(),
                        types[j].iterable_value_type(),
                    ]);
                    types[j] = if matches!(
                        (&types[i], &types[j]),
                        (Type::Iterable(..), Type::Iterable(..))
                    ) {
                        Type::Iterable(bx(key), bx(item))
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
                return Type::Never(NeverKind::Implicit);
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
        self.query(&|t| Tri::from(matches!(t, Type::Null)))
    }

    /// `isVoid()`
    pub fn is_void(&self) -> Tri {
        self.query(&|t| Tri::from(matches!(t, Type::Void)))
    }

    /// `$type instanceof MixedType && !$type->isExplicitMixed()`
    pub fn is_implicit_mixed(&self) -> bool {
        matches!(self, Type::Mixed { explicit: false })
    }

    /// `TypeCombinator::addNull()`: unchanged when it may already be null
    pub fn add_null(self) -> Type {
        if is_super_type_of(&Type::Null, &self).no() {
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
