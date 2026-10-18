//! Port of `PhpCsFixer\Fixer\Alias\BacktickToShellExecFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

struct BacktickToShellExecFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(BacktickToShellExecFixer)
}

impl Fixer for BacktickToShellExecFixer {
    fn name(&self) -> &'static str {
        "backtick_to_shell_exec"
    }
    fn priority(&self) -> i32 {
        17
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("`")
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut backtick_started = false;
        let mut backtick_tokens: BTreeMap<usize, Token> = BTreeMap::new();
        for index in (1..tokens.count()).rev() {
            let token = tokens[index].clone();
            if !token.equals("`", true) {
                if backtick_started {
                    backtick_tokens.insert(index, token);
                }
                continue;
            }
            backtick_tokens.insert(index, token);
            if backtick_started {
                fix_backticks(tokens, std::mem::take(&mut backtick_tokens));
            }
            backtick_started = !backtick_started;
        }
    }
}

fn fix_backticks(tokens: &mut Tokens, backtick_tokens: BTreeMap<usize, Token>) {
    let opening_backtick_index = *backtick_tokens.keys().next().unwrap();
    let closing_backtick_index = *backtick_tokens.keys().next_back().unwrap();

    let mut inner: Vec<Token> = backtick_tokens.into_values().collect();
    inner.remove(0);
    inner.pop();

    // Double-quoted strings are parsed differently if they contain
    // variables or not, so we need to build the new token array accordingly
    let count = inner.len();
    let mut new_tokens = vec![Token::new(T_STRING, "shell_exec"), Token::char("(")];
    if count != 1 {
        new_tokens.push(Token::char("\""));
    }
    for token in inner {
        if !token.is_kind(T_ENCAPSED_AND_WHITESPACE) {
            new_tokens.push(token);
            continue;
        }
        let mut content = token.content.clone();
        // Escaping special chars depends on the context: too tricky
        if Preg::r#match("/[`\"']/u", &content) {
            return;
        }
        let mut kind = T_ENCAPSED_AND_WHITESPACE;
        if count == 1 {
            content = [&b"\""[..], &content, b"\""].concat();
            kind = T_CONSTANT_ENCAPSED_STRING;
        }
        new_tokens.push(Token::new(kind, content));
    }
    if count != 1 {
        new_tokens.push(Token::char("\""));
    }
    new_tokens.push(Token::char(")"));
    tokens.override_range(opening_backtick_index, closing_backtick_index, new_tokens);
}
