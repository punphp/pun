//! Port of `PhpCsFixer\Fixer\Whitespace\HeredocIndentationFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::php_compat::substr;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct HeredocIndentationFixer {
    start_plus_one: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(HeredocIndentationFixer {
        start_plus_one: true,
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

impl Fixer for HeredocIndentationFixer {
    fn name(&self) -> &'static str {
        "heredoc_indentation"
    }
    fn priority(&self) -> i32 {
        -26
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_START_HEREDOC)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["indentation"])?;
        self.start_plus_one = o.string(
            "indentation",
            "start_plus_one",
            &["start_plus_one", "same_as_start"],
        )? == "start_plus_one";
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count();
        while index > 0 {
            index -= 1;
            if !tokens[index].is_kind(T_END_HEREDOC) {
                continue;
            }
            let end = index;
            index = tokens
                .get_prev_token_of_kind(index, &[T_START_HEREDOC.into()], true)
                .unwrap();
            self.fix_indentation(tokens, index, end);
        }
    }
}

impl HeredocIndentationFixer {
    fn fix_indentation(&self, tokens: &mut Tokens, start: usize, end: usize) {
        let mut indent = WhitespacesAnalyzer::detect_indent(tokens, start);
        if self.start_plus_one {
            indent.extend_from_slice(self.whitespaces_config.get_indent().as_bytes());
        }

        let end_content = tokens[end].get_content();
        let current_indent = Preg::match_groups(r"/^\h*/", end_content)
            .unwrap()
            .str(0)
            .to_vec();
        let current_indent_length = current_indent.len();

        let content = [
            &indent[..],
            substr(end_content, current_indent_length, None),
        ]
        .concat();
        tokens.set(end, Token::new(T_END_HEREDOC, content));

        if end == start + 1 {
            return;
        }

        let mut index = end - 1;
        let mut last = true;
        while index > start {
            if tokens[index].is_given_kind(&[T_ENCAPSED_AND_WHITESPACE, T_WHITESPACE]) {
                let mut content = tokens[index].get_content().to_vec();
                if !current_indent.is_empty() {
                    let pattern = [&b"/(?<=\\v)(?!"[..], &current_indent, b")\\h+/"].concat();
                    content = Preg::replace_limit_uncached(pattern, "", content, -1).0;
                }
                let regex_end: &[u8] = if last && current_indent.is_empty() {
                    b"(?!\\v|$)"
                } else {
                    b"(?!\\v)"
                };
                let pattern = [&b"/(?<=\\v)"[..], &current_indent, regex_end, b"/"].concat();
                content = Preg::replace_limit_uncached(pattern, &indent, content, -1).0;
                let id = tokens[index].get_id().unwrap();
                tokens.set(index, Token::new(id, content));
            }
            index -= 1;
            last = false;
        }

        index += 1;

        if !tokens[index].is_kind(T_ENCAPSED_AND_WHITESPACE) {
            tokens.insert_at(index, vec![Token::new(T_ENCAPSED_AND_WHITESPACE, indent)]);
            return;
        }

        let mut content = tokens[index].get_content().to_vec();
        if !matches!(content.first(), Some(b'\r' | b'\n'))
            && (current_indent.is_empty() || content.starts_with(&current_indent))
        {
            content = [&indent[..], substr(&content, current_indent_length, None)].concat();
        } else if !current_indent.is_empty() {
            let pattern = [&b"/^(?!"[..], &current_indent, b")\\h+/"].concat();
            content = Preg::replace_limit_uncached(pattern, "", content, -1).0;
        }
        tokens.set(index, Token::new(T_ENCAPSED_AND_WHITESPACE, content));
    }
}
