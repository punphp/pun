//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocAnnotationWithoutDotFixer`.
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::preg::{Preg, preg_quote};
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{TRIM_CHARS, ltrim_chars, mb_strtolower};
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PhpdocAnnotationWithoutDotFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocAnnotationWithoutDotFixer)
}

const TAGS: [&str; 7] = [
    "throws",
    "return",
    "param",
    "internal",
    "deprecated",
    "var",
    "type",
];

impl Fixer for PhpdocAnnotationWithoutDotFixer {
    fn name(&self) -> &'static str {
        "phpdoc_annotation_without_dot"
    }
    fn priority(&self) -> i32 {
        // Must run before PhpdocAlignFixer.
        // Must run after AlignMultilineCommentFixer, CommentToPhpdocFixer, PhpdocIndentFixer, PhpdocToCommentFixer.
        17
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            let doc = DocBlock::new(tokens[index].get_content());
            let annotations = doc.get_annotations();
            if annotations.is_empty() {
                continue;
            }
            for annotation in annotations {
                let tag = annotation.get_tag();
                if !tag.valid() || !TAGS.iter().any(|t| t.as_bytes() == tag.get_name()) {
                    continue;
                }
                // malformed PHPDoc, missing asterisk !
                let malformed = doc.get_line(annotation.get_end() + 1).is_some_and(|line| {
                    let line = line.borrow();
                    let trimmed = ltrim_chars(line.get_content(), TRIM_CHARS);
                    trimmed.is_empty() || !trimmed.starts_with(b"*")
                });
                if malformed {
                    continue;
                }
                let content = annotation.get_content();
                if !Preg::r#match(r"/[.。]\h*$/u", &content)
                    || Preg::r#match(r"/[.。](?!\h*$)/u", &content)
                {
                    continue;
                }
                let end_line = doc.get_line(annotation.get_end()).unwrap();
                let new = Preg::replace(
                    r"/(?<![.。])[.。]\h*(\H+)$/u",
                    r"\1",
                    end_line.borrow().get_content(),
                );
                end_line.borrow_mut().set_content(new);
                let start_line = doc.get_line(annotation.get_start()).unwrap();
                let optional_type_reg_ex = if annotation.support_types() {
                    [
                        &br"(?:"[..],
                        &preg_quote(&annotation.get_types().join(&b'|'), Some(b'/')),
                        br"\s+(?:\$\w+\s+)?)",
                    ]
                    .concat()
                } else {
                    vec![]
                };
                let pattern = [
                    &br"/^(\s*\*\s*@\w+\s+"[..],
                    &optional_type_reg_ex,
                    br")(\p{Lu}?(?=\p{Ll}|\p{Zs}))(.*)$/",
                ]
                .concat();
                let content = Preg::replace_callback_limit(
                    &pattern,
                    |m| [m.str(1), &mb_strtolower(m.str(2)), m.str(3)].concat(),
                    start_line.borrow().get_content(),
                    1,
                )
                .0;
                start_line.borrow_mut().set_content(content);
            }
            tokens.set(index, Token::new(T_DOC_COMMENT, doc.get_content()));
        }
    }
}
