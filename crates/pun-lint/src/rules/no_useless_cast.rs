//! `lint/dead-code/no-useless-cast`, ported from Rector 2.6.1's `RecastingRemovalRector` (rules/DeadCode/Rector/Cast).
//!
//! A cast to the type its value already has goes: `(string)`, `(bool)`, `(int)`, `(float)`, `(object)` or `(array)`
//! of a value whose native type is that type (PHPStan's `StringType`, `BooleanType`, `IntegerType`, `FloatType`,
//! `ObjectType` or `ArrayType`, their constant forms included), and `(array)` of a constant array. It stays on an
//! array item, on a property without a native type (or one PHPStan has no reflection of, such as a magic one), and on
//! a variable whose native type is wider than its type, that may be undefined, or that another variable's constant
//! string names.
//!
//! Mago's types stand in for PHPStan's native ones, which refine a string only by its case (`strtolower()` gives an
//! intersection type, no `StringType`) and an array not at all; `$a ** $b` of integers is `int|float` unless both are
//! literals, and `$this` is PHPStan's `ThisType`, which is no `ObjectType`. An enum's `name` and `value` are no
//! native properties.
//!
//! The value keeps its code. Rector prints it without the parentheses the cast's operand had; Pun keeps the ones the
//! meaning needs (lint.md: meaning over bytes).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::string::TStringCasing;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Access, BinaryOperator, ClassLikeMemberSelector, Expression, Literal, Node, NodeKind,
    UnaryPrefixOperator, Variable,
};
use pun_problem::{Edit, Fix, Safety};

use super::class_method::{object_class_names, qualified, walk};
use super::php_parser::{keeps_parentheses, parent_of};
use super::rector_batch5::resolve_class_name;
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessCast;

/// The PHPStan type class a native type is an instance of, for the casts.
#[derive(PartialEq, Clone, Copy)]
enum Kind {
    String,
    Bool,
    Int,
    Float,
    Object,
    Array,
    ConstantArray,
    Mixed,
    Other,
}

impl Rule for NoUselessCast {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-cast"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::UnaryPrefix])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::UnaryPrefix(cast) = node else {
            return;
        };
        let wanted = match cast.operator {
            UnaryPrefixOperator::StringCast(..) | UnaryPrefixOperator::BinaryCast(..) => {
                Kind::String
            }
            UnaryPrefixOperator::BoolCast(..) | UnaryPrefixOperator::BooleanCast(..) => Kind::Bool,
            UnaryPrefixOperator::ArrayCast(..) => Kind::Array,
            UnaryPrefixOperator::IntCast(..) | UnaryPrefixOperator::IntegerCast(..) => Kind::Int,
            UnaryPrefixOperator::ObjectCast(..) => Kind::Object,
            UnaryPrefixOperator::DoubleCast(..)
            | UnaryPrefixOperator::RealCast(..)
            | UnaryPrefixOperator::FloatCast(..) => Kind::Float,
            _ => return,
        };
        let expr = unparenthesize(cast.operand);
        let facts = facts(ctx);
        // a constant (other than `true`, `false` and `null`) has no native type
        if matches!(expr, Expression::ConstantAccess(_)) {
            return;
        }
        let Some(native) = facts.native_type(node_span(expr)) else {
            return;
        };
        let mut kind = kind(native);
        // `$a ** $b` of integers is `int|float` for PHPStan unless both are known
        if let Expression::Binary(b) = expr
            && matches!(b.operator, BinaryOperator::Exponentiation(_))
            && kind == Kind::Int
            && !(is_int_literal(b.lhs) && is_int_literal(b.rhs))
        {
            kind = Kind::Other;
        }
        let same = kind == wanted || (kind == Kind::ConstantArray && wanted == Kind::Array);
        if !same || should_skip(&facts, ancestors, expr, native) {
            return;
        }
        let span = node_span(expr);
        let text = &ctx.source[span.start..span.end];
        let text = if keeps_parentheses(expr, parent_of(ancestors)) {
            format!("({text})")
        } else {
            text.to_string()
        };
        let span = node_span(cast);
        out.push(Report {
            span,
            message: "This cast does not change the value's type; remove it.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, text)],
            }),
        });
    }
}

