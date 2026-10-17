//! Port of `PhpCsFixer\Fixer\Whitespace\NoSpacesAroundOffsetFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use serde_json::Value;
use std::path::Path;

struct NoSpacesAroundOffsetFixer {
    positions: Vec<String>,
}

const VALUES: &[&str] = &["inside", "outside"];

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoSpacesAroundOffsetFixer {
        positions: VALUES.iter().map(|s| s.to_string()).collect(),
    })
}

impl Fixer for NoSpacesAroundOffsetFixer {
    fn name(&self) -> &'static str {
        "no_spaces_around_offset"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["positions"])?;
        self.positions = r.strings("positions", VALUES, VALUES)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("[") || tokens.is_token_kind_found(CT_ARRAY_INDEX_BRACE_OPEN)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let inside = self.positions.iter().any(|p| p == "inside");
        let outside = self.positions.iter().any(|p| p == "outside");
        for index in 0..tokens.count() {
            let token = &tokens[index];
            if !token.equals_any(&["[".into(), CT_ARRAY_INDEX_BRACE_OPEN.into()], true) {
                continue;
            }
            if inside {
                // `$a{0}` (CT_ARRAY_INDEX_BRACE_OPEN) is a parse error since PHP 8, kept for parity with upstream
                let block_type = if token.equals("[", true) {
                    Tokens::BLOCK_TYPE_INDEX_BRACKET
                } else {
                    Tokens::BLOCK_TYPE_INDEX_BRACE
                };
                let end_index = tokens.find_block_end(block_type, index);
                if tokens[index + 1].is_whitespace(Some(b" \t")) {
                    tokens.clear_at(index + 1);
                }
                if tokens[end_index - 1].is_whitespace(Some(b" \t")) {
                    tokens.clear_at(end_index - 1);
                }
            }
            if outside {
                let prev_non_whitespace_index =
                    tokens.get_prev_non_whitespace(index, None).unwrap();
                if tokens[prev_non_whitespace_index].is_comment() {
                    continue;
                }
                tokens.remove_leading_whitespace(index, None);
            }
        }
    }
}
