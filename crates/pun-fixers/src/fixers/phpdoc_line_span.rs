//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocLineSpanFixer`.
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::{self, *};
use pun_tokens::tokens_analyzer::ClassyElementType;
use pun_tokens::{BlockType, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;

const PROPERTY_PART_KINDS: &[Kind] = &[
    T_PUBLIC,
    T_PROTECTED,
    T_PRIVATE,
    T_FINAL,
    T_ABSTRACT,
    T_COMMENT,
    T_VAR,
    T_STATIC,
    T_STRING,
    T_NS_SEPARATOR,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
    CT_ARRAY_TYPEHINT,
    CT_NULLABLE_TYPE,
    CT_TYPE_ALTERNATION,
    CT_TYPE_INTERSECTION,
    T_ATTRIBUTE,
    T_READONLY,
    T_PRIVATE_SET,
    T_PROTECTED_SET,
    T_PUBLIC_SET,
];

/// The option names, in upstream's order, with their defaults (`Some(true)` = multi, `Some(false)` = single).
const OPTIONS: [(&str, Option<bool>); 8] = [
    ("class", Some(true)),
    ("trait_import", None),
    ("const", Some(true)),
    ("property", Some(true)),
    ("method", Some(true)),
    ("case", Some(true)),
    ("function", Some(true)),
    ("other", None),
];

struct PhpdocLineSpanFixer {
    /// The configured span per `OPTIONS` entry.
    spans: [Option<bool>; 8],
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocLineSpanFixer {
        spans: OPTIONS.map(|(_, d)| d),
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

impl Fixer for PhpdocLineSpanFixer {
    fn name(&self) -> &'static str {
        "phpdoc_line_span"
    }
    fn priority(&self) -> i32 {
        5
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &OPTIONS.map(|(n, _)| n))?;
        for (i, (name, default)) in OPTIONS.into_iter().enumerate() {
            self.spans[i] = match o.get(name) {
                None => default,
                Some(Value::Null) => None,
                Some(Value::String(s)) if s == "multi" => Some(true),
                Some(Value::String(s)) if s == "single" => Some(false),
                Some(v) => return Err(o.invalid_or_null(name, v, &["single", "multi"])),
            };
        }
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let elements = TokensAnalyzer::new(tokens).get_classy_elements();
        let mut handled = HashSet::new();
        for (&index, element) in &elements {
            let kind = match element.kind {
                ClassyElementType::PromotedProperty => ClassyElementType::Property,
                k => k,
            };
            let Some(doc_index) = get_doc_block_index(tokens, index) else {
                continue;
            };
            handled.insert(doc_index);
            self.fix_doc_block(tokens, doc_index, kind.as_str());
        }

        if self.span("class").is_none()
            && self.span("function").is_none()
            && self.span("other").is_none()
        {
            return;
        }

        let mut index = tokens.count() - 1;
        while index > 0 {
            if handled.contains(&index) {
                index -= 1;
                continue;
            }
            if tokens[index].is_kind(T_FUNCTION) && !TokensAnalyzer::new(tokens).is_lambda(index) {
                if let Some(doc_index) = get_doc_block_index(tokens, index)
                    && handled.insert(doc_index)
                {
                    self.fix_doc_block(tokens, doc_index, "function");
                }
            } else if tokens[index].is_kind(T_DOC_COMMENT) {
                self.fix_doc_block(tokens, index, "other");
            } else if tokens[index].is_classy()
                && let Some(doc_index) = get_doc_block_index(tokens, index)
            {
                handled.insert(doc_index);
                self.fix_doc_block(tokens, doc_index, "class");
            }
            index -= 1;
        }
    }
}

fn get_doc_block_index(tokens: &Tokens, mut index: usize) -> Option<usize> {
    loop {
        index = tokens.get_prev_non_whitespace(index, None)?;
        if tokens[index].is_kind(CT_ATTRIBUTE_CLOSE) {
            index = tokens.find_block_start(BlockType::Attribute, index);
        }
        if !tokens[index].is_given_kind(PROPERTY_PART_KINDS) {
            break;
        }
    }
    tokens[index].is_kind(T_DOC_COMMENT).then_some(index)
}

impl PhpdocLineSpanFixer {
    fn span(&self, name: &str) -> Option<bool> {
        let i = OPTIONS.iter().position(|(n, _)| *n == name).unwrap();
        self.spans[i]
    }

    fn fix_doc_block(&self, tokens: &mut Tokens, doc_index: usize, kind: &str) {
        let Some(multi) = self.span(kind) else {
            return;
        };
        let mut doc = DocBlock::new(tokens[doc_index].get_content());
        if multi {
            let indent = WhitespacesAnalyzer::detect_indent(tokens, doc_index);
            doc.make_multi_line(
                &indent,
                self.whitespaces_config.get_line_ending().as_bytes(),
            );
        } else {
            doc.make_single_line();
        }
        tokens.set(doc_index, Token::new(T_DOC_COMMENT, doc.get_content()));
    }
}
