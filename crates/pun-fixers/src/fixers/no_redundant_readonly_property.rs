//! Port of `PhpCsFixer\Fixer\ClassNotation\NoRedundantReadonlyPropertyFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::Tokens;
use pun_tokens::tokens_analyzer::{ClassyElementType, TokensAnalyzer};
use std::path::Path;

struct NoRedundantReadonlyPropertyFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoRedundantReadonlyPropertyFixer)
}

/// `EXPECTED_KINDS_PROPERTY_KINDS`: `EXPECTED_KINDS_GENERIC` followed by `PROPERTY_TYPE_DECLARATION_KINDS`.
const EXPECTED_KINDS_PROPERTY_KINDS: &[Kind] = &[
    T_ABSTRACT,
    T_FINAL,
    T_PRIVATE,
    T_PROTECTED,
    T_PUBLIC,
    T_STATIC,
    T_VAR,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
    T_READONLY,
    T_PRIVATE_SET,
    T_PROTECTED_SET,
    T_PUBLIC_SET,
    T_STRING,
    T_NS_SEPARATOR,
    CT_NULLABLE_TYPE,
    CT_ARRAY_TYPEHINT,
    CT_TYPE_ALTERNATION,
    CT_TYPE_INTERSECTION,
    CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN,
    CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
];

impl Fixer for NoRedundantReadonlyPropertyFixer {
    fn name(&self) -> &'static str {
        "no_redundant_readonly_property"
    }
    fn priority(&self) -> i32 {
        3
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        // `\PHP_VERSION_ID >= 8_02_00`: always true on the PHP 8.5 oracle.
        tokens.is_any_token_kinds_found(&[T_CLASS])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let elements = TokensAnalyzer::new(tokens).get_classy_elements();
        for (index, element) in elements {
            if !matches!(
                element.kind,
                ClassyElementType::Property | ClassyElementType::PromotedProperty
            ) {
                continue;
            }
            let modifiers = TokensAnalyzer::new(tokens).get_classy_modifiers(element.class_index);
            if modifiers.readonly.is_none() {
                continue;
            }
            let mut read_only_index = None;
            let mut prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            while tokens[prev_index].is_given_kind(EXPECTED_KINDS_PROPERTY_KINDS)
                || tokens[prev_index].equals("&", true)
            {
                if tokens[prev_index].is_kind(T_READONLY) {
                    read_only_index = Some(prev_index);
                    break;
                }
                prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
            }
            if let Some(read_only_index) = read_only_index {
                tokens.remove_trailing_whitespace(read_only_index, None);
                tokens.clear_at(read_only_index);
            }
        }
    }
}
