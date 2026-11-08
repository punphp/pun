//! Port of `PhpCsFixer\Fixer\LanguageConstruct\FunctionToConstantFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

/// `$availableFunctions`' keys, in upstream's order.
const AVAILABLE: &[&str] = &[
    "get_called_class",
    "get_class",
    "get_class_this",
    "php_sapi_name",
    "phpversion",
    "pi",
];

struct FunctionToConstantFixer {
    functions: Vec<String>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(FunctionToConstantFixer { functions: vec![] })
}

/// `$availableFunctions[$name]`: the tokens that replace the call.
fn replacement(name: &str) -> Vec<Token> {
    let class = |first: Token| {
        vec![
            first,
            Token::new(T_DOUBLE_COLON, "::"),
            Token::new(CT_CLASS_CONSTANT, "class"),
        ]
    };
    match name {
        "get_called_class" | "get_class_this" => class(Token::new(T_STATIC, "static")),
        "get_class" => class(Token::new(T_STRING, "self")),
        "php_sapi_name" => vec![Token::new(T_STRING, "PHP_SAPI")],
        "phpversion" => vec![Token::new(T_STRING, "PHP_VERSION")],
        _ => vec![Token::new(T_STRING, "M_PI")],
    }
}

impl FunctionToConstantFixer {
    fn has(&self, name: &str) -> bool {
        self.functions.iter().any(|f| f == name)
    }

    fn get_replace_candidate(
        &self,
        tokens: &Tokens,
        analyzer: &FunctionsAnalyzer,
        index: usize,
    ) -> Option<(usize, usize, Vec<Token>)> {
        if !tokens[index].is_kind(T_STRING) {
            return None;
        }
        let lower = String::from_utf8_lossy(&strtolower(tokens[index].get_content())).into_owned();
        if lower == "get_class" {
            return self.fix_get_class_call(tokens, analyzer, index);
        }
        if !self.has(&lower) {
            return None;
        }
        if !analyzer.is_global_function_call(tokens, index) {
            return None;
        }
        // test if function call without parameters
        let brace_open_index = tokens.get_next_meaningful_token(index).unwrap();
        let brace_close_index = tokens.get_next_meaningful_token(brace_open_index).unwrap();
        if !tokens[brace_close_index].equals(")", true) {
            return None;
        }
        Some((brace_open_index, brace_close_index, replacement(&lower)))
    }

    fn fix_get_class_call(
        &self,
        tokens: &Tokens,
        analyzer: &FunctionsAnalyzer,
        index: usize,
    ) -> Option<(usize, usize, Vec<Token>)> {
        if !self.has("get_class") && !self.has("get_class_this") {
            return None;
        }
        if !analyzer.is_global_function_call(tokens, index) {
            return None;
        }
        let brace_open_index = tokens.get_next_meaningful_token(index).unwrap();
        let brace_close_index =
            tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, brace_open_index);
        if Some(brace_close_index) == tokens.get_next_meaningful_token(brace_open_index) {
            // no arguments passed
            if self.has("get_class") {
                return Some((
                    brace_open_index,
                    brace_close_index,
                    replacement("get_class"),
                ));
            }
        } else if self.has("get_class_this") {
            let mut is_this = false;
            for i in brace_open_index + 1..brace_close_index {
                if tokens[i].equals_any(
                    &[
                        T_WHITESPACE.into(),
                        T_COMMENT.into(),
                        T_DOC_COMMENT.into(),
                        ")".into(),
                    ],
                    true,
                ) {
                    continue;
                }
                if tokens[i].is_kind(T_VARIABLE) && strtolower(tokens[i].get_content()) == b"$this"
                {
                    is_this = true;
                    continue;
                }
                if !is_this && tokens[i].equals("(", true) {
                    continue;
                }
                is_this = false;
                break;
            }
            if is_this {
                return Some((
                    brace_open_index,
                    brace_close_index,
                    replacement("get_class_this"),
                ));
            }
        }
        None
    }
}

fn fix_function_call_to_constant(
    tokens: &mut Tokens,
    index: usize,
    brace_open_index: usize,
    brace_close_index: usize,
    replacements: Vec<Token>,
) {
    for i in (brace_open_index..=brace_close_index).rev() {
        if tokens[i].is_given_kind(&[T_WHITESPACE, T_COMMENT, T_DOC_COMMENT]) {
            continue;
        }
        tokens.clear_token_and_merge_surrounding_whitespace(i);
    }
    // `$replacements[0]` is never `T_CLASS_C`, so upstream's check comes down to `static` and `self`.
    if replacements[0].is_kind(T_STATIC)
        || (replacements[0].is_kind(T_STRING) && replacements[0].get_content() == b"self")
    {
        let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[prev_index].is_kind(T_NS_SEPARATOR) {
            tokens.clear_at(prev_index);
        }
    }
    tokens.clear_at(index);
    tokens.insert_at(index, replacements);
}

impl Fixer for FunctionToConstantFixer {
    fn name(&self) -> &'static str {
        "function_to_constant"
    }
    fn priority(&self) -> i32 {
        2
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["functions"])?;
        self.functions = c.strings("functions", AVAILABLE, AVAILABLE)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let analyzer = FunctionsAnalyzer::new();
        let mut index = tokens.count() as isize - 4;
        while index > 0 {
            let i = index as usize;
            if let Some((open, close, replacements)) =
                self.get_replace_candidate(tokens, &analyzer, i)
            {
                fix_function_call_to_constant(tokens, i, open, close, replacements);
            }
            index -= 1;
        }
    }
}
