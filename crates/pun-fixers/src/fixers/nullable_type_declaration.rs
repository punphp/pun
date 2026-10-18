//! Port of `PhpCsFixer\Fixer\LanguageConstruct\NullableTypeDeclarationFixer`.
use crate::analyzers::analysis::TypeAnalysis;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::php_compat::substr_count;
use pun_tokens::tokens_analyzer::ClassyElementType;
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Default)]
struct NullableTypeDeclarationFixer {
    question_mark: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(NullableTypeDeclarationFixer::default())
}

const PROPERTY_MODIFIERS: &[Kind] = &[
    T_PRIVATE,
    T_PROTECTED,
    T_PUBLIC,
    T_STATIC,
    T_ABSTRACT,
    T_FINAL,
    T_VAR,
    T_READONLY,
    T_PRIVATE_SET,
    T_PROTECTED_SET,
    T_PUBLIC_SET,
];

impl Fixer for NullableTypeDeclarationFixer {
    fn name(&self) -> &'static str {
        "nullable_type_declaration"
    }
    fn priority(&self) -> i32 {
        2
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["syntax"])?;
        self.question_mark =
            c.string("syntax", "question_mark", &["union", "question_mark"])? == "question_mark";
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(if self.question_mark {
            CT_TYPE_ALTERNATION
        } else {
            CT_NULLABLE_TYPE
        })
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        for (index, is_property) in get_elements(tokens).into_iter().rev() {
            if is_property {
                self.normalize_property_type(tokens, index);
                continue;
            }
            self.normalize_method_return_type(&functions_analyzer, tokens, index);
            self.normalize_method_argument_type(&functions_analyzer, tokens, index);
        }
    }
}

/// index => true for 'property', false for 'function'.
fn get_elements(tokens: &Tokens) -> BTreeMap<usize, bool> {
    let mut elements: BTreeMap<usize, bool> = TokensAnalyzer::new(tokens)
        .get_classy_elements()
        .into_iter()
        .filter_map(|(i, e)| match e.kind {
            ClassyElementType::Method => Some((i, false)),
            ClassyElementType::Property => Some((i, true)),
            _ => None,
        })
        .collect();
    for (index, token) in tokens.iter().enumerate() {
        if token.is_kind(T_FN) || (token.is_kind(T_FUNCTION) && !elements.contains_key(&index)) {
            elements.insert(index, false);
        }
    }
    elements
}

fn collect_type_analysis(
    tokens: &Tokens,
    start_index: usize,
    end_index: usize,
) -> Option<TypeAnalysis> {
    let mut type_ = Vec::new();
    let type_start_index = tokens.get_next_meaningful_token(start_index).unwrap();
    let mut type_end_index = type_start_index;
    for i in type_start_index..end_index {
        if tokens[i].is_whitespace(None) || tokens[i].is_comment() {
            continue;
        }
        type_.extend_from_slice(&tokens[i].content);
        type_end_index = i;
    }
    (!type_.is_empty())
        .then(|| TypeAnalysis::new(&type_, Some(type_start_index), Some(type_end_index)))
}

fn is_type_normalizable(type_analysis: &TypeAnalysis) -> bool {
    let type_ = &type_analysis.name;
    if type_.eq_ignore_ascii_case(b"null") || !type_analysis.is_nullable() {
        return false;
    }
    if type_.contains(&b'&') {
        return false; // skip DNF types
    }
    if !type_.contains(&b'|') {
        return true;
    }
    substr_count(type_, b"|") == 1
        && Preg::r#match("/(?:\\|null$|^null\\|)/i", type_)
}

impl NullableTypeDeclarationFixer {
    fn normalize_property_type(&self, tokens: &mut Tokens, mut index: usize) {
        let property_end_index = index;
        loop {
            index = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[index].is_given_kind(PROPERTY_MODIFIERS) {
                break;
            }
        }
        if let Some(property_type) = collect_type_analysis(tokens, index, property_end_index)
            .filter(is_type_normalizable)
        {
            self.normalize_nullable_type(tokens, &property_type);
        }
    }

