//! Port of `PhpCsFixer\Fixer\FunctionNotation\PhpdocToReturnTypeFixer`.
#[path = "../analyzers/abstract_phpdoc_to_type_declaration_fixer.rs"]
mod abstract_phpdoc_to_type;

use abstract_phpdoc_to_type::{
    Common, Options, create_type_declaration_tokens, doc_comment, find_function_doc_comment,
    get_types_to_exclude, is_valid_syntax, tokens_from_raw_type,
};
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

const EXCLUDE_FUNC_NAMES: &[Proto<'static>] = &[
    Proto::KindContent(T_STRING, b"__construct"),
    Proto::KindContent(T_STRING, b"__destruct"),
    Proto::KindContent(T_STRING, b"__clone"),
];

struct PhpdocToReturnTypeFixer {
    options: Options,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocToReturnTypeFixer {
        options: Options::default(),
    })
}

fn is_skipped_type(type_: &[u8]) -> bool {
    matches!(type_, b"resource" | b"null")
}

/// `TYPE_CHECK_TEMPLATE`.
fn type_check_code(type_: &[u8]) -> String {
    format!(
        "<?php function f(): {} {{}}",
        String::from_utf8_lossy(type_)
    )
}

/// `hasReturnTypeHint`: `index` is the end of the function definition line, at `{`, `;` or `=>`.
fn has_return_type_hint(tokens: &Tokens, index: usize) -> bool {
    let end_func_index = tokens
        .get_prev_token_of_kind(index, &[Proto::Char(b")")], true)
        .unwrap();
    let next_index = tokens.get_next_meaningful_token(end_func_index).unwrap();
    tokens[next_index].is_kind(CT_TYPE_COLON)
}

impl Fixer for PhpdocToReturnTypeFixer {
    fn name(&self) -> &'static str {
        "phpdoc_to_return_type"
    }
    fn priority(&self) -> i32 {
        13
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_FUNCTION, T_FN])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        self.options = Options::configure(self.name(), options)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let common = Common {
            options: &self.options,
            is_skipped_type,
        };
        let mut tokens_to_insert: BTreeMap<usize, Vec<Token>> = BTreeMap::new();
        let mut types_to_exclude: Vec<Vec<u8>> = Vec::new();
        for index in 0..tokens.count() {
            if tokens[index].is_kind(T_DOC_COMMENT) {
                types_to_exclude.extend(get_types_to_exclude(tokens[index].get_content()));
                continue;
            }
            if !tokens[index].is_given_kind(&[T_FUNCTION, T_FN]) {
                continue;
            }
            let func_name = tokens.get_next_meaningful_token(index).unwrap();
            if tokens[func_name].equals_any(EXCLUDE_FUNC_NAMES, false) {
                continue;
            }
            let Some(doc_comment_index) = find_function_doc_comment(tokens, index) else {
                continue;
            };
            let (doc, scope) = doc_comment(tokens, doc_comment_index);
            let return_type_annotations = doc.get_annotations_of_type(&["return"]);
            let [return_type_annotation] = return_type_annotations[..] else {
                continue;
            };
            let Some(type_info) = common.type_info(return_type_annotation, &scope, true) else {
                continue;
            };
            let return_type = type_info.common_type;
            if types_to_exclude.contains(&return_type) {
                continue;
            }
            let params_start_index = tokens
                .get_next_token_of_kind(index, &[Proto::Char(b"(")], true)
                .unwrap();
            let params_end_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, params_start_index);
            let body_start_index = tokens
                .get_next_token_of_kind(
                    params_end_index,
                    &[
                        Proto::Char(b"{"),
                        Proto::Char(b";"),
                        Proto::Kind(T_DOUBLE_ARROW),
                    ],
                    true,
                )
                .unwrap();
            if has_return_type_hint(tokens, body_start_index) {
                continue;
            }
            let code = type_check_code(&return_type);
            if !is_valid_syntax(&code) {
                continue;
            }
            let mut new_tokens = vec![
                Token::new(CT_TYPE_COLON, ":"),
                Token::new(T_WHITESPACE, " "),
            ];
            new_tokens.extend(create_type_declaration_tokens(
                tokens_from_raw_type(&code, 7, 3),
                &return_type,
                type_info.is_nullable,
            ));
            tokens_to_insert.insert(params_end_index + 1, new_tokens);
        }
        tokens.insert_slices(tokens_to_insert);
    }
}
