//! Whether a type's conversion is an `ErrorType`: `toString()`, `toNumber()`, `toInteger()`, `toFloat()` and
//! `toBitwiseNotType()` (the casts and unary operators PHP rejects, e.g. an array to string). `toBoolean()` never is.

use super::{Accessory, StrictMixed, Type, has_method, is_super_type_of, symbols};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cast {
    String,
    Number,
    Integer,
    Float,
    Boolean,
    BitwiseNot,
}

/// `$t->toX() instanceof ErrorType`, with `sm` saying which `mixed` is a StrictMixedType (whose casts all fail).
pub fn cast_is_error(t: &Type, cast: Cast, sm: StrictMixed) -> bool {
    use Cast as C;
    use Type as T;
    if cast == C::Boolean {
        return false;
    }
    if sm.explicit || sm.implicit {
        return cast_is_error(&sm.transform(t), cast, StrictMixed::default());
    }
    let rec = |t: &Type| cast_is_error(t, cast, sm);
    match t {
        T::Reasoned(t, _) => rec(t),
        T::LateArrayShape(s) => rec(&s.resolve()),
        T::StrictMixed => true,
        T::Mixed { .. } | T::SubtractedMixed { .. } | T::Wildcard | T::Error => cast == C::BitwiseNot,
        T::Never(_) | T::OffsetAccess(..) | T::Projection(..) => false,
        T::Void => true,
        T::Null | T::Bool | T::ConstBool(_) => cast == C::BitwiseNot,
        T::Int | T::ConstInt(_) | T::IntRange(..) | T::Float | T::ConstFloat(_) => false,
        T::String | T::StringAcceptingStringable | T::StringNotAcceptingStringable | T::ClassString(_) => {
            cast == C::Number
        }
        T::ConstString(v) => cast == C::Number && !std::str::from_utf8(v).is_ok_and(is_numeric),
        T::Resource => matches!(cast, C::Number | C::BitwiseNot),
        T::ObjectWithoutClass | T::ObjectShape(_) | T::Closure(_) | T::Callable(_) | T::Iterable(..) => true,
        T::Object(..) | T::EnumCase(..) => object(t, cast),
        // StaticType: only its string cast goes through the class
        T::Static(..) | T::This(_) => cast != C::String || object(t, cast),
        T::Array(..) | T::ConstArray(_) | T::Accessory(Accessory::List | Accessory::NonEmptyArray) => {
            matches!(cast, C::String | C::Number | C::BitwiseNot)
        }
        T::Accessory(Accessory::HasMethod(n)) => cast != C::String || !n.eq_ignore_ascii_case("__toString"),
        T::Accessory(a) => {
            cast == C::Number && !matches!(a, Accessory::NumericString | Accessory::DecimalIntString { .. })
        }
        T::Union(ts) => ts.iter().any(rec),
        // BenevolentUnionType::unionTypes() drops the members whose cast is an error
        T::Benevolent(ts) => ts.iter().all(rec),
        T::Intersection(ts) => ts.iter().all(rec),
        T::Template(t) => rec(&t.bound),
        T::Conditional(c) => rec(&c.then) || rec(&c.otherwise),
    }
}

/// ObjectType's casts: numbers only of `SimpleXMLElement` and `GMP` (and an int of a curl handle), a string of a
/// class with `__toString()` (or `BcMath\Number`, `GMP`).
fn object(t: &Type, cast: Cast) -> bool {
    let is = |class: &str| is_super_type_of(&Type::Object(class.into(), vec![]), t).yes();
    let numeric = is("SimpleXMLElement") || is("GMP");
    match cast {
        Cast::String => {
            if is("BcMath\\Number") || is("GMP") {
                return false;
            }
            !t.object_class_names().first().and_then(|n| symbols::lookup(n)).is_some_and(|c| has_method(&c, "__tostring"))
        }
        Cast::Integer => !(numeric || is("CurlHandle") || is("CurlMultiHandle")),
        Cast::Number | Cast::Float => !numeric,
        // (a boolean cast never reaches here)
        Cast::BitwiseNot | Cast::Boolean => true,
    }
}

/// PHP's `is_numeric()` of a string: optional leading whitespace, a sign, digits with an optional fraction and
/// exponent, optional trailing whitespace.
pub fn is_numeric(s: &str) -> bool {
    let t = s.trim_start_matches([' ', '\t', '\n', '\r', '\x0b', '\x0c']);
    let t = t.trim_end_matches([' ', '\t', '\n', '\r', '\x0b', '\x0c']);
    let t = t.strip_prefix(['+', '-']).unwrap_or(t);
    let b = t.as_bytes();
    let mut i = 0;
    let digits = |i: &mut usize| {
        let start = *i;
        while *i < b.len() && b[*i].is_ascii_digit() {
            *i += 1;
        }
        *i - start
    };
    let int = digits(&mut i);
    let mut frac = 0;
    if i < b.len() && b[i] == b'.' {
        i += 1;
        frac = digits(&mut i);
    }
    if int + frac == 0 {
        return false;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        if digits(&mut i) == 0 {
            return false;
        }
    }
    i == b.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_strings() {
        for s in ["1", " 1", "1 ", "-1.5", ".5", "5.", "1e3", "+1E-3"] {
            assert!(is_numeric(s), "{s}");
        }
        for s in ["", "bla", "1e", ".", "1x", "--1", "e3"] {
            assert!(!is_numeric(s), "{s}");
        }
    }
}
