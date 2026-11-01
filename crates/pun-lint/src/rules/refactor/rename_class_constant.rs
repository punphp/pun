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

use mago_syntax::cst::{ClassLikeConstantSelector, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{assert_class, assert_constant, is_object_type, items, object, string};
use crate::syntax::node_span;
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
            out.push(Report::fixed(
                node_span(fetch),
                format!(
                    "Rename the class constant `{}::{}` to `{}::{}`.",
                    rename.old_class,
                    rename.old_constant,
                    rename.new_class.as_deref().unwrap_or(&rename.old_class),
                    rename.new_constant
                ),
                Safety::Unsafe,
                vec![Edit::new(span, text)],
            ));
            return;
        }
    }
}
