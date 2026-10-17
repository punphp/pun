//! `TypeNodeResolver`: a parsed PHPDoc type to a [`Type`], for built-in signatures (no namespace, `use` aliases
//! or type aliases; `self` / `static` / `$this` resolve in the given class). Also reads Pun's extensions of the
//! syntax that `scripts/export-phpstan-symbols.php` writes: `__template<T, 'scope', BOUND[, DEFAULT]>`, `*ERROR*`,
//! `*NEVER*` (PHPStan's own `__benevolent<…>` is handled the way PHPStan does).

use crate::phpdoc::{self, ConstExpr, Node, ShapeKey};
use crate::symbols;
use crate::types::{
    Accessory, Conditional, NeverKind, Param, ShapeBuilder, Signature, Subject, Template, Type,
    intersect, is_super_type_of, mixed, to_benevolent, union,
};

/// `TypeStringResolver::resolve($type, new NameScope(null, [], $class))`; a parse error is an `ErrorType`.
pub fn resolve_str(input: &str, class: Option<&str>) -> Type {
    match phpdoc::parse(input) {
        Ok(node) => resolve(&node, class),
        Err(_) => Type::Error,
    }
}

fn explicit_mixed() -> Type {
    Type::Mixed { explicit: true }
}

fn accessory(base: Type, accessories: &[Accessory]) -> Type {
    let mut types = vec![base];
    types.extend(accessories.iter().cloned().map(Type::Accessory));
    Type::Intersection(types)
}

