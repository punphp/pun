//! Port of `PhpCsFixer\Fixer\StringNotation\MultilineStringToHeredocFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::preg_quote;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct MultilineStringToHeredocFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(MultilineStringToHeredocFixer)
}

const UNESCAPE_DOUBLE: &str = r#"~(\\\\)|\\(")~"#;

impl Fixer for MultilineStringToHeredocFixer {
    fn name(&self) -> &'static str {
        "multiline_string_to_heredoc"
    }
    fn priority(&self) -> i32 {
        // Must run before EscapeImplicitBackslashesFixer, HeredocIndentationFixer, StringImplicitBackslashesFixer.
        16
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_CONSTANT_ENCAPSED_STRING, T_ENCAPSED_AND_WHITESPACE])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut complex_string_start_index = None;
        // `foreach ($tokens ...)` reads the live size, so tokens inserted on the way are visited too.
        let mut index = 0;
        while index < tokens.count() {
            match complex_string_start_index {
                None => {
                    if tokens[index].is_kind(T_CONSTANT_ENCAPSED_STRING) {
                        convert_string_to_heredoc(tokens, index, index);
                    } else if tokens[index]
                        .equals_any(&["\"".into(), "b\"".into(), "B\"".into()], true)
                    {
                        complex_string_start_index = Some(index);
                    }
                }
                Some(start) => {
                    if tokens[index].equals("\"", true) {
                        convert_string_to_heredoc(tokens, start, index);
                        complex_string_start_index = None;
                    }
                }
            }
            index += 1;
        }
    }
}

fn convert_string_to_heredoc(tokens: &mut Tokens, start: usize, end: usize) {
    let mut closing_marker = b"EOD".to_vec();
    let (content, is_single_quoted, constant_string_token) =
        if tokens[start].is_kind(T_CONSTANT_ENCAPSED_STRING) {
            let mut content = tokens[start].get_content();
            if content[0].eq_ignore_ascii_case(&b'b') {
                content = &content[1..];
            }
            let is_single_quoted = content.starts_with(b"'");
            let content = &content[1..content.len() - 1];
            let content = if is_single_quoted {
                Preg::replace(r"~\\([\\'])~", "$1", content)
            } else {
                Preg::replace(UNESCAPE_DOUBLE, "$1$2", content)
            };
            let token = Token::new(T_ENCAPSED_AND_WHITESPACE, [&content[..], b"\n"].concat());
            (content, is_single_quoted, Some(token))
        } else {
            (
                tokens.generate_partial_code(start + 1, end - 1),
                false,
                None,
            )
        };

    if !content.contains(&b'\n') && !content.contains(&b'\r') {
        return;
    }

    loop {
        let pattern = [
            &b"~(^|[\\r\\n])\\s*"[..],
            &preg_quote(&closing_marker, Some(b'~')),
            b"(?!\\w)~",
        ]
        .concat();
        if Preg::match_groups_uncached(pattern, &content).is_none() {
            break;
        }
        closing_marker.push(b'_');
    }

    let quoting: &[u8] = if is_single_quoted { b"'" } else { b"" };
    let heredoc_start = Token::new(
        T_START_HEREDOC,
        [&b"<<<"[..], quoting, &closing_marker, quoting, b"\n"].concat(),
    );
    let heredoc_end = Token::new(T_END_HEREDOC, closing_marker);

    if let Some(constant_string_token) = constant_string_token {
        tokens.override_range(
            start,
            end,
            vec![heredoc_start, constant_string_token, heredoc_end],
        );
        return;
    }
    for i in start + 1..end {
        if tokens[i].is_kind(T_ENCAPSED_AND_WHITESPACE) {
            let content = Preg::replace(UNESCAPE_DOUBLE, "$1$2", tokens[i].get_content());
            tokens.set(i, Token::new(T_ENCAPSED_AND_WHITESPACE, content));
        }
    }
    tokens.set(start, heredoc_start);
    tokens.set(end, heredoc_end);
    if tokens[end - 1].is_kind(T_ENCAPSED_AND_WHITESPACE) {
        let content = [tokens[end - 1].get_content(), b"\n"].concat();
        tokens.set(end - 1, Token::new(T_ENCAPSED_AND_WHITESPACE, content));
    } else {
        tokens.insert_at(end, vec![Token::new(T_ENCAPSED_AND_WHITESPACE, "\n")]);
    }
}
