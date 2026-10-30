//! `lint/types/add-var-phpdoc-from-default`, ported from Rector 2.6.1's `DocblockVarArrayFromPropertyDefaultsRector`
//! (rules/TypeDeclarationDocblocks/Rector/Class_): a property declared `array` with an array literal default gets the
//! default's generalized type as its `@var`, unless a parent class's property already documents a wider one.

use mago_codex::visibility::Visibility;
use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_phpstan::types::{Type, is_super_type_of};

use super::docblock::{class_members, class_name, docs_report, is_array_hint, properties};
use super::facts::{Facts, facts};
use crate::rules::phpdoc_writer::type_mapper::is_useful_array_tag;
use crate::rules::phpdoc_writer::{DocTarget, decorate_var, expr_type, phpstan_type};
use crate::syntax::node_span;
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
        let class = class_name(&scope);
        let facts = facts(ctx);
        let mut changes = Vec::new();
        for property in properties(members) {
            if !property.hint.is_some_and(is_array_hint) || property.names.len() > 1 {
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
            let target = DocTarget::of(ctx.source, property.start);
            let mut info = target.info(ctx.source);
            // type is already known
            if is_useful_array_tag(info.var_tag().map(|t| &t.ty)) {
                continue;
            }
            if !property.is_private
                && class.as_deref().is_some_and(|c| {
                    has_useful_parent_property_var_tag(&facts, c, property.names[0], &default_type)
                })
            {
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

/// `hasUsefulParentPropertyVarTag()`: a parent class declares the property with a useful `@var` whose type takes the
/// default's.
fn has_useful_parent_property_var_tag(facts: &Facts, class: &str, name: &str, default: &Type) -> bool {
    let codebase = facts.documented();
    let mut current = codebase
        .get_class_like(class.as_bytes())
        .and_then(|c| c.direct_parent_class);
    let property = format!("${name}");
    while let Some(parent) = current {
        let Some(parent_class) = codebase.get_class_like(parent.as_bytes()) else {
            return false;
        };
        current = parent_class.direct_parent_class;
        let Some(p) = codebase.get_property(parent.as_bytes(), property.as_bytes()) else {
            continue;
        };
        if p.read_visibility == Visibility::Private {
            return false;
        }
        let Some(doc_type) = p.type_metadata.as_ref().filter(|t| t.from_docblock) else {
            continue;
        };
        // `resolveVarTagValueNode()`: the parent property's own doc comment
        let useful = p.name_span.and_then(|span| {
            let source = facts.source_of(span.file_id)?;
            let start = property_start(&source, span.start.offset as usize)?;
            let info = DocTarget::of(&source, start).info(&source);
            Some(is_useful_array_tag(info.var_tag().map(|t| &t.ty)))
        });
        if useful != Some(true) {
            continue;
        }
        if phpstan_type(&doc_type.type_union).is_some_and(|t| is_super_type_of(&t, default).yes()) {
            return true;
        }
    }
    false
}

/// Where the property statement whose name is at `name` starts in `source` (Mago keeps only a plain property's name
/// span), for `AstResolver::resolvePropertyFromPropertyReflection()`.
fn property_start(source: &str, name: usize) -> Option<usize> {
    let arena = mago_allocator::LocalArena::new();
    let file = mago_database::file::File::ephemeral(
        std::borrow::Cow::Borrowed(b"parent.php"),
        std::borrow::Cow::Owned(source.as_bytes().to_vec()),
    );
    let program = mago_syntax::parser::parse_file(&arena, &file);
    fn find(node: Node<'_, '_>, name: usize, out: &mut Option<usize>) {
        node.visit_children(|child| {
            let span = node_span(&child);
            if out.is_some() || !(span.start..span.end).contains(&name) {
                return;
            }
            if let Node::Property(_) = child {
                *out = Some(span.start);
            } else {
                find(child, name, out);
            }
        });
    }
    let mut out = None;
    find(Node::Program(program), name, &mut out);
    out
}