pub fn resolve(node: &Node, class: Option<&str>) -> Type {
    let r = |n: &Node| resolve(n, class);
    match node {
        Node::Ident(name) => identifier(name, class),
        Node::This => match class.and_then(symbols::class) {
            Some(c) => Type::This(c.name.clone()),
            None => Type::Error,
        },
        Node::Star(s) => match s.as_str() {
            "*" => Type::Wildcard,
            "NEVER" => Type::Never(NeverKind::Explicit),
            _ => Type::Error,
        },
        Node::Nullable(inner) => union(vec![r(inner), Type::Null]),
        Node::Union(members) => {
            let mut iterable = Vec::new();
            let mut others = Vec::new();
            for m in members {
                match m {
                    Node::Array(item) => iterable.push(r(item)),
                    m => others.push(r(m)),
                }
            }
            if !iterable.is_empty() {
                let array_item = union(iterable);
                let mut add_array = true;
                for t in &mut others {
                    if !t.is_iterable().yes()
                        || !is_super_type_of(&t.iterable_value_type(), &array_item).yes()
                    {
                        continue;
                    }
                    *t = match t {
                        Type::Object(_, args) if args.is_empty() => Type::Intersection(vec![
                            t.clone(),
                            Type::Iterable(Box::new(mixed()), Box::new(array_item.clone())),
                        ]),
                        Type::Array(..) | Type::ConstArray(_) => {
                            Type::array(mixed(), array_item.clone())
                        }
                        Type::Iterable(..) => {
                            Type::Iterable(Box::new(mixed()), Box::new(array_item.clone()))
                        }
                        _ => continue,
                    };
                    add_array = false;
                }
                if add_array {
                    others.push(Type::array(mixed(), array_item));
                }
            }
            union(others)
        }
        Node::Inter(members) => {
            let mut types = members.iter().map(r);
            let first = types.next().expect("intersection member");
            types.fold(first, |acc, t| intersect(vec![acc, t]))
        }
        Node::Conditional {
            subject,
            target,
            then,
            otherwise,
            negated,
        } => Type::Conditional(Box::new(Conditional {
            subject: Subject::Type(r(subject)),
            target: r(target),
            then: r(then),
            otherwise: r(otherwise),
            negated: *negated,
        })),
        Node::ConditionalForParameter {
            param,
            target,
            then,
            otherwise,
            negated,
        } => Type::Conditional(Box::new(Conditional {
            subject: Subject::Parameter(param.clone()),
            target: r(target),
            then: r(then),
            otherwise: r(otherwise),
            negated: *negated,
        })),
        Node::Array(item) => Type::array(mixed(), r(item)),
        Node::Generic { base, args } => generic(base, args, class),
        Node::Callable { ident, params, ret } => {
            let main = identifier(ident, class);
            let params: Vec<Param> = params
                .iter()
                .map(|p| Param {
                    name: p.name.trim_start_matches('$').to_string(),
                    ty: r(&p.ty),
                    optional: p.optional || p.variadic,
                    by_ref: p.by_ref,
                    variadic: p.variadic,
                })
                .collect();
            // ponytail: a conditional return on the callable's own parameter is a type assertion in PHPStan
            // (`callable(mixed $x): ($x is int ? true : false)`); kept as a plain conditional return here.
            let sig = Box::new(Signature {
                params,
                ret: r(ret),
            });
            match main {
                Type::Callable(_) => Type::Callable(Some(sig)),
                Type::Object(c, args) if args.is_empty() && c == "Closure" => Type::Closure(sig),
                _ => Type::Error,
            }
        }
        Node::ArrayShape {
            kind,
            items,
            sealed,
            unsealed,
        } => {
            let mut builder = ShapeBuilder::new();
            let mut explicit = Vec::new();
            for item in items {
                let key = item.key.as_ref().map(|k| match k {
                    ShapeKey::Int(i) => Type::ConstInt(int_value(i)),
                    ShapeKey::Str(s) | ShapeKey::Ident(s) => Type::ConstString(s.clone()),
                });
                explicit.extend(key.clone());
                builder.set(key, r(&item.value), item.optional);
            }
            let is_list = kind.ends_with("list");
            let default_key = || {
                if is_list {
                    Type::int_range(Some(0), None)
                } else {
                    mixed()
                }
            };
            if !sealed {
                match unsealed {
                    None => builder.make_unsealed(default_key(), mixed()),
                    Some((key, value)) => {
                        let key = key.as_ref().map_or_else(default_key, |k| array_key(r(k)));
                        let value = r(value);
                        // finite extra keys are optional keys, except those written explicitly
                        let finite = match &key {
                            Type::ConstInt(_) | Type::ConstString(_) => vec![key.clone()],
                            Type::Union(types)
                                if types.iter().all(|t| {
                                    matches!(t, Type::ConstInt(_) | Type::ConstString(_))
                                }) =>
                            {
                                types.clone()
                            }
                            _ => vec![],
                        };
                        for k in finite.iter().filter(|k| !explicit.contains(k)) {
                            builder.set(Some(k.clone()), value.clone(), true);
                        }
                        if finite.is_empty() {
                            builder.make_unsealed(key, value);
                        }
                    }
                }
            }
            let array = builder.get();
            let mut accessories = Vec::new();
            if is_list {
                accessories.push(Type::Accessory(Accessory::List));
            }
            if kind.starts_with("non-empty") {
                accessories.push(Type::Accessory(Accessory::NonEmptyArray));
            }
            if accessories.is_empty() {
                return array;
            }
            accessories.insert(0, array);
            intersect(accessories)
        }
        Node::ObjectShape(items) => Type::ObjectShape(
            items
                .iter()
                .map(|(name, optional, ty)| (name.clone(), r(ty), *optional))
                .collect(),
        ),
        Node::Const(c) => constant(c, class),
    }
}

/// PHP's `(int)` of a phpdoc-parser integer literal: the leading decimal digits (`0x1A` is 0), saturating.
fn int_value(s: &str) -> i64 {
    let s = s.replace('_', "");
    let sign = usize::from(s.starts_with(['-', '+']));
    let digits = s[sign..].bytes().take_while(u8::is_ascii_digit).count();
    let n = &s[..sign + digits];
    n.parse::<i64>().unwrap_or(if n.starts_with('-') {
        i64::MIN
    } else {
        i64::MAX
    })
}

/// `transformUnsafeArrayKey()` without the opt-in string-key check: `intersect(toArrayKey(), int|string)`.
fn array_key(key: Type) -> Type {
    let k = intersect(vec![
        to_array_key(&key),
        Type::Union(vec![Type::Int, Type::String]),
    ]);
    to_array_key(&k)
}

