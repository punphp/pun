//! `lint/laravel/prefer-latest-oldest`, ported from rector-laravel 2.7.3's `EloquentOrderByToLatestOrOldestRector`
//! (src/Rector/MethodCall).
//!
//! On a query builder (`Illuminate\Contracts\Database\Query\Builder`), `orderBy($column)` and
//! `orderBy($column, 'asc')` become `oldest($column)`, and `orderBy($column, <any other string>)` and
//! `orderByDesc($column)` become `latest($column)`, for a column whose value matches one of the `allowed-patterns`
//! (`fnmatch`, default `*_at`, `*_date` and `*_on`; an empty list allows every column), or a variable whose name
//! does. The column `'created_at'` is dropped, since it is the default. Values are read as Rector's
//! `ValueResolver::getValue` reads them: a literal, a constant's name, a class constant's value, a concatenation of
//! such values, or the one value of the type of anything else.
//!
//! Rector reads a concatenated part it cannot resolve as `''`. Pun does too for the patterns, but drops the column
//! only when every part is known to give `created_at`, and leaves a call whose direction it cannot fully resolve,
//! where Rector would change what the query sorts by. Pun also leaves a call with an unpacked argument, which Rector
//! drops.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{
    Access, Argument, BinaryOperator, ClassConstantAccess, ClassLikeConstantSelector, Expression,
    Literal, Node, NodeKind, Variable,
};
use pun_problem::{Edit, Safety, Span};
use pun_tokens::php_compat::fnmatch;

use super::object_type::{
    class_string_class, is_object_type, name_class, object_class, special_class,
};
use super::print::reprinted_call;
use super::{is_name, member_name};
use crate::rules::rector_batch5::{Value, value};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};
use pun_tokens::php_compat::float_to_string;

pub struct PreferLatestOldest {
    pub allowed_patterns: Vec<String>,
}

impl Default for PreferLatestOldest {
    fn default() -> Self {
        Self {
            allowed_patterns: ["*_at", "*_date", "*_on"].map(String::from).to_vec(),
        }
    }
}

impl Rule for PreferLatestOldest {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-latest-oldest"
    }

    /// `configure`: `allowed-patterns` (rector-laravel's `allowed_patterns`), the `fnmatch` patterns a column must
    /// match. Options without it allow every column, as Rector's `configure` does.
    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        let map = match options {
            serde_json::Value::Null => return Ok(()),
            serde_json::Value::Object(map) => map,
            _ => return Err("options must be a table".into()),
        };
        if let Some(key) = map.keys().find(|k| *k != "allowed-patterns") {
            return Err(format!("unknown option `{key}`"));
        }
        let patterns = match map.get("allowed-patterns") {
            None => Some(Vec::new()),
            Some(v) => v.as_array().and_then(|a| {
                a.iter()
                    .map(|v| v.as_str().map(str::to_string))
                    .collect::<Option<Vec<_>>>()
            }),
        };
        self.allowed_patterns = patterns.ok_or("`allowed-patterns` must be a list of strings")?;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else { return };
        let Some(name) = member_name(&call.method) else {
            return;
        };
        let desc = is_name(name, "orderByDesc");
        if !desc && !is_name(name, "orderBy") {
            return;
        }
        let arguments: Vec<&Argument<'_>> = call.argument_list.arguments.iter().collect();
        if arguments
            .iter()
            .any(|a| matches!(a, Argument::Positional(p) if p.ellipsis.is_some()))
        {
            return;
        }
        let Some(column_at) = arg_at(&arguments, "column", 0) else {
            return;
        };
        let column = arguments[column_at].value();
        if !is_object_type(
            ctx,
            ancestors,
            call.object,
            "Illuminate\\Contracts\\Database\\Query\\Builder",
        ) || !self.is_allowed(ctx, ancestors, column)
        {
            return;
        }
        let direction = match arg_at(&arguments, "direction", 1) {
            None => "asc".to_string(),
            Some(at) => match get_value(ctx, ancestors, arguments[at].value()) {
                Got::Str(s, true) => s,
                _ => return,
            },
        };
        let method = if desc || !direction.eq_ignore_ascii_case("asc") {
            "latest"
        } else {
            "oldest"
        };
        let edits = if matches!(get_value(ctx, ancestors, column), Got::Str(s, true) if s == "created_at")
        {
            // The arguments go, so php-parser prints the call anew.
            reprinted_call(
                ctx.source,
                call.object,
                node_span(call).end,
                &format!("->{method}()"),
            )
        } else {
            let mut edits = vec![Edit::new(node_span(&call.method), method.to_string())];
            let span = |a: &Argument<'_>| node_span(a);
            if column_at > 0 {
                edits.push(Edit::new(
                    Span::new(span(arguments[0]).start, span(arguments[column_at]).start),
                    String::new(),
                ));
            }
            if let Some(last) = arguments.get(column_at + 1..).and_then(|rest| rest.last()) {
                edits.push(Edit::new(
                    Span::new(span(arguments[column_at]).end, span(last).end),
                    String::new(),
                ));
            }
            edits
        };
        out.push(Report::fixed(
            node_span(call),
            format!(
                "Use `{method}()` instead of `{}()`.",
                String::from_utf8_lossy(name)
            ),
            Safety::Unsafe,
            edits,
        ));
    }
}

