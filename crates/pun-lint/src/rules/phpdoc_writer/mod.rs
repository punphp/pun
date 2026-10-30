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

use std::collections::HashMap;
use std::sync::Arc;

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Argument, ArrayElement, Call, ClassLikeMemberSelector, Expression, Hint, Literal, MethodCall, Node, Variable,
};
use pun_phpstan::phpdoc::Node as DocNode;
use pun_phpstan::resolve::{Scope, resolve_in};
use pun_phpstan::symbols::{Class, with_classes};
use pun_phpstan::types::{NeverKind, ShapeBuilder, Template, Type, is_super_type_of, union};
use pun_problem::{Edit, Span};

pub use doc_info::{PhpDocInfo, TypedValue};
pub use type_node::TypeNode;

use super::php_parser::{previous_token_end, tokens};
use super::require_named_argument_order::{Target, callee};
use super::phpunit::annotations::reformatted;
use super::rector_conditions::indentation;
use super::types::declare::expression_order;
use super::types::docblock::{class_members, class_name, is_this, method};
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

    /// `PhpDocInfoFactory::createFromNodeOrEmpty()`.
    pub fn info(&self, source: &str) -> PhpDocInfo {
        match self.doc {
            Some(i) => PhpDocInfo::parse(&source[self.comments[i].start..self.comments[i].end]),
            None => PhpDocInfo::empty(),
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
    /// `ancestors` lead to (`PhpDocTypeMapper`): a union's members each through PHPStan's `TypeNodeResolver` and kept
    /// as written (`UnionPhpDocTypeMapper`), any other node through the resolver.
    pub fn doc_type(tag: &TypedValue, ancestors: &[Node<'_, '_>], class: Option<&str>) -> Type {
        match &tag.parsed {
            Some(DocNode::Union(types)) => {
                let types: Vec<Type> = types.iter().map(|t| doc_type(t, ancestors, class)).collect();
                type_mapper::create_mixed_passed_or_union_type(&types, true)
            }
            parsed => parsed.as_ref().map_or(Type::Mixed { explicit: false }, |p| doc_type(p, ancestors, class)),
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
        let class = self.ancestors.iter().filter_map(|a| if let Node::Class(c) = a { Some(c) } else { None }).next_back()?;
        let parent = class.extends.as_ref()?.types.nodes.first()?;
        Some(resolve_written_class_name(self.ancestors, &String::from_utf8_lossy(parent.value())))
    }
    fn resolve_name(&self, name: &str) -> String {
        resolve_written_class_name(self.ancestors, name)
    }
    fn in_namespace(&self) -> bool {
        self.ancestors.iter().any(|a| matches!(a, Node::Namespace(_)))
    }
}

/// PHPStan's type of a parsed doc type written at the node `ancestors` lead to (`TypeNodeResolver`): names resolved
/// through the namespace and `use` imports, `self`, `static` and `$this` the class `class`, which the resolver knows.
pub fn doc_type(parsed: &DocNode, ancestors: &[Node<'_, '_>], class: Option<&str>) -> Type {
    let own = class.map(|name| Arc::new(own_class(name)));
    let provider = |name: &str| own.clone().filter(|c| c.name.eq_ignore_ascii_case(name));
    with_classes(&provider, || resolve_in(parsed, &NameScope { ancestors, class }))
}

/// The class being declared, as the resolver sees it: its name and nothing else.
fn own_class(name: &str) -> Class {
    Class {
        name: name.to_string(),
        ancestors: vec![],
        templates: vec![],
        cases: vec![],
        case_values: vec![],
        is_interface: false,
        is_trait: false,
        is_enum: false,
        is_final: false,
        properties: vec![],
        constants: vec![],
        methods: HashMap::new(),
        iterable: None,
        generics: HashMap::new(),
        property_docs: HashMap::new(),
    }
}

/// PHPStan's type for Mago's `t`. `None` for a type Pun does not map.
pub fn phpstan_type(t: &TUnion) -> Option<Type> {
    let types: Vec<Type> = t.types.iter().map(phpstan_atomic).collect::<Option<_>>()?;
    // (Mago's unions are never empty)
    Some(match types.len() {
        1 => types.into_iter().next().expect("one type"),
        _ => union(types),
    })
}

fn phpstan_atomic(atomic: &TAtomic) -> Option<Type> {
    Some(match atomic {
        TAtomic::Null => Type::Null,
        TAtomic::Never => Type::Never(NeverKind::Implicit),
        // PHPStan's type of a call to a `void` function
        TAtomic::Void => Type::Null,
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
            Some(v) => Type::ConstString(v.to_vec()),
            None => Type::String,
        },
        TAtomic::Scalar(TScalar::ClassLikeString(c)) => Type::ClassString(match c {
            TClassLikeString::Literal { value } => Some(Box::new(Type::Object(value.to_string(), vec![], false))),
            TClassLikeString::OfType { constraint, .. }
                if let TAtomic::Object(TObject::Named(n)) = &**constraint
                    && n.type_parameters.is_none() =>
            {
                Some(Box::new(Type::Object(n.name.to_string(), vec![], false)))
            }
            _ => None,
        }),
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
                    // (a constant's key, not yet resolved, is neither)
                    let key = match key.get_integer() {
                        Some(i) => Type::ConstInt(i),
                        None => Type::ConstString(key.get_string()?.to_vec()),
                    };
                    b.set(Some(key), phpstan_type(value)?, *optional);
                }
                b.get()
            } else {
                let (key, value) = match &k.parameters {
                    Some((key, value)) => (phpstan_type(key)?, phpstan_type(value)?),
                    None => return Some(Type::ConstArray(Box::new(pun_phpstan::types::ConstArray::empty()))),
                };
                Type::array(key, value)
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
                // as Rector runs PHPStan, a list is a plain `array<int<0, max>, T>`
                Type::array(Type::int_range(Some(0), None), phpstan_type(&l.element_type)?)
            }
        }
        TAtomic::Object(TObject::Named(n)) if n.intersection_types.is_none() && n.is_this => {
            Type::This(n.name.to_string())
        }
        TAtomic::Object(TObject::Named(n)) if n.intersection_types.is_none() => {
            let args = match &n.type_parameters {
                Some(params) => params.iter().map(phpstan_type).collect::<Option<_>>()?,
                None => vec![],
            };
            Type::Object(n.name.to_string(), args, false)
        }
        TAtomic::Object(TObject::Enum(e)) => match &e.case {
            Some(case) => Type::EnumCase(e.name.to_string(), case.to_string()),
            None => Type::Object(e.name.to_string(), vec![], false),
        },
        TAtomic::Object(TObject::Any) => Type::ObjectWithoutClass,
        TAtomic::Resource(_) => Type::Resource,
        // a template type, printed by its name
        TAtomic::GenericParameter(g) if g.intersection_types.is_none() => Type::Template(Box::new(Template {
            name: g.parameter_name.to_string(),
            scope: String::new(),
            bound: phpstan_type(&g.constraint)?,
            default: None,
            argument: false,
        })),
        TAtomic::Iterable(i) if i.intersection_types.is_none() => Type::Iterable(
            Box::new(phpstan_type(&i.key_type)?),
            Box::new(phpstan_type(&i.value_type)?),
        ),
        _ => return None,
    })
}