/// `Type::toArrayKey()`
fn to_array_key(t: &Type) -> Type {
    match t {
        Type::Template(_) => t.clone(),
        Type::Mixed { .. } => Type::Benevolent(vec![Type::Int, Type::String]),
        Type::Union(types) => union(types.iter().map(to_array_key).collect()),
        Type::Benevolent(types) => match union(types.iter().map(to_array_key).collect()) {
            Type::Union(types) => Type::Benevolent(types),
            t => t,
        },
        Type::Float => Type::Int,
        Type::Bool => Type::Union(vec![Type::ConstInt(0), Type::ConstInt(1)]),
        Type::Null => Type::ConstString(String::new()),
        Type::ConstBool(b) => Type::ConstInt(*b as i64),
        Type::ConstString(s) if s.parse::<i64>().is_ok_and(|i| i.to_string() == *s) => {
            Type::ConstInt(s.parse().expect("int"))
        }
        _ => t.clone(),
    }
}

fn identifier(name: &str, class: Option<&str>) -> Type {
    use Accessory as A;
    let string = || Type::String;
    match name.to_ascii_lowercase().as_str() {
        "int" | "integer" => return Type::Int,
        "positive-int" => return Type::int_range(Some(1), None),
        "negative-int" => return Type::int_range(None, Some(-1)),
        "non-positive-int" => return Type::int_range(None, Some(0)),
        "non-negative-int" => return Type::int_range(Some(0), None),
        "non-zero-int" => {
            return Type::Union(vec![
                Type::int_range(None, Some(-1)),
                Type::int_range(Some(1), None),
            ]);
        }
        "string" => return Type::String,
        "lowercase-string" => return accessory(string(), &[A::LowercaseString]),
        "uppercase-string" => return accessory(string(), &[A::UppercaseString]),
        "literal-string" => return accessory(string(), &[A::LiteralString]),
        "class-string" | "interface-string" | "trait-string" => return Type::ClassString(None),
        "enum-string" => {
            return Type::ClassString(Some(Box::new(Type::Object("UnitEnum".into(), vec![]))));
        }
        "callable-string" => return Type::Intersection(vec![string(), Type::Callable(None)]),
        "array-key" => return Type::Benevolent(vec![Type::Int, Type::String]),
        "scalar" => return Type::Union(vec![Type::Int, Type::Float, Type::String, Type::Bool]),
        "number" => return Type::Union(vec![Type::Int, Type::Float]),
        "numeric" => {
            return Type::Union(vec![
                Type::Int,
                Type::Float,
                accessory(string(), &[A::NumericString]),
            ]);
        }
        "numeric-string" => return accessory(string(), &[A::NumericString]),
        "non-empty-string" => return accessory(string(), &[A::NonEmptyString]),
        "non-empty-lowercase-string" => {
            return accessory(string(), &[A::NonEmptyString, A::LowercaseString]);
        }
        "non-empty-uppercase-string" => {
            return accessory(string(), &[A::NonEmptyString, A::UppercaseString]);
        }
        "truthy-string" | "non-falsy-string" => return accessory(string(), &[A::NonFalsyString]),
        "non-empty-literal-string" => {
            return accessory(string(), &[A::NonEmptyString, A::LiteralString]);
        }
        "bool" | "boolean" => return Type::Bool,
        "true" => return Type::ConstBool(true),
        "false" => return Type::ConstBool(false),
        "null" => return Type::Null,
        "float" | "double" => return Type::Float,
        "array" | "associative-array" => return Type::array(mixed(), mixed()),
        "non-empty-array" => return accessory(Type::array(mixed(), mixed()), &[A::NonEmptyArray]),
        "iterable" => return Type::Iterable(Box::new(mixed()), Box::new(mixed())),
        "callable" => return Type::Callable(None),
        "resource" | "open-resource" | "closed-resource" => return Type::Resource,
        "mixed" => return explicit_mixed(),
        "void" => return Type::Void,
        "object" => return Type::ObjectWithoutClass,
        "callable-object" => {
            return Type::Intersection(vec![Type::ObjectWithoutClass, Type::Callable(None)]);
        }
        "callable-array" => {
            return Type::Intersection(vec![Type::array(mixed(), mixed()), Type::Callable(None)]);
        }
        "never" | "noreturn" | "never-return" | "never-returns" | "no-return" => {
            return Type::Never(NeverKind::NonAccepting);
        }
        "list" => return Type::list(mixed()),
        "non-empty-list" => {
            return Type::Intersection(vec![
                Type::array(Type::IntRange(Some(0), None), mixed()),
                Type::Accessory(A::NonEmptyArray),
                Type::Accessory(A::List),
            ]);
        }
        "__stringandstringable" => return Type::StringAcceptingStringable,
        "__stringnotstringable" => return Type::StringNotAcceptingStringable,
        _ => {}
    }
    if let Some(class) = class {
        match name.to_ascii_lowercase().as_str() {
            "self" => return Type::Object(class.to_string(), vec![]),
            "static" => {
                return match symbols::class(class) {
                    Some(c) => Type::Static(c.name.clone()),
                    None => Type::Error,
                };
            }
            _ => {}
        }
    }
    let name = name.trim_start_matches('\\');
    if name.contains('-') && !name.starts_with("OCI-") {
        return Type::Error;
    }
    if might_be_constant(name)
        && symbols::class(name).is_none()
        && let Some(ty) = symbols::symbols(symbols::PhpVersion::Php84)
            .constants
            .get(name)
    {
        return match ty.as_str() {
            "INF" => Type::ConstFloat(f64::INFINITY),
            "-INF" => Type::ConstFloat(f64::NEG_INFINITY),
            "NAN" => Type::ConstFloat(f64::NAN),
            // ConstantResolver builds some unions by hand (`new UnionType([...])`, e.g. PHP_SAPI's
            // 'cli'|…|non-falsy-string), so a constant's top-level union is not normalized again.
            ty => match phpdoc::parse(ty) {
                Ok(Node::Union(members)) => {
                    Type::Union(members.iter().map(|m| resolve(m, None)).collect())
                }
                _ => resolve_str(ty, None),
            },
        };
    }
    Type::Object(name.to_string(), vec![])
}

