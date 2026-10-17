//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocNoUselessInheritdocFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

struct PhpdocNoUselessInheritdocFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocNoUselessInheritdocFixer)
}

impl Fixer for PhpdocNoUselessInheritdocFixer {
    fn name(&self) -> &'static str {
        "phpdoc_no_useless_inheritdoc"
    }
    fn priority(&self) -> i32 {
        6
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
            && tokens.is_any_token_kinds_found(&[Kind::T_CLASS, Kind::T_INTERFACE])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let count = tokens.count() as isize - 4;
        let mut index = 1;
        while (index as isize) < count {
            if tokens[index].is_given_kind(&[Kind::T_CLASS, Kind::T_INTERFACE]) {
                index = fix_classy(tokens, index);
            }
            index += 1;
        }
    }
}

fn fix_classy(tokens: &mut Tokens, index: usize) -> usize {
    let class_open_index = tokens
        .get_next_token_of_kind(index, &["{".into()], true)
        .unwrap();
    let class_end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, class_open_index);
    let mut extending_or_implementing =
        is_extending_or_implementing(tokens, index, class_open_index);
    if !extending_or_implementing {
        fix_classy_outside(tokens, index);
    }
    if !extending_or_implementing
        && is_using_trait(tokens, index, class_open_index, class_end_index)
    {
        extending_or_implementing = true;
    }
    fix_classy_inside(
        tokens,
        class_open_index,
        class_end_index,
        !extending_or_implementing,
    );
    class_end_index
}

fn fix_classy_inside(
    tokens: &mut Tokens,
    class_open_index: usize,
    class_end_index: usize,
    fix_this_level: bool,
) {
    let mut i = class_open_index;
    while i < class_end_index {
        if tokens[i].is_kind(Kind::T_CLASS) {
            i = fix_classy(tokens, i);
        } else if fix_this_level && tokens[i].is_kind(Kind::T_DOC_COMMENT) {
            fix_token(tokens, i);
        }
        i += 1;
    }
}

fn fix_classy_outside(tokens: &mut Tokens, class_index: usize) {
    let previous_index = tokens.get_prev_non_whitespace(class_index, None).unwrap();
    if tokens[previous_index].is_kind(Kind::T_DOC_COMMENT) {
        fix_token(tokens, previous_index);
    }
}

fn fix_token(tokens: &mut Tokens, token_index: usize) {
    let (content, count) = Preg::replace_callback_limit(
        r"#(\h*(?:@{*|{*\h*@)\h*inheritdoc\h*)([^}]*)((?:}*)\h*)#i",
        |m| [&b" "[..], m.str(2)].concat(),
        &tokens[token_index].content,
        -1,
    );
    if count > 0 {
        tokens.set(token_index, Token::new(Kind::T_DOC_COMMENT, content));
    }
}

fn is_extending_or_implementing(
    tokens: &Tokens,
    class_index: usize,
    class_open_index: usize,
) -> bool {
    (class_index..class_open_index)
        .any(|i| tokens[i].is_given_kind(&[Kind::T_EXTENDS, Kind::T_IMPLEMENTS]))
}

fn is_using_trait(
    tokens: &Tokens,
    class_index: usize,
    class_open_index: usize,
    class_close_index: usize,
) -> bool {
    if tokens[class_index].is_kind(Kind::T_INTERFACE) {
        return false;
    }
    tokens
        .get_next_token_of_kind(class_open_index, &[Kind::CT_USE_TRAIT.into()], true)
        .is_some_and(|u| u < class_close_index)
}
