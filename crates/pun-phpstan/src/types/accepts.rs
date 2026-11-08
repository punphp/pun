//! `Type::accepts()`: whether a value of one type may be passed where another is declared (parameters, returns,
//! property assignments). It differs from `isSuperTypeOf()` where PHP coerces: `float` accepts `int`, a non-strict
//! `string` accepts a `Stringable` object, an implicit `mixed` is accepted by anything.

use super::{
    Accessory, NeverKind, Tri, Type, canonical_class, has_method, is_super_type_of,
    signature_super, static_of, symbols,
};

/// How `mixed` is accepted: RuleLevelHelper's `StrictMixedType` (levels 9 and 10) is accepted by `mixed` alone.
#[derive(Clone, Copy, Default, Debug)]
pub struct StrictMixed {
    pub explicit: bool,
    pub implicit: bool,
}

impl StrictMixed {
    /// RuleLevelHelper::transformCommonType(): unlike findTypeToCheck this traverses containers, but a
    /// mixed template stops traversal before its default or nested bound can be rewritten.
    pub fn transform(self, t: &Type) -> Type {
        if !self.explicit && !self.implicit {
            return t.clone();
        }
        if let Type::Object(class, args, exact) = t
            && !args.is_empty()
        {
            return self.transform_generic_object_arguments(class, args, *exact, t);
        }
        self.transform_mixed(t)
            .unwrap_or_else(|| t.traverse(&|t| self.transform(t)))
    }

    /// `transformMixedType()`: the checked copy of a mixed this level maps, else `None`.
    fn transform_mixed(self, t: &Type) -> Option<Type> {
        t.mixed_explicitness()
            .is_some_and(|explicit| {
                if explicit {
                    self.explicit
                } else {
                    self.implicit
                }
            })
            .then(|| t.to_strict_mixed())
    }

    /// `transformGenericObjectTypeArguments()` (PHPStan 2.3.2): an invariant argument of an invariant template is
    /// compared with equals(), so only a top-level mixed in it is mapped, not one nested deeper.
    fn transform_generic_object_arguments(
        self,
        class: &str,
        args: &[Type],
        exact: bool,
        t: &Type,
    ) -> Type {
        let Some(reflection) = symbols::lookup(class) else {
            return t.traverse(&|t| self.transform(t));
        };
        let args = args
            .iter()
            .enumerate()
            .map(|(i, argument)| {
                let invariant = !matches!(argument, Type::Projection(..))
                    && reflection
                        .templates
                        .get(i)
                        .is_some_and(|template| template.variance == "invariant");
                if invariant {
                    self.transform_mixed(argument)
                        .unwrap_or_else(|| argument.clone())
                } else {
                    self.transform(argument)
                }
            })
            .collect();
        Type::Object(class.to_string(), args, exact)
    }
}

/// `$a->accepts($b, $strictTypes)->result`.
pub fn accepts(a: &Type, b: &Type, strict_types: bool) -> Tri {
    accepts_with(a, b, strict_types, StrictMixed::default())
}

