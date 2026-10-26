//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitInternalClassFixer`.
use crate::analyzers::abstract_php_unit_fixer::{
    apply_fix, ensure_is_doc_block_with_annotation, is_candidate,
};
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::{Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

struct PhpUnitInternalClassFixer {
    types: Vec<String>,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitInternalClassFixer {
        types: vec!["normal".to_string(), "final".to_string()],
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

impl Fixer for PhpUnitInternalClassFixer {
    fn name(&self) -> &'static str {
        "php_unit_internal_class"
    }
    fn priority(&self) -> i32 {
        68
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["types"])?;
        self.types = c.strings(
            "types",
            &["normal", "final"],
            &["normal", "final", "abstract"],
        )?;
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let line_end = self
            .whitespaces_config
            .get_line_ending()
            .as_bytes()
            .to_vec();
        apply_fix(tokens, |tokens, start_index, _| {
            let class_index = tokens
                .get_prev_token_of_kind(start_index, &[T_CLASS.into()], true)
                .unwrap();
            let prev_index = tokens.get_prev_meaningful_token(class_index).unwrap();
            if tokens[prev_index].is_kind(T_NEW) {
                return;
            }
            let modifiers = TokensAnalyzer::new(tokens).get_classy_modifiers(class_index);
            let kind = if modifiers.final_.is_some() {
                "final"
            } else if modifiers.abstract_.is_some() {
                "abstract"
            } else {
                "normal"
            };
            if !self.types.iter().any(|t| t == kind) {
                return;
            }
            ensure_is_doc_block_with_annotation(
                tokens,
                class_index,
                "internal",
                &["internal"],
                &[],
                &line_end,
            );
        });
    }
}
