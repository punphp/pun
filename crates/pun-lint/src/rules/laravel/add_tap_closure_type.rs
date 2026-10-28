//! `lint/laravel/add-tap-closure-type`, ported from rector-laravel 2.5.0's `TypeHintTappableCallRector`
//! (src/Rector/FuncCall).
//!
//! `$x->tap(function ($y) {...})` on an object using `Illuminate\Support\Traits\Tappable`, and
//! `tap($value, function ($y) {...})` (the function `tap` as written, any case, `\` or not), give the closure's first
//! parameter the type of `$x` or `$value` as Rector declares a parameter type
//! (`StaticTypeMapper::mapPHPStanTypeToPhpParserNode(..., TypeKind::PARAM)`): classes fully qualified, `$this` and
//! `static` as `self`, constants as their general type (`true` and `false` stay), a closure as `\Closure`. A parameter
//! whose type PHPStan holds equal (`TypeComparator::areTypesEqual`) stays; a type with no declaration
//! takes the parameter's type away (`null` alone). Only a `function` counts, not an arrow function.
//!
//! The type of `new` with a class name is Rector's own (`NewTypeResolver`): the name as written, and none for
//! `new static`.
//!
//! Pun leaves the call alone where Mago does not tell what PHPStan would: a `mixed` value (PHPStan writes `mixed` for
//! a declared one and takes the type away for an inferred one; Mago has no such difference), an `object` value (Mago's
//! type of a conditional return type PHPStan resolves to `mixed`), a `never` value, a type Pun does not declare, a
//! class the codebase does not know, a hint naming the value's class when that class is generic (whether PHPStan's
//! type has generics decides the comparison), an intersection hint, and another case of a keyword hint that is not
//! `int`, `float`, `string` or `bool` (PHPStan may hold it equal).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{Argument, Closure, Expression, Hint, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::names::resolve_class_name;
use super::object_type::is_object_type;
use super::print::set_param_type;
use super::{is_name, member_name};
use crate::rules::rector_code_quality::named;
use crate::rules::types::class_scope;
use crate::rules::types::declare::{return_declaration, union_order};
use crate::rules::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const TAPPABLE: &str = "Illuminate\\Support\\Traits\\Tappable";

pub struct AddTapClosureType;

impl Rule for AddTapClosureType {
    fn id(&self) -> &'static str {
        "lint/laravel/add-tap-closure-type"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (closure, value) = match node {
            Node::MethodCall(call) => {
                let args: Vec<&Expression<'_>> = call.argument_list.arguments.iter().map(argument_value).collect();
                if !member_name(&call.method).is_some_and(|n| is_name(n, "tap"))
                    || args.is_empty()
                    || !is_object_type(ctx, ancestors, call.object, TAPPABLE)
                {
                    return;
                }
                (args[0], call.object)
            }
            Node::FunctionCall(call) => {
                let args: Vec<&Expression<'_>> = call.argument_list.arguments.iter().map(argument_value).collect();
                if !named(call, "tap") || args.len() < 2 {
                    return;
                }
                (args[1], args[0])
            }
            _ => return,
        };
        let Expression::Closure(closure) = closure else {
            return;
        };
        if let Some(fix) = refactor_parameter(ctx, ancestors, closure, value) {
            out.push(Report {
                span: node_span(closure.parameter_list.parameters.first().unwrap()),
                message: "Type the tap closure's parameter with the tapped value's type.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Unsafe,
                    edits: vec![fix],
                }),
            });
        }
    }
}

/// `Arg::$value`, unpacked or named.
fn argument_value<'a>(argument: &'a Argument<'a>) -> &'a Expression<'a> {
    match argument {
        Argument::Positional(a) => a.value,
        Argument::Named(a) => a.value,
    }
}

