//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocReturnSelfReferenceFixer`.
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::{PhpKey, resolve};
use crate::analyzers::utils::natural_language_join;
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
const DEFAULT: [(&str, &str); 6] = [
    ("this", "$this"),
    ("@this", "$this"),
    ("$self", "self"),
    ("@self", "self"),
    ("$static", "static"),
    ("@static", "static"),
];

#[derive(Default)]
struct PhpdocReturnSelfReferenceFixer {
    replacements: Vec<(Vec<u8>, Vec<u8>)>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocReturnSelfReferenceFixer::default())
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
        let Some(entries) = c.php_array("replacements") else {
            self.replacements = DEFAULT
                .iter()
                .map(|(f, t)| (f.as_bytes().to_vec(), t.as_bytes().to_vec()))
                .collect();
            return Ok(());
        };
        let defaults: Vec<&str> = DEFAULT.iter().map(|(f, _)| *f).collect();
        let mut replacements: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        for (from, to) in entries {
            let to = to.as_str().unwrap();
            // the normalizer strtolower()s string keys
            let from = match from {
                PhpKey::Str(k) => format!("string#{}", k.to_ascii_lowercase()),
                k => k.typed(),
            };
            let Some(key) = from
                .strip_prefix("string#")
                .filter(|k| defaults.contains(k))
            else {
                bail!(
                    "[{}] Invalid configuration: Unknown key \"{from}\", expected any of {}.",
                    self.name(),
                    natural_language_join(&defaults)
                );
            };
            if !TO_TYPES.contains(&to) {
                bail!(
                    "[{}] Invalid configuration: Unknown value \"string#{to}\", expected any of {}.",
                    self.name(),
                    natural_language_join(TO_TYPES)
                );
            }
            replacements.retain(|(f, _)| f != key.as_bytes());
            replacements.push((key.as_bytes().to_vec(), to.as_bytes().to_vec()));
        }
        self.replacements = replacements;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.count() > 10
            && tokens.is_all_token_kinds_found(&[T_DOC_COMMENT, T_FUNCTION])
            && tokens.is_any_token_kinds_found(CLASSY_KINDS)
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
        loop {
            let token_index = tokens.get_prev_meaningful_token(index).unwrap();
            if !tokens[token_index].is_given_kind(&[
                T_STATIC,
                T_FINAL,
                T_ABSTRACT,
                T_PRIVATE,
                T_PROTECTED,
                T_PUBLIC,
            ]) {
                break;
            }
            index = token_index;
        }
        let doc_index = tokens.get_prev_non_whitespace(index, None).unwrap();
        if !tokens[doc_index].is_kind(T_DOC_COMMENT) {
            return;
        }
        let doc_block = DocBlock::new(&tokens[doc_index].content);
        let Some(returns_block) = doc_block
            .get_annotations_of_type(&["return"])
            .into_iter()
            .next()
        else {
            return;
        };
        let types = returns_block.get_types();
        if types.is_empty() {
            return;
        }
        let new_types: Vec<Vec<u8>> = types
            .iter()
            .map(|t| {
                let lower = strtolower(t);
                self.replacements
                    .iter()
                    .find(|(f, _)| *f == lower)
                    .map_or_else(|| t.clone(), |(_, to)| to.clone())
            })
            .collect();
        if types == new_types {
            return;
        }
        returns_block.set_types(&new_types);
        tokens.set(
            doc_index,
            Token::new(T_DOC_COMMENT, doc_block.get_content()),
        );
    }
}
