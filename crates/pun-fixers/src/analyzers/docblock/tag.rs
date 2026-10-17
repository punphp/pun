//! `PhpCsFixer\DocBlock\Tag`.

use super::line::LineRef;
use super::preg;
use std::cell::RefCell;

pub const PSR_STANDARD_TAGS: &[&str] = &[
    "api",
    "author",
    "category",
    "copyright",
    "deprecated",
    "example",
    "global",
    "internal",
    "license",
    "link",
    "method",
    "package",
    "param",
    "property",
    "property-read",
    "property-write",
    "return",
    "see",
    "since",
    "subpackage",
    "throws",
    "todo",
    "uses",
    "var",
    "version",
];

#[derive(Clone, Debug)]
pub struct Tag {
    line: LineRef,
    name: RefCell<Option<Vec<u8>>>,
}

impl Tag {
    pub fn new(line: LineRef) -> Self {
        Tag {
            line,
            name: RefCell::new(None),
        }
    }

    pub fn get_name(&self) -> Vec<u8> {
        if self.name.borrow().is_none() {
            let content = self.line.borrow().get_content().to_vec();
            let name = match tag_name_ascii(&content) {
                Some(name) => name,
                None => match preg::match_all(r"/@[a-zA-Z0-9_-]+(?=\s|$)/", &content).first() {
                    Some(m) => m.str(0)[1..].to_vec(),
                    None => b"other".to_vec(),
                },
            };
            *self.name.borrow_mut() = Some(name);
        }
        self.name.borrow().clone().unwrap()
    }

    pub fn valid(&self) -> bool {
        PSR_STANDARD_TAGS
            .iter()
            .any(|t| t.as_bytes() == self.get_name())
    }
}

/// The first `/@[a-zA-Z0-9_-]+(?=\s|$)/` match for ASCII content: the `+` must stop at its longest run (a shorter one
/// is followed by another name byte, never `\s`), so the first `@` whose full run ends at whitespace or the end wins.
pub(super) fn tag_name_ascii(content: &[u8]) -> Option<Vec<u8>> {
    if !content.is_ascii() {
        return None;
    }
    let name_byte = |b: &u8| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-');
    for (at, _) in content.iter().enumerate().filter(|(_, b)| **b == b'@') {
        let len = content[at + 1..]
            .iter()
            .take_while(|b| name_byte(b))
            .count();
        let next = content.get(at + 1 + len);
        if len > 0 && next.is_none_or(|b| matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')) {
            return Some(content[at + 1..at + 1 + len].to_vec());
        }
    }
    Some(b"other".to_vec())
}
