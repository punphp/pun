//! `lint/laravel/add-has-factory-trait`, ported from rector-laravel 2.5.0's `AddHasFactoryToModelsRector`
//! (src/Rector/Class_).
//!
//! A class (named or anonymous) that is an Eloquent model gets `use \Illuminate\Database\Eloquent\Factories\HasFactory;`
//! as its first statement, unless it, a trait it uses or a parent already uses the trait. With `items` (Rector's
//! configuration, a list of class names), only those classes change: a name matches ignoring ASCII case, written
//! without a leading `\`; an anonymous class has no name. Rector fails on a name with `*`; Pun rejects it.
//!
//! php-parser prints the new statement before the class's first statement (or comment), followed by a new line at
//! that line's indentation. A class without statements is printed anew (`pStmt_Class`): each attribute group on its
//! own line, the modifiers in php-parser's order, the names as written, `{` on the next line.

use mago_syntax::cst::{AttributeList, ClassLikeMember, Extends, Implements, Modifier, Node, NodeKind, Sequence};
use pun_problem::{Edit, Fix, Safety, Span};
use serde_json::Value;

use super::is_a;
use super::names::resolve_class_name;
use super::print::kept_nodes;
use crate::rules::rector_batch5::modifiers_text;
use crate::rules::rector_conditions::indentation;
use crate::rules::types::facts::facts;
use crate::rules::types::namespace_of;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const TRAIT: &str = "Illuminate\\Database\\Eloquent\\Factories\\HasFactory";
const USE: &str = "use \\Illuminate\\Database\\Eloquent\\Factories\\HasFactory;";

#[derive(Default)]
pub struct AddHasFactoryTrait {
    pub items: Vec<String>,
}

/// The parts of a class `pStmt_Class` and `pExpr_New` print.
struct Parts<'a> {
    attributes: &'a Sequence<'a, AttributeList<'a>>,
    modifiers: &'a Sequence<'a, Modifier<'a>>,
    extends: Option<&'a Extends<'a>>,
    implements: Option<&'a Implements<'a>>,
    left_brace: usize,
    right_brace: usize,
}

