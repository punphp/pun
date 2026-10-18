//! Port of `PhpCsFixer\Fixer\Casing\MagicConstantCasingFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

const MAGIC_CONSTANTS: &[(Kind, &str)] = &[
    (Kind::T_LINE, "__LINE__"),
    (Kind::T_FILE, "__FILE__"),
    (Kind::T_DIR, "__DIR__"),
    (Kind::T_FUNC_C, "__FUNCTION__"),
    (Kind::T_CLASS_C, "__CLASS__"),
    (Kind::T_METHOD_C, "__METHOD__"),
    (Kind::T_NS_C, "__NAMESPACE__"),
    (Kind::CT_CLASS_CONSTANT, "class"),
    (Kind::T_TRAIT_C, "__TRAIT__"),
    (Kind::T_PROPERTY_C, "__PROPERTY__"),
];

struct MagicConstantCasingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(MagicConstantCasingFixer)
}

impl Fixer for MagicConstantCasingFixer {
    fn name(&self) -> &'static str {
        "magic_constant_casing"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        MAGIC_CONSTANTS
            .iter()
            .any(|&(k, _)| tokens.is_token_kind_found(k))
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            let Some(id) = tokens[index].id else { continue };
            let Some((_, c)) = MAGIC_CONSTANTS.iter().find(|(k, _)| *k == id) else {
                continue;
            };
            tokens.set(index, Token::new(id, c.as_bytes().to_vec()));
        }
    }
}
