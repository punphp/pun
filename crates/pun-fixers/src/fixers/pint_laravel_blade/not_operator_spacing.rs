//! Port of `App\PrettierFormatters\NotOperatorSpacing`: enforces Pint's "! $value" spacing inside
//! JS/Alpine/PHP attribute values.
use super::php::{ctype_alnum, ctype_alpha, stripos, strpos, substr};

const JS_ATTRIBUTE_PREFIXES: &[&[u8]] = &[b"x-", b"@", b":"];
const RAW_TEXT_ELEMENTS: &[&[u8]] = &[b"script", b"style"];

pub(super) fn post_format(content: &[u8]) -> Vec<u8> {
    let length = content.len();
    let mut offset = 0;
    let mut inserts: Vec<usize> = Vec::new();

    while offset < length {
        let c = content[offset];
        if c == b'<' && offset + 1 < length && content[offset + 1] == b'?' {
            offset = strpos(content, b"?>", offset + 2).map_or(length, |e| e + 2);
            continue;
        }
        if c == b'{' && substr(content, offset, Some(4)) == b"{{--" {
            offset = strpos(content, b"--}}", offset + 4).map_or(length, |e| e + 4);
            continue;
        }
        if c == b'{' && substr(content, offset, Some(3)) == b"{!!" {
            offset = strpos(content, b"!!}", offset + 3).map_or(length, |e| e + 3);
            continue;
        }
        if c == b'{' && substr(content, offset, Some(2)) == b"{{" {
            offset = strpos(content, b"}}", offset + 2).map_or(length, |e| e + 2);
            continue;
        }
        if c == b'<' && offset + 1 < length && starts_tag(content[offset + 1]) {
            offset = handle_tag(content, length, offset, &mut inserts);
            continue;
        }
        offset += 1;
    }

    if inserts.is_empty() {
        return content.to_vec();
    }
    inserts.sort();
    let mut result = Vec::with_capacity(content.len() + inserts.len());
    let mut cursor = 0;
    for position in inserts {
        result.extend_from_slice(&content[cursor..position]);
        result.push(b' ');
        cursor = position;
    }
    result.extend_from_slice(&content[cursor..]);
    result
}

fn handle_tag(content: &[u8], length: usize, offset: usize, inserts: &mut Vec<usize>) -> usize {
    let mut cursor = offset + 1;
    let is_closing = cursor < length && content[cursor] == b'/';
    if is_closing {
        cursor += 1;
    }
    let name_start = cursor;
    while cursor < length && is_name_char(content[cursor]) {
        cursor += 1;
    }
    let name = &content[name_start..cursor];

    while cursor < length {
        let c = content[cursor];
        if c == b'>' {
            cursor += 1;
            break;
        }
        if c == b'/' {
            cursor += 1;
            continue;
        }
        if c == b'{' && substr(content, cursor, Some(2)) == b"{{" {
            cursor = strpos(content, b"}}", cursor + 2).map_or(length, |e| e + 2);
            continue;
        }
        // A directive's "(...)" may contain ">", so consume it as a balanced group.
        if c == b'@' {
            cursor = handle_in_tag_directive(content, length, cursor, inserts);
            continue;
        }
        if is_name_start(c) {
            cursor = handle_attribute(content, length, cursor, inserts);
            continue;
        }
        cursor += 1;
    }

    if !is_closing && RAW_TEXT_ELEMENTS.contains(&&name.to_ascii_lowercase()[..]) {
        // Guard the offset: a malformed tag can leave the cursor past the content end.
        let needle = [&b"</"[..], name].concat();
        let close = if cursor < length {
            stripos(content, &needle, cursor)
        } else {
            None
        };
        return close.unwrap_or(length);
    }
    cursor
}

