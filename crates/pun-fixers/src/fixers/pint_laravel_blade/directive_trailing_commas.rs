//! Port of `App\PrettierFormatters\DirectiveTrailingCommas`: adds trailing commas to multiline
//! call/array arguments inside Blade directive arguments.
use super::php::{ctype_alnum, stripos, strpos};

const CONTROL_DIRECTIVES: &[&[u8]] = &[
    b"if", b"elseif", b"unless", b"while", b"for", b"foreach", b"forelse", b"switch", b"case",
];

const NON_CALL_KEYWORDS: &[&[u8]] = &[
    b"if", b"elseif", b"else", b"while", b"for", b"foreach", b"switch", b"catch", b"declare",
    b"do", b"match", b"empty",
];

struct Frame {
    is_call: bool,
    multiline: bool,
    has_content: bool,
    last_content_end: usize,
    last_content_char: u8,
}

fn new_frame(open_index: usize, is_call: bool) -> Frame {
    Frame {
        is_call,
        multiline: false,
        has_content: false,
        last_content_end: open_index + 1,
        last_content_char: 0,
    }
}

fn is_word(c: Option<u8>) -> bool {
    c.is_some_and(|c| c == b'_' || ctype_alnum(c))
}

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
        if c == b'{' && content[offset..].starts_with(b"{{--") {
            offset = strpos(content, b"--}}", offset + 4).map_or(length, |e| e + 4);
            continue;
        }
        if c == b'{' && content[offset..].starts_with(b"{!!") {
            offset = strpos(content, b"!!}", offset + 3).map_or(length, |e| e + 3);
            continue;
        }
        if c == b'{' && content[offset..].starts_with(b"{{") {
            offset = strpos(content, b"}}", offset + 2).map_or(length, |e| e + 2);
            continue;
        }
        if c == b'@' {
            offset = handle_at(content, length, offset, &mut inserts);
            continue;
        }
        offset += 1;
    }

    if inserts.is_empty() {
        return content.to_vec();
    }
    inserts.sort_unstable();
    let mut result = Vec::with_capacity(content.len() + inserts.len());
    let mut cursor = 0;
    for position in inserts {
        result.extend_from_slice(&content[cursor..position]);
        result.push(b',');
        cursor = position;
    }
    result.extend_from_slice(&content[cursor..]);
    result
}

fn handle_at(content: &[u8], length: usize, offset: usize, inserts: &mut Vec<usize>) -> usize {
    let prev = if offset > 0 {
        Some(content[offset - 1])
    } else {
        None
    };
    if prev == Some(b'@') || is_word(prev) {
        return offset + 1;
    }
    let mut cursor = offset + 1;
    while cursor < length && is_word(Some(content[cursor])) {
        cursor += 1;
    }
    if cursor == offset + 1 {
        return offset + 1;
    }
    let lower = content[offset + 1..cursor].to_ascii_lowercase();

    if lower == b"verbatim" {
        return skip_to(content, b"@endverbatim", cursor).unwrap_or(length);
    }
    if lower == b"php" {
        if cursor < length && content[cursor] == b'(' {
            return scan_directive_args(content, length, cursor, &lower, inserts);
        }
        return skip_to(content, b"@endphp", cursor).unwrap_or(length);
    }
    let mut paren = cursor;
    while paren < length && (content[paren] == b' ' || content[paren] == b'\t') {
        paren += 1;
    }
    if paren < length && content[paren] == b'(' {
        return scan_directive_args(content, length, paren, &lower, inserts);
    }
    cursor
}

fn mark(stack: &mut [Frame], end: usize, c: u8) {
    if let Some(top) = stack.last_mut() {
        top.has_content = true;
        top.last_content_end = end;
        top.last_content_char = c;
    }
}

fn mark_newlines(stack: &mut [Frame], content: &[u8], from: usize, to: usize) {
    if !content[from..to.min(content.len())].contains(&b'\n') {
        return;
    }
    for frame in stack {
        frame.multiline = true;
    }
}