impl PreferLatestOldest {
    /// `isAllowedPattern`.
    fn is_allowed(
        &self,
        ctx: &Context<'_>,
        ancestors: &[Node<'_, '_>],
        column: &Expression<'_>,
    ) -> bool {
        if self.allowed_patterns.is_empty() {
            return true;
        }
        if let Got::Str(value, _) = get_value(ctx, ancestors, column)
            && (self.allowed_patterns.iter()).any(|p| fnmatch(p.as_bytes(), value.as_bytes()))
        {
            return true;
        }
        let Expression::Variable(Variable::Direct(v)) = unparenthesize(column) else {
            return false;
        };
        let name = &v.name[1..];
        (self.allowed_patterns.iter()).any(|p| fnmatch(p.trim_start_matches('$').as_bytes(), name))
    }
}

/// php-parser's `getArg($name, $position)`: the argument named `name`, or the positional one at `position`.
fn arg_at(arguments: &[&Argument<'_>], name: &str, position: usize) -> Option<usize> {
    (arguments.iter().enumerate()).position(|(i, a)| match a {
        Argument::Named(n) => n.name.value == name.as_bytes(),
        Argument::Positional(_) => i == position,
    })
}

/// What `ValueResolver::getValue` gives: a string, and whether every part of it was known (a concatenated part
/// Rector cannot resolve reads as `''`); another value; or nothing (`null`).
pub(crate) enum Got {
    Str(String, bool),
    Other(Value),
    Unknown,
}

/// PHP's string of a value, as a concatenation makes it.
fn to_string(got: Got) -> (String, bool) {
    match got {
        Got::Str(s, known) => (s, known),
        Got::Other(Value::Int(i)) => (i.to_string(), true),
        Got::Other(Value::Float(f)) => (float_to_string(f), true),
        Got::Other(Value::Bool(true)) => ("1".into(), true),
        // `false` and `null`; strings are never `Other`
        Got::Other(_) => (String::new(), true),
        Got::Unknown => (String::new(), false),
    }
}

/// `ValueResolver::getValue`.
pub(crate) fn get_value(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
) -> Got {
    let expr = unparenthesize(expr);
    match expr {
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)) => {
            let (left, l) = to_string(get_value(ctx, ancestors, b.lhs));
            let (right, r) = to_string(get_value(ctx, ancestors, b.rhs));
            return Got::Str(left + &right, l && r);
        }
        Expression::Access(Access::ClassConstant(c)) => {
            return class_constant(ctx, ancestors, expr, c);
        }
        Expression::Literal(Literal::String(_)) | Expression::ConstantAccess(_) => {}
        Expression::Literal(_) | Expression::UnaryPrefix(_) => {}
        _ => return type_value(ctx, expr),
    }
    match value(expr) {
        Some(Value::Str(s) | Value::Constant(s)) => Got::Str(s, true),
        Some(v) => Got::Other(v),
        None => type_value(ctx, expr),
    }
}

