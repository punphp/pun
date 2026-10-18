//! `PhpCsFixer\DocBlock\Line`.

use crate::analyzers::preg::Preg;
use pun_tokens::php_compat::{isspace, trim};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Line {
    content: Vec<u8>,
}

/// Lines are shared between a DocBlock and its annotations/tags, like upstream's objects.
pub(crate) type LineRef = Rc<RefCell<Line>>;

impl Line {
    pub(crate) fn new(content: impl Into<Vec<u8>>) -> Self {
        Line {
            content: content.into(),
        }
    }

    pub(crate) fn shared(content: impl Into<Vec<u8>>) -> LineRef {
        Rc::new(RefCell::new(Self::new(content)))
    }

    pub(crate) fn get_content(&self) -> &[u8] {
        &self.content
    }

    pub(crate) fn contains_useful_content(&self) -> bool {
        Preg::r#match(r"/\*\s*\S+/", &self.content)
            && !trim(
                &self
                    .content
                    .iter()
                    .map(|&c| if c == b'/' || c == b'*' { b' ' } else { c })
                    .collect::<Vec<_>>(),
            )
            .is_empty()
    }

    pub(crate) fn contains_a_tag(&self) -> bool {
        if !self.content.is_ascii() {
            return Preg::r#match(r"/\*\s*@/", &self.content);
        }
        // ASCII `\s`: space, TAB, LF, VT, FF, CR.
        self.content.iter().enumerate().any(|(i, &b)| {
            b == b'*' && self.content[i + 1..].iter().find(|&&c| !isspace(c)) == Some(&b'@')
        })
    }

    pub(crate) fn is_the_start(&self) -> bool {
        self.content.windows(3).any(|w| w == b"/**")
    }

    pub(crate) fn is_the_end(&self) -> bool {
        self.content.windows(2).any(|w| w == b"*/")
    }

    pub(crate) fn set_content(&mut self, content: impl Into<Vec<u8>>) {
        self.content = content.into();
    }

    pub(crate) fn remove(&mut self) {
        self.content.clear();
    }

    pub(crate) fn add_blank(&mut self) {
        let Some(m) = Preg::match_groups(r"/^(\h*\*)[^\r\n]*(\r?\n)$/", &self.content) else {
            return;
        };
        let add = [m.str(1), m.str(2)].concat();
        self.content.extend(add);
    }
}
