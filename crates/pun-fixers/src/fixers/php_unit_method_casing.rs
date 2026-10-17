//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitMethodCasingFixer`.
use crate::analyzers::abstract_php_unit_fixer::{
    apply_fix, get_doc_block_index, is_candidate, is_test_attribute_present,
};
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use crate::analyzers::utils::Utils;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;

struct PhpUnitMethodCasingFixer {
    camel_case: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpUnitMethodCasingFixer { camel_case: true })
}

impl Fixer for PhpUnitMethodCasingFixer {
    fn name(&self) -> &'static str {
        "php_unit_method_casing"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["case"])?;
        self.camel_case =
            c.string("case", "camel_case", &["camel_case", "snake_case"])? == "camel_case";
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start, end| {
            self.apply_php_unit_class_fix(tokens, start, end)
        });
    }
}

fn is_method(tokens: &Tokens, index: usize) -> bool {
    tokens[index].is_kind(T_FUNCTION) && !TokensAnalyzer::new(tokens).is_lambda(index)
}

impl PhpUnitMethodCasingFixer {
    fn apply_php_unit_class_fix(&self, tokens: &mut Tokens, start_index: usize, end_index: usize) {
        let mut existing_function_names_lowercase: HashSet<Vec<u8>> = HashSet::new();
        for index in (start_index + 1..end_index).rev() {
            if !is_method(tokens, index) {
                continue;
            }
            let function_name_index = tokens.get_next_meaningful_token(index).unwrap();
            existing_function_names_lowercase
                .insert(tokens[function_name_index].content.to_ascii_lowercase());
        }
        for index in (start_index + 1..end_index).rev() {
            if !self.is_test_method(tokens, index) {
                continue;
            }
            let function_name_index = tokens.get_next_meaningful_token(index).unwrap();
            let function_name = tokens[function_name_index].content.clone();
            let function_name_lowercase = function_name.to_ascii_lowercase();
            let new_function_name = self.update_method_casing(&function_name);
            let new_function_name_lowercase = new_function_name.to_ascii_lowercase();
            if existing_function_names_lowercase.contains(&new_function_name_lowercase)
                && function_name_lowercase != new_function_name_lowercase
            {
                continue;
            }
            existing_function_names_lowercase.insert(new_function_name_lowercase);
            if new_function_name != function_name {
                tokens.set(function_name_index, Token::new(T_STRING, new_function_name));
            }
            let doc_block_index = get_doc_block_index(tokens, index);
            if tokens[doc_block_index].is_kind(T_DOC_COMMENT) {
                self.update_doc_block(tokens, doc_block_index);
            }
        }
    }

    fn update_method_casing(&self, function_name: &[u8]) -> Vec<u8> {
        let mut parts: Vec<&[u8]> = split(function_name, b"::");
        let function_name_part = parts.pop().unwrap();
        let new_part = if self.camel_case {
            // ucwords($s, '_'), str_replace('_', ''), lcfirst
            let mut out = Vec::with_capacity(function_name_part.len());
            let mut upper_next = true;
            for &c in function_name_part {
                out.push(if upper_next {
                    c.to_ascii_uppercase()
                } else {
                    c
                });
                upper_next = c == b'_';
            }
            out.retain(|&c| c != b'_');
            if let Some(first) = out.first_mut() {
                *first = first.to_ascii_lowercase();
            }
            out
        } else {
            Utils::camel_case_to_underscore(&String::from_utf8_lossy(function_name_part))
                .into_bytes()
        };
        let mut parts: Vec<Vec<u8>> = parts.into_iter().map(<[u8]>::to_vec).collect();
        parts.push(new_part);
        parts.join(&b"::"[..])
    }

    fn is_test_method(&self, tokens: &Tokens, index: usize) -> bool {
        // Check if we are dealing with a (non-abstract, non-lambda) function
        if !is_method(tokens, index) {
            return false;
        }
        // if the function name starts with test it's a test
        let function_name_index = tokens.get_next_meaningful_token(index).unwrap();
        if tokens[function_name_index].content.starts_with(b"test") {
            return true;
        }
        if is_test_attribute_present(tokens, index) {
            return true;
        }
        let doc_block_index = get_doc_block_index(tokens, index);
        tokens[doc_block_index].is_kind(T_DOC_COMMENT)
            && tokens[doc_block_index]
                .content
                .windows(5)
                .any(|w| w == b"@test")
    }

    fn update_doc_block(&self, tokens: &mut Tokens, doc_block_index: usize) {
        let doc = DocBlock::new(&tokens[doc_block_index].content);
        let mut lines: Vec<Vec<u8>> = doc
            .get_lines()
            .iter()
            .map(|l| l.borrow().get_content().to_vec())
            .collect();
        let mut doc_block_needs_update = false;
        for line in lines.iter_mut() {
            if !line.windows(8).any(|w| w == b"@depends") {
                continue;
            }
            let new_line_content = Preg::replace_callback(
                "/(@depends\\s+)(.+)(\\b)/",
                |m| [m.str(1), &self.update_method_casing(m.str(2))[..], m.str(3)].concat(),
                &line[..],
            );
            if new_line_content != *line {
                *line = new_line_content;
                doc_block_needs_update = true;
            }
        }
        if doc_block_needs_update {
            tokens.set(doc_block_index, Token::new(T_DOC_COMMENT, lines.concat()));
        }
    }
}

/// PHP `explode($sep, $s)`.
fn split<'a>(s: &'a [u8], sep: &[u8]) -> Vec<&'a [u8]> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i + sep.len() <= s.len() {
        if &s[i..i + sep.len()] == sep {
            out.push(&s[start..i]);
            i += sep.len();
            start = i;
        } else {
            i += 1;
        }
    }
    out.push(&s[start..]);
    out
}
