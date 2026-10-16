//! Port of `PhpCsFixer\Fixer\ControlStructure\ControlStructureContinuationPositionFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind, Kind::*, Tokens};
use serde_json::Value;
use std::path::Path;

const CONTROL_CONTINUATION_TOKENS: &[Kind] = &[T_CATCH, T_ELSE, T_ELSEIF, T_FINALLY, T_WHILE];

#[derive(Default)]
struct ControlStructureContinuationPositionFixer {
    next_line: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ControlStructureContinuationPositionFixer::default())
}

impl Fixer for ControlStructureContinuationPositionFixer {
    fn name(&self) -> &'static str {
        "control_structure_continuation_position"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CONTROL_CONTINUATION_TOKENS)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["position"])?;
        self.next_line = o.string("position", "same_line", &["next_line", "same_line"])? == "next_line";
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() - 1;
        while 0 < index {
            let token = &tokens[index];
            if token.is_given_kind(CONTROL_CONTINUATION_TOKENS) {
                let is_while = token.is_kind(T_WHILE);
                let prev_index = tokens.get_prev_non_whitespace(index, None).unwrap();
                if tokens[prev_index].equals("}", true)
                    && (!is_while
                        || tokens[tokens
                            .get_prev_meaningful_token(tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, prev_index))
                            .unwrap()]
                        .is_kind(T_DO))
                {
                    let ws = if self.next_line {
                        [self.whitespaces_config.get_line_ending().as_bytes(), &WhitespacesAnalyzer::detect_indent(tokens, index)]
                            .concat()
                    } else {
                        b" ".to_vec()
                    };
                    tokens.ensure_whitespace_at_index(index - 1, 1, &ws);
                }
            }
            index -= 1;
        }
    }
}
