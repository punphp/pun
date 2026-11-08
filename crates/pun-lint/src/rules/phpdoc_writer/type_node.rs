//! phpdoc-parser's type nodes as Rector prints them: `(string) $typeNode` with Rector's own node classes
//! (`SpacingAwareArrayTypeNode`, `BracketsAwareUnionTypeNode`, `BracketsAwareIntersectionTypeNode`), which
//! `PhpDocNodeMapper` puts in place of phpdoc-parser's when a doc is parsed and the static type mapper creates.

use std::fmt;

use pun_phpstan::phpdoc::{ConstExpr, Node, ShapeKey};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TypeNode {
    /// `IdentifierTypeNode`: a name as written (`int`, `\Foo\Bar`, `Bar`).
    Identifier(String),
    /// `SpacingAwareArrayTypeNode`: `T[]`; `generic_parent` is `ArrayTypeMapper::HAS_GENERIC_TYPE_PARENT`.
    Array {
        inner: Box<TypeNode>,
        generic_parent: bool,
    },
    /// `GenericTypeNode`: `base<args>`.
    Generic { base: String, args: Vec<TypeNode> },
    /// `BracketsAwareUnionTypeNode`, `wrapped` in parentheses.
    Union { types: Vec<TypeNode>, wrapped: bool },
    /// `BracketsAwareIntersectionTypeNode`.
    Intersection(Vec<TypeNode>),
    /// `ArrayShapeNode`: `kind{key?: value, ...}`; `unsealed` is the printed `<K, V>` of `...<K, V>`.
    Shape {
        kind: String,
        items: Vec<(Option<String>, bool, TypeNode)>,
        sealed: bool,
        unsealed: String,
    },
    /// `CallableTypeNode`: `callable<T>(int): string`, its templates and parameters printed.
    Callable {
        ident: String,
        templates: Vec<String>,
        params: Vec<String>,
        ret: Box<TypeNode>,
    },
    /// `NullableTypeNode`.
    Nullable(Box<TypeNode>),
    /// Any other node, printed as PHPStan's `toPhpDocNode()` prints it.
    Raw(String),
}

impl TypeNode {
    pub(crate) fn ident(name: &str) -> TypeNode {
        TypeNode::Identifier(name.to_string())
    }

    /// The node phpdoc-parser parses from `node`, as Rector's `PhpDocNodeMapper` leaves it. `None` for a node Pun
    /// does not print as Rector does (a quoted string, an object shape, a conditional, an offset access).
    pub(crate) fn from_parsed(node: &Node) -> Option<TypeNode> {
        from_parsed(node, false, None)
    }

    /// [`Self::from_parsed`] that prints object shapes, conditionals, offset accesses and quoted strings too; `quote`
    /// gives a string as written.
    pub(crate) fn from_parsed_all(
        node: &Node,
        quote: &dyn Fn(&[u8]) -> Option<String>,
    ) -> Option<TypeNode> {
        from_parsed(node, false, Some(quote))
    }

    /// `SpacingAwareArrayTypeNode::isGenericArrayCandidate()`'s other half: a union or an array.
    fn is_union_or_array(&self) -> bool {
        matches!(self, TypeNode::Union { .. } | TypeNode::Array { .. })
    }
}

