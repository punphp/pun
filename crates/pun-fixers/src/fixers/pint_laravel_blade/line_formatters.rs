//! Ports of Pint's line-based Prettier post-formatters: `StripSensitiveLeadingBlankLines`,
//! `JoinDanglingCloseBracket`, `JoinDanglingOpenBracket`, `DedentHuggedTerminator`, `CollapseShortSlots`.
#![allow(clippy::needless_range_loop, reason = "ports Pint's index loops over `$lines` as written")]
use super::php::{join_lines, lines, ltrim, rtrim, strpos, trim};
use crate::analyzers::preg::RawPreg;

/// `StripSensitiveLeadingBlankLines`: drops the blank lines prettier injects after a wrapped `<pre>`/`<textarea>` tag.
pub mod strip_sensitive_leading_blank_lines {
    use super::*;

    const SENSITIVE: &[&[u8]] = &[b"pre", b"textarea"];

    pub fn post_format(content: &[u8]) -> Vec<u8> {
        let lines = lines(content);
        let line_count = lines.len();
        let mut result: Vec<Vec<u8>> = Vec::new();
        let mut index = 0;
        while index < line_count {
            result.push(lines[index].clone());
            if opens_inline_sensitive_tag(&lines[index]) {
                index += 1;
                while index < line_count && trim(&lines[index]).is_empty() {
                    index += 1;
                }
                continue;
            }
            let Some(terminator) = opens_wrapped_sensitive_tag(&lines, line_count, index) else {
                index += 1;
                continue;
            };
            for tag_line in index + 1..=terminator {
                result.push(lines[tag_line].clone());
            }
            index = terminator + 1;
            while index < line_count && trim(&lines[index]).is_empty() {
                index += 1;
            }
        }
        join_lines(&result)
    }

    fn opens_inline_sensitive_tag(line: &[u8]) -> bool {
        let trimmed = trim(line);
        if !RawPreg::r#match("/^<(pre|textarea)\\b/i", trimmed) {
            return false;
        }
        match open_tag_end(trimmed) {
            Some(end) if end == trimmed.len() - 1 && !(end > 0 && trimmed[end - 1] == b'/') => {}
            _ => return false,
        }
        // A "@name" directive (not "@@" escaped, nor an "@" glued to a word).
        RawPreg::r#match("/(?<![\\w@])@[a-zA-Z]/", trimmed)
    }

    fn open_tag_end(trimmed: &[u8]) -> Option<usize> {
        let length = trimmed.len();
        let mut offset = 0;
        let mut depth: usize = 0;
        while offset < length {
            let c = trimmed[offset];
            if c == b'"' || c == b'\'' {
                let close = strpos(trimmed, &[c], offset + 1)?;
                offset = close + 1;
                continue;
            }
            // A ">" inside a directive's argument list — most commonly the ">"
            // of a "=>" inside "@class([...])" — is not the tag terminator.
            if c == b'(' || c == b'[' || c == b'{' {
                depth += 1;
            } else if c == b')' || c == b']' || c == b'}' {
                depth = depth.saturating_sub(1);
            } else if c == b'>' && depth == 0 {
                return Some(offset);
            }
            offset += 1;
        }
        None
    }

    fn opens_wrapped_sensitive_tag(
        lines: &[Vec<u8>],
        line_count: usize,
        index: usize,
    ) -> Option<usize> {
        let m = RawPreg::match_groups("/^<([A-Za-z][A-Za-z0-9]*)$/", trim(&lines[index]))?;
        if !SENSITIVE.contains(&&m.str(1).to_ascii_lowercase()[..]) {
            return None;
        }
        for tag_line in index + 1..line_count {
            let trimmed = trim(&lines[tag_line]);
            if trimmed == b">" {
                return Some(tag_line);
            }
            if trimmed == b"/>" {
                return None;
            }
        }
        None
    }
}

/// `JoinDanglingCloseBracket`: joins a closing tag's dangling ">" back onto the preceding line.
pub mod join_dangling_close_bracket {
    use super::*;

    pub fn post_format(content: &[u8]) -> Vec<u8> {
        let lines = lines(content);
        let line_count = lines.len();
        let mut result: Vec<Vec<u8>> = Vec::new();
        let mut index = 0;
        while index < line_count {
            let line = &lines[index];
            let dangles_close_bracket = lines.get(index + 1).is_some_and(|next| {
                trim(next) == b">"
                    && RawPreg::r#match("/<\\/[A-Za-z][A-Za-z0-9:_.\\-]*$/", rtrim(line))
            });
            if dangles_close_bracket {
                result.push([rtrim(line), b">"].concat());
                index += 2;
                continue;
            }
            result.push(line.clone());
            index += 1;
        }
        join_lines(&result)
    }
}

/// `JoinDanglingOpenBracket`: pulls a hugged opening terminator back up onto its bare tag name line.
pub mod join_dangling_open_bracket {
    use super::*;

