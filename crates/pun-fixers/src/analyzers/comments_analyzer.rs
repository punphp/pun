//! Port of `PhpCsFixer\Tokenizer\Analyzer\CommentsAnalyzer` (`isHeaderComment`, `isBeforeStructuralElement`,
//! `isBeforeReturn`, `getCommentBlockIndices`). Callers only pass comment indices, so upstream's
//! `InvalidArgumentException` checks are left out.
use crate::analyzers::preg::Preg;
use pun_tokens::Kind::{self, *};
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Proto, Tokens};

const SKIP_TYPES: &[Kind] = &[
    T_PRIVATE,
    T_PROTECTED,
    T_PUBLIC,
    T_VAR,
    T_FUNCTION,
    T_FN,
    T_ABSTRACT,
    T_CONST,
    T_NAMESPACE,
    T_REQUIRE,
    T_REQUIRE_ONCE,
    T_INCLUDE,
    T_INCLUDE_ONCE,
    T_FINAL,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
    T_READONLY,
    T_PUBLIC_SET,
    T_PROTECTED_SET,
    T_PRIVATE_SET,
];

pub(crate) fn is_header_comment(tokens: &Tokens, index: usize) -> bool {
    if tokens.get_next_meaningful_token(index).is_none() {
        return false;
    }
    let mut prev_index = tokens.get_prev_non_whitespace(index, None).unwrap();
    if tokens[prev_index].equals(";", true) {
        let brace_close_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
        if !tokens[brace_close_index].equals(")", true) {
            return false;
        }
        let brace_open_index =
            tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, brace_close_index);
        let declare_index = tokens.get_prev_meaningful_token(brace_open_index).unwrap();
        if !tokens[declare_index].is_kind(T_DECLARE) {
            return false;
        }
        prev_index = tokens.get_prev_non_whitespace(declare_index, None).unwrap();
    }
    tokens[prev_index].is_kind(T_OPEN_TAG)
}

pub(crate) fn is_before_structural_element(tokens: &Tokens, index: usize) -> bool {
    let Some(next_index) = get_next_token_index(tokens, index) else {
        return false;
    };
    if tokens[next_index].equals("}", true) {
        return false;
    }
    let docs = tokens[index].get_content();
    is_structural_element(tokens, next_index)
        || is_valid_control(tokens, docs, next_index)
        || is_valid_variable(tokens, next_index)
        || is_valid_variable_assignment(tokens, docs, next_index)
        || tokens[next_index].is_kind(CT_USE_TRAIT)
}

pub(crate) fn is_before_return(tokens: &Tokens, index: usize) -> bool {
    let Some(next_index) = get_next_token_index(tokens, index) else {
        return false;
    };
    !tokens[next_index].equals("}", true) && tokens[next_index].is_kind(T_RETURN)
}

fn is_structural_element(tokens: &Tokens, index: usize) -> bool {
    let token = &tokens[index];
    if token.is_classy() || token.is_given_kind(SKIP_TYPES) {
        return true;
    }
    if token.is_kind(T_STRING) {
        let content = strtolower(token.get_content());
        return content == b"get" || content == b"set";
    }
    if token.is_kind(T_CASE) {
        let enum_parent = tokens
            .get_prev_token_of_kind(index, &[T_ENUM.into(), T_SWITCH.into()], true)
            .unwrap();
        return tokens[enum_parent].is_kind(T_ENUM);
    }
    if token.is_kind(T_STATIC) {
        let next = tokens.get_next_meaningful_token(index).unwrap();
        return !tokens[next].is_kind(T_DOUBLE_COLON);
    }
    false
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

fn is_valid_control(tokens: &Tokens, docs_content: &[u8], control_index: usize) -> bool {
    if !tokens[control_index].is_given_kind(&[T_FOR, T_FOREACH, T_IF, T_SWITCH, T_WHILE]) {
        return false;
    }
    let open_parenthesis_index = tokens.get_next_meaningful_token(control_index).unwrap();
    let close_parenthesis_index =
        tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis_index);
    (open_parenthesis_index + 1..close_parenthesis_index)
        .any(|i| tokens[i].is_kind(T_VARIABLE) && contains(docs_content, tokens[i].get_content()))
}

