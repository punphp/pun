//! Rector's phpdoc writer, shared by the rules that add or change `@param`, `@var` and `@return` tags, ported from
//! Rector 2.6.1 (src/BetterPhpDocParser, src/PHPStanStaticTypeMapper, src/Comments) on PHPStan 2.2.7:
//!
//! - [`DocTarget`]: a node's comments as php-parser assigns them, its [`PhpDocInfo`]
//!   (`PhpDocInfoFactory::createFromNodeOrEmpty()`), and the edit that writes a changed doc back as php-parser's
//!   format-preserving printer does (`DocBlockUpdater`, then `pComments()` at the node's indentation).
//! - [`doc_info`]: `PhpDocInfo` (`getReturnTagValue()`, `getParamTagValueByName()`, `getVarTagValueNode()`, the
//!   `PhpDocTypeChanger` setters) and `PhpDocInfoPrinter`.
//! - [`type_mapper`]: `PHPStanStaticTypeMapper::mapToPHPStanPhpDocTypeNode()` with `ArrayTypeMapper` and
//!   `UnionTypeMapper`, `TypeNormalizer::generalizeConstantTypes()`, `ConstantArrayTypeGeneralizer` and
//!   `NodeDocblockTypeDecorator`'s checks.
//! - [`type_node`]: the type nodes, printed as Rector prints them.
//! - [`phpstan_type`] and [`expr_type`]: PHPStan's type of an expression (`getType()`, `getNativeType()`) from
//!   Mago's, and of a doc type ([`DocTarget::doc_type`], `mapPHPStanPhpDocTypeNodeToPHPStanType()`).

pub mod doc_info;
pub mod type_mapper;
pub mod type_node;

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{ArrayElement, Expression, Literal, Node};
use pun_phpstan::phpdoc::Node as DocNode;
use pun_phpstan::resolve::{Scope, resolve_in};
use pun_phpstan::types::{NeverKind, ShapeBuilder, Type, intersect, union, Accessory};
use pun_problem::{Edit, Span};

pub use doc_info::{PhpDocInfo, TypedValue};
pub use type_node::TypeNode;

use super::php_parser::{previous_token_end, tokens};
use super::phpunit::annotations::reformatted;
use super::rector_conditions::indentation;
use super::types::facts::facts;
use super::types::names::resolve_written_class_name;
use crate::Context;
use crate::syntax::{node_span, unparenthesize};

/// A node that carries a doc comment (a function, method, property or class), as php-parser sees it: where it
/// starts (its first attribute or modifier) and the comments php-parser gives it.
pub struct DocTarget {
    /// The node's first token.
    pub start: usize,
    /// The comments between the token before the node and the node.
    pub comments: Vec<Span>,
    /// Which of them is the doc comment (`getDocComment()`: the last `/**` one).
    pub doc: Option<usize>,
}

impl DocTarget {
    /// The comments of the node that starts at `start`.
    pub fn of(source: &str, start: usize) -> DocTarget {
        let tokens = tokens(source);
        let before = previous_token_end(source, &tokens, start);
        let comments: Vec<Span> = tokens
            .iter()
            .filter(|t| t.start >= before && t.end <= start && t.kind.is_comment())
            .map(|t| Span::new(t.start, t.end))
            .collect();
        let doc = comments.iter().rposition(|c| {
            let text = &source[c.start..c.end];
            text.starts_with("/**") && text.as_bytes().get(3).is_some_and(u8::is_ascii_whitespace)
        });
        DocTarget {
            start,
            comments,
            doc,
        }
    }

    /// `PhpDocInfoFactory::createFromNodeOrEmpty()`. `None` where Pun does not read the doc comment as
    /// phpdoc-parser does.
    pub fn info(&self, source: &str) -> Option<PhpDocInfo> {
        match self.doc {
            Some(i) => PhpDocInfo::parse(&source[self.comments[i].start..self.comments[i].end]),
            None => Some(PhpDocInfo::empty()),
        }
    }

    /// The edit that gives the node the doc comment `doc` (`Node::setDocComment()`, which replaces the doc comment
    /// among the node's comments or adds it after them), as php-parser's format-preserving printer writes changed
    /// comments: `pComments()` (each comment's `getReformattedText()`, its line breaks followed by the node's
    /// indentation, a tab counting four spaces) from the first comment to the node, then a line break and the
    /// indentation.
    pub fn edit(&self, source: &str, doc: &str) -> Edit {
        let nl = format!("\n{}", " ".repeat(indentation(source, self.start)));
        let mut texts: Vec<&str> = self.comments.iter().map(|c| &source[c.start..c.end]).collect();
        match self.doc {
            Some(i) => texts[i] = doc,
            None => texts.push(doc),
        }
        let printed: Vec<String> = texts.iter().map(|t| reformatted(t).replace('\n', &nl)).collect();
        let from = self.comments.first().map_or(self.start, |c| c.start);
        Edit::new(Span::new(from, self.start), format!("{}{nl}", printed.join(&nl)))
    }

