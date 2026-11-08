//! Port of `PhpCsFixer\Fixer\FunctionNotation\VoidReturnFixer`.
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::{Kind, Kind::*, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

const PREVIOUS_TOKENS: &[Kind] = &[
    T_ABSTRACT,
    T_FINAL,
    T_PRIVATE,
    T_PROTECTED,
    T_PUBLIC,
    T_STATIC,
    T_ATTRIBUTE,
];

struct VoidReturnFixer {
    fix_lambda: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(VoidReturnFixer { fix_lambda: false })
}

impl Fixer for VoidReturnFixer {
    fn name(&self) -> &'static str {
        "void_return"
    }
    fn priority(&self) -> i32 {
        5
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["fix_lambda"])?;
        self.fix_lambda = c.bool("fix_lambda", true)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_FUNCTION)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_kind(T_FUNCTION) {
                continue;
            }
            if !self.fix_lambda && TokensAnalyzer::new(tokens).is_lambda(index) {
                continue;
            }
            let function_name = tokens.get_next_meaningful_token(index).unwrap();
            // These cause syntax errors.
            if tokens[function_name].equals_any(
                &[
                    (T_STRING, "__clone").into(),
                    (T_STRING, "__construct").into(),
                    (T_STRING, "__debugInfo").into(),
                    (T_STRING, "__destruct").into(),
                    (T_STRING, "__isset").into(),
                    (T_STRING, "__serialize").into(),
                    (T_STRING, "__set_state").into(),
                    (T_STRING, "__sleep").into(),
                    (T_STRING, "__toString").into(),
                ],
                false,
            ) {
                continue;
            }
            let start_index = tokens
                .get_next_token_of_kind(index, &["{".into(), ";".into()], true)
                .unwrap();
            if has_return_type_hint(tokens, start_index) {
                continue;
            }
            if tokens[start_index].equals(";", true) {
                // No function body defined, fallback to PHPDoc.
                if find_return_annotation_types(tokens, index)
                    .iter()
                    .any(|t| t == &[b"void".to_vec()])
                {
                    fix_function_definition(tokens, start_index);
                }
                continue;
            }
            if find_return_annotation_types(tokens, index)
                .iter()
                .any(|t| t != &[b"void".to_vec()])
            {
                continue;
            }
            let end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, start_index);
            if has_void_return(tokens, start_index, end_index) {
                fix_function_definition(tokens, start_index);
            }
        }
    }
}

fn has_return_type_hint(tokens: &Tokens, index: usize) -> bool {
    let end_func_index = tokens
        .get_prev_token_of_kind(index, &[")".into()], true)
        .unwrap();
    let next_index = tokens.get_next_meaningful_token(end_func_index).unwrap();
    tokens[next_index].is_kind(CT_TYPE_COLON)
}

fn has_void_return(tokens: &Tokens, start_index: usize, end_index: usize) -> bool {
    let analyzer = TokensAnalyzer::new(tokens);
    let mut i = start_index;
    while i < end_index {
        // skip anonymous classes and lambda functions
        if (tokens[i].is_kind(T_CLASS) && analyzer.is_anonymous_class(i))
            || (tokens[i].is_kind(T_FUNCTION) && analyzer.is_lambda(i))
        {
            let open = tokens
                .get_next_token_of_kind(i, &["{".into()], true)
                .unwrap();
            i = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, open) + 1;
            continue;
        }
        if tokens[i].is_given_kind(&[T_YIELD, T_YIELD_FROM]) {
            return false; // Generators cannot return void.
        }
        if tokens[i].is_kind(T_RETURN) {
            i = tokens.get_next_meaningful_token(i).unwrap();
            if !tokens[i].equals(";", true) {
                return false;
            }
        }
        i += 1;
    }
    true
}

fn fix_function_definition(tokens: &mut Tokens, index: usize) {
    let end_func_index = tokens
        .get_prev_token_of_kind(index, &[")".into()], true)
        .unwrap();
    tokens.insert_at(
        end_func_index + 1,
        vec![
            Token::new(CT_TYPE_COLON, ":"),
            Token::new(T_WHITESPACE, " "),
            Token::new(T_STRING, "void"),
        ],
    );
}

/// `findReturnAnnotations`, mapped to each annotation's `getTypes()`.
fn find_return_annotation_types(tokens: &Tokens, mut index: usize) -> Vec<Vec<Vec<u8>>> {
    loop {
        index = tokens.get_prev_non_whitespace(index, None).unwrap();
        if tokens[index].is_kind(CT_ATTRIBUTE_CLOSE) {
            index = tokens
                .get_prev_token_of_kind(index, &[T_ATTRIBUTE.into()], true)
                .unwrap();
        }
        if !tokens[index].is_given_kind(PREVIOUS_TOKENS) {
            break;
        }
    }
    if !tokens[index].is_kind(T_DOC_COMMENT) {
        return vec![];
    }
    let doc = DocBlock::new(tokens[index].get_content());
    doc.get_annotations_of_type(&["return"])
        .iter()
        .map(|a| a.get_types())
        .collect()
}