/// `resolveClassConstFetch` on a class written as a name (else the type of the fetch): `X::class` is the class; a constant the codebase does not
/// have, or one of an enum, is `X::NAME`; another is the value Mago infers for it.
fn class_constant(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    c: &ClassConstantAccess<'_>,
) -> Got {
    let class = match unparenthesize(c.class) {
        Expression::Identifier(id) => name_class(ancestors, id),
        e @ (Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) => {
            special_class(ancestors, e)
        }
        e => return expression_class_value(ctx, expr, e, &c.constant),
    };
    let (Some(class), ClassLikeConstantSelector::Identifier(name)) = (class, &c.constant) else {
        return Got::Unknown;
    };
    let constant = String::from_utf8_lossy(name.value);
    if constant == "class" {
        return Got::Str(class, true);
    }
    let facts = facts(ctx);
    let known = facts.codebase.get_class_like(class.as_bytes());
    match known.filter(|k| !k.kind.is_enum()).and_then(|_| {
        facts
            .codebase
            .get_class_constant_type(class.as_bytes(), name.value)
    }) {
        Some(t) => literal(&t),
        None => Got::Str(format!("{class}::{constant}"), true),
    }
}

/// The type of a constant fetched on a class written as an expression (`ConstExprEvaluator` cannot evaluate it), as
/// PHPStan's `getClassConstFetchTypeByReflection` gives it: through a literal class string, the constant's value;
/// through an object, its value only when the class or the constant is final (else PHPStan gives `mixed`, or the
/// constant's declared type, neither of them one value).
fn expression_class_value(
    ctx: &Context<'_>,
    expr: &Expression<'_>,
    class: &Expression<'_>,
    constant: &ClassLikeConstantSelector<'_>,
) -> Got {
    let facts = facts(ctx);
    let (Some(t), ClassLikeConstantSelector::Identifier(name)) =
        (facts.full_type(node_span(class)), constant)
    else {
        return Got::Unknown;
    };
    let class_string = class_string_class(ctx, t);
    let read = match (&class_string, &t.types[..]) {
        (Some(c), _) => facts.codebase.get_class_like(c.as_bytes()).is_some(),
        (None, [atomic]) => object_class(atomic).is_some_and(|c| {
            (facts.codebase.get_class_like(c.as_bytes())).is_some_and(|k| k.flags.is_final())
                || (facts.codebase.get_class_constant(c.as_bytes(), name.value))
                    .is_some_and(|k| k.flags.is_final())
        }),
        (None, _) => false,
    };
    if read {
        type_value(ctx, expr)
    } else {
        Got::Unknown
    }
}

/// `resolveConstantType($nodeTypeResolver->getType($expr))`: the value of a constant scalar type.
fn type_value(ctx: &Context<'_>, expr: &Expression<'_>) -> Got {
    facts(ctx)
        .full_type(node_span(expr))
        .map_or(Got::Unknown, literal)
}

fn literal(t: &mago_codex::ttype::union::TUnion) -> Got {
    let [atomic] = &t.types[..] else {
        return Got::Unknown;
    };
    match atomic {
        TAtomic::Null => Got::Other(Value::Null),
        TAtomic::Scalar(TScalar::Bool(b)) if b.is_true() => Got::Other(Value::Bool(true)),
        TAtomic::Scalar(TScalar::Bool(b)) if b.is_false() => Got::Other(Value::Bool(false)),
        TAtomic::Scalar(TScalar::Integer(i)) => i
            .get_literal_value()
            .map_or(Got::Unknown, |i| Got::Other(Value::Int(i))),
        TAtomic::Scalar(s) => s
            .get_known_literal_string_value()
            .and_then(|v| std::str::from_utf8(v).ok())
            .map_or(Got::Unknown, |v| Got::Str(v.to_string(), true)),
        _ => Got::Unknown,
    }
}
