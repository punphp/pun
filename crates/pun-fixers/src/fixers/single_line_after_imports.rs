//! Port of `PhpCsFixer\Fixer\Import\SingleLineAfterImportsFixer`.
use crate::analyzers::utils::Utils;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, TokensAnalyzer, php_compat};
use std::path::Path;

#[derive(Default)]
struct SingleLineAfterImportsFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(SingleLineAfterImportsFixer::default())
}

impl Fixer for SingleLineAfterImportsFixer {
    fn name(&self) -> &'static str {
        "single_line_after_imports"
    }
    fn priority(&self) -> i32 {
        -11
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_USE)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let ending = self.whitespaces_config.get_line_ending().as_bytes();
        let uses: Vec<usize> = TokensAnalyzer::new(tokens)
            .get_import_use_indexes(false)
            .into_values()
            .flatten()
            .collect();

        let mut added = 0;
        for index in uses {
            let index = index + added;
            let mut indent: Vec<u8> = Vec::new();

            // if previous line ends with comment and current line starts with whitespace, use current indent
            if tokens[index - 1].is_whitespace(Some(b" \t")) && tokens[index - 2].is_kind(T_COMMENT)
            {
                indent = tokens[index - 1].content.clone();
            } else if tokens[index - 1].is_whitespace(None) {
                indent = Utils::calculate_trailing_whitespace_indent(&tokens[index - 1]);
            }

            let semicolon_index = tokens
                .get_next_token_of_kind(index, &[";".into(), T_CLOSE_TAG.into()], true)
                .unwrap();
            let mut insert_index = semicolon_index;

            if tokens[semicolon_index].is_kind(T_CLOSE_TAG) {
                if tokens[insert_index - 1].is_whitespace(None) {
                    insert_index -= 1;
                }
                tokens.insert_at(insert_index, vec![Token::char(";")]);
                added += 1;
            }

            if semicolon_index == tokens.count() - 1 {
                tokens.insert_at(
                    insert_index + 1,
                    vec![Token::new(T_WHITESPACE, [ending, ending, &indent].concat())],
                );
                added += 1;
            } else {
                let mut newline = ending.to_vec();
                // upstream decrements when tokens[semicolonIndex] is `?>`, but a `;` was inserted at or before
                // that index, shifting `?>` past it
                insert_index += 1;
                if tokens[insert_index].is_whitespace(Some(b" \t"))
                    && tokens[insert_index + 1].is_comment()
                {
                    insert_index += 1;
                }
                if tokens[insert_index].is_comment() {
                    insert_index += 1;
                }
                let after_semicolon = tokens.get_next_meaningful_token(semicolon_index);
                if !after_semicolon.is_some_and(|a| tokens[a].is_kind(T_USE)) {
                    newline.extend_from_slice(ending);
                }

                if tokens[insert_index].is_whitespace(None) {
                    let next_token = tokens[insert_index].clone();
                    if php_compat::substr_count(&next_token.content, b"\n") == 2 {
                        continue;
                    }
                    let new = Token::new(
                        T_WHITESPACE,
                        [
                            &newline[..],
                            &indent,
                            php_compat::ltrim_chars(&next_token.content, php_compat::TRIM_CHARS),
                        ]
                        .concat(),
                    );
                    let next_meaningful_after_use_index =
                        tokens.get_next_meaningful_token(insert_index);
                    if next_meaningful_after_use_index.is_some_and(|n| tokens[n].is_kind(T_USE)) {
                        if php_compat::substr_count(&next_token.content, b"\n") < 1 {
                            tokens.set(insert_index, new);
                        }
                    } else {
                        tokens.set(insert_index, new);
                    }
                } else {
                    tokens.insert_at(
                        insert_index,
                        vec![Token::new(T_WHITESPACE, [&newline[..], &indent].concat())],
                    );
                    added += 1;
                }
            }
        }
    }
}