pub fn accepts_with(a: &Type, b: &Type, strict: bool, sm: StrictMixed) -> Tri {
    use Type as T;
    if sm.explicit || sm.implicit {
        return accepts_with(a, &sm.transform(b), strict, StrictMixed::default());
    }
    if let T::Reasoned(t, _) = b {
        return accepts_with(a, t, strict, sm);
    }
    if let T::SubtractedObject { object, .. } = b {
        return accepts_with(a, object, strict, sm);
    }
    if let T::LateArrayShape(s) = b {
        return accepts_with(a, &s.resolve(), strict, sm);
    }
    let rec = |x: &Type, y: &Type| accepts_with(x, y, strict, sm);
    // a CompoundType `$b` answers through its `isAcceptedBy()`, a StrictMixedType too
    let compound = b.is_compound();
    let by = || accepted_by(a, b, strict, sm);
    // `$type instanceof StringType` (and IntegerType for an int)
    let string = matches!(
        b,
        T::String
            | T::ConstString(_)
            | T::ClassString(_)
            | T::StringAcceptingStringable
            | T::StringNotAcceptingStringable
    );
    let integer = matches!(b, T::Int | T::ConstInt(_) | T::IntRange(..));
    // `$this->isXxx()->yes()` first, then the compound, then the answer itself (the accessory types)
    let accessory = |r: Tri| {
        if r.yes() {
            Tri::Yes
        } else if compound {
            by()
        } else {
            r
        }
    };
    match a {
        T::Reasoned(t, _) => rec(t, b),
        T::SubtractedObject { object, .. } => {
            if compound {
                by()
            } else {
                rec(object, b)
            }
        }
        T::LateArrayShape(s) => rec(&s.resolve(), b),
        // MixedType (ErrorType extends it) accepts anything
        T::Mixed { .. }
        | T::StrictMixed
        | T::SubtractedMixed { .. }
        | T::Error
        | T::Wildcard
        | T::OffsetAccess(..) => Tri::Yes,
        T::Never(NeverKind::NonAccepting) => Tri::from(matches!(b, T::Never(_))),
        T::Never(_) => Tri::Yes,
        T::Union(types) | T::Benevolent(types) => union(a, types, b, strict, sm),
        T::Intersection(types) => types.iter().map(|t| rec(t, b)).fold(Tri::Yes, Tri::and),
        T::Conditional(c) => rec(&c.resolve(), b),
        // TemplateTypeParameterStrategy; TemplateTypeArgumentStrategy adds a maybe
        T::Template(t) => {
            if compound {
                by()
            } else if t.argument {
                rec(&t.bound, b).and(Tri::Maybe)
            } else {
                rec(&t.bound, b)
            }
        }
        _ if compound
            && !matches!(
                a,
                T::Float
                    | T::ConstFloat(_)
                    | T::ConstInt(_)
                    | T::ConstBool(_)
                    | T::ConstString(_)
                    | T::Int
                    | T::IntRange(..)
                    | T::Bool
                    | T::Null
                    | T::Resource
                    | T::String
                    | T::StringAcceptingStringable
                    | T::StringNotAcceptingStringable
                    | T::Iterable(..)
                    | T::Accessory(_)
                    | T::Callable(_)
                    | T::ConstArray(_)
                    | T::EnumCase(..)
            ) =>
        {
            by()
        }
        T::Float => {
            if matches!(b, T::Float | T::ConstFloat(_)) || b.is_integer().yes() {
                Tri::Yes
            } else if compound {
                by()
            } else {
                Tri::No
            }
        }
        // ConstantScalarTypeTrait: the same constant, else the parent's answer and maybe
        T::ConstFloat(_) => match b {
            T::ConstFloat(_) => Tri::from(a == b),
            _ if compound => by(),
            _ => Tri::from(matches!(b, T::Float) || b.is_integer().yes()).and(Tri::Maybe),
        },
        T::ConstString(x) => match b {
            T::ConstString(y) => Tri::from(x == y),
            _ if compound => by(),
            _ if string => Tri::Maybe,
            _ => string_accepts(&T::String, b, strict).and(Tri::Maybe),
        },
        // JustNullableTypeTrait: an instance of the constant's own class (`static`) only
        T::ConstInt(x) => match b {
            T::ConstInt(y) => Tri::from(x == y),
            _ if compound => by(),
            _ => Tri::No,
        },
        T::ConstBool(x) => match b {
            T::ConstBool(y) => Tri::from(x == y),
            _ if compound => by(),
            _ => Tri::No,
        },
        T::String | T::StringAcceptingStringable | T::StringNotAcceptingStringable => {
            if string {
                Tri::Yes
            } else if compound {
                by()
            } else {
                string_accepts(a, b, strict)
            }
        }
        T::Int => {
            if integer {
                Tri::Yes
            } else if compound {
                by()
            } else {
                Tri::No
            }
        }
        T::IntRange(..) => {
            if integer {
                is_super_type_of(a, b)
            } else if compound {
                by()
            } else {
                Tri::No
            }
        }
        T::Bool | T::Null | T::Resource => {
            let same = match a {
                T::Bool => matches!(b, T::Bool | T::ConstBool(_)),
                _ => std::mem::discriminant(a) == std::mem::discriminant(b),
            };
            if same {
                Tri::Yes
            } else if compound {
                by()
            } else {
                Tri::No
            }
        }
        T::Void => b.is_void().or(b.is_null()),
        T::ClassString(None) => b.is_class_string(),
        T::ClassString(Some(object)) => match b {
            T::ConstString(name) => {
                if b.is_class_string().yes() {
                    rec(
                        object,
                        &T::Object(String::from_utf8_lossy(name).into_owned(), vec![], false),
                    )
                } else {
                    Tri::No
                }
            }
            T::ClassString(Some(theirs)) => rec(object, theirs),
            T::ClassString(None) => rec(object, &T::ObjectWithoutClass),
            _ if string => Tri::Maybe,
            _ => Tri::No,
        },
        T::Array(key, item) => match b {
            T::ConstArray(c) => {
                let mut result = Tri::Yes;
                for (k, v) in c.keys.iter().zip(&c.values) {
                    result = result.and(rec(key, k)).and(rec(item, v));
                }
                result
            }
            T::Array(k, v) => rec(item, v).and(rec(key, k)),
            _ => Tri::No,
        },
        T::ConstArray(_) if compound && !matches!(b, T::Intersection(_)) => by(),
        T::ConstArray(c) if c.keys.is_empty() && c.unsealed.is_none() => match b {
            T::ConstArray(theirs) => Tri::from(theirs.keys.is_empty()),
            _ => b.is_array(),
        },
        // checkOurKeys() of a general array; ponytail: a constant array or an unsealed shape is isSuperTypeOf() here
        T::ConstArray(c) if c.unsealed.is_none() && !matches!(b, T::ConstArray(_)) => {
            our_keys(c, b, strict, sm)
        }
        T::Iterable(key, item) => {
            if b.is_constant_array().yes() && b.is_iterable_at_least_once().no() {
                return Tri::Yes;
            }
            if b.is_iterable().yes() {
                return rec(item, &b.iterable_value_type()).and(rec(key, &b.iterable_key_type()));
            }
            if compound { by() } else { Tri::No }
        }
        T::Object(class, args, exact) if args.is_empty() => match b {
            T::Static(bc, _) | T::This(bc) => subclass(class, bc, *exact),
            T::Closure(_) => is_super_type_of(a, &T::Object("Closure".into(), vec![], false)),
            T::ObjectWithoutClass => Tri::Maybe,
            _ => match b.object_class_names().first() {
                Some(that) => subclass(class, that, *exact),
                None => Tri::No,
            },
        },
        // GenericObjectType in an accepts context: a raw reference to the class is its plain object's answer
        T::Object(class, _, exact) if matches!(b, T::Object(bc, bargs, _) if bargs.is_empty() && canonical_class(bc) == canonical_class(class)) => {
            is_super_type_of(&T::Object(class.clone(), vec![], *exact), b)
        }
        T::Object(class, args, exact) => super::object_super(class, args, b, false, *exact),
        T::ObjectWithoutClass => Tri::from(
            matches!(b, T::ObjectWithoutClass | T::ObjectShape(_))
                || !b.object_class_names().is_empty(),
        ),
        T::ObjectShape(props) => object_shape(a, props, b, strict),
        T::Closure(ours) => match b {
            T::Closure(_) => signature_super(ours, b, Some(strict)),
            _ => rec(&T::Object("Closure".into(), vec![], false), b),
        },
        // CallableType::isSuperTypeOfInternal() with `$treatMixedAsAny`; a compound other than a callable answers itself
        T::Callable(sig) => {
            if compound && !matches!(b, T::Callable(_)) {
                return by();
            }
            let callable = b.is_callable();
            match sig {
                Some(ours) if !callable.no() => {
                    callable.and(signature_super(ours, b, Some(strict)))
                }
                _ => callable,
            }
        }
        // StaticType::accepts(): only a `static` (for `$this`, a `$this`), by the classes
        T::Static(class, args) => match b {
            T::Static(bc, bargs) => rec(&static_of(class, args), &static_of(bc, bargs)),
            T::This(bc) => rec(&static_of(class, args), &static_of(bc, &[])),
            _ => Tri::No,
        },
        T::This(class) => match b {
            T::This(bc) => rec(&static_of(class, &[]), &static_of(bc, &[])),
            _ => Tri::No,
        },
        T::Accessory(Accessory::NonEmptyString) => accessory(b.is_non_empty_string()),
        T::Accessory(Accessory::NonFalsyString) => accessory(b.is_non_falsy_string()),
        T::Accessory(Accessory::NumericString) => accessory(b.is_numeric_string()),
        T::Accessory(Accessory::LiteralString) => {
            // `$type instanceof MixedType`, a template of mixed (TemplateMixedType) too
            if b.mixed_explicitness().is_some() {
                return Tri::No;
            }
            accessory(b.is_literal_string())
        }
        T::Accessory(Accessory::LowercaseString) => accessory(b.is_lowercase_string()),
        T::Accessory(Accessory::UppercaseString) => accessory(b.is_uppercase_string()),
        T::Accessory(Accessory::DecimalIntString { inverse }) => {
            let decimal = b.is_decimal_int_string();
            let decimal = if *inverse { decimal.negate() } else { decimal };
            if b.is_string().yes() && decimal.yes() {
                Tri::Yes
            } else if compound {
                by()
            } else {
                b.is_string().and(decimal)
            }
        }
        T::Accessory(Accessory::List) => accessory(b.is_list()),
        T::Accessory(Accessory::NonEmptyArray) => {
            accessory(b.is_array().and(b.is_iterable_at_least_once()))
        }
        T::Accessory(Accessory::HasMethod(_)) => {
            if compound {
                by()
            } else {
                Tri::No
            }
        }
        T::Accessory(Accessory::ModelProperty(t)) => {
            super::larastan::model_property_accepts(t, b, strict, by)
        }
        T::Accessory(Accessory::ViewString) => super::larastan::view_string_accepts(b, by),
        // GenericObjectType, EnumCaseObjectType, a ConstantArrayType (ponytail: its key-by-key checkOurKeys() and
        // unsealed shapes are isSuperTypeOf() here)
        _ => is_super_type_of(a, b),
    }
}