    /// `StaticTypeMapper::mapPHPStanPhpDocTypeNodeToPHPStanType()` of a tag's type written at the node
    /// `ancestors` lead to: names resolved through the namespace and `use` imports, `self` and `static` the class
    /// `class`.
    pub fn doc_type(tag: &TypedValue, ancestors: &[Node<'_, '_>], class: Option<&str>) -> Type {
        match &tag.parsed {
            Some(parsed) => doc_type(parsed, ancestors, class),
            None => Type::Mixed { explicit: false },
        }
    }
}

/// PHPStan's `NameScope` of a place in the file.
struct NameScope<'a, 'b> {
    ancestors: &'a [Node<'b, 'b>],
    class: Option<&'a str>,
}

impl Scope for NameScope<'_, '_> {
    fn class(&self) -> Option<&str> {
        self.class
    }
    fn parent(&self) -> Option<String> {
        None
    }
    fn resolve_name(&self, name: &str) -> String {
        resolve_written_class_name(self.ancestors, name)
    }
    fn in_namespace(&self) -> bool {
        self.ancestors.iter().any(|a| matches!(a, Node::Namespace(_)))
    }
}

/// The PHPStan type of a parsed doc type written at the node `ancestors` lead to.
pub fn doc_type(parsed: &DocNode, ancestors: &[Node<'_, '_>], class: Option<&str>) -> Type {
    resolve_in(parsed, &NameScope { ancestors, class })
}

/// PHPStan's type for Mago's `t`. `None` for a type Pun does not map.
pub fn phpstan_type(t: &TUnion) -> Option<Type> {
    let types: Vec<Type> = t.types.iter().map(phpstan_atomic).collect::<Option<_>>()?;
    Some(match types.len() {
        0 => Type::Never(NeverKind::Implicit),
        1 => types.into_iter().next().expect("one type"),
        _ => union(types),
    })
}

fn phpstan_atomic(atomic: &TAtomic) -> Option<Type> {
    Some(match atomic {
        TAtomic::Null => Type::Null,
        TAtomic::Never => Type::Never(NeverKind::Implicit),
        TAtomic::Void => Type::Void,
        TAtomic::Mixed(_) => Type::Mixed { explicit: false },
        TAtomic::Scalar(TScalar::Bool(b)) => match b.value {
            Some(v) => Type::ConstBool(v),
            None => Type::Bool,
        },
        TAtomic::Scalar(TScalar::Integer(i)) => match i {
            TInteger::Literal(v) => Type::ConstInt(*v),
            TInteger::From(min) => Type::int_range(Some(*min), None),
            TInteger::To(max) => Type::int_range(None, Some(*max)),
            TInteger::Range(min, max) => Type::int_range(Some(*min), Some(*max)),
            TInteger::Unspecified | TInteger::UnspecifiedLiteral => Type::Int,
        },
        TAtomic::Scalar(TScalar::Float(f)) => match f.get_literal_value() {
            Some(v) => Type::ConstFloat(v),
            None => Type::Float,
        },
        TAtomic::Scalar(TScalar::String(s)) => match s.get_known_literal_value() {
            Some(v) => Type::ConstString(String::from_utf8_lossy(v).into_owned()),
            None if s.is_non_empty || s.is_truthy => {
                intersect(vec![Type::String, Type::Accessory(Accessory::NonEmptyString)])
            }
            None => Type::String,
        },
        TAtomic::Scalar(TScalar::ClassLikeString(c)) => match c {
            TClassLikeString::Literal { value } => {
                Type::ClassString(Some(Box::new(Type::Object(value.to_string(), vec![]))))
            }
            TClassLikeString::OfType { constraint, .. } => match &**constraint {
                TAtomic::Object(TObject::Named(n)) if n.type_parameters.is_none() => {
                    Type::ClassString(Some(Box::new(Type::Object(n.name.to_string(), vec![]))))
                }
                _ => Type::ClassString(None),
            },
            _ => Type::ClassString(None),
        },
        TAtomic::Scalar(TScalar::ArrayKey) => {
            Type::Benevolent(vec![Type::Int, Type::String])
        }
        TAtomic::Array(TArray::Keyed(k)) => {
            if let Some(items) = &k.known_items {
                if k.parameters.is_some() {
                    return None;
                }
                let mut b = ShapeBuilder::new();
                for (key, (optional, value)) in items.iter() {
                    let key = match key {
                        ArrayKey::Integer(i) => Type::ConstInt(*i),
                        ArrayKey::String(s) => Type::ConstString(s.to_string()),
                        _ => return None,
                    };
                    b.set(Some(key), phpstan_type(value)?, *optional);
                }
                b.get()
            } else {
                let (key, value) = match &k.parameters {
                    Some((key, value)) => (phpstan_type(key)?, phpstan_type(value)?),
                    None => return Some(Type::ConstArray(Box::new(pun_phpstan::types::ConstArray::empty()))),
                };
                let array = Type::array(key, value);
                if k.non_empty {
                    intersect(vec![array, Type::Accessory(Accessory::NonEmptyArray)])
                } else {
                    array
                }
            }
        }
        TAtomic::Array(TArray::List(l)) => {
            if let Some(elements) = &l.known_elements {
                if !l.element_type.is_never() {
                    return None;
                }
                let mut b = ShapeBuilder::new();
                for (index, (optional, value)) in elements.iter() {
                    b.set(Some(Type::ConstInt(*index as i64)), phpstan_type(value)?, *optional);
                }
                b.get()
            } else {
                let value = phpstan_type(&l.element_type)?;
                let mut types = vec![
                    Type::array(Type::int_range(Some(0), None), value),
                    Type::Accessory(Accessory::List),
                ];
                if l.non_empty {
                    types.push(Type::Accessory(Accessory::NonEmptyArray));
                }
                intersect(types)
            }
        }
        TAtomic::Object(TObject::Named(n)) if n.intersection_types.is_none() && !n.is_this => {
            let args = match &n.type_parameters {
                Some(params) => params.iter().map(phpstan_type).collect::<Option<_>>()?,
                None => vec![],
            };
            Type::Object(n.name.to_string(), args)
        }
        TAtomic::Object(TObject::Enum(e)) => Type::Object(e.name.to_string(), vec![]),
        TAtomic::Object(TObject::Any) => Type::ObjectWithoutClass,
        _ => return None,
    })
}

