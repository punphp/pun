//! Port of `PhpCsFixer\Tokenizer\Analyzer\CommentsAnalyzer`.
use super::preg::Preg;
use pun_tokens::{Kind, Kind::*, Proto, Token, Tokens, php_compat};

pub struct CommentsAnalyzer;

const TYPE_HASH: u8 = 1;
const TYPE_DOUBLE_SLASH: u8 = 2;
const TYPE_SLASH_ASTERISK: u8 = 3;
const SKIP_TYPES: &[Kind] = &[
    T_PRIVATE, T_PROTECTED, T_PUBLIC, T_VAR, T_FUNCTION, T_FN, T_ABSTRACT, T_CONST, T_NAMESPACE, T_REQUIRE,
    T_REQUIRE_ONCE, T_INCLUDE, T_INCLUDE_ONCE, T_FINAL, CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED, CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE, T_READONLY,
    T_PUBLIC_SET, T_PROTECTED_SET, T_PRIVATE_SET,
];

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    needle.is_empty() || haystack.windows(needle.len()).any(|w| w == needle)
}

impl CommentsAnalyzer {
    pub fn is_header_comment(&self, tokens: &Tokens, index: usize) -> bool {
        assert!(tokens[index].is_given_kind(&[T_COMMENT, T_DOC_COMMENT]), "Given index must point to a comment.");
        if tokens.get_next_meaningful_token(index).is_none() {
            return false;
        }
        let mut prev_index = tokens.get_prev_non_whitespace(index, None).unwrap();
        if tokens[prev_index].equals(";", true) {
            let brace_close_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
            if !tokens[brace_close_index].equals(")", true) {
                return false;
            }
            let brace_open_index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, brace_close_index);
            let declare_index = tokens.get_prev_meaningful_token(brace_open_index).unwrap();
            if !tokens[declare_index].is_kind(T_DECLARE) {
                return false;
            }
            prev_index = tokens.get_prev_non_whitespace(declare_index, None).unwrap();
        }
        tokens[prev_index].is_kind(T_OPEN_TAG)
    }

    /// Check if comment at given index precedes structural element.
    pub fn is_before_structural_element(&self, tokens: &Tokens, index: usize) -> bool {
        let token = &tokens[index];
        assert!(token.is_given_kind(&[T_COMMENT, T_DOC_COMMENT]), "Given index must point to a comment.");
        let Some(next_index) = self.get_next_token_index(tokens, index) else { return false };
        if tokens[next_index].equals("}", true) {
            return false;
        }
        self.is_structural_element(tokens, next_index)
            || self.is_valid_control(tokens, token, next_index)
            || self.is_valid_variable(tokens, next_index)
            || self.is_valid_variable_assignment(tokens, token, next_index)
            || tokens[next_index].is_kind(CT_USE_TRAIT)
    }

    /// Check if comment at given index precedes return statement.
    pub fn is_before_return(&self, tokens: &Tokens, index: usize) -> bool {
        assert!(tokens[index].is_given_kind(&[T_COMMENT, T_DOC_COMMENT]), "Given index must point to a comment.");
        let Some(next_index) = self.get_next_token_index(tokens, index) else { return false };
        if tokens[next_index].equals("}", true) {
            return false;
        }
        tokens[next_index].is_kind(T_RETURN)
    }

    /// Indices that are part of a comment started at given index (T_COMMENT).
    pub fn get_comment_block_indices(&self, tokens: &Tokens, mut index: usize) -> Vec<usize> {
        assert!(tokens[index].is_kind(T_COMMENT), "Given index must point to a comment.");
        let comment_type = Self::get_comment_type(tokens[index].get_content());
        let mut indices = vec![index];
        if comment_type == TYPE_SLASH_ASTERISK {
            return indices;
        }
        let count = tokens.len();
        index += 1;
        while index < count {
            if tokens[index].is_comment() {
                if comment_type == Self::get_comment_type(tokens[index].get_content()) {
                    indices.push(index);
                    index += 1;
                    continue;
                }
                break;
            }
            if !tokens[index].is_whitespace(None) || Self::get_line_break_count(tokens, index, index + 1) > 1 {
                break;
            }
            index += 1;
        }
        indices
    }

    fn is_structural_element(&self, tokens: &Tokens, index: usize) -> bool {
        let token = &tokens[index];
        if token.is_classy() || token.is_given_kind(SKIP_TYPES) {
            return true;
        }
        if token.is_kind(T_STRING) {
            let content = php_compat::strtolower(token.get_content());
            return content == b"get" || content == b"set";
        }
        if token.is_kind(T_CASE) {
            let enum_parent = tokens.get_prev_token_of_kind(index, &[T_ENUM.into(), T_SWITCH.into()], true).unwrap();
            return tokens[enum_parent].is_kind(T_ENUM);
        }
        if token.is_kind(T_STATIC) {
            return !tokens[tokens.get_next_meaningful_token(index).unwrap()].is_kind(T_DOUBLE_COLON);
        }
        false
    }

    fn is_valid_control(&self, tokens: &Tokens, docs_token: &Token, control_index: usize) -> bool {
        if !tokens[control_index].is_given_kind(&[T_FOR, T_FOREACH, T_IF, T_SWITCH, T_WHILE]) {
            return false;
        }
        let open_parenthesis_index = tokens.get_next_meaningful_token(control_index).unwrap();
        let close_parenthesis_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis_index);
        let docs_content = docs_token.get_content();
        (open_parenthesis_index + 1..close_parenthesis_index)
            .any(|i| tokens[i].is_kind(T_VARIABLE) && contains(docs_content, tokens[i].get_content()))
    }

    fn is_valid_variable_assignment(&self, tokens: &Tokens, docs_token: &Token, language_construct_index: usize) -> bool {
        if !tokens[language_construct_index].is_given_kind(&[T_LIST, T_PRINT, T_ECHO, CT_DESTRUCTURING_BRACKET_OPEN]) {
            return false;
        }
        let end_kind: Proto = if tokens[language_construct_index].is_kind(CT_DESTRUCTURING_BRACKET_OPEN) {
            CT_DESTRUCTURING_BRACKET_CLOSE.into()
        } else {
            ")".into()
        };
        // upstream: `$index < null` is false for ints, so no iteration when not found
        let Some(end_index) = tokens.get_next_token_of_kind(language_construct_index, &[end_kind], true) else { return false };
        let docs_content = docs_token.get_content();
        (language_construct_index + 1..end_index)
            .any(|i| tokens[i].is_kind(T_VARIABLE) && contains(docs_content, tokens[i].get_content()))
    }

    fn is_valid_variable(&self, tokens: &Tokens, index: usize) -> bool {
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

    fn get_comment_type(content: &[u8]) -> u8 {
        if content.starts_with(b"#") {
            return TYPE_HASH;
        }
        if content.get(1) == Some(&b'*') {
            return TYPE_SLASH_ASTERISK;
        }
        TYPE_DOUBLE_SLASH
    }

    fn get_line_break_count(tokens: &Tokens, white_start: usize, white_end: usize) -> usize {
        (white_start..white_end).map(|i| Preg::match_all("/\\R/u", tokens[i].get_content(), 0).len()).sum()
    }

    fn get_next_token_index(&self, tokens: &Tokens, start_index: usize) -> Option<usize> {
        let mut next_index = Some(start_index);
        loop {
            next_index = tokens.get_next_meaningful_token(next_index.unwrap());
            while let Some(n) = next_index {
                if !tokens[n].is_kind(T_ATTRIBUTE) {
                    break;
                }
                let end = tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, n);
                next_index = tokens.get_next_meaningful_token(end);
            }
            match next_index {
                Some(n) if tokens[n].equals("(", true) => continue,
                _ => return next_index,
            }
        }
    }
}
