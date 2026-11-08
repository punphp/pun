//! PHPStan's `Type::describe(VerbosityLevel::typeOnly())` for the types Mago's analyser gives, as the report rules
//! ported from phpstan-strict-rules print them in their messages (`Variable method call on %s.`).
//!
//! A union is described as PHPStan's `UnionType` describes itself at that level: each member generalized (a literal
//! `'a'` is `string`, `1` is `int`, an enum case stays `Enum::Case`, `true` and `false` stay, both are `bool`), the
//! arrays merged into one as `TypeCombinator::union` merges them, the same descriptions merged, sorted as
//! `UnionTypeHelper::sortTypes` sorts them (`null` last, `true` and `false` before it, the rest by `strcasecmp` of their
//! descriptions), and joined with `|`; anything with `mixed` in it is `mixed`. An object is its class name, with its
//! type arguments in `<>` when it has them, a class the codebase does not know too; `$this` is `$this(Class)` and
//! `static` is `static(Class)`.

use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;

/// One member of a union, generalized, as the parts of the union it stands for (`array-key` is `int` and `string`); an
/// intersection (`A&B`) is one part, its members described and sorted as `IntersectionType::describe` sorts them
/// (`UnionTypeHelper::sortTypes`).
fn atomic(t: &TAtomic) -> Vec<String> {
    let Some(others) = t.get_intersection_types().filter(|o| !o.is_empty()) else {
        return single(t);
    };
    let mut parts = single(t);
    parts.extend(others.iter().flat_map(single));
    parts.sort_by_key(|p| p.to_ascii_lowercase());
    vec![parts.join("&")]
}

fn single(t: &TAtomic) -> Vec<String> {
    let one = |s: &str| vec![s.to_string()];
    match t {
        // The value of a call of a `void` function is `null` in PHPStan's scope.
        TAtomic::Null | TAtomic::Void => one("null"),
        TAtomic::Never => one("never"),
        TAtomic::Scalar(s) => match s {
            // `ConstantBooleanType` is not generalized.
            TScalar::Bool(b) if b.is_true() => one("true"),
            TScalar::Bool(b) if b.is_false() => one("false"),
            TScalar::Bool(_) => one("bool"),
            TScalar::Integer(_) => one("int"),
            TScalar::Float(_) => one("float"),
            TScalar::String(_) | TScalar::ClassLikeString(_) => one("string"),
            TScalar::ArrayKey => vec!["int".into(), "string".into()],
            // `numeric-string` is `string` once generalized.
            TScalar::Numeric => vec!["float".into(), "int".into(), "string".into()],
            TScalar::Generic => vec!["bool".into(), "float".into(), "int".into(), "string".into()],
        },
        TAtomic::Array(a) => vec![Array::of(a).print()],
        TAtomic::Iterable(_) => one("iterable"),
        TAtomic::Resource(_) => one("resource"),
        TAtomic::Callable(c) => match c.is_closure() {
            true => one("Closure"),
            false => one("callable"),
        },
        TAtomic::Object(TObject::Named(n)) => {
            let mut name = n.name.to_string();
            if let Some(params) = &n.type_parameters {
                let params: Vec<String> = params.iter().map(describe).collect();
                name = format!("{name}<{}>", params.join(", "));
            }
            if n.is_this {
                vec![format!("$this({name})")]
            } else if n.is_static {
                vec![format!("static({name})")]
            } else {
                vec![name]
            }
        }
        // An enum case is `EnumCaseObjectType`, `Enum::Case`.
        TAtomic::Object(TObject::Enum(e)) => match &e.case {
            Some(case) => vec![format!("{}::{case}", e.name)],
            None => vec![e.name.to_string()],
        },
        TAtomic::Object(_) => one("object"),
        // A class the codebase does not know stays a reference; PHPStan's `ObjectType` describes it by its name.
        TAtomic::Reference(TReference::Symbol { name, .. }) => vec![name.to_string()],
        _ => one("mixed"),
    }
}

/// An array as PHPStan sees it: its keys (`None` for any key, `array-key`), its values (`None` for `mixed`), whether it
/// is a `list<V>`, whether it is empty (`array{}`), and for a constant list (`array{1, 2}`) its required and total
/// number of items.
struct Array {
    keys: Option<Vec<String>>,
    values: Option<Vec<String>>,
    list: bool,
    empty: bool,
    shape: Option<(usize, usize)>,
}

