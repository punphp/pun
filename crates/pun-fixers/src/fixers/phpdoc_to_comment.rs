//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocToCommentFixer`, with the parts of
//! `PhpCsFixer\Tokenizer\Analyzer\CommentsAnalyzer` it uses (`isHeaderComment`, `isBeforeReturn`,
//! `isBeforeStructuralElement`).
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Proto, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct PhpdocToCommentFixer {
    ignored_tags: Vec<Vec<u8>>,
    allow_before_return_statement: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocToCommentFixer {
        ignored_tags: Vec::new(),
        // Future::getV4OrV3(true, false): the 3.x default.
        allow_before_return_statement: false,
    })
}

impl Fixer for PhpdocToCommentFixer {
    fn name(&self) -> &'static str {
        "phpdoc_to_comment"
    }
    fn priority(&self) -> i32 {
        25
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &["ignored_tags", "allow_before_return_statement"],
        )?;
        self.ignored_tags = o
            .strings("ignored_tags", &[], &[])?
            .iter()
            .map(|t| strtolower(t.as_bytes()))
            .collect();
        self.allow_before_return_statement = o.bool("allow_before_return_statement", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        'tokens: for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            if is_header_comment(tokens, index) {
                continue;
            }
            if self.allow_before_return_statement && is_before_return(tokens, index) {
                continue;
            }
            if is_before_structural_element(tokens, index) {
                continue;
            }
            for m in Preg::match_all(r"~\@([a-zA-Z0-9_\\\-]+)\b~", tokens[index].get_content()) {
                if self.ignored_tags.contains(&strtolower(m.str(1))) {
                    continue 'tokens;
                }
            }
            let content = tokens[index].get_content();
            let trimmed = &content[content
                .iter()
                .take_while(|&&c| c == b'/' || c == b'*')
                .count()..];
            tokens.set(index, Token::new(T_COMMENT, [&b"/*"[..], trimmed].concat()));
        }
    }
}

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

fn is_header_comment(tokens: &Tokens, index: usize) -> bool {
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

fn is_before_structural_element(tokens: &Tokens, index: usize) -> bool {
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

fn is_before_return(tokens: &Tokens, index: usize) -> bool {
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
