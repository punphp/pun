//! Port of `PhpCsFixer\Fixer\ClassNotation\ProtectedToPrivateFixer`.
use pun_format::Fixer;
use pun_tokens::tokens_analyzer::ClassyElementType as T;
use pun_tokens::{Kind, Token, Tokens, TokensAnalyzer};
use std::collections::HashMap;
use std::path::Path;

const MODIFIER_KINDS: &[Kind] = &[
    Kind::T_PUBLIC,
    Kind::T_PROTECTED,
    Kind::T_PRIVATE,
    Kind::T_FINAL,
    Kind::T_ABSTRACT,
    Kind::T_NS_SEPARATOR,
    Kind::T_STRING,
    Kind::CT_NULLABLE_TYPE,
    Kind::CT_ARRAY_TYPEHINT,
    Kind::T_STATIC,
    Kind::CT_TYPE_ALTERNATION,
    Kind::CT_TYPE_INTERSECTION,
    Kind::T_READONLY,
    Kind::T_PRIVATE_SET,
    Kind::T_PROTECTED_SET,
];

struct ProtectedToPrivateFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(ProtectedToPrivateFixer)
}

impl Fixer for ProtectedToPrivateFixer {
    fn name(&self) -> &'static str {
        "protected_to_private"
    }
    fn priority(&self) -> i32 {
        66
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[
            Kind::T_PROTECTED,
            Kind::CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
            Kind::T_PROTECTED_SET,
        ]) && (tokens.is_all_token_kinds_found(&[Kind::T_CLASS, Kind::T_FINAL])
            || tokens.is_token_kind_found(Kind::T_ENUM))
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let elements = TokensAnalyzer::new(tokens).get_classy_elements();
        let mut classes_candidate: HashMap<usize, bool> = HashMap::new();
        for (index, element) in elements {
            let class_index = element.class_index;
            let candidate = *classes_candidate
                .entry(class_index)
                .or_insert_with(|| is_class_candidate(tokens, class_index));
            if !candidate
                || !matches!(
                    element.kind,
                    T::Method | T::Property | T::PromotedProperty | T::Const
                )
            {
                continue;
            }
            let mut previous_index = index;
            let (
                mut protected_index,
                mut protected_promoted_index,
                mut protected_set_index,
                mut is_final,
            ) = (None, None, None, false);
            loop {
                previous_index = tokens.get_prev_meaningful_token(previous_index).unwrap();
                let t = &tokens[previous_index];
                if t.is_kind(Kind::T_PROTECTED) {
                    protected_index = Some(previous_index);
                } else if t.is_kind(Kind::CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED) {
                    protected_promoted_index = Some(previous_index);
                } else if t.is_kind(Kind::T_PROTECTED_SET) {
                    protected_set_index = Some(previous_index);
                } else if t.is_kind(Kind::T_FINAL) {
                    is_final = true;
                }
                if !tokens[previous_index].is_given_kind(MODIFIER_KINDS) {
                    break;
                }
            }
            if is_final && element.kind == T::Const {
                continue;
            }
            if let Some(i) = protected_index {
                tokens.set(i, Token::new(Kind::T_PRIVATE, b"private".to_vec()));
            }
            if let Some(i) = protected_promoted_index {
                tokens.set(
                    i,
                    Token::new(
                        Kind::CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
                        b"private".to_vec(),
                    ),
                );
            }
            if let Some(i) = protected_set_index {
                tokens.set(i, Token::new(Kind::T_PRIVATE_SET, b"private(set)".to_vec()));
            }
        }
    }
}

fn is_class_candidate(tokens: &Tokens, class_index: usize) -> bool {
    if tokens[class_index].is_kind(Kind::T_ENUM) {
        return true;
    }
    let analyzer = TokensAnalyzer::new(tokens);
    if !tokens[class_index].is_kind(Kind::T_CLASS) || analyzer.is_anonymous_class(class_index) {
        return false;
    }
    if analyzer.get_classy_modifiers(class_index).final_.is_none() {
        return false;
    }
    let class_name_index = tokens.get_next_meaningful_token(class_index).unwrap();
    let class_extends_index = tokens.get_next_meaningful_token(class_name_index).unwrap();
    if tokens[class_extends_index].is_kind(Kind::T_EXTENDS) {
        return false;
    }
    if !tokens.is_token_kind_found(Kind::CT_USE_TRAIT) {
        return true;
    }
    let class_open_index = tokens
        .get_next_token_of_kind(class_name_index, &["{".into()], true)
        .unwrap();
    let class_close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, class_open_index);
    tokens
        .get_next_token_of_kind(class_open_index, &[Kind::CT_USE_TRAIT.into()], true)
        .is_none_or(|u| u > class_close_index)
}
