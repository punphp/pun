//! `lint/types/add-return-type-from-try-catch`, ported from Rector 2.8.4's `AddReturnTypeFromTryCatchTypeRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): a method whose `return`s all return a value, and whose first-level
//! `try` / `catch` / `finally` blocks each return a value of the same type, gets that type.
//!
//! Each block's type is the native type of the first `return` among its own statements. A block `catch` without one
//! stops the rule; a `finally` without one does not count. `TypeComparator::areTypesEqual()` compares the types by
//! the kind of constant they are (`1` and `2` are equal, `1` and `'a'` are not, `1` and `int` are not) and arrays of
//! known items alike; Pun leaves a method alone whose types are unions of constants or other shapes it cannot compare
//! by identity.

use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Block, Node, NodeKind, Statement};

use super::declare::{return_declaration, union_order};
use super::facts::{Facts, facts_with_dead_code};
use super::native::native_type;
use super::silent_void::Place;
use super::{
    SCALAR_TYPES, add_return_type, class_scope, has_only_return_with_expr, returns_scoped, untyped,
};
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromTryCatch;

impl Rule for AddReturnTypeFromTryCatch {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-try-catch"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
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
        let scope = class_scope(ancestors);
        // a `catch` no throw point reaches is dead code to Mago, and typed by PHPStan
        let facts = facts_with_dead_code(ctx);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let returns = returns_scoped(f.body);
        if !has_only_return_with_expr(&place, f.body, &returns) {
            return;
        }
        let mut try_type: Option<&TUnion> = None;
        let mut catch_types: Vec<&TUnion> = Vec::new();
        for statement in f.body.statements.iter() {
            let Statement::Try(r#try) = statement else {
                continue;
            };
            // skip if there is no catch
            if r#try.catch_clauses.is_empty() {
                continue;
            }
            try_type = block_type(&facts, &r#try.block);
            for clause in r#try.catch_clauses.iter() {
                // each catch must have type
                let Some(t) = block_type(&facts, &clause.block) else {
                    return;
                };
                catch_types.push(t);
            }
            if let Some(t) =
                (r#try.finally_clause.as_ref()).and_then(|c| block_type(&facts, &c.block))
            {
                catch_types.push(t);
            }
        }
        let Some(try_type) = try_type else {
            return;
        };
        let Some(wanted) = hash(try_type) else {
            return;
        };
        if catch_types
            .iter()
            .any(|t| hash(t).as_ref() != Some(&wanted))
        {
            return;
        }
        // Rector's mapper gives no node for the type of `Foo::class`
        if (try_type.types.iter())
            .any(|t| matches!(t, TAtomic::Scalar(TScalar::ClassLikeString(c)) if c.is_literal()))
        {
            return;
        }
        let types = union_order(try_type.types.iter());
        if let Some(r#type) = return_declaration(types, &facts.codebase, ctx.php_version, &[], true)
        {
            out.push(add_return_type(f.span, f.at, &r#type));
        }
    }
}

/// `matchReturnType()`: the native type of the first `return` with a value among the block's own statements.
fn block_type<'f>(facts: &'f Facts, block: &Block<'_>) -> Option<&'f TUnion> {
    block
        .statements
        .iter()
        .find_map(|statement| match statement {
            Statement::Return(r) => r.value.and_then(|value| native_type(facts, value)),
            _ => None,
        })
}

/// `TypeHasher::createTypeHash()` with `ScalarTypeComparator::areEqualScalar()`: constants hash as their kind, every
/// boolean and float alike. `None` for the shapes Pun does not compare.
fn hash(ty: &TUnion) -> Option<String> {
    let mut keys: Vec<String> = Vec::new();
    let mut constants = 0;
    for atomic in ty.types.iter() {
        let key = match atomic {
            TAtomic::Null => "null".to_string(),
            TAtomic::Scalar(TScalar::Bool(_)) => "bool".to_string(),
            TAtomic::Scalar(TScalar::Float(_)) => "float".to_string(),
            TAtomic::Scalar(TScalar::Integer(i)) if i.is_literal() => {
                constants += 1;
                "int-constant".to_string()
            }
            TAtomic::Scalar(TScalar::String(s)) if s.is_known_literal() => {
                constants += 1;
                "string-constant".to_string()
            }
            // a `ConstantArrayType`: every array of known items is the same
            TAtomic::Array(array) if array.is_sealed() => "constant-array".to_string(),
            TAtomic::Scalar(TScalar::ClassLikeString(c)) if c.is_literal() => {
                constants += 1;
                "string-constant".to_string()
            }
            // `$this`, `static` and the class itself have the class's hash
            TAtomic::Object(TObject::Named(named)) if named.intersection_types.is_none() => {
                format!("object:{}", named.name.to_ascii_lowercase())
            }
            TAtomic::Scalar(_) | TAtomic::Object(_) | TAtomic::Array(_) => {
                atomic.get_id().to_string()
            }
            _ => return None,
        };
        keys.push(key);
    }
    if constants > 1 {
        return None;
    }
    keys.sort();
    keys.dedup();
    Some(keys.join("|"))
}
