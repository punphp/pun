//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocVarWithoutNameFixer`.
use crate::analyzers::docblock::{
    DocBlock, LineRef, type_expression::REGEX_IDENTIFIER,
};
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::php_compat::str_replace;
use pun_tokens::{Kind, Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

const PROPERTY_MODIFIER_KINDS: &[Kind] = &[
    Kind::T_PRIVATE,
    Kind::T_PROTECTED,
    Kind::T_PUBLIC,
    Kind::T_VAR,
    Kind::T_READONLY,
    Kind::T_PRIVATE_SET,
    Kind::T_PROTECTED_SET,
    Kind::T_PUBLIC_SET,
];

struct PhpdocVarWithoutNameFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocVarWithoutNameFixer)
}

impl Fixer for PhpdocVarWithoutNameFixer {
    fn name(&self) -> &'static str {
        "phpdoc_var_without_name"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
            && tokens.is_any_token_kinds_found(&[Kind::T_CLASS, Kind::T_TRAIT])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(Kind::T_DOC_COMMENT) {
                continue;
            }
            let Some(mut next_index) = tokens.get_next_meaningful_token(index) else {
                continue;
            };
            if tokens[next_index].is_kind(Kind::T_STATIC) {
                next_index = tokens.get_next_meaningful_token(next_index).unwrap();
            }
            if !tokens[next_index].is_given_kind(PROPERTY_MODIFIER_KINDS) {
                continue;
            }
            let doc = DocBlock::new(&tokens[index].content);
            let first_level_lines = get_first_level_lines(&doc);
            for annotation in doc.get_annotations_of_type(&["type", "var"]) {
                if let Some(line) = first_level_lines.get(&annotation.get_start()) {
                    fix_line(line);
                }
            }
            tokens.set(index, Token::new(Kind::T_DOC_COMMENT, doc.get_content()));
        }
    }
}

fn fix_line(line: &LineRef) {
    let content = line.borrow().get_content().to_vec();
    let matches: Vec<Vec<u8>> =
        Preg::match_all(format!(r"/ \${REGEX_IDENTIFIER}(?<!\$this)/"), &content)
            .iter()
            .map(|m| m.str(0).to_vec())
            .collect();
    for m in matches {
        let new = str_replace(&m, b"", line.borrow().get_content());
        line.borrow_mut().set_content(new);
    }
}

fn get_first_level_lines(doc: &DocBlock) -> BTreeMap<usize, LineRef> {
    let mut nested = 0i32;
    let mut lines = BTreeMap::new();
    for (index, line) in doc.get_lines().iter().enumerate() {
        let content = line.borrow().get_content().to_vec();
        if Preg::r#match(r"/\s*\*\s*}$/", &content) {
            nested -= 1;
        }
        if nested <= 0 {
            lines.insert(index, line.clone());
        }
        if Preg::r#match(r"/\s\{$/", &content) {
            nested += 1;
        }
    }
    lines
}
