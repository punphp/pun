//! Port of `PhpCsFixer\Fixer\FunctionNotation\PhpdocToPropertyTypeFixer`.
#[path = "../analyzers/abstract_phpdoc_to_type_declaration_fixer.rs"]
mod abstract_phpdoc_to_type;

use crate::analyzers::docblock::Annotation;
use abstract_phpdoc_to_type::{
    Common, Options, Scope, TypeInfo, create_type_declaration_tokens, doc_comment,
    get_types_to_exclude, is_valid_syntax, tokens_from_raw_type,
};
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

struct PhpdocToPropertyTypeFixer {
    options: Options,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocToPropertyTypeFixer {
        options: Options::default(),
    })
}

fn is_skipped_type(type_: &[u8]) -> bool {
    matches!(type_, b"resource" | b"null")
}

/// `TYPE_CHECK_TEMPLATE`.
fn type_check_code(type_: &[u8]) -> String {
    format!(
        "<?php class A {{ private {} $b; }}",
        String::from_utf8_lossy(type_)
    )
}

/// `findNextUntypedPropertiesDeclaration`: the property names in order with their indices (a repeated name keeps its
/// first place and its last index, as a PHP array).
fn find_next_untyped_properties_declaration(
    tokens: &Tokens,
    mut index: usize,
) -> Vec<(Vec<u8>, usize)> {
    loop {
        index = tokens.get_next_meaningful_token(index).unwrap();
        if !tokens[index].is_given_kind(&[T_PRIVATE, T_PROTECTED, T_PUBLIC, T_STATIC, T_VAR]) {
            break;
        }
    }
    if !tokens[index].is_kind(T_VARIABLE) {
        return Vec::new();
    }
    let mut properties: Vec<(Vec<u8>, usize)> = Vec::new();
    while !tokens[index].equals(Proto::Char(b";"), true) {
        if tokens[index].is_kind(T_VARIABLE) {
            let name = tokens[index].get_content();
            match properties.iter_mut().find(|(n, _)| n == name) {
                Some(entry) => entry.1 = index,
                None => properties.push((name.to_vec(), index)),
            }
        }
        index = tokens.get_next_meaningful_token(index).unwrap();
    }
    properties
}

impl PhpdocToPropertyTypeFixer {
    /// `fixClass`.
    fn fix_class(
        &self,
        common: &Common,
        tokens: &Tokens,
        index: usize,
        types_to_exclude: &[Vec<u8>],
        tokens_to_insert: &mut BTreeMap<usize, Vec<Token>>,
    ) {
        let mut index = tokens
            .get_next_token_of_kind(index, &[Proto::Char(b"{")], true)
            .unwrap();
        let class_end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
        while index < class_end_index {
            if tokens[index].is_kind(T_FUNCTION) {
                index = tokens
                    .get_next_token_of_kind(index, &[Proto::Char(b"{"), Proto::Char(b";")], true)
                    .unwrap();
                if tokens[index].equals(Proto::Char(b"{"), true) {
                    index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
                }
                index += 1;
                continue;
            }
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                index += 1;
                continue;
            }
            let doc_comment_index = index;
            let property_indices =
                find_next_untyped_properties_declaration(tokens, doc_comment_index);
            if property_indices.is_empty() {
                index += 1;
                continue;
            }
            let (doc, scope) = doc_comment(tokens, doc_comment_index);
            let Some(type_info) = resolve_applicable_type(
                common,
                &property_indices,
                &doc.get_annotations_of_type(&["var"]),
                &scope,
            ) else {
                index += 1;
                continue;
            };
            let property_type = type_info.common_type;
            if [&b"callable"[..], b"never", b"void"].contains(&&property_type[..])
                || types_to_exclude.contains(&property_type)
            {
                index += 1;
                continue;
            }
            let code = type_check_code(&property_type);
            if !is_valid_syntax(&code) {
                index += 1;
                continue;
            }
            let mut new_tokens = create_type_declaration_tokens(
                tokens_from_raw_type(&code, 8, 5),
                &property_type,
                type_info.is_nullable,
            );
            new_tokens.push(Token::new(T_WHITESPACE, " "));
            // `$tokensToInsert += $this->fixClass(...)`: an index already planned keeps its tokens.
            tokens_to_insert
                .entry(property_indices[0].1)
                .or_insert(new_tokens);
            index = property_indices.iter().map(|p| p.1).max().unwrap() + 2;
        }
    }
}

/// `resolveApplicableType`.
fn resolve_applicable_type(
    common: &Common,
    property_indices: &[(Vec<u8>, usize)],
    annotations: &[&Annotation],
    scope: &Scope,
) -> Option<TypeInfo> {
    let mut property_types: Vec<(Vec<u8>, TypeInfo)> = Vec::new();
    for annotation in annotations {
        let property_name = match annotation.get_variable_name() {
            Some(name) => name,
            None if property_indices.len() != 1 => continue,
            None => property_indices[0].0.clone(),
        };
        if !property_indices.iter().any(|(n, _)| *n == property_name) {
            continue;
        }
        let Some(type_info) = common.type_info(annotation, scope, false) else {
            continue;
        };
        match property_types.iter().find(|(n, _)| *n == property_name) {
            Some((_, existing)) if *existing != type_info => return None,
            Some(_) => {}
            None => property_types.push((property_name, type_info)),
        }
    }
    if property_types.len() != property_indices.len() {
        return None;
    }
    let mut types = property_types.into_iter().map(|(_, t)| t);
    let type_ = types.next().unwrap();
    for property_type in types {
        if property_type != type_ {
            return None;
        }
    }
    Some(type_)
}

impl Fixer for PhpdocToPropertyTypeFixer {
    fn name(&self) -> &'static str {
        "phpdoc_to_property_type"
    }
    fn priority(&self) -> i32 {
        8
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
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
            if tokens[index].is_given_kind(&[T_CLASS, T_TRAIT]) {
                self.fix_class(
                    &common,
                    tokens,
                    index,
                    &types_to_exclude,
                    &mut tokens_to_insert,
                );
            }
        }
        tokens.insert_slices(tokens_to_insert);
    }
}