/// ConstantArrayType::checkOurKeys() against a type that is not a constant array (ArrayType::hasOffsetValueType():
/// maybe unless the key type rules the key out), and the type an array.
fn our_keys(c: &super::ConstArray, b: &Type, strict: bool, sm: StrictMixed) -> Tri {
    let (key, value) = (b.iterable_key_type(), b.iterable_value_type());
    let mut result = Tri::Yes;
    for (i, (k, v)) in c.keys.iter().zip(&c.values).enumerate() {
        let optional = c.optional.contains(&i);
        if is_super_type_of(&key, k).no() || b.is_array().no() {
            if optional {
                continue;
            }
            return Tri::No;
        }
        if !optional {
            result = result.and(Tri::Maybe);
        }
        let accepts = accepts_with(v, &value, strict, sm);
        if accepts.no() {
            return Tri::No;
        }
        result = result.and(accepts);
    }
    result.and(b.is_array())
}

/// StringType's non-string: a non-strict `string` accepts an object with `__toString()`.
fn string_accepts(a: &Type, b: &Type, strict: bool) -> Tri {
    if matches!(a, Type::StringNotAcceptingStringable) {
        return Tri::No;
    }
    let names = b.object_class_names();
    let [name] = names.as_slice() else {
        return Tri::No;
    };
    if strict && !matches!(a, Type::StringAcceptingStringable) {
        return Tri::No;
    }
    Tri::from(symbols::lookup(name).is_some_and(|c| has_method(&c, "__tostring")))
}

