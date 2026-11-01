//! `lint/types/prefer-specific-return-type`, ported from Rector 2.6.1's `NarrowObjectReturnTypeRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): a method of a final class, or a final method, declared to return
//! `object` or a class that is neither final nor an interface, whose every `return` returns an object of one class
//! below it, returns that class instead. A `@return` naming one type takes the class too, unless it names it already.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Hint, Modifier, Node, NodeKind};
use pun_phpstan::types::{Type, is_super_type_of};
use pun_problem::{Edit, Safety};

use super::compare::classes::with_classes;
use super::compare::mapper::{TypeScope, expr_type};
use super::compare::{RType, are_types_equal};
use super::docblock::{FunctionLike, class_name};
use super::facts::{Facts, facts, is_builtin_class};
use super::names::resolve_class_name;
use super::{ancestor_names, class_scope, returns_scoped};
use crate::rules::phpdoc_writer::{DocTarget, TypeNode};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferSpecificReturnType;

impl Rule for PreferSpecificReturnType {
    fn id(&self) -> &'static str {
        "lint/types/prefer-specific-return-type"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
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
        let (Some(m), Some(return_type)) = (f.method, f.return_type) else {
            return;
        };
        // an `Identifier` or a `FullyQualified` name, as php-parser's name resolver leaves a class name
        let declared = match &return_type.hint {
            Hint::Object(_) => "object".to_string(),
            Hint::Identifier(id) => resolve_class_name(ancestors, id),
            _ => return,
        };
        let scope = class_scope(ancestors).expect("a method is in a class-like");
        let is_final = |modifiers: &mago_syntax::cst::Sequence<'_, Modifier<'_>>| {
            modifiers.iter().any(|m| matches!(m, Modifier::Final(_)))
        };
        let final_by_keyword = scope.is_final && !scope.is_interface;
        if !final_by_keyword && !is_final(&m.modifiers) {
            return;
        }
        let facts = facts(ctx);
        let Some(actual) = actual_returned_class(&facts, &f, ancestors) else {
            return;
        };
        // already most narrow type
        if declared == actual {
            return;
        }
        let codebase = &facts.codebase;
        if declared != "object" {
            // non-existing class, already last final object, or no class (this rule narrows only object or class
            // types, not interfaces)
            let Some(class) = codebase.get_class_like(declared.as_bytes()) else {
                return;
            };
            if class.kind != SymbolKind::Class || class.flags.is_final() {
                return;
            }
        }
        let target = f.target(ctx.source);
        let info = target.doc.map(|_| target.info(ctx.source));
        let class = class_name(ancestors);
        // isNarrowingValid
        if declared != "object" {
            if !is_instance_of(codebase, &actual, &declared) {
                return;
            }
            let generic = info.as_ref().and_then(|i| i.return_tag()).is_some_and(|tag| {
                matches!(DocTarget::doc_type(tag, ancestors, class.as_deref()), Type::Object(_, args, _) if !args.is_empty())
            });
            if generic {
                return;
            }
        }
        let is_private = m
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Private(_)));
        if !is_private && !is_narrowing_valid_from_parent(&facts, &scope, f.name_text, &actual) {
            return;
        }
        let mut edits = vec![Edit::new(
            node_span(&return_type.hint),
            format!("\\{actual}"),
        )];
        // updateDocblock
        if let Some(mut info) = info
            && let Some(tag) = info.return_tag()
            && let TypeNode::Identifier(_) = tag.ty
            && let Some(parsed) = &tag.parsed
        {
            let mut chain = ancestors.to_vec();
            chain.push(node);
            let type_scope = TypeScope::new(&chain, facts.documented());
            let same = with_classes(facts.documented(), || {
                let old = type_scope.map_doc(parsed);
                old.is_object_type()
                    && old.class_name().is_some()
                    && are_types_equal(&old, &RType::T(Type::Object(actual.clone(), vec![], false)))
            });
            if !same {
                info.set_return_type(TypeNode::Identifier(format!("\\{actual}")));
                edits.push(target.edit(ctx.source, &info.print()));
            }
        }
        out.push(Report::fixed(
            f.name,
            format!("Narrow the return type to `{actual}`, the class every `return` returns."),
            Safety::Unsafe,
            edits,
        ));
    }
}

/// `ObjectType($parent)->isSuperTypeOf(ObjectType($child))->yes()` between two class names.
fn is_instance_of(codebase: &CodebaseMetadata, child: &str, parent: &str) -> bool {
    codebase.get_class_like(child.as_bytes()).is_some()
        && codebase.is_instance_of(child.as_bytes(), parent.as_bytes())
}

/// `getActualReturnedClass()`: the one class whose objects every `return` returns natively, not an anonymous class.
fn actual_returned_class(
    facts: &Facts,
    f: &FunctionLike<'_, '_>,
    ancestors: &[Node<'_, '_>],
) -> Option<String> {
    let returns = returns_scoped(f.body?);
    let mut returned: Option<String> = None;
    for r in &returns {
        let t = expr_type(facts, ancestors, r.value?, true);
        if !is_super_type_of(&Type::ObjectWithoutClass, &t).yes() {
            return None;
        }
        let [class] = &t.object_class_names()[..] else {
            return None;
        };
        // skip anonymous classes, they have no usable name
        if class.starts_with("class@anonymous") || class.starts_with("{anonymous-class") {
            return None;
        }
        let class = facts
            .codebase
            .get_class_like(class.as_bytes())
            .map_or_else(|| class.clone(), |c| c.original_name.to_string());
        match &returned {
            None => returned = Some(class),
            Some(r) if *r != class => return None,
            Some(_) => {}
        }
    }
    returned
}

/// `isNarrowingValidFromParent()`: every ancestor (not a built-in class) with the method, whose native return type is
/// one class, takes the actual class, and does not document a generic object.
fn is_narrowing_valid_from_parent(
    facts: &Facts,
    scope: &super::ClassScope,
    method: &[u8],
    actual: &str,
) -> bool {
    for ancestor in ancestor_names(facts, scope) {
        if Some(&ancestor) == scope.name.as_ref() || is_builtin_class(ancestor.as_bytes()) {
            continue;
        }
        let Some(native) = facts
            .codebase
            .get_declaring_method(ancestor.as_bytes(), method)
        else {
            continue;
        };
        let Some(parent) = native
            .return_type_declaration_metadata
            .as_ref()
            .and_then(|t| object_type(&t.type_union))
        else {
            continue;
        };
        if !is_instance_of(&facts.codebase, actual, &parent) {
            return false;
        }
        let generic = (facts.documented().get_declaring_method(ancestor.as_bytes(), method))
            .and_then(|m| m.return_type_metadata.as_ref())
            .is_some_and(|t| {
                matches!(&t.type_union.types[..], [TAtomic::Object(TObject::Named(n))] if n.type_parameters.is_some())
            });
        if generic {
            return false;
        }
    }
    true
}

/// The class of a native return type that is an `ObjectType`: one class, not `static` (a parent's enum return type
/// leaves no class to narrow to, as a child may only declare the enum too).
fn object_type(t: &TUnion) -> Option<String> {
    match &t.types[..] {
        [TAtomic::Object(TObject::Named(n))] if !n.is_static && n.intersection_types.is_none() => {
            Some(n.name.to_string())
        }
        _ => None,
    }
}
