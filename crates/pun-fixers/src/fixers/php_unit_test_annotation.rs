//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitTestAnnotationFixer`. Doc block lines are kept as their contents.
use crate::analyzers::abstract_php_unit_fixer::{
    apply_fix, create_doc_block, get_doc_block_index, is_candidate,
};
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::helpers::contains;
use crate::analyzers::preg::Preg;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{str_replace, stripos, trim};
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

struct PhpUnitTestAnnotationFixer {
    annotation: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpUnitTestAnnotationFixer {
        annotation: false,
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

impl Fixer for PhpUnitTestAnnotationFixer {
    fn name(&self) -> &'static str {
        "php_unit_test_annotation"
    }
    fn priority(&self) -> i32 {
        10
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["style"])?;
        self.annotation = c.string("style", "prefix", &["prefix", "annotation"])? == "annotation";
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
            if self.annotation {
                self.apply_test_annotation(tokens, start_index, end_index);
            } else {
                self.apply_test_prefix(tokens, start_index, end_index);
            }
        });
    }
}

impl PhpUnitTestAnnotationFixer {
    fn line_end(&self) -> &[u8] {
        self.whitespaces_config.get_line_ending().as_bytes()
    }

    /// `applyTestAnnotation`. Upstream does not move `i` past the tokens a new doc block inserts.
    fn apply_test_annotation(&self, tokens: &mut Tokens, start_index: usize, end_index: usize) {
        let mut i = end_index - 1;
        while i > start_index {
            if is_test_method(tokens, i) {
                let function_name_index = tokens.get_next_meaningful_token(i).unwrap();
                let function_name = tokens[function_name_index].get_content().to_vec();
                if has_test_prefix(&function_name) && !has_proper_test_annotation(tokens, i) {
                    let new_function_name = remove_test_prefix(&function_name);
                    tokens.set(function_name_index, Token::new(T_STRING, new_function_name));
                }
                let doc_block_index = get_doc_block_index(tokens, i);
                if tokens[doc_block_index].is_kind(T_DOC_COMMENT) {
                    let lines = self.update_doc_block(tokens, doc_block_index);
                    let lines = self.add_test_annotation(lines, tokens, doc_block_index);
                    tokens.set(doc_block_index, Token::new(T_DOC_COMMENT, lines.concat()));
                } else {
                    // Create a new docblock if it didn't have one before;
                    create_doc_block(tokens, doc_block_index, "test", self.line_end());
                }
            }
            i -= 1;
        }
    }

    /// `applyTestPrefix`.
    fn apply_test_prefix(&self, tokens: &mut Tokens, start_index: usize, end_index: usize) {
        let mut i = end_index - 1;
        while i > start_index {
            if is_test_method(tokens, i) {
                let doc_block_index = get_doc_block_index(tokens, i);
                if tokens[doc_block_index].is_kind(T_DOC_COMMENT) {
                    let lines = self.update_doc_block(tokens, doc_block_index);
                    tokens.set(doc_block_index, Token::new(T_DOC_COMMENT, lines.concat()));
                    let function_name_index = tokens.get_next_meaningful_token(i).unwrap();
                    let function_name = tokens[function_name_index].get_content().to_vec();
                    if !has_test_prefix(&function_name) {
                        tokens.set(
                            function_name_index,
                            Token::new(T_STRING, add_test_prefix(&function_name)),
                        );
                    }
                }
            }
            i -= 1;
        }
    }

    /// `updateDocBlock`.
    fn update_doc_block(&self, tokens: &Tokens, doc_block_index: usize) -> Vec<Vec<u8>> {
        let doc = DocBlock::new(tokens[doc_block_index].get_content());
        let lines = doc
            .get_lines()
            .iter()
            .map(|l| l.borrow().get_content().to_vec())
            .collect();
        self.update_lines(lines, tokens, doc_block_index)
    }

    /// `updateLines`.
    fn update_lines(
        &self,
        mut lines: Vec<Vec<u8>>,
        tokens: &Tokens,
        doc_block_index: usize,
    ) -> Vec<Vec<u8>> {
        let needs_annotation = self.annotation;
        let doc = DocBlock::new(tokens[doc_block_index].get_content());
        for i in 0..lines.len() {
            let mut line = lines[i].clone();
            // If we need to add test annotation and it is a single line comment we need to deal with that separately
            if needs_annotation
                && contains(&line, b"/**")
                && contains(&line, b"*/")
                && !does_doc_block_contain_test(&doc)
            {
                let lines = self.split_up_doc_block(&lines, tokens, doc_block_index);
                return self.update_lines(lines, tokens, doc_block_index);
            }
            if !needs_annotation
                && contains(&line, b" @test")
                && !contains(&line, b"@testWith")
                && !contains(&line, b"@testdox")
            {
                // We remove @test from the doc block
                line = str_replace(b" @test", b"", &line);
                lines[i].clone_from(&line);
            }
            // ignore the line if it isn't @depends
            if !contains(&line, b"@depends") {
                continue;
            }
            lines[i] = self.update_depends_annotation(&line);
        }
        lines
    }

