//! Port of `PhpCsFixer\Fixer\StringNotation\HeredocClosingMarkerFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{mb_strtoupper, preg_quote, trim};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

const RESERVED_CLOSING_MARKERS: &[&str] = &[
    "CSS", "DIFF", "HTML", "JS", "JSON", "MD", "PHP", "PYTHON", "RST", "TS", "SQL", "XML", "YAML",
];

struct HeredocClosingMarkerFixer {
    closing_marker: Vec<u8>,
    reserved_closing_markers: Vec<String>,
    explicit_heredoc_style: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(HeredocClosingMarkerFixer {
        closing_marker: b"EOD".to_vec(),
        reserved_closing_markers: vec![],
        explicit_heredoc_style: false,
    })
}

impl Fixer for HeredocClosingMarkerFixer {
    fn name(&self) -> &'static str {
        "heredoc_closing_marker"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_START_HEREDOC)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &[
                "closing_marker",
                "reserved_closing_markers",
                "explicit_heredoc_style",
            ],
        )?;
        self.closing_marker = o.string("closing_marker", "EOD", &[])?.into_bytes();
        self.reserved_closing_markers =
            o.strings("reserved_closing_markers", RESERVED_CLOSING_MARKERS, &[])?;
        self.explicit_heredoc_style = o.bool("explicit_heredoc_style", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // `$reservedClosingMarkersMap[mb_strtoupper($v)] = $v`: a later duplicate key replaces the value.
        let mut reserved: Vec<(Vec<u8>, &[u8])> = vec![];
        for v in &self.reserved_closing_markers {
            let key = mb_strtoupper(v.as_bytes());
            match reserved.iter_mut().find(|(k, _)| *k == key) {
                Some(e) => e.1 = v.as_bytes(),
                None => reserved.push((key, v.as_bytes())),
            }
        }
        let mut start_index = None;
        for index in 0..tokens.count() {
            if tokens[index].is_kind(T_START_HEREDOC) {
                start_index = Some(index);
                continue;
            }
            let Some(start) = start_index else { continue };
            if !tokens[index].is_kind(T_END_HEREDOC) {
                continue;
            }
            let mut existing = mb_strtoupper(trim(tokens[index].get_content()));
            let mut new_closing_marker = loop {
                let found = reserved
                    .iter()
                    .find(|(k, _)| *k == existing)
                    .map(|(_, v)| v.to_vec());
                if !existing.ends_with(b"_") {
                    break found;
                }
                existing.pop();
                if found.is_some() {
                    break found;
                }
            }
            .unwrap_or_else(|| self.closing_marker.clone());

            let content = tokens.generate_partial_code(start + 1, index - 1);
            loop {
                let pattern = [
                    &b"~(^|[\\r\\n])\\s*"[..],
                    &preg_quote(&new_closing_marker, Some(b'~')),
                    b"(?!\\w)~",
                ]
                .concat();
                if Preg::match_groups_uncached(pattern, &content).is_none() {
                    break;
                }
                new_closing_marker.push(b'_');
            }

            let (start_token, end_token) =
                self.convert_closing_marker(&tokens[start], &tokens[index], &new_closing_marker);
            tokens.set(start, start_token);
            tokens.set(index, end_token);
            start_index = None;
        }
    }
}

impl HeredocClosingMarkerFixer {
    fn convert_closing_marker(&self, start: &Token, end: &Token, marker: &[u8]) -> (Token, Token) {
        let is_nowdoc = start.get_content().contains(&b'\'');
        let quote: &[u8] = if is_nowdoc {
            b"'"
        } else if self.explicit_heredoc_style {
            b"\""
        } else {
            b""
        };
        let replacement = [quote, marker, quote].concat();
        (
            Token::new(
                start.id.unwrap(),
                Preg::replace(
                    "/<<<\\h*\\K[\"']?[^\\s\"']+[\"']?/",
                    &replacement,
                    start.get_content(),
                ),
            ),
            Token::new(
                end.id.unwrap(),
                Preg::replace("/\\S+/", marker, end.get_content()),
            ),
        )
    }
}