    pub fn post_format(content: &[u8]) -> Vec<u8> {
        let lines = lines(content);
        let line_count = lines.len();
        let mut result: Vec<Vec<u8>> = Vec::new();
        let mut index = 0;
        while index < line_count {
            let line = &lines[index];
            if let Some(m) = RawPreg::match_groups("/^<([A-Za-z][A-Za-z0-9:_.\\-]*)$/", trim(line))
                && index + 1 < line_count
            {
                let tag = m.str(1);
                let next_line = ltrim(&lines[index + 1]);
                let closing = [&b"</"[..], tag, b">"].concat();
                if !next_line.is_empty() && next_line[0] == b'>' && next_line.ends_with(&closing) {
                    result.push([rtrim(line), next_line].concat());
                    index += 2;
                    continue;
                }
            }
            result.push(line.clone());
            index += 1;
        }
        join_lines(&result)
    }
}

/// `DedentHuggedTerminator`: re-indents a hugged opening terminator line to match its tag.
pub mod dedent_hugged_terminator {
    use super::*;

    pub fn post_format(content: &[u8]) -> Vec<u8> {
        let lines = lines(content);
        let line_count = lines.len();
        let mut result: Vec<Vec<u8>> = Vec::new();
        let mut index = 0;
        while index < line_count {
            let line = &lines[index];
            let terminator = RawPreg::match_groups("/^<([A-Za-z][A-Za-z0-9:_.\\-]*)$/", trim(line))
                .and_then(|m| find_hugged_terminator(&lines, line_count, index + 1, m.str(1)));
            let Some(terminator) = terminator else {
                result.push(line.clone());
                index += 1;
                continue;
            };
            let indent = &line[..line.len() - ltrim(line).len()];
            result.push(line.clone());
            for attribute in index + 1..terminator {
                result.push(lines[attribute].clone());
            }
            result.push([indent, ltrim(&lines[terminator])].concat());
            index = terminator + 1;
        }
        join_lines(&result)
    }

    fn find_hugged_terminator(
        lines: &[Vec<u8>],
        line_count: usize,
        start: usize,
        tag: &[u8],
    ) -> Option<usize> {
        let closing_tag = [&b"</"[..], tag, b">"].concat();
        for index in start..line_count {
            let trimmed = trim(&lines[index]);
            if trimmed.is_empty() || trimmed[0] == b'<' {
                return None;
            }
            if trimmed[0] == b'>' {
                return trimmed.ends_with(&closing_tag).then_some(index);
            }
        }
        None
    }
}

/// `CollapseShortSlots`: collapses a short single-body `<x-slot>` back onto one line.
pub mod collapse_short_slots {
    use super::*;

    const PRINT_WIDTH: usize = 120;

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

    fn try_collapse(
        lines: &[Vec<u8>],
        line_count: usize,
        index: usize,
    ) -> Option<(Vec<u8>, usize)> {
        let line = &lines[index];
        let trimmed = ltrim(line);
        if !opens_slot(trimmed) {
            return None;
        }
        let indent = &line[..line.len() - trimmed.len()];
        let tag_end = open_tag_end(trimmed)?;
        if trimmed[tag_end - 1] == b'/' {
            return None;
        }
        if !rtrim(&trimmed[tag_end + 1..]).is_empty() {
            return None;
        }
        if index + 2 >= line_count {
            return None;
        }
        let body = trim(&lines[index + 1]);
        let closing_tag = ltrim(&lines[index + 2]);
        if !closes_slot(closing_tag) {
            return None;
        }
        if !body_qualifies(body) {
            return None;
        }
        let collapsed = [indent, trimmed, body, rtrim(closing_tag)].concat();
        if collapsed.len() > PRINT_WIDTH {
            return None;
        }
        Some((collapsed, index + 3))
    }

    fn opens_slot(trimmed: &[u8]) -> bool {
        let Some(rest) = trimmed.strip_prefix(b"<x-slot") else {
            return false;
        };
        rest.is_empty() || matches!(rest[0], b'>' | b':' | b' ' | b'\t')
    }

    fn closes_slot(trimmed: &[u8]) -> bool {
        RawPreg::r#match("/^<\\/x-slot[:>]/", trimmed)
    }

    fn open_tag_end(trimmed: &[u8]) -> Option<usize> {
        let length = trimmed.len();
        let mut offset = 0;
        while offset < length {
            let c = trimmed[offset];
            if c == b'"' || c == b'\'' {
                let end = strpos(trimmed, &[c], offset + 1)?;
                offset = end + 1;
                continue;
            }
            if c == b'>' {
                return Some(offset);
            }
            offset += 1;
        }
        None
    }

    fn body_qualifies(body: &[u8]) -> bool {
        !body.is_empty() && !body.contains(&b'<') && !body.contains(&b'@')
    }
}