/// `ObjectType::checkSubclassAcceptability()`: an unknown class accepts only itself.
fn subclass(class: &str, that: &str, exact: bool) -> Tri {
    if that == class {
        return Tri::Yes;
    }
    match (symbols::lookup(class), symbols::lookup(that)) {
        (Some(this), Some(other)) => Tri::from(
            this.name == other.name
                || (!exact && !this.is_final && other.ancestors.contains(&this.name)),
        ),
        _ => Tri::No,
    }
}

/// ObjectShapeType::accepts(): a universal object crate (`stdClass`) maybe, else each required property: present yes,
/// absent from a final class no, else maybe.
// ponytail: the property types are not compared (no property types in the symbols)
fn object_shape(a: &Type, props: &[(String, Type, bool)], b: &Type, strict: bool) -> Tri {
    let names = b.object_class_names();
    let Some(class) = names.first().and_then(|n| symbols::lookup(n)) else {
        return is_super_type_of(a, b);
    };
    if class.name == "stdClass" {
        return Tri::Maybe;
    }
    // SimpleXMLElementClassPropertyReflectionExtension: every property, `__benevolent<static|null>`
    if class.name == "SimpleXMLElement" || class.ancestors.iter().any(|c| c == "SimpleXMLElement") {
        let property = Type::Benevolent(vec![
            Type::Object(class.name.clone(), vec![], false),
            Type::Null,
        ]);
        return props
            .iter()
            .map(|(_, ty, _)| accepts(ty, &property, strict))
            .fold(Tri::Yes, Tri::and);
    }
    props
        .iter()
        .filter(|(name, _, optional)| !optional && !class.properties.contains(name))
        .map(|_| if class.is_final { Tri::No } else { Tri::Maybe })
        .fold(Tri::Yes, Tri::and)
}

