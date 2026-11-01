//! `lint/phpunit/prefer-property-assertions`, ported from rector-phpunit's `AssertArrayCastedObjectToAssertSameRector`
//! (rules/CodeQuality/Rector/Expression).
//!
//! In a test class, `$this->assertSame(['name' => 'John', 'surname' => 'Doe'], (array) $object);` on an object of one
//! class becomes one `$this->assertSame('John', $object->name);` per item. Rector drops a message argument, and so
//! does Pun.
//!
//! Rector reads the expected values with `ValueResolver::getValue()`; Pun reads an array of string keys (not number
//! strings, not repeated) whose values are `null`, booleans, integers or strings, and leaves any other array alone.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{
    Array, ArrayElement, Call, Expression, Node, NodeKind, Statement, UnaryPrefixOperator,
};
use pun_problem::{Edit, Safety, Span};

use super::{AnyCall, is_in_test_class};
use crate::rules::rector_batch5::{Value, float_string, value};
use crate::rules::rector_stmts::{Frame, enclosing, line_indent};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferPropertyAssertions;

impl Rule for PreferPropertyAssertions {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-property-assertions"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Statement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Statement(statement) = node else {
            return;
        };
        let Statement::Expression(expression) = statement else {
            return;
        };
        let Expression::Call(Call::Method(method)) = expression.expression else {
            return;
        };
        let call = AnyCall::of(Node::MethodCall(method)).unwrap();
        if call.is_static
            || !call.is_name(&["assertSame"])
            || !is_in_test_class(ctx, ancestors, None)
        {
            return;
        }
        let args = call.args();
        let (Some(expected), Some(actual)) = (args.first(), args.get(1)) else {
            return;
        };
        let Expression::Array(array) = unparenthesize(expected.value()) else {
            return;
        };
        let Expression::UnaryPrefix(cast) = unparenthesize(actual.value()) else {
            return;
        };
        if !matches!(cast.operator, UnaryPrefixOperator::ArrayCast(..)) {
            return;
        }
        // an `ObjectType`
        let Some(items) = items(array) else {
            return;
        };
        let facts = facts(ctx);
        let object = matches!(
            facts.full_type(node_span(cast.operand)).map(|t| &t.types[..]),
            Some([TAtomic::Object(TObject::Named(named))])
                if named.intersection_types.is_none() && !named.is_this && !named.is_static
        ) || matches!(
            facts
                .full_type(node_span(cast.operand))
                .map(|t| &t.types[..]),
            Some([TAtomic::Object(TObject::Enum(_))])
        );
        if !object || !is_dereferenceable(cast.operand) {
            return;
        }
        let object =
            ctx.source[node_span(cast.operand).start..node_span(cast.operand).end].to_string();
        let (list, index) = enclosing(ctx.source, statement, ancestors);
        let span = node_span(statement);
        let indent = line_indent(ctx.source, span.start);
        let lines: Vec<String> = items
            .iter()
            .map(|(name, v)| format!("$this->assertSame({v}, {object}->{name});"))
            .collect();
        let edit = if list.frame == Frame::Bare {
            let inner = format!("{indent}    ");
            Edit::new(
                span,
                format!(
                    "{{\n{inner}{}\n{indent}}}",
                    lines.join(&format!("\n{inner}"))
                ),
            )
        } else if index > 0 && lines.len() > 1 && list.stmts.len() > index {
            // php-parser's differ sees one removal and several additions: the removed statement goes with the
            // whitespace before it
            let from = list.before(ctx.source, index);
            Edit::new(
                Span::new(from, span.end),
                format!("\n{indent}{}", lines.join(&format!("\n{indent}"))),
            )
        } else {
            Edit::new(span, lines.join(&format!("\n{indent}")))
        };
        out.push(Report::fixed(
            span,
            "Assert the properties of the object one by one.",
            Safety::Safe,
            vec![edit],
        ));
    }
}

/// A PHP array key.
#[derive(Clone, PartialEq)]
enum Key {
    Int(i64),
    Str(String),
}