impl Rule for AddHasFactoryTrait {
    fn id(&self) -> &'static str {
        "lint/laravel/add-has-factory-trait"
    }

    /// `configure`: `items`, the class names to change (Rector's configuration list).
    fn configure(&mut self, options: &Value) -> Result<(), String> {
        let map = match options {
            Value::Null => return Ok(()),
            Value::Object(map) => map,
            _ => return Err("options must be a table".into()),
        };
        if let Some(key) = map.keys().find(|k| *k != "items") {
            return Err(format!("unknown option `{key}`"));
        }
        let items = (map.get("items").and_then(Value::as_array)).and_then(|a| {
            a.iter()
                .map(|v| v.as_str().filter(|s| !s.contains('*')).map(str::to_string))
                .collect::<Option<Vec<_>>>()
        });
        self.items = items.ok_or("`items` must be a list of class names, without `*`")?;
        Ok(())
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
        let (name, parts, start) = match node {
            Node::Class(c) => {
                let local = String::from_utf8_lossy(c.name.value);
                let name = match namespace_of(ancestors) {
                    Some(ns) => format!("{ns}\\{local}"),
                    None => local.into_owned(),
                };
                let parts = Parts {
                    attributes: &c.attribute_lists,
                    modifiers: &c.modifiers,
                    extends: c.extends.as_ref(),
                    implements: c.implements.as_ref(),
                    left_brace: c.left_brace.start.offset as usize,
                    right_brace: c.right_brace.end.offset as usize,
                };
                let start = (c.attribute_lists.first().map(|a| node_span(a).start))
                    .or_else(|| c.modifiers.first().map(|m| node_span(m).start))
                    .unwrap_or(c.class.span.start.offset as usize);
                (Some(name), parts, start)
            }
            Node::AnonymousClass(c) => {
                let parts = Parts {
                    attributes: &c.attribute_lists,
                    modifiers: &c.modifiers,
                    extends: c.extends.as_ref(),
                    implements: c.implements.as_ref(),
                    left_brace: c.left_brace.start.offset as usize,
                    right_brace: c.right_brace.end.offset as usize,
                };
                (None, parts, c.new.span.start.offset as usize)
            }
            _ => return,
        };
        let class = match (&name, node) {
            (Some(name), _) => name.clone(),
            (None, Node::AnonymousClass(c)) => facts.anonymous_class_name(mago_span::HasSpan::span(c)),
            _ => return,
        };
        if !is_a(ctx, &class, &["Illuminate\\Database\\Eloquent\\Model"])
            || (!self.items.is_empty()
                && !name.as_ref().is_some_and(|n| self.items.iter().any(|i| i.eq_ignore_ascii_case(n))))
        {
            return;
        }
        let Some(meta) = facts.codebase.get_class_like(class.as_bytes()) else {
            return;
        };
        let members = match node {
            Node::Class(c) => &c.members,
            Node::AnonymousClass(c) => &c.members,
            _ => return,
        };
        // The project's codebase is the files as they were; the class's own `use` is read from the syntax too.
        let own = members.iter().any(|m| match m {
            ClassLikeMember::TraitUse(u) => (u.trait_names.iter())
                .any(|t| resolve_class_name(ancestors, t).eq_ignore_ascii_case(TRAIT)),
            _ => false,
        });
        if own || meta.used_traits.iter().any(|t| t.as_bytes().eq_ignore_ascii_case(TRAIT.as_bytes())) {
            return;
        }
        let source = ctx.source;
        let inside = &source[parts.left_brace + 1..parts.right_brace - 1];
        let first = inside.len() - inside.trim_start().len();
        let edit = if inside.trim().is_empty() {
            Edit::new(Span::new(start, parts.right_brace), header(ctx, node, &parts, start))
        } else {
            let at = parts.left_brace + 1 + first;
            let indent = " ".repeat(indentation(source, at));
            Edit::new(Span::new(at, at), format!("{USE}\n{indent}"))
        };
        out.push(Report {
            span: match node {
                Node::Class(c) => node_span(&c.name),
                _ => Span::new(start, parts.left_brace),
            },
            message: "Add the `HasFactory` trait to the model.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![edit],
            }),
        });
    }
}

/// The class printed anew with the trait as its only statement.
fn header(ctx: &Context<'_>, node: Node<'_, '_>, parts: &Parts<'_>, start: usize) -> String {
    let source = ctx.source;
    let text = |s: Span| &source[s.start..s.end];
    let nl = format!("\n{}", " ".repeat(indentation(source, start)));
    let mut out = String::new();
    if let Node::AnonymousClass(_) = node {
        out.push_str("new ");
    }
    for list in parts.attributes.iter() {
        out.push_str(text(node_span(list)));
        out.push_str(&nl);
    }
    out.push_str(&modifiers_text(parts.modifiers));
    out.push_str("class");
    match node {
        Node::Class(c) => {
            out.push(' ');
            out.push_str(text(node_span(&c.name)));
        }
        Node::AnonymousClass(c) => {
            if let Some(list) = c.argument_list.as_ref().filter(|l| !l.arguments.is_empty()) {
                let args = kept_nodes(source, list.arguments.iter().map(node_span), start);
                out.push_str(&format!("({})", args.join(", ")));
            }
        }
        _ => {}
    }
    if let Some(e) = parts.extends {
        let names: Vec<&str> = e.types.iter().map(|t| text(node_span(t))).collect();
        out.push_str(&format!(" extends {}", names.join(", ")));
    }
    if let Some(i) = parts.implements {
        let names: Vec<&str> = i.types.iter().map(|t| text(node_span(t))).collect();
        out.push_str(&format!(" implements {}", names.join(", ")));
    }
    out.push_str(&format!("{nl}{{{nl}    {USE}{nl}}}"));
    out
}
