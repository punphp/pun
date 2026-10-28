//! phpdoc-parser's type nodes as Rector prints them: `(string) $typeNode` with Rector's own node classes
//! (`SpacingAwareArrayTypeNode`, `BracketsAwareUnionTypeNode`, `BracketsAwareIntersectionTypeNode`), which
//! `PhpDocNodeMapper` puts in place of phpdoc-parser's when a doc is parsed and the static type mapper creates.

use std::fmt;

use pun_phpstan::phpdoc::{Node, ShapeKey};

/// A type node.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeNode {
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
    /// `ArrayShapeNode`: `kind{key?: value, ...}`.
    Shape {
        kind: String,
        items: Vec<(Option<String>, bool, TypeNode)>,
        sealed: bool,
    },
    /// `NullableTypeNode`.
    Nullable(Box<TypeNode>),
    /// Any other node, printed as PHPStan's `toPhpDocNode()` prints it.
    Raw(String),
}

impl TypeNode {
    pub fn ident(name: &str) -> TypeNode {
        TypeNode::Identifier(name.to_string())
    }

    /// The node phpdoc-parser parses from `node`, as Rector's `PhpDocNodeMapper` leaves it. `None` for a node Pun
    /// does not print as Rector does (a constant, a callable, a conditional).
    pub fn from_parsed(node: &Node) -> Option<TypeNode> {
        from_parsed(node, false)
    }

    /// `SpacingAwareArrayTypeNode::isGenericArrayCandidate()`'s other half: a union or an array.
    fn is_union_or_array(&self) -> bool {
        matches!(self, TypeNode::Union { .. } | TypeNode::Array { .. })
    }
}

fn from_parsed(node: &Node, in_array: bool) -> Option<TypeNode> {
    Some(match node {
        Node::Ident(name) => TypeNode::Identifier(name.clone()),
        Node::This => TypeNode::ident("$this"),
        Node::Nullable(inner) => TypeNode::Nullable(Box::new(from_parsed(inner, false)?)),
        // `UnionTypeNodePhpDocNodeVisitor`: a union inside `[]` is wrapped in parentheses
        Node::Union(types) => TypeNode::Union {
            types: types
                .iter()
                .map(|t| from_parsed(t, false))
                .collect::<Option<_>>()?,
            wrapped: in_array,
        },
        Node::Inter(types) => TypeNode::Intersection(
            types
                .iter()
                .map(|t| from_parsed(t, false))
                .collect::<Option<_>>()?,
        ),
        Node::Array(inner) => TypeNode::Array {
            inner: Box::new(from_parsed(inner, true)?),
            generic_parent: false,
        },
        Node::Generic { base, args } => TypeNode::Generic {
            base: base.clone(),
            args: args
                .iter()
                .map(|t| from_parsed(t, false))
                .collect::<Option<_>>()?,
        },
        Node::ArrayShape {
            kind,
            items,
            sealed,
            unsealed: None,
        } => TypeNode::Shape {
            kind: kind.clone(),
            items: items
                .iter()
                .map(|i| {
                    let key = match &i.key {
                        None => None,
                        Some(ShapeKey::Int(k) | ShapeKey::Ident(k)) => Some(k.clone()),
                        Some(_) => return None,
                    };
                    Some((key, i.optional, from_parsed(&i.value, false)?))
                })
                .collect::<Option<_>>()?,
            sealed: *sealed,
        },
        _ => return None,
    })
}

impl fmt::Display for TypeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeNode::Identifier(name) | TypeNode::Raw(name) => {
                f.write_str(name)
            }
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
            } => {
                let mut printed: Vec<String> = items
                    .iter()
                    .map(|(key, optional, value)| match key {
                        Some(key) => format!("{key}{}: {value}", if *optional { "?" } else { "" }),
                        None => value.to_string(),
                    })
                    .collect();
                if !sealed {
                    printed.push("...".to_string());
                }
                write!(f, "{kind}{{{}}}", printed.join(", "))
            }
            TypeNode::Nullable(inner) => write!(f, "?{inner}"),
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
            types
                .iter()
                .map(|t| format!("{t}[]"))
                .collect::<Vec<_>>()
                .join("|")
        }
        _ => format!("{printed}[]"),
    }
}
