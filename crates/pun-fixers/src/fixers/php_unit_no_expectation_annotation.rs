//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitNoExpectationAnnotationFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate, target_fulfills};
use crate::analyzers::docblock::{Annotation, DocBlock};

use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{ltrim_chars, rtrim, trim, var_export_string};
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

struct PhpUnitNoExpectationAnnotationFixer {
    fix_message_reg_exp: bool,
    use_class_const: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitNoExpectationAnnotationFixer {
        fix_message_reg_exp: true,
        use_class_const: true,
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

impl Fixer for PhpUnitNoExpectationAnnotationFixer {
    fn name(&self) -> &'static str {
        "php_unit_no_expectation_annotation"
    }
    fn priority(&self) -> i32 {
        10
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["target", "use_class_const"])?;
        let target = c.string("target", "newest", &["3.2", "4.3", "newest"])?;
        self.use_class_const = c.bool("use_class_const", true)?;
        self.fix_message_reg_exp = target_fulfills(&target, "4.3");
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start_index, end_index| {
            self.apply_php_unit_class_fix(tokens, start_index, end_index);
        });
    }
}

impl PhpUnitNoExpectationAnnotationFixer {
    fn apply_php_unit_class_fix(&self, tokens: &mut Tokens, start_index: usize, end_index: usize) {
        let mut i = end_index - 1;
        while i > start_index {
            if !tokens[i].is_kind(T_FUNCTION) || TokensAnalyzer::new(tokens).is_lambda(i) {
                i -= 1;
                continue;
            }
            let function_index = i;
            let mut doc_block_index = i;
            // ignore abstract functions
            let brace_index = tokens
                .get_next_token_of_kind(function_index, &[";".into(), "{".into()], true)
                .unwrap();
            if !tokens[brace_index].equals("{", true) {
                i -= 1;
                continue;
            }
            loop {
                doc_block_index = tokens
                    .get_prev_non_whitespace(doc_block_index, None)
                    .unwrap();
                if !tokens[doc_block_index].is_given_kind(&[
                    T_PUBLIC,
                    T_PROTECTED,
                    T_PRIVATE,
                    T_FINAL,
                    T_ABSTRACT,
                    T_COMMENT,
                ]) {
                    break;
                }
            }
            if !tokens[doc_block_index].is_kind(T_DOC_COMMENT) {
                i -= 1;
                continue;
            }
            let doc = DocBlock::new(tokens[doc_block_index].get_content());
            let mut annotations: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
            for annotation in doc.get_annotations_of_type(&[
                "expectedException",
                "expectedExceptionCode",
                "expectedExceptionMessage",
                "expectedExceptionMessageRegExp",
            ]) {
                let tag = annotation.get_tag().get_name().to_vec();
                let content = extract_content_from_annotation(annotation);
                annotations.insert(tag, content);
                annotation.remove();
            }
            if !annotations.contains_key(b"expectedException".as_slice())
                || (!self.fix_message_reg_exp
                    && annotations.contains_key(b"expectedExceptionMessageRegExp".as_slice()))
            {
                i -= 1;
                continue;
            }
            let original_indent = WhitespacesAnalyzer::detect_indent(tokens, doc_block_index);
            let param_list = self.annotations_to_param_list(&annotations);
            let mut new_methods_code = b"<?php $this->".to_vec();
            new_methods_code.extend_from_slice(
                if annotations.contains_key(b"expectedExceptionMessageRegExp".as_slice()) {
                    b"setExpectedExceptionRegExp".as_slice()
                } else {
                    b"setExpectedException".as_slice()
                },
            );
            new_methods_code.push(b'(');
            new_methods_code.extend(param_list.join(b", ".as_slice()));
            new_methods_code.extend_from_slice(b");");
            let mut new_methods: Vec<Token> = Tokens::from_code(&new_methods_code)
                .iter()
                .cloned()
                .collect();
            let mut whitespace = self
                .whitespaces_config
                .get_line_ending()
                .as_bytes()
                .to_vec();
            whitespace.extend(&original_indent);
            whitespace.extend(self.whitespaces_config.get_indent().as_bytes());
            new_methods[0] = Token::new(T_WHITESPACE, whitespace);
            // apply changes
            let mut doc_content = doc.get_content();
            if doc_content.is_empty() {
                doc_content = b"/** */".to_vec();
            }
            tokens.set(doc_block_index, Token::new(T_DOC_COMMENT, doc_content));
            let size = new_methods.len();
            tokens.insert_at(brace_index + 1, new_methods);
            let whitespace_index = brace_index + size + 1;
            let mut content = self
                .whitespaces_config
                .get_line_ending()
                .as_bytes()
                .to_vec();
            content.extend(tokens[whitespace_index].get_content());
            tokens.set(whitespace_index, Token::new(T_WHITESPACE, content));
            i = doc_block_index - 1;
        }
    }

    /// `annotationsToParamList`.
    fn annotations_to_param_list(&self, annotations: &HashMap<Vec<u8>, Vec<u8>>) -> Vec<Vec<u8>> {
        let mut params = Vec::new();
        let expected = &annotations[b"expectedException".as_slice()];
        let mut exception_class = ltrim_chars(expected, b"\\");
        if let Some(star) = exception_class.iter().position(|&b| b == b'*') {
            exception_class = &exception_class[..star];
        }
        let exception_class = trim(exception_class);
        if self.use_class_const {
            params.push([b"\\", exception_class, b"::class"].concat());
        } else {
            params.push([b"'", exception_class, b"'"].concat());
        }
        let get = |k: &str| annotations.get(k.as_bytes());
        if let Some(message) = get("expectedExceptionMessage") {
            params.push(var_export_string(message));
        } else if let Some(message) = get("expectedExceptionMessageRegExp") {
            params.push(var_export_string(message));
        } else if get("expectedExceptionCode").is_some() {
            params.push(b"null".to_vec());
        }
        if let Some(code) = get("expectedExceptionCode") {
            params.push(code.clone());
        }
        params
    }
}

/// `extractContentFromAnnotation`.
fn extract_content_from_annotation(annotation: &Annotation) -> Vec<u8> {
    let tag = annotation.get_tag().get_name().to_vec();
    let annotation_content = annotation.get_content();
    let pattern = [b"/@".as_slice(), &tag, br"\s+(.+)$/s"].concat();
    let Some(matches) = Preg::match_groups(&pattern, &annotation_content) else {
        return Vec::new();
    };
    let mut content = Preg::replace(r"/\*+\/$/", "", matches.str(1));
    if Preg::r#match(r"/\R/u", &content) {
        content = Preg::replace(r"/\s*\R+\s*\*\s*/u", " ", &content);
    }
    rtrim(&content).to_vec()
}
