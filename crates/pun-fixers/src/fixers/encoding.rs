//! Port of `PhpCsFixer\Fixer\Basic\EncodingFixer`.
use pun_format::Fixer;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct EncodingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(EncodingFixer)
}

const BOM: &[u8] = b"\xEF\xBB\xBF";

impl Fixer for EncodingFixer {
    fn name(&self) -> &'static str {
        "encoding"
    }
    fn priority(&self) -> i32 {
        // must run first (at least before Fixers that using Tokens) - for speed reason of whole fixing process
        100
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let content = tokens[0].get_content();
        if let Some(new_content) = content.strip_prefix(BOM) {
            if new_content.is_empty() {
                tokens.clear_at(0);
            } else {
                let t = Token {
                    id: tokens[0].get_id(),
                    content: new_content.to_vec(),
                };
                tokens.set(0, t);
            }
        }
    }
}