/// UnionType::accepts().
fn union(a: &Type, types: &[Type], b: &Type, strict: bool, sm: StrictMixed) -> Tri {
    use Type as T;
    let rec = |x: &Type, y: &Type| accepts_with(x, y, strict, sm);
    if let T::Iterable(k, v) = b {
        // IterableType::toArrayOrTraversable()
        let alt = T::Union(vec![
            T::Array(k.clone(), v.clone()),
            T::Object(
                "Traversable".into(),
                vec![(**k).clone(), (**v).clone()],
                false,
            ),
        ]);
        return rec(a, &alt);
    }
    // EQUAL_UNION_CLASSES
    for (base, classes) in [
        ("DateTimeInterface", ["DateTimeImmutable", "DateTime"]),
        ("Throwable", ["Error", "Exception"]),
    ] {
        if matches!(b, T::Object(n, args, _) if args.is_empty() && n == base) {
            let all = T::Union(
                classes
                    .iter()
                    .map(|c| T::Object((*c).into(), vec![], false))
                    .collect(),
            );
            if rec(a, &all).yes() {
                return Tri::Yes;
            }
            break;
        }
    }
    let result = types.iter().map(|t| rec(t, b)).fold(Tri::No, Tri::or);
    if result.yes() {
        return result;
    }
    if b.is_compound() && !matches!(b, T::Callable(_) | T::Template(_) | T::Intersection(_)) {
        return accepted_by(a, b, strict, sm);
    }
    if matches!(b, T::Template(t) if matches!(t.bound, T::Union(_) | T::Benevolent(_))) {
        return result.or(accepted_by(a, b, strict, sm));
    }
    result
}

/// `CompoundType::isAcceptedBy()` of `b`.
fn accepted_by(a: &Type, b: &Type, strict: bool, sm: StrictMixed) -> Tri {
    use Type as T;
    let rec = |x: &Type, y: &Type| accepts_with(x, y, strict, sm);
    match b {
        T::StrictMixed => Tri::from(
            matches!(
                a,
                T::StrictMixed | T::Mixed { .. } | T::SubtractedMixed { .. } | T::Error
            ) || matches!(a, T::Template(t) if matches!(t.bound, T::StrictMixed)),
        )
        .or(Tri::Maybe),
        T::Union(types) => Tri::extreme_identity(types.iter().map(|t| rec(a, t))),
        T::Benevolent(types) => types.iter().map(|t| rec(a, t)).fold(Tri::No, Tri::or),
        T::Mixed { .. } | T::Error => Tri::Yes,
        T::SubtractedMixed { .. } => Tri::from(!is_super_type_of(b, a).no()),
        T::Intersection(types) => {
            let result = Tri::max_min(types.iter().map(|t| rec(a, t)));
            if result.yes() {
                let super_type = is_super_type_of(a, b);
                if super_type.no() {
                    return Tri::No;
                }
                // a template member's eager yes is not trusted over the holistic maybe
                if super_type.maybe()
                    && types
                        .iter()
                        .any(|t| matches!(t, T::Template(_)) && rec(a, t).yes())
                {
                    return Tri::Maybe;
                }
            }
            result
        }
        // IntegerRangeType::isSubTypeOf(): an integer, a union (a template of a union too) or an intersection asks
        // itself, anything else no
        T::IntRange(..) => match a {
            T::Template(at) if matches!(at.bound, T::Union(_) | T::Benevolent(_)) => {
                is_super_type_of(&at.bound, b)
            }
            T::Int
            | T::ConstInt(_)
            | T::IntRange(..)
            | T::Union(_)
            | T::Benevolent(_)
            | T::Intersection(_) => is_super_type_of(a, b),
            _ => Tri::No,
        },
        // TemplateMixedType::isAcceptedBy(), as MixedType's
        T::Template(t) if matches!(t.bound, T::Mixed { .. }) => {
            Tri::from(!is_super_type_of(b, a).no())
        }
        // TemplateStrictMixedType::isAcceptedBy() uses the template's subtype relation, not the bound's accepts().
        T::Template(t) if matches!(t.bound, T::StrictMixed) => is_super_type_of(a, b),
        // TemplateTypeTrait::isAcceptedBy(): a union or intersection reaches here only with a bound of its kind (their
        // accepts() ask the members first), so it compares with the bound like any other type
        T::Template(t) => match a {
            T::Template(at) => {
                let r = rec(&at.bound, &t.bound);
                if at.name == t.name && at.scope == t.scope {
                    r
                } else {
                    r.and(Tri::Maybe)
                }
            }
            _ => rec(a, &t.bound),
        },
        // LateResolvableTypeTrait
        T::Conditional(c) => {
            let resolved = c.resolve();
            if resolved.is_compound() {
                accepted_by(a, &resolved, strict, sm)
            } else {
                rec(a, &resolved)
            }
        }
        // MixedType's is above; NeverType, IntegerRangeType, CallableType, IterableType and the accessories answer
        // `isSubTypeOf()`
        _ => is_super_type_of(a, b),
    }
}
