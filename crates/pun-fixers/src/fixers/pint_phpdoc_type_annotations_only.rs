//! Port of Pint's `App\Fixers\TypeAnnotationsOnlyFixer` (`Pint/phpdoc_type_annotations_only`).
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{TRIM_CHARS, ltrim_chars, trim};
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct TypeAnnotationsOnlyFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(TypeAnnotationsOnlyFixer)
}

impl Fixer for TypeAnnotationsOnlyFixer {
    fn name(&self) -> &'static str {
        "Pint/phpdoc_type_annotations_only"
    }
    fn priority(&self) -> i32 {
        56
    }
    fn supports(&self, file: &Path) -> bool {
        let path = file.to_string_lossy().replace('\\', "/");
        !(path.starts_with("config/") || path.contains("/config/"))
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_COMMENT, T_DOC_COMMENT])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if tokens[index].is_kind(T_COMMENT) {
                process_comment(tokens, index);
            } else if tokens[index].is_kind(T_DOC_COMMENT) {
                process_doc_comment(tokens, index);
            }
        }
    }
}

fn process_comment(tokens: &mut Tokens, index: usize) {
    let content = tokens[index].get_content().to_vec();
    if content.contains(&b'@') {
        return;
    }
    let prev = tokens.get_prev_non_whitespace(index, None);
    let next = tokens.get_next_non_whitespace(index, None);
    if let (Some(p), Some(n)) = (prev, next)
        && tokens[p].equals("{", true)
        && tokens[n].equals("}", true)
    {
        let placeholder: &[u8] = if ltrim_chars(&content, TRIM_CHARS).starts_with(b"#") {
            b"#"
        } else {
            b"//"
        };
        if trim(&content) != placeholder {
            tokens.set(index, Token::new(T_COMMENT, placeholder));
        }
        return;
    }
    clear_and_clean_whitespace(tokens, index);
}

fn process_doc_comment(tokens: &mut Tokens, index: usize) {
    let content = tokens[index].get_content().to_vec();
    if !content.contains(&b'\n') {
        if !content.contains(&b'@') {
            clear_and_clean_whitespace(tokens, index);
        }
        return;
    }
    let lines: Vec<&[u8]> = content.split(|&c| c == b'\n').collect();
    let indent = detect_indent(&lines);
    let (mut kept, mut depth) = (Vec::new(), 0i64);
    for line in lines {
        if depth > 0 || line.contains(&b'@') {
            kept.push(line);
            depth += count(line, b'{') - count(line, b'}');
        }
    }
    if kept.is_empty() {
        clear_and_clean_whitespace(tokens, index);
        return;
    }
    let mut rebuilt = b"/**\n".to_vec();
    for line in kept {
        rebuilt.extend_from_slice(line);
        rebuilt.push(b'\n');
    }
    rebuilt.extend_from_slice(indent);
    rebuilt.extend_from_slice(b"*/");
    if rebuilt != content {
        tokens.set(index, Token::new(T_DOC_COMMENT, rebuilt));
    }
}

fn count(s: &[u8], c: u8) -> i64 {
    s.iter().filter(|&&b| b == c).count() as i64
}

fn clear_and_clean_whitespace(tokens: &mut Tokens, index: usize) {
    let prev = tokens.get_prev_non_whitespace(index, None);
    let next = tokens.get_next_non_whitespace(index, None);
    let had_blank_line_before = prev.is_some_and(|p| {
        (p + 1..index)
            .any(|i| tokens[i].is_whitespace(None) && count(tokens[i].get_content(), b'\n') >= 2)
    });
    tokens.clear_token_and_merge_surrounding_whitespace(index);
    let (Some(prev), Some(next)) = (prev, next) else {
        return;
    };
    // Pint's loop always finds one: at worst the cleared token itself, which is empty and so whitespace.
    let i = (prev + 1..next)
        .find(|&i| tokens[i].is_whitespace(None))
        .unwrap();
    let ws = tokens[i].get_content();
    if count(ws, b'\n') > 1 {
        let last_newline = ws.iter().rposition(|&c| c == b'\n').unwrap();
        let mut new = if had_blank_line_before {
            b"\n".to_vec()
        } else {
            vec![]
        };
        new.extend_from_slice(&ws[last_newline..]);
        tokens.set(i, Token::new(T_WHITESPACE, new));
    }
}

/// First `/^(\s+)\*/` match's group, else a single space.
fn detect_indent<'a>(lines: &[&'a [u8]]) -> &'a [u8] {
    for line in lines {
        let n = line
            .iter()
            .take_while(|c| b" \t\x0B\x0C\r".contains(c))
            .count();
        if n > 0 && line.get(n) == Some(&b'*') {
            return &line[..n];
        }
    }
    b" "
}
