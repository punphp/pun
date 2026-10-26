//! `lint/types/add-fluent-return-type`, ported from Rector 2.6.1's `ReturnTypeFromStrictFluentReturnRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): a method that always returns `$this` (or `new static`) returns
//! `static`, or `self` in a final or anonymous class, an enum, or before PHP 8.0; one that always returns `new self`
//! returns `self`.

use mago_syntax::cst::{Node, NodeKind};

use super::facts::facts;
use super::native::{Kind, returned_kinds};
use super::silent_void::Place;
use super::{add_return_type, class_scope, returns_scoped, untyped};
use crate::{Context, Report, Rule};

pub struct AddFluentReturn;

/// `PhpVersionFeature::HAS_RETURN_TYPE`.
const HAS_RETURN_TYPE: u32 = 70000;
/// `PhpVersionFeature::STATIC_RETURN_TYPE`.
const STATIC_RETURN_TYPE: u32 = 80000;

impl Rule for AddFluentReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-fluent-return-type"
    }

    fn min_php_version(&self) -> u32 {
        HAS_RETURN_TYPE
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = untyped(node, ancestors, ctx) else {
            return;
        };
        let Some(scope) = class_scope(ancestors) else {
            return;
        };
        let facts = facts(ctx);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: Some(&scope),
        };
        let returns = returns_scoped(f.body);
        let Some(kinds) = returned_kinds(&place, f.body, &returns) else {
            return;
        };
        let Some((name, is_static, is_this)) = one_class(&kinds) else {
            return;
        };
        let same_class = scope
            .name
            .as_deref()
            .is_some_and(|n| n.eq_ignore_ascii_case(name));
        let self_or_static = if scope.is_final || ctx.php_version < STATIC_RETURN_TYPE {
            "self"
        } else {
            "static"
        };
        let r#type = if is_this || (is_static && same_class) {
            self_or_static
        } else if same_class {
            "self"
        } else {
            return;
        };
        out.push(add_return_type(f.span, f.at, r#type));
    }
}

/// The returned objects as PHPStan's `TypeCombinator::union` joins them, when they are all of one class: `$this`
/// and `static` of it make `static` (`$this` alone stays `$this`), and the class itself with either makes the class.
fn one_class(kinds: &[Kind]) -> Option<(&str, bool, bool)> {
    let mut objects = kinds.iter().map(|k| match k {
        Kind::Object {
            name,
            is_static,
            is_this,
        } => Some((name.as_str(), *is_static, *is_this)),
        _ => None,
    });
    let first = objects.next()??;
    objects.try_fold(first, |(name, is_static, is_this), next| {
        let (other, other_static, other_this) = next?;
        name.eq_ignore_ascii_case(other).then_some((
            name,
            is_static && other_static,
            is_this && other_this,
        ))
    })
}
