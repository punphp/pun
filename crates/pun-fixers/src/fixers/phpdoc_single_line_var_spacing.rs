//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocSingleLineVarSpacingFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{php_compat, Kind::*, Token, Tokens};
use std::path::Path;

struct PhpdocSingleLineVarSpacingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocSingleLineVarSpacingFixer)
}

impl Fixer for PhpdocSingleLineVarSpacingFixer {
    fn name(&self) -> &'static str {
        "phpdoc_single_line_var_spacing"
    }
    fn priority(&self) -> i32 {
        -10
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_COMMENT, T_DOC_COMMENT])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_comment() {
                continue;
            }
            let content = &tokens[index].content;
            let fixed_content = fix_token_content(content);
            if *content != fixed_content {
                tokens.set(index, Token::new(T_DOC_COMMENT, fixed_content));
            }
        }
    }
}

fn fix_token_content(content: &[u8]) -> Vec<u8> {
    Preg::replace_callback(
        "#^/\\*\\*\\h*@var\\h+(\\S+)\\h*(\\$\\S+)?\\h*([^\\n]*)\\*/$#",
        |m| {
            let mut content = b"/** @var".to_vec();
            for i in 1..m.len() {
                if !m.str(i).is_empty() {
                    content.push(b' ');
                    content.extend_from_slice(m.str(i));
                }
            }
            [php_compat::rtrim(&content), b" */"].concat()
        },
        content,
    )
}