/// `getType()` of `expr`: an array literal as PHPStan's `ConstantArrayTypeBuilder` builds it (its items in the
/// order written), a literal or `Name::class` from the syntax, anything else Mago's type with the docblocks read.
pub fn expr_type(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> Option<Type> {
    let expr = unparenthesize(expr);
    match expr {
        // (Mago's type where a key is no constant)
        Expression::Array(a) => array_type(ctx, ancestors, a.elements.iter())
            .or_else(|| facts(ctx).full_type(node_span(expr)).and_then(phpstan_type)),
        Expression::LegacyArray(a) => array_type(ctx, ancestors, a.elements.iter())
            .or_else(|| facts(ctx).full_type(node_span(expr)).and_then(phpstan_type)),
        Expression::Literal(Literal::String(s)) => {
            s.value.map(|v| Type::ConstString(v.to_vec()))
        }
        Expression::Literal(Literal::Integer(i)) => match i.value {
            Some(v) if v <= i64::MAX as u64 => Some(Type::ConstInt(v as i64)),
            // past `PHP_INT_MAX` a decimal integer is a float
            _ => String::from_utf8_lossy(i.raw).replace('_', "").parse::<f64>().ok().map(Type::ConstFloat),
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
                    false,
                ))))),
                _ => facts(ctx).full_type(node_span(expr)).and_then(phpstan_type),
            }
        }
        // PHPStan's superglobals are `array<mixed>`
        Expression::ArrayAccess(a) if is_superglobal(a.array) => Some(Type::Mixed { explicit: true }),
        // `union(removeNull($left), $right)` with a `mixed` left is `mixed`
        Expression::Binary(b)
            if b.operator.is_null_coalesce()
                && expr_type(ctx, ancestors, b.lhs).is_some_and(|t| type_mapper::is_mixed(&t)) =>
        {
            Some(Type::Mixed { explicit: true })
        }
        // `GetClassDynamicReturnTypeExtension`: the class string of an object's class
        Expression::Call(Call::Function(c))
            if let Expression::Identifier(id) = c.function
                && String::from_utf8_lossy(id.value()).trim_start_matches('\\').eq_ignore_ascii_case("get_class")
                && let [Argument::Positional(arg)] = &c.argument_list.arguments.nodes[..]
                && let Some(Type::Object(name, _, _)) = expr_type(ctx, ancestors, arg.value) =>
        {
            Some(Type::ClassString(Some(Box::new(Type::Object(name, vec![], false)))))
        }
        // a function PHPStan does not know gives an `ErrorType`
        Expression::Call(Call::Function(c))
            if matches!(c.function, Expression::Identifier(_))
                && callee(&facts(ctx), Target::Function(c.function), ancestors).is_none() =>
        {
            Some(Type::Error)
        }
        // the class of `new Name()`, known or not
        Expression::Instantiation(i) => {
            let t = facts(ctx).full_type(node_span(expr)).and_then(phpstan_type);
            match i.class {
                Expression::Identifier(id) if t.as_ref().is_none_or(|t| matches!(t, Type::Never(_))) => Some(
                    Type::Object(resolve_written_class_name(ancestors, &String::from_utf8_lossy(id.value())), vec![], false),
                ),
                _ => t,
            }
        }
        Expression::Call(Call::Method(c)) if is_this(c.object) => this_method_return_type(ctx, ancestors, c)
            .or_else(|| facts(ctx).full_type(node_span(expr)).and_then(phpstan_type)),
        Expression::Conditional(_) => {
            let facts = facts(ctx);
            ordered_type(expression_order(expr, |e| facts.full_type(node_span(e)))?)
        }
        _ => facts(ctx).full_type(node_span(expr)).and_then(phpstan_type),
    }
}

