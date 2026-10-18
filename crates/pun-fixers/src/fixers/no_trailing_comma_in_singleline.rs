//! Port of `PhpCsFixer\Fixer\Basic\NoTrailingCommaInSinglelineFixer`.
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use serde_json::Value;
use std::path::Path;

const ELEMENTS: &[&str] = &["arguments", "array", "array_destructuring", "group_import"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Element {
    Arguments,
    Array,
    ArrayDestructuring,
    GroupImport,
}

/// ELEMENTS' variants, in the same order.
const CHOICES: &[(&str, Element)] = &[
    ("arguments", Element::Arguments),
    ("array", Element::Array),
    ("array_destructuring", Element::ArrayDestructuring),
    ("group_import", Element::GroupImport),
];

struct NoTrailingCommaInSinglelineFixer {
    elements: Vec<Element>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(NoTrailingCommaInSinglelineFixer {
        elements: CHOICES.iter().map(|&(_, e)| e).collect(),
    })
}

impl Fixer for NoTrailingCommaInSinglelineFixer {
    fn name(&self) -> &'static str {
        "no_trailing_comma_in_singleline"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(",")
            && (tokens.is_token_kind_found(")")
                || tokens.is_any_token_kinds_found(&[
                    CT_ARRAY_BRACKET_CLOSE,
                    CT_DESTRUCTURING_BRACKET_CLOSE,
                    CT_GROUP_IMPORT_BRACE_CLOSE,
                ]))
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["elements"])?;
        self.elements = o.choices("elements", ELEMENTS, CHOICES)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].equals(")", true)
                && !tokens[index].is_given_kind(&[
                    CT_ARRAY_BRACKET_CLOSE,
                    CT_DESTRUCTURING_BRACKET_CLOSE,
                    CT_GROUP_IMPORT_BRACE_CLOSE,
                ])
            {
                continue;
            }
            let mut comma_index = tokens.get_prev_meaningful_token(index).unwrap();
            if !tokens[comma_index].equals(",", true) {
                continue;
            }
            let (block_type, _) = Tokens::detect_block_type(&tokens[index]).unwrap();
            let block_open_index = tokens.find_block_start(block_type, index);
            if tokens.is_partial_code_multiline(block_open_index, index) {
                continue;
            }
            if !self.should_be_cleared(tokens, block_open_index) {
                continue;
            }
            loop {
                tokens.clear_token_and_merge_surrounding_whitespace(comma_index);
                comma_index = tokens.get_prev_meaningful_token(comma_index).unwrap();
                if !tokens[comma_index].equals(",", true) {
                    break;
                }
            }
            tokens.remove_trailing_whitespace(comma_index, None);
        }
    }
}

impl NoTrailingCommaInSinglelineFixer {
    fn has(&self, e: Element) -> bool {
        self.elements.contains(&e)
    }

    fn should_be_cleared(&self, tokens: &Tokens, open_index: usize) -> bool {
        if tokens[open_index].is_kind(CT_ARRAY_BRACKET_OPEN) {
            return self.has(Element::Array);
        }
        if tokens[open_index].is_kind(CT_DESTRUCTURING_BRACKET_OPEN) {
            return self.has(Element::ArrayDestructuring);
        }
        if tokens[open_index].is_kind(CT_GROUP_IMPORT_BRACE_OPEN) {
            return self.has(Element::GroupImport);
        }
        let before_open = tokens.get_prev_meaningful_token(open_index).unwrap();
        if tokens[before_open].is_kind(T_ARRAY) {
            return self.has(Element::Array);
        }
        if tokens[before_open].is_kind(T_LIST) {
            return self.has(Element::ArrayDestructuring);
        }
        if tokens[before_open].is_given_kind(&[T_UNSET, T_ISSET, T_VARIABLE, T_CLASS]) {
            return self.has(Element::Arguments);
        }
        if tokens[before_open].is_kind(T_STRING) {
            return !AttributeAnalyzer::is_attribute(tokens, before_open) && self.has(Element::Arguments);
        }
        if tokens[before_open].equals_any(
            &[
                ")".into(),
                "]".into(),
                CT_DYNAMIC_VAR_BRACE_CLOSE.into(),
                CT_ARRAY_INDEX_BRACE_CLOSE.into(),
            ],
            true,
        ) {
            let (block_type, _) = Tokens::detect_block_type(&tokens[before_open]).unwrap();
            return (block_type == Tokens::BLOCK_TYPE_INDEX_BRACE
                || block_type == Tokens::BLOCK_TYPE_DYNAMIC_VAR_BRACE
                || block_type == Tokens::BLOCK_TYPE_INDEX_BRACKET
                || block_type == Tokens::BLOCK_TYPE_PARENTHESIS)
                && self.has(Element::Arguments);
        }
        false
    }
}
