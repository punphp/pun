//! Port of `PhpCsFixer\Fixer\Whitespace\TypeDeclarationSpacesFixer`.
use crate::analyzers::analysis::TypeAnalysis;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind, Kind::*, Tokens, TokensAnalyzer, token::CLASSY_KINDS};
use serde_json::Value;
use std::path::Path;

const PROPERTY_MODIFIERS: &[Kind] = &[
    T_PRIVATE,
    T_PROTECTED,
    T_PUBLIC,
    T_STATIC,
    T_VAR,
    T_FINAL,
    T_READONLY,
    T_PUBLIC_SET,
    T_PROTECTED_SET,
    T_PRIVATE_SET,
];

#[derive(Default)]
struct TypeDeclarationSpacesFixer {
    elements: Vec<String>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(TypeDeclarationSpacesFixer::default())
}

impl Fixer for TypeDeclarationSpacesFixer {
    fn name(&self) -> &'static str {
        "type_declaration_spaces"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CLASSY_KINDS)
            || tokens.is_any_token_kinds_found(&[T_FN, T_FUNCTION])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["elements"])?;
        self.elements = o.strings(
            "elements",
            // Future::getV4OrV3(['function', 'property', 'constant'], ['function', 'property'])
            &["function", "property"],
            &["function", "property", "constant"],
        )?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let has = |e: &str| self.elements.iter().any(|x| x == e);
        for (index, ty) in get_elements(tokens).into_iter().rev() {
            if ty == "property" && has("property") {
                ensure_single_space_at_property_typehint(tokens, index);
                continue;
            }
            if ty == "method" && has("function") {
                ensure_single_space_at_function_argument_typehint(
                    &functions_analyzer,
                    tokens,
                    index,
                );
                continue;
            }
            if ty == "const" && has("constant") {
                ensure_single_space_at_constant_typehint(tokens, index);
            }
        }
    }
}

/// In PHP array insertion order: classy elements by index, then other functions/arrow functions by index.
fn get_elements(tokens: &Tokens) -> Vec<(usize, &'static str)> {
    let mut elements: Vec<(usize, &'static str)> = TokensAnalyzer::new(tokens)
        .get_classy_elements()
        .into_iter()
        .map(|(i, e)| (i, e.kind.as_str()))
        .filter(|(_, t)| ["method", "property", "const"].contains(t))
        .collect();
    let classy: std::collections::HashSet<usize> = elements.iter().map(|(i, _)| *i).collect();
    for (index, token) in tokens.iter().enumerate() {
        if token.is_kind(T_FN) || (token.is_kind(T_FUNCTION) && !classy.contains(&index)) {
            elements.push((index, "method"));
        }
    }
    elements
}

fn ensure_single_space_at_function_argument_typehint(
    functions_analyzer: &FunctionsAnalyzer,
    tokens: &mut Tokens,
    index: usize,
) {
    for (_, argument_info) in functions_analyzer
        .get_function_arguments(tokens, index)
        .into_iter()
        .rev()
    {
        let Some(argument_type) = argument_info.type_analysis else {
            continue;
        };
        tokens.ensure_whitespace_at_index(argument_type.get_end_index() + 1, 0, b" ");
    }
}

fn ensure_single_space_at_property_typehint(tokens: &mut Tokens, mut index: usize) {
    let property_index = index;
    loop {
        index = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[index].is_given_kind(PROPERTY_MODIFIERS) {
            break;
        }
    }
    let Some(property_type) = collect_type_analysis(tokens, index, property_index) else {
        return;
    };
    tokens.ensure_whitespace_at_index(property_type.get_end_index() + 1, 0, b" ");
}

fn ensure_single_space_at_constant_typehint(tokens: &mut Tokens, index: usize) {
    let const_index = index;
    // upstream's null check: a class constant always has `=` in parsed code
    let equals_index = tokens.get_next_token_of_kind(const_index, &["=".into()], true).unwrap();
    // upstream returns unless this is T_STRING; the tokenizer turns keyword-named constants into T_STRING too
    let name_index = tokens.get_prev_meaningful_token(equals_index).unwrap();
    let type_end_index = match tokens.get_prev_meaningful_token(name_index) {
        Some(i) if !tokens[i].is_kind(T_CONST) => i,
        _ => return,
    };
    tokens.ensure_whitespace_at_index(type_end_index + 1, 0, b" ");
}

fn collect_type_analysis(
    tokens: &Tokens,
    start_index: usize,
    end_index: usize,
) -> Option<TypeAnalysis> {
    let mut ty = Vec::new();
    let type_start_index = tokens.get_next_meaningful_token(start_index).unwrap();
    let mut type_end_index = type_start_index;
    for i in type_start_index..end_index {
        if tokens[i].is_whitespace(None) || tokens[i].is_comment() {
            continue;
        }
        ty.extend_from_slice(&tokens[i].content);
        type_end_index = i;
    }
    (!ty.is_empty()).then(|| TypeAnalysis::new(&ty, Some(type_start_index), Some(type_end_index)))
}
