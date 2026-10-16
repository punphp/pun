//! Port of `PhpCsFixer\Fixer\Whitespace\ArrayIndentationFixer`.
use crate::analyzers::indentation_trait::{compute_new_line_content, extract_indent, is_new_line_token};
use crate::analyzers::preg::{Preg, preg_quote};
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

#[derive(Default)]
struct ArrayIndentationFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ArrayIndentationFixer::default())
}

struct Scope {
    is_array: bool,
    end_index: usize,
    initial_indent: Vec<u8>,
    new_indent: Vec<u8>,
}

impl ArrayIndentationFixer {
    fn find_expression_end_index(tokens: &Tokens, index: usize, parent_scope_end_index: usize) -> usize {
        let mut end_index = None;
        let mut search_end_index = index + 1;
        while search_end_index < parent_scope_end_index {
            let search_end_token = &tokens[search_end_index];
            if search_end_token.equals_any(&["(".into(), "{".into()], true)
                || search_end_token.is_given_kind(&[CT_ARRAY_BRACKET_OPEN, CT_DESTRUCTURING_BRACKET_OPEN])
            {
                let (t, _) = Tokens::detect_block_type(search_end_token).unwrap();
                search_end_index = tokens.find_block_end(t, search_end_index) + 1;
                continue;
            }
            if search_end_token.equals(",", true) {
                end_index = tokens.get_prev_meaningful_token(search_end_index);
                break;
            }
            search_end_index += 1;
        }
        end_index.unwrap_or_else(|| tokens.get_prev_meaningful_token(parent_scope_end_index).unwrap())
    }
}

impl Fixer for ArrayIndentationFixer {
    fn name(&self) -> &'static str {
        "array_indentation"
    }
    fn priority(&self) -> i32 {
        // Must run before AlignMultilineCommentFixer, BinaryOperatorSpacesFixer.
        // Must run after MethodArgumentSpaceFixer.
        29
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_ARRAY, T_LIST, CT_ARRAY_BRACKET_OPEN, CT_DESTRUCTURING_BRACKET_OPEN])
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut last_indent: Vec<u8> = Vec::new();
        let mut scopes: Vec<Scope> = Vec::new();
        let mut previous_line_initial_indent: Vec<u8> = Vec::new();
        let mut previous_line_new_indent: Vec<u8> = Vec::new();
        let mut index = 0;
        while index < tokens.count() {
            let token = tokens[index].clone();
            let i = index;
            index += 1;
            if token.is_comment() {
                continue;
            }
            if token.is_given_kind(&[CT_ARRAY_BRACKET_OPEN, CT_DESTRUCTURING_BRACKET_OPEN])
                || (token.equals("(", true) && tokens[tokens.get_prev_meaningful_token(i).unwrap()].is_given_kind(&[T_ARRAY, T_LIST]))
            {
                let (block_type, _) = Tokens::detect_block_type(&token).unwrap();
                let end_index = tokens.find_block_end(block_type, i);
                scopes.push(Scope { is_array: true, end_index, initial_indent: last_indent.clone(), new_indent: Vec::new() });
                continue;
            }
            if is_new_line_token(tokens, i) {
                last_indent = extract_indent(&compute_new_line_content(tokens, i));
            }
            if scopes.is_empty() {
                continue;
            }
            let current_scope = scopes.len() - 1;
            if token.is_whitespace(None) {
                if !Preg::r#match("/\\R/", token.get_content()) {
                    continue;
                }
                let scope = &scopes[current_scope];
                let content;
                if scope.is_array {
                    let mut indent = false;
                    for search_end_index in i + 1..scope.end_index {
                        let search_end_token = &tokens[search_end_index];
                        if (!search_end_token.is_whitespace(None) && !search_end_token.is_comment())
                            || (search_end_token.is_whitespace(None) && Preg::r#match("/\\R/", search_end_token.get_content()))
                        {
                            indent = true;
                            break;
                        }
                    }
                    let replacement = [
                        b"$1".as_slice(),
                        &scope.initial_indent,
                        if indent { self.whitespaces_config.get_indent().as_bytes() } else { b"" },
                    ]
                    .concat();
                    content = Preg::replace("/(\\R+)\\h*$/", replacement, token.get_content());
                    previous_line_initial_indent = extract_indent(token.get_content());
                    previous_line_new_indent = extract_indent(&content);
                } else {
                    let pattern = [b"/(\\R)".as_slice(), &preg_quote(&scope.initial_indent, Some(b'/')), b"(\\h*)$/"].concat();
                    let replacement = [b"$1".as_slice(), &scope.new_indent, b"$2"].concat();
                    content = Preg::replace(pattern, replacement, token.get_content());
                }
                last_indent = extract_indent(&content);
                tokens.set(i, Token::new(T_WHITESPACE, content));
                continue;
            }
            if i == scopes[current_scope].end_index {
                while scopes.last().is_some_and(|s| s.end_index == i) {
                    scopes.pop();
                }
                continue;
            }
            if token.equals(",", true) {
                continue;
            }
            if scopes[current_scope].is_array {
                let end_index = Self::find_expression_end_index(tokens, i, scopes[current_scope].end_index);
                if end_index == i {
                    continue;
                }
                scopes.push(Scope {
                    is_array: false,
                    end_index,
                    initial_indent: previous_line_initial_indent.clone(),
                    new_indent: previous_line_new_indent.clone(),
                });
            }
        }
    }
}