fn from_parsed(
    node: &Node,
    in_array: bool,
    all: Option<&dyn Fn(&[u8]) -> Option<String>>,
) -> Option<TypeNode> {
    Some(match node {
        Node::Ident(name) => TypeNode::Identifier(name.clone()),
        Node::This => TypeNode::ident("$this"),
        Node::Nullable(inner) => TypeNode::Nullable(Box::new(from_parsed(inner, false, all)?)),
        // `UnionTypeNodePhpDocNodeVisitor`: a union written in parentheses (inside `[]`, a callable's return) is wrapped
        Node::Union(types) => TypeNode::Union {
            types: types
                .iter()
                .map(|t| from_parsed(t, false, all))
                .collect::<Option<_>>()?,
            wrapped: in_array,
        },
        Node::Inter(types) => TypeNode::Intersection(
            types
                .iter()
                .map(|t| from_parsed(t, false, all))
                .collect::<Option<_>>()?,
        ),
        Node::Array(inner) => TypeNode::Array {
            inner: Box::new(from_parsed(inner, true, all)?),
            generic_parent: false,
        },
        Node::Generic { base, args } => TypeNode::Generic {
            base: base.clone(),
            args: args
                .iter()
                .map(|t| from_parsed(t, false, all))
                .collect::<Option<_>>()?,
        },
        Node::ArrayShape {
            kind,
            items,
            sealed,
            unsealed,
        } => TypeNode::Shape {
            kind: kind.clone(),
            items: items
                .iter()
                .map(|i| {
                    // (a quoted key is printed in its own quotes, which Pun's node does not keep)
                    let key = match &i.key {
                        None => None,
                        Some(ShapeKey::Int(k) | ShapeKey::Ident(k)) => Some(k.clone()),
                        Some(ShapeKey::Fetch(class, name)) => Some(format!("{class}::{name}")),
                        Some(ShapeKey::Str(k)) => Some(all?(k)?),
                    };
                    Some((key, i.optional, from_parsed(&i.value, false, all)?))
                })
                .collect::<Option<_>>()?,
            sealed: *sealed,
            // `ArrayShapeUnsealedTypeNode`
            unsealed: match unsealed {
                None => String::new(),
                Some((None, value)) => format!("<{}>", from_parsed(value, false, all)?),
                Some((Some(key), value)) => {
                    format!(
                        "<{}, {}>",
                        from_parsed(key, false, all)?,
                        from_parsed(value, false, all)?
                    )
                }
            },
        },
        // `CallableTypeNode` with its templates (`TemplateTagValueNode`: `T of Bound super Lower = Default`)
        Node::Callable {
            ident,
            templates,
            params,
            ret,
        } => TypeNode::Callable {
            ident: ident.clone(),
            templates: templates
                .iter()
                .map(|t| {
                    let mut printed = t.name.clone();
                    for (word, node) in [
                        (" of ", &t.bound),
                        (" super ", &t.lower),
                        (" = ", &t.default),
                    ] {
                        if let Some(node) = node {
                            printed.push_str(word);
                            printed.push_str(&from_parsed(node, false, all)?.to_string());
                        }
                    }
                    Some(printed)
                })
                .collect::<Option<_>>()?,
            // `CallableTypeParameterNode`
            params: params
                .iter()
                .map(|p| {
                    let printed = format!(
                        "{} {}{}{}",
                        from_parsed(&p.ty, false, all)?,
                        if p.by_ref { "&" } else { "" },
                        if p.variadic { "..." } else { "" },
                        p.name
                    );
                    Some(format!(
                        "{}{}",
                        printed.trim(),
                        if p.optional { "=" } else { "" }
                    ))
                })
                .collect::<Option<_>>()?,
            // a union return is written in parentheses
            ret: Box::new(from_parsed(ret, true, all)?),
        },
        // `ConstTypeNode`: a number or `Class::NAME` (`true`, `false`, `null` and `NAME` are identifiers, and an
        // array is no type; a string is printed in its own quotes, which Pun's node does not keep)
        Node::Const(ConstExpr::Int(v) | ConstExpr::Float(v)) => TypeNode::Raw(v.clone()),
        Node::Const(ConstExpr::Fetch(class, name)) => TypeNode::Raw(format!("{class}::{name}")),
        // `ObjectShapeNode`, `ObjectShapeItemNode`
        Node::Const(ConstExpr::Str(v)) if all.is_some() => TypeNode::Raw(all?(v)?),
        Node::ObjectShape(items) if all.is_some() => TypeNode::Raw(format!(
            "object{{{}}}",
            items
                .iter()
                .map(|(key, optional, value)| {
                    let key = all?(key.as_bytes()).unwrap_or_else(|| key.clone());
                    Some(format!(
                        "{key}{}: {}",
                        if *optional { "?" } else { "" },
                        from_parsed(value, false, all)?
                    ))
                })
                .collect::<Option<Vec<_>>>()?
                .join(", ")
        )),
        // `ConditionalTypeNode`, `ConditionalTypeForParameterNode`
        Node::Conditional {
            subject,
            target,
            then,
            otherwise,
            negated,
        } if all.is_some() => TypeNode::Raw(format!(
            "({} {} {} ? {} : {})",
            from_parsed(subject, false, all)?,
            if *negated { "is not" } else { "is" },
            from_parsed(target, false, all)?,
            from_parsed(then, false, all)?,
            from_parsed(otherwise, false, all)?
        )),
        Node::ConditionalForParameter {
            param,
            target,
            then,
            otherwise,
            negated,
        } if all.is_some() => TypeNode::Raw(format!(
            "({param} {} {} ? {} : {})",
            if *negated { "is not" } else { "is" },
            from_parsed(target, false, all)?,
            from_parsed(then, false, all)?,
            from_parsed(otherwise, false, all)?
        )),
        // `GenericTypeNode::__toString()`'s arguments of a variance: `*` (bivariant), `covariant T`, `contravariant T`
        Node::Star(star) if all.is_some() && star == "*" => TypeNode::ident("*"),
        Node::Projection(covariant, inner) if all.is_some() => TypeNode::Raw(format!(
            "{} {}",
            if *covariant {
                "covariant"
            } else {
                "contravariant"
            },
            from_parsed(inner, false, all)?
        )),
        // `OffsetAccessTypeNode`: a callable or a nullable base in parentheses (a union is written in them)
        Node::OffsetAccess(base, offset) if all.is_some() => {
            let printed = from_parsed(base, true, all)?;
            TypeNode::Raw(match **base {
                Node::Callable { .. } | Node::Nullable(_) => {
                    format!("({printed})[{}]", from_parsed(offset, false, all)?)
                }
                _ => format!("{printed}[{}]", from_parsed(offset, false, all)?),
            })
        }
        _ => return None,
    })
}

