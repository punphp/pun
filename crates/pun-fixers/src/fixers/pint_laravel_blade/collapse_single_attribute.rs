//! Port of `App\PrettierFormatters\CollapseSingleAttribute`: collapses a tag prettier wrapped solely
//! because of one attribute.
#![allow(clippy::needless_range_loop, reason = "ports Pint's index loops over `$lines` as written")]
use super::php::{ctype_space, join_lines, lines, ltrim, rtrim, trim};
use crate::analyzers::preg::RawPreg;

pub fn post_format(content: &[u8]) -> Vec<u8> {
    let lines = lines(content);
    let line_count = lines.len();
    let mut result: Vec<Vec<u8>> = Vec::new();
    let mut index = 0;
    while index < line_count {
        if let Some((line, next_index)) = try_collapse(&lines, line_count, index) {
            result.push(line);
            index = next_index;
            continue;
        }
        result.push(lines[index].clone());
        index += 1;
    }
    join_lines(&result)
}

fn wrap(indent: &[u8], tag: &[u8], attribute: &[u8], body: &[u8]) -> Vec<u8> {
    [
        indent, b"<", tag, b" ", attribute, b">", body, b"</", tag, b">",
    ]
    .concat()
}

fn try_collapse(lines: &[Vec<u8>], line_count: usize, index: usize) -> Option<(Vec<u8>, usize)> {
    let line = &lines[index];
    let trimmed = ltrim(line);
    let indent = &line[..line.len() - trimmed.len()];

    if let Some(m) = RawPreg::match_groups("/^<([A-Za-z][A-Za-z0-9:_.\\-]*)\\s+(\\S.*)$/", trimmed)
    {
        let tag = m.str(1);
        let attribute = rtrim(m.str(2));
        if attribute_count(attribute) != 1 {
            return None;
        }
        let (body, next_index) = extract_hugged_content(lines, line_count, index + 1, tag)?;
        return Some((wrap(indent, tag, attribute, &body), next_index));
    }

    if !RawPreg::r#match("/^<[A-Za-z][A-Za-z0-9:_.\\-]*$/", trimmed) {
        return None;
    }
    let tag = &trimmed[1..];

    if let Some(wrapped) = try_collapse_wrapped_attribute(lines, line_count, index, indent, tag) {
        return Some(wrapped);
    }
    if index + 2 >= line_count {
        return None;
    }
    let attribute = trim(&lines[index + 1]);
    if attribute.is_empty() || attribute == b">" || attribute == b"/>" {
        return None;
    }
    if attribute_count(attribute) != 1 {
        return None;
    }
    let terminator = trim(&lines[index + 2]);
    if terminator == b">" || terminator == b"/>" {
        let suffix: &[u8] = if terminator == b"/>" { b" />" } else { b">" };
        return Some((
            [indent, trimmed, b" ", attribute, suffix].concat(),
            index + 3,
        ));
    }
    let (body, next_index) = extract_hugged_content(lines, line_count, index + 2, tag)?;
    Some((wrap(indent, tag, attribute, &body), next_index))
}

fn try_collapse_wrapped_attribute(
    lines: &[Vec<u8>],
    line_count: usize,
    index: usize,
    indent: &[u8],
    tag: &[u8],
) -> Option<(Vec<u8>, usize)> {
    let start = index + 1;
    if start >= line_count {
        return None;
    }
    let first_line = &lines[start];
    let trimmed_first = ltrim(first_line);
    if !opens_blade_attribute(trimmed_first) {
        return None;
    }
    let dedent_by =
        first_line.len() as isize - trimmed_first.len() as isize - indent.len() as isize;
    if dedent_by <= 0 {
        return None;
    }
    let end = wrapped_attribute_end(lines, line_count, start)?;
    if end == start || end + 1 >= line_count {
        return None;
    }
    let terminator = trim(&lines[end + 1]);
    if terminator != b">" && terminator != b"/>" {
        return None;
    }
    let suffix: &[u8] = if terminator == b"/>" { b" />" } else { b">" };
    let mut result = vec![[indent, b"<", tag, b" ", rtrim(trimmed_first)].concat()];
    for continuation in start + 1..=end {
        let mut line = dedent(&lines[continuation], dedent_by as usize).to_vec();
        if continuation == end {
            line = [rtrim(&line), suffix].concat();
        }
        result.push(line);
    }
    Some((join_lines(&result), end + 2))
}