/// `refactorParameter`: the edit that gives the closure's first parameter the type of `value`, `None` for no change.
fn refactor_parameter(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    closure: &Closure<'_>,
    value: &Expression<'_>,
) -> Option<Edit> {
    let param = closure.parameter_list.parameters.first()?;
    let facts = facts(ctx);
    let own = class_scope(ancestors).and_then(|s| s.name);
    if let Some((declared, values)) = new_type(ancestors, &facts, value) {
        return parameter_edit(ctx, ancestors, param, own.as_deref(), declared, &values);
    }
    let t = facts.full_type(node_span(unparenthesize(value)))?;
    // a `mixed`, a class the codebase does not know, `object` (Mago's type of a conditional return type PHPStan
    // resolves to `mixed`, a template's bound) and `never` (Mago's type of code it finds wrong)
    let unknown = |a: &TAtomic| match a {
        TAtomic::Object(TObject::Named(n)) => facts.codebase.get_class_like(n.name.as_bytes()).is_none(),
        TAtomic::Mixed(_) | TAtomic::Reference(_) | TAtomic::Object(TObject::Any) | TAtomic::Never => true,
        _ => false,
    };
    if t.types.iter().any(unknown) {
        return None;
    }
    let declared = declaration(&facts, &t.types, ctx.php_version)?;
    let values = value_members(&facts, &t.types, own.as_deref());
    parameter_edit(ctx, ancestors, param, own.as_deref(), declared, &values)
}

/// `NewTypeResolver`: the type of `new` with a class name is that name's object as written (resolved), `new static`
/// a class `static` that has no declaration, `new self` and `new parent` the class's and its parent's. `None` for
/// any other value, or a class the codebase does not know.
fn new_type(
    ancestors: &[Node<'_, '_>],
    facts: &Facts,
    value: &Expression<'_>,
) -> Option<(Option<String>, Vec<Member>)> {
    let Expression::Instantiation(new) = unparenthesize(value) else {
        return None;
    };
    let name = match new.class {
        Expression::Static(_) => {
            let values = vec![Member::Class { name: "static".to_string(), generic: false }];
            return Some((None, values));
        }
        Expression::Identifier(id) => resolve_class_name(ancestors, id),
        Expression::Self_(_) => class_scope(ancestors)?.name?,
        Expression::Parent(_) => class_scope(ancestors)?.parent_class?,
        _ => return None,
    };
    facts.codebase.get_class_like(name.as_bytes())?;
    Some((Some(format!("\\{name}")), vec![Member::Class { name, generic: false }]))
}

/// The edit that gives `param` the declaration `declared` of a value whose type has `values`, as `refactorParameter`
/// does: none when the parameter's type is equal, the type taken away when there is no declaration.
fn parameter_edit(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    param: &mago_syntax::cst::FunctionLikeParameter<'_>,
    own: Option<&str>,
    declared: Option<String>,
    values: &[Member],
) -> Option<Edit> {
    if let Some(hint) = &param.hint {
        let members = hint_members(ancestors, hint, own)?;
        if equal(&members, values)? {
            return None;
        }
        let text = &ctx.source[node_span(hint).start..node_span(hint).end];
        if let Some(declared) = &declared {
            // the same text: nothing changes; another case of a type Pun does not compare: equal or not is unknown
            if text == declared
                || (text.eq_ignore_ascii_case(declared) && members.iter().any(|m| *m == Member::Other))
            {
                return None;
            }
        }
    }
    match (declared, &param.hint) {
        (Some(declared), _) => Some(set_param_type(param, &declared)),
        // the type taken away, with the space after it
        (None, Some(hint)) => {
            let span = node_span(hint);
            let rest = &ctx.source[span.end..];
            let end = span.end + rest.len() - rest.trim_start().len();
            Some(Edit::new(Span::new(span.start, end), ""))
        }
        (None, None) => None,
    }
}

/// The parameter declaration of a type: `Some(None)` where Rector has none (`null`, `never`, `void` alone),
/// `None` where Pun does not know Rector's.
fn declaration(facts: &Facts, types: &[TAtomic], php_version: u32) -> Option<Option<String>> {
    if types.iter().all(|a| matches!(a, TAtomic::Null | TAtomic::Never | TAtomic::Void)) {
        return Some(None);
    }
    let declared = return_declaration(union_order(types), &facts.codebase, php_version, &[], true)?;
    // `StaticTypeMapper`: `$this` and `static` are `self` but in a return type
    let self_named = |m: &str| match m {
        "static" => "self".to_string(),
        "?static" => "?self".to_string(),
        m => m.to_string(),
    };
    Some(Some(declared.split('|').map(self_named).collect::<Vec<_>>().join("|")))
}

