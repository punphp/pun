//! Port of `PhpCsFixer\Fixer\IndentationTrait` (used by braces_position, no_multiple_statements_per_line,
//! array_indentation, statement_indentation, attribute_block_no_spaces).
use super::preg::Preg;
use pun_tokens::{Kind::*, Tokens};

pub fn get_line_indentation(tokens: &Tokens, index: usize) -> Vec<u8> {
    match get_previous_newline_token_index(tokens, index) {
        None => Vec::new(),
        Some(i) => extract_indent(&compute_new_line_content(tokens, i)),
    }
}

pub fn extract_indent(content: &[u8]) -> Vec<u8> {
    if !content.is_ascii() {
        return Preg::match_groups("/\\R(\\h*)[^\\r\\n]*$/D", content)
            .map(|m| m.str(1).to_vec())
            .unwrap_or_default();
    }
    // ASCII: the leftmost (atomic) `\R` with no CR/LF after it is the one covering the last CR/LF (a CRLF pair counts
    // from its CR), else the first VT/FF; group 1 is the run of spaces and tabs after it.
    let after = match content.iter().rposition(|b| matches!(b, b'\r' | b'\n')) {
        Some(l) => l + 1,
        None => match content.iter().position(|b| matches!(b, 0x0b | 0x0c)) {
            Some(p) => p + 1,
            None => return Vec::new(),
        },
    };
    let rest = &content[after..];
    rest[..rest
        .iter()
        .position(|b| !matches!(b, b' ' | b'\t'))
        .unwrap_or(rest.len())]
        .to_vec()
}

pub fn get_previous_newline_token_index(tokens: &Tokens, mut index: usize) -> Option<usize> {
    while index > 0 {
        index = tokens.get_prev_token_of_kind(
            index,
            &[T_WHITESPACE.into(), T_INLINE_HTML.into()],
            true,
        )?;
        if is_new_line_token(tokens, index) {
            return Some(index);
        }
    }
    None
}

pub fn compute_new_line_content(tokens: &Tokens, index: usize) -> Vec<u8> {
    let content = tokens[index].get_content();
    if index != 0 && tokens[index - 1].is_given_kind(&[T_OPEN_TAG, T_CLOSE_TAG]) {
        return [
            Preg::replace("/\\S/", "", tokens[index - 1].get_content()).as_slice(),
            content,
        ]
        .concat();
    }
    content.to_vec()
}

pub fn is_new_line_token(tokens: &Tokens, index: usize) -> bool {
    let token = &tokens[index];
    if token.is_kind(T_OPEN_TAG)
        && index + 1 < tokens.count()
        && !tokens[index + 1].is_whitespace(None)
        && Preg::r#match("/\\R/", token.get_content())
    {
        return true;
    }
    if !tokens[index].is_given_kind(&[T_WHITESPACE, T_INLINE_HTML]) {
        return false;
    }
    Preg::r#match("/\\R/", compute_new_line_content(tokens, index))
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_indent_matches_php() {
        // PhpCsFixer\Preg::match('/\R(\h*)[^\r\n]*$/D', $s, $m) ? $m[1] : '' on PHP 8.5.
        let cases: &[(&str, &str)] = &[
            ("", ""),
            ("abc", ""),
            ("\n    x", "    "),
            ("\r\n\t y", "\t "),
            ("a\n  b\n    ", "    "),
            ("\x0b  a", "  "),
            ("\x0c\t\n  z", "  "),
            ("a\x0b \n\t\tq", "\t\t"),
            ("\r", ""),
            ("  \r\n", ""),
            ("x\n\x0b  y", ""),
            ("\n \x0c  \t", " "),
            ("\r\r\n  w", "  "),
            ("é\n  x", "  "),
        ];
        for &(s, want) in cases {
            assert_eq!(
                super::extract_indent(s.as_bytes()),
                want.as_bytes(),
                "{s:?}"
            );
        }
    }
}
