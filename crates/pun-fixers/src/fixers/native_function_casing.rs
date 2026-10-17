//! Port of `PhpCsFixer\Fixer\Casing\NativeFunctionCasingFixer`. The native function list is the
//! oracle PHP's `get_defined_functions()['internal']` (all lowercase on PHP 8.5).
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use crate::analyzers::php_functions::is_internal_function;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NativeFunctionCasingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NativeFunctionCasingFixer)
}

impl Fixer for NativeFunctionCasingFixer {
    fn name(&self) -> &'static str {
        "native_function_casing"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        for index in 0..tokens.count() {
            if !functions_analyzer.is_global_function_call(tokens, index) {
                continue;
            }
            if !is_internal_function(&tokens[index].content) {
                continue;
            }
            let lower = tokens[index].content.to_ascii_lowercase();
            tokens.set(index, Token::new(T_STRING, lower));
        }
    }
}
