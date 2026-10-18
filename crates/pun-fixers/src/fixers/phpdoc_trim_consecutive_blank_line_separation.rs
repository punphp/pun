//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocTrimConsecutiveBlankLineSeparationFixer`.
use crate::analyzers::docblock::{DocBlock, ShortDescription};
use pun_format::Fixer;
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

struct PhpdocTrimConsecutiveBlankLineSeparationFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocTrimConsecutiveBlankLineSeparationFixer)
}

impl Fixer for PhpdocTrimConsecutiveBlankLineSeparationFixer {
    fn name(&self) -> &'static str {
        "phpdoc_trim_consecutive_blank_line_separation"
    }
    fn priority(&self) -> i32 {
        -41
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(Kind::T_DOC_COMMENT) {
                continue;
            }
            let doc = DocBlock::new(&tokens[index].content);
            if let Some(summary_end) = ShortDescription::new(&doc).get_end() {
                fix_summary(&doc, summary_end);
                fix_description(&doc, summary_end);
            }
            fix_all_the_rest(&doc);
            tokens.set(index, Token::new(Kind::T_DOC_COMMENT, doc.get_content()));
        }
    }
}

fn fix_summary(doc: &DocBlock, summary_end: usize) {
    // findNonBlankLine: the closing line always qualifies, so it is never null
    let non_blank_after_summary = (summary_end + 1..doc.get_lines().len())
        .find(|&i| {
            let line = doc.get_lines()[i].borrow();
            line.contains_a_tag() || line.contains_useful_content() || line.is_the_end()
        })
        .unwrap_or(0);
    remove_extra_blank_lines_between(doc, summary_end, non_blank_after_summary);
}

fn fix_description(doc: &DocBlock, summary_end: usize) {
    let annotation_start = find_first_annotation_or_end(doc);
    // assuming the end of the Description appears before the first Annotation
    let description_end = match reverse_find_last_useful_content(doc, annotation_start) {
        Some(end) if end != summary_end && annotation_start != doc.get_lines().len() - 1 => end,
        _ => return, // no Description, or no content after it
    };
    remove_extra_blank_lines_between(doc, description_end, annotation_start);
}

fn fix_all_the_rest(doc: &DocBlock) {
    let annotation_start = find_first_annotation_or_end(doc);
    if let Some(last_line) = reverse_find_last_useful_content(doc, doc.get_lines().len() - 1)
        && annotation_start != last_line
    {
        remove_extra_blank_lines_between(doc, annotation_start, last_line);
    }
}

fn remove_extra_blank_lines_between(doc: &DocBlock, from: usize, to: usize) {
    for index in from + 1..to {
        let next = doc.get_lines()[index + 1].borrow().clone();
        let mut current = doc.get_lines()[index].borrow_mut();
        if !current.is_the_end()
            && !current.contains_useful_content()
            && !next.is_the_end()
            && !next.contains_useful_content()
        {
            current.remove();
        }
    }
}

fn find_first_annotation_or_end(doc: &DocBlock) -> usize {
    let lines = doc.get_lines();
    lines
        .iter()
        .position(|l| l.borrow().contains_a_tag())
        .unwrap_or(lines.len() - 1)
}

fn reverse_find_last_useful_content(doc: &DocBlock, from: usize) -> Option<usize> {
    (0..from)
        .rev()
        .find(|&i| doc.get_lines()[i].borrow().contains_useful_content())
}
