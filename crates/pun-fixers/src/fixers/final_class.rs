//! Port of `PhpCsFixer\Fixer\ClassNotation\FinalClassFixer` (AbstractProxyFixer over a
//! FinalInternalClassFixer configured with `include: []`, `consider_absent_docblock_as_internal_class: true`).
use crate::analyzers::final_internal_class::FinalInternalClassFixer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use std::path::Path;

struct FinalClassFixer {
    proxy: FinalInternalClassFixer,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    let mut proxy = FinalInternalClassFixer::default();
    proxy
        .configure_options(
            &serde_json::json!({"include": [], "consider_absent_docblock_as_internal_class": true}),
        )
        .unwrap();
    Box::new(FinalClassFixer { proxy })
}

impl Fixer for FinalClassFixer {
    fn name(&self) -> &'static str {
        "final_class"
    }
    fn priority(&self) -> i32 {
        67
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_CLASS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        self.proxy.apply_fix(tokens);
    }
}
