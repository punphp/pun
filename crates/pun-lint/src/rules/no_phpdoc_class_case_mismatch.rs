//! `lint/complexity/no-phpdoc-class-case-mismatch`, ported from Rector 2.8.4's `FixClassCaseSensitivityVarDocblockRector`
//! (rules/CodeQuality/Rector/Property).
//!
//! A `@var` tag of a property or an expression statement whose type is one class name written in another case than
//! the class's own (`@var AutomailingService` for `AutoMailingService`) gets the class's case. The name is looked up as
//! Rector does: a fully qualified one as it is, else through a `use` import without alias whose last part is the
//! name's first (none in a file of several namespaces), then in the namespace, then in the global one. Only a difference of case changes: the written parts
//! keep their number and a leading `\`.

use mago_syntax::cst::{ClassLikeMember, Node, NodeKind, Statement};
use mago_syntax::token::TokenKind;
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_stmts::{previous_token_end, tokens};
use super::types::facts::{Facts, facts};
use super::types::names::class_imports;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoPhpdocClassCaseMismatch;

impl Rule for NoPhpdocClassCaseMismatch {
    fn id(&self) -> &'static str {
        "lint/complexity/no-phpdoc-class-case-mismatch"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Statement, NodeKind::ClassLikeMember])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if !matches!(
            node,
            Node::Statement(Statement::Expression(_))
                | Node::ClassLikeMember(ClassLikeMember::Property(_))
        ) {
            return;
        }
        let source = ctx.source;
        let span = node_span(&node);
        let from = ancestors
            .iter()
            .rev()
            .map(|a| node_span(a).start)
            .find(|start| *start < span.start)
            .unwrap_or(0);
        // php-parser's `getDocComment`: the last doc comment before the node
        let Some(doc) = tokens(
            source,
            previous_token_end(ctx, from, span.start),
            span.start,
        )
        .into_iter()
        .rfind(|t| t.kind == TokenKind::DocBlockComment)
        .map(|t| Span::new(t.start, t.end)) else {
            return;
        };
        let text = &source[doc.start..doc.end];
        if !text.contains("@var") {
            return;
        }
        let facts = facts(ctx);
        let mut edits = Vec::new();
        for (at, _) in text.match_indices("@var") {
            // a tag starts its line (a `@var` further on is description)
            let line = &text[text[..at].rfind('\n').map_or(0, |i| i + 1)..at];
            let rest = &text[at + 4..];
            if !line.trim_start_matches([' ', '\t', '*', '/']).is_empty()
                || !rest.starts_with([' ', '\t'])
            {
                continue;
            }
            let start = at + 4 + rest.len() - rest.trim_start_matches([' ', '\t']).len();
            let rest = &text[start..];
            let end = rest
                .find(|c: char| c.is_whitespace() || c == '*')
                .unwrap_or(rest.len());
            let written = &rest[..end];
            // an `IdentifierTypeNode` only: a name and nothing else
            if !is_name(written)
                || rest[end..].starts_with(|c: char| !c.is_whitespace() && c != '*')
            {
                continue;
            }
            if let Some(corrected) = corrected(&facts, ancestors, written) {
                let from = doc.start + start;
                edits.push(Edit::new(Span::new(from, from + written.len()), corrected));
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report {
            span: doc,
            message: "This class name's case differs from the class's.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `correctClassNameCasing`: the name in the class's case, when only the case differs.
fn corrected(facts: &Facts, ancestors: &[Node<'_, '_>], written: &str) -> Option<String> {
    let existing = existing_class_name(facts, ancestors, written)?;
    let real = facts
        .codebase
        .get_class_like(existing.as_bytes())?
        .original_name;
    let real = String::from_utf8_lossy(real.as_bytes()).into_owned();
    let leading = written.starts_with('\\');
    let parts: Vec<&str> = written.trim_start_matches('\\').split('\\').collect();
    let real_parts: Vec<&str> = real.split('\\').collect();
    let real_parts = &real_parts[real_parts.len().saturating_sub(parts.len())..];
    if !parts
        .join("\\")
        .eq_ignore_ascii_case(&real_parts.join("\\"))
    {
        return None;
    }
    let corrected = format!(
        "{}{}",
        if leading { "\\" } else { "" },
        real_parts.join("\\")
    );
    (corrected != written).then_some(corrected)
}

/// `resolveExistingClassName`.
fn existing_class_name(facts: &Facts, ancestors: &[Node<'_, '_>], written: &str) -> Option<String> {
    let has = |name: &str| facts.codebase.get_class_like(name.as_bytes()).is_some();
    let bare = written.trim_start_matches('\\');
    if written.starts_with('\\') {
        return has(bare).then(|| bare.to_string());
    }
    let parts: Vec<&str> = bare.split('\\').collect();
    let (namespace, mut imports) = class_imports(ancestors);
    // `FileNode::getUsesAndGroupUses()`: the file's own statements unless it has exactly one namespace, so a file of
    // several namespaces has no imports for this lookup
    if let Some(Node::Program(p)) = ancestors.first()
        && (p.statements.iter())
            .filter(|s| matches!(s, Statement::Namespace(_)))
            .count()
            > 1
    {
        imports.clear();
    }
    for (full, alias) in &imports {
        let last = full.rsplit('\\').next().unwrap_or(full);
        if alias.is_some() || !last.eq_ignore_ascii_case(parts[0]) {
            continue;
        }
        let mut candidate = full.clone();
        for part in &parts[1..] {
            candidate.push('\\');
            candidate.push_str(part);
        }
        if has(&candidate) {
            return Some(candidate);
        }
    }
    if let Some(namespace) = namespace {
        let candidate = format!("{namespace}\\{bare}");
        if has(&candidate) {
            return Some(candidate);
        }
    }
    has(bare).then(|| bare.to_string())
}

fn is_name(written: &str) -> bool {
    let name = written.strip_prefix('\\').unwrap_or(written);
    !name.is_empty()
        && name.split('\\').all(|part| {
            part.chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                && part.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
}
