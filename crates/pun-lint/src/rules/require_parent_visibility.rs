//! `lint/style/require-parent-visibility`, ported from Rector 2.6.1's `MakeInheritedMethodVisibilitySameAsParentRector`
//! (rules/CodingStyle/Rector/ClassMethod).
//!
//! A method that overrides a public or protected method of a parent class gets that method's visibility. Magic methods,
//! anonymous classes, and a public method an implemented interface declares are left alone. The parents are the
//! classes the codebase knows, nearest first.

use mago_codex::visibility::Visibility;
use mago_syntax::cst::{ClassLikeMember, Modifier, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::no_unused_public_method_param::MAGIC;
use super::types::facts::facts;
use super::types::namespace_of;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct RequireParentVisibility;

impl Rule for RequireParentVisibility {
    fn id(&self) -> &'static str {
        "lint/style/require-parent-visibility"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Class(class) = node else {
            return;
        };
        let name = String::from_utf8_lossy(class.name.value);
        let name = match namespace_of(ancestors) {
            Some(ns) => format!("{ns}\\{name}"),
            None => name.into_owned(),
        };
        let facts = facts(ctx);
        let codebase = &facts.codebase;
        let own = codebase.get_class_like(name.as_bytes());
        let Some(own) = own.filter(|_| class.extends.is_some()) else {
            return;
        };
        // `getParents()`: the parent classes, nearest first, as far as they are known.
        let mut parents = Vec::new();
        let mut next = own.direct_parent_class.clone();
        while let Some(parent) = next {
            let Some(known) = codebase.get_class_like(parent.as_bytes()) else {
                break;
            };
            next = known.direct_parent_class.clone();
            parents.push(parent);
        }
        for member in class.members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let method_name = method.name.value;
            let lower = String::from_utf8_lossy(method_name).to_ascii_lowercase();
            if MAGIC.contains(&lower.as_str()) {
                continue;
            }
            let current = method.modifiers.iter().find_map(|m| match m {
                Modifier::Public(_) => Some(Visibility::Public),
                Modifier::Protected(_) => Some(Visibility::Protected),
                Modifier::Private(_) => Some(Visibility::Private),
                _ => None,
            });
            let mut current = current.unwrap_or(Visibility::Public);
            if current == Visibility::Public
                && own.all_parent_interfaces.iter().any(|i| {
                    codebase
                        .get_declaring_method(i.as_bytes(), method_name)
                        .is_some()
                })
            {
                continue;
            }
            let mut wanted = None;
            for parent in &parents {
                let Some(visibility) = codebase
                    .get_declaring_method(parent.as_bytes(), method_name)
                    .and_then(|m| m.method_metadata.as_ref())
                    .map(|m| m.visibility)
                else {
                    continue;
                };
                if visibility == Visibility::Private {
                    continue;
                }
                if visibility == current {
                    break;
                }
                wanted = Some(visibility);
                current = visibility;
            }
            let Some(wanted) = wanted else {
                continue;
            };
            // php-parser's `pModifiers` prints the modifiers again, in its order.
            let mut words: Vec<&str> = Vec::new();
            let has = |f: fn(&Modifier<'_>) -> bool| method.modifiers.iter().any(f);
            if has(|m| matches!(m, Modifier::Final(_))) {
                words.push("final");
            }
            if has(|m| matches!(m, Modifier::Abstract(_))) {
                words.push("abstract");
            }
            words.push(match wanted {
                Visibility::Protected => "protected",
                _ => "public",
            });
            if has(|m| matches!(m, Modifier::Static(_))) {
                words.push("static");
            }
            let end = method.function.span.start.offset as usize;
            let start = method.modifiers.first().map_or(end, |m| node_span(m).start);
            out.push(Report {
                span: node_span(&method.name),
                message: "Give the method the visibility of the parent method it overrides."
                    .to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: vec![Edit::new(
                        Span::new(start, end),
                        format!("{} ", words.join(" ")),
                    )],
                }),
            });
        }
    }
}
