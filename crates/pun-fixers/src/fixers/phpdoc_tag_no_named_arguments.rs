//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocTagNoNamedArgumentsFixer`, with the parts of `DocBlockAnnotationTrait` it
//! uses (`ensureIsDocBlockWithAnnotation` without preventing attributes).
use crate::analyzers::abstract_php_unit_fixer::get_doc_block_index;
use crate::analyzers::analysis::NamespaceUseType;
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::fully_qualified_name_analyzer::FullyQualifiedNameAnalyzer;
use crate::analyzers::preg::Preg;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct PhpdocTagNoNamedArgumentsFixer {
    description: String,
    fix_attribute: bool,
    fix_internal: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocTagNoNamedArgumentsFixer::default())
}

impl Fixer for PhpdocTagNoNamedArgumentsFixer {
    fn name(&self) -> &'static str {
        "phpdoc_tag_no_named_arguments"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_CLASS, T_TRAIT, T_INTERFACE, T_ENUM])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &["description", "fix_attribute", "fix_internal"],
        )?;
        self.description = o.string("description", "", &[])?;
        self.fix_attribute = o.bool("fix_attribute", true)?;
        self.fix_internal = o.bool("fix_internal", true)?;
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let mut index = tokens.count() - 1;
        while index > 0 {
            if !tokens[index].is_classy() {
                index -= 1;
                continue;
            }
            let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[prev_index].is_kind(T_NEW) {
                index -= 1;
                continue;
            }
            if !self.fix_attribute && is_attribute_class(tokens, prev_index) {
                index -= 1;
                continue;
            }
            let preventing: &[&str] = if self.fix_internal {
                &["no-named-arguments"]
            } else {
                &["internal", "no-named-arguments"]
            };
            ensure_is_doc_block_with_annotation(
                tokens,
                index,
                "no-named-arguments",
                preventing,
                line_ending,
            );
            let doc_block_index = tokens
                .get_prev_token_of_kind(index + 2, &[T_DOC_COMMENT.into()], true)
                .unwrap();
            let content = tokens[doc_block_index].get_content().to_vec();
            let tag = format!("@no-named-arguments {}", self.description);
            let replacement = [tag.trim_end().as_bytes(), line_ending].concat();
            let new_content = Preg::replace(r"/@no-named-arguments.*\R/", &replacement, &content);
            if new_content != content {
                tokens.set(doc_block_index, Token::new(T_DOC_COMMENT, new_content));
            }
            index -= 1;
        }
    }
}

fn is_attribute_class(tokens: &Tokens, mut index: usize) -> bool {
    while tokens[index].is_given_kind(&[T_FINAL, T_READONLY]) {
        index = tokens.get_prev_meaningful_token(index).unwrap();
    }
    if !tokens[index].is_kind(CT_ATTRIBUTE_CLOSE) {
        return false;
    }
    let start = tokens.find_block_start(Tokens::BLOCK_TYPE_ATTRIBUTE, index);
    let fqn = FullyQualifiedNameAnalyzer::new(tokens);
    for attribute_analysis in AttributeAnalyzer::collect(tokens, start) {
        for attribute in &attribute_analysis.attributes {
            let name = fqn.get_fully_qualified_name(
                &attribute.name,
                attribute.start,
                NamespaceUseType::Class,
            );
            if strtolower(&name) == b"attribute" {
                return true;
            }
        }
    }
    false
}

/// `DocBlockAnnotationTrait::ensureIsDocBlockWithAnnotation` (no preventing attributes).
fn ensure_is_doc_block_with_annotation(
    tokens: &mut Tokens,
    index: usize,
    annotation: &str,
    preventing_annotations: &[&str],
    line_end: &[u8],
) {
    let doc_block_index = get_doc_block_index(tokens, index);
    if tokens[doc_block_index].is_kind(T_DOC_COMMENT) {
        update_doc_block_if_needed(
            tokens,
            doc_block_index,
            annotation,
            preventing_annotations,
            line_end,
        );
    } else {
        create_doc_block(tokens, doc_block_index, annotation, line_end);
    }
}

fn create_doc_block(
    tokens: &mut Tokens,
    doc_block_index: usize,
    annotation: &str,
    line_end: &[u8],
) {
    let next = tokens
        .get_next_non_whitespace(doc_block_index, None)
        .unwrap();
    let original_indent = WhitespacesAnalyzer::detect_indent(tokens, next);
    let doc = [
        b"/**",
        line_end,
        &original_indent,
        b" * @",
        annotation.as_bytes(),
        line_end,
        &original_indent,
        b" */",
    ]
    .concat();
    let index = tokens.get_next_meaningful_token(doc_block_index).unwrap();
    tokens.insert_at(
        index,
        vec![
            Token::new(T_DOC_COMMENT, doc),
            Token::new(T_WHITESPACE, [line_end, &original_indent].concat()),
        ],
    );
    if !tokens[index - 1].is_kind(T_WHITESPACE) {
        let mut extra_new_lines = line_end.to_vec();
        if !tokens[index - 1].is_kind(T_OPEN_TAG) {
            extra_new_lines.extend_from_slice(line_end);
        }
        extra_new_lines.extend(WhitespacesAnalyzer::detect_indent(tokens, index));
        tokens.insert_at(index, vec![Token::new(T_WHITESPACE, extra_new_lines)]);
    }
}

fn update_doc_block_if_needed(
    tokens: &mut Tokens,
    doc_block_index: usize,
    annotation: &str,
    preventing_annotations: &[&str],
    line_end: &[u8],
) {
    let mut doc = DocBlock::new(tokens[doc_block_index].get_content());
    for preventing in preventing_annotations {
        if !doc.get_annotations_of_type(&[*preventing]).is_empty() {
            return;
        }
    }
    // makeDocBlockMultiLineIfNeeded
    if doc.get_lines().len() == 1 && doc.get_annotations_of_type(&[annotation]).is_empty() {
        let next = tokens
            .get_next_non_whitespace(doc_block_index, None)
            .unwrap();
        let indent = WhitespacesAnalyzer::detect_indent(tokens, next);
        doc.make_multi_line(&indent, line_end);
    }
    // addAnnotation
    let original_indent = WhitespacesAnalyzer::detect_indent(tokens, doc_block_index);
    let mut lines: Vec<Vec<u8>> = doc
        .get_lines()
        .iter()
        .map(|l| l.borrow().get_content().to_vec())
        .collect();
    let at = lines.len() - 1;
    lines.insert(
        at,
        [
            &original_indent[..],
            b" * @",
            annotation.as_bytes(),
            line_end,
        ]
        .concat(),
    );
    tokens.set(doc_block_index, Token::new(T_DOC_COMMENT, lines.concat()));
}
