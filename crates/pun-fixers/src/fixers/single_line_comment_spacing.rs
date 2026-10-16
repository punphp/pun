//! Port of `PhpCsFixer\Fixer\Comment\SingleLineCommentSpacingFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{php_compat, Kind::*, Token, Tokens};
use std::path::Path;

struct SingleLineCommentSpacingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SingleLineCommentSpacingFixer)
}

impl Fixer for SingleLineCommentSpacingFixer {
    fn name(&self) -> &'static str {
        "single_line_comment_spacing"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_kind(T_COMMENT) {
                continue;
            }
            let content = tokens[index].content.clone();
            let content_length = content.len();
            let new_content;
            if content[0] == b'/' {
                if content_length < 3 {
                    continue; // cheap check for "//"
                }
                if content[1] == b'*' {
                    // slash asterisk comment
                    if content_length < 5 || content[2] == b'*' || content.contains(&b'\n') {
                        continue; // cheap check for "/**/", comment that looks like a PHPDoc, or multi line comment
                    }
                    let c = [php_compat::rtrim(&content[..content_length - 2]), b" ", &content[content_length - 2..]].concat();
                    new_content = fix_comment_leading_space(&c, "/*");
                } else {
                    // double slash comment
                    new_content = fix_comment_leading_space(&content, "//");
                }
            } else {
                // hash comment
                if content_length < 2 || content[1] == b'[' {
                    continue; // cheap check for "#" or annotation (like) comment
                }
                new_content = fix_comment_leading_space(&content, "#");
            }
            if new_content != content {
                tokens.set(index, Token::new(T_COMMENT, new_content));
            }
        }
    }
}

// fix space between comment open and leading text
fn fix_comment_leading_space(content: &[u8], prefix: &str) -> Vec<u8> {
    let quoted = match prefix {
        "/*" => "/\\*",
        p => p,
    };
    if Preg::r#match(format!("@^{quoted}\\h+.*$@"), content) {
        return content.to_vec();
    }
    let position = prefix.len();
    [&content[..position], b" ", &content[position..]].concat()
}
