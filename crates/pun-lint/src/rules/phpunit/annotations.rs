//! What rector-phpunit's annotation-to-attribute rules (rules/AnnotationsToAttributes) share: a test class's or test
//! method's doc comment (`PhpDocInfoFactory::createFromNode`), the values of its generic tags
//! (`GenericTagValueNode`), the doc without the tags that moved (`PhpDocTagRemover`, `DocBlockUpdater`), and the new
//! attribute groups as php-parser prints them when they are added to the node's `attrGroups`.

use mago_syntax::cst::{AttributeList, Node, Sequence};
use pun_problem::{Edit, Span};

use crate::rules::lint_b_doc::Doc;
use crate::rules::rector_stmts::line_indent;
use crate::syntax::node_span;

/// A class or a method that may get attributes: its span's start (attributes included), the end of its last
/// attribute list, and its doc comment.
pub(crate) struct Annotated<'s> {
    pub start: usize,
    pub last_attribute_end: Option<usize>,
    pub doc: Option<Doc<'s>>,
}

impl<'s> Annotated<'s> {
    /// The class or method `node`, whose ancestors are `ancestors`; `None` for any other node.
    pub(crate) fn of(
        source: &'s str,
        node: Node<'_, '_>,
        ancestors: &[Node<'_, '_>],
    ) -> Option<Self> {
        let lists: &Sequence<'_, AttributeList<'_>> = match node {
            Node::Class(class) => &class.attribute_lists,
            Node::Method(method) => &method.attribute_lists,
            _ => return None,
        };
        let start = node_span(&node).start;
        Some(Annotated {
            start,
            last_attribute_end: lists.last().map(|l| node_span(l).end),
            doc: Doc::of(source, ancestors, start),
        })
    }

    /// The edits that give the doc comment `new_doc` (`""` removes it) and add `groups` (each `#[...]`) after the
    /// node's attributes: a line each, at the node's indentation.
    pub(crate) fn edits(
        &self,
        source: &str,
        new_doc: Option<&str>,
        groups: &[String],
    ) -> Vec<Edit> {
        let indent = line_indent(source, self.start);
        let mut edits = Vec::new();
        let mut at_start = self.start;
        if let (Some(doc), Some(new_doc)) = (self.doc.as_ref().map(|d| d.span), new_doc) {
            if new_doc.is_empty() {
                // The doc goes with the whitespace after it; what the node gets in front then takes its place.
                let after =
                    doc.end + source[doc.end..].len() - source[doc.end..].trim_start().len();
                if after == self.start && self.last_attribute_end.is_none() && !groups.is_empty() {
                    at_start = doc.start;
                } else {
                    edits.push(Edit::new(Span::new(doc.start, after), ""));
                }
            } else {
                edits.push(Edit::new(doc, new_doc));
            }
        }
        if groups.is_empty() {
            return edits;
        }
        match self.last_attribute_end {
            Some(end) => {
                let text: String = groups.iter().map(|g| format!("\n{indent}{g}")).collect();
                edits.push(Edit::new(Span::new(end, end), text));
            }
            None => {
                let text: String = groups.iter().map(|g| format!("{g}\n{indent}")).collect();
                edits.push(Edit::new(Span::new(at_start, self.start), text));
            }
        }
        edits
    }
}

/// Whether the doc tag `tag` (its text from the `@`) is a `@name` tag whose value phpdoc-parser reads as a
/// `GenericTagValueNode`: the name as written, then nothing, whitespace, or a value that does not open a Doctrine
/// annotation's `(`.
pub(crate) fn is_generic_tag(tag: &str, name: &str) -> bool {
    let Some(rest) = tag.strip_prefix('@').and_then(|t| t.strip_prefix(name)) else {
        return false;
    };
    rest.is_empty() || rest.starts_with([' ', '\t', '\n', '\r'])
}

/// The `GenericTagValueNode` value of the doc tag `tag` named `name`: what follows the name, its leading whitespace
/// aside, with each further line of the tag after its `*` and one space, the lines joined with `\n`.
pub(crate) fn generic_tag_value(tag: &str, name: &str) -> String {
    let rest = &tag[1 + name.len()..];
    let mut lines = rest.split('\n');
    let mut value = vec![
        lines
            .next()
            .unwrap_or_default()
            .trim_start()
            .trim_end_matches('\r')
            .to_string(),
    ];
    // A blank line adds nothing.
    for line in lines {
        let line = line.trim_end_matches('\r').trim_start();
        let line = line.strip_prefix('*').unwrap_or(line);
        if !line.trim().is_empty() {
            value.push(line.strip_prefix(' ').unwrap_or(line).to_string());
        }
    }
    value.join("\n").trim_end().to_string()
}

/// The doc comment `doc` as Rector prints it without its generic `@name` tags (`""` when nothing is left), and their
/// values in order; `None` when it has none, or when one has a further tag inside a line, which phpdoc-parser splits
/// off and Pun does not follow.
pub(crate) fn take_generic_tags(doc: &Doc<'_>, name: &str) -> Option<(String, Vec<String>)> {
    let removed: Vec<bool> = (0..doc.len())
        .map(|i| is_generic_tag(doc.text(i), name))
        .collect();
    if !removed.contains(&true) {
        return None;
    }
    let mut values = Vec::new();
    for (i, _) in removed.iter().enumerate().filter(|(_, r)| **r) {
        let text = doc.text(i);
        if text[1..]
            .split([' ', '\t', '\n'])
            .any(|w| w.starts_with('@') && w.as_bytes().get(1).is_some_and(u8::is_ascii_alphabetic))
        {
            return None;
        }
        values.push(generic_tag_value(text, name));
    }
    Some((doc.print(&removed), values))
}

/// php-parser's `pSingleQuotedString`: `'` and the backslashes before a `'`, a `\` or the end are escaped.
pub(crate) fn single_quoted(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = String::with_capacity(value.len() + 2);
    out.push('\'');
    for (i, c) in value.char_indices() {
        let escape = match c {
            '\'' => true,
            '\\' => {
                matches!(bytes.get(i + 1), None | Some(b'\'') | Some(b'\\'))
                    || (i > 0 && bytes[i - 1] == b'\\')
            }
            _ => false,
        };
        if escape {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('\'');
    out
}
