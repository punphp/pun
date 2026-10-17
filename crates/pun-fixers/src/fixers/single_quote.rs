//! Port of `PhpCsFixer\Fixer\StringNotation\SingleQuoteFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;
use std::sync::LazyLock;

struct SingleQuoteFixer {
    strings_containing_single_quote_chars: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SingleQuoteFixer {
        strings_containing_single_quote_chars: false,
    })
}

// regex: odd number of backslashes, not followed by double quote or dollar
static ESCAPE: LazyLock<pcre2::bytes::Regex> =
    LazyLock::new(|| pcre2::bytes::Regex::new(r#"(?<!\\)(?:\\{2})*\\(?!["$\\])"#).unwrap());

impl Fixer for SingleQuoteFixer {
    fn name(&self) -> &'static str {
        "single_quote"
    }
    fn priority(&self) -> i32 {
        10
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(
            self.name(),
            options,
            &["strings_containing_single_quote_chars"],
        )?;
        self.strings_containing_single_quote_chars =
            c.bool("strings_containing_single_quote_chars", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_CONSTANT_ENCAPSED_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            if !tokens[index].is_kind(T_CONSTANT_ENCAPSED_STRING) {
                continue;
            }
            let mut content = &tokens[index].content[..];
            let mut prefix: &[u8] = b"";
            if content[0].eq_ignore_ascii_case(&b'b') {
                prefix = &content[..1];
                content = &content[1..];
            }
            if content[0] == b'"'
                && (self.strings_containing_single_quote_chars || !content.contains(&b'\''))
                && !ESCAPE.is_match(content).unwrap_or(false)
            {
                let inner = &content[1..content.len() - 1];
                // str_replace(['\"', '\$', '\''], ['"', '$', '\\\''], ...): sequential passes.
                let inner = str_replace(b"\\\"", b"\"", inner);
                let inner = str_replace(b"\\$", b"$", &inner);
                let inner = str_replace(b"'", b"\\'", &inner);
                let new = [prefix, b"'", &inner, b"'"].concat();
                tokens.set(index, Token::new(T_CONSTANT_ENCAPSED_STRING, new));
            }
        }
    }
}

fn str_replace(search: &[u8], replace: &[u8], subject: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(subject.len());
    let mut i = 0;
    while i < subject.len() {
        if subject[i..].starts_with(search) {
            out.extend_from_slice(replace);
            i += search.len();
        } else {
            out.push(subject[i]);
            i += 1;
        }
    }
    out
}