/// A value of `ValueResolver::getValue()`, by the order PHP keeps its array items in.
enum Val {
    Scalar(Value),
    Array(Vec<(Key, Val)>),
}

/// The property name and the printed expected value of each item.
fn items<'a>(array: &'a Array<'a>) -> Option<Vec<(String, String)>> {
    let items = evaluate_array(array)?;
    let mut out = Vec::new();
    for (key, v) in &items {
        let Key::Str(name) = key else {
            return None;
        };
        let mut chars = name.chars();
        let valid = chars
            .next()
            .is_some_and(|c| c == '_' || c.is_ascii_alphabetic())
            && chars.all(|c| c == '_' || c.is_ascii_alphanumeric());
        if !valid {
            return None;
        }
        out.push((name.clone(), print(v)?));
    }
    (!out.is_empty()).then_some(out)
}

/// `ConstExprEvaluator::evaluateDirectly()` on literals and arrays of them.
fn evaluate(expr: &Expression<'_>) -> Option<Val> {
    let Expression::Array(array) = unparenthesize(expr) else {
        return match value(expr)? {
            Value::Constant(_) => None,
            scalar => Some(Val::Scalar(scalar)),
        };
    };
    evaluate_array(array).map(Val::Array)
}

fn evaluate_array<'a>(array: &'a Array<'a>) -> Option<Vec<(Key, Val)>> {
    let mut items: Vec<(Key, Val)> = Vec::new();
    let mut next: i64 = 0;
    for element in array.elements.iter() {
        let (key, v) = match element {
            ArrayElement::KeyValue(item) => {
                let key = match value(item.key)? {
                    Value::Int(i) => Key::Int(i),
                    Value::Str(s) => match s.parse::<i64>() {
                        Ok(i) if i.to_string() == s => Key::Int(i),
                        _ => Key::Str(s),
                    },
                    _ => return None,
                };
                (key, evaluate(item.value)?)
            }
            ArrayElement::Value(item) => (Key::Int(next), evaluate(item.value)?),
            _ => return None,
        };
        if let Key::Int(i) = key {
            next = next.max(i.checked_add(1)?);
        }
        match items.iter_mut().find(|(k, _)| *k == key) {
            Some(existing) => existing.1 = v,
            None => items.push((key, v)),
        }
    }
    Some(items)
}

/// `BuilderHelpers::normalizeValue()` printed by php-parser's pretty printer.
fn print(v: &Val) -> Option<String> {
    Some(match v {
        Val::Scalar(Value::Null) => "null".to_string(),
        Val::Scalar(Value::Bool(b)) => b.to_string(),
        Val::Scalar(Value::Int(i)) => i.to_string(),
        Val::Scalar(Value::Float(f)) => {
            let text = float_string(*f)?;
            if text.contains('.') {
                text
            } else {
                format!("{text}.0")
            }
        }
        Val::Scalar(Value::Str(s)) => format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'")),
        Val::Scalar(Value::Constant(_)) => return None,
        Val::Array(items) => {
            let mut last: Option<i64> = Some(-1);
            let mut parts = Vec::new();
            for (key, v) in items {
                let sequential = match (last, key) {
                    (Some(l), Key::Int(i)) if l + 1 == *i => {
                        last = Some(*i);
                        true
                    }
                    _ => {
                        last = None;
                        false
                    }
                };
                parts.push(if sequential {
                    print(v)?
                } else {
                    let key = match key {
                        Key::Int(i) => i.to_string(),
                        Key::Str(s) => {
                            format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
                        }
                    };
                    format!("{key} => {}", print(v)?)
                });
            }
            format!("[{}]", parts.join(", "))
        }
    })
}

/// Whether `->name` can follow the expression as written.
fn is_dereferenceable(expr: &Expression<'_>) -> bool {
    matches!(
        expr,
        Expression::Variable(_)
            | Expression::Access(_)
            | Expression::Call(_)
            | Expression::Instantiation(_)
    ) && !matches!(expr, Expression::Instantiation(_))
}