    /// `splitUpDocBlock`.
    fn split_up_doc_block(
        &self,
        lines: &[Vec<u8>],
        tokens: &Tokens,
        doc_block_index: usize,
    ) -> Vec<Vec<u8>> {
        let line_content = get_single_line_doc_block_entry(lines);
        let line_end = self.line_end();
        let next = tokens
            .get_next_non_whitespace(doc_block_index, None)
            .unwrap();
        let original_indent = WhitespacesAnalyzer::detect_indent(tokens, next);
        vec![
            [b"/**", line_end].concat(),
            [&original_indent, b" * ".as_slice(), &line_content, line_end].concat(),
            [&original_indent, b" */".as_slice()].concat(),
        ]
    }

    /// `updateDependsAnnotation`, with `removeTestPrefixFromDependsAnnotation` and `addTestPrefixToDependsAnnotation`.
    fn update_depends_annotation(&self, line: &[u8]) -> Vec<u8> {
        let depends_index = find_where_depends_function_name_starts(line);
        let (head, depends_function_name) = line.split_at(depends_index.min(line.len()));
        let depends_function_name = if self.annotation {
            if has_test_prefix(depends_function_name) {
                remove_test_prefix(depends_function_name)
            } else {
                depends_function_name.to_vec()
            }
        } else if has_test_prefix(depends_function_name) {
            depends_function_name.to_vec()
        } else {
            add_test_prefix(depends_function_name)
        };
        [head, &depends_function_name].concat()
    }

    /// `addTestAnnotation`.
    fn add_test_annotation(
        &self,
        mut lines: Vec<Vec<u8>>,
        tokens: &Tokens,
        doc_block_index: usize,
    ) -> Vec<Vec<u8>> {
        let doc = DocBlock::new(tokens[doc_block_index].get_content());
        if !does_doc_block_contain_test(&doc) {
            let original_indent = WhitespacesAnalyzer::detect_indent(tokens, doc_block_index);
            let line_end = self.line_end();
            let line = [
                &original_indent,
                b" *".as_slice(),
                line_end,
                &original_indent,
                b" * @test",
                line_end,
            ]
            .concat();
            let at = lines.len() - 1;
            lines.insert(at, line);
        }
        lines
    }
}

/// `isTestMethod`, with `isMethod`.
fn is_test_method(tokens: &Tokens, index: usize) -> bool {
    // Check if we are dealing with a (non-abstract, non-lambda) function
    if !tokens[index].is_kind(T_FUNCTION) || TokensAnalyzer::new(tokens).is_lambda(index) {
        return false;
    }
    // if the function name starts with test it is a test
    let function_name_index = tokens.get_next_meaningful_token(index).unwrap();
    if has_test_prefix(tokens[function_name_index].get_content()) {
        return true;
    }
    let doc_block_index = get_doc_block_index(tokens, index);
    // If the function doesn't have test in its name, and no doc block, it is not a test
    tokens[doc_block_index].is_kind(T_DOC_COMMENT)
        && contains(tokens[doc_block_index].get_content(), b"@test")
}

fn has_test_prefix(function_name: &[u8]) -> bool {
    function_name.starts_with(b"test")
}

/// `hasProperTestAnnotation`.
fn has_proper_test_annotation(tokens: &Tokens, index: usize) -> bool {
    let doc_block_index = get_doc_block_index(tokens, index);
    Preg::r#match(r"/\*\s+@test\b/", tokens[doc_block_index].get_content())
}

/// `removeTestPrefix`.
fn remove_test_prefix(function_name: &[u8]) -> Vec<u8> {
    let mut remainder = Preg::replace(r"/^test(?=[A-Z_])_?/", "", function_name);
    if remainder.is_empty() {
        return function_name.to_vec();
    }
    remainder[0] = remainder[0].to_ascii_lowercase();
    remainder
}

/// `addTestPrefix`.
fn add_test_prefix(function_name: &[u8]) -> Vec<u8> {
    let mut name = function_name.to_vec();
    if let Some(first) = name.first_mut() {
        *first = first.to_ascii_uppercase();
    }
    [b"test".as_slice(), &name].concat()
}

/// `getSingleLineDocBlockEntry`. Out-of-range offsets read as PHP's `null`.
fn get_single_line_doc_block_entry(lines: &[Vec<u8>]) -> Vec<u8> {
    let line = str_replace(b"*/", b"", &lines[0]);
    let line = trim(&line);
    let at = |i: isize| usize::try_from(i).ok().and_then(|i| line.get(i)).copied();
    let mut i = line.len() as isize;
    loop {
        i -= 1;
        if !(at(i) != Some(b'*') && at(i - 1) != Some(b'*') && at(i - 2) != Some(b'/')) {
            break;
        }
    }
    if at(i) == Some(b' ') {
        i += 1;
    }
    line[i as usize..].to_vec()
}

/// `findWhereDependsFunctionNameStarts`.
fn find_where_depends_function_name_starts(line: &[u8]) -> usize {
    let mut index = stripos(line, b"@depends", 0).unwrap_or(0) + 8;
    while line.get(index) == Some(&b' ') {
        index += 1;
    }
    index
}

/// `doesDocBlockContainTest`.
fn does_doc_block_contain_test(doc: &DocBlock) -> bool {
    !doc.get_annotations_of_type(&["test"]).is_empty()
}
