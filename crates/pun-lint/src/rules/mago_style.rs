//! Small style report rules, ported from Mago 1.57.0's linter (mago-linter src/rule/):
//!
//! - `lint/style/no-inline-html` (`no-inline`, best_practices/no_inline.rs): text outside the PHP tags.
//! - `lint/style/no-isset` (`no-isset`, clarity/no_isset.rs): every `isset()`; with `allow-array-checks` (default
//!   false), not one whose arguments are all array accesses.
//! - `lint/style/no-multiple-classes-per-file` (`single-class-per-file`, best_practices/single_class_per_file.rs):
//!   every named class, interface, trait or enum after the first in the file (anywhere, a repeated name counted
//!   once), reported on its name.
//! - `lint/style/prefer-while-loop` (`prefer-while-loop`, best_practices/prefer_while_loop.rs): a `for` without
//!   initializations and increments.
//! - `lint/style/require-namespace` (`require-namespace`, best_practices/require_namespace.rs): the first class-like,
//!   function or constant declared at the top of a file that has no namespace.
//! - `lint/style/require-sorted-integer-keys` (`sorted-integer-keys`, best_practices/sorted_integer_keys.rs): an
//!   array of two or more items, each with a literal integer key, whose keys are not ascending.

use std::collections::HashSet;

use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::mago_helpers::{boolean, configure};
use crate::syntax::{node_span, span};
use crate::{Context, Report, Rule};

fn report(at: pun_problem::Span, message: String, help: &str) -> Report {
    Report {
        span: at,
        message,
        help: Some(help.into()),
        fix: None,
    }
}

pub struct NoInlineHtml;

impl Rule for NoInlineHtml {
    fn id(&self) -> &'static str {
        "lint/style/no-inline-html"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Inline])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        out.push(report(
            node_span(&node),
            "Inline content is not allowed outside of PHP tags.".into(),
            "Remove the inline content, or move it inside PHP tags using `echo`.",
        ));
    }
}

#[derive(Default)]
pub struct NoIsset {
    pub allow_array_checks: bool,
}

impl Rule for NoIsset {
    fn id(&self) -> &'static str {
        "lint/style/no-isset"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::IssetConstruct])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "allow-array-checks" => self.allow_array_checks = boolean(value)?,
                _ => return None,
            }
            Some(())
        })
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::IssetConstruct(isset) = node else {
            return;
        };
        if self.allow_array_checks
            && isset
                .values
                .iter()
                .all(|v| matches!(v, Expression::ArrayAccess(_)))
        {
            return;
        }
        out.push(report(
            node_span(isset),
            "Use of the `isset` construct.".into(),
            "Use explicit null comparison (`!== null`) or the null coalescing operator (`??`).",
        ));
    }
}

pub struct NoMultipleClassesPerFile;

impl Rule for NoMultipleClassesPerFile {
    fn id(&self) -> &'static str {
        "lint/style/no-multiple-classes-per-file"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        let mut collector = ClassLikeCollector(Vec::new());
        collector.walk_program(program, &mut ());
        let mut seen = HashSet::new();
        let mut definitions = collector.0;
        definitions.retain(|d| seen.insert(d.1));
        let Some(&(first_kind, first_name, _)) = definitions.first() else {
            return;
        };
        let first_name = String::from_utf8_lossy(first_name);
        for &(kind, name, at) in &definitions[1..] {
            out.push(report(
                at,
                format!(
                    "File contains multiple class-like definitions; found {kind} `{}` but {first_kind} `{first_name}` is already defined in this file.",
                    String::from_utf8_lossy(name)
                ),
                "Move each class-like definition into its own file.",
            ));
        }
    }
}

struct ClassLikeCollector<'arena>(Vec<(&'static str, &'arena [u8], pun_problem::Span)>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for ClassLikeCollector<'arena> {
    fn walk_in_class(&mut self, class: &'ast Class<'arena>, _: &mut ()) {
        self.0
            .push(("class", class.name.value, span(class.name.span())));
    }

    fn walk_in_interface(&mut self, interface: &'ast Interface<'arena>, _: &mut ()) {
        self.0.push((
            "interface",
            interface.name.value,
            span(interface.name.span()),
        ));
    }

    fn walk_in_trait(&mut self, r#trait: &'ast Trait<'arena>, _: &mut ()) {
        self.0
            .push(("trait", r#trait.name.value, span(r#trait.name.span())));
    }

    fn walk_in_enum(&mut self, r#enum: &'ast Enum<'arena>, _: &mut ()) {
        self.0
            .push(("enum", r#enum.name.value, span(r#enum.name.span())));
    }
}

pub struct PreferWhileLoop;

impl Rule for PreferWhileLoop {
    fn id(&self) -> &'static str {
        "lint/style/prefer-while-loop"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::For])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::For(r#for) = node else { return };
        if r#for.initializations.is_empty() && r#for.increments.is_empty() {
            out.push(report(
                node_span(r#for),
                "Use `while` loop instead of `for` loop.".into(),
                "Use a `while` loop instead of a `for` loop.",
            ));
        }
    }
}

pub struct RequireNamespace;

impl Rule for RequireNamespace {
    fn id(&self) -> &'static str {
        "lint/style/require-namespace"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        if program
            .statements
            .iter()
            .any(|s| matches!(s, Statement::Namespace(_)))
        {
            return;
        }
        let first = program.statements.iter().find(|s| {
            matches!(
                s,
                Statement::Class(_)
                    | Statement::Interface(_)
                    | Statement::Enum(_)
                    | Statement::Trait(_)
                    | Statement::Function(_)
                    | Statement::Constant(_)
            )
        });
        if let Some(definition) = first {
            out.push(report(
                node_span(definition),
                "File contains definitions without a namespace declaration.".into(),
                "Add a namespace declaration at the top of the file.",
            ));
        }
    }
}

pub struct RequireSortedIntegerKeys;

impl Rule for RequireSortedIntegerKeys {
    fn id(&self) -> &'static str {
        "lint/style/require-sorted-integer-keys"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Array, NodeKind::LegacyArray])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let elements = match node {
            Node::Array(array) => &array.elements,
            Node::LegacyArray(array) => &array.elements,
            _ => return,
        };
        if elements.len() < 2 {
            return;
        }
        let mut keys = Vec::with_capacity(elements.len());
        for element in elements.iter() {
            let ArrayElement::KeyValue(kv) = element else {
                return;
            };
            let Expression::Literal(Literal::Integer(LiteralInteger { value: Some(v), .. })) =
                kv.key
            else {
                return;
            };
            keys.push(*v);
        }
        if keys.windows(2).all(|w| w[0] <= w[1]) {
            return;
        }
        out.push(report(
            node_span(&node),
            "Integer array keys are not in ascending order.".into(),
            "Sort the array entries by key in ascending order to enable the packed array optimization.",
        ));
    }
}
