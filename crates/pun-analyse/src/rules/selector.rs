//! `ParametersAcceptorSelector::selectFromArgs()`'s parameters of some built-ins typed from their arguments (the
//! ArrayMapArgVisitor, ArrayFilterArgVisitor, ImplodeArgVisitor, ArrayWalkArgVisitor and ArrayFindArgVisitor cases):
//! the callback's parameters from the array's types, `implode()`'s from the argument count.

use pun_phpstan::symbols::ByRef;
use pun_phpstan::types::{Level, Param, Signature, Type, mixed, union};

use super::params::{Acceptor, Arg, Counts, ParamType, Variants};
use super::{Checker, declared};

/// The variants of the built-in written `name` (lowercase, no leading `\`) for these arguments.
pub(super) fn select_from_args(c: &Checker, name: &str, counts: &Counts, variants: Variants) -> Variants {
    let args = &counts.args;
    let Some(first) = args.first() else {
        return variants;
    };
    let ty = |a: &Arg| c.types.get(&a.value).map(|t| c.to_phpstan(t));
    let param = |name: &str, t: Type| Param { name: name.into(), ty: t, optional: false, by_ref: false, variadic: false };
    let callable = |params: Vec<Param>, ret: Type| Type::Callable(Some(Box::new(Signature { params, ret, templates: vec![] })));
    let overridden = match name {
        // ArrayMapArgVisitor: the arguments other than the callback, when there are any
        "array_map" if args.len() > 1 => {
            let mut params = Vec::new();
            for (i, a) in args.iter().enumerate() {
                if a.name.is_none() && i == 0 || a.name.as_deref() == Some("callback") {
                    continue;
                }
                let Some(t) = ty(a) else {
                    return variants;
                };
                if a.unpack.is_none() {
                    params.push(param("item", t.iterable_value_type()));
                    continue;
                }
                for array in constant_arrays(&t) {
                    params.extend(array.into_iter().map(|v| param("item", v.iterable_value_type())));
                }
            }
            Some((0, union(vec![callable(params, mixed()), Type::Null])))
        }
        "array_filter" => {
            let Some(array) = ty(first) else {
                return variants;
            };
            let params = match args.get(2).and_then(ty) {
                // ARRAY_FILTER_USE_KEY
                Some(Type::ConstInt(2)) => vec![param("key", array.iterable_key_type())],
                // ARRAY_FILTER_USE_BOTH
                Some(Type::ConstInt(1)) => vec![param("item", array.iterable_value_type()), param("key", array.iterable_key_type())],
                _ => vec![param("item", array.iterable_value_type())],
            };
            Some((1, union(vec![callable(params, Type::Bool), Type::Null])))
        }
        "implode" | "join" if args.len() <= 2 => return implode(c, args, variants),
        "array_walk" => {
            let Some(array) = ty(first) else {
                return variants;
            };
            let mut params = vec![param("item", array.iterable_value_type()), param("key", array.iterable_key_type())];
            params.extend(args.get(2).and_then(ty).map(|t| param("arg", t)));
            Some((1, callable(params, mixed())))
        }
        "array_all" | "array_any" | "array_find" | "array_find_key" => {
            let Some(array) = ty(first) else {
                return variants;
            };
            let params = vec![param("value", array.iterable_value_type()), param("key", array.iterable_key_type())];
            Some((1, callable(params, Type::Bool)))
        }
        _ => None,
    };
    let Some((p, t)) = overridden else {
        return variants;
    };
    let mut acceptor = variants.all.into_iter().next().expect("a variant");
    if let (Some(described), Some(param_type)) = (acceptor.types.get_mut(p), acceptor.param_types.get_mut(p)) {
        *described = declared::with_classes(c, || t.describe(Level::TypeOnly));
        *param_type = ParamType::Typed(t);
    }
    Variants { all: vec![acceptor], named: None }
}

/// The values of each constant array `t` is (`getConstantArrays()`).
fn constant_arrays(t: &Type) -> Vec<Vec<Type>> {
    match t {
        Type::ConstArray(a) => vec![a.values.clone()],
        Type::Union(ts) if ts.iter().all(|t| matches!(t, Type::ConstArray(_))) => ts.iter().flat_map(constant_arrays).collect(),
        _ => vec![],
    }
}

/// ImplodeArgVisitor: `(string $separator, array $array)` with two arguments (or the one named `array`), else
/// `(array $separator)`.
fn implode(c: &Checker, args: &[Arg], variants: Variants) -> Variants {
    let acceptor = variants.named.and_then(|n| n.into_iter().next()).unwrap_or_else(|| variants.all.into_iter().next().expect("a variant"));
    let array = Type::array(mixed(), mixed());
    let params: Vec<(String, Type)> = match acceptor.names.as_slice() {
        [first, second, ..] if args.len() > 1 || args[0].name.as_deref() == Some("array") => {
            vec![(first.clone(), Type::String), (second.clone(), array)]
        }
        names => names.first().map(|first| (first.clone(), array)).into_iter().collect(),
    };
    let n = params.len();
    let acceptor = Acceptor {
        optional: vec![false; n],
        names: params.iter().map(|(name, _)| name.clone()).collect(),
        types: params.iter().map(|(_, t)| declared::with_classes(c, || t.describe(Level::TypeOnly))).collect(),
        variadics: vec![false; n],
        param_types: params.into_iter().map(|(_, t)| ParamType::Typed(t)).collect(),
        by_ref: vec![ByRef::No; n],
        ..acceptor
    };
    Variants { all: vec![acceptor], named: None }
}
