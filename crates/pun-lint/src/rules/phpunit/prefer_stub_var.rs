//! `lint/phpunit/prefer-stub-var`, ported from rector-phpunit's `MockObjectVarToStubRector`
//! (rules/PHPUnit120/Rector/Property).
//!
//! In a test class, a property whose native type is `PHPUnit\Framework\MockObject\Stub` (or an intersection with it)
//! gets `\PHPUnit\Framework\MockObject\Stub` for each `MockObject` (by its short name) in the type of its first `@var`
//! tag: the type itself, or a name in a union or an intersection. Rector runs it only for PHPUnit 11 and later; Pun
//! leaves that to the rule's group.

use mago_syntax::cst::{Hint, Node, NodeKind, Property};
use pun_problem::Safety;

use super::annotations::Annotated;
use super::phpdoc_type::{TagType, short_name, typed_tag};
use super::{is_in_test_class, report, resolve};
use crate::{Context, Report, Rule};

/// `PHPUnitClassName::STUB`.
const STUB: &str = "PHPUnit\\Framework\\MockObject\\Stub";

pub struct PreferStubVar;

impl Rule for PreferStubVar {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-stub-var"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Property])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Property(property) = node else {
            return;
        };
        // only inside PHPUnit TestCase scope
        if !is_in_test_class(ctx, ancestors, None) {
            return;
        }
        // only properties already converted to a Stub native type
        let hint = match property {
            Property::Plain(p) => p.hint.as_ref(),
            Property::Hooked(p) => p.hint.as_ref(),
        };
        let is_stub =
            |h: &Hint<'_>| matches!(h, Hint::Identifier(id) if resolve(ancestors, id) == STUB);
        let stub_native = match hint {
            Some(h @ Hint::Intersection(_)) => {
                let mut stack = vec![h];
                let mut any = false;
                while let Some(h) = stack.pop() {
                    match h {
                        Hint::Intersection(i) => stack.extend([i.left, i.right]),
                        other => any |= is_stub(other),
                    }
                }
                any
            }
            Some(h) => is_stub(h),
            None => false,
        };
        if !stub_native {
            return;
        }
        let Some(annotated) = Annotated::of(ctx.source, node, ancestors) else {
            return;
        };
        let Some(doc) = annotated.doc.as_ref() else {
            return;
        };
        let Ok(Some(tag)) = typed_tag(doc, "@var", None) else {
            return;
        };
        let stub = format!("\\{STUB}");
        let replaced = |m: &&str| {
            if !m.ends_with(']') && short_name(m) == "MockObject" {
                stub.clone()
            } else {
                m.to_string()
            }
        };
        let new_type = match &tag.ty {
            TagType::Union(members) | TagType::Intersection(members) => {
                let printed: Vec<String> = members.iter().map(replaced).collect();
                if printed.iter().zip(members).all(|(p, m)| p == m) {
                    return;
                }
                printed.join(if matches!(tag.ty, TagType::Union(_)) {
                    "|"
                } else {
                    "&"
                })
            }
            TagType::Name(name) if short_name(name) == "MockObject" => stub,
            _ => return,
        };
        let text = doc.text_all();
        let child = doc.text(tag.index).trim_end();
        let from = child.as_ptr() as usize - text.as_ptr() as usize;
        let new_doc = format!(
            "{}{}{}",
            &text[..from],
            tag.print_with(&new_type),
            &text[from + child.len()..]
        );
        let Some(edits) = annotated.edits(ctx.source, Some(&new_doc), &[]) else {
            return;
        };
        out.push(report(
            doc.span,
            "Type the stub property's @var as a Stub.",
            Safety::Safe,
            edits,
        ));
    }
}
