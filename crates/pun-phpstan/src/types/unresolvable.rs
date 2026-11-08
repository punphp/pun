//! ErrorType/NeverType explanations and the comparisons that produce them, not diagnostic-message inference.

use std::cell::RefCell;

use super::*;

impl Type {
    /// ErrorType and NeverType keep a reason without changing their identity or ordinary operations.
    pub fn with_unresolvable_reason(self, reason: String) -> Type {
        match self {
            Type::Error | Type::Never(_) => Type::Reasoned(Box::new(self), reason),
            Type::Reasoned(t, _) => Type::Reasoned(t, reason),
            t => t,
        }
    }

    pub fn unresolvable_reason(&self) -> Option<&str> {
        match self {
            Type::Reasoned(_, reason) => Some(reason),
            _ => None,
        }
    }

    pub(crate) fn unreasoned(&self) -> &Type {
        match self {
            Type::Reasoned(t, _) => t.unreasoned(),
            t => t,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type as T;
        let (a, b) = (self.unreasoned(), other.unreasoned());
        match (a, b) {
            (T::Mixed { explicit: a }, T::Mixed { explicit: b }) => a == b,
            (
                T::SubtractedMixed {
                    explicit: a,
                    excluded: x,
                },
                T::SubtractedMixed {
                    explicit: b,
                    excluded: y,
                },
            ) => a == b && x == y,
            (T::Never(a), T::Never(b)) => a == b,
            (T::ConstBool(a), T::ConstBool(b)) => a == b,
            (T::ConstInt(a), T::ConstInt(b)) => a == b,
            (T::ConstFloat(a), T::ConstFloat(b)) => a == b || (a.is_nan() && b.is_nan()),
            (T::ConstString(a), T::ConstString(b)) => a == b,
            (T::This(a), T::This(b)) => a == b,
            (T::IntRange(a, x), T::IntRange(b, y)) => a == b && x == y,
            (T::ClassString(a), T::ClassString(b)) => a == b,
            (
                T::SubtractedObject {
                    object: a,
                    excluded: x,
                },
                T::SubtractedObject {
                    object: b,
                    excluded: y,
                },
            ) => a == b && x == y,
            // ObjectType::equals() omits the final reflection override; union containment must still see it.
            (T::Object(a, x, _), T::Object(b, y, _)) | (T::Static(a, x), T::Static(b, y)) => {
                a == b && x == y
            }
            (T::EnumCase(a, x), T::EnumCase(b, y)) => a == b && x == y,
            (T::Array(a, x), T::Array(b, y))
            | (T::Iterable(a, x), T::Iterable(b, y))
            | (T::OffsetAccess(a, x), T::OffsetAccess(b, y)) => a == b && x == y,
            (T::ConstArray(a), T::ConstArray(b)) => a == b,
            (T::Callable(a), T::Callable(b)) => a == b,
            (T::Closure(a), T::Closure(b)) => a == b,
            (T::Union(a), T::Union(b))
            | (T::Benevolent(a), T::Benevolent(b))
            | (T::Intersection(a), T::Intersection(b)) => {
                a.len() == b.len() && {
                    // UnionType/IntersectionType::equals(): match each alternative once, without sorting the
                    // stored members. Their written order still selects the first reflected union method.
                    let mut remaining: Vec<_> = b.iter().collect();
                    a.iter().all(|inner| {
                        let Some(i) = remaining.iter().position(|other| inner == *other) else {
                            return false;
                        };
                        remaining.remove(i);
                        true
                    })
                }
            }
            (T::Accessory(a), T::Accessory(b)) => a == b,
            (T::Conditional(a), T::Conditional(b)) => a == b,
            // TemplateTypeTrait::equals() omits the strategy. StrictMixedType::equals() also accepts its
            // template subclass (the reverse comparison still uses TemplateTypeTrait).
            (T::Template(a), T::Template(b)) if matches!(a.bound, T::StrictMixed) => {
                a.name == b.name
                    && a.scope == b.scope
                    && a.bound == b.bound
                    && a.default == b.default
            }
            (T::Template(a), T::Template(b)) => a == b,
            (T::ObjectShape(a), T::ObjectShape(b)) => a == b,
            (T::Projection(a, x), T::Projection(b, y)) => a == b && x == y,
            (T::LateArrayShape(a), T::LateArrayShape(b)) => a == b,
            (T::StrictMixed, T::Template(t)) => matches!(t.bound, T::StrictMixed),
            (T::StrictMixed, T::StrictMixed)
            | (T::Error, T::Error)
            | (T::NonexistentParent, T::NonexistentParent)
            | (T::Void, T::Void)
            | (T::Null, T::Null)
            | (T::Bool, T::Bool)
            | (T::Int, T::Int)
            | (T::Float, T::Float)
            | (T::String, T::String)
            | (T::StringAcceptingStringable, T::StringAcceptingStringable)
            | (T::StringNotAcceptingStringable, T::StringNotAcceptingStringable)
            | (T::Resource, T::Resource)
            | (T::ObjectWithoutClass, T::ObjectWithoutClass)
            | (T::Wildcard, T::Wildcard) => true,
            _ => false,
        }
    }
}

/// The part of ExtendedPropertyReflection used by ObjectShapeType::isSuperTypeOf(). Static properties are absent.
#[derive(Clone, Debug)]
pub struct PropertyReflection {
    pub name: String,
    pub declaring_class: String,
    pub public: bool,
    pub readable: bool,
    pub ty: Type,
}

type Properties = Vec<(String, Tri, Vec<PropertyReflection>)>;
thread_local! {
    static PROPERTIES: RefCell<Properties> = const { RefCell::new(Vec::new()) };
}

/// Reflection supplied by the argument resolver, scoped to the current call and thread.
pub fn with_property_reflections<R>(
    properties: Vec<(String, Tri, Vec<PropertyReflection>)>,
    f: impl FnOnce() -> R,
) -> R {
    struct Restore(Properties);
    impl Drop for Restore {
        fn drop(&mut self) {
            PROPERTIES.with(|p| {
                p.replace(std::mem::take(&mut self.0));
            });
        }
    }
    let _restore = Restore(PROPERTIES.with(|p| p.replace(properties)));
    f()
}

/// IsSuperTypeOfResult, including reasons. Intersection uses this result at the source of an implicit never.
pub(super) fn compare(a: &Type, b: &Type) -> (Tri, Vec<String>) {
    // LateResolvableTypeTrait delegates the comparison, including its reasons, without erasing the wrapper
    // before TypeCombinator's MixedType fast path. In particular int&T['value'] is not int&mixed~int.
    if matches!(a, Type::OffsetAccess(..)) {
        let resolved = crate::resolve::resolve_late(a);
        if resolved != *a {
            return compare(&resolved, b);
        }
    }
    if matches!(b, Type::OffsetAccess(..)) {
        let resolved = crate::resolve::resolve_late(b);
        if resolved != *b {
            return compare(a, &resolved);
        }
    }
    if let (Type::ObjectShape(shape), Type::Object(class, _, _)) = (a, b)
        && let Some((missing, properties)) = PROPERTIES.with(|p| {
            p.borrow()
                .iter()
                .find(|(n, _, _)| n.eq_ignore_ascii_case(class))
                .map(|(_, missing, p)| (*missing, p.clone()))
        })
    {
        let mut result = Tri::Yes;
        let mut reasons = Vec::new();
        for (name, ty, optional) in shape {
            let Some(property) = properties.iter().find(|p| p.name == *name) else {
                if !optional {
                    result = result.and(missing);
                    reasons.push(format!(
                        "{} {} have property ${name}.",
                        b.describe(Level::TypeOnly),
                        if missing.no() {
                            "does not"
                        } else {
                            "might not"
                        }
                    ));
                }
                continue;
            };
            if !property.public {
                return (
                    Tri::No,
                    vec![format!(
                        "Property {}::${name} is not public.",
                        property.declaring_class
                    )],
                );
            }
            if !property.readable {
                return (
                    Tri::No,
                    vec![format!(
                        "Property {}::${name} is not readable.",
                        property.declaring_class
                    )],
                );
            }
            let (value, why) = compare(ty, &property.ty);
            if value.no() {
                return (value, why);
            }
            result = result.and(value);
        }
        return (result, reasons);
    }
    let result = is_super_type_of(a, b);
    let reasons = match a {
        Type::SubtractedMixed { excluded, .. } if result.no() => vec![format!(
            "Type {} has already been eliminated from {}.",
            excluded.describe(Level::Precise),
            a.describe(Level::TypeOnly)
        )],
        Type::Object(class, args, _) if result.no() && !args.is_empty() => {
            super::reasons::generic(class, args, b)
        }
        _ => vec![],
    };
    (result, reasons)
}
