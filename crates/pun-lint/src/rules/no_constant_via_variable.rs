//! `lint/complexity/no-constant-via-variable`, ported from Rector 2.6.1's `VariableConstFetchToClassConstFetchRector`
//! (rules/CodeQuality/Rector/ClassConstFetch).
//!
//! `$value::NAME`, where the native type of `$value` is one known class that has the constant, becomes
//! `\Fully\Qualified\Class::NAME` when that class is final by keyword (an enum is) or the constant it has is
//! (`final public const`). A docblock's `@final` does not count.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{ClassLikeConstantSelector, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoConstantViaVariable;

impl Rule for NoConstantViaVariable {
    fn id(&self) -> &'static str {
        "lint/complexity/no-constant-via-variable"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ClassConstantAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ClassConstantAccess(fetch) = node else {
            return;
        };
        let Expression::Variable(_) = fetch.class else {
            return;
        };
        let ClassLikeConstantSelector::Identifier(constant) = &fetch.constant else {
            return;
        };
        let facts = facts(ctx);
        let Some(t) = facts.native_type(node_span(fetch.class)) else {
            return;
        };
        let codebase = &facts.codebase;
        // Mago gives a value of a class it does not know no named type, so a named one is known
        let metadata = match &t.types[..] {
            [TAtomic::Object(TObject::Named(n))] if !n.is_this && !n.is_static => {
                codebase.get_class_like(n.name.as_bytes())
            }
            [TAtomic::Object(TObject::Enum(e))] => codebase.get_class_like(e.get_name().as_bytes()),
            _ => None,
        };
        let Some(metadata) = metadata else { return };
        // `hasConstant()`: the class, a parent or an interface declares it, or an enum has the case
        let name = constant.value;
        let declared = std::iter::once(&metadata.name)
            .chain(metadata.all_parent_classes.iter())
            .chain(metadata.all_parent_interfaces.iter())
            .find_map(|c| codebase.get_class_constant(c.as_bytes(), name));
        let is_case = codebase
            .get_enum_case(metadata.name.as_bytes(), name)
            .is_some();
        if declared.is_none() && !is_case {
            return;
        }
        if !metadata.flags.is_final() && !declared.is_some_and(|c| c.flags.is_final()) {
            return;
        }
        let span = node_span(fetch.class);
        let text = format!(
            "\\{}",
            String::from_utf8_lossy(metadata.original_name.as_bytes())
        );
        out.push(Report {
            span,
            message: "This constant is the same for every value of the class; name the class."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, text)],
            }),
        });
    }
}
