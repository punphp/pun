//! `PhpCsFixer\DocBlock\TagComparator` (deprecated upstream).

use super::Tag;

pub const DEFAULT_GROUPS: &[&[&str]] = &[
    &["deprecated", "link", "see", "since"],
    &["author", "copyright", "license"],
    &["category", "package", "subpackage"],
    &["property", "property-read", "property-write"],
];

pub fn should_be_together(first: &Tag, second: &Tag, groups: &[&[&str]]) -> bool {
    let first_name = first.get_name();
    let second_name = second.get_name();
    if first_name == second_name {
        return true;
    }
    groups.iter().any(|g| {
        g.iter().any(|t| t.as_bytes() == first_name)
            && g.iter().any(|t| t.as_bytes() == second_name)
    })
}
