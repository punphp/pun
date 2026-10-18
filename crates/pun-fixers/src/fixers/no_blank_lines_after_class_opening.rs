//! Port of `PhpCsFixer\Fixer\ClassNotation\NoBlankLinesAfterClassOpeningFixer`.
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, php_compat::substr_count, token::CLASSY_KINDS};
use std::path::Path;

#[derive(Default)]
struct NoBlankLinesAfterClassOpeningFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoBlankLinesAfterClassOpeningFixer::default())
}

impl NoBlankLinesAfterClassOpeningFixer {
    fn fix_whitespace(&self, tokens: &mut Tokens, index: usize) {
        let content = tokens[index].get_content();
        if substr_count(content, b"\n") > 1 {
            // the final bit of the whitespace must be the next statement's indentation
            let last = content.iter().rposition(|&b| b == b'\n').unwrap();
            let new = [
                self.whitespaces_config.get_line_ending().as_bytes(),
                &content[last + 1..],
            ]
            .concat();
            tokens.set(index, Token::new(T_WHITESPACE, new));
        }
    }
}

impl Fixer for NoBlankLinesAfterClassOpeningFixer {
    fn name(&self) -> &'static str {
        "no_blank_lines_after_class_opening"
    }
    fn priority(&self) -> i32 {
        // Must run after OrderedClassElementsFixer, PhpUnitDataProviderMethodOrderFixer.
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CLASSY_KINDS)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_classy() {
                continue;
            }
            let start_brace_index = tokens
                .get_next_token_of_kind(index, &["{".into()], true)
                .unwrap();
            if !tokens[start_brace_index + 1].is_whitespace(None) {
                continue;
            }
            self.fix_whitespace(tokens, start_brace_index + 1);
        }
    }
}