fn opens_blade_attribute(trimmed: &[u8]) -> bool {
    trimmed.starts_with(b"@") || trimmed.starts_with(b"{{") || trimmed.starts_with(b"{!!")
}

fn wrapped_attribute_end(lines: &[Vec<u8>], line_count: usize, start: usize) -> Option<usize> {
    let mut depth: isize = 0;
    let mut quote: Option<u8> = None;
    let mut opened = false;
    for index in start..line_count {
        for &c in &lines[index] {
            if let Some(q) = quote {
                if c == q {
                    quote = None;
                }
                continue;
            }
            if c == b'"' || c == b'\'' {
                quote = Some(c);
                continue;
            }
            if c == b'(' || c == b'[' || c == b'{' {
                depth += 1;
                opened = true;
                continue;
            }
            if c == b')' || c == b']' || c == b'}' {
                depth -= 1;
                if depth < 0 {
                    return None;
                }
            }
        }
        if opened && depth == 0 {
            return Some(index);
        }
        if quote.is_some() {
            return None;
        }
    }
    None
}

fn dedent(line: &[u8], amount: usize) -> &[u8] {
    let mut strip = 0;
    while strip < amount && strip < line.len() && line[strip] == b' ' {
        strip += 1;
    }
    &line[strip..]
}

fn extract_hugged_content(
    lines: &[Vec<u8>],
    line_count: usize,
    start: usize,
    tag: &[u8],
) -> Option<(Vec<u8>, usize)> {
    if start >= line_count {
        return None;
    }
    let trimmed = trim(&lines[start]);
    if trimmed.is_empty() || trimmed[0] != b'>' {
        return None;
    }
    let closing_tag = [&b"</"[..], tag].concat();
    let closing_full = [&closing_tag[..], b">"].concat();
    // substr($trimmed, 1, $len): a negative or zero length gives ""
    let body = |len: isize| -> Vec<u8> {
        if len <= 0 {
            Vec::new()
        } else {
            trimmed[1..(1 + len as usize).min(trimmed.len())].to_vec()
        }
    };
    if trimmed.ends_with(&closing_full) {
        let len = trimmed.len() as isize - 1 - closing_tag.len() as isize - 1;
        return Some((body(len), start + 1));
    }
    if trimmed.ends_with(&closing_tag) {
        if start + 1 >= line_count || trim(&lines[start + 1]) != b">" {
            return None;
        }
        let len = trimmed.len() as isize - 1 - closing_tag.len() as isize;
        return Some((body(len), start + 2));
    }
    if start + 1 < line_count && trim(&lines[start + 1]) == &closing_full[..] {
        return Some((trimmed[1..].to_vec(), start + 2));
    }
    None
}

fn attribute_count(attributes: &[u8]) -> isize {
    let mut total = 0;
    let mut in_token = false;
    let mut depth: isize = 0;
    let mut quote: Option<u8> = None;
    let begin_token = |total: &mut isize, in_token: &mut bool, depth: isize| {
        if !*in_token && depth == 0 {
            *total += 1;
        }
        *in_token = true;
    };
    for &c in attributes {
        if let Some(q) = quote {
            if c == q {
                quote = None;
            }
            continue;
        }
        if c == b'"' || c == b'\'' {
            begin_token(&mut total, &mut in_token, depth);
            quote = Some(c);
            continue;
        }
        if c == b'(' || c == b'[' || c == b'{' {
            begin_token(&mut total, &mut in_token, depth);
            depth += 1;
            continue;
        }
        if c == b')' || c == b']' || c == b'}' {
            depth -= 1;
            if depth < 0 {
                return -1;
            }
            continue;
        }
        if depth == 0 {
            // A bare ">" closes the start tag, so it was never wrapped.
            if c == b'>' {
                return -1;
            }
            if ctype_space(c) {
                in_token = false;
                continue;
            }
        }
        begin_token(&mut total, &mut in_token, depth);
    }
    if quote.is_some() || depth != 0 {
        return -1;
    }
    total
}
