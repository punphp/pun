//! Port of `PhpCsFixer\Fixer\Import\NoLeadingImportSlashFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind, Tokens, TokensAnalyzer};
use std::path::Path;

struct NoLeadingImportSlashFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoLeadingImportSlashFixer)
}

impl Fixer for NoLeadingImportSlashFixer {
    fn name(&self) -> &'static str {
        "no_leading_import_slash"
    }
    fn priority(&self) -> i32 {
        -20
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_USE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let uses_indices = TokensAnalyzer::new(tokens)
            .get_import_use_indexes(false)
            .remove(&0)
            .unwrap_or_default();
        for idx in uses_indices {
            let mut next_token_idx = tokens.get_next_meaningful_token(idx).unwrap();
            let next_token = &tokens[next_token_idx];
            if next_token.is_kind(Kind::T_NS_SEPARATOR) {
                remove_leading_import_slash(tokens, next_token_idx);
            } else if next_token.is_given_kind(&[Kind::CT_FUNCTION_IMPORT, Kind::CT_CONST_IMPORT]) {
                next_token_idx = tokens.get_next_meaningful_token(next_token_idx).unwrap();
                if tokens[next_token_idx].is_kind(Kind::T_NS_SEPARATOR) {
                    remove_leading_import_slash(tokens, next_token_idx);
                }
            }
        }
    }
}

fn remove_leading_import_slash(tokens: &mut Tokens, index: usize) {
    // Upstream turns a `\` glued to the previous non-whitespace, non-comment token into a space, but PHP 8 lexes
    // `use\Foo` / `function\foo` as one name, so the separator is always preceded by whitespace or a comment.
    tokens.clear_at(index);
}
