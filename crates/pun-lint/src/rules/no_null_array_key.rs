//! `lint/php-8.5/no-null-array-key`, ported from Rector 2.8.4's `ArrayKeyExistsNullToEmptyStringRector`
//! (rules/Php85/Rector/FuncCall).
//!
//! PHP 8.5 deprecates `null` as the key of `array_key_exists()`. A key whose type is `null` (`isNull()->yes()`)
//! becomes `''`; a key that only may be `null` stays (Rector 2.8.4 dropped the `(string)` cast it wrote for one).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{
    Access, Argument, BinaryOperator, ClassLikeMemberSelector, Expression, Literal, Node, NodeKind, Variable,
};
use pun_problem::{Edit, Fix, Safety};

use super::class_method::type_of;
use super::lint_b_helpers::value;
use super::rector_code_quality::{args, named};
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNullArrayKey;

impl Rule for NoNullArrayKey {
    fn id(&self) -> &'static str {
        "lint/php-8.5/no-null-array-key"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersion::PHP_85
        80500
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        if !named(c, "array_key_exists") {
            return;
        }
        let args = args(c);
        if args.len() != 2 {
            return;
        }
        // `ArgsAnalyzer::resolveArgPosition($args, 'key', 0)`
        let position = (args.iter())
            .position(|a| matches!(a, Argument::Named(n) if n.name.value.eq_ignore_ascii_case(b"key")))
            .unwrap_or(0);
        let key = unparenthesize(value(args[position]));
        let facts = facts(ctx);
        // PHPStan 2.3: `$v ?? null` of a variable no scope defines is the right side's type
        let typed = match key {
            Expression::Binary(b)
                if matches!(b.operator, BinaryOperator::NullCoalesce(_))
                    && is_undefined(ctx, ancestors, unparenthesize(b.lhs)) =>
            {
                unparenthesize(b.rhs)
            }
            _ => key,
        };
        // PHPStan's type of a property no class declares is an error, never `null`; Mago's is `null`
        if is_undeclared_property(&facts, typed) {
            return;
        }
        let is_null = match type_of(&facts, typed) {
            Some(t) => !t.types.is_empty() && t.types.iter().all(|a| matches!(a, TAtomic::Null)),
            None => matches!(typed, Expression::Literal(Literal::Null(_))),
        };
        if !is_null {
            return;
        }
        out.push(Report {
            span: node_span(key),
            message: "PHP 8.5 deprecates a `null` key in `array_key_exists()`: pass a string.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(node_span(key), "''")],
            }),
        });
    }
}

/// Whether `expr` is a variable named nowhere else in the outermost function, method or file around it: no scope
/// defines it.
fn is_undefined(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    let Expression::Variable(Variable::Direct(v)) = expr else {
        return false;
    };
    if v.name == b"$this".as_slice() || v.name.starts_with(b"$_") || v.name == b"$GLOBALS".as_slice() {
        return false;
    }
    let outer = (ancestors.iter())
        .find(|a| matches!(a, Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_)))
        .map(|a| node_span(a))
        .unwrap_or_else(|| pun_problem::Span::new(0, ctx.source.len()));
    let text = &ctx.source.as_bytes()[outer.start..outer.end];
    let name = v.name;
    let mut count = 0;
    let mut at = 0;
    while let Some(i) = text[at..].windows(name.len()).position(|w| w == name) {
        let end = at + i + name.len();
        if !text.get(end).is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c >= 0x80) {
            count += 1;
        }
        at = end;
    }
    count == 1
}

/// Whether `expr` reads `$object->name` where every class the object may be declares no property `$name`.
fn is_undeclared_property(facts: &Facts, expr: &Expression<'_>) -> bool {
    let Expression::Access(Access::Property(access)) = expr else {
        return false;
    };
    let ClassLikeMemberSelector::Identifier(name) = &access.property else {
        return false;
    };
    let Some(t) = facts.full_type(node_span(access.object)) else {
        return false;
    };
    let property = [b"$".as_slice(), name.value].concat();
    let classes: Vec<_> = (t.types.iter())
        .filter_map(|a| match a {
            TAtomic::Object(TObject::Named(n)) => Some(n.name),
            _ => None,
        })
        .collect();
    !classes.is_empty()
        && classes.len() == t.types.len()
        && classes.iter().all(|c| {
            facts.codebase.get_class_like(c.as_bytes()).is_some()
                && facts.codebase.get_declaring_property(c.as_bytes(), &property).is_none()
        })
}
