//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitMockFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate, target_fulfills};
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct PhpUnitMockFixer {
    fix_create_partial_mock: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitMockFixer::default())
}

impl Fixer for PhpUnitMockFixer {
    fn name(&self) -> &'static str {
        "php_unit_mock"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["target"])?;
        let target = c.string("target", "newest", &["5.4", "5.5", "newest"])?;
        self.fix_create_partial_mock = target_fulfills(&target, "5.5");
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            let analyzer = ArgumentsAnalyzer;
            let mut index = start_index;
            while index < end_index {
                if !tokens[index].is_object_operator() {
                    index += 1;
                    continue;
                }
                index = tokens.get_next_meaningful_token(index).unwrap();
                if tokens[index].equals(
                    (T_STRING, "getMockWithoutInvokingTheOriginalConstructor"),
                    false,
                ) {
                    tokens.set(index, Token::new(T_STRING, b"createMock".to_vec()));
                } else if tokens[index].equals((T_STRING, "getMock"), false) {
                    let open = tokens.get_next_meaningful_token(index).unwrap();
                    let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
                    match analyzer.count_arguments(tokens, open, close) {
                        1 => tokens.set(index, Token::new(T_STRING, b"createMock".to_vec())),
                        2 if self.fix_create_partial_mock => {
                            tokens.set(index, Token::new(T_STRING, b"createPartialMock".to_vec()));
                        }
                        _ => {}
                    }
                }
                index += 1;
            }
        });
    }
}
