//! `lint/types/prefer-generic-collection-phpdoc`, ported from Rector 2.8.4's
//! `NarrowArrayCollectionUnionReturnDocblockRector` (rules/TypeDeclarationDocblocks/Rector/ClassMethod): a
//! `@return Item[]|Collection` (or `ArrayCollection`) becomes `@return Collection<int, Item>`.

use mago_syntax::cst::{Node, NodeKind};

use super::docblock::FunctionLike;
use crate::rules::phpdoc_writer::TypeNode;
use crate::{Context, Report, Rule};

pub struct PreferGenericCollectionPhpdoc;

impl Rule for PreferGenericCollectionPhpdoc {
    fn id(&self) -> &'static str {
        "lint/types/prefer-generic-collection-phpdoc"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = FunctionLike::of(node) else {
            return;
        };
        let target = f.target(ctx);
        // `createFromNode()`: no doc comment, no return tag
        if target.doc.is_none() {
            return;
        }
        let mut info = target.info(ctx.source);
        let Some(TypeNode::Union { types, .. }) = info.return_tag().map(|t| &t.ty) else {
            return;
        };
        let Some(generic) = match_generic_collection(types) else {
            return;
        };
        info.set_return_type(generic);
        out.extend(super::docblock::doc_report(
            f.name,
            "Use a generic collection `@return` type.",
            ctx.source,
            &target,
            &info,
        ));
    }
}

/// `matchGenericCollectionTypeNode()`.
fn match_generic_collection(types: &[TypeNode]) -> Option<TypeNode> {
    if types.len() != 2 {
        return None;
    }
    let (mut item, mut collection) = (None, None);
    for t in types {
        match t {
            TypeNode::Array { inner, .. } if matches!(**inner, TypeNode::Identifier(_)) => {
                item = Some((**inner).clone());
            }
            TypeNode::Identifier(name) => {
                let short = name.trim_start_matches('\\').rsplit('\\').next().unwrap_or(name);
                if matches!(short, "Collection" | "ArrayCollection") {
                    collection = Some(name.clone());
                }
            }
            _ => {}
        }
    }
    Some(TypeNode::Generic {
        base: collection?,
        args: vec![TypeNode::ident("int"), item?],
    })
}
