//! Port of `PhpCsFixer\Fixer\Comment\SingleLineCommentStyleFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, php_compat};
use serde_json::Value;
use std::path::Path;

struct SingleLineCommentStyleFixer {
    asterisk_enabled: bool,
    hash_enabled: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SingleLineCommentStyleFixer {
        asterisk_enabled: true,
        hash_enabled: true,
    })
}

impl Fixer for SingleLineCommentStyleFixer {
    fn name(&self) -> &'static str {
        "single_line_comment_style"
    }
    fn priority(&self) -> i32 {
        -31
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["comment_types"])?;
        let types = o.strings(
            "comment_types",
            &["asterisk", "hash"],
            &["asterisk", "hash"],
        )?;
        self.asterisk_enabled = types.iter().any(|t| t == "asterisk");
        self.hash_enabled = types.iter().any(|t| t == "hash");
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_COMMENT) {
                continue;
            }
            let token = tokens[index].clone();
            let content = &token.content;
            // substr($content, 2, -2) ?: ''
            let comment_content: &[u8] = if content.len() > 4 {
                &content[2..content.len() - 2]
            } else {
                b""
            };

            if self.hash_enabled && content.starts_with(b"#") {
                // upstream skips `#[` (an attribute before PHP 8); PHP 8 never lexes it as a comment
                tokens.set(
                    index,
                    Token {
                        id: token.id,
                        content: [&b"//"[..], &content[1..]].concat(),
                    },
                );
                continue;
            }

            if !self.asterisk_enabled
                || comment_content.windows(2).any(|w| w == b"?>")
                || !content.starts_with(b"/*")
                || Preg::r#match("/[^\\s\\*].*\\R.*[^\\s\\*]/s", comment_content)
            {
                continue;
            }

            let next_token_index = index + 1;
            if next_token_index < tokens.count() {
                let next_token = tokens[next_token_index].clone();
                if !next_token.is_whitespace(None) || !Preg::r#match("/\\R/", &next_token.content) {
                    continue;
                }
                tokens.set(
                    next_token_index,
                    Token {
                        id: next_token.id,
                        content: php_compat::ltrim_chars(&next_token.content, b" \t").to_vec(),
                    },
                );
            }

            let mut new_content = b"//".to_vec();
            if Preg::r#match("/[^\\s\\*]/", comment_content) {
                new_content = [
                    &b"// "[..],
                    &Preg::replace(
                        "/[\\s\\*]*([^\\s\\*](?:.+[^\\s\\*])?)[\\s\\*]*/",
                        "\\1",
                        comment_content,
                    ),
                ]
                .concat();
            }
            tokens.set(
                index,
                Token {
                    id: token.id,
                    content: new_content,
                },
            );
        }
    }
}