/// [`phpstan_type`] of a ternary's members in the order [`expression_order`] gives them (Mago's own order is not
/// PHPStan's).
fn ordered_type(types: Vec<&TAtomic>) -> Option<Type> {
    phpstan_type(&TUnion::from_vec(types.into_iter().cloned().collect()))
}

/// Whether `expr` is one of PHP's superglobal variables.
fn is_superglobal(expr: &Expression<'_>) -> bool {
    matches!(
        expr,
        Expression::Variable(Variable::Direct(v)) if matches!(
            v.name,
            b"$GLOBALS" | b"$_SERVER" | b"$_GET" | b"$_POST" | b"$_FILES" | b"$_COOKIE" | b"$_SESSION"
                | b"$_REQUEST" | b"$_ENV"
        )
    )
}

/// The return type PHPStan gives `$this->method()` of a method of the class declared `array`, `?array` or not at all
/// with a `@return` tag: the tag's type where it narrows the declared one (`TypehintHelper::decideType()`), Mago's
/// own order of a union lost otherwise.
fn this_method_return_type(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], call: &MethodCall<'_>) -> Option<Type> {
    let ClassLikeMemberSelector::Identifier(name) = &call.method else {
        return None;
    };
    let (members, _, _) = ancestors.iter().rev().find_map(|a| class_members(*a))?;
    let m = method(members, &String::from_utf8_lossy(name.value))?;
    let info = DocTarget::of(ctx.source, node_span(m).start).info(ctx.source);
    let tag = info.return_tag()?;
    let doc = doc_type(tag.parsed.as_ref()?, ancestors, class_name(ancestors).as_deref());
    let array = || Type::array(Type::Mixed { explicit: false }, Type::Mixed { explicit: false });
    let native = match m.return_type_hint.as_ref().map(|r| &r.hint) {
        None => return Some(doc),
        Some(Hint::Array(_)) => array(),
        Some(Hint::Nullable(n)) if matches!(n.hint, Hint::Array(_)) => union(vec![array(), Type::Null]),
        // an `iterable` declared one takes the tag's array key and value types
        Some(Hint::Iterable(_)) if let Type::Array(key, item) = doc => return Some(Type::Iterable(key, item)),
        Some(_) => return None,
    };
    Some(if is_super_type_of(&native, &doc).yes() { doc } else { native })
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
                    Type::ConstString(s) => match std::str::from_utf8(&s).map(str::parse::<i64>) {
                        // a decimal integer string key is an integer key (`toArrayKey()`)
                        Ok(Ok(i)) if i.to_string().as_bytes() == s => Type::ConstInt(i),
                        _ => Type::ConstString(s),
                    },
                    key @ Type::ConstInt(_) => key,
                    _ => return None,
                };
                b.set(Some(key), expr_type(ctx, ancestors, kv.value)?, false);
            }
            ArrayElement::Value(v) => b.set(None, expr_type(ctx, ancestors, v.value)?, false),
            // a constant array unpacked: its integer keys renumbered, its string keys kept
            // (an empty element is only valid in a destructuring)
            other => {
                let ArrayElement::Variadic(v) = other else { return None };
                let Type::ConstArray(spread) = expr_type(ctx, ancestors, v.value)? else {
                    return None;
                };
                for (key, value) in spread.keys.iter().zip(&spread.values) {
                    let key = matches!(key, Type::ConstString(_)).then(|| key.clone());
                    b.set(key, value.clone(), false);
                }
            }
        }
    }
    Some(b.get())
}

