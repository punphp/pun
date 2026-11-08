//! `lint/types/add-var-phpdoc-from-dim-fetch`, ported from Rector 2.8.4's `AddVarArrayDocblockFromDimFetchAssignRector`
//! (rules/TypeDeclarationDocblocks/Rector/Class_): a property declared `array` that the class's methods append to
//! (`$this->items[] = ...`) gets the appended values' type as its `@var`.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_phpstan::types::{Type, mixed};

use super::docblock::{
    assignments, class_members, docs_report, is_array_hint, methods, properties, this_property,
};
use crate::rules::phpdoc_writer::type_mapper::{is_useful_array_tag, uniquate_types};
use crate::rules::phpdoc_writer::{DocTarget, decorate_var, expr_type};
use crate::{Context, Report, Rule};

pub struct AddVarPhpdocFromDimFetch;

impl Rule for AddVarPhpdocFromDimFetch {
    fn id(&self) -> &'static str {
        "lint/types/add-var-phpdoc-from-dim-fetch"
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
        let mut changes = Vec::new();
        for property in properties(members) {
            if !property.hint.is_some_and(is_array_hint) {
                continue;
            }
            let target = DocTarget::of(ctx, property.start);
            let mut info = target.info(ctx.source);
            if is_useful_array_tag(info.var_tag().map(|t| &t.ty)) {
                continue;
            }
            // `hasNonAppendAssignToPropertyName()`: a keyed dim assign (`$this->prop['x'] = ...`) or a direct assign
            // carries a value type this rule does not read from bare appends; skip to avoid a too-narrow `@var`
            let non_append = methods(members).any(|m| {
                assignments(Node::Method(m)).into_iter().any(|a| match a.lhs {
                    Expression::ArrayAccess(access) => this_property(access.array) == Some(property.names[0]),
                    lhs => this_property(lhs) == Some(property.names[0]),
                })
            });
            if non_append {
                continue;
            }
            // `ArrayDimFetchFinder::findDimFetchAssignToPropertyName()`: `$this->name[] = expr` in the methods
            let mut types: Vec<Option<Type>> = Vec::new();
            for m in methods(members) {
                for a in assignments(Node::Method(m)) {
                    let Expression::ArrayAppend(append) = a.lhs else {
                        continue;
                    };
                    if this_property(append.array) != Some(property.names[0]) {
                        continue;
                    }
                    let mut method_scope = scope.clone();
                    method_scope.push(Node::Method(m));
                    types.push(expr_type(ctx, &method_scope, a.rhs));
                }
            }
            // nothing to add (or a type Pun does not map)
            let Some(types) = types.into_iter().collect::<Option<Vec<Type>>>().filter(|t| !t.is_empty()) else {
                continue;
            };
            let mut unique = uniquate_types(&types);
            let item = if unique.len() > 1 {
                Type::Union(unique)
            } else {
                unique.remove(0)
            };
            if decorate_var(&mut info, &Type::array(mixed(), item)) {
                changes.push((target, info));
            }
        }
        if !changes.is_empty() {
            out.extend(docs_report(
                span,
                "Add the `@var` array type of the property from the values appended to it.",
                ctx.source,
                &changes,
            ));
        }
    }
}
