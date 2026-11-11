//! `lint/dead-code/no-empty-method`, ported from Rector 2.8.4's `RemoveEmptyClassMethodRector`
//! (rules/DeadCode/Rector/ClassMethod), with `IsClassMethodUsedAnalyzer`, `ClassMethodManipulator` and
//! `ControllerClassMethodManipulator` in `class_method.rs`.
//!
//! A method of a class (or anonymous class) with an empty body goes, unless it is abstract, a `final` method of a
//! class that is not final, a public or protected method that is not magic in a class that is not final, called in
//! the class or one of its traits, a named constructor, the constructor of an anonymous class that extends another,
//! required by a parent or an interface, a constructor with promoted parameters, `@deprecated`, a controller's or
//! presenter's annotated action, a private or protected `__clone`, `__invoke`, the documented constructor of an
//! attribute class, or (Pun's own skip) a method that carries an attribute. A comment alone in the body counts as a statement (php-parser's `Nop`). Rector's
//! `treatClassesAsFinal` feature flag is off by default and has no Pun setting.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::class_method::{
    ClassInfo, doc_tags, has_parent_method_or_interface_method, is_class_method_used,
    is_controller_class_method, is_final, is_magic, is_named_constructor, is_private, is_public,
    member_doc, with_class_info,
};
use super::rector_batch5::{has_attribute, remove_members};
use super::rector_stmts::line_indent;
use super::types::facts::facts;
use super::types::statement_count;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoEmptyMethod;

impl Rule for NoEmptyMethod {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-empty-method"
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
        let facts = facts(ctx);
        with_class_info(node, ancestors, &facts, |class| {
            let mut removed = vec![false; class.members.len()];
            let mut methods: Vec<&Method<'_>> = Vec::new();
            for (index, member) in class.members.iter().enumerate() {
                let ClassLikeMember::Method(method) = member else {
                    continue;
                };
                let MethodBody::Concrete(body) = &method.body else {
                    continue;
                };
                if statement_count(ctx.source, body) > 0
                    || method
                        .modifiers
                        .iter()
                        .any(|m| matches!(m, Modifier::Abstract(_)))
                {
                    continue;
                }
                if is_final(method) && !class.is_final {
                    continue;
                }
                // Not Rector's: an attribute (Livewire's `#[On]`, a route, a test hook) makes a framework call the
                // method, so it is not dead.
                if !method.attribute_lists.is_empty() {
                    continue;
                }
                // `shouldSkipNonFinalNonPrivateClassMethod`.
                if !class.is_final && !is_magic(method.name.value) && !is_private(method) {
                    continue;
                }
                if should_skip(ctx, ancestors, &facts, &class, method, index) {
                    continue;
                }
                removed[index] = true;
                methods.push(method);
            }
            if !removed.contains(&true) {
                return;
            }
            let edits = if removed.iter().all(|r| *r) && class.is_anonymous {
                // php-parser prints an anonymous class whose body it empties anew: `{` on a line of its own at the indent
                // of the line the class starts on.
                let open = class.left_brace.start.offset as usize;
                let from = ctx.source[..open].trim_end().len();
                let indent = line_indent(ctx.source, node_span(&node).start);
                vec![Edit::new(
                    Span::new(from, class.right_brace.end.offset as usize),
                    format!("\n{indent}{{\n{indent}}}"),
                )]
            } else {
                remove_members(
                    ctx.source,
                    class.members,
                    class.left_brace,
                    class.right_brace,
                    &removed,
                )
            };
            for method in methods {
                out.push(Report {
                    span: node_span(method),
                    message: format!(
                        "Remove the empty method `{}()`, which nothing requires.",
                        String::from_utf8_lossy(method.name.value)
                    ),
                    help: None,
                    fix: Some(Fix {
                        safety: Safety::Safe,
                        edits: edits.clone(),
                    }),
                });
            }
        });
    }
}

/// `shouldSkipClassMethod`.
fn should_skip(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    facts: &super::types::facts::Facts,
    class: &ClassInfo<'_, '_>,
    method: &Method<'_>,
    index: usize,
) -> bool {
    let name = method.name.value;
    let is = |other: &[u8]| name.eq_ignore_ascii_case(other);
    if is_class_method_used(facts, ancestors, class, name) || is_named_constructor(class, method) {
        return true;
    }
    if class.is_anonymous && class.extends.is_some() && is(b"__construct") {
        return true;
    }
    if has_parent_method_or_interface_method(facts, class, name) {
        return true;
    }
    let doc = member_doc(ctx.source, class, index);
    if method
        .parameter_list
        .parameters
        .iter()
        .any(|p| !p.modifiers.is_empty())
        || doc.is_some_and(|d| doc_tags(d).contains(&"@deprecated"))
        || is_controller_class_method(ctx.source, class, method, index)
    {
        return true;
    }
    if is(b"__clone") {
        return !is_public(method);
    }
    if is(b"__invoke") {
        return true;
    }
    // `isAttributeMarkerConstructor`.
    let Node::Class(c) = class.node else {
        return false;
    };
    is(b"__construct") && has_attribute(ancestors, &c.attribute_lists, "Attribute") && doc.is_some()
}
