//! Port of `PhpCsFixer\Fixer\FunctionNotation\NullableTypeDeclarationForDefaultNullValueFixer`.
use crate::analyzers::analysis::{ArgumentAnalysis, TypeAnalysis};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct NullableTypeDeclarationForDefaultNullValueFixer {
    use_nullable_type_declaration: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NullableTypeDeclarationForDefaultNullValueFixer { use_nullable_type_declaration: true })
}

const CONSTRUCTOR_PROPERTY_MODIFIERS: &[Kind] = &[
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
    T_PUBLIC_SET,
    T_PROTECTED_SET,
    T_PRIVATE_SET,
    T_READONLY,
];

impl Fixer for NullableTypeDeclarationForDefaultNullValueFixer {
    fn name(&self) -> &'static str {
        "nullable_type_declaration_for_default_null_value"
    }
    fn priority(&self) -> i32 {
        3
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["use_nullable_type_declaration"])?;
        self.use_nullable_type_declaration = c.bool("use_nullable_type_declaration", true)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_VARIABLE) && tokens.is_any_token_kinds_found(&[T_FUNCTION, T_FN])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_given_kind(&[T_FUNCTION, T_FN]) {
                continue;
            }
            let arguments = functions_analyzer.get_function_arguments(tokens, index);
            self.fix_function_parameters(tokens, arguments.into_iter().map(|(_, a)| a).collect());
        }
    }
}

impl NullableTypeDeclarationForDefaultNullValueFixer {
    fn fix_function_parameters(&self, tokens: &mut Tokens, arguments: Vec<ArgumentAnalysis>) {
        for argument_info in arguments.into_iter().rev() {
            let Some(argument_type_info) = argument_info.type_analysis else { continue };
            let lower = argument_type_info.name.to_ascii_lowercase();
            if lower == b"mixed" || lower == b"null" || !argument_info.default.as_ref().is_some_and(|d| d.eq_ignore_ascii_case(b"null")) {
                continue;
            }
            if !self.use_nullable_type_declaration {
                let visibility = &tokens[tokens.get_prev_meaningful_token(argument_type_info.get_start_index()).unwrap()];
                if visibility.is_given_kind(CONSTRUCTOR_PROPERTY_MODIFIERS) {
                    continue;
                }
            }
            let name = &argument_type_info.name;
            if name.contains(&b'|') || name.contains(&b'&') {
                self.fix_union_type_parameter(tokens, &argument_type_info);
            } else {
                self.fix_single_type_parameter(tokens, &argument_type_info);
            }
        }
    }

    fn fix_single_type_parameter(&self, tokens: &mut Tokens, info: &TypeAnalysis) {
        if self.use_nullable_type_declaration {
            if !info.is_nullable() {
                tokens.insert_at(info.get_start_index(), vec![Token::new(CT_NULLABLE_TYPE, "?")]);
            }
        } else if info.is_nullable() {
            let start_index = info.get_start_index();
            tokens.remove_trailing_whitespace(start_index, None);
            tokens.clear_token_and_merge_surrounding_whitespace(start_index);
        }
    }

    fn fix_union_type_parameter(&self, tokens: &mut Tokens, info: &TypeAnalysis) {
        let name = &info.name;
        if self.use_nullable_type_declaration {
            if info.is_nullable() {
                return;
            }
            let mut end_index = info.get_end_index();
            if name.contains(&b'&') && !name.contains(&b'|') {
                end_index += 2;
                tokens.insert_at(info.get_start_index(), vec![Token::new(CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN, "(")]);
                tokens.insert_at(end_index, vec![Token::new(CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE, ")")]);
            }
            tokens.insert_at(end_index + 1, vec![Token::new(CT_TYPE_ALTERNATION, "|"), Token::new(T_STRING, "null")]);
        } else if info.is_nullable() {
            let start_index = info.get_start_index();
            let mut index = tokens.get_next_token_of_kind(start_index - 1, &[(T_STRING, "null").into()], false).unwrap();
            if index == start_index {
                tokens.remove_trailing_whitespace(index, None);
                tokens.clear_token_and_merge_surrounding_whitespace(index);
                index = tokens.get_next_meaningful_token(index).unwrap();
                if tokens[index].equals((CT_TYPE_ALTERNATION, "|"), true) {
                    tokens.remove_trailing_whitespace(index, None);
                    tokens.clear_token_and_merge_surrounding_whitespace(index);
                }
            } else {
                tokens.remove_leading_whitespace(index, None);
                tokens.clear_token_and_merge_surrounding_whitespace(index);
                index = tokens.get_prev_meaningful_token(index).unwrap();
                if tokens[index].equals((CT_TYPE_ALTERNATION, "|"), true) {
                    tokens.remove_leading_whitespace(index, None);
                    tokens.clear_token_and_merge_surrounding_whitespace(index);
                }
            }
            if name.contains(&b'&') && name.iter().filter(|&&c| c == b'|').count() == 1 {
                let index = tokens
                    .get_next_token_of_kind(start_index - 1, &[CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN.into()], true)
                    .unwrap();
                tokens.remove_trailing_whitespace(index, None);
                tokens.clear_token_and_merge_surrounding_whitespace(index);
                let index = tokens
                    .get_prev_token_of_kind(info.get_end_index() + 1, &[CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE.into()], true)
                    .unwrap();
                tokens.remove_leading_whitespace(index, None);
                tokens.clear_token_and_merge_surrounding_whitespace(index);
            }
        }
    }
}