fn is_int_literal(expr: &Expression<'_>) -> bool {
    matches!(
        unparenthesize(expr),
        Expression::Literal(Literal::Integer(_))
    )
}

/// The PHPStan class of the type Mago's `t` stands for.
fn kind(t: &TUnion) -> Kind {
    match t.types.as_ref() {
        [atomic] => atomic_kind(atomic),
        [a, b] if is_bool(a) && is_bool(b) => Kind::Bool,
        // Mago's union of arrays is one `ArrayType` for PHPStan
        types if types.iter().all(|a| matches!(a, TAtomic::Array(_))) => Kind::Array,
        _ => Kind::Other,
    }
}

fn is_bool(atomic: &TAtomic) -> bool {
    matches!(atomic, TAtomic::Scalar(TScalar::Bool(_)))
}

fn atomic_kind(atomic: &TAtomic) -> Kind {
    match atomic {
        TAtomic::Mixed(_) => Kind::Mixed,
        // PHPStan's native types refine a string only by its case (`strtolower()`, `strtoupper()`)
        TAtomic::Scalar(TScalar::String(s))
            if s.literal.is_none() && s.casing != TStringCasing::Unspecified =>
        {
            Kind::Other
        }
        TAtomic::Scalar(TScalar::String(_)) => Kind::String,
        TAtomic::Scalar(TScalar::ClassLikeString(_)) => Kind::String,
        TAtomic::Scalar(TScalar::Integer(_)) => Kind::Int,
        TAtomic::Scalar(TScalar::Float(_)) => Kind::Float,
        TAtomic::Scalar(TScalar::Bool(_)) => Kind::Bool,
        // nor an array: a list or a non-empty array is a plain `ArrayType` there
        TAtomic::Array(TArray::Keyed(k)) if k.parameters.is_none() => Kind::ConstantArray,
        TAtomic::Array(_) => Kind::Array,
        TAtomic::Object(TObject::Named(n)) if !n.is_this => Kind::Object,
        TAtomic::Object(TObject::Enum(_)) => Kind::Object,
        _ => Kind::Other,
    }
}

/// `shouldSkip`: an array item, a property PHPStan has no native type of, or `isNonTypedFromParam`.
fn should_skip(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    native: &TUnion,
) -> bool {
    match expr {
        Expression::ArrayAccess(_) => true,
        Expression::Access(Access::Property(p)) => {
            let ClassLikeMemberSelector::Identifier(id) = &p.property else {
                return true;
            };
            let class = match unparenthesize(p.object) {
                Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => {
                    this_class(facts, ancestors)
                }
                object => {
                    let span = node_span(object);
                    let t = facts.full_type(span).or_else(|| facts.native_type(span));
                    match t.map(object_class_names) {
                        Some(names) if names.len() == 1 => Some(names[0].clone()),
                        _ => None,
                    }
                }
            };
            let Some(class) = class else {
                return true;
            };
            !has_native_type(
                facts,
                &class,
                &format!("${}", String::from_utf8_lossy(id.value)),
            )
        }
        Expression::Access(Access::StaticProperty(p)) => {
            let Variable::Direct(v) = &p.property else {
                return true;
            };
            let class = match p.class {
                Expression::Identifier(id) => Some(resolve_class_name(ancestors, id)),
                Expression::Self_(_) | Expression::Static(_) => this_class(facts, ancestors),
                Expression::Parent(_) => this_class(facts, ancestors).and_then(|c| {
                    facts
                        .codebase
                        .get_class_like(c.as_bytes())
                        .and_then(|m| m.direct_parent_class.map(|p| p.to_string()))
                }),
                class => match facts.full_type(node_span(class)).map(object_class_names) {
                    Some(names) if names.len() == 1 => Some(names[0].clone()),
                    _ => None,
                },
            };
            let Some(class) = class else {
                return true;
            };
            !has_native_type(facts, &class, &String::from_utf8_lossy(v.name))
        }
        Expression::Variable(Variable::Direct(_)) => {
            // `hasVariableType()` must be a yes, and the type within the native type
            native.possibly_undefined()
                || facts
                    .full_type(node_span(expr))
                    .is_some_and(|full| !within(full, native))
                || names_variable(ancestors, node_span(expr).start)
        }
        _ => false,
    }
}