/// `NodeDocblockTypeDecorator::decorateGenericIterableReturnType()` of a PHPStan type.
pub fn decorate_return(info: &mut PhpDocInfo, t: &Type) -> bool {
    decorate_return_node(info, type_mapper::create_type_node(t), t)
}

/// `decorateGenericIterableReturnType()` of a type node; `t` is its PHPStan type. Not for bare `mixed`, nor for a
/// node with no value iterable type.
pub fn decorate_return_node(info: &mut PhpDocInfo, node: TypeNode, t: &Type) -> bool {
    if type_mapper::is_bare_mixed_type(t) || matches!(node, TypeNode::Identifier(_)) {
        return false;
    }
    info.set_return_type(node);
    true
}

/// `decorateGenericIterableVarType()`.
pub fn decorate_var(info: &mut PhpDocInfo, t: &Type) -> bool {
    let node = type_mapper::create_type_node(t);
    if type_mapper::is_bare_mixed_type(t) || matches!(node, TypeNode::Identifier(_)) {
        return false;
    }
    info.set_var_type(node);
    true
}

/// `decorateGenericIterableParamType()` of the parameter `$name`.
pub fn decorate_param(info: &mut PhpDocInfo, t: &Type, name: &str, is_variadic: bool) -> bool {
    if type_mapper::is_bare_mixed_type(t) {
        return false;
    }
    let node = type_mapper::create_type_node(t);
    if matches!(node, TypeNode::Identifier(_)) {
        return false;
    }
    if info.param_tag(name).is_some_and(|tag| tag.ty.to_string() == node.to_string()) {
        return false;
    }
    info.set_param_type(name, is_variadic, node);
    true
}

/// `PhpDocTypeChanger::changeReturnType()` of an `array<K, V>` type over a missing or not useful `@return` tag.
/// Its guards never refuse such a change: `hasInvalidTag()` is never true, as Rector's `DoctrineAnnotationDecorator`
/// turns every invalid tag into a generic one; the type is not a union (`isLegal()`); and it is not the tag's type
/// (`areTypesEqual()`), which would be useful.
pub fn change_return_type(info: &mut PhpDocInfo, t: &Type) {
    info.set_return_type(type_mapper::map_to_phpdoc(t));
}

/// `PhpDocTypeChanger::changeParamType()` of an `array<K, V>` type over a parameter without a `@param` tag (see
/// [`change_return_type`] for its guards).
pub fn change_param_type(info: &mut PhpDocInfo, t: &Type, name: &str, is_variadic: bool) {
    info.set_param_type(name, is_variadic, type_mapper::map_to_phpdoc(t));
}

/// `NodeTypeResolver::getNativeType()` of `expr`: a literal from the syntax, anything else Mago's native type.
pub fn native_expr_type(ctx: &Context<'_>, expr: &Expression<'_>) -> Option<Type> {
    let expr = unparenthesize(expr);
    match expr {
        Expression::Literal(_) => expr_type(ctx, &[], expr),
        Expression::Conditional(_) => {
            let facts = facts(ctx);
            ordered_type(expression_order(expr, |e| facts.native_type(node_span(e)))?)
        }
        _ => facts(ctx).native_type(node_span(expr)).and_then(phpstan_type),
    }
}
