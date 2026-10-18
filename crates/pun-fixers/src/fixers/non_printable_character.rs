//! Port of `PhpCsFixer\Fixer\Basic\NonPrintableCharacterFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::php_compat::str_replace;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct NonPrintableCharacterFixer {
    use_escape_sequences_in_strings: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(NonPrintableCharacterFixer {
        use_escape_sequences_in_strings: true,
    })
}

const TOKENS: &[Kind] = &[
    T_STRING_VARNAME,
    T_INLINE_HTML,
    T_VARIABLE,
    T_COMMENT,
    T_ENCAPSED_AND_WHITESPACE,
    T_CONSTANT_ENCAPSED_STRING,
    T_DOC_COMMENT,
];

/// `$symbolsReplace`: character => [replacement, codepoint].
const SYMBOLS_REPLACE: &[(&[u8], &[u8], &str)] = &[
    (b"\xe2\x80\x8b", b"", "200b"),  // ZWSP U+200B
    (b"\xe2\x80\x87", b" ", "2007"), // FIGURE SPACE U+2007
    (b"\xe2\x80\xaf", b" ", "202f"), // NBSP U+202F
    (b"\xe2\x81\xa0", b"", "2060"),  // WORD JOINER U+2060
    (b"\xc2\xa0", b" ", "a0"),       // NO-BREAK SPACE U+A0
];

/// `strtr($content, $map)`: no key is a prefix of another, so a left-to-right scan is strtr's longest-first match.
fn strtr(content: &[u8], map: &[(&[u8], Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len());
    let mut i = 0;
    'scan: while i < content.len() {
        for (from, to) in map {
            if content[i..].starts_with(from) {
                out.extend_from_slice(to);
                i += from.len();
                continue 'scan;
            }
        }
        out.push(content[i]);
        i += 1;
    }
    out
}

impl Fixer for NonPrintableCharacterFixer {
    fn name(&self) -> &'static str {
        "non_printable_character"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["use_escape_sequences_in_strings"])?;
        self.use_escape_sequences_in_strings = c.bool("use_escape_sequences_in_strings", true)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(TOKENS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let replacements: Vec<(&[u8], Vec<u8>)> = SYMBOLS_REPLACE
            .iter()
            .map(|&(c, r, _)| (c, r.to_vec()))
            .collect();
        let escape_sequences: Vec<(&[u8], Vec<u8>)> = SYMBOLS_REPLACE
            .iter()
            .map(|&(c, _, cp)| (c, format!("\\u{{{cp}}}").into_bytes()))
            .collect();
        let mut pattern = b"/".to_vec();
        for (i, (c, _)) in escape_sequences.iter().enumerate() {
            if i > 0 {
                pattern.push(b'|');
            }
            pattern.extend_from_slice(c);
        }
        pattern.push(b'/');
        for index in 0..tokens.len() {
            let token = tokens[index].clone();
            let mut content = token.get_content().to_vec();
            if self.use_escape_sequences_in_strings
                && token.is_given_kind(&[T_CONSTANT_ENCAPSED_STRING, T_ENCAPSED_AND_WHITESPACE])
            {
                if !Preg::r#match(&pattern, &content) {
                    continue;
                }
                let previous_token = &tokens[index - 1];
                let mut string_type_changed = false;
                let mut swap_quotes = false;
                if previous_token.is_kind(T_START_HEREDOC) {
                    let previous_token_content = previous_token.get_content();
                    if previous_token_content.contains(&b'\'') {
                        let new = str_replace(b"'", b"", previous_token_content);
                        tokens.set(index - 1, Token::new(T_START_HEREDOC, new));
                        string_type_changed = true;
                    }
                } else if content.starts_with(b"'") {
                    string_type_changed = true;
                    swap_quotes = true;
                }
                if swap_quotes {
                    content = str_replace(b"\\'", b"'", &content);
                }
                if string_type_changed {
                    content = Preg::replace("/(\\\\{1,2})/", "\\\\\\\\", &content);
                    content = str_replace(b"$", b"\\$", &content);
                }
                if swap_quotes {
                    content = str_replace(b"\"", b"\\\"", &content);
                    content = Preg::replace("/^'(.*)'$/s", "\"$1\"", &content);
                }
                tokens.set(
                    index,
                    Token::new(token.id.unwrap(), strtr(&content, &escape_sequences)),
                );
                continue;
            }
            if token.is_given_kind(TOKENS) {
                let new_content = strtr(&content, &replacements);
                // variable name cannot contain space
                if token.is_given_kind(&[T_STRING_VARNAME, T_VARIABLE])
                    && new_content.contains(&b' ')
                {
                    continue;
                }
                // multiline comment must have "*/" only at the end
                if token.is_given_kind(&[T_COMMENT, T_DOC_COMMENT])
                    && new_content.starts_with(b"/*")
                    && new_content.windows(2).position(|w| w == b"*/")
                        != Some(new_content.len() - 2)
                {
                    continue;
                }
                tokens.set(index, Token::new(token.id.unwrap(), new_content));
            }
        }
    }
}
