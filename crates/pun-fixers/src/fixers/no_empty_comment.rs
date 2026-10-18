//! Port of `PhpCsFixer\Fixer\Comment\NoEmptyCommentFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use std::path::Path;

struct NoEmptyCommentFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoEmptyCommentFixer)
}

#[derive(PartialEq, Clone, Copy)]
enum CommentType {
    Hash,
    DoubleSlash,
    SlashAsterisk,
}

impl Fixer for NoEmptyCommentFixer {
    fn name(&self) -> &'static str {
        "no_empty_comment"
    }
    fn priority(&self) -> i32 {
        // Must run before NoExtraBlankLinesFixer, NoTrailingWhitespaceFixer, NoWhitespaceInBlankLineFixer,
        // NoWhitespaceInEmptyArrayFixer.
        // Must run after PhpdocToCommentFixer.
        2
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let count = tokens.count();
        let mut index = 1;
        while index < count {
            if tokens[index].is_kind(T_COMMENT) {
                let (block_start, block_end, is_empty) = get_comment_block(tokens, index);
                index = block_end;
                if is_empty {
                    for i in block_start..=index {
                        tokens.clear_token_and_merge_surrounding_whitespace(i);
                    }
                }
            }
            index += 1;
        }
    }
}

/// `[blockStart, blockEnd, isEmpty]`.
fn get_comment_block(tokens: &Tokens, mut index: usize) -> (usize, usize, bool) {
    let comment_type = get_comment_type(tokens[index].get_content());
    let mut empty = is_empty_comment(tokens[index].get_content());
    if comment_type == CommentType::SlashAsterisk {
        return (index, index, empty);
    }
    let start = index;
    let count = tokens.count();
    index += 1;
    while index < count {
        if tokens[index].is_comment() {
            if comment_type != get_comment_type(tokens[index].get_content()) {
                break;
            }
            // don't retest if already known the block not being empty
            if empty {
                empty = is_empty_comment(tokens[index].get_content());
            }
            index += 1;
            continue;
        }
        if !tokens[index].is_whitespace(None) || get_line_break_count(tokens, index, index + 1) > 1
        {
            break;
        }
        index += 1;
    }
    (start, index - 1, empty)
}

fn get_comment_type(content: &[u8]) -> CommentType {
    if content.starts_with(b"#") {
        return CommentType::Hash;
    }
    if content[1] == b'*' {
        return CommentType::SlashAsterisk;
    }
    CommentType::DoubleSlash
}

fn get_line_break_count(tokens: &Tokens, white_start: usize, white_end: usize) -> usize {
    (white_start..white_end)
        .map(|i| Preg::match_all(r"/\R/u", tokens[i].get_content()).len())
        .sum()
}

fn is_empty_comment(content: &[u8]) -> bool {
    Preg::r#match(
        match get_comment_type(content) {
            // single line comment starting with '#'
            CommentType::Hash => r"|^#\s*$|",
            // comment starting with '/*' and ending with '*/' (but not a PHPDoc)
            CommentType::SlashAsterisk => r"|^/\*[\s\*]*\*+/$|",
            // single line comment starting with '//'
            CommentType::DoubleSlash => r"|^//\s*$|",
        },
        content,
    )
}
