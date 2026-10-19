//! `lint/phpdoc/no-mixed-overruled-by-native`, ported from Rector 2.6.1's `RemoveMixedDocblockOverruledByNativeTypeRector`
//! (rules/DeadCode/Rector/ClassMethod).
//!
//! A `@param mixed $x` without a description, on a parameter with a native type, and a `@return mixed` without one,
//! on a function or method with a native return type, say less than the code: they go from the doc comment, and the
//! doc comment with them when nothing else is left.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Fix, Safety};

use super::lint_b_doc::Doc;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoMixedOverruledByNative;

impl Rule for NoMixedOverruledByNative {
    fn id(&self) -> &'static str {
        "lint/phpdoc/no-mixed-overruled-by-native"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method, NodeKind::Function])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        // A method's and a function's parameter list and return type are their children.
        let mut list = None;
        let mut has_return_type = false;
        node.visit_children(|child| match child {
            Node::FunctionLikeParameterList(l) => list = Some(l),
            Node::FunctionLikeReturnTypeHint(_) => has_return_type = true,
            _ => {}
        });
        let Some(list) = list else { return };
        let start = node_span(&node).start;
        let Some(doc) = Doc::of(ctx.source, ancestors, start) else {
            return;
        };
        // `resolveDeclaredParamNames()`: the parameters with a native type.
        let typed: Vec<&[u8]> = list
            .parameters
            .iter()
            .filter(|p| p.hint.is_some())
            .map(|p| &p.variable.name[1..])
            .collect();
        let tags: Vec<Vec<&str>> = (0..doc.len()).map(|i| doc.words(i)).collect();
        let mut removed = vec![false; tags.len()];
        for (i, words) in tags.iter().enumerate() {
            // `@param mixed $name`, the name without `&` and `...`, and nothing after it.
            if let ["@param", "mixed", name] = words[..]
                && let Some(name) = name.trim_start_matches(['&', '.']).strip_prefix('$')
            {
                removed[i] = typed.contains(&name.as_bytes());
            }
        }
        // `getReturnTagValue()`: the first `@return` with a type; then `removeByType()` takes every one.
        let returns: Vec<usize> = (0..tags.len())
            .filter(|&i| tags[i].first() == Some(&"@return") && tags[i].len() > 1)
            .collect();
        if has_return_type
            && let Some(&first) = returns.first()
            && tags[first][1..] == ["mixed"]
        {
            for i in returns {
                removed[i] = true;
            }
        }
        if !removed.contains(&true) {
            return;
        }
        out.push(Report {
            span: doc.span,
            message: "Remove the `mixed` tag that the native type overrules.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![doc.remove(&removed, start)],
            }),
        });
    }
}
