//! `lint/types/add-param-phpdoc-from-dim-fetch`, ported from Rector 2.6.1's `AddParamArrayDocblockFromDimFetchAccessRector`
//! (rules/TypeDeclarationDocblocks/Rector/ClassMethod): a parameter declared `array` that is read only with string
//! (or only with integer) keys gets `@param array<string, mixed>` (or `array<int, mixed>`).

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_phpstan::types::Type;

use super::class_scope;
use super::docblock::{FunctionLike, doc_report, find_scoped, is_array_hint, variable_name};
use super::facts::facts;
use super::never::has_parent_class_method;
use crate::rules::phpdoc_writer::type_mapper::is_useful_array_tag;
use crate::rules::phpdoc_writer::{TypeNode, expr_type};
use crate::{Context, Report, Rule};

pub struct AddParamPhpdocFromDimFetch;

impl Rule for AddParamPhpdocFromDimFetch {
    fn id(&self) -> &'static str {
        "lint/types/add-param-phpdoc-from-dim-fetch"
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
        let Some(f) = FunctionLike::of(node) else {
            return;
        };
        let target = f.target(ctx);
        let mut info = target.info(ctx.source);
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let mut changed = false;
        for param in f.parameters.parameters.iter() {
            if !param.hint.as_ref().is_some_and(is_array_hint) {
                continue;
            }
            let name = String::from_utf8_lossy(&param.variable.name[1..]).into_owned();
            // already defined, lets skip it
            if is_useful_array_tag(info.param_tag(&name).map(|t| &t.ty)) {
                continue;
            }
            // `ArrayDimFetchFinder::findByVariableName()`: the dims of `$name[...]` reads
            let mut dims: Vec<Option<&Expression<'_>>> = Vec::new();
            find_scoped(node, &mut |n| match n {
                Node::ArrayAccess(a) if variable_name(a.array) == Some(name.as_str()) => dims.push(Some(a.index)),
                Node::ArrayAppend(a) if variable_name(a.array) == Some(name.as_str()) => dims.push(None),
                _ => {}
            });
            if dims.is_empty() {
                continue;
            }
            // skip for now, not to create more error on incompatible parent @param doc override
            if f.method.is_some() && !f.is_private() {
                let facts = facts(ctx);
                if class_scope(ancestors).is_none_or(|s| has_parent_class_method(&facts, &s, f.name_text)) {
                    continue;
                }
            }
            let keys: Vec<Option<Type>> = dims.into_iter().flatten().map(|d| expr_type(ctx, &scope, d)).collect();
            // most likely not being read
            if keys.is_empty() {
                continue;
            }
            let key = if keys.iter().all(|k| k.as_ref().is_some_and(|k| k.is_string().yes())) {
                "string"
            } else if keys.iter().all(|k| k.as_ref().is_some_and(|k| k.is_integer().yes())) {
                "int"
            } else {
                continue;
            };
            info.set_param_type(
                &name,
                false,
                TypeNode::Generic {
                    base: "array".to_string(),
                    args: vec![TypeNode::ident(key), TypeNode::ident("mixed")],
                },
            );
            changed = true;
        }
        if changed {
            out.extend(doc_report(
                f.name,
                "Add the `@param` array key type from the parameter's reads.",
                ctx.source,
                &target,
                &info,
            ));
        }
    }
}
