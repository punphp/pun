//! Port of `PhpCsFixer\Fixer\Phpdoc\AlignMultilineCommentFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{TRIM_CHARS, ltrim_chars};
use pun_tokens::{Kind, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct AlignMultilineCommentFixer {
    comment_type: String,
    token_kinds: Vec<Kind>,
    line_ending: Vec<u8>,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(AlignMultilineCommentFixer {
        comment_type: String::new(),
        token_kinds: vec![],
        line_ending: b"\n".to_vec(),
    })
}

impl Fixer for AlignMultilineCommentFixer {
    fn name(&self) -> &'static str {
        "align_multiline_comment"
    }
    fn priority(&self) -> i32 {
        27
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.line_ending = config.get_line_ending().as_bytes().to_vec();
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let c = resolve(self.name(), options, &["comment_type"])?;
        self.comment_type = c.string(
            "comment_type",
            "phpdocs_only",
            &["phpdocs_only", "phpdocs_like", "all_multiline"],
        )?;
        self.token_kinds = vec![T_DOC_COMMENT];
        if self.comment_type != "phpdocs_only" {
            self.token_kinds.push(T_COMMENT);
        }
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&self.token_kinds)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            let token = &tokens[index];
            if !token.is_given_kind(&self.token_kinds) {
                continue;
            }
            let mut whitespace = vec![];
            let mut previous_index = index - 1;
            if tokens[previous_index].is_whitespace(None) {
                whitespace = tokens[previous_index].content.clone();
                previous_index -= 1;
            }
            if tokens[previous_index].is_kind(T_OPEN_TAG) {
                whitespace = [
                    Preg::replace(r"/\S/", "", &tokens[previous_index].content),
                    whitespace,
                ]
                .concat();
            }
            let Some(m) = Preg::match_groups(r"/\R(\h*)$/", &whitespace) else {
                continue;
            };
            if token.is_kind(T_COMMENT)
                && self.comment_type != "all_multiline"
                && Preg::r#match(r"/\R(?:\R|\s*[^\s\*])/", &token.content)
            {
                continue;
            }
            let indentation = m.str(1).to_vec();
            let mut lines = Preg::split_str(r"/\R/u", &token.content);
            for line in lines.iter_mut().skip(1) {
                let trimmed = ltrim_chars(line, TRIM_CHARS);
                if token.is_kind(T_COMMENT) && trimmed.first() != Some(&b'*') {
                    continue;
                }
                let trimmed: Vec<u8> = match trimmed.first() {
                    None => b"*".to_vec(),
                    Some(b'*') => trimmed.to_vec(),
                    Some(_) => [b"* ", trimmed].concat(),
                };
                *line = [&indentation[..], b" ", &trimmed].concat();
            }
            let id = token.id.unwrap();
            tokens.set(index, Token::new(id, lines.join(&self.line_ending[..])));
        }
    }
}
