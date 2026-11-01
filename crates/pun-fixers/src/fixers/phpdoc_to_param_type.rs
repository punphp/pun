//! Port of `PhpCsFixer\Fixer\FunctionNotation\PhpdocToParamTypeFixer`.
use crate::analyzers::abstract_phpdoc_to_type_declaration_fixer::{
    Common, Options, create_type_declaration_tokens, doc_comment, find_function_doc_comment,
    get_types_to_exclude, is_valid_syntax, tokens_from_raw_type,
};
use crate::analyzers::docblock::Annotation;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

const EXCLUDE_FUNC_NAMES: &[Proto<'static>] = &[
    Proto::KindContent(T_STRING, b"__clone"),
    Proto::KindContent(T_STRING, b"__destruct"),
];

struct PhpdocToParamTypeFixer {
    options: Options,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocToParamTypeFixer {
        options: Options::default(),
    })
}

fn is_skipped_type(type_: &[u8]) -> bool {
    matches!(type_, b"resource" | b"static" | b"void")
}

/// `TYPE_CHECK_TEMPLATE`.
fn type_check_code(type_: &[u8]) -> String {
    format!(
        "<?php function f({} $x) {{}}",
        String::from_utf8_lossy(type_)
    )
}

/// `findCorrectVariable`.
fn find_correct_variable(
    tokens: &Tokens,
    start_index: usize,
    param_type_annotation: &Annotation,
) -> Option<usize> {
    let end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start_index);
    let variable_name = param_type_annotation.get_variable_name();
    (start_index + 1..end_index).find(|&index| {
        tokens[index].is_kind(T_VARIABLE)
            && variable_name.as_deref() == Some(tokens[index].get_content())
    })
}

/// `hasParamTypeHint`.
fn has_param_type_hint(tokens: &Tokens, index: usize) -> bool {
    let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
    !tokens[prev_index].equals_any(&[Proto::Char(b","), Proto::Char(b"(")], true)
}

impl Fixer for PhpdocToParamTypeFixer {
    fn name(&self) -> &'static str {
        "phpdoc_to_param_type"
    }
    fn priority(&self) -> i32 {
        8
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
            for param_type_annotation in doc.get_annotations_of_type(&["param"]) {
                let Some(type_info) = common.type_info(param_type_annotation, &scope, false) else {
                    continue;
                };
                let param_type = type_info.common_type;
                if types_to_exclude.contains(&param_type) {
                    continue;
                }
                let start_index = tokens
                    .get_next_token_of_kind(index, &[Proto::Char(b"(")], true)
                    .unwrap();
                let Some(mut variable_index) =
                    find_correct_variable(tokens, start_index, param_type_annotation)
                else {
                    continue;
                };
                let by_ref_index = tokens.get_prev_meaningful_token(variable_index).unwrap();
                if tokens[by_ref_index].equals(Proto::Char(b"&"), true) {
                    variable_index = by_ref_index;
                }
                if has_param_type_hint(tokens, variable_index) {
                    continue;
                }
                let code = type_check_code(&param_type);
                if !is_valid_syntax(&code) {
                    continue;
                }
                let mut new_tokens = create_type_declaration_tokens(
                    tokens_from_raw_type(&code, 4, 6),
                    &param_type,
                    type_info.is_nullable,
                );
                new_tokens.push(Token::new(T_WHITESPACE, " "));
                tokens_to_insert.insert(variable_index, new_tokens);
            }
        }
        tokens.insert_slices(tokens_to_insert);
    }
}
