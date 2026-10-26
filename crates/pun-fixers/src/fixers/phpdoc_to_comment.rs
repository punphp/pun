//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocToCommentFixer`.
use crate::analyzers::comments_analyzer::{
    is_before_return, is_before_structural_element, is_header_comment,
};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct PhpdocToCommentFixer {
    ignored_tags: Vec<Vec<u8>>,
    allow_before_return_statement: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocToCommentFixer {
        ignored_tags: Vec::new(),
        // Future::getV4OrV3(true, false): the 3.x default.
        allow_before_return_statement: false,
    })
}

impl Fixer for PhpdocToCommentFixer {
    fn name(&self) -> &'static str {
        "phpdoc_to_comment"
    }
    fn priority(&self) -> i32 {
        25
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &["ignored_tags", "allow_before_return_statement"],
        )?;
        self.ignored_tags = o
            .strings("ignored_tags", &[], &[])?
            .iter()
            .map(|t| strtolower(t.as_bytes()))
            .collect();
        self.allow_before_return_statement = o.bool("allow_before_return_statement", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        'tokens: for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            if is_header_comment(tokens, index) {
                continue;
            }
            if self.allow_before_return_statement && is_before_return(tokens, index) {
                continue;
            }
            if is_before_structural_element(tokens, index) {
                continue;
            }
            for m in Preg::match_all(r"~\@([a-zA-Z0-9_\\\-]+)\b~", tokens[index].get_content()) {
                if self.ignored_tags.contains(&strtolower(m.str(1))) {
                    continue 'tokens;
                }
            }
            let content = tokens[index].get_content();
            let trimmed = &content[content
                .iter()
                .take_while(|&&c| c == b'/' || c == b'*')
                .count()..];
            tokens.set(index, Token::new(T_COMMENT, [&b"/*"[..], trimmed].concat()));
        }
    }
}
