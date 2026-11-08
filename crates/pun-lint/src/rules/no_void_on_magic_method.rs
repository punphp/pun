//! `lint/phpdoc/no-void-on-magic-method`, ported from Rector 2.8.4's `RemoveVoidDocblockFromMagicMethodRector`
//! (rules/DeadCode/Rector/ClassMethod).
//!
//! `__construct()`, `__destruct()` and `__clone()` return nothing, so a `@return void` tag on one without a native
//! return type says nothing: the doc comment loses its `@return` tags, and goes when nothing else is left in it. A
//! `@return` with a description, a type other than `void` (case counts), or a `@phpstan-`/`@psalm-` prefix stays.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Fix, Safety};

use super::lint_b_doc::Doc;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoVoidOnMagicMethod;

impl Rule for NoVoidOnMagicMethod {
    fn id(&self) -> &'static str {
        "lint/phpdoc/no-void-on-magic-method"
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
        let Node::Method(method) = node else { return };
        if method.return_type_hint.is_some()
            || !["__construct", "__destruct", "__clone"]
                .iter()
                .any(|n| method.name.value.eq_ignore_ascii_case(n.as_bytes()))
        {
            return;
        }
        let start = node_span(method).start;
        let Some(doc) = Doc::of(ctx, ancestors, start) else {
            return;
        };
        let words: Vec<Vec<&str>> = (0..doc.len()).map(|i| doc.words(i)).collect();
        let is_return = |w: &Vec<&str>| w.first() == Some(&"@return");
        // A `@return` without a type takes the next line into its value, and Rector then leaves the doc alone.
        if words.iter().any(|w| is_return(w) && w.len() == 1) {
            return;
        }
        // `PhpDocInfo::getReturnTagValue()`: the first `@return`, which must be `void` with no description.
        if words.iter().find(|w| is_return(w)) != Some(&vec!["@return", "void"]) {
            return;
        }
        // `removeByType(ReturnTagValueNode::class)`: every `@return` tag.
        let removed: Vec<bool> = words.iter().map(is_return).collect();
        let edit = doc.remove(&removed, start);
        out.push(Report {
            span: doc.span,
            message: "Remove the useless `@return void` from this magic method.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![edit],
            }),
        });
    }
}
