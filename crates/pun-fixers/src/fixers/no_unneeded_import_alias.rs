//! Port of `PhpCsFixer\Fixer\Import\NoUnneededImportAliasFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind, Tokens};
use std::path::Path;

struct NoUnneededImportAliasFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUnneededImportAliasFixer)
}

impl Fixer for NoUnneededImportAliasFixer {
    fn name(&self) -> &'static str {
        "no_unneeded_import_alias"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[Kind::T_USE, Kind::T_AS])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_kind(Kind::T_AS) {
                continue;
            }
            let alias_index = tokens.get_next_meaningful_token(index).unwrap();
            if !tokens[alias_index].is_kind(Kind::T_STRING) {
                continue;
            }
            let mut import_index = tokens.get_prev_meaningful_token(index).unwrap();
            if !tokens[import_index].is_kind(Kind::T_STRING) || tokens[import_index].content != tokens[alias_index].content {
                continue;
            }
            loop {
                import_index = tokens.get_prev_meaningful_token(import_index).unwrap();
                let t = &tokens[import_index];
                if !(t.is_given_kind(&[Kind::T_NS_SEPARATOR, Kind::T_STRING, Kind::T_AS]) || t.equals(",", true)) {
                    break;
                }
            }
            if tokens[import_index].is_given_kind(&[Kind::CT_FUNCTION_IMPORT, Kind::CT_CONST_IMPORT]) {
                import_index = tokens.get_prev_meaningful_token(import_index).unwrap();
            }
            if !tokens[import_index].is_given_kind(&[Kind::T_USE, Kind::CT_GROUP_IMPORT_BRACE_OPEN]) {
                continue;
            }
            tokens.clear_token_and_merge_surrounding_whitespace(alias_index);
            tokens.clear_token_and_merge_surrounding_whitespace(index);
        }
    }
}
