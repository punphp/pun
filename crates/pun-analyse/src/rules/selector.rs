//! `ParametersAcceptorSelector::selectFromArgs()`'s parameters of some built-ins typed from their arguments (the
//! ArrayMapArgVisitor, ArrayFilterArgVisitor, ImplodeArgVisitor, ArrayWalkArgVisitor, ArrayFindArgVisitor and
//! CurlSetOpt(Array)ArgVisitor cases): the callback's parameters from the array's types, `implode()`'s from the argument
//! count, `curl_setopt()`'s value from the option.

use pun_phpstan::symbols::ByRef;
use pun_phpstan::types::{Level, Param, ShapeBuilder, Signature, Type, mixed, union};

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
        // CurlSetOptArgVisitor: the value's type from the option's
        "curl_setopt" if args.len() >= 3 => {
            let option = ty(&args[1]).unwrap_or_else(mixed);
            let types: Option<Vec<Type>> = constant_scalars(&option).iter().map(|o| curl_option(c, o)).collect();
            types.filter(|ts| !ts.is_empty()).map(|ts| (2, union(ts)))
        }
        // CurlSetOptArrayArgVisitor: the options array's shape from its keys
        "curl_setopt_array" if args.len() >= 2 => {
            let options = ty(&args[1]).unwrap_or_else(mixed);
            let keys = constant_scalars(&options.iterable_key_type());
            let mut shape = ShapeBuilder::new();
            for key in &keys {
                let Some(t) = curl_option(c, key) else {
                    return variants;
                };
                let required = matches!(&options, Type::ConstArray(a) if a.keys.iter().position(|k| k == key).is_some_and(|i| !a.optional.contains(&i)));
                shape.set(Some(key.clone()), t, !required);
            }
            (!keys.is_empty()).then(|| (1, shape.get()))
        }
        _ => None,
    };
    match overridden {
        Some((p, t)) => with_type(c, variants, p, t),
        None => variants,
    }
}

/// The first variant with parameter `p` typed `t`.
fn with_type(c: &Checker, variants: Variants, p: usize, t: Type) -> Variants {
    let mut acceptor = variants.all.into_iter().next().expect("a variant");
    if let (Some(described), Some(param_type)) = (acceptor.types.get_mut(p), acceptor.param_types.get_mut(p)) {
        *described = declared::with_classes(c, || t.describe(Level::TypeOnly));
        *param_type = ParamType::Typed(t);
    }
    Variants { all: vec![acceptor], named: None }
}

/// ClosureBindArgVisitor and ClosureBindToVarVisitor: `Closure::bind($var, $newThis)` (`p` 1) or
/// `$var->bindTo($newThis)` (`p` 0) in a function-like whose parameter `$var` is `@param-closure-this` typed and keeps
/// its original value at `at`: `$newThis` takes that type.
pub(super) fn closure_bind(c: &Checker, var: &str, at: u32, p: usize, variants: Variants) -> Variants {
    let function = c.var_tag_functions.last().and_then(|span| c.codebase.function_likes.iter().find(|(_, f)| f.span == *span));
    let Some(((class, _), f)) = function else {
        return variants;
    };
    let declaring = c.codebase.get_class_like(class.as_bytes());
    let Some(t) = declared::codex_closure_this(c, f, declaring).remove(var) else {
        return variants;
    };
    // `ParameterVariableOriginalValueExpr`: no assignment to `$var` in the body before `at`
    // ponytail: a textual scan (the body from its first `{`); a by-reference use or `foreach (... as $var)` is missed
    let source = String::from_utf8_lossy(&c.file.contents[f.span.start.offset as usize..at as usize]);
    let body = &source[source.find('{').unwrap_or(source.len())..];
    let needle = format!("${var}");
    let assigned = body.match_indices(&needle).any(|(i, _)| {
        let rest = &body[i + needle.len()..];
        let rest = rest.trim_start();
        !rest.starts_with(|ch: char| ch.is_alphanumeric() || ch == '_')
            && ["=", ".=", "+=", "-=", "*=", "/=", "%=", "**=", "??=", "&=", "|=", "^=", "<<=", ">>="]
                .iter()
                .any(|op| rest.starts_with(op) && !(*op == "=" && rest[1..].starts_with(['=', '>'])))
    });
    if assigned { variants } else { with_type(c, variants, p, t) }
}

/// `getConstantScalarTypes()` of an option or key type.
fn constant_scalars(t: &Type) -> Vec<Type> {
    match t {
        Type::ConstInt(_) | Type::ConstString(_) => vec![t.clone()],
        Type::Union(ts) => ts.iter().flat_map(constant_scalars).collect(),
        _ => vec![],
    }
}

/// `getCurlOptValueType()`: the value type of curl option `option` (an integer the oracle's curl knows), exported
/// per the PHP version from which it holds.
fn curl_option(c: &Checker, option: &Type) -> Option<Type> {
    static OPTIONS: std::sync::OnceLock<Vec<(u32, std::collections::HashMap<i64, String>)>> = std::sync::OnceLock::new();
    let Type::ConstInt(option) = option else {
        return None;
    };
    let options = OPTIONS.get_or_init(|| {
        let by_version: std::collections::BTreeMap<u32, std::collections::HashMap<i64, String>> =
            serde_json::from_str(include_str!("curl-options.json")).expect("curl-options.json");
        by_version.into_iter().collect()
    });
    let (_, types) = options.iter().rev().find(|(v, _)| *v <= c.php_version).unwrap_or(&options[0]);
    types.get(option).map(|t| pun_phpstan::resolve::resolve_str(t, None))
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
