//! `lint/types/add-param-type-spl-fixed-array`, ported from Rector 2.6.1's `AddParamTypeSplFixedArrayRector`
//! (rules/TypeDeclaration/Rector/FunctionLike) with `PhpDocTypeChanger::changeParamType`: a parameter of a function or
//! method declared as PHP-CS-Fixer's `Tokens` class gets the generic `@param Tokens<Token>` type in the docblock.

use mago_syntax::cst::{FunctionLikeParameterList, Hint, Node, NodeKind};

use super::names::resolve_class_name;
use super::param::report;
use crate::rules::phpunit::annotations::Annotated;
use crate::rules::phpunit::phpdoc_type::{TagType, typed_tag};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// `SPL_FIXED_ARRAY_TO_SINGLE`: each `SplFixedArray` class with the class of its items.
const SPL_FIXED_ARRAY_TO_SINGLE: [(&str, &str); 2] = [
    ("PhpCsFixer\\Tokenizer\\Tokens", "PhpCsFixer\\Tokenizer\\Token"),
    (
        "PhpCsFixer\\Doctrine\\Annotation\\Tokens",
        "PhpCsFixer\\Doctrine\\Annotation\\Token",
    ),
];

pub struct AddParamTypeSplFixedArray;

impl Rule for AddParamTypeSplFixedArray {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-spl-fixed-array"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let params: &FunctionLikeParameterList<'_> = match node {
            Node::Function(f) => &f.parameter_list,
            Node::Method(m) => &m.parameter_list,
            _ => return,
        };
        let Some(annotated) = Annotated::of(ctx.source, node, ancestors) else {
            return;
        };
        let mut appended: Vec<String> = Vec::new();
        let mut replaced: Vec<(usize, String)> = Vec::new();
        for param in params.parameters.iter() {
            // a class name only: `?Tokens` and unions are no `TypeWithClassName`
            let Some(Hint::Identifier(id)) = &param.hint else {
                continue;
            };
            let class = resolve_class_name(ancestors, id);
            let Some((fixed, single)) = SPL_FIXED_ARRAY_TO_SINGLE.iter().find(|(c, _)| *c == class)
            else {
                continue;
            };
            let generic = format!("\\{fixed}<\\{single}>");
            let name = String::from_utf8_lossy(param.variable.name);
            match annotated.doc.as_ref().map(|d| typed_tag(d, "@param", Some(&name))) {
                // `hasInvalidTag('@param')`: better skip
                Some(Err(())) => return,
                Some(Ok(Some(tag))) => {
                    // the tag's type is replaced unless it is the generic type already
                    match &tag.ty {
                        TagType::Name(n) if super::names::resolve_written(ancestors, n) == *fixed => {
                            replaced.push((tag.index, tag.print_with(&generic)));
                        }
                        _ => continue,
                    }
                }
                _ => appended.push(format!("@param {generic} {name}")),
            }
        }
        if appended.is_empty() && replaced.is_empty() {
            return;
        }
        let new_doc = match &annotated.doc {
            None => format!("/**\n * {}\n */", appended.join("\n * ")),
            Some(doc) => {
                let mut text = doc.text_all().to_string();
                // replace from the last tag back, so earlier offsets hold
                for (index, printed) in replaced.iter().rev() {
                    let child = doc.text(*index).trim_end();
                    let from = child.as_ptr() as usize - doc.text_all().as_ptr() as usize;
                    text.replace_range(from..from + child.len(), printed);
                }
                if appended.is_empty() {
                    text
                } else if replaced.is_empty() {
                    let appended: Vec<&str> = appended.iter().map(String::as_str).collect();
                    doc.print_appending(&vec![false; doc.len()], &appended)
                } else {
                    // a tag replaced and another added in one docblock: not followed
                    return;
                }
            }
        };
        let Some(edits) = annotated.edits(ctx.source, Some(&new_doc), &[]) else {
            return;
        };
        out.push(report(
            node_span(&node),
            "Add the generic `Tokens<Token>` type to the parameter's @param tag.",
            edits,
        ));
    }
}
