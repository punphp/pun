//! Port of `PhpCsFixer\Tokenizer\Analyzer\ClassyAnalyzer`.
use crate::analyzers::analysis::TypeAnalysis;
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;

pub struct ClassyAnalyzer;

impl ClassyAnalyzer {
    pub fn is_classy_invocation(&self, tokens: &Tokens, index: usize) -> bool {
        let token = &tokens[index];
        assert!(
            token.is_kind(T_STRING),
            "No T_STRING at given index {index}, got \"{:?}\".",
            token.get_name()
        );
        if TypeAnalysis::new(&token.content, None, None).is_reserved_type() {
            return false;
        }
        let next = tokens.get_next_meaningful_token(index).unwrap();
        let next_token = &tokens[next];
        if next_token.is_kind(T_NS_SEPARATOR) {
            return false;
        }
        if next_token.is_given_kind(&[
            T_DOUBLE_COLON,
            T_ELLIPSIS,
            CT_TYPE_ALTERNATION,
            CT_TYPE_INTERSECTION,
            T_VARIABLE,
        ]) {
            return true;
        }
        let mut prev = tokens.get_prev_meaningful_token(index).unwrap();
        while tokens[prev].is_given_kind(&[CT_NAMESPACE_OPERATOR, T_NS_SEPARATOR, T_STRING]) {
            prev = tokens.get_prev_meaningful_token(prev).unwrap();
        }
        let prev_token = &tokens[prev];
        if prev_token.is_given_kind(&[
            T_EXTENDS,
            T_INSTANCEOF,
            T_INSTEADOF,
            T_IMPLEMENTS,
            T_NEW,
            CT_NULLABLE_TYPE,
            CT_TYPE_ALTERNATION,
            CT_TYPE_INTERSECTION,
            CT_TYPE_COLON,
            CT_USE_TRAIT,
        ]) {
            return true;
        }
        if next_token.equals(")", true)
            && prev_token.equals("(", true)
            && tokens[tokens.get_prev_meaningful_token(prev).unwrap()].is_kind(T_CATCH)
        {
            return true;
        }
        if AttributeAnalyzer::is_attribute(tokens, index) {
            return true;
        }
        // `Foo & $bar` could be:
        //   - function reference parameter: function baz(Foo & $bar) {}
        //   - bit operator: $x = Foo & $bar;
        if next_token.equals("&", true)
            && tokens[tokens.get_next_meaningful_token(next).unwrap()].is_kind(T_VARIABLE)
        {
            let check_index = tokens
                .get_prev_token_of_kind(
                    prev + 1,
                    &[
                        ";".into(),
                        "{".into(),
                        "}".into(),
                        T_FUNCTION.into(),
                        T_OPEN_TAG.into(),
                        T_OPEN_TAG_WITH_ECHO.into(),
                    ],
                    true,
                )
                .unwrap();
            return tokens[check_index].is_kind(T_FUNCTION);
        }
        if !prev_token.equals(",", true) {
            return false;
        }
        loop {
            prev = tokens.get_prev_meaningful_token(prev).unwrap();
            if !tokens[prev].equals_any(
                &[
                    ",".into(),
                    T_NS_SEPARATOR.into(),
                    T_STRING.into(),
                    CT_NAMESPACE_OPERATOR.into(),
                ],
                true,
            ) {
                break;
            }
        }
        tokens[prev].is_given_kind(&[T_IMPLEMENTS, CT_USE_TRAIT])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected values: PhpCsFixer's ClassyAnalyzer::isClassyInvocation on each T_STRING of the same code.
    #[test]
    fn matches_oracle() {
        let tokens = Tokens::from_code(b"<?php new self; new Foo; function f(): int {}");
        let got: Vec<(usize, bool)> = (0..tokens.count())
            .filter(|&i| tokens[i].is_kind(T_STRING))
            .map(|i| (i, ClassyAnalyzer.is_classy_invocation(&tokens, i)))
            .collect();
        assert_eq!(got, [(3, false), (8, true), (13, false), (18, false)]);
    }
}
