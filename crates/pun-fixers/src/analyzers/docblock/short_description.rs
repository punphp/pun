//! `PhpCsFixer\DocBlock\ShortDescription`.

use super::DocBlock;

pub(crate) struct ShortDescription<'a> {
    doc: &'a DocBlock,
}

impl<'a> ShortDescription<'a> {
    pub(crate) fn new(doc: &'a DocBlock) -> Self {
        ShortDescription { doc }
    }

    pub(crate) fn get_end(&self) -> Option<usize> {
        let mut reached_content = false;
        for (index, line) in self.doc.get_lines().iter().enumerate() {
            let line = line.borrow();
            if reached_content && (line.contains_a_tag() || !line.contains_useful_content()) {
                return Some(index - 1);
            }
            if line.contains_a_tag() {
                return None;
            }
            if line.contains_useful_content() {
                reached_content = true;
            }
        }
        None
    }
}