/// `(?:^|\\)[A-Z_][A-Z0-9_]*$`
fn might_be_constant(name: &str) -> bool {
    let last = name.rsplit('\\').next().unwrap_or(name);
    let mut bytes = last.bytes();
    bytes
        .next()
        .is_some_and(|b| b.is_ascii_uppercase() || b == b'_')
        && bytes.all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
}

fn generic(base: &str, args: &[Node], class: Option<&str>) -> Type {
    let lower = base.to_ascii_lowercase();
    // Pun's template serialization (`__template<T, 'scope', bound, default>`)
    if lower == "__template"
        && let [
            Node::Ident(name),
            Node::Const(ConstExpr::Str(scope)),
            bound,
            rest @ ..,
        ] = args
    {
        return Type::Template(Box::new(Template {
            name: name.clone(),
            scope: scope.clone(),
            bound: resolve(bound, class),
            default: rest.first().map(|d| resolve(d, class)),
            argument: false,
        }));
    }
    let types: Vec<Type> = args.iter().map(|a| resolve(a, class)).collect();
    match lower.as_str() {
        "array" | "non-empty-array" => {
            let array = match types.as_slice() {
                [item] => Type::array(mixed(), item.clone()),
                [key, item] => {
                    let key = array_key(key.clone());
                    match key {
                        Type::ConstInt(_) | Type::ConstString(_) => {
                            let mut builder = ShapeBuilder::new();
                            builder.set(Some(key), item.clone(), true);
                            builder.get()
                        }
                        key => Type::array(key, item.clone()),
                    }
                }
                _ => return Type::Error,
            };
            if lower == "non-empty-array" {
                return intersect(vec![array, Type::Accessory(Accessory::NonEmptyArray)]);
            }
            array
        }
        "list" | "non-empty-list" => {
            let [item] = types.as_slice() else {
                return Type::Error;
            };
            let list = Type::list(item.clone());
            if lower == "non-empty-list" {
                return intersect(vec![list, Type::Accessory(Accessory::NonEmptyArray)]);
            }
            list
        }
        "iterable" => match types.as_slice() {
            [item] => Type::Iterable(Box::new(explicit_mixed()), Box::new(item.clone())),
            [key, item] => Type::Iterable(Box::new(key.clone()), Box::new(item.clone())),
            _ => Type::Error,
        },
        "class-string" | "interface-string" => match types.as_slice() {
            [g] if g.is_object_yes() || matches!(g, Type::Mixed { .. }) => {
                Type::ClassString(Some(Box::new(g.clone())))
            }
            _ => Type::Error,
        },
        "int" => {
            let bound = |t: &Type, node: &Node, keyword: &str| match t {
                Type::ConstInt(i) => Ok(Some(*i)),
                _ if *node == Node::Ident(keyword.to_string()) => Ok(None),
                _ => Err(()),
            };
            match (types.as_slice(), args) {
                ([min, max], [min_node, max_node]) => {
                    match (bound(min, min_node, "min"), bound(max, max_node, "max")) {
                        (Ok(min), Ok(max)) => Type::int_range(min, max),
                        _ => Type::Error,
                    }
                }
                _ => Type::Error,
            }
        }
        "int-mask" => int_mask(union(types)).unwrap_or(Type::Error),
        "int-mask-of" => match types.as_slice() {
            [t] => int_mask(t.clone()).unwrap_or(Type::Error),
            _ => Type::Error,
        },
        "__benevolent" => match types.as_slice() {
            [t] => to_benevolent(t.clone()),
            _ => Type::Error,
        },
        _ => generic_object(base, types, class),
    }
}

