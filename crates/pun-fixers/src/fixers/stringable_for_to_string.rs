//! Port of `PhpCsFixer\Fixer\ClassNotation\StringableForToStringFixer`.
use crate::analyzers::analysis::NamespaceUseAnalysis;
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

struct StringableForToStringFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(StringableForToStringFixer)
}

impl Fixer for StringableForToStringFixer {
    fn name(&self) -> &'static str {
        "stringable_for_to_string"
    }
    fn priority(&self) -> i32 {
        37
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_CLASS, T_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let use_declarations = NamespaceUsesAnalyzer.get_declarations_from_tokens(tokens, false);
        let mut stringable_interfaces: Vec<Vec<u8>> = vec![b"stringable".to_vec()];
        let mut index = 1;
        while index < tokens.count() {
            if tokens[index].is_kind(T_NAMESPACE) {
                stringable_interfaces.clear();
            } else if tokens[index].is_kind(T_USE) {
                if let Some(name) = get_name_from_use(index, &use_declarations) {
                    stringable_interfaces.push(name);
                }
            } else if tokens[index].is_kind(T_CLASS) {
                let class_start_index = tokens
                    .get_next_token_of_kind(index, &["{".into()], true)
                    .unwrap();
                let class_end_index =
                    tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, class_start_index);
                if does_have_to_string_method(tokens, class_start_index, class_end_index)
                    && !does_implement_stringable(
                        tokens,
                        index,
                        class_start_index,
                        &stringable_interfaces,
                    )
                {
                    add_stringable_interface(tokens, index);
                }
            }
            index += 1;
        }
    }
}

fn get_name_from_use(index: usize, use_declarations: &[NamespaceUseAnalysis]) -> Option<Vec<u8>> {
    let use_declaration = use_declarations.iter().find(|u| u.start_index == index)?;
    let lowercased_full_name = strtolower(&use_declaration.full_name);
    if lowercased_full_name != b"stringable" && lowercased_full_name != b"\\stringable" {
        return None;
    }
    Some(strtolower(&use_declaration.short_name))
}

fn does_have_to_string_method(
    tokens: &Tokens,
    class_start_index: usize,
    class_end_index: usize,
) -> bool {
    let mut index = class_start_index;
    while index < class_end_index {
        index += 1;
        if tokens[index].equals("{", true) {
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
            continue;
        }
        if !tokens[index].is_kind(T_FUNCTION) {
            continue;
        }
        let function_name_index = tokens.get_next_meaningful_token(index).unwrap();
        if tokens[function_name_index].equals((T_STRING, "__toString"), false) {
            return true;
        }
    }
    false
}

fn does_implement_stringable(
    tokens: &Tokens,
    class_keyword_index: usize,
    class_open_brace_index: usize,
    stringable_interfaces: &[Vec<u8>],
) -> bool {
    let implemented = get_interfaces(tokens, class_keyword_index, class_open_brace_index);
    if implemented.is_empty() {
        return false;
    }
    if implemented.iter().any(|i| i == b"\\stringable") {
        return true;
    }
    stringable_interfaces
        .iter()
        .any(|s| implemented.contains(s))
}

fn get_interfaces(
    tokens: &Tokens,
    class_keyword_index: usize,
    class_open_brace_index: usize,
) -> Vec<Vec<u8>> {
    let implements_index = tokens
        .get_next_token_of_kind(
            class_keyword_index,
            &["{".into(), T_IMPLEMENTS.into()],
            true,
        )
        .unwrap();
    let mut interfaces = Vec::new();
    let mut interface = Vec::new();
    let mut index = tokens.get_next_meaningful_token(implements_index).unwrap();
    while index < class_open_brace_index {
        if tokens[index].equals(",", true) {
            interfaces.push(strtolower(&interface));
            interface.clear();
        } else {
            interface.extend_from_slice(&tokens[index].content);
        }
        index = tokens.get_next_meaningful_token(index).unwrap();
    }
    if !interface.is_empty() {
        interfaces.push(strtolower(&interface));
    }
    interfaces
}

fn add_stringable_interface(tokens: &mut Tokens, class_index: usize) {
    let implements_index = tokens
        .get_next_token_of_kind(class_index, &["{".into(), T_IMPLEMENTS.into()], true)
        .unwrap();
    if tokens[implements_index].equals("{", true) {
        let prev_index = tokens.get_prev_meaningful_token(implements_index).unwrap();
        tokens.insert_slices(BTreeMap::from([(
            prev_index + 1,
            vec![
                Token::new(T_WHITESPACE, " "),
                Token::new(T_IMPLEMENTS, "implements"),
                Token::new(T_WHITESPACE, " "),
                Token::new(T_NS_SEPARATOR, "\\"),
                Token::new(T_STRING, "Stringable"),
            ],
        )]));
        return;
    }
    let after_implements_index = tokens.get_next_meaningful_token(implements_index).unwrap();
    tokens.insert_slices(BTreeMap::from([(
        after_implements_index,
        vec![
            Token::new(T_NS_SEPARATOR, "\\"),
            Token::new(T_STRING, "Stringable"),
            Token::char(","),
            Token::new(T_WHITESPACE, " "),
        ],
    )]));
}
