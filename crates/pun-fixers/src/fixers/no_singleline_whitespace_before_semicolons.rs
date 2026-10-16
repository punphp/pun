//! Port of `PhpCsFixer\Fixer\Semicolon\NoSinglelineWhitespaceBeforeSemicolonsFixer`.
use pun_format::Fixer;
use pun_tokens::Tokens;
use std::path::Path;

struct NoSinglelineWhitespaceBeforeSemicolonsFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoSinglelineWhitespaceBeforeSemicolonsFixer)
}

impl Fixer for NoSinglelineWhitespaceBeforeSemicolonsFixer {
    fn name(&self) -> &'static str {
        "no_singleline_whitespace_before_semicolons"
    }
    fn priority(&self) -> i32 {
        // Must run after CombineConsecutiveIssetsFixer, FunctionToConstantFixer, LongToShorthandOperatorFixer,
        // NoEmptyStatementFixer, NoUnneededImportAliasFixer, SimplifiedIfReturnFixer, SingleImportPerStatementFixer.
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(";")
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        while index < tokens.count() {
            if !tokens[index].equals(";", true) || !tokens[index - 1].is_whitespace(Some(b" \t")) {
                index += 1;
                continue;
            }
            if tokens[index - 2].equals(";", true) {
                // do not remove all whitespace before the semicolon because it is also whitespace after another semicolon
                tokens.ensure_whitespace_at_index(index - 1, 0, b" ");
            } else if !tokens[index - 2].is_comment() {
                tokens.clear_at(index - 1);
            }
            index += 1;
        }
    }
}
