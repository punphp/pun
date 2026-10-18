//! Port of `PhpCsFixer\Fixer\ClassNotation\FinalInternalClassFixer`. The logic lives in
//! `analyzers::final_internal_class`, which `final_class` proxies to as well.
use crate::analyzers::final_internal_class::FinalInternalClassFixer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use serde_json::Value;
use std::path::Path;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(FinalInternalClassFixer::default())
}

impl Fixer for FinalInternalClassFixer {
    fn name(&self) -> &'static str {
        "final_internal_class"
    }
    fn priority(&self) -> i32 {
        67
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        self.configure_options(options)
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_CLASS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        self.apply_fix(tokens);
    }
}
