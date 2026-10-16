//! Port of `PhpCsFixer\Fixer\Whitespace\CompactNullableTypeDeclarationFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

struct CompactNullableTypeDeclarationFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(CompactNullableTypeDeclarationFixer)
}

impl Fixer for CompactNullableTypeDeclarationFixer {
    fn name(&self) -> &'static str {
        "compact_nullable_type_declaration"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(CT_NULLABLE_TYPE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_kind(CT_NULLABLE_TYPE) {
                continue;
            }
            // remove whitespaces only if there are only whitespaces between '?' and the variable type
            if tokens[index + 1].is_whitespace(None)
                && tokens[index + 2].is_given_kind(&[CT_ARRAY_TYPEHINT, T_CALLABLE, T_NS_SEPARATOR, T_STATIC, T_STRING])
            {
                tokens.remove_trailing_whitespace(index, None);
            }
        }
    }
}
