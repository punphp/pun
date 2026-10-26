//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitSizeClassFixer`.
use crate::analyzers::abstract_php_unit_fixer::{
    apply_fix, ensure_is_doc_block_with_annotation, is_candidate,
};
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use serde_json::Value;
use std::path::Path;

const SIZES: [&str; 3] = ["small", "medium", "large"];

struct PhpUnitSizeClassFixer {
    group: String,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitSizeClassFixer {
        group: "small".to_string(),
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

impl Fixer for PhpUnitSizeClassFixer {
    fn name(&self) -> &'static str {
        "php_unit_size_class"
    }
    fn priority(&self) -> i32 {
        9
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["group"])?;
        self.group = c.string("group", "small", &SIZES)?;
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
            let type_index = tokens.get_prev_meaningful_token(class_index).unwrap();
            if tokens[type_index].is_kind(T_ABSTRACT) {
                return;
            }
            ensure_is_doc_block_with_annotation(
                tokens,
                class_index,
                &self.group,
                &SIZES,
                &[
                    "phpunit\\framework\\attributes\\small",
                    "phpunit\\framework\\attributes\\medium",
                    "phpunit\\framework\\attributes\\large",
                ],
                &line_end,
            );
        });
    }
}