fn handle_attribute(
    content: &[u8],
    length: usize,
    mut cursor: usize,
    inserts: &mut Vec<usize>,
) -> usize {
    let name_start = cursor;
    while cursor < length && is_name_char(content[cursor]) {
        cursor += 1;
    }
    let name = &content[name_start..cursor];
    while cursor < length && is_space(content[cursor]) {
        cursor += 1;
    }
    if cursor >= length || content[cursor] != b'=' {
        return cursor;
    }
    cursor += 1;
    while cursor < length && is_space(content[cursor]) {
        cursor += 1;
    }
    if cursor >= length {
        return cursor;
    }
    let quote = content[cursor];
    // Only double-quoted values are scanned: prettier emits JS attributes
    // double-quoted and escapes any inner '"', so the value holds no raw '"'.
    if quote != b'"' {
        if quote == b'\'' {
            return strpos(content, b"'", cursor + 1).map_or(length, |e| e + 1);
        }
        while cursor < length && !is_space(content[cursor]) && content[cursor] != b'>' {
            cursor += 1;
        }
        return cursor;
    }
    let value_start = cursor + 1;
    let value_end = strpos(content, b"\"", value_start).unwrap_or(length);
    if is_js_attribute(name) {
        scan_value(content, value_start, value_end, inserts);
    }
    (value_end + 1).min(length)
}

fn handle_in_tag_directive(
    content: &[u8],
    length: usize,
    cursor: usize,
    inserts: &mut Vec<usize>,
) -> usize {
    let mut probe = cursor + 1;
    while probe < length && is_name_char(content[probe]) {
        probe += 1;
    }
    let mut after = probe;
    while after < length && is_space(content[after]) {
        after += 1;
    }
    if after < length && content[after] == b'=' {
        return handle_attribute(content, length, cursor, inserts);
    }
    if after < length && content[after] == b'(' {
        return skip_balanced_parens(content, length, after);
    }
    probe
}

fn skip_balanced_parens(content: &[u8], length: usize, mut cursor: usize) -> usize {
    let mut depth = 0;
    while cursor < length {
        let c = content[cursor];
        if c == b'\'' || c == b'"' || c == b'`' {
            cursor = skip_string(content, length, cursor, c);
            continue;
        }
        if c == b'(' {
            depth += 1;
        } else if c == b')' {
            depth -= 1;
            if depth == 0 {
                return cursor + 1;
            }
        }
        cursor += 1;
    }
    cursor
}

fn scan_value(content: &[u8], start: usize, end: usize, inserts: &mut Vec<usize>) {
    let mut offset = start;
    while offset < end {
        let c = content[offset];
        // A "!" inside a JS string literal is text, not an operator.
        if c == b'\'' || c == b'`' {
            offset = skip_string(content, end, offset, c).min(end);
            continue;
        }
        if c == b'{' && substr(content, offset, Some(2)) == b"{{" {
            offset = match strpos(content, b"}}", offset + 2) {
                Some(close) if close < end => close + 2,
                _ => end,
            };
            continue;
        }
        if c != b'!' {
            offset += 1;
            continue;
        }
        let next = (offset + 1 < end).then(|| content[offset + 1]);
        // "!=" is a comparison, "!!" double negation, and a space is already correct.
        match next {
            None => offset += 1,
            Some(b'!') => offset += 2,
            Some(n) if n == b'=' || is_space(n) => offset += 1,
            _ => {
                inserts.push(offset + 1);
                offset += 1;
            }
        }
    }
}

fn skip_string(content: &[u8], length: usize, mut offset: usize, quote: u8) -> usize {
    offset += 1;
    while offset < length {
        let c = content[offset];
        if c == b'\\' {
            offset += 2;
            continue;
        }
        if c == quote {
            return offset + 1;
        }
        offset += 1;
    }
    length
}

fn is_js_attribute(name: &[u8]) -> bool {
    JS_ATTRIBUTE_PREFIXES.iter().any(|p| name.starts_with(p))
}

fn starts_tag(c: u8) -> bool {
    c == b'/' || ctype_alpha(c)
}

fn is_name_start(c: u8) -> bool {
    ctype_alpha(c) || c == b'_' || c == b':' || c == b'@'
}

fn is_name_char(c: u8) -> bool {
    ctype_alnum(c) || c == b'-' || c == b'_' || c == b':' || c == b'.' || c == b'@'
}

fn is_space(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r'
}
