//! `lint/types/add-return-phpdoc-direct-array`, ported from Rector 2.6.1's
//! `DocblockReturnArrayFromDirectArrayInstanceRector` (rules/TypeDeclarationDocblocks/Rector/ClassMethod): a function
//! or method whose one `return` returns an array literal gets `@return array<key, item>`, the literal's type
//! generalized (`ConstantArrayTypeGeneralizer`).

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_phpstan::types::Type;

use super::docblock::{FunctionLike, doc_report, referenced_classes};
use super::facts::facts;
use super::never::has_parent_class_method;
use super::silent_void::Place;
use super::{class_scope};
use crate::rules::phpdoc_writer::type_mapper::{generalize_constant_array, is_useful_array_tag};
use crate::rules::phpdoc_writer::{TypeNode, expr_type};
use crate::{Context, Report, Rule};

pub struct AddReturnPhpdocDirectArray;

impl Rule for AddReturnPhpdocDirectArray {
    fn id(&self) -> &'static str {
        "lint/types/add-return-phpdoc-direct-array"
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
        if f.body.is_none() {
            return;
        }
        let facts = facts(ctx);
        let scope = class_scope(ancestors);
        // skip overridden methods to not conflict with parent/interface @return docblock
        if f.method.is_some()
            && !f.is_private()
            && scope
                .as_ref()
                .is_none_or(|s| has_parent_class_method(&facts, s, f.name_text))
        {
            return;
        }
        let target = f.target(ctx.source);
        let Some(mut info) = target.info(ctx.source) else {
            return;
        };
        if is_useful_array_tag(info.return_tag().map(|t| &t.ty)) {
            return;
        }
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let Some(value) = f.only_return_with_expr(&place).and_then(|r| r.value) else {
            return;
        };
        let empty = match value {
            Expression::Array(a) => a.elements.is_empty(),
            Expression::LegacyArray(a) => a.elements.is_empty(),
            _ => return,
        };
        // `shouldSkipReturnMixedAndEmptyArray()`: `T[]` is better than `array{}`
        if empty && matches!(info.return_tag().map(|t| &t.ty), Some(TypeNode::Array { .. })) {
            return;
        }
        let Some(Type::ConstArray(returned)) = expr_type(ctx, ancestors, value) else {
            return;
        };
        // better handled by shared-interface/class rule, to avoid turning objects to mixed
        let classes = referenced_classes(&Type::ConstArray(returned.clone()));
        if !classes.is_empty() && classes.len() == returned.values.len() {
            return;
        }
        info.set_return_type(generalize_constant_array(&returned));
        out.push(doc_report(
            f.name,
            "Add the `@return` array type of the returned array.",
            ctx.source,
            &target,
            &info,
        ));
    }
}
