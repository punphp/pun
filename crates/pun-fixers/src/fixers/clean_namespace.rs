//! Port of `PhpCsFixer\Fixer\NamespaceNotation\CleanNamespaceFixer`. Only a candidate on PHP < 8.0
//! (`\PHP_VERSION_ID < 8_00_00`); the oracle runs PHP 8.5, so it never applies.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

struct CleanNamespaceFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(CleanNamespaceFixer)
}

/// PHP_VERSION_ID of the oracle (PHP 8.5).
const PHP_VERSION_ID: u32 = 8_05_00;

impl Fixer for CleanNamespaceFixer {
    fn name(&self) -> &'static str {
        "clean_namespace"
    }
    fn priority(&self) -> i32 {
        // Must run before PhpUnitDataProviderReturnTypeFixer.
        10
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        PHP_VERSION_ID < 8_00_00 && tokens.is_token_kind_found(T_NS_SEPARATOR)
    }
    /// Upstream's applyFix (strip whitespace/comments inside `Foo \ Bar` names) is unreachable here:
    /// `is_candidate` is always false on PHP 8.5, and AbstractFixer::fix gates applyFix on it.
    fn fix(&self, _file: &Path, _tokens: &mut Tokens) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    // PHP 8.5: `(new CleanNamespaceFixer)->isCandidate(Tokens::fromCode(...))` is false even with T_NS_SEPARATOR.
    #[test]
    fn never_a_candidate_on_php_8() {
        let tokens = Tokens::from_code(b"<?php\nuse Foo\\{A, B};\n");
        assert!(tokens.is_token_kind_found(T_NS_SEPARATOR));
        assert!(!fixer().is_candidate(&tokens));
        // ...so the real fixer never touches it; neither may a direct fix() call.
        let mut tokens = tokens;
        fixer().fix(Path::new("t.php"), &mut tokens);
        assert!(!tokens.is_changed());
    }
}