impl fmt::Display for TypeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeNode::Identifier(name) | TypeNode::Raw(name) => f.write_str(name),
            TypeNode::Array {
                inner,
                generic_parent,
            } => f.write_str(&print_array(inner, *generic_parent)),
            TypeNode::Generic { base, args } => {
                write!(f, "{base}<{}>", join(args, ", "))
            }
            TypeNode::Union { types, wrapped } => {
                // `array_unique()` of the printed types
                let mut printed: Vec<String> = Vec::new();
                for t in types {
                    let s = t.to_string();
                    if !printed.contains(&s) {
                        printed.push(s);
                    }
                }
                if *wrapped {
                    write!(f, "({})", printed.join("|"))
                } else {
                    f.write_str(&printed.join("|"))
                }
            }
            TypeNode::Intersection(types) => f.write_str(&join(types, "&")),
            TypeNode::Shape {
                kind,
                items,
                sealed,
                unsealed,
            } => {
                let mut printed: Vec<String> = items
                    .iter()
                    .map(|(key, optional, value)| match key {
                        Some(key) => format!("{key}{}: {value}", if *optional { "?" } else { "" }),
                        None => value.to_string(),
                    })
                    .collect();
                if !sealed {
                    printed.push(format!("...{unsealed}"));
                }
                write!(f, "{kind}{{{}}}", printed.join(", "))
            }
            TypeNode::Nullable(inner) => write!(f, "?{inner}"),
            TypeNode::Callable {
                ident,
                templates,
                params,
                ret,
            } => {
                // phpdoc-parser 2.4's `Printer::printType()` (Rector 2.8.4 prints with it): `ident(params): ret`, the
                // return type in parentheses when it is a union, an intersection or a callable
                let wrap = matches!(
                    ret.as_ref(),
                    TypeNode::Union { .. } | TypeNode::Intersection(_) | TypeNode::Callable { .. }
                );
                let mut ret = ret.to_string();
                if wrap && !ret.starts_with('(') {
                    ret = format!("({ret})");
                }
                let params = format!("({})", params.join(", "));
                let ret = format!(": {ret}");
                let templates = if templates.is_empty() {
                    String::new()
                } else {
                    format!("<{}>", templates.join(", "))
                };
                write!(f, "{ident}{templates}{params}{ret}")
            }
        }
    }
}

fn join(types: &[TypeNode], separator: &str) -> String {
    types
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(separator)
}

/// `SpacingAwareArrayTypeNode::__toString()`.
fn print_array(inner: &TypeNode, generic_parent: bool) -> String {
    let printed = inner.to_string();
    if generic_parent && inner.is_union_or_array() {
        return format!("array<{printed}>");
    }
    match inner {
        // `printArrayType()`
        TypeNode::Array { .. } => printed
            .split('|')
            .map(|t| format!("{t}[]"))
            .collect::<Vec<_>>()
            .join("|"),
        // `printUnionType()`
        TypeNode::Union {
            types,
            wrapped: false,
        } => {
            let mut base = None;
            let all_generic = types.iter().all(|t| match t {
                TypeNode::Generic { base: b, args } if args.len() == 1 => {
                    *base.get_or_insert(b) == b
                }
                _ => false,
            });
            if all_generic {
                return format!("array<int, {printed}>");
            }
            types.iter().map(array_of).collect::<Vec<_>>().join("|")
        }
        _ => array_of(inner),
    }
}

/// phpdoc-parser's `ArrayTypeNode::__toString()`: `T[]`, an intersection, a callable or a nullable type in
/// parentheses.
fn array_of(t: &TypeNode) -> String {
    match t {
        TypeNode::Intersection(_) | TypeNode::Callable { .. } | TypeNode::Nullable(_) => {
            format!("({t})[]")
        }
        _ => format!("{t}[]"),
    }
}