    fn normalize_method_argument_type(
        &self,
        fa: &FunctionsAnalyzer,
        tokens: &mut Tokens,
        index: usize,
    ) {
        let arguments = fa.get_function_arguments(tokens, index).into_iter().rev();
        for argument_type in arguments
            .filter_map(|(_, a)| a.type_analysis)
            .filter(is_type_normalizable)
        {
            self.normalize_nullable_type(tokens, &argument_type);
        }
    }

    fn normalize_method_return_type(
        &self,
        fa: &FunctionsAnalyzer,
        tokens: &mut Tokens,
        index: usize,
    ) {
        let Some(return_type) = fa.get_function_return_type(tokens, index) else {
            return;
        };
        if !is_type_normalizable(&return_type) {
            return;
        }
        self.normalize_nullable_type(tokens, &return_type);
    }

    fn normalize_nullable_type(&self, tokens: &mut Tokens, type_analysis: &TypeAnalysis) {
        let mut type_ = type_analysis.name.clone();
        if !type_.contains(&b'|') && !type_.contains(&b'&') && type_analysis.is_nullable() {
            type_.insert(0, b'?');
        }
        let (normalized_type, normalized_type_as_string) = if self.question_mark {
            let n = convert_to_nullable_type(&type_);
            let s = n.concat();
            (n, s)
        } else {
            let n = convert_to_explicit_union_type(&type_);
            let s = n.join(&b'|');
            (n, s)
        };
        if normalized_type_as_string == type_ {
            return;
        }
        tokens.override_range(
            type_analysis.get_start_index(),
            type_analysis.get_end_index(),
            create_type_declaration_tokens(&normalized_type, self.question_mark),
        );
        let prev_start_index = type_analysis.get_start_index() - 1;
        if !tokens[prev_start_index].is_whitespace(None)
            && !tokens[prev_start_index].equals("(", true)
        {
            tokens.ensure_whitespace_at_index(prev_start_index, 1, b" ");
        }
    }
}

fn convert_to_nullable_type(type_: &[u8]) -> Vec<Vec<u8>> {
    if type_.starts_with(b"?") {
        return vec![type_.to_vec()];
    }
    vec![
        b"?".to_vec(),
        Preg::replace("/(?:\\|null$|^null\\|)/i", "", type_),
    ]
}

fn convert_to_explicit_union_type(type_: &[u8]) -> Vec<Vec<u8>> {
    if type_.contains(&b'|') {
        return vec![type_.to_vec()];
    }
    vec![b"null".to_vec(), type_[1..].to_vec()]
}

fn create_type_declaration_tokens(types: &[Vec<u8>], is_question_mark_syntax: bool) -> Vec<Token> {
    let count = types.len();
    let mut new_tokens = Vec::new();
    for (index, type_) in types.iter().enumerate() {
        let special_type = match &type_.to_ascii_lowercase()[..] {
            b"?" => Some(CT_NULLABLE_TYPE),
            b"array" => Some(CT_ARRAY_TYPEHINT),
            b"callable" => Some(T_CALLABLE),
            b"static" => Some(T_STATIC),
            _ => None,
        };
        if let Some(k) = special_type {
            new_tokens.push(Token::new(k, type_.clone()));
        } else {
            for (ns_index, value) in type_.split(|&c| c == b'\\').enumerate() {
                if ns_index == 0 && value.is_empty() {
                    continue;
                }
                if ns_index > 0 {
                    new_tokens.push(Token::new(T_NS_SEPARATOR, "\\"));
                }
                new_tokens.push(Token::new(T_STRING, value));
            }
        }
        if index + 2 <= count && !is_question_mark_syntax {
            new_tokens.push(Token::new(CT_TYPE_ALTERNATION, "|"));
        }
    }
    new_tokens
}