fn scan_directive_args(
    content: &[u8],
    length: usize,
    start: usize,
    directive: &[u8],
    inserts: &mut Vec<usize>,
) -> usize {
    let mut stack = vec![new_frame(start, !CONTROL_DIRECTIVES.contains(&directive))];
    let mut offset = start + 1;

    while offset < length && !stack.is_empty() {
        let c = content[offset];
        if c == b'\'' || c == b'"' || c == b'`' {
            offset = skip_string(content, length, offset, c);
            mark(&mut stack, offset, content[offset - 1]);
            continue;
        }
        if c == b'<' && content[offset..].starts_with(b"<<<") {
            let from = offset;
            offset = skip_heredoc(content, length, offset);
            mark_newlines(&mut stack, content, from, offset);
            mark(&mut stack, offset, b'x');
            continue;
        }
        if c == b'/' && offset + 1 < length && content[offset + 1] == b'/' {
            offset = to_end_of_line(content, length, offset);
            continue;
        }
        if c == b'#' && !(offset + 1 < length && content[offset + 1] == b'[') {
            offset = to_end_of_line(content, length, offset);
            continue;
        }
        if c == b'/' && offset + 1 < length && content[offset + 1] == b'*' {
            let stop = strpos(content, b"*/", offset + 2).map_or(length, |e| e + 2);
            mark_newlines(&mut stack, content, offset, stop);
            offset = stop;
            continue;
        }
        if c == b'\n' {
            for frame in &mut stack {
                frame.multiline = true;
            }
            offset += 1;
            continue;
        }
        if c == b'\r' || c == b' ' || c == b'\t' {
            offset += 1;
            continue;
        }
        if c == b'(' || c == b'[' || c == b'{' {
            mark(&mut stack, offset + 1, c);
            let is_call = c == b'(' && paren_is_call(content, offset);
            stack.push(new_frame(offset, is_call));
            offset += 1;
            continue;
        }
        if c == b')' || c == b']' || c == b'}' {
            let frame = stack.pop().unwrap();
            if let Some(position) = decide_insert(content, &frame, c, offset) {
                inserts.push(position);
            }
            if !stack.is_empty() {
                mark(&mut stack, offset + 1, c);
            }
            offset += 1;
            continue;
        }
        mark(&mut stack, offset + 1, c);
        offset += 1;
    }
    offset
}

fn decide_insert(content: &[u8], frame: &Frame, close: u8, close_index: usize) -> Option<usize> {
    if !frame.multiline || !frame.has_content {
        return None;
    }
    if frame.last_content_char == b',' {
        return None;
    }
    if close == b']' {
        // Array literals always accept a trailing comma.
    } else if close == b')' {
        if !frame.is_call {
            return None;
        }
    } else {
        return None;
    }
    if !close_on_own_line(content, close_index) {
        return None;
    }
    Some(frame.last_content_end)
}

fn close_on_own_line(content: &[u8], index: usize) -> bool {
    // the first character before the close that is not a space, tab or CR is a newline (or there is none)
    content[..index]
        .iter()
        .rev()
        .find(|&&c| c != b' ' && c != b'\t' && c != b'\r')
        .is_none_or(|&c| c == b'\n')
}

fn paren_is_call(content: &[u8], paren_index: usize) -> bool {
    let mut cursor = paren_index as isize - 1;
    while cursor >= 0 && (content[cursor as usize] == b' ' || content[cursor as usize] == b'\t') {
        cursor -= 1;
    }
    // the directive's `@` comes earlier, so the start of the content is never reached
    let c = content[cursor as usize];
    if c == b']' || c == b')' {
        return true;
    }
    if is_word(Some(c)) {
        let end = cursor as usize;
        while cursor >= 0 && is_word(Some(content[cursor as usize])) {
            cursor -= 1;
        }
        let identifier = content[(cursor + 1) as usize..=end].to_ascii_lowercase();
        return !NON_CALL_KEYWORDS.contains(&&identifier[..]);
    }
    false
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

fn skip_heredoc(content: &[u8], length: usize, offset: usize) -> usize {
    let mut cursor = offset + 3;
    while cursor < length && (content[cursor] == b' ' || content[cursor] == b'\t') {
        cursor += 1;
    }
    if cursor < length && (content[cursor] == b'"' || content[cursor] == b'\'') {
        cursor += 1;
    }
    let label_start = cursor;
    while cursor < length && is_word(Some(content[cursor])) {
        cursor += 1;
    }
    let label = &content[label_start..cursor];
    if label.is_empty() {
        return offset + 3;
    }
    let mut search = cursor;
    loop {
        let Some(newline) = strpos(content, b"\n", search) else {
            return length;
        };
        let line_start = newline + 1;
        let mut probe = line_start;
        while probe < length && (content[probe] == b' ' || content[probe] == b'\t') {
            probe += 1;
        }
        if content[probe.min(length)..].starts_with(label) {
            let after = probe + label.len();
            let next = if after < length {
                content[after]
            } else {
                b'\n'
            };
            if !is_word(Some(next)) {
                return after;
            }
        }
        search = line_start;
    }
}

fn to_end_of_line(content: &[u8], length: usize, offset: usize) -> usize {
    strpos(content, b"\n", offset).unwrap_or(length)
}

fn skip_to(content: &[u8], directive: &[u8], from: usize) -> Option<usize> {
    stripos(content, directive, from).map(|p| p + directive.len())
}
