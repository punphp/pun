//! Port of `PhpCsFixer\Fixer\Comment\CommentToPhpdocFixer`.
use crate::analyzers::comments_analyzer::{
    get_comment_block_indices, is_before_structural_element, is_header_comment,
};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use crate::analyzers::utils::Utils;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::php_compat::{ltrim_chars, rtrim_chars, strtolower, trim};
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct CommentToPhpdocFixer {
    ignored_tags: Vec<Vec<u8>>,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(CommentToPhpdocFixer::default())
}

impl Fixer for CommentToPhpdocFixer {
    fn name(&self) -> &'static str {
        "comment_to_phpdoc"
    }
    fn priority(&self) -> i32 {
        // Should be run before all other PHPDoc fixers
        26
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["ignored_tags"])?;
        self.ignored_tags = c
            .strings("ignored_tags", &[], &[])?
            .iter()
            .map(|t| strtolower(t.as_bytes()))
            .collect();
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let limit = tokens.count();
        let mut index = 0;
        while index < limit {
            if !tokens[index].is_kind(T_COMMENT)
                || is_header_comment(tokens, index)
                || !is_before_structural_element(tokens, index)
                || Preg::r#match(
                    r"~(?:#|//|/\*+|\R(?:\s*\*)?)\s*\@(?=\w+-(ignore|suppress))([a-zA-Z0-9_\\\-]+)(?=\s|\(|$)~",
                    tokens[index].get_content(),
                )
            {
                index += 1;
                continue;
            }
            let comment_indices = get_comment_block_indices(tokens, index);
            if self.is_comment_candidate(tokens, &comment_indices) {
                self.fix_comment(tokens, &comment_indices);
            }
            index = *comment_indices.iter().max().unwrap() + 1;
        }
    }
}

impl CommentToPhpdocFixer {
    fn is_comment_candidate(&self, tokens: &Tokens, indices: &[usize]) -> bool {
        indices.iter().any(|&index| {
            Preg::match_groups(
                r"~(?:#|//|/\*+|\R(?:\s*\*)?)\s*\@([a-zA-Z0-9_\\\-]+)(?=\s|\(|$)~",
                tokens[index].get_content(),
            )
            .is_some_and(|m| !self.ignored_tags.contains(&strtolower(m.str(1))))
        })
    }

    fn fix_comment(&self, tokens: &mut Tokens, indices: &[usize]) {
        if indices.len() == 1 {
            fix_comment_single_line(tokens, indices[0]);
        } else {
            self.fix_comment_multi_line(tokens, indices);
        }
    }

    fn fix_comment_multi_line(&self, tokens: &mut Tokens, indices: &[usize]) {
        let ending = self.whitespaces_config.get_line_ending().as_bytes();
        let start_index = indices[0];
        let indent = Utils::calculate_trailing_whitespace_indent(&tokens[start_index - 1]);
        let mut new_content = [b"/**", ending].concat();
        let count = *indices.iter().max().unwrap();
        for index in start_index..=count {
            if !tokens[index].is_comment() {
                continue;
            }
            let content = tokens[index].get_content();
            if content.windows(2).any(|w| w == b"*/") {
                return;
            }
            let mut message = get_message(content);
            if !trim(&message[..message.len().min(1)]).is_empty() {
                message.insert(0, b' ');
            }
            new_content.extend_from_slice(&indent);
            new_content.extend_from_slice(b" *");
            new_content.extend_from_slice(&message);
            new_content.extend_from_slice(ending);
        }
        for index in start_index..=count {
            tokens.clear_at(index);
        }
        new_content.extend_from_slice(&indent);
        new_content.extend_from_slice(b" */");
        tokens.insert_at(start_index, vec![Token::new(T_DOC_COMMENT, new_content)]);
    }
}

fn fix_comment_single_line(tokens: &mut Tokens, index: usize) {
    let mut message = get_message(tokens[index].get_content());
    if !trim(&message[..message.len().min(1)]).is_empty() {
        message.insert(0, b' ');
    }
    if !trim(&message[message.len().saturating_sub(1)..]).is_empty() {
        message.push(b' ');
    }
    tokens.set(
        index,
        Token::new(T_DOC_COMMENT, [&b"/**"[..], &message, b"*/"].concat()),
    );
}

fn get_message(content: &[u8]) -> Vec<u8> {
    if let Some(rest) = content.strip_prefix(b"#") {
        return rest.to_vec();
    }
    if let Some(rest) = content.strip_prefix(b"//") {
        return rest.to_vec();
    }
    rtrim_chars(ltrim_chars(content, b"/*"), b"*/").to_vec()
}