/// The defined variables loop of `isNonTypedFromParam`: a variable of the function-like around the cast (before
/// `at`) holds a literal string that is the name of one of its variables, which a variable variable may write.
fn names_variable(ancestors: &[Node<'_, '_>], at: usize) -> bool {
    let Some(root) = ancestors.iter().rev().find(|a| {
        matches!(
            a,
            Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_)
        )
    }) else {
        return false;
    };
    let mut names: Vec<&[u8]> = Vec::new();
    let mut values: Vec<Vec<u8>> = Vec::new();
    walk(*root, &mut |node, path| {
        if !path.is_empty()
            && matches!(
                node,
                Node::Function(_) | Node::Closure(_) | Node::ArrowFunction(_) | Node::Class(_)
            )
        {
            return false;
        }
        match node {
            Node::DirectVariable(v) if v.span.start.offset as usize <= at => {
                names.push(&v.name[1..]);
            }
            Node::Assignment(a) if node_span(a).start < at => {
                if let (
                    Expression::Variable(Variable::Direct(_)),
                    Expression::Literal(Literal::String(s)),
                ) = (a.lhs, unparenthesize(a.rhs))
                    && let Some(value) = s.value
                {
                    values.push(value.to_vec());
                }
            }
            _ => {}
        }
        true
    });
    values.iter().any(|v| names.contains(&v.as_slice()))
}

/// The class `$this` and `self` stand for: the enclosing class (Mago's name for an anonymous one); none in a trait,
/// whose `$this` PHPStan types as the class using it.
fn this_class(facts: &Facts, ancestors: &[Node<'_, '_>]) -> Option<String> {
    let index = ancestors.iter().rposition(|a| {
        matches!(
            a,
            Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Trait(_)
                | Node::Enum(_)
                | Node::Interface(_)
        )
    })?;
    match ancestors[index] {
        Node::Class(c) => Some(qualified(&ancestors[..index], c.name.value)),
        Node::Enum(e) => Some(qualified(&ancestors[..index], e.name.value)),
        Node::AnonymousClass(c) => Some(facts.anonymous_class_name(mago_span::HasSpan::span(c))),
        _ => None,
    }
}

/// `hasNativeProperty()` and a native type other than `mixed` on the property `name` (with its `$`) of `class`.
fn has_native_type(facts: &Facts, class: &str, name: &str) -> bool {
    // an enum's `name` and `value` are no native properties for PHPStan
    if facts
        .codebase
        .get_class_like(class.as_bytes())
        .is_some_and(|c| c.kind.is_enum())
    {
        return false;
    }
    facts
        .codebase
        .get_declaring_property(class.as_bytes(), name.as_bytes())
        .and_then(|p| p.type_declaration_metadata.as_ref())
        .is_some_and(|t| !t.type_union.is_mixed())
}

/// `$nativeType->isSuperTypeOf($type)->yes()`, for the native types the casts keep: every part of `full` is of the
/// native type's kind.
fn within(full: &TUnion, native: &TUnion) -> bool {
    let wanted = kind(native);
    full.types.iter().all(|a| {
        let k = atomic_kind(a);
        k == wanted
            || (wanted == Kind::ConstantArray && matches!(a, TAtomic::Array(_)))
            || (wanted == Kind::Bool && is_bool(a))
            || (wanted == Kind::String && matches!(a, TAtomic::Scalar(TScalar::String(_))))
            || (wanted == Kind::Array && matches!(a, TAtomic::Array(_)))
    })
}
