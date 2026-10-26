//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitTestClassRequiresCoversFixer`.
use crate::analyzers::abstract_php_unit_fixer::{
    apply_fix, ensure_is_doc_block_with_annotation, is_candidate,
};
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::{Tokens, TokensAnalyzer};
use std::path::Path;

#[derive(Default)]
struct PhpUnitTestClassRequiresCoversFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpUnitTestClassRequiresCoversFixer::default())
}

impl Fixer for PhpUnitTestClassRequiresCoversFixer {
    fn name(&self) -> &'static str {
        "php_unit_test_class_requires_covers"
    }
    fn priority(&self) -> i32 {
        9
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
            let analyzer = TokensAnalyzer::new(tokens);
            if analyzer.is_anonymous_class(class_index) {
                return;
            }
            // don't add `@covers` annotation for abstract base classes
            if analyzer
                .get_classy_modifiers(class_index)
                .abstract_
                .is_some()
            {
                return;
            }
            ensure_is_doc_block_with_annotation(
                tokens,
                class_index,
                "coversNothing",
                &["covers", "coversDefaultClass", "coversNothing"],
                &[
                    "phpunit\\framework\\attributes\\coversclass",
                    "phpunit\\framework\\attributes\\coversnothing",
                    "phpunit\\framework\\attributes\\coversmethod",
                    "phpunit\\framework\\attributes\\coversfunction",
                    "phpunit\\framework\\attributes\\coverstrait",
                ],
                &line_end,
            );
        });
    }
}
