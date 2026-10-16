//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocReturnSelfReferenceFixer`.
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::token::CLASSY_KINDS;
use pun_tokens::tokens_analyzer::ClassyElementType;
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

const TO_TYPES: [&str; 3] = ["$this", "static", "self"];
const DEFAULT: [(&str, &str); 6] =
    [("this", "$this"), ("@this", "$this"), ("$self", "self"), ("@self", "self"), ("$static", "static"), ("@static", "static")];

struct PhpdocReturnSelfReferenceFixer {
    replacements: Vec<(Vec<u8>, Vec<u8>)>,
}

pub fn fixer() -> Box<dyn Fixer> {
    let replacements = DEFAULT.iter().map(|(f, t)| (f.as_bytes().to_vec(), t.as_bytes().to_vec())).collect();
    Box::new(PhpdocReturnSelfReferenceFixer { replacements })
}

/// `Utils::naturalLanguageJoin`: `"a", "b" and "c"`.
fn join(items: &[&str]) -> String {
    let quoted: Vec<String> = items.iter().map(|s| format!("\"{s}\"")).collect();
    let (last, rest) = quoted.split_last().unwrap();
    format!("{} and {last}", rest.join(", "))
}

impl Fixer for PhpdocReturnSelfReferenceFixer {
    fn name(&self) -> &'static str {
        "phpdoc_return_self_reference"
    }
    fn priority(&self) -> i32 {
        10
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let c = resolve(self.name(), options, &["replacements"])?;
        c.check_types("replacements", &["string[]"])?;
        let entries: Vec<(String, &str)> = match c.get("replacements") {
            None => return Ok(()),
            Some(Value::Object(map)) => map.iter().map(|(k, v)| (format!("string#{}", k.to_ascii_lowercase()), v.as_str().unwrap())).collect(),
            Some(v) => v.as_array().unwrap().iter().enumerate().map(|(i, v)| (format!("integer#{i}"), v.as_str().unwrap())).collect(),
        };
        let defaults: Vec<&str> = DEFAULT.iter().map(|(f, _)| *f).collect();
        let mut replacements: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        for (from, to) in entries {
            let Some(key) = from.strip_prefix("string#").filter(|k| defaults.contains(k)) else {
                bail!("[{}] Invalid configuration: Unknown key \"{from}\", expected any of {}.", self.name(), join(&defaults));
            };
            if !TO_TYPES.contains(&to) {
                bail!("[{}] Invalid configuration: Unknown value \"string#{to}\", expected any of {}.", self.name(), join(&TO_TYPES));
            }
            replacements.retain(|(f, _)| f != key.as_bytes());
            replacements.push((key.as_bytes().to_vec(), to.as_bytes().to_vec()));
        }
        self.replacements = replacements;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.count() > 10 && tokens.is_all_token_kinds_found(&[T_DOC_COMMENT, T_FUNCTION]) && tokens.is_any_token_kinds_found(CLASSY_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let methods: Vec<usize> = TokensAnalyzer::new(tokens)
            .get_classy_elements()
            .into_iter()
            .filter(|(_, e)| e.kind == ClassyElementType::Method)
            .map(|(i, _)| i)
            .collect();
        for index in methods {
            self.fix_method(tokens, index);
        }
    }
}

impl PhpdocReturnSelfReferenceFixer {
    fn fix_method(&self, tokens: &mut Tokens, mut index: usize) {
        // find PHPDoc of method (if any)
        loop {
            let token_index = tokens.get_prev_meaningful_token(index).unwrap();
            if !tokens[token_index].is_given_kind(&[T_STATIC, T_FINAL, T_ABSTRACT, T_PRIVATE, T_PROTECTED, T_PUBLIC]) {
                break;
            }
            index = token_index;
        }
        let doc_index = tokens.get_prev_non_whitespace(index, None).unwrap();
        if !tokens[doc_index].is_kind(T_DOC_COMMENT) {
            return;
        }
        let doc_block = DocBlock::new(&tokens[doc_index].content);
        let Some(returns_block) = doc_block.get_annotations_of_type(&["return"]).into_iter().next() else { return };
        let types = returns_block.get_types();
        if types.is_empty() {
            return;
        }
        let new_types: Vec<Vec<u8>> = types
            .iter()
            .map(|t| {
                let lower = strtolower(t);
                self.replacements.iter().find(|(f, _)| *f == lower).map_or_else(|| t.clone(), |(_, to)| to.clone())
            })
            .collect();
        if types == new_types {
            return;
        }
        returns_block.set_types(&new_types);
        tokens.set(doc_index, Token::new(T_DOC_COMMENT, doc_block.get_content()));
    }
}