fn is_valid_variable_assignment(
    tokens: &Tokens,
    docs_content: &[u8],
    language_construct_index: usize,
) -> bool {
    if !tokens[language_construct_index].is_given_kind(&[
        T_LIST,
        T_PRINT,
        T_ECHO,
        CT_DESTRUCTURING_BRACKET_OPEN,
    ]) {
        return false;
    }
    let end_kind: Proto<'_> =
        if tokens[language_construct_index].is_kind(CT_DESTRUCTURING_BRACKET_OPEN) {
            CT_DESTRUCTURING_BRACKET_CLOSE.into()
        } else {
            ")".into()
        };
    let end_index = tokens
        .get_next_token_of_kind(language_construct_index, &[end_kind], true)
        .unwrap();
    (language_construct_index + 1..end_index)
        .any(|i| tokens[i].is_kind(T_VARIABLE) && contains(docs_content, tokens[i].get_content()))
}

fn is_valid_variable(tokens: &Tokens, index: usize) -> bool {
    if !tokens[index].is_kind(T_VARIABLE) {
        return false;
    }
    let next_index = tokens.get_next_meaningful_token(index).unwrap();
    tokens[next_index].equals_any(
        &[
            "=".into(),
            (T_PLUS_EQUAL, "+=").into(),
            (T_MINUS_EQUAL, "-=").into(),
            (T_MUL_EQUAL, "*=").into(),
            (T_DIV_EQUAL, "/=").into(),
            (T_MOD_EQUAL, "%=").into(),
            (T_POW_EQUAL, "**=").into(),
            (T_AND_EQUAL, "&=").into(),
            (T_OR_EQUAL, "|=").into(),
            (T_XOR_EQUAL, "^=").into(),
            (T_SL_EQUAL, "<<=").into(),
            (T_SR_EQUAL, ">>=").into(),
            (T_COALESCE_EQUAL, "??=").into(),
            (T_CONCAT_EQUAL, ".=").into(),
        ],
        true,
    )
}

fn get_next_token_index(tokens: &Tokens, start_index: usize) -> Option<usize> {
    let mut next_index = Some(start_index);
    loop {
        next_index = tokens.get_next_meaningful_token(next_index.unwrap());
        while let Some(i) = next_index
            && tokens[i].is_kind(T_ATTRIBUTE)
        {
            let end = tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, i);
            next_index = tokens.get_next_meaningful_token(end);
        }
        match next_index {
            Some(i) if tokens[i].equals("(", true) => continue,
            _ => return next_index,
        }
    }
}

/// `getCommentBlockIndices`: the comment at `index` and the comments of the same type on the following lines
/// (no blank line between them). `/* */` comments stand alone.
pub(crate) fn get_comment_block_indices(tokens: &Tokens, mut index: usize) -> Vec<usize> {
    let comment_type = get_comment_type(tokens[index].get_content());
    let mut indices = vec![index];
    if comment_type == CommentType::SlashAsterisk {
        return indices;
    }
    index += 1;
    while index < tokens.count() {
        if tokens[index].is_comment() {
            if comment_type == get_comment_type(tokens[index].get_content()) {
                indices.push(index);
                index += 1;
                continue;
            }
            break;
        }
        if !tokens[index].is_whitespace(None)
            || Preg::match_all(r"/\R/u", tokens[index].get_content()).len() > 1
        {
            break;
        }
        index += 1;
    }
    indices
}

#[derive(PartialEq)]
enum CommentType {
    Hash,
    DoubleSlash,
    SlashAsterisk,
}

fn get_comment_type(content: &[u8]) -> CommentType {
    if content.starts_with(b"#") {
        CommentType::Hash
    } else if content[1] == b'*' {
        CommentType::SlashAsterisk
    } else {
        CommentType::DoubleSlash
    }
}
