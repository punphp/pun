//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitDataProviderStaticFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate};
use crate::analyzers::data_provider_analyzer::get_data_providers;
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Default)]
struct PhpUnitDataProviderStaticFixer {
    force: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitDataProviderStaticFixer::default())
}

impl Fixer for PhpUnitDataProviderStaticFixer {
    fn name(&self) -> &'static str {
        "php_unit_data_provider_static"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["force"])?;
        self.force = c.bool("force", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            let mut inserts = BTreeMap::new();
            for provider in get_data_providers(tokens, start_index, end_index) {
                if let Some(method_start) =
                    tokens.get_next_token_of_kind(provider.name_index, &["{".into()], true)
                {
                    let method_end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, method_start);
                    if !self.force
                        && tokens
                            .find_sequence(
                                &[(T_VARIABLE, "$this").into()],
                                method_start,
                                Some(method_end),
                                |_| true,
                            )
                            .is_some()
                    {
                        continue;
                    }
                }
                let function_index = tokens
                    .get_prev_token_of_kind(provider.name_index, &[T_FUNCTION.into()], true)
                    .unwrap();
                if TokensAnalyzer::new(tokens)
                    .get_method_attributes(function_index)
                    .static_
                {
                    continue;
                }
                inserts.insert(
                    function_index,
                    vec![
                        Token::new(T_STATIC, b"static".to_vec()),
                        Token::new(T_WHITESPACE, b" ".to_vec()),
                    ],
                );
            }
            tokens.insert_slices(inserts);
        });
    }
}
