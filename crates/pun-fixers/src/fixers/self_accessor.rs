//! Port of `PhpCsFixer\Fixer\ClassNotation\SelfAccessorFixer`.
use crate::analyzers::namespaces_analyzer::NamespacesAnalyzer;
use crate::analyzers::preg::{PREG_SPLIT_DELIM_CAPTURE, PREG_SPLIT_NO_EMPTY, Preg};
use pun_format::Fixer;
use pun_tokens::{Kind, Proto, Token, Tokens, TokensAnalyzer};
use std::path::Path;

struct SelfAccessorFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SelfAccessorFixer)
}

impl Fixer for SelfAccessorFixer {
    fn name(&self) -> &'static str {
        "self_accessor"
    }
    fn priority(&self) -> i32 {
        -11
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[Kind::T_CLASS, Kind::T_INTERFACE])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for namespace in NamespacesAnalyzer.get_declarations(tokens) {
            let mut index = namespace.scope_start_index;
            while index < namespace.scope_end_index {
                if !tokens[index].is_given_kind(&[Kind::T_CLASS, Kind::T_INTERFACE]) || TokensAnalyzer::new(tokens).is_anonymous_class(index) {
                    index += 1;
                    continue;
                }
                let name_index = tokens.get_next_token_of_kind(index, &[Kind::T_STRING.into()], true).unwrap();
                let start_index = tokens.get_next_token_of_kind(name_index, &["{".into()], true).unwrap();
                let end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, start_index);
                let name = tokens[name_index].content.clone();
                replace_name_occurrences(tokens, &namespace.full_name, &name, start_index, end_index);
                index = end_index + 1;
            }
        }
    }
}

fn replace_name_occurrences(tokens: &mut Tokens, namespace: &[u8], name: &[u8], start_index: usize, end_index: usize) {
    let mut inside_method_signature_until: Option<usize> = None;
    let mut i = start_index;
    while i < end_index {
        if Some(i) == inside_method_signature_until {
            inside_method_signature_until = None;
        }
        let token = &tokens[i];
        if token.is_kind(Kind::T_CLASS) && TokensAnalyzer::new(tokens).is_anonymous_class(i) {
            i = tokens.get_next_token_of_kind(i, &["{".into()], true).unwrap();
            i = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, i) + 1;
            continue;
        }
        if token.is_kind(Kind::T_FN) {
            i = TokensAnalyzer::new(tokens).get_last_token_index_of_arrow_function(i);
            i = tokens.get_next_meaningful_token(i).unwrap() + 1;
            continue;
        }
        if token.is_kind(Kind::T_FUNCTION) {
            if TokensAnalyzer::new(tokens).is_lambda(i) {
                i = tokens.get_next_token_of_kind(i, &["{".into()], true).unwrap();
                i = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, i) + 1;
                continue;
            }
            i = tokens.get_next_token_of_kind(i, &["(".into()], true).unwrap();
            inside_method_signature_until = tokens.get_next_token_of_kind(i, &["{".into(), ";".into()], true);
            i += 1;
            continue;
        }
        if !token.equals(Proto::KindContent(Kind::T_STRING, name), false) {
            i += 1;
            continue;
        }
        let next_token = &tokens[tokens.get_next_meaningful_token(i).unwrap()];
        if next_token.is_kind(Kind::T_NS_SEPARATOR) {
            i += 1;
            continue;
        }
        let mut class_start_index = i;
        let mut prev_token = &tokens[tokens.get_prev_meaningful_token(i).unwrap()];
        if prev_token.is_kind(Kind::T_NS_SEPARATOR) {
            match get_class_start(tokens, i, namespace) {
                None => {
                    i += 1;
                    continue;
                }
                Some(s) => {
                    class_start_index = s;
                    prev_token = &tokens[tokens.get_prev_meaningful_token(s).unwrap()];
                }
            }
        }
        if prev_token.is_given_kind(&[Kind::T_STRING, Kind::T_PAAMAYIM_NEKUDOTAYIM]) || prev_token.is_object_operator() {
            i += 1;
            continue;
        }
        if prev_token.is_given_kind(&[Kind::T_INSTANCEOF, Kind::T_NEW])
            || next_token.is_kind(Kind::T_PAAMAYIM_NEKUDOTAYIM)
            || (inside_method_signature_until.is_some_and(|u| i < u)
                && prev_token.equals_any(
                    &["(".into(), ",".into(), Kind::CT_NULLABLE_TYPE.into(), Kind::CT_TYPE_ALTERNATION.into(), Kind::CT_TYPE_COLON.into()],
                    true,
                ))
        {
            for j in class_start_index..i {
                tokens.clear_token_and_merge_surrounding_whitespace(j);
            }
            tokens.set(i, Token::new(Kind::T_STRING, b"self".to_vec()));
        }
        i += 1;
    }
}

fn get_class_start(tokens: &Tokens, mut index: usize, namespace: &[u8]) -> Option<usize> {
    let ns = [if namespace.is_empty() { vec![] } else { [&b"\\"[..], namespace].concat() }, b"\\".to_vec()].concat();
    for piece in Preg::split(r"/(\\)/", &ns, -1, PREG_SPLIT_NO_EMPTY | PREG_SPLIT_DELIM_CAPTURE).iter().rev() {
        index = tokens.get_prev_meaningful_token(index)?;
        if piece.text == b"\\" {
            if !tokens[index].is_kind(Kind::T_NS_SEPARATOR) {
                return None;
            }
        } else if !tokens[index].equals(Proto::KindContent(Kind::T_STRING, &piece.text), false) {
            return None;
        }
    }
    Some(index)
}
