//! `lint/types/add-var-phpdoc-from-getter`, ported from Rector 2.8.4's `DocblockVarArrayFromGetterReturnRector`
//! (rules/TypeDeclarationDocblocks/Rector/Class_): a property declared `array` whose getter has a `@return` gets
//! that type as its `@var`.

use mago_syntax::cst::{Node, NodeKind};

use super::MAGIC;
use super::docblock::{
    class_members, class_name, docs_report, is_array_hint, method_statements, methods,
    properties, returned_this_property,
};
use crate::rules::phpdoc_writer::type_mapper::is_useful_array_tag;
use crate::rules::phpdoc_writer::{DocTarget, decorate_var};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddVarPhpdocFromGetter;

impl Rule for AddVarPhpdocFromGetter {
    fn id(&self) -> &'static str {
        "lint/types/add-var-phpdoc-from-getter"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some((members, _, span)) = class_members(node) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let class = class_name(&scope);
        let mut changes = Vec::new();
        for property in properties(members) {
            // property type must be known
            if !property.hint.is_some_and(is_array_hint) || property.names.len() > 1 {
                continue;
            }
            let target = DocTarget::of(ctx, property.start);
            let mut info = target.info(ctx.source);
            // type is already known, skip it
            if is_useful_array_tag(info.var_tag().map(|t| &t.ty)) {
                continue;
            }
            // `PropertyGetterFinder::find()`
            let getter = methods(members).find(|m| {
                let name = String::from_utf8_lossy(m.name.value).to_ascii_lowercase();
                !MAGIC.contains(&name.as_str())
                    && method_statements(m).is_some()
                    && returned_this_property(m) == Some(property.names[0])
            });
            let Some(getter) = getter else {
                continue;
            };
            let getter_info = DocTarget::of(ctx, node_span(getter).start).info(ctx.source);
            let Some(tag) = getter_info.return_tag() else {
                continue;
            };
            let t = DocTarget::doc_type(tag, &scope, class.as_deref());
            if decorate_var(&mut info, &t) {
                changes.push((target, info));
            }
        }
        if !changes.is_empty() {
            out.extend(docs_report(
                span,
                "Add the `@var` array type of the property from its getter's `@return`.",
                ctx.source,
                &changes,
            ));
        }
    }
}
