//! Port of `PhpCsFixer\DocBlock`: DocBlock, Line, Annotation, Tag, TagComparator, ShortDescription, TypeExpression.
//!
//! Upstream shares `Line` objects between a DocBlock and the annotations/tags built from it, so editing an
//! annotation edits the docblock; here lines are `Rc<RefCell<Line>>` for the same aliasing.

mod annotation;
mod line;
pub mod preg;
#[macro_use]
mod regex;
mod short_description;
mod tag;
pub mod type_expression;

#[cfg(test)]
mod tests;

pub use annotation::{Annotation, TAGS_WITH_TYPES};
pub use line::{Line, LineRef};
pub use short_description::ShortDescription;
pub use type_expression::TypeExpression;

use std::cell::OnceCell;

#[derive(Clone, Debug)]
pub struct DocBlock {
    lines: Vec<LineRef>,
    annotations: OnceCell<Vec<Annotation>>,
}

impl DocBlock {
    pub fn new(content: &[u8]) -> Self {
        let lines = split_lines_ascii(content)
            .unwrap_or_else(|| {
                preg::split(
                    r"/([^\n\r]+\R*)/",
                    content,
                    preg::SPLIT_NO_EMPTY | preg::SPLIT_DELIM_CAPTURE,
                )
            })
            .into_iter()
            .map(Line::shared)
            .collect();
        DocBlock {
            lines,
            annotations: OnceCell::new(),
        }
    }

    pub fn get_lines(&self) -> &[LineRef] {
        &self.lines
    }

    pub fn get_line(&self, pos: usize) -> Option<&LineRef> {
        self.lines.get(pos)
    }

    pub fn get_annotations(&self) -> &[Annotation] {
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

    pub fn is_multi_line(&self) -> bool {
        self.lines.len() != 1
    }

    pub fn get_annotation(&self, pos: usize) -> Option<&Annotation> {
        self.get_annotations().get(pos)
    }

    pub fn get_annotations_of_type(&self, types: &[&str]) -> Vec<&Annotation> {
        self.get_annotations()
            .iter()
            .filter(|a| types.iter().any(|t| t.as_bytes() == a.get_tag().get_name()))
            .collect()
    }

    pub fn get_content(&self) -> Vec<u8> {
        self.lines
            .iter()
            .flat_map(|l| l.borrow().get_content().to_vec())
            .collect()
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

/// PHP `str_replace` for one needle.
pub fn replace_all(s: &[u8], from: &[u8], to: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    let mut i = 0;
    while i < s.len() {
        if s[i..].starts_with(from) {
            out.extend_from_slice(to);
            i += from.len();
        } else {
            out.push(s[i]);
            i += 1;
        }
    }
    out
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