/// A part of a type as `TypeComparator` compares it.
#[derive(Debug, PartialEq)]
enum Member {
    /// An object of a class, by its name; `generic` when the class has template types.
    Class { name: String, generic: bool },
    /// `null`.
    Null,
    /// A general scalar (`int`, `float`, `string`, `bool`).
    Scalar(&'static str),
    /// `true` or `false`.
    Bool(bool),
    /// A float's value.
    Float,
    /// Anything else: equal to nothing.
    Other,
}

/// The members of a parameter's hint (`?T` is `T|null`); `None` for an intersection.
fn hint_members(ancestors: &[Node<'_, '_>], hint: &Hint<'_>, own: Option<&str>) -> Option<Vec<Member>> {
    let mut out = Vec::new();
    let mut stack = vec![hint];
    while let Some(h) = stack.pop() {
        let class = |name: String| Member::Class { name, generic: false };
        out.push(match h {
            Hint::Parenthesized(p) => {
                stack.push(&p.hint);
                continue;
            }
            Hint::Nullable(n) => {
                stack.push(&n.hint);
                Member::Null
            }
            Hint::Union(u) => {
                stack.push(&u.right);
                stack.push(&u.left);
                continue;
            }
            Hint::Intersection(_) => return None,
            Hint::Identifier(id) => class(resolve_class_name(ancestors, id)),
            Hint::Self_(_) | Hint::Static(_) => class(own?.to_string()),
            Hint::Parent(_) => class(class_scope(ancestors)?.parent_class?),
            Hint::Null(_) => Member::Null,
            Hint::True(_) => Member::Bool(true),
            Hint::False(_) => Member::Bool(false),
            Hint::Integer(_) => Member::Scalar("int"),
            Hint::Float(_) => Member::Scalar("float"),
            Hint::String(_) => Member::Scalar("string"),
            Hint::Bool(_) => Member::Scalar("bool"),
            _ => Member::Other,
        });
    }
    Some(out)
}

/// The members of a value's type.
fn value_members(facts: &Facts, types: &[TAtomic], own: Option<&str>) -> Vec<Member> {
    let class = |name: &str| {
        let known = facts.codebase.get_class_like(name.as_bytes());
        Member::Class {
            name: known.map_or_else(|| name.to_string(), |c| c.original_name.to_string()),
            generic: known.is_some_and(|c| !c.template_types.is_empty()),
        }
    };
    types
        .iter()
        .map(|a| match a {
            TAtomic::Null => Member::Null,
            TAtomic::Object(TObject::Named(n)) if n.intersection_types.is_none() => {
                match (n.is_this || n.is_static, own) {
                    (true, Some(own)) => Member::Class { name: own.to_string(), generic: false },
                    _ => class(&n.name.to_string()),
                }
            }
            TAtomic::Scalar(TScalar::Integer(i)) if i.is_unspecified() => Member::Scalar("int"),
            TAtomic::Scalar(TScalar::String(s)) if s.is_boring() => Member::Scalar("string"),
            TAtomic::Scalar(TScalar::Float(f)) if !f.is_literal() => Member::Scalar("float"),
            TAtomic::Scalar(TScalar::Float(_)) => Member::Float,
            TAtomic::Scalar(TScalar::Bool(b)) if b.is_true() => Member::Bool(true),
            TAtomic::Scalar(TScalar::Bool(b)) if b.is_false() => Member::Bool(false),
            TAtomic::Scalar(TScalar::Bool(_)) => Member::Scalar("bool"),
            _ => Member::Other,
        })
        .collect()
}

/// `TypeComparator::areTypesEqual` of a hint's type and a value's: one type each by `ScalarTypeComparator` (every
/// boolean equal, every float, an `int` or `string` only to the general one) or by class name; a union by its members.
/// `None` where the class names match but the class is generic.
fn equal(hint: &[Member], value: &[Member]) -> Option<bool> {
    if let ([h], [v]) = (hint, value) {
        let boolean = |m: &Member| matches!(m, Member::Scalar("bool") | Member::Bool(_));
        let float = |m: &Member| matches!(m, Member::Scalar("float") | Member::Float);
        if (boolean(h) && boolean(v)) || (float(h) && float(v)) {
            return Some(true);
        }
    }
    let same = |h: &Member, v: &Member| match (h, v) {
        (Member::Class { name: a, .. }, Member::Class { name: b, generic }) if a == b => {
            if *generic { None } else { Some(true) }
        }
        (Member::Other, _) | (_, Member::Other) => Some(false),
        _ => Some(h == v),
    };
    if hint.len() != value.len() {
        return Some(false);
    }
    for h in hint {
        let mut found = false;
        for v in value {
            if same(h, v)? {
                found = true;
            }
        }
        if !found {
            return Some(false);
        }
    }
    Some(true)
}