/// `getType()` of `expr`: an array literal as PHPStan's `ConstantArrayTypeBuilder` builds it (its items in the
/// order written), a literal or `Name::class` from the syntax, anything else Mago's type with the docblocks read.
pub fn expr_type(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> Option<Type> {
    let expr = unparenthesize(expr);
    match expr {
        Expression::Array(a) => array_type(ctx, ancestors, a.elements.iter()),
        Expression::LegacyArray(a) => array_type(ctx, ancestors, a.elements.iter()),
        Expression::Literal(Literal::String(s)) => {
            s.value.map(|v| Type::ConstString(String::from_utf8_lossy(v).into_owned()))
        }
        Expression::Literal(Literal::Integer(i)) => match i.value {
            Some(v) if v <= i64::MAX as u64 => Some(Type::ConstInt(v as i64)),
            _ => None,
        },
        Expression::Literal(Literal::True(_)) => Some(Type::ConstBool(true)),
        Expression::Literal(Literal::False(_)) => Some(Type::ConstBool(false)),
        Expression::Literal(Literal::Null(_)) => Some(Type::Null),
        Expression::Access(mago_syntax::cst::Access::ClassConstant(c))
            if matches!(c.constant, mago_syntax::cst::ClassLikeConstantSelector::Identifier(ref id) if id.value.eq_ignore_ascii_case(b"class")) =>
        {
            match c.class {
                Expression::Identifier(id) => Some(Type::ClassString(Some(Box::new(Type::Object(
                    resolve_written_class_name(ancestors, &String::from_utf8_lossy(id.value())),
                    vec![],
                ))))),
                _ => facts(ctx).full_type(node_span(expr)).and_then(phpstan_type),
            }
        }
        _ => facts(ctx).full_type(node_span(expr)).and_then(phpstan_type),
    }
}

/// `ConstantArrayTypeBuilder` over an array literal's elements.
fn array_type<'a>(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    elements: impl Iterator<Item = &'a ArrayElement<'a>>,
) -> Option<Type> {
    let mut b = ShapeBuilder::new();
    for element in elements {
        match element {
            ArrayElement::KeyValue(kv) => {
                let key = match expr_type(ctx, ancestors, kv.key)? {
                    Type::ConstString(s) => match s.parse::<i64>() {
                        // a decimal integer string key is an integer key (`toArrayKey()`)
                        Ok(i) if i.to_string() == s => Type::ConstInt(i),
                        _ => Type::ConstString(s),
                    },
                    key @ Type::ConstInt(_) => key,
                    _ => return None,
                };
                b.set(Some(key), expr_type(ctx, ancestors, kv.value)?, false);
            }
            ArrayElement::Value(v) => b.set(None, expr_type(ctx, ancestors, v.value)?, false),
            _ => return None,
        }
    }
    Some(b.get())
}
