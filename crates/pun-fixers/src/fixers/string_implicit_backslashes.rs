//! Port of `PhpCsFixer\Fixer\StringNotation\StringImplicitBackslashesFixer`, with PHP-CS-Fixer 3's defaults
//! (`single_quoted: unescape`, `double_quoted: escape`, `heredoc: escape`), as Pint runs without future mode.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{rtrim, strtolower, substr};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
enum Strategy {
    Escape,
    Unescape,
    Ignore,
}

const STRATEGIES: &[(&str, Strategy)] = &[
    ("escape", Strategy::Escape),
    ("unescape", Strategy::Unescape),
    ("ignore", Strategy::Ignore),
];

struct StringImplicitBackslashesFixer {
    single_quoted: Strategy,
    double_quoted: Strategy,
    heredoc: Strategy,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(StringImplicitBackslashesFixer {
        single_quoted: Strategy::Unescape,
        double_quoted: Strategy::Escape,
        heredoc: Strategy::Escape,
    })
}

const SINGLE_QUOTED_RESERVED_REGEX: &str = r"['\\]";
const DOUBLE_QUOTED_RESERVED_REGEX: &str = r#"(?:[efnrtv$"\\0-7]|x[0-9A-Fa-f]|u{|$)"#;
const HEREDOC_SYNTAX_RESERVED_REGEX: &str = r"(?:[efnrtv$\\0-7]|x[0-9A-Fa-f]|u{|$)";

impl Fixer for StringImplicitBackslashesFixer {
    fn name(&self) -> &'static str {
        "string_implicit_backslashes"
    }
    fn priority(&self) -> i32 {
        15
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_ENCAPSED_AND_WHITESPACE, T_CONSTANT_ENCAPSED_STRING])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &["single_quoted", "double_quoted", "heredoc"],
        )?;
        self.single_quoted = o.choice("single_quoted", "unescape", STRATEGIES)?;
        self.double_quoted = o.choice("double_quoted", "escape", STRATEGIES)?;
        self.heredoc = o.choice("heredoc", "escape", STRATEGIES)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut double_quote_opened = false;
        for index in 0..tokens.count() {
            let token = &tokens[index];
            if token.equals_any(&["\"".into(), "b\"".into(), "B\"".into()], true) {
                double_quote_opened = !double_quote_opened;
            }
            if !token.is_given_kind(&[T_ENCAPSED_AND_WHITESPACE, T_CONSTANT_ENCAPSED_STRING]) {
                continue;
            }
            let content = token.get_content();
            if !content.contains(&b'\\') {
                continue;
            }
            // nowdoc syntax
            if token.is_kind(T_ENCAPSED_AND_WHITESPACE)
                && rtrim(tokens[index - 1].get_content()).ends_with(b"'")
            {
                continue;
            }
            let first_two_characters = strtolower(substr(content, 0, Some(2)));
            let is_single_quoted_string = token.is_kind(T_CONSTANT_ENCAPSED_STRING)
                && (content[0] == b'\'' || first_two_characters == b"b'");
            let is_double_quoted_string = (token.is_kind(T_CONSTANT_ENCAPSED_STRING)
                && (content[0] == b'"' || first_two_characters == b"b\""))
                || (token.is_kind(T_ENCAPSED_AND_WHITESPACE) && double_quote_opened);
            let (strategy, reserved_regex) = if is_single_quoted_string {
                (self.single_quoted, SINGLE_QUOTED_RESERVED_REGEX)
            } else if is_double_quoted_string {
                (self.double_quoted, DOUBLE_QUOTED_RESERVED_REGEX)
            } else {
                (self.heredoc, HEREDOC_SYNTAX_RESERVED_REGEX)
            };
            let new_content = match strategy {
                Strategy::Ignore => continue,
                Strategy::Escape => Preg::replace(
                    format!(r"/(?<!\\)\\((?:\\\\)*)(?!{reserved_regex})/"),
                    r"\\\\$1",
                    content,
                ),
                Strategy::Unescape => Preg::replace(
                    format!(r"/(?<!\\)\\\\((?:\\\\)*)(?!{reserved_regex})/"),
                    r"\\$1",
                    content,
                ),
            };
            if new_content != content {
                let id = token.get_id().unwrap();
                tokens.set(index, Token::new(id, new_content));
            }
        }
    }
}
