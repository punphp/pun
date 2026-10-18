//! Port of `PhpCsFixer\DocBlock`: DocBlock, Line, Annotation, Tag, TagComparator, ShortDescription, TypeExpression.
//!
//! Upstream shares `Line` objects between a DocBlock and the annotations/tags built from it, so editing an
//! annotation edits the docblock; here lines are `Rc<RefCell<Line>>` for the same aliasing.

mod annotation;
mod line;
#[macro_use]
mod regex;
mod short_description;
mod tag;
pub(crate) mod type_expression;

#[cfg(test)]
mod tests;

pub(crate) use annotation::{Annotation, TAGS_WITH_TYPES};
pub(crate) use line::{Line, LineRef};
pub(crate) use short_description::ShortDescription;
pub(crate) use type_expression::TypeExpression;

use crate::analyzers::preg::{PREG_SPLIT_DELIM_CAPTURE, PREG_SPLIT_NO_EMPTY, Preg};
use pun_tokens::php_compat::{str_replace, trim};
use std::cell::OnceCell;

#[derive(Clone, Debug)]
pub(crate) struct DocBlock {
    lines: Vec<LineRef>,
    annotations: OnceCell<Vec<Annotation>>,
}

impl DocBlock {
    pub(crate) fn new(content: &[u8]) -> Self {
        let lines = split_lines_ascii(content)
            .unwrap_or_else(|| {
                Preg::split(
                    r"/([^\n\r]+\R*)/",
                    content,
                    -1,
                    PREG_SPLIT_NO_EMPTY | PREG_SPLIT_DELIM_CAPTURE,
                )
                .into_iter()
                .map(|p| p.text)
                .collect()
            })
            .into_iter()
            .map(Line::shared)
            .collect();
        DocBlock {
            lines,
            annotations: OnceCell::new(),
        }
    }

    pub(crate) fn get_lines(&self) -> &[LineRef] {
        &self.lines
    }

    pub(crate) fn get_line(&self, pos: usize) -> Option<&LineRef> {
        self.lines.get(pos)
    }

    pub(crate) fn get_annotations(&self) -> &[Annotation] {
        self.annotations.get_or_init(|| {
            let mut annotations = vec![];
            let total = self.lines.len();
            let mut index = 0;
            while index < total {
                if self.lines[index].borrow().contains_a_tag() {
                    let len = self.find_annotation_length(index);
                    let lines = (index..index + len)
                        .map(|i| (i, self.lines[i].clone()))
                        .collect();
                    let annotation = Annotation::new(lines);
                    index = annotation.get_end();
                    annotations.push(annotation);
                }
                index += 1;
            }
            annotations
        })
    }

    pub(crate) fn is_multi_line(&self) -> bool {
        self.lines.len() != 1
    }

    pub(crate) fn get_annotation(&self, pos: usize) -> Option<&Annotation> {
        self.get_annotations().get(pos)
    }

    pub(crate) fn get_annotations_of_type(&self, types: &[&str]) -> Vec<&Annotation> {
        self.get_annotations()
            .iter()
            .filter(|a| types.iter().any(|t| t.as_bytes() == a.get_tag().get_name()))
            .collect()
    }

    pub(crate) fn get_content(&self) -> Vec<u8> {
        self.lines
            .iter()
            .flat_map(|l| l.borrow().get_content().to_vec())
            .collect()
    }

    pub(crate) fn make_multi_line(&mut self, indent: &[u8], line_end: &[u8]) {
        if self.is_multi_line() {
            return;
        }
        let line_content = single_line_doc_block_entry(self.lines[0].borrow().get_content());
        let middle = if line_content.is_empty() {
            [indent, b" *", line_end].concat()
        } else {
            [indent, b" * ", &line_content, line_end].concat()
        };
        self.set_lines(vec![
            [&b"/**"[..], line_end].concat(),
            middle,
            [indent, b" */"].concat(),
        ]);
    }

    pub(crate) fn make_single_line(&mut self) {
        if !self.is_multi_line() {
            return;
        }
        let useful: Vec<&LineRef> = (self.lines.iter())
            .filter(|l| l.borrow().contains_useful_content())
            .collect();
        if useful.len() > 1 {
            return;
        }
        let line_content = (useful.first())
            .map(|l| single_line_doc_block_entry(l.borrow().get_content()))
            .unwrap_or_default();
        self.set_lines(vec![[&b"/** "[..], &line_content, b" */"].concat()]);
    }

    fn set_lines(&mut self, lines: Vec<Vec<u8>>) {
        self.lines = lines.into_iter().map(Line::shared).collect();
        self.annotations = OnceCell::new();
    }

    fn find_annotation_length(&self, start: usize) -> usize {
        let mut index = start;
        loop {
            index += 1;
            let Some(line) = self.get_line(index) else {
                break;
            };
            let line = line.borrow();
            if line.contains_a_tag() {
                break;
            }
            if !line.contains_useful_content() {
                match self.get_line(index + 1) {
                    None => break,
                    Some(next) => {
                        let next = next.borrow();
                        if !next.contains_useful_content() || next.contains_a_tag() {
                            break;
                        }
                    }
                }
            }
        }
        index - start
    }
}

/// `getSingleLineDocBlockEntry`: the line without `*/`, the opening `/**` or a leading `*`, trimmed.
fn single_line_doc_block_entry(line: &[u8]) -> Vec<u8> {
    let line = str_replace(b"*/", b"", line);
    let line = trim(&line);
    let line = (line.strip_prefix(b"/**"))
        .or_else(|| line.strip_prefix(b"*"))
        .unwrap_or(line);
    trim(line).to_vec()
}

/// `preg_split('/([^\n\r]+\R*)/', ..., NO_EMPTY | DELIM_CAPTURE)` for ASCII content not starting with CR/LF:
/// each piece is a run without CR/LF (VT and FF included) plus the run of CR, LF, VT and FF after it.
fn split_lines_ascii(content: &[u8]) -> Option<Vec<Vec<u8>>> {
    if !content.is_ascii() || matches!(content.first(), Some(b'\n' | b'\r')) {
        return None;
    }
    let mut lines = Vec::new();
    let mut i = 0;
    while i < content.len() {
        let start = i;
        while i < content.len() && !matches!(content[i], b'\n' | b'\r') {
            i += 1;
        }
        while i < content.len() && matches!(content[i], b'\n' | b'\r' | 0x0b | 0x0c) {
            i += 1;
        }
        lines.push(content[start..i].to_vec());
    }
    Some(lines)
}