impl Type {
    fn is_object_yes(&self) -> bool {
        match self {
            Type::Object(..)
            | Type::ObjectWithoutClass
            | Type::Static(_)
            | Type::This(_)
            | Type::ObjectShape(_)
            | Type::Closure(_) => true,
            Type::Template(t) => t.bound.is_object_yes(),
            Type::Union(types) | Type::Intersection(types) => {
                if matches!(self, Type::Union(_)) {
                    types.iter().all(Type::is_object_yes)
                } else {
                    types.iter().any(Type::is_object_yes)
                }
            }
            _ => false,
        }
    }
}

fn generic_object(base: &str, mut types: Vec<Type>, class: Option<&str>) -> Type {
    let main = identifier(base, class);
    let class_name = match &main {
        Type::Object(name, _) => name.clone(),
        _ => return Type::Error,
    };
    let Some(reflection) = symbols::class(&class_name) else {
        return Type::Object(class_name, types);
    };
    let iterable = main.is_iterable().yes();
    // ponytail: template defaults are not filled in for missing arguments; no built-in class can miss one (only
    // FFI\CData has a default, on its only template). Add for user-defined classes.
    if !reflection.templates.is_empty() {
        if ["Traversable", "IteratorAggregate", "Iterator"].contains(&class_name.as_str())
            && types.len() == 1
        {
            return Type::Object(
                class_name,
                vec![explicit_mixed(), types.pop().expect("one type")],
            );
        }
        if class_name == "Generator" && (types.len() == 1 || types.len() == 2) {
            if types.len() == 1 {
                types.insert(0, explicit_mixed());
            }
            types.extend([explicit_mixed(), explicit_mixed()]);
            return Type::Object(class_name, types);
        }
        if !iterable || types.len() != 1 || reflection.templates.len() == 1 {
            return Type::Object(class_name, types);
        }
    }
    if iterable {
        match types.len() {
            1 => {
                return intersect(vec![
                    main,
                    Type::Iterable(
                        Box::new(explicit_mixed()),
                        Box::new(types.pop().expect("one")),
                    ),
                ]);
            }
            2 => {
                let item = types.pop().expect("two");
                let key = types.pop().expect("two");
                return intersect(vec![main, Type::Iterable(Box::new(key), Box::new(item))]);
            }
            _ => {}
        }
    }
    Type::Object(class_name, types)
}

