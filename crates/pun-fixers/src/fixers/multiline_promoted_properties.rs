//! Port of `PhpCsFixer\Fixer\FunctionNotation\MultilinePromotedPropertiesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::{self, *};
use pun_tokens::tokens_analyzer::ClassyElementType;
use pun_tokens::{BlockType, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

const PROMOTION_KINDS: &[Kind] = &[
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
];

struct MultilinePromotedPropertiesFixer {
    keep_blank_lines: bool,
    minimum_number_of_parameters: i64,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(MultilinePromotedPropertiesFixer {
        keep_blank_lines: false,
        minimum_number_of_parameters: 1,
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

impl Fixer for MultilinePromotedPropertiesFixer {
    fn name(&self) -> &'static str {
        "multiline_promoted_properties"
    }
    fn priority(&self) -> i32 {
        // Must run before BracesPositionFixer, TrailingCommaInMultilineFixer.
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(PROMOTION_KINDS)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &["keep_blank_lines", "minimum_number_of_parameters"],
        )?;
        self.keep_blank_lines = o.bool("keep_blank_lines", false)?;
        o.check_types("minimum_number_of_parameters", &["int"])?;
        self.minimum_number_of_parameters = o
            .get("minimum_number_of_parameters")
            .map_or(1, |v| v.as_i64().unwrap());
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // Upstream iterates the elements found before any edit, so later indices go stale the same way.
        let elements = TokensAnalyzer::new(tokens).get_classy_elements();
        for (&index, element) in &elements {
            if element.kind != ClassyElementType::Method {
                continue;
            }
            let open = tokens
                .get_next_token_of_kind(index, &["(".into()], true)
                .unwrap();
            let close = tokens.find_block_end(BlockType::Parenthesis, open);
            if self.should_be_multiline(tokens, open, close) {
                self.make_multiline(tokens, open, close);
            } else if self.should_be_singleline(tokens, open, close) {
                make_singleline(tokens, open, close);
            }
        }
    }
}

impl MultilinePromotedPropertiesFixer {
    fn should_be_multiline(&self, tokens: &Tokens, open: usize, close: usize) -> bool {
        let range = open + 1..close;
        let parameters = range
            .clone()
            .filter(|&i| tokens[i].is_kind(T_VARIABLE))
            .count() as i64;
        range
            .into_iter()
            .any(|i| tokens[i].is_given_kind(PROMOTION_KINDS))
            && parameters >= self.minimum_number_of_parameters
    }

    fn should_be_singleline(&self, tokens: &Tokens, open: usize, close: usize) -> bool {
        let (mut promoted, mut parameters, mut multiline) = (false, 0, false);
        for index in open + 1..close {
            let token = &tokens[index];
            if token.is_kind(T_VARIABLE) {
                parameters += 1;
            }
            if token.is_given_kind(PROMOTION_KINDS) {
                promoted = true;
            }
            if token.is_given_kind(&[T_COMMENT, T_DOC_COMMENT, T_ATTRIBUTE]) {
                return false;
            }
            if token.is_whitespace(None) && token.get_content().contains(&b'\n') {
                multiline = true;
            }
        }
        promoted && multiline && parameters < self.minimum_number_of_parameters
    }

    fn make_multiline(&self, tokens: &mut Tokens, open: usize, close: usize) {
        let indent = WhitespacesAnalyzer::detect_indent(tokens, open);
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        tokens.ensure_whitespace_at_index(close - 1, 1, &[line_ending, &indent].concat());
        let mut index = tokens.get_prev_meaningful_token(close).unwrap();
        if !tokens[index].equals(",", true) {
            index = close;
        }
        while index > open {
            index = tokens.get_prev_meaningful_token(index).unwrap();
            if let Some((block_type, false)) = Tokens::detect_block_type(&tokens[index]) {
                index = tokens.find_block_start(block_type, index);
                continue;
            }
            if !tokens[index].equals_any(&["(".into(), ",".into()], true) {
                continue;
            }
            self.fix_parameter(tokens, index + 1, &indent);
        }
    }

    fn fix_parameter(&self, tokens: &mut Tokens, index: usize, indent: &[u8]) {
        if self.keep_blank_lines
            && tokens[index].is_whitespace(None)
            && tokens[index].get_content().contains(&b'\n')
        {
            return;
        }
        let whitespace = [
            self.whitespaces_config.get_line_ending().as_bytes(),
            indent,
            self.whitespaces_config.get_indent().as_bytes(),
        ]
        .concat();
        tokens.ensure_whitespace_at_index(index, 0, &whitespace);
    }
}

fn make_singleline(tokens: &mut Tokens, open: usize, close: usize) {
    for index in (open + 1..close).rev() {
        if !tokens[index].is_whitespace(None) || !tokens[index].get_content().contains(&b'\n') {
            continue;
        }
        if index == open + 1 || index == close - 1 {
            tokens.clear_at(index);
        } else {
            tokens.set(index, Token::new(T_WHITESPACE, " "));
        }
    }
}
