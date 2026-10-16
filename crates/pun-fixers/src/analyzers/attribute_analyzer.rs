//! Port of `PhpCsFixer\Tokenizer\Analyzer\AttributeAnalyzer`.
use crate::analyzers::analysis::{Attribute, AttributeAnalysis};
use crate::analyzers::preg::Preg;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Tokens};

pub struct AttributeAnalyzer;

const TOKEN_KINDS_NOT_ALLOWED_IN_ATTRIBUTE: &[Proto<'static>] = &[
    Proto::Char(b";"),
    Proto::Char(b"{"),
    Proto::Kind(T_ATTRIBUTE),
    Proto::Kind(T_FUNCTION),
    Proto::Kind(T_OPEN_TAG),
    Proto::Kind(T_OPEN_TAG_WITH_ECHO),
    Proto::Kind(T_PRIVATE),
    Proto::Kind(T_PROTECTED),
    Proto::Kind(T_PUBLIC),
    Proto::Kind(T_RETURN),
    Proto::Kind(T_VARIABLE),
    Proto::Kind(CT_ATTRIBUTE_CLOSE),
];

impl AttributeAnalyzer {
    pub fn is_attribute(tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].is_kind(T_STRING) || !tokens.is_any_token_kinds_found(&[T_ATTRIBUTE]) {
            return false;
        }
        // T_OPEN_TAG is in the list, so a T_STRING always finds one.
        let attribute_start_index = tokens.get_prev_token_of_kind(index, TOKEN_KINDS_NOT_ALLOWED_IN_ATTRIBUTE, true).unwrap();
        if !tokens[attribute_start_index].is_kind(T_ATTRIBUTE) {
            return false;
        }
        // now, between attribute start and the attribute candidate index cannot be more "(" than ")"
        let mut count = 0i32;
        for i in attribute_start_index + 1..index {
            if tokens[i].equals("(", true) {
                count += 1;
            } else if tokens[i].equals(")", true) {
                count -= 1;
            }
        }
        count == 0
    }

    pub fn collect(tokens: &Tokens, mut index: usize) -> Vec<AttributeAnalysis> {
        assert!(tokens[index].is_kind(T_ATTRIBUTE), "Given index must point to an attribute.");
        // Rewind to first attribute in group
        loop {
            let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            if !tokens[prev_index].is_kind(CT_ATTRIBUTE_CLOSE) {
                break;
            }
            index = tokens.find_block_start(Tokens::BLOCK_TYPE_ATTRIBUTE, prev_index);
        }
        let mut elements = Vec::new();
        let mut opening_index = index;
        loop {
            let element = Self::collect_one(tokens, opening_index);
            opening_index = tokens.get_next_meaningful_token(element.end_index).unwrap();
            elements.push(element);
            if !tokens[opening_index].is_kind(T_ATTRIBUTE) {
                break;
            }
        }
        elements
    }

    pub fn collect_one(tokens: &Tokens, index: usize) -> AttributeAnalysis {
        assert!(tokens[index].is_kind(T_ATTRIBUTE), "Given index must point to an attribute.");
        let mut start_index = index;
        let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[prev_index].is_kind(CT_ATTRIBUTE_CLOSE) {
            // Include comments/PHPDoc if they are present
            start_index = tokens.get_next_non_whitespace(prev_index, None).unwrap();
        }
        let closing_index = tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, index);
        let end_index = tokens.get_next_non_whitespace(closing_index, None).unwrap();
        AttributeAnalysis {
            start_index,
            end_index: end_index - 1,
            opening_bracket_index: index,
            closing_bracket_index: closing_index,
            attributes: Self::collect_attributes(tokens, index, closing_index),
        }
    }

    fn collect_attributes(tokens: &Tokens, mut index: usize, closing_index: usize) -> Vec<Attribute> {
        let mut elements = Vec::new();
        loop {
            let attribute_start_index = index + 1;
            let name_start_index = tokens.get_next_token_of_kind(index, &[T_STRING.into(), T_NS_SEPARATOR.into()], true).unwrap();
            index = tokens
                .get_next_token_of_kind(attribute_start_index, &["(".into(), ",".into(), CT_ATTRIBUTE_CLOSE.into()], true)
                .unwrap();
            let attribute_name = tokens.generate_partial_code(name_start_index, tokens.get_prev_meaningful_token(index).unwrap());
            // Find closing parentheses, we need to do this in case there's a comma inside the parentheses
            if tokens[index].equals("(", true) {
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
                index = tokens.get_next_token_of_kind(index, &[",".into(), CT_ATTRIBUTE_CLOSE.into()], true).unwrap();
            }
            elements.push(Attribute { start: attribute_start_index, end: index - 1, name: attribute_name });
            let mut next_index = index;
            // In case there's a comma right before T_ATTRIBUTE_CLOSE
            if next_index < closing_index {
                next_index = tokens.get_next_meaningful_token(index).unwrap();
            }
            if next_index >= closing_index {
                break;
            }
        }
        // End last element at newline if it exists and there's no trailing comma
        index -= 1;
        while tokens[index].is_whitespace(None) {
            if Preg::r#match("/\\R/", &tokens[index].content) {
                elements.last_mut().unwrap().end = index - 1;
                break;
            }
            index -= 1;
        }
        elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected values: PhpCsFixer\Tokenizer\Analyzer\AttributeAnalyzer::collect($tokens, 1) on the same code
    /// (the trailing ` ]` without a newline, and a last element ended at the line break).
    #[test]
    fn collect_matches_oracle() {
        let tokens = Tokens::from_code(b"<?php\n#[A, B(1, 2) ]\n#[C,\n  D\n]\nclass X {}");
        let got: Vec<_> = AttributeAnalyzer::collect(&tokens, 1)
            .iter()
            .map(|a| {
                let attrs: Vec<_> = a.attributes.iter().map(|x| (x.start, x.end, String::from_utf8_lossy(&x.name).into_owned())).collect();
                (a.start_index, a.end_index, a.opening_bracket_index, a.closing_bracket_index, attrs)
            })
            .collect();
        let s = |v: &str| v.to_string();
        assert_eq!(
            got,
            vec![
                (1, 14, 1, 13, vec![(2, 2, s("A")), (4, 12, s("B"))]),
                (15, 22, 15, 21, vec![(16, 16, s("C")), (18, 19, s("D"))]),
            ]
        );
    }

    /// Expected values: AttributeAnalyzer::isAttribute on each T_STRING of the same code.
    #[test]
    fn is_attribute_matches_oracle() {
        let tokens = Tokens::from_code(b"<?php #[A(B), C(D)] class X {}");
        let got: Vec<(usize, bool)> = (0..tokens.count())
            .filter(|&i| tokens[i].is_kind(T_STRING))
            .map(|i| (i, AttributeAnalyzer::is_attribute(&tokens, i)))
            .collect();
        assert_eq!(got, [(2, true), (4, false), (8, true), (10, false), (16, false)]);
    }
}
