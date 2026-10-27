//! `lint/refactor/rename-class-constant`, ported from Rector 2.6.1's `RenameClassConstFetchRector`
//! (rules/Renaming/Rector/ClassConstFetch).
//!
//! Options: `items = [{ old-class = "A", old-constant = "OLD", new-constant = "NEW" }]` (`RenameClassConstFetch`), or
//! with a `new-class` too (`RenameClassAndConstFetch`). A class constant fetch whose constant is the old one (without
//! case) on a class of the old class's type (`NodeTypeResolver::isObjectType`) gets the new constant name; with a new
//! class it becomes `\NewClass::NEW`.
//!
//! The class's type: a name resolved through the imports and the namespace; `self` and `static` the enclosing class
//! (an anonymous one has none); `parent` the parent classes, every one of which must be of the type. An expression
//! (`$object::CONST`) takes its type from Mago's analyser, `null` and `false` left out, each remaining type a named
//! object of the old class's type. A class is of the type when it is the same name or extends or implements it in the
//! codebase.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{ClassLikeConstantSelector, Expression, Identifier, Node, NodeKind};
use pun_problem::Edit;

use super::{assert_class, assert_constant, items, object, report, string};
use crate::rules::rector_batch5::resolve_class_name;
use crate::rules::types::class_scope;
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct Rename {
    pub old_class: String,
    pub old_constant: String,
    pub new_class: Option<String>,
    pub new_constant: String,
}

#[derive(Default)]
pub struct RenameClassConstant {
    pub renames: Vec<Rename>,
}

impl Rule for RenameClassConstant {
    fn id(&self) -> &'static str {
        "lint/refactor/rename-class-constant"
    }

    /// Rector runs the rule once per file; its fix could match again on what it wrote.
    fn once(&self) -> bool {
        true
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        let mut renames = Vec::new();
        for item in items(options)? {
            let map = object(
                item,
                &["old-class", "old-constant", "new-class", "new-constant"],
                &["old-class", "old-constant", "new-constant"],
            )?;
            let old_class = string(map, "old-class")?;
            let old_constant = string(map, "old-constant")?;
            let new_class = match map.contains_key("new-class") {
                true => Some(string(map, "new-class")?),
                false => None,
            };
            let new_constant = string(map, "new-constant")?;
            assert_class(&old_class)?;
            assert_constant(&old_constant)?;
            if let Some(class) = &new_class {
                assert_class(class)?;
            }
            assert_constant(&new_constant)?;
            renames.push(Rename {
                old_class,
                old_constant,
                new_class,
                new_constant,
            });
        }
        self.renames = renames;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ClassConstantAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ClassConstantAccess(fetch) = node else {
            return;
        };
        let ClassLikeConstantSelector::Identifier(constant) = &fetch.constant else {
            return;
        };
        let name = String::from_utf8_lossy(constant.value);
        for rename in &self.renames {
            if !name.eq_ignore_ascii_case(&rename.old_constant)
                || !is_object_type(ctx, ancestors, fetch.class, &rename.old_class)
            {
                continue;
            }
            let (span, text) = match &rename.new_class {
                Some(class) => (
                    node_span(fetch),
                    format!("\\{class}::{}", rename.new_constant),
                ),
                None => (node_span(constant), rename.new_constant.clone()),
            };
            // A rename to what is already written would repeat in every fix round.
            if ctx.source[span.start..span.end] == text {
                return;
            }
            out.push(report(
                node_span(fetch),
                format!(
                    "Rename the class constant `{}::{}` to `{}::{}`.",
                    rename.old_class,
                    rename.old_constant,
                    rename.new_class.as_deref().unwrap_or(&rename.old_class),
                    rename.new_constant
                ),
                vec![Edit::new(span, text)],
            ));
            return;
        }
    }
}

/// `isObjectTypeOfObjectType`: the same name, or a class the codebase knows to extend or implement `required`.
fn is_class(ctx: &Context<'_>, resolved: &str, required: &str) -> bool {
    resolved == required
        || facts(ctx)
            .codebase
            .is_instance_of(resolved.as_bytes(), required.as_bytes())
}

/// `NodeTypeResolver::isObjectType($class, new ObjectType($required))` for the class part of a class constant fetch.
fn is_object_type(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    class: &Expression<'_>,
    required: &str,
) -> bool {
    match unparenthesize(class) {
        Expression::Identifier(id) => {
            let written = String::from_utf8_lossy(id.value());
            match written.to_ascii_lowercase().as_str() {
                "self" | "static" if matches!(id, Identifier::Local(_)) => {
                    match class_scope(ancestors).and_then(|c| c.name) {
                        Some(name) => is_class(ctx, &name, required),
                        None => is_class(ctx, &written, required),
                    }
                }
                "parent" if matches!(id, Identifier::Local(_)) => {
                    parents(ctx, ancestors).is_some_and(|chain| {
                        chain.iter().all(|p| is_class(ctx, p, required))
                    })
                }
                _ => is_class(ctx, &resolve_class_name(ancestors, id), required),
            }
        }
        Expression::Static(_) | Expression::Self_(_) => class_scope(ancestors)
            .and_then(|c| c.name)
            .is_some_and(|name| is_class(ctx, &name, required)),
        Expression::Parent(_) => parents(ctx, ancestors)
            .is_some_and(|chain| chain.iter().all(|p| is_class(ctx, p, required))),
        other => {
            let facts = facts(ctx);
            let Some(t) = facts.full_type(node_span(other)) else {
                return false;
            };
            let kept: Vec<&TAtomic> = (t.types.iter())
                .filter(|a| !a.is_null() && !matches!(a, TAtomic::Scalar(s) if s.is_false()))
                .collect();
            !kept.is_empty()
                && kept.iter().all(|a| match a {
                    TAtomic::Object(TObject::Named(n)) => {
                        is_class(ctx, &n.name.to_string(), required)
                    }
                    _ => false,
                })
        }
    }
}

/// `NameTypeResolver::resolveParent`: the enclosing class's parent classes, nearest first; `None` outside a class,
/// in an anonymous class, or for a class without a parent.
fn parents(ctx: &Context<'_>, ancestors: &[Node<'_, '_>]) -> Option<Vec<String>> {
    let scope = class_scope(ancestors)?;
    scope.name.as_ref()?;
    let first = scope.parent_class?;
    let facts = facts(ctx);
    let mut chain = vec![first.clone()];
    let mut next = first;
    while let Some(parent) = facts
        .codebase
        .get_class_like(next.as_bytes())
        .and_then(|c| c.direct_parent_class)
    {
        next = parent.to_string();
        if chain.iter().any(|c| c.eq_ignore_ascii_case(&next)) {
            break;
        }
        chain.push(next.clone());
    }
    Some(chain)
}
