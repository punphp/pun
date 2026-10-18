//! Port of `PhpCsFixer\Fixer\ClassNotation\ModernSerializationMethodsFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::tokens_analyzer::{ClassyElementType, TokensAnalyzer};
use pun_tokens::{Token, Tokens};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

struct ModernSerializationMethodsFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(ModernSerializationMethodsFixer)
}

impl Fixer for ModernSerializationMethodsFixer {
    fn name(&self) -> &'static str {
        "modern_serialization_methods"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_CLASS, T_FUNCTION])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // class index => method name (case-sensitive, as upstream's array keys) => name index
        let mut methods_by_class: BTreeMap<usize, HashMap<Vec<u8>, usize>> = BTreeMap::new();
        for (index, element) in TokensAnalyzer::new(tokens).get_classy_elements() {
            if element.kind != ClassyElementType::Method {
                continue;
            }
            let function_name_index = tokens.get_next_meaningful_token(index).unwrap();
            methods_by_class
                .entry(element.class_index)
                .or_default()
                .insert(
                    tokens[function_name_index].content.to_vec(),
                    function_name_index,
                );
        }
        let mut tokens_to_insert: BTreeMap<usize, Vec<Token>> = BTreeMap::new();
        for methods in methods_by_class.values() {
            if let Some(&sleep) = methods.get(&b"__sleep"[..])
                && !methods.contains_key(&b"__serialize"[..])
            {
                tokens.set(sleep, Token::new(T_STRING, "__serialize"));
            }
            if let Some(&wakeup) = methods.get(&b"__wakeup"[..])
                && !methods.contains_key(&b"__unserialize"[..])
            {
                tokens.set(wakeup, Token::new(T_STRING, "__unserialize"));
                let open_parenthesis_index = tokens
                    .get_next_token_of_kind(wakeup, &["(".into()], true)
                    .unwrap();
                tokens_to_insert.insert(
                    open_parenthesis_index + 1,
                    vec![
                        Token::new(CT_ARRAY_TYPEHINT, "array"),
                        Token::new(T_WHITESPACE, " "),
                        Token::new(T_VARIABLE, "$data"),
                    ],
                );
            }
        }
        if !tokens_to_insert.is_empty() {
            tokens.insert_slices(tokens_to_insert);
        }
    }
}
