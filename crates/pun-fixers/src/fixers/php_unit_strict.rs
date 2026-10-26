//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitStrictFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate};
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

const ASSERTION_MAP: &[(&str, &str)] = &[
    ("assertAttributeEquals", "assertAttributeSame"),
    ("assertAttributeNotEquals", "assertAttributeNotSame"),
    ("assertEquals", "assertSame"),
    ("assertNotEquals", "assertNotSame"),
];

struct PhpUnitStrictFixer {
    assertions: Vec<String>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitStrictFixer {
        assertions: ASSERTION_MAP
            .iter()
            .map(|(from, _)| from.to_string())
            .collect(),
    })
}

impl Fixer for PhpUnitStrictFixer {
    fn name(&self) -> &'static str {
        "php_unit_strict"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["assertions"])?;
        let all: Vec<&str> = ASSERTION_MAP.iter().map(|(from, _)| *from).collect();
        self.assertions = c.strings("assertions", &all, &all)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            for method_before in &self.assertions {
                let &(_, method_after) = ASSERTION_MAP
                    .iter()
                    .find(|(from, _)| from == method_before)
                    .unwrap();
                let mut index = start_index;
                while index < end_index {
                    // Upstream's search is not bounded by the class end.
                    let Some(method_index) = tokens.get_next_token_of_kind(
                        index,
                        &[(T_STRING, method_before.as_str()).into()],
                        true,
                    ) else {
                        break;
                    };
                    if FunctionsAnalyzer::new().is_the_same_class_call(tokens, method_index) {
                        let open = tokens.get_next_meaningful_token(method_index).unwrap();
                        let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
                        if matches!(
                            ArgumentsAnalyzer.count_arguments(tokens, open, close),
                            2 | 3
                        ) {
                            tokens.set(
                                method_index,
                                Token::new(T_STRING, method_after.as_bytes().to_vec()),
                            );
                        }
                    }
                    // A call that is not the same class's makes upstream search again from each next index, which
                    // finds the same call until it passes it.
                    index = method_index + 1;
                }
            }
        });
    }
}