/// `expandIntMaskToType()`
fn int_mask(t: Type) -> Option<Type> {
    // `TypeUtils::getConstantIntegers()`: every member a constant integer
    let members = match &t {
        Type::Union(types) => types.as_slice(),
        t => std::slice::from_ref(t),
    };
    let ints: Vec<i64> = members
        .iter()
        .map(|t| match t {
            Type::ConstInt(i) => Some(*i),
            _ => None,
        })
        .collect::<Option<_>>()?;
    let mut values: Vec<i64> = Vec::new();
    for int in ints {
        if int != 0 && !values.contains(&int) {
            for v in values.clone() {
                let computed = v | int;
                if !values.contains(&computed) {
                    values.push(computed);
                }
            }
        }
        if !values.contains(&int) {
            values.push(int);
        }
    }
    if !values.contains(&0) {
        values.push(0);
    }
    let min = *values.iter().min().expect("values");
    let max = *values.iter().max().expect("values");
    if max - min == values.len() as i64 - 1 || values.len() > 256 {
        return Some(Type::int_range(Some(min), Some(max)));
    }
    Some(union(values.into_iter().map(Type::ConstInt).collect()))
}

fn constant(c: &ConstExpr, class: Option<&str>) -> Type {
    match c {
        ConstExpr::Int(i) => Type::ConstInt(int_value(i)),
        ConstExpr::Float(f) => Type::ConstFloat(f.replace('_', "").parse().unwrap_or(0.0)),
        ConstExpr::Str(s) => Type::ConstString(s.clone()),
        ConstExpr::Fetch(class_name, name) => {
            let is_static = class.is_some() && class_name.eq_ignore_ascii_case("static");
            let class_name = match (class, class_name.to_ascii_lowercase().as_str()) {
                (Some(c), "self" | "static") => c,
                _ => class_name.trim_start_matches('\\'),
            };
            let Some(reflection) = symbols::class(class_name) else {
                return Type::Error;
            };
            if name.eq_ignore_ascii_case("class") {
                if is_static && !reflection.is_final {
                    return Type::ClassString(Some(Box::new(Type::Static(
                        reflection.name.clone(),
                    ))));
                }
                return Type::ConstString(reflection.name.clone());
            }
            let matches = |constant: &str| match name.split_once('*') {
                None => constant == name,
                Some(_) => wildcard(name, constant),
            };
            let cases = reflection
                .cases
                .iter()
                .filter(|c| matches(c))
                .map(|c| Type::EnumCase(reflection.name.clone(), c.clone()));
            // declared, then inherited (as PHP's reflection lists them)
            let mut seen = Vec::new();
            let constants = std::iter::once(reflection)
                .chain(
                    reflection
                        .ancestors
                        .iter()
                        .filter_map(|a| symbols::class(a)),
                )
                .flat_map(|c| c.constants.iter().map(move |(n, t)| (c, n, t)))
                .filter(|(_, n, _)| {
                    let new = !seen.contains(n);
                    seen.push(*n);
                    new && matches(n)
                })
                .map(|(c, _, t)| resolve_str(t, Some(&c.name)));
            let types: Vec<Type> = cases.chain(constants).collect();
            if types.is_empty() {
                return Type::Error;
            }
            union(types)
        }
    }
}

/// `{^PREFIX.*?REST$}` with `*` the only metacharacter.
fn wildcard(pattern: &str, s: &str) -> bool {
    let mut parts = pattern.split('*');
    let Some(mut rest) = s.strip_prefix(parts.next().expect("a part")) else {
        return false;
    };
    let last = parts.next_back().expect("a `*`");
    for part in parts {
        let Some(pos) = rest.find(part) else {
            return false;
        };
        rest = &rest[pos + part.len()..];
    }
    rest.ends_with(last)
}
