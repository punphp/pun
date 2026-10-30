//! `lint/types/add-getter-return-phpdoc`, ported from Rector 2.6.1's
//! `DocblockGetterReturnArrayFromPropertyDocblockVarRector` (rules/TypeDeclarationDocblocks/Rector/ClassMethod): a
//! getter declared `: array` that returns `$this->property` gets the property's `@var` array type as `@return`.

use mago_syntax::cst::{Node, NodeKind};

use super::docblock::{
    class_members, class_name, docs_report, is_array_hint, method, methods, properties,
    returned_this_property,
};
use crate::rules::phpdoc_writer::type_mapper::is_useful_array_tag;
use pun_phpstan::phpdoc::Node as DocNode;

use crate::rules::phpdoc_writer::{DocTarget, decorate_return_node};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddGetterReturnPhpdoc;

impl Rule for AddGetterReturnPhpdoc {
    fn id(&self) -> &'static str {
        "lint/types/add-getter-return-phpdoc"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some((members, false, span)) = class_members(node) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let class = class_name(&scope);
        let mut changes = Vec::new();
        for m in methods(members) {
            if !m.return_type_hint.as_ref().is_some_and(|r| is_array_hint(&r.hint)) {
                continue;
            }
            let target = DocTarget::of(ctx.source, node_span(m).start);
            let mut info = target.info(ctx.source);
            if is_useful_array_tag(info.return_tag().map(|t| &t.ty)) {
                continue;
            }
            // `GetterClassMethodPropertyFinder::find()`: the property, or else a promoted parameter of the constructor
            let Some(name) = returned_this_property(m) else {
                continue;
            };
            let start = match properties(members).iter().find(|p| p.names.contains(&name)) {
                Some(p) => p.start,
                None => {
                    let Some(param) = method(members, "__construct").and_then(|c| {
                        c.parameter_list.parameters.iter().find(|p| {
                            p.is_promoted_property() && &p.variable.name[1..] == name.as_bytes()
                        })
                    }) else {
                        continue;
                    };
                    node_span(param).start
                }
            };
            let property_info = DocTarget::of(ctx.source, start).info(ctx.source);
            let Some(var) = property_info.var_tag() else {
                continue;
            };
            // is type useful?
            if !matches!(var.parsed, Some(DocNode::Generic { .. } | DocNode::Array(_))) {
                continue;
            }
            let t = DocTarget::doc_type(var, &scope, class.as_deref());
            if decorate_return_node(&mut info, var.ty.clone(), &t) {
                changes.push((target, info));
            }
        }
        if !changes.is_empty() {
            out.extend(docs_report(
                span,
                "Add the `@return` array type of the getter from its property's `@var`.",
                ctx.source,
                &changes,
            ));
        }
    }
}
