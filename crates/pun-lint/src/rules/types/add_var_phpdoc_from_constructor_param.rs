//! `lint/types/add-var-phpdoc-from-constructor-param`, ported from Rector 2.8.4's
//! `DocblockVarFromParamDocblockInConstructorRector` (rules/TypeDeclarationDocblocks/Rector/Class_): a property
//! declared `array` that the constructor assigns gets the assigned value's array type as its `@var`.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_phpstan::types::Type;

use super::docblock::{
    assignments, class_members, docs_report, is_array_hint, method, properties, this_property,
};
use crate::rules::phpdoc_writer::type_mapper::is_useful_array_tag;
use crate::rules::phpdoc_writer::{DocTarget, decorate_var, expr_type};
use crate::{Context, Report, Rule};

pub struct AddVarPhpdocFromConstructorParam;

impl Rule for AddVarPhpdocFromConstructorParam {
    fn id(&self) -> &'static str {
        "lint/types/add-var-phpdoc-from-constructor-param"
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
        let Some(constructor) = method(members, "__construct") else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        scope.push(Node::Method(constructor));
        let mut changes = Vec::new();
        for property in properties(members) {
            if !property.hint.is_some_and(is_array_hint) {
                continue;
            }
            let target = DocTarget::of(ctx, property.start);
            let mut info = target.info(ctx.source);
            // @var tag already given
            if is_useful_array_tag(info.var_tag().map(|t| &t.ty)) {
                continue;
            }
            // `ConstructorAssignedTypeResolver::resolve()`: the type of the first `$this->property = ...`
            let assigned: Option<&Expression<'_>> = assignments(Node::Method(constructor))
                .into_iter()
                .find(|a| this_property(a.lhs) == Some(property.names[0]))
                .map(|a| a.rhs);
            let Some(t @ Type::Array(..)) = assigned.and_then(|e| expr_type(ctx, &scope, e)) else {
                continue;
            };
            if decorate_var(&mut info, &t) {
                changes.push((target, info));
            }
        }
        if !changes.is_empty() {
            out.extend(docs_report(
                span,
                "Add the `@var` array type of the property from the constructor's assignment.",
                ctx.source,
                &changes,
            ));
        }
    }
}
