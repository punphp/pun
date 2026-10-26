//! Shared parts of `PhpCsFixer\Fixer\AbstractPhpUnitFixer` and `DocBlockAnnotationTrait`.
use crate::analyzers::analysis::NamespaceUseType;
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use crate::analyzers::fully_qualified_name_analyzer::FullyQualifiedNameAnalyzer;
use crate::analyzers::php_unit_test_case_analyzer::PhpUnitTestCaseAnalyzer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;

/// `AbstractPhpUnitFixer::isCandidate`.
pub(crate) fn is_candidate(tokens: &Tokens) -> bool {
    tokens.is_all_token_kinds_found(&[T_CLASS, T_STRING])
}

/// `AbstractPhpUnitFixer::applyFix`: runs `apply_php_unit_class_fix(tokens, start, end)` per PHPUnit class,
/// lazily like upstream's generator (each class is located after the previous one was fixed).
pub(crate) fn apply_fix(
    tokens: &mut Tokens,
    mut apply_php_unit_class_fix: impl FnMut(&mut Tokens, usize, usize),
) {
    let analyzer = PhpUnitTestCaseAnalyzer;
    let mut before = tokens.count();
    while let Some((class_index, (start, end))) = analyzer.next_php_unit_class(tokens, before) {
        apply_php_unit_class_fix(tokens, start, end);
        before = class_index;
    }
}

/// `AbstractPhpUnitFixer::isTestAttributePresent`.
pub(crate) fn is_test_attribute_present(tokens: &Tokens, index: usize) -> bool {
    // The class's `{` always precedes a method.
    let attribute_index = tokens
        .get_prev_token_of_kind(index, &["{".into(), T_ATTRIBUTE.into()], true)
        .unwrap();
    if !tokens[attribute_index].is_kind(T_ATTRIBUTE) {
        return false;
    }
    let fqn = FullyQualifiedNameAnalyzer::new(tokens);
    for attribute_analysis in AttributeAnalyzer::collect(tokens, attribute_index) {
        for attribute in &attribute_analysis.attributes {
            let name = fqn
                .get_fully_qualified_name(&attribute.name, attribute.start, NamespaceUseType::Class)
                .to_ascii_lowercase();
            if name == b"phpunit\\framework\\attributes\\test" {
                return true;
            }
        }
    }
    false
}

/// `DocBlockAnnotationTrait::getDocBlockIndex`.
pub(crate) fn get_doc_block_index(tokens: &Tokens, mut index: usize) -> usize {
    loop {
        index = tokens.get_prev_non_whitespace(index, None).unwrap();
        if tokens[index].is_kind(CT_ATTRIBUTE_CLOSE) {
            index = tokens
                .get_prev_token_of_kind(index, &[T_ATTRIBUTE.into()], true)
                .unwrap();
        }
        if !tokens[index].is_given_kind(&[
            T_PUBLIC,
            T_PROTECTED,
            T_PRIVATE,
            T_FINAL,
            T_ABSTRACT,
            T_COMMENT,
            T_ATTRIBUTE,
            T_READONLY,
        ]) {
            return index;
        }
    }
}

/// One call `AbstractPhpUnitFixer::getPreviousAssertCall` yields.
pub(crate) struct AssertCall {
    pub index: usize,
    pub lowered_name: Vec<u8>,
    pub open_brace_index: usize,
    pub close_brace_index: usize,
}

/// `AbstractPhpUnitFixer::getPreviousAssertCall` as a cursor: each [`PreviousAssertCalls::next`] looks back from
/// where the previous one stopped, on the tokens as they are now (upstream's generator is lazy too).
pub(crate) struct PreviousAssertCalls {
    index: usize,
    start_index: usize,
    functions: crate::analyzers::functions_analyzer::FunctionsAnalyzer,
}

impl PreviousAssertCalls {
    pub(crate) fn new(start_index: usize, end_index: usize) -> Self {
        Self {
            index: end_index,
            start_index,
            functions: crate::analyzers::functions_analyzer::FunctionsAnalyzer::new(),
        }
    }

    pub(crate) fn next(&mut self, tokens: &Tokens) -> Option<AssertCall> {
        while self.index > self.start_index {
            let index = tokens.get_prev_token_of_kind(self.index, &[T_STRING.into()], true)?;
            // the loop's `--$index`
            self.index = index.saturating_sub(1);
            let lowered = pun_tokens::php_compat::strtolower(tokens[index].get_content());
            if !lowered.starts_with(b"assert") {
                continue;
            }
            let Some(open_brace_index) = tokens.get_next_meaningful_token(index) else {
                continue;
            };
            if !tokens[open_brace_index].equals("(", true) {
                continue;
            }
            if !self.functions.is_the_same_class_call(tokens, index) {
                continue;
            }
            return Some(AssertCall {
                index,
                lowered_name: lowered,
                open_brace_index,
                close_brace_index: tokens
                    .find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_brace_index),
            });
        }
        None
    }
}
