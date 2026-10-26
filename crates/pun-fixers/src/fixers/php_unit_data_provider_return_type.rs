//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitDataProviderReturnTypeFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate};
use crate::analyzers::data_provider_analyzer::get_data_providers;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PhpUnitDataProviderReturnTypeFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpUnitDataProviderReturnTypeFixer)
}

impl Fixer for PhpUnitDataProviderReturnTypeFixer {
    fn name(&self) -> &'static str {
        "php_unit_data_provider_return_type"
    }
    fn priority(&self) -> i32 {
        9
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            let functions = FunctionsAnalyzer::new();
            for provider in get_data_providers(tokens, start_index, end_index)
                .into_iter()
                .rev()
            {
                let Some(r#type) = functions.get_function_return_type(tokens, provider.name_index)
                else {
                    let arguments_start = tokens
                        .get_next_token_of_kind(provider.name_index, &["(".into()], true)
                        .unwrap();
                    let arguments_end =
                        tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, arguments_start);
                    tokens.insert_at(
                        arguments_end + 1,
                        vec![
                            Token::new(CT_TYPE_COLON, b":".to_vec()),
                            Token::new(T_WHITESPACE, b" ".to_vec()),
                            Token::new(T_STRING, b"iterable".to_vec()),
                        ],
                    );
                    continue;
                };
                if r#type.name == b"iterable" {
                    continue;
                }
                let type_start = tokens
                    .get_next_meaningful_token(r#type.get_start_index() - 1)
                    .unwrap();
                let type_end = r#type.get_end_index();
                if tokens.generate_partial_code(type_start, type_end) != r#type.name {
                    continue;
                }
                tokens.override_range(
                    type_start,
                    type_end,
                    vec![Token::new(T_STRING, b"iterable".to_vec())],
                );
            }
        });
    }
}
