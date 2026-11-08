//! `lint/types/add-var-phpdoc-from-default`, ported from Rector 2.8.4's `DocblockVarArrayFromPropertyDefaultsRector`
//! (rules/TypeDeclarationDocblocks/Rector/Class_): a property declared `array` with an array literal default gets the
//! default's generalized type as its `@var` when the property is private (a parent's property then does not count) and
//! the class has a name.

use mago_syntax::cst::{Expression, Node, NodeKind};

use super::docblock::{class_members, class_name, docs_report, is_array_hint, properties};
use crate::rules::phpdoc_writer::type_mapper::is_useful_array_tag;
use crate::rules::phpdoc_writer::{DocTarget, decorate_var, expr_type};
use crate::{Context, Report, Rule};

pub struct AddVarPhpdocFromDefault;

impl Rule for AddVarPhpdocFromDefault {
    fn id(&self) -> &'static str {
        "lint/types/add-var-phpdoc-from-default"
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
        // `fetchClassLikeReflection()`: an anonymous class has no name, and Rector leaves it alone
        if class_name(&scope).is_none() {
            return;
        }
        let mut changes = Vec::new();
        for property in properties(members) {
            // only a private property is safe: a child class can assign a protected or public one a wider type (and
            // a private one overrides no parent's, so `hasUsefulParentPropertyVarTag()` never stops it)
            if !property.hint.is_some_and(is_array_hint) || !property.is_private || property.names.len() > 1 {
                continue;
            }
            // an array default, of a type Pun maps
            let Some(default_type) = property
                .default
                .filter(|d| matches!(d, Expression::Array(_) | Expression::LegacyArray(_)))
                .and_then(|d| expr_type(ctx, &scope, d))
            else {
                continue;
            };
            let target = DocTarget::of(ctx, property.start);
            let mut info = target.info(ctx.source);
            // type is already known
            if is_useful_array_tag(info.var_tag().map(|t| &t.ty)) {
                continue;
            }
            if decorate_var(&mut info, &default_type) {
                changes.push((target, info));
            }
        }
        if !changes.is_empty() {
            out.extend(docs_report(
                span,
                "Add the `@var` array type of the property from its default.",
                ctx.source,
                &changes,
            ));
        }
    }
}
