//! Port of `PhpCsFixer\Fixer\NamespaceNotation\BlankLineAfterNamespaceFixer`.
use crate::analyzers::preg::Preg;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

#[derive(Default)]
struct BlankLineAfterNamespaceFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(BlankLineAfterNamespaceFixer::default())
}

impl BlankLineAfterNamespaceFixer {
    fn get_index_to_ensure_blank_line_after(tokens: &Tokens, index: usize) -> usize {
        let mut index_to_ensure_blank_line = index;
        let mut next_index = tokens.get_non_empty_sibling(index_to_ensure_blank_line, 1);
        while let Some(n) = next_index {
            let token = &tokens[n];
            if token.is_whitespace(None) {
                if Preg::r#match("/\\R/", token.get_content()) {
                    break;
                }
                let next_next_index = tokens.get_non_empty_sibling(n, 1).unwrap();
                if !tokens[next_next_index].is_comment() {
                    break;
                }
            }
            if !token.is_whitespace(None) && !token.is_comment() {
                break;
            }
            index_to_ensure_blank_line = n;
            next_index = tokens.get_non_empty_sibling(index_to_ensure_blank_line, 1);
        }
        index_to_ensure_blank_line
    }

    fn get_token_to_insert(&self, current_content: &[u8], is_last_index: bool) -> Token {
        let ending = self.whitespaces_config.get_line_ending();
        let empty_lines = if is_last_index { ending.to_string() } else { format!("{ending}{ending}") };
        let indent = Preg::match_groups("/^.*\\R( *)$/s", current_content, 0).map(|m| m.str(1).to_vec()).unwrap_or_default();
        Token::new(T_WHITESPACE, [empty_lines.as_bytes(), &indent].concat())
    }
}

impl Fixer for BlankLineAfterNamespaceFixer {
    fn name(&self) -> &'static str {
        "blank_line_after_namespace"
    }
    fn priority(&self) -> i32 {
        // Must run after NoUnusedImportsFixer.
        -20
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_NAMESPACE)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let last_index = tokens.count() - 1;
        for index in (0..=last_index).rev() {
            if !tokens[index].is_kind(T_NAMESPACE) {
                continue;
            }
            let semicolon_index = tokens.get_next_token_of_kind(index, &[";".into(), "{".into(), T_CLOSE_TAG.into()], true).unwrap();
            if !tokens[semicolon_index].equals(";", true) {
                continue;
            }
            let index_to_ensure_blank_line_after = Self::get_index_to_ensure_blank_line_after(tokens, semicolon_index);
            let index_to_ensure_blank_line = tokens.get_non_empty_sibling(index_to_ensure_blank_line_after, 1);
            match index_to_ensure_blank_line {
                Some(i) if tokens[i].is_whitespace(None) => {
                    let t = self.get_token_to_insert(tokens[i].get_content(), i == last_index);
                    tokens.set(i, t);
                }
                _ => {
                    let t = self.get_token_to_insert(b"", index_to_ensure_blank_line_after == last_index);
                    tokens.insert_at(index_to_ensure_blank_line_after + 1, vec![t]);
                }
            }
        }
    }
}
