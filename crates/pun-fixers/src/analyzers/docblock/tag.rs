//! `PhpCsFixer\DocBlock\Tag`.

use super::line::LineRef;
use super::preg;
use std::cell::RefCell;

pub const PSR_STANDARD_TAGS: &[&str] = &[
    "api", "author", "category", "copyright", "deprecated", "example", "global", "internal", "license", "link", "method", "package",
    "param", "property", "property-read", "property-write", "return", "see", "since", "subpackage", "throws", "todo", "uses", "var",
    "version",
];

#[derive(Clone, Debug)]
pub struct Tag {
    line: LineRef,
    name: RefCell<Option<Vec<u8>>>,
}

impl Tag {
    pub fn new(line: LineRef) -> Self {
        Tag { line, name: RefCell::new(None) }
    }

    pub fn get_name(&self) -> Vec<u8> {
        if self.name.borrow().is_none() {
            let content = self.line.borrow().get_content().to_vec();
            let name = match preg::match_all(r"/@[a-zA-Z0-9_-]+(?=\s|$)/", &content).first() {
                Some(m) => m.str(0)[1..].to_vec(),
                None => b"other".to_vec(),
            };
            *self.name.borrow_mut() = Some(name);
        }
        self.name.borrow().clone().unwrap()
    }

    /// Panics like upstream's RuntimeException on an unknown (`other`) tag.
    pub fn set_name(&self, name: &[u8]) {
        let current = self.get_name();
        if current == b"other" {
            panic!("Cannot set name on unknown tag.");
        }
        let pattern = [&b"/@"[..], &current, b"/"].concat();
        let new = preg::replace(&pattern, &[b"@", name].concat(), self.line.borrow().get_content(), 1);
        self.line.borrow_mut().set_content(new);
        *self.name.borrow_mut() = Some(name.to_vec());
    }

    pub fn valid(&self) -> bool {
        PSR_STANDARD_TAGS.iter().any(|t| t.as_bytes() == self.get_name())
    }
}
