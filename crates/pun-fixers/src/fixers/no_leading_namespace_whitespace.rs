//! Port of `PhpCsFixer\Fixer\NamespaceNotation\NoLeadingNamespaceWhitespaceFixer`.
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, php_compat::trim};
use std::path::Path;

#[derive(Default)]
struct NoLeadingNamespaceWhitespaceFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoLeadingNamespaceWhitespaceFixer::default())
}

fn ends_with_whitespace(s: &[u8]) -> bool {
    // upstream's `'' === $str` guard never fires: freshly tokenized tokens are non-empty
    s.last().is_some_and(|&b| trim(&[b]).is_empty())
}

impl Fixer for NoLeadingNamespaceWhitespaceFixer {
    fn name(&self) -> &'static str {
        "no_leading_namespace_whitespace"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_NAMESPACE)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_kind(T_NAMESPACE) {
                continue;
            }
            let before_namespace_index = index - 1;
            let before_namespace = tokens[before_namespace_index].clone();
            if !before_namespace.is_whitespace(None) {
                if !ends_with_whitespace(before_namespace.get_content()) {
                    tokens.insert_at(
                        index,
                        vec![Token::new(
                            T_WHITESPACE,
                            self.whitespaces_config.get_line_ending(),
                        )],
                    );
                }
                continue;
            }
            match before_namespace
                .get_content()
                .iter()
                .rposition(|&b| b == b'\n')
            {
                None => {
                    if ends_with_whitespace(tokens[index - 2].get_content()) {
                        tokens.clear_at(before_namespace_index);
                    } else {
                        tokens.set(before_namespace_index, Token::new(T_WHITESPACE, " "));
                    }
                }
                Some(last_newline) => {
                    tokens.set(
                        before_namespace_index,
                        Token::new(
                            T_WHITESPACE,
                            &before_namespace.get_content()[..=last_newline],
                        ),
                    );
                }
            }
        }
    }
}
