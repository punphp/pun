//! Port of `PhpCsFixer\Fixer\Phpdoc\GeneralPhpdocAnnotationRemoveFixer` (the configurable rule; the proxy used by
//! phpdoc_no_access / phpdoc_no_package lives in analyzers/general_phpdoc_annotation_remove.rs).
use crate::analyzers::docblock::{Annotation, DocBlock};
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct GeneralPhpdocAnnotationRemoveFixer {
    annotations: Vec<String>,
    case_sensitive: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(GeneralPhpdocAnnotationRemoveFixer {
        annotations: vec![],
        case_sensitive: true,
    })
}

impl GeneralPhpdocAnnotationRemoveFixer {
    fn get_annotations_to_remove<'d>(&self, doc: &'d DocBlock) -> Vec<&'d Annotation> {
        if self.case_sensitive {
            let types: Vec<&str> = self.annotations.iter().map(String::as_str).collect();
            return doc.get_annotations_of_type(&types);
        }
        let types_to_search_for: Vec<Vec<u8>> = self
            .annotations
            .iter()
            .map(|t| strtolower(t.as_bytes()))
            .collect();
        doc.get_annotations()
            .iter()
            .filter(|a| types_to_search_for.contains(&strtolower(a.get_tag().get_name())))
            .collect()
    }
}

impl Fixer for GeneralPhpdocAnnotationRemoveFixer {
    fn name(&self) -> &'static str {
        "general_phpdoc_annotation_remove"
    }
    fn priority(&self) -> i32 {
        10
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["annotations", "case_sensitive"])?;
        self.annotations = c.strings("annotations", &[], &[])?;
        self.case_sensitive = c.bool("case_sensitive", true)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        if self.annotations.is_empty() {
            return;
        }
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            let doc = DocBlock::new(tokens[index].get_content());
            let annotations = self.get_annotations_to_remove(&doc);
            // nothing to do if there are no annotations
            if annotations.is_empty() {
                continue;
            }
            for annotation in annotations {
                annotation.remove();
            }
            let content = doc.get_content();
            if content.is_empty() {
                tokens.clear_token_and_merge_surrounding_whitespace(index);
            } else {
                tokens.set(index, Token::new(T_DOC_COMMENT, content));
            }
        }
    }
}
