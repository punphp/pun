//! Port of `PhpCsFixer\Fixer\ClassNotation\ModifierKeywordsFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::token::CLASSY_KINDS;
use pun_tokens::tokens_analyzer::ClassyElementType as T;
use pun_tokens::{Kind, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

const PROPERTY_TYPE_DECLARATION_KINDS: &[Kind] = &[
    Kind::T_STRING,
    Kind::T_NS_SEPARATOR,
    Kind::CT_NULLABLE_TYPE,
    Kind::CT_ARRAY_TYPEHINT,
    Kind::CT_TYPE_ALTERNATION,
    Kind::CT_TYPE_INTERSECTION,
    Kind::CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN,
    Kind::CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
];
const EXPECTED_KINDS_GENERIC: &[Kind] = &[
    Kind::T_ABSTRACT,
    Kind::T_FINAL,
    Kind::T_PRIVATE,
    Kind::T_PROTECTED,
    Kind::T_PUBLIC,
    Kind::T_STATIC,
    Kind::T_VAR,
    Kind::CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
    Kind::CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
    Kind::CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
    Kind::T_READONLY,
    Kind::T_PRIVATE_SET,
    Kind::T_PROTECTED_SET,
    Kind::T_PUBLIC_SET,
];
const ELEMENTS: &[&str] = &["const", "method", "property"];

struct ModifierKeywordsFixer {
    elements: Vec<T>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(ModifierKeywordsFixer { elements: vec![] })
}

impl Fixer for ModifierKeywordsFixer {
    fn name(&self) -> &'static str {
        "modifier_keywords"
    }
    fn priority(&self) -> i32 {
        56
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve("modifier_keywords", options, &["elements"])?;
        self.elements = r.choices(
            "elements",
            ELEMENTS,
            &[
                ("const", T::Const),
                ("method", T::Method),
                ("property", T::Property),
            ],
        )?;
        if self.elements.contains(&T::Property) {
            self.elements.push(T::PromotedProperty);
        }
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CLASSY_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let elements = TokensAnalyzer::new(tokens).get_classy_elements();
        for (&index, element) in elements.iter().rev() {
            if !self.elements.contains(&element.kind) {
                continue;
            }
            let mut index = index;
            let (
                mut abstract_final_index,
                mut visibility_index,
                mut visibility_set_index,
                mut static_index,
                mut type_index,
                mut read_only_index,
            ) = (None, None, None, None, None, None);
            let mut prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            let is_property = matches!(element.kind, T::Property | T::PromotedProperty);
            let expected = |t: &Token| {
                t.is_given_kind(EXPECTED_KINDS_GENERIC)
                    || (is_property && t.is_given_kind(PROPERTY_TYPE_DECLARATION_KINDS))
            };
            while expected(&tokens[prev_index]) || tokens[prev_index].equals("&", true) {
                let t = &tokens[prev_index];
                if t.is_given_kind(&[Kind::T_ABSTRACT, Kind::T_FINAL]) {
                    abstract_final_index = Some(prev_index);
                } else if t.is_kind(Kind::T_STATIC) {
                    static_index = Some(prev_index);
                } else if t.is_kind(Kind::T_READONLY) {
                    read_only_index = Some(prev_index);
                } else if t.is_given_kind(&[
                    Kind::T_PRIVATE_SET,
                    Kind::T_PROTECTED_SET,
                    Kind::T_PUBLIC_SET,
                ]) {
                    visibility_set_index = Some(prev_index);
                } else if t.is_given_kind(PROPERTY_TYPE_DECLARATION_KINDS) {
                    type_index = Some(prev_index);
                } else if !t.equals("&", true) {
                    visibility_index = Some(prev_index);
                }
                prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
            }
            if let Some(t) = type_index {
                index = t;
            }
            if element.kind == T::Property && tokens[prev_index].equals(",", true) {
                continue;
            }
            for keyword in [static_index.or(read_only_index), visibility_set_index]
                .into_iter()
                .flatten()
            {
                if is_keyword_placed_properly(tokens, keyword, index) {
                    index = keyword;
                } else {
                    move_token_and_ensure_single_space_follows(tokens, keyword, index);
                }
            }
            match visibility_index {
                None => {
                    let kind = if element.kind == T::PromotedProperty {
                        Kind::CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC
                    } else {
                        Kind::T_PUBLIC
                    };
                    tokens.insert_at(
                        index,
                        vec![
                            Token::new(kind, b"public".to_vec()),
                            Token::new(Kind::T_WHITESPACE, b" ".to_vec()),
                        ],
                    );
                }
                Some(v) => {
                    if tokens[v].is_kind(Kind::T_VAR) {
                        tokens.set(v, Token::new(Kind::T_PUBLIC, b"public".to_vec()));
                    }
                    if is_keyword_placed_properly(tokens, v, index) {
                        index = v;
                    } else {
                        move_token_and_ensure_single_space_follows(tokens, v, index);
                    }
                }
            }
            let Some(af) = abstract_final_index else {
                continue;
            };
            if is_keyword_placed_properly(tokens, af, index) {
                continue;
            }
            move_token_and_ensure_single_space_follows(tokens, af, index);
        }
    }
}

fn is_keyword_placed_properly(
    tokens: &Tokens,
    keyword_index: usize,
    compared_index: usize,
) -> bool {
    tokens[keyword_index + 1].get_content() == b" "
        && (keyword_index + 2 == compared_index
            || (keyword_index + 3 == compared_index && tokens[keyword_index + 2].equals("&", true)))
}

fn move_token_and_ensure_single_space_follows(
    tokens: &mut Tokens,
    from_index: usize,
    to_index: usize,
) {
    let t = tokens[from_index].clone();
    tokens.insert_at(
        to_index,
        vec![t, Token::new(Kind::T_WHITESPACE, b" ".to_vec())],
    );
    tokens.clear_at(from_index);
    if tokens[from_index + 1].is_whitespace(None) {
        tokens.clear_at(from_index + 1);
    }
}
