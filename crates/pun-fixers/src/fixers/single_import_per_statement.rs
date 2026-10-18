//! Port of `PhpCsFixer\Fixer\Import\SingleImportPerStatementFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, TokensAnalyzer, php_compat};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct SingleImportPerStatementFixer {
    group_to_single_imports: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(SingleImportPerStatementFixer::default())
}

impl Fixer for SingleImportPerStatementFixer {
    fn name(&self) -> &'static str {
        "single_import_per_statement"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_USE)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["group_to_single_imports"])?;
        self.group_to_single_imports = o.bool("group_to_single_imports", true)?;
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let uses: Vec<usize> = TokensAnalyzer::new(tokens)
            .get_import_use_indexes(false)
            .into_values()
            .flatten()
            .collect();
        for &index in uses.iter().rev() {
            let end_index = tokens
                .get_next_token_of_kind(index, &[";".into(), T_CLOSE_TAG.into()], true)
                .unwrap();
            let group_close = tokens.get_prev_meaningful_token(end_index).unwrap();
            if tokens[group_close].is_kind(CT_GROUP_IMPORT_BRACE_CLOSE) {
                if self.group_to_single_imports {
                    self.fix_group_use(tokens, index, end_index);
                }
            } else {
                self.fix_multiple_use(tokens, index, end_index);
            }
        }
    }
}

fn get_group_declaration(tokens: &Tokens, index: usize) -> (Vec<u8>, usize, usize, Vec<u8>) {
    let mut group_prefix = Vec::new();
    let mut comment = Vec::new();
    let mut i = index + 1;
    let group_open_index = loop {
        let t = &tokens[i];
        if t.is_kind(CT_GROUP_IMPORT_BRACE_OPEN) {
            break i;
        }
        if t.is_comment() {
            comment.extend_from_slice(&t.content);
            if !tokens[i - 1].is_whitespace(None) && !tokens[i + 1].is_whitespace(None) {
                group_prefix.push(b' ');
            }
        } else if t.is_whitespace(None) {
            group_prefix.push(b' ');
        } else {
            group_prefix.extend_from_slice(&t.content);
        }
        i += 1;
    };
    (
        php_compat::rtrim(&group_prefix).to_vec(),
        group_open_index,
        tokens.find_block_end(Tokens::BLOCK_TYPE_GROUP_IMPORT_BRACE, group_open_index),
        comment,
    )
}

fn get_group_statements(
    tokens: &Tokens,
    group_prefix: &[u8],
    group_open_index: usize,
    group_close_index: usize,
    comment: &[u8],
) -> Vec<Vec<u8>> {
    let mut statements: Vec<Vec<u8>> = Vec::new();
    let mut statement = group_prefix.to_vec();
    let mut i = group_open_index + 1;
    while i <= group_close_index {
        let token = &tokens[i];
        if token.equals(",", true)
            && tokens[tokens.get_next_meaningful_token(i).unwrap()]
                .is_kind(CT_GROUP_IMPORT_BRACE_CLOSE)
        {
            i += 1;
            continue;
        }
        if token.equals_any(&[",".into(), CT_GROUP_IMPORT_BRACE_CLOSE.into()], true) {
            statements.push([&b"use"[..], &statement, b";"].concat());
            statement = group_prefix.to_vec();
            i += 1;
            continue;
        }
        if token.is_whitespace(None) {
            let j = tokens.get_next_meaningful_token(i).unwrap();
            if tokens[j].is_kind(T_AS) {
                statement.extend_from_slice(b" as ");
                i += 2;
            } else if tokens[j].is_kind(CT_FUNCTION_IMPORT) {
                statement = [&b" function"[..], &statement].concat();
                i += 2;
            } else if tokens[j].is_kind(CT_CONST_IMPORT) {
                statement = [&b" const"[..], &statement].concat();
                i += 2;
            }
            if token.is_whitespace(Some(b" \t")) || !tokens[i - 1].content.starts_with(b"//") {
                i += 1;
                continue;
            }
        }
        statement.extend_from_slice(&token.content);
        i += 1;
    }
    if !comment.is_empty() {
        statements[0].push(b' ');
        statements[0].extend_from_slice(comment);
    }
    statements
}

impl SingleImportPerStatementFixer {
    fn fix_group_use(&self, tokens: &mut Tokens, index: usize, end_index: usize) {
        let (group_prefix, group_open_index, group_close_index, comment) =
            get_group_declaration(tokens, index);
        let statements = get_group_statements(
            tokens,
            &group_prefix,
            group_open_index,
            group_close_index,
            &comment,
        );

        tokens.clear_range(index, group_close_index);
        if tokens[end_index].equals(";", true) {
            tokens.clear_at(end_index);
        }

        let ending = self.whitespaces_config.get_line_ending().as_bytes();
        let mut import_tokens =
            Tokens::from_code(&[&b"<?php "[..], &statements.join(ending)].concat());
        import_tokens.clear_at(0);
        import_tokens.clear_empty_tokens();
        tokens.insert_at(index, import_tokens.iter().cloned().collect());
    }

    fn fix_multiple_use(&self, tokens: &mut Tokens, index: usize, end_index: usize) {
        let next_token_index = tokens.get_next_meaningful_token(index).unwrap();
        let leading_tokens = if tokens[next_token_index].is_kind(CT_FUNCTION_IMPORT) {
            vec![
                Token::new(CT_FUNCTION_IMPORT, "function"),
                Token::new(T_WHITESPACE, " "),
            ]
        } else if tokens[next_token_index].is_kind(CT_CONST_IMPORT) {
            vec![
                Token::new(CT_CONST_IMPORT, "const"),
                Token::new(T_WHITESPACE, " "),
            ]
        } else {
            vec![]
        };
        let ending = self.whitespaces_config.get_line_ending().as_bytes();

        let mut i = end_index - 1;
        while i > index {
            if !tokens[i].equals(",", true) {
                i -= 1;
                continue;
            }
            tokens.set(i, Token::char(";"));
            i = tokens.get_next_meaningful_token(i).unwrap();
            tokens.insert_at(i, vec![Token::new(T_USE, "use")]);
            tokens.insert_at(i + 1, vec![Token::new(T_WHITESPACE, " ")]);
            for (offset, leading_token) in leading_tokens.iter().enumerate() {
                tokens.insert_at(i + 2 + offset, vec![leading_token.clone()]);
            }
            let indent = WhitespacesAnalyzer::detect_indent(tokens, index);
            if tokens[i - 1].is_whitespace(None) {
                tokens.set(i - 1, Token::new(T_WHITESPACE, [ending, &indent].concat()));
            } else if !tokens[i - 1].content.contains(&b'\n') {
                tokens.insert_at(
                    i,
                    vec![Token::new(T_WHITESPACE, [ending, &indent].concat())],
                );
            }
            i -= 1;
        }
    }
}
