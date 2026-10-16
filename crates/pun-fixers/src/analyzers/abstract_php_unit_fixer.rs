//! Shared parts of `PhpCsFixer\Fixer\AbstractPhpUnitFixer` and `DocBlockAnnotationTrait`.
use crate::analyzers::analysis::NamespaceUseType;
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use crate::analyzers::fully_qualified_name_analyzer::FullyQualifiedNameAnalyzer;
use crate::analyzers::php_unit_test_case_analyzer::PhpUnitTestCaseAnalyzer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;

/// `AbstractPhpUnitFixer::isCandidate`.
pub fn is_candidate(tokens: &Tokens) -> bool {
    tokens.is_all_token_kinds_found(&[T_CLASS, T_STRING])
}

/// `AbstractPhpUnitFixer::applyFix`: runs `apply_php_unit_class_fix(tokens, start, end)` per PHPUnit class,
/// lazily like upstream's generator (each class is located after the previous one was fixed).
pub fn apply_fix(tokens: &mut Tokens, mut apply_php_unit_class_fix: impl FnMut(&mut Tokens, usize, usize)) {
    let analyzer = PhpUnitTestCaseAnalyzer;
    let mut index = tokens.count() as isize - 1;
    while let Some((class_index, (start, end))) = analyzer.next_php_unit_class(tokens, index) {
        apply_php_unit_class_fix(tokens, start, end);
        index = class_index as isize - 1;
    }
}

/// `AbstractPhpUnitFixer::isTestAttributePresent`.
pub fn is_test_attribute_present(tokens: &Tokens, index: usize) -> bool {
    // The class's `{` always precedes a method.
    let attribute_index = tokens.get_prev_token_of_kind(index, &["{".into(), T_ATTRIBUTE.into()], true).unwrap();
    if !tokens[attribute_index].is_kind(T_ATTRIBUTE) {
        return false;
    }
    let fqn = FullyQualifiedNameAnalyzer::new(tokens);
    for attribute_analysis in AttributeAnalyzer::collect(tokens, attribute_index) {
        for attribute in &attribute_analysis.attributes {
            let name = fqn.get_fully_qualified_name(&attribute.name, attribute.start, NamespaceUseType::Class).to_ascii_lowercase();
            if name == b"phpunit\\framework\\attributes\\test" {
                return true;
            }
        }
    }
    false
}

/// `DocBlockAnnotationTrait::getDocBlockIndex`.
pub fn get_doc_block_index(tokens: &Tokens, mut index: usize) -> usize {
    loop {
        index = tokens.get_prev_non_whitespace(index, None).unwrap();
        if tokens[index].is_kind(CT_ATTRIBUTE_CLOSE) {
            index = tokens.get_prev_token_of_kind(index, &[T_ATTRIBUTE.into()], true).unwrap();
        }
        if !tokens[index].is_given_kind(&[T_PUBLIC, T_PROTECTED, T_PRIVATE, T_FINAL, T_ABSTRACT, T_COMMENT, T_ATTRIBUTE, T_READONLY]) {
            return index;
        }
    }
}
