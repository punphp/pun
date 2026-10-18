//! Port of `PhpCsFixer\AbstractPhpdocTypesFixer` (applyFix + fixType); subclasses pass `normalize`.
use crate::analyzers::docblock::{Annotation, DocBlock, TAGS_WITH_TYPES, TypeExpression};
use pun_tokens::{Kind, Token, Tokens};
use std::cell::RefCell;
use std::collections::HashMap;

pub(crate) fn is_candidate(tokens: &Tokens) -> bool {
    tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
}

/// Types content => fixed types content.
type FixedTypes = HashMap<Vec<u8>, Vec<u8>>;

thread_local! {
    /// fixType's result per (`memo_key`, types content): it depends on nothing else, and the same few types recur
    /// across every docblock of a project. Only successful parses are stored.
    // ponytail: unbounded per thread; a project has at most tens of thousands of distinct type strings.
    static FIXED_TYPES: RefCell<HashMap<String, FixedTypes>> = RefCell::default();
}

/// `memo_key` names the fixer and its configuration: two fixers (or configurations) never share results.
/// `may_change(docblock)` is false only when `normalize` provably leaves every type in it as it is (the docblock is
/// then not parsed at all).
pub(crate) fn apply_fix(
    tokens: &mut Tokens,
    memo_key: &str,
    may_change: &dyn Fn(&[u8]) -> bool,
    normalize: &dyn Fn(&[u8]) -> Vec<u8>,
) {
    for index in 0..tokens.count() {
        if !tokens[index].is_kind(Kind::T_DOC_COMMENT) || !may_change(tokens[index].get_content()) {
            continue;
        }
        let doc = DocBlock::new(tokens[index].get_content());
        let annotations = doc.get_annotations_of_type(TAGS_WITH_TYPES);
        if annotations.is_empty() {
            continue;
        }
        for annotation in annotations {
            fix_type(annotation, memo_key, normalize);
        }
        tokens.set(index, Token::new(Kind::T_DOC_COMMENT, doc.get_content()));
    }
}

fn fix_type(annotation: &Annotation, memo_key: &str, normalize: &dyn Fn(&[u8]) -> Vec<u8>) {
    let Some(content) = annotation.get_types_content() else {
        return;
    };
    let cached = FIXED_TYPES.with(|m| {
        m.borrow()
            .get(memo_key)
            .and_then(|m| m.get(&content).cloned())
    });
    let new = cached.unwrap_or_else(|| {
        let type_expression = annotation.get_type_expression().unwrap();
        let new = type_expression.map_types(&mut |t| {
            if !t.is_composite_type() {
                TypeExpression::new(&normalize(t.to_string()))
            } else {
                t
            }
        });
        let new = new.to_string().to_vec();
        FIXED_TYPES.with(|m| {
            m.borrow_mut()
                .entry(memo_key.to_string())
                .or_default()
                .insert(content, new.clone())
        });
        new
    });
    annotation.set_types(&[new]);
}
