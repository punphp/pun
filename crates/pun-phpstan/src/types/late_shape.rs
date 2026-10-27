//! `LateResolvableArrayShapeType`: preserve template keys until substitution, including their ErrorType reasons.

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LateArrayShape {
    pub items: Vec<(Option<Type>, Type, bool)>,
    pub unsealed: Option<(Option<Type>, Type)>,
    pub kind: String,
}

impl LateArrayShape {
    pub fn create(self) -> Type {
        if self
            .items
            .iter()
            .filter_map(|(k, _, _)| k.as_ref())
            .any(contains_template)
            || self
                .unsealed
                .as_ref()
                .and_then(|(k, _)| k.as_ref())
                .is_some_and(contains_template)
        {
            Type::LateArrayShape(Box::new(self))
        } else {
            self.resolve()
        }
    }

    fn key(t: &Type) -> Type {
        if t == &Type::Error {
            return t.clone();
        }
        let t = t.resolve_to_bounds();
        let key = crate::resolve::to_array_key(&t);
        if key == Type::Error {
            return Type::Error.with_unresolvable_reason(format!(
                "Type {} cannot be used as an array shape key.",
                t.describe(Level::TypeOnly)
            ));
        }
        key
    }

    pub fn resolve(&self) -> Type {
        let mut builder = ShapeBuilder::new();
        let mut explicit = Vec::new();
        for (key, value, optional) in &self.items {
            let key = key.as_ref().map(Self::key);
            if key.as_ref().is_some_and(|t| *t == Type::Error) {
                return key.unwrap();
            }
            explicit.extend(key.clone());
            builder.set(key, value.clone(), *optional);
        }
        let list = self.kind.ends_with("list");
        if let Some((key, value)) = &self.unsealed {
            let key = key.as_ref().map_or_else(
                || {
                    if list {
                        Type::int_range(Some(0), None)
                    } else {
                        mixed()
                    }
                },
                Self::key,
            );
            let finite = match &key {
                Type::ConstInt(_) | Type::ConstString(_) => vec![key.clone()],
                Type::Union(ts)
                    if ts
                        .iter()
                        .all(|t| matches!(t, Type::ConstInt(_) | Type::ConstString(_))) =>
                {
                    ts.clone()
                }
                _ => vec![],
            };
            for k in finite.iter().filter(|k| !explicit.contains(k)) {
                builder.set(Some(k.clone()), value.clone(), true);
            }
            if finite.is_empty() {
                builder.make_unsealed(key, value.clone());
            }
        }
        let array = builder.get();
        // Non-constant template bounds degrade the builder to a general array, without widening its values.
        // Keep this late-shape operation here; ordinary literal-key ShapeBuilder users are unchanged.
        let array = match array {
            Type::ConstArray(a)
                if a.keys
                    .iter()
                    .any(|k| !matches!(k, Type::ConstInt(_) | Type::ConstString(_))) =>
            {
                let array = Type::array(a.iterable_key_type(), a.iterable_value_type());
                if a.is_iterable_at_least_once().yes() {
                    intersect(vec![array, Type::Accessory(Accessory::NonEmptyArray)])
                } else {
                    array
                }
            }
            array => array,
        };
        let mut types = vec![array];
        if list {
            types.push(Type::Accessory(Accessory::List));
        }
        if self.kind.starts_with("non-empty-") {
            types.push(Type::Accessory(Accessory::NonEmptyArray));
        }
        intersect(types)
    }

    pub fn traverse(&self, f: &dyn Fn(&Type) -> Type) -> Type {
        Self {
            items: self
                .items
                .iter()
                .map(|(k, v, o)| (k.as_ref().map(f), f(v), *o))
                .collect(),
            unsealed: self
                .unsealed
                .as_ref()
                .map(|(k, v)| (k.as_ref().map(f), f(v))),
            kind: self.kind.clone(),
        }
        .create()
    }

    pub fn describe(&self) -> String {
        self.print(false)
    }

    pub fn phpdoc(&self) -> String {
        self.print(true)
    }

    fn print(&self, ast: bool) -> String {
        // Printer and the AST's __toString() differ in union/intersection spacing and parentheses.
        // Template nodes print their names, not describe(precise)'s scope and inference strategy.
        fn node(t: &Type, ast: bool) -> String {
            let (types, separator) = match t {
                Type::Template(t) => return t.name.clone(),
                Type::Union(ts) | Type::Benevolent(ts) => (ts, if ast { " | " } else { "|" }),
                Type::Intersection(ts) => (ts, if ast { " & " } else { "&" }),
                _ => return to_phpdoc(t),
            };
            let text = sort_types(types)
                .iter()
                .map(|child| {
                    let text = node(child, ast);
                    if !ast
                        && matches!(t, Type::Union(_) | Type::Benevolent(_))
                        && matches!(child, Type::Intersection(_))
                    {
                        format!("({text})")
                    } else {
                        text
                    }
                })
                .collect::<Vec<_>>()
                .join(separator);
            if ast { format!("({text})") } else { text }
        }
        let mut items: Vec<_> = self
            .items
            .iter()
            .map(|(k, v, o)| {
                let key = k.as_ref().map(|k| match k {
                    Type::Template(t) => t.name.clone(),
                    Type::ConstString(s) if is_valid_identifier(&String::from_utf8_lossy(s)) => String::from_utf8_lossy(s).into_owned(),
                    _ => node(k, ast),
                });
                format!(
                    "{}{}",
                    key.map_or_else(String::new, |k| format!(
                        "{k}{}: ",
                        if *o { "?" } else { "" }
                    )),
                    node(v, ast)
                )
            })
            .collect();
        if let Some((k, v)) = &self.unsealed {
            items.push(k.as_ref().map_or_else(
                || {
                    if v.is_plain_implicit_mixed() {
                        "...".to_string()
                    } else {
                        format!("...<{}>", node(v, ast))
                    }
                },
                |k| format!("...<{}, {}>", node(k, ast), node(v, ast)),
            ));
        }
        format!("{}{{{}}}", self.kind, items.join(", "))
    }
}
