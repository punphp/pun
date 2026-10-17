//! Port of `PhpCsFixer\Fixer\CastNotation\CastSpacesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, token::CAST_KINDS};
use serde_json::Value;
use std::path::Path;

struct CastSpacesFixer {
    space: String,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(CastSpacesFixer {
        space: "single".into(),
    })
}

impl Fixer for CastSpacesFixer {
    fn name(&self) -> &'static str {
        "cast_spaces"
    }
    fn priority(&self) -> i32 {
        // Must run after NoShortBoolCastFixer.
        -10
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["space"])?;
        self.space = r.string("space", "single", &["none", "single"])?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CAST_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        while index < tokens.count() {
            let token = &tokens[index];
            if !token.is_cast() {
                index += 1;
                continue;
            }
            // strtr with INSIDE_CAST_SPACE_REPLACE_MAP
            let content: Vec<u8> = token
                .get_content()
                .iter()
                .copied()
                .filter(|b| !b" \t\n\r\0\x0B".contains(b))
                .collect();
            let t = Token {
                id: token.get_id(),
                content,
            };
            tokens.set(index, t);
            if self.space == "single" {
                // force single whitespace after cast token:
                if tokens[index + 1].is_whitespace(Some(b" \t")) {
                    // - if next token is whitespaces that contains only spaces and tabs - override next token with single space
                    tokens.set(index + 1, Token::new(T_WHITESPACE, " "));
                } else if !tokens[index + 1].is_whitespace(None) {
                    // - if next token is not whitespaces that contains spaces, tabs and new lines - append single space to current token
                    tokens.insert_at(index + 1, vec![Token::new(T_WHITESPACE, " ")]);
                }
                index += 1;
                continue;
            }
            // force no whitespace after cast token:
            if tokens[index + 1].is_whitespace(None) {
                tokens.clear_at(index + 1);
            }
            index += 1;
        }
    }
}
