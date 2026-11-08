//! `lint/types/add-return-type-from-parent`, ported from Rector 2.8.4's
//! `AddReturnTypeDeclarationBasedOnParentClassMethodRector` (rules/TypeDeclaration/Rector/ClassMethod): a method of a
//! class without a return type gets the native return type of the method it overrides (found by
//! `ParentClassMethodTypeOverrideGuard::getParentClassMethod`), or of the first one up the chain that declares one.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_span::HasSpan;
use mago_syntax::cst::{Method, Modifier, Node, NodeKind};

use super::declare::{return_declaration, text_order};
use super::facts::{Facts, facts};
use super::{SCALAR_TYPES, add_return_type, class_scope};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromParent;

/// `PhpVersionFeature::MIXED_TYPE`.
const MIXED_TYPE: u32 = 80000;

impl Rule for AddReturnTypeFromParent {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-parent"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
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
        let Node::Method(
            m @ Method {
                return_type_hint: None,
                ..
            },
        ) = node
        else {
            return;
        };
        // a method of a class (`Class_`, anonymous ones too)
        let Some(class_like @ (Node::Class(_) | Node::AnonymousClass(_))) =
            ancestors.iter().rev().find(|a| {
                matches!(
                    a,
                    Node::Class(_)
                        | Node::AnonymousClass(_)
                        | Node::Interface(_)
                        | Node::Trait(_)
                        | Node::Enum(_)
                )
            })
        else {
            return;
        };
        let name = m.name.value.to_ascii_lowercase();
        if name == b"__construct" || name == b"__destruct" {
            return;
        }
        if m.modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Private(_)))
        {
            return;
        }
        let facts = facts(ctx);
        let class = match class_like {
            Node::AnonymousClass(c) => facts.anonymous_class_name(c.span()),
            // a named class always has its name
            _ => class_scope(ancestors)
                .and_then(|s| s.name)
                .unwrap_or_default(),
        };
        let Some(r#type) = parent_return_type(&facts, &class, &name, ctx.php_version) else {
            return;
        };
        out.push(add_return_type(
            node_span(&m.name),
            node_span(&m.parameter_list).end,
            &r#type,
        ));
    }
}

/// `getReturnTypeRecursive` and `processClassMethodReturnType`: up the methods each one overrides, the first native
/// return type other than an implicit `mixed`, as a declaration.
fn parent_return_type(
    facts: &Facts,
    class: &str,
    method: &[u8],
    php_version: u32,
) -> Option<String> {
    let codebase = &facts.codebase;
    let mut parent = parent_method(codebase, class, method)?;
    // a bound for circular parents (Pun/parent_cycle), which Mago keeps
    for _ in 0..64 {
        if parent
            .method_metadata
            .as_ref()
            .is_some_and(|m| m.visibility.is_private())
        {
            return None;
        }
        let owner = declaring_class(codebase, class, parent)?;
        // PHPStan 2.3 reads a built-in method's tentative return type as its native one (`applyTentativeReturnType`)
        if let Some(declared) = parent.return_type_declaration_metadata.as_ref() {
            if declared.type_union.is_mixed() {
                return (php_version >= MIXED_TYPE).then(|| "mixed".to_string());
            }
            let order = facts
                .source_of(declared.span.file_id)
                .and_then(|s| {
                    s.get(declared.span.start.offset as usize..declared.span.end.offset as usize)
                        .map(text_order)
                })
                .unwrap_or_default();
            let types: Vec<TAtomic> = declared
                .type_union
                .types
                .iter()
                .map(|t| parent_to_class(codebase, t, &owner))
                .collect();
            return return_declaration(types.iter(), codebase, php_version, &order, true);
        }
        parent = parent_method(codebase, &owner, method)?;
    }
    None
}

/// The class that declares `method`, found among `class`'s ancestors.
fn declaring_class(
    codebase: &CodebaseMetadata,
    class: &str,
    method: &FunctionLikeMetadata,
) -> Option<String> {
    let meta = codebase.get_class_like(class.as_bytes())?;
    let lowercase = method.name.as_bytes().to_ascii_lowercase();
    std::iter::once(&meta.name)
        .chain(meta.all_parent_classes.iter())
        .chain(meta.all_parent_interfaces.iter())
        .chain(meta.used_traits.iter())
        .find(|c| {
            codebase
                .get_method(c.as_bytes(), &lowercase)
                .is_some_and(|m| std::ptr::eq(m, method))
        })
        .map(|c| String::from_utf8_lossy(c.as_bytes()).into_owned())
}

/// `parent` in a declaration names the parent class of the class that declares it (PHPStan resolves it; Mago keeps
/// the word).
fn parent_to_class(codebase: &CodebaseMetadata, t: &TAtomic, owner: &str) -> TAtomic {
    if let TAtomic::Object(TObject::Named(n)) = t
        && n.name.as_bytes().eq_ignore_ascii_case(b"parent")
        && let Some(parent) = codebase
            .get_class_like(owner.as_bytes())
            .and_then(|c| c.direct_parent_class)
    {
        return TAtomic::Object(TObject::Named(TNamedObject::new(parent)));
    }
    t.clone()
}

/// `ParentClassMethodTypeOverrideGuard::resolveParentClassMethod` for a method that is not private: the method of
/// the first parent class that has one (inherited ones too), else of the first interface that has one, else an
/// abstract method of a used trait. `None` when a parent class is not known.
fn parent_method<'c>(
    codebase: &'c CodebaseMetadata,
    class: &str,
    method: &[u8],
) -> Option<&'c FunctionLikeMetadata> {
    let meta = codebase.get_class_like(class.as_bytes())?;
    for parent in &meta.all_parent_classes {
        if let Some(m) = codebase.get_declaring_method(parent.as_bytes(), method) {
            return Some(m);
        }
    }
    for interface in &meta.all_parent_interfaces {
        if let Some(m) = codebase.get_declaring_method(interface.as_bytes(), method) {
            return Some(m);
        }
    }
    for r#trait in &meta.used_traits {
        if let Some(m) = codebase.get_declaring_method(r#trait.as_bytes(), method) {
            return m
                .method_metadata
                .as_ref()
                .is_some_and(|mm| mm.is_abstract)
                .then_some(m);
        }
    }
    None
}
