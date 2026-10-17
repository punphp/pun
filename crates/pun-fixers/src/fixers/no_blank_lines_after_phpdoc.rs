//! Port of `PhpCsFixer\Fixer\Phpdoc\NoBlankLinesAfterPhpdocFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

struct NoBlankLinesAfterPhpdocFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoBlankLinesAfterPhpdocFixer)
}

/// Cleanup a whitespace token.
fn fix_whitespace(tokens: &mut Tokens, index: usize) {
    let content = tokens[index].get_content();
    // if there is more than one new line in the whitespace, then we need to fix it
    if content.iter().filter(|&&b| b == b'\n').count() > 1 {
        // the final bit of the whitespace must be the next statement's indentation
        let last = content.iter().rposition(|&b| b == b'\n').unwrap();
        let new = content[last..].to_vec();
        tokens.set(index, Token::new(T_WHITESPACE, new));
    }
}

impl Fixer for NoBlankLinesAfterPhpdocFixer {
    fn name(&self) -> &'static str {
        "no_blank_lines_after_phpdoc"
    }
    fn priority(&self) -> i32 {
        // Must run before HeaderCommentFixer, PhpdocAlignFixer.
        // Must run after AlignMultilineCommentFixer, CommentToPhpdocFixer, PhpdocIndentFixer, PhpdocScalarFixer,
        // PhpdocToCommentFixer, PhpdocTypesFixer.
        -20
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            // get the next non-whitespace token inc comments, provided
            // that there is whitespace between it and the current token
            let next = tokens.get_next_non_whitespace(index, None);
            if let Some(next) = next
                && index + 2 == next
                && !tokens[next].is_given_kind(&[
                    T_BREAK,
                    T_COMMENT,
                    T_CONTINUE,
                    T_DECLARE,
                    T_DOC_COMMENT,
                    T_GOTO,
                    T_INCLUDE,
                    T_INCLUDE_ONCE,
                    T_NAMESPACE,
                    T_REQUIRE,
                    T_REQUIRE_ONCE,
                    T_RETURN,
                    T_THROW,
                    T_USE,
                    T_WHITESPACE,
                ])
            {
                fix_whitespace(tokens, index + 1);
            }
        }
    }
}