impl Array {
    fn of(a: &TArray) -> Array {
        let parts = |t: &TUnion| t.types.iter().flat_map(atomic).collect::<Vec<String>>();
        let (mut keys, mut values, mut shape, mut list) = (Vec::new(), Vec::new(), None, false);
        match a {
            TArray::List(l) => {
                let Some(known) = &l.known_elements else {
                    return Array {
                        keys: Some(vec!["int".into()]),
                        values: Some(parts(&l.element_type)),
                        list: true,
                        empty: false,
                        shape: None,
                    };
                };
                // A list of known items is a constant array, `array<int, V>`.
                keys.push("int".into());
                for (_, value) in known.values() {
                    values.extend(parts(value));
                }
                let required = known.values().filter(|(optional, _)| !optional).count();
                shape = Some((required, known.len()));
                // Mago folds `array{}` into a list that may be empty; PHPStan keeps it apart, which generalizes the
                // list.
                if !l.non_empty {
                    shape = None;
                    list = true;
                }
            }
            TArray::Keyed(k) => {
                if let Some((key, value)) = &k.parameters {
                    let key = parts(key);
                    let any_key =
                        key.iter().any(|k| k == "int") && key.iter().any(|k| k == "string");
                    let value = (!value.is_mixed()).then(|| parts(value));
                    if k.known_items.is_none() {
                        return Array {
                            keys: (!any_key).then_some(key),
                            values: value,
                            list: false,
                            empty: false,
                            shape: None,
                        };
                    }
                    keys.extend(key);
                    values.extend(value.unwrap_or_else(|| vec!["mixed".into()]));
                }
                for (key, (_, value)) in k.known_items.iter().flat_map(|known| known.iter()) {
                    keys.push(match key {
                        ArrayKey::Integer(_) => "int".into(),
                        _ => "string".into(),
                    });
                    values.extend(parts(value));
                }
            }
        }
        Array {
            empty: values.is_empty(),
            keys: Some(keys),
            values: Some(values),
            list,
            shape,
        }
    }

    /// `ConstantArrayType::generalize`, as a union generalizes its members: a constant list is a `list<V>`.
    fn generalize(self) -> Array {
        Array {
            list: self.list || self.shape.is_some(),
            shape: None,
            ..self
        }
    }

    /// `TypeCombinator::union` of two arrays: the keys and values joined, a list while both are. `array{}` stays apart
    /// (`reduceArrays`), so the union generalizes the other; two constant lists merge into one, the items past the
    /// shorter one optional.
    fn merge(self, other: Array) -> Array {
        // An empty one second.
        let mut pair = [self, other];
        pair.sort_by_key(|a| a.empty);
        let [this, other] = pair;
        if other.empty {
            return this.generalize();
        }
        let both = |a: Option<Vec<String>>, b: Option<Vec<String>>| Some([a?, b?].concat());
        let shape = match (this.shape, other.shape) {
            (Some((r1, t1)), Some((r2, t2))) => Some((r1.min(r2), t1.max(t2))),
            _ => None,
        };
        let list = |a: &Array| a.list || a.shape.is_some();
        Array {
            list: shape.is_none() && list(&this) && list(&other),
            keys: both(this.keys, other.keys),
            values: both(this.values, other.values),
            empty: false,
            shape,
        }
    }

    /// `ArrayType::describe`: `array` for any key and value, `array<V>` for any key, `array<K, V>` else, `list<V>`.
    fn print(self) -> String {
        if self.empty {
            return "array".into();
        }
        let values = self.values.map_or("mixed".into(), join);
        // `shouldBeDescribedAsAList`: a constant list with more than one optional item (they are its last ones).
        let list = self.list
            || self
                .shape
                .is_some_and(|(required, total)| total - required > 1);
        match (list, self.keys) {
            (true, _) => format!("list<{values}>"),
            (false, None) if values == "mixed" => "array".into(),
            (false, None) => format!("array<{values}>"),
            (false, Some(keys)) => format!("array<{}, {values}>", join(keys)),
        }
    }
}

/// A union of `parts` as `UnionType::describe` prints it.
fn join(mut parts: Vec<String>) -> String {
    let mut seen: Vec<String> = Vec::new();
    parts.retain(|p| {
        let new = !seen.contains(p);
        seen.push(p.clone());
        new
    });
    drop(seen);
    if parts.iter().any(|p| p == "mixed") {
        return "mixed".into();
    }
    let has = |s: &str| parts.iter().any(|p| p == s);
    if has("bool") || (has("true") && has("false")) {
        parts.retain(|p| p != "true" && p != "false");
        if !parts.iter().any(|p| p == "bool") {
            parts.push("bool".into());
        }
    }
    // `null` last, a `true` or `false` (`ConstantBooleanType`) before it, the rest by `strcasecmp`.
    let rank = |p: &str| (p == "null", p == "true" || p == "false");
    parts.sort_by(|a, b| {
        (rank(a).cmp(&rank(b)))
            .then_with(|| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()))
            .then_with(|| a.cmp(b))
    });
    parts.join("|")
}

/// `$type->describe(VerbosityLevel::typeOnly())` for `t`. A lone `array-key` is the benevolent union
/// `(int|string)`.
pub(crate) fn describe(t: &TUnion) -> String {
    if let [TAtomic::Scalar(TScalar::ArrayKey)] = t.types.as_ref() {
        return "(int|string)".into();
    }
    let mut parts = Vec::new();
    let mut arrays: Option<Array> = None;
    for a in t.types.iter() {
        match a {
            TAtomic::Array(array)
                if a.get_intersection_types()
                    .is_none_or(<[mago_codex::ttype::atomic::TAtomic]>::is_empty) =>
            {
                let array = Array::of(array);
                arrays = Some(match arrays {
                    Some(before) => before.merge(array),
                    None => array,
                });
            }
            _ => parts.extend(atomic(a)),
        }
    }
    // Beside other types the union generalizes the array.
    let others = !parts.is_empty();
    parts.extend(arrays.map(|a| if others { a.generalize() } else { a }.print()));
    join(parts)
}
