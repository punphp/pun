//! `PhpCsFixer\DocBlock\Tag`.

use super::line::LineRef;
use crate::analyzers::preg::Preg;
use pun_tokens::php_compat::isspace;
use std::cell::OnceCell;

#[derive(Clone, Debug)]
pub(crate) struct Tag {
    line: LineRef,
    name: OnceCell<Vec<u8>>,
}

impl Tag {
    pub(crate) fn new(line: LineRef) -> Self {
        Tag {
            line,
            name: OnceCell::new(),
        }
    }

    pub(crate) fn get_name(&self) -> &[u8] {
        self.name.get_or_init(|| {
            let line = self.line.borrow();
            let content = line.get_content();
            match tag_name_ascii(content) {
                Some(name) => name,
                None => match Preg::match_all(r"/@[a-zA-Z0-9_-]+(?=\s|$)/", content).first() {
                    Some(m) => m.str(0)[1..].to_vec(),
                    None => b"other".to_vec(),
                },
            }
        })
    }

    /// `valid()`: the name is one of `PSR_STANDARD_TAGS`.
    pub(crate) fn valid(&self) -> bool {
        PSR_STANDARD_TAGS
            .iter()
            .any(|t| t.as_bytes() == self.get_name())
    }
}

pub(super) const PSR_STANDARD_TAGS: [&str; 25] = [
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
        if len > 0 && next.is_none_or(|&b| isspace(b)) {
            return Some(content[at + 1..at + 1 + len].to_vec());
        }
    }
    Some(b"other".to_vec())
}
