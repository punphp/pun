//! Port of `PhpCsFixer\Fixer\Whitespace\BlankLineBetweenImportGroupsFixer`.
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Tokens, TokensAnalyzer};
use std::path::Path;

#[derive(Default)]
struct BlankLineBetweenImportGroupsFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(BlankLineBetweenImportGroupsFixer::default())
}

#[derive(PartialEq, Clone, Copy)]
enum ImportType {
    Class,
    Const,
    Function,
}

impl BlankLineBetweenImportGroupsFixer {
    fn walk_over_uses(&self, tokens: &mut Tokens, uses: &[usize]) {
        let uses_count = uses.len();
        if uses_count < 2 {
            return;
        }
        let mut previous_type = None;
        for i in (0..uses_count).rev() {
            let index = uses[i];
            let start_index = tokens.get_next_meaningful_token(index + 1).unwrap();
            let end_index = tokens
                .get_next_token_of_kind(start_index, &[";".into(), T_CLOSE_TAG.into()], true)
                .unwrap();
            let ty = if tokens[start_index].is_kind(CT_CONST_IMPORT) {
                ImportType::Const
            } else if tokens[start_index].is_kind(CT_FUNCTION_IMPORT) {
                ImportType::Function
            } else {
                ImportType::Class
            };
            if previous_type.is_some_and(|p| p != ty) {
                self.ensure_line(tokens, end_index + 1);
            }
            previous_type = Some(ty);
        }
    }

    fn ensure_line(&self, tokens: &mut Tokens, index: usize) {
        // ponytail: upstream caches the line ending in a method-static (first config wins for the
        // process); Pint always uses "\n", so reading the config each time is equivalent.
        let line_ending = self.whitespaces_config.get_line_ending().repeat(2);
        let index = Self::get_insert_index(tokens, index);
        let indent = WhitespacesAnalyzer::detect_indent(tokens, index);
        tokens.ensure_whitespace_at_index(index, 1, &[line_ending.as_bytes(), &indent].concat());
    }

    fn get_insert_index(tokens: &Tokens, mut index: usize) -> usize {
        let tokens_count = tokens.count();
        while index < tokens_count - 1 {
            if !tokens[index].is_whitespace(None) && !tokens[index].is_comment() {
                return index - 1;
            }
            if tokens[index].get_content().contains(&b'\n') {
                break;
            }
            index += 1;
        }
        index
    }
}

impl Fixer for BlankLineBetweenImportGroupsFixer {
    fn name(&self) -> &'static str {
        "blank_line_between_import_groups"
    }
    fn priority(&self) -> i32 {
        // Must run after OrderedImportsFixer.
        -40
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_USE)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let namespaces_imports = TokensAnalyzer::new(tokens).get_import_use_indexes(true);
        for uses in namespaces_imports.values().rev() {
            self.walk_over_uses(tokens, uses);
        }
    }
}
