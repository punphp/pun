//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocNoDuplicateTypesFixer`.
use crate::analyzers::docblock::type_expression::REGEX_TYPES;
use crate::analyzers::docblock::{DocBlock, TAGS_WITH_TYPES, TypeExpression};
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;
use std::sync::LazyLock;

struct PhpdocNoDuplicateTypesFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocNoDuplicateTypesFixer)
}

static METHOD_CALLABLE: LazyLock<String> =
    LazyLock::new(|| format!(r"/\*\h*@method\h+{REGEX_TYPES}\h+\K(?&callable)/"));

impl Fixer for PhpdocNoDuplicateTypesFixer {
    fn name(&self) -> &'static str {
        "phpdoc_no_duplicate_types"
    }
    fn priority(&self) -> i32 {
        // Must run before PhpdocAlignFixer.
        // Must run after AlignMultilineCommentFixer, CommentToPhpdocFixer, PhpdocArrayTypeFixer, PhpdocIndentFixer,
        // PhpdocListTypeFixer, PhpdocScalarFixer, PhpdocToCommentFixer, PhpdocTypesFixer.
        0
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
            let annotations = doc.get_annotations_of_type(TAGS_WITH_TYPES);
            if annotations.is_empty() {
                continue;
            }
            for annotation in annotations {
                // fix main types
                if let Some(type_expression) = annotation.get_type_expression() {
                    annotation.set_types(&type_expression.remove_duplicate_types().get_types());
                }
                // fix @method parameters types
                let line = doc.get_line(annotation.get_start()).unwrap();
                let content = Preg::replace_callback(
                    &*METHOD_CALLABLE,
                    |m| {
                        TypeExpression::new(m.str(0))
                            .remove_duplicate_types()
                            .get_types()
                            .join(&b'|')
                    },
                    line.borrow().get_content(),
                );
                line.borrow_mut().set_content(content);
            }
            tokens.set(index, Token::new(T_DOC_COMMENT, doc.get_content()));
        }
    }
}
