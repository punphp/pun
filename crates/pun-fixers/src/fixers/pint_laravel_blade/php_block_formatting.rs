//! Port of `App\PrettierFormatters\PhpBlockFormatting` (runs Pint over the PHP in @php blocks, <?php
//! islands, directive arguments, echoes and bound component attributes) and `App\Support\PhpFragmentFormatter`.
use super::php::{contains, ltrim_chars, rtrim, strpos, strrpos, trim, trim_chars};
use crate::analyzers::preg::RawPreg;
use pun_format::{Fixer, Pipeline};
use pun_tokens::Tokens;
use std::cell::OnceCell;
use std::path::Path;

/// `PhpFragmentFormatter::DENYLIST` / `FRAGMENT_DENYLIST`.
const DENYLIST: &[&str] = &["no_closing_tag", "Pint/laravel_blade"];
const FRAGMENT_DENYLIST: &[&str] = &[
    "fully_qualified_strict_types",
    "no_unused_imports",
    "declare_strict_types",
];

/// `PhpFragmentFormatter`: the active preset's fixers (never the project's own overrides, like Pint's
/// sequential runner), memoized per fragment/file key.
pub struct PhpFragmentFormatter {
    preset: String,
    fragment: OnceCell<Option<Pipeline>>,
    file: OnceCell<Option<Pipeline>>,
}

impl PhpFragmentFormatter {
    pub fn new(preset: String) -> Self {
        PhpFragmentFormatter {
            preset,
            fragment: OnceCell::new(),
            file: OnceCell::new(),
        }
    }

    /// Format a PHP document that already opens with "<?php".
    pub fn format(&self, code: &[u8], fragment: bool) -> Vec<u8> {
        // Tokens::fromCode (TOKEN_PARSE) throws a CompileError on invalid code: leave it as-is.
        if pun_format::lint(code).is_err() {
            return code.to_vec();
        }
        let mut tokens = Tokens::from_code(code);
        let file = Path::new("fragment.php");
        for fixer in self.fixers(fragment).into_iter().flat_map(Pipeline::fixers) {
            // AbstractFixer::fix only applies to non-empty tokens.
            if tokens.count() == 0 || !fixer.supports(file) || !fixer.is_candidate(&tokens) {
                continue;
            }
            fixer.fix(file, &mut tokens);
            if tokens.is_changed() {
                tokens.clear_empty_tokens();
                tokens.clear_changed();
            }
        }
        tokens.generate_code()
    }

    fn fixers(&self, fragment: bool) -> Option<&Pipeline> {
        let cell = if fragment { &self.fragment } else { &self.file };
        cell.get_or_init(|| {
            let deny: Vec<&str> = if fragment {
                [DENYLIST, FRAGMENT_DENYLIST].concat()
            } else {
                DENYLIST.to_vec()
            };
            let rules: Vec<pun_config::Rule> = preset_rules(&self.preset)
                .into_iter()
                .filter(|r| !deny.contains(&r.name.as_str()))
                .collect();
            Pipeline::new(&rules, &|| {
                crate::all()
                    .into_iter()
                    .filter(|f: &Box<dyn Fixer>| f.name() != "Pint/laravel_blade")
                    .collect()
            })
            .ok()
        })
        .as_ref()
    }
}

/// `resources/presets/{$preset}.php`'s `getRules()`, read like pun_config::load reads presets.
fn preset_rules(preset: &str) -> Vec<pun_config::Rule> {
    let Some((_, json)) = pun_config::PRESETS.iter().find(|(n, _)| *n == preset) else {
        return Vec::new();
    };
    let Ok(mut rules) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(json)
    else {
        return Vec::new();
    };
    if let Some(serde_json::Value::Object(r)) = rules.remove("rules_as_configured") {
        rules = r;
    }
    rules
        .into_iter()
        .filter(|(_, v)| *v != serde_json::Value::Bool(false))
        .map(|(name, v)| pun_config::Rule {
            name,
            options: if v == serde_json::Value::Bool(true) {
                serde_json::Value::Null
            } else {
                v
            },
        })
        .collect()
}

fn control_directive(lower: &[u8]) -> Option<&'static [u8]> {
    match lower {
        b"for" => Some(b"for"),
        b"foreach" | b"forelse" => Some(b"foreach"),
        _ => None,
    }
}

const SKIP_DIRECTIVES: &[&[u8]] = &[
    b"php",
    b"media",
    b"supports",
    b"scope",
    b"keyframes",
    b"font",
    b"import",
    b"charset",
    b"namespace",
    b"page",
    b"container",
    b"layer",
    b"property",
    b"apply",
    b"tailwind",
];

const COMPONENT_PREFIXES: &[&str] = &["x", "s", "statamic", "flux", "livewire", "native"];

type Edit = (usize, usize, Vec<u8>);
type Span = (usize, usize);

fn edit_spans(edits: &[Edit]) -> Vec<Span> {
    edits.iter().map(|e| (e.0, e.0 + e.1)).collect()
}

pub struct PhpBlockFormatting {
    formatter: PhpFragmentFormatter,
}

impl PhpBlockFormatting {
    pub fn new(formatter: PhpFragmentFormatter) -> Self {
        PhpBlockFormatting { formatter }
    }

    pub fn post_format(&self, content: &[u8]) -> Vec<u8> {
        let verbatim = verbatim_spans(content);
        let mut edits = self.region_edits(content, &verbatim);
        let more = self.directive_edits(content, &edits, &verbatim);
        edits.extend(more);
        let more = self.echo_edits(content, &edits, &verbatim);
        edits.extend(more);
        let more = self.attribute_edits(content, &edits, &verbatim);
        edits.extend(more);
        edits.sort_by_key(|e| e.0); // usort is stable in PHP 8

        let mut result = Vec::with_capacity(content.len());
        let mut cursor = 0;
        for (offset, length, replacement) in edits {
            if offset < cursor {
                continue;
            }
            result.extend_from_slice(&content[cursor..offset]);
            result.extend_from_slice(&replacement);
            cursor = offset + length;
        }
        result.extend_from_slice(&content[cursor.min(content.len())..]);
        result
    }

    fn region_edits(&self, content: &[u8], verbatim: &[Span]) -> Vec<Edit> {
        let mut edits = Vec::new();
        for m in RawPreg::match_all("/@php(?![\\w(]).*?@endphp|<\\?php.*?\\?>/is", content) {
            let region = m.str(0);
            let offset = m.offset(0) as usize;
            if within(offset, verbatim) {
                continue;
            }
            edits.push((
                offset,
                region.len(),
                self.format_region(region, indent_at(content, offset)),
            ));
        }
        edits
    }

    fn directive_edits(&self, content: &[u8], regions: &[Edit], verbatim: &[Span]) -> Vec<Edit> {
        let matches = RawPreg::match_all("/@([a-zA-Z_]\\w*)\\s*\\(/", content);
        let mut spans = edit_spans(regions);
        spans.extend_from_slice(verbatim);
        let mut edits = Vec::new();
        for m in matches {
            let name = m.str(1);
            let start = m.offset(0) as usize;
            if SKIP_DIRECTIVES.contains(&&name.to_ascii_lowercase()[..]) || within(start, &spans) {
                continue;
            }
            let open = start + m.str(0).len() - 1;
            let Some(close) = match_paren(content, open) else {
                continue;
            };
            let arg = &content[open + 1..close];
            let formatted = self.format_directive_arg(name, arg, line_indent_at(content, start));
            if formatted != arg {
                edits.push((open + 1, close - open - 1, formatted));
            }
        }
        edits
    }

    fn format_directive_arg(&self, name: &[u8], arg: &[u8], indent: &[u8]) -> Vec<u8> {
        if trim(arg).is_empty() {
            return arg.to_vec();
        }
        let host = match control_directive(&name.to_ascii_lowercase()) {
            Some(keyword) => [keyword, b" (", arg, b") {}"].concat(),
            None => [b"__pint__(", arg, b");"].concat(),
        };
        let formatted = strip_php_wrapper(
            &self
                .formatter
                .format(&[b"<?php\n", &host[..], b"\n"].concat(), true),
        );
        let Some(open) = strpos(&formatted, b"(", 0) else {
            return arg.to_vec();
        };
        let Some(close) = match_paren(&formatted, open) else {
            return arg.to_vec();
        };
        reindent_arg(&formatted[open + 1..close], indent)
    }

    fn echo_edits(&self, content: &[u8], regions: &[Edit], verbatim: &[Span]) -> Vec<Edit> {
        let matches = RawPreg::match_all("/\\{\\{(?!--)(.*?)\\}\\}|\\{!!(.*?)!!\\}/s", content);
        if matches.is_empty() {
            return Vec::new();
        }
        let mut spans = edit_spans(regions);
        spans.extend_from_slice(verbatim);
        spans.extend(comment_spans(content));
        let mut edits = Vec::new();
        for m in matches {
            let region = m.str(0);
            let start = m.offset(0) as usize;
            let unescaped = m.offset(1) == -1;
            let expr = if unescaped { m.str(2) } else { m.str(1) };
            if !contains(region, b"\n") || within(start, &spans) {
                continue;
            }
            // "@{{ ... }}" is an escaped literal echo emitted verbatim, not PHP.
            if start > 0 && content[start - 1] == b'@' {
                continue;
            }
            if let Some(formatted) =
                self.format_echo(expr, line_indent_at(content, start), unescaped)
            {
                if formatted != region {
                    edits.push((start, region.len(), formatted));
                }
            }
        }
        edits
    }

    fn format_echo(&self, expr: &[u8], indent: &[u8], unescaped: bool) -> Option<Vec<u8>> {
        if trim(expr).is_empty() {
            return None;
        }
        // De-indent to column zero so Pint's output stays idempotent across runs.
        let core = deindent(trim_chars(expr, b"\r\n"));
        let formatted = strip_php_wrapper(
            &self
                .formatter
                .format(&[b"<?php\n", &core[..], b";\n"].concat(), true),
        );
        let formatted = RawPreg::replace("/;$/", "", &formatted).unwrap_or_default();
        if trim(&formatted).is_empty() {
            return None;
        }
        let inner = reindent(&formatted, &[indent, b"    "].concat());
        let (head, tail): (&[u8], &[u8]) = if unescaped {
            (b"{!!", b"!!}")
        } else {
            (b"{{", b"}}")
        };
        Some([head, b"\n", &inner, b"\n", indent, tail].concat())
    }

    fn attribute_edits(&self, content: &[u8], regions: &[Edit], verbatim: &[Span]) -> Vec<Edit> {
        let tags = component_tag_spans(content);
        if tags.is_empty() {
            return Vec::new();
        }
        let mut skip = edit_spans(regions);
        skip.extend_from_slice(verbatim);
        let mut edits = Vec::new();
        for m in RawPreg::match_all(
            "/(?<![\\w:-])(:[\\w.:-]+)\\s*=\\s*(?:\"([^\"]*)\"|'([^']*)')/",
            content,
        ) {
            let name = m.str(1);
            let start = m.offset(0) as usize;
            // "::attr" is an escaped literal attribute, not a bound expression.
            if name.starts_with(b"::") || !within(start, &tags) || within(start, &skip) {
                continue;
            }
            let (value, value_start) = if m.offset(2) != -1 {
                (m.str(2), m.offset(2))
            } else {
                (m.str(3), m.offset(3))
            };
            if let Some(formatted) = self.format_attribute_expr(value) {
                if formatted != value {
                    edits.push((value_start as usize, value.len(), formatted));
                }
            }
        }
        edits
    }

    fn format_attribute_expr(&self, expr: &[u8]) -> Option<Vec<u8>> {
        if trim(expr).is_empty() || contains(expr, b"{{") {
            return None;
        }
        let formatted = strip_php_wrapper(
            &self
                .formatter
                .format(&[b"<?php\n__pint__(", expr, b");\n"].concat(), true),
        );
        let open = strpos(&formatted, b"(", 0)?;
        let close = match_paren(&formatted, open)?;
        let inner = &formatted[open + 1..close];
        if contains(inner, b"\n") {
            None
        } else {
            Some(inner.to_vec())
        }
    }

    fn format_region(&self, region: &[u8], indent: Option<&[u8]>) -> Vec<u8> {
        let Some(indent) = indent else {
            return region.to_vec();
        };
        if let Some(m) = RawPreg::match_groups("/^@php(.*)@endphp$/is", region) {
            return self.format_php_block(m.str(1), indent);
        }
        self.format_island(region, indent)
    }

    fn format_php_block(&self, body: &[u8], indent: &[u8]) -> Vec<u8> {
        let core = dedent(trim_chars(body, b"\r\n"));
        if trim(&core).is_empty() {
            return if contains(body, b"\n") {
                b"@php @endphp".to_vec()
            } else {
                [b"@php", body, b"@endphp"].concat()
            };
        }
        let statements = strip_php_wrapper(
            &self
                .formatter
                .format(&[b"<?php\n", &core[..], b"\n"].concat(), true),
        );
        if !contains(body, b"\n") && !contains(&statements, b"\n") {
            return [b"@php ", &statements[..], b" @endphp"].concat();
        }
        let reindented = reindent(&statements, &[indent, b"    "].concat());
        [b"@php\n", &reindented[..], b"\n", indent, b"@endphp"].concat()
    }

    fn format_island(&self, region: &[u8], indent: &[u8]) -> Vec<u8> {
        let formatted = rtrim(&self.formatter.format(region, false)).to_vec();
        if indent.is_empty() {
            return formatted;
        }
        reindent_arg(&formatted, indent)
    }
}

/// `explode("\n")->map(index 0 or "" unchanged, else $indent.$line)->implode("\n")`.
fn reindent_arg(inner: &[u8], indent: &[u8]) -> Vec<u8> {
    let lines: Vec<Vec<u8>> = inner
        .split(|&b| b == b'\n')
        .enumerate()
        .map(|(index, line)| {
            if index == 0 || line.is_empty() {
                line.to_vec()
            } else {
                [indent, line].concat()
            }
        })
        .collect();
    lines.join(&b'\n')
}

fn component_tag_spans(content: &[u8]) -> Vec<Span> {
    let pattern = format!("/<(?:{})(?:-|:)[\\w.:-]*/i", COMPONENT_PREFIXES.join("|"));
    RawPreg::match_all(pattern, content)
        .iter()
        .map(|m| {
            let start = m.offset(0) as usize;
            (start, tag_end(content, start + m.str(0).len()))
        })
        .collect()
}

fn tag_end(content: &[u8], cursor: usize) -> usize {
    let mut quote: Option<u8> = None;
    for (offset, &c) in content.iter().enumerate().skip(cursor) {
        if let Some(q) = quote {
            if c == q {
                quote = None;
            }
            continue;
        }
        if c == b'"' || c == b'\'' {
            quote = Some(c);
        } else if c == b'>' {
            return offset;
        }
    }
    content.len()
}

fn comment_spans(content: &[u8]) -> Vec<Span> {
    RawPreg::match_all("/\\{\\{--.*?--\\}\\}/s", content)
        .iter()
        .map(|m| (m.offset(0) as usize, m.offset(0) as usize + m.str(0).len()))
        .collect()
}

fn verbatim_spans(content: &[u8]) -> Vec<Span> {
    RawPreg::match_all("/@verbatim\\b.*?@endverbatim/is", content)
        .iter()
        .map(|m| (m.offset(0) as usize, m.offset(0) as usize + m.str(0).len()))
        .collect()
}

fn match_paren(content: &[u8], open: usize) -> Option<usize> {
    let mut depth: isize = 0;
    let mut quote: Option<u8> = None;
    let length = content.len();
    let mut offset = open;
    while offset < length {
        let c = content[offset];
        if let Some(q) = quote {
            if c == b'\\' {
                offset += 1;
            } else if c == q {
                quote = None;
            }
            offset += 1;
            continue;
        }
        if c == b'\'' || c == b'"' {
            quote = Some(c);
        } else if c == b'(' {
            depth += 1;
        } else if c == b')' {
            depth -= 1;
            if depth == 0 {
                return Some(offset);
            }
        }
        offset += 1;
    }
    None
}

fn within(offset: usize, spans: &[Span]) -> bool {
    spans
        .iter()
        .any(|&(start, end)| offset >= start && offset < end)
}

fn line_start(content: &[u8], offset: usize) -> usize {
    strrpos(&content[..offset], b"\n").map_or(0, |p| p + 1)
}

fn line_indent_at(content: &[u8], offset: usize) -> &[u8] {
    let rest = &content[line_start(content, offset)..];
    let n = rest
        .iter()
        .take_while(|&&c| c == b' ' || c == b'\t')
        .count();
    &rest[..n]
}

fn indent_at(content: &[u8], offset: usize) -> Option<&[u8]> {
    let prefix = &content[line_start(content, offset)..offset];
    if trim(prefix).is_empty() {
        Some(prefix)
    } else {
        None
    }
}

fn strip_php_wrapper(formatted: &[u8]) -> Vec<u8> {
    rtrim(&RawPreg::replace("/^<\\?php\\s*\\n/", "", formatted).unwrap_or_default()).to_vec()
}

fn deindent(text: &[u8]) -> Vec<u8> {
    let lines: Vec<&[u8]> = text
        .split(|&b| b == b'\n')
        .map(|line| ltrim_chars(line, b" \t"))
        .collect();
    lines.join(&b'\n')
}

fn dedent(text: &[u8]) -> Vec<u8> {
    let lines: Vec<&[u8]> = text.split(|&b| b == b'\n').collect();
    let shortest = lines
        .iter()
        .filter(|line| !trim(line).is_empty())
        .map(|line| line.len() - ltrim_chars(line, b" \t").len())
        .min()
        .unwrap_or(0);
    if shortest == 0 {
        return text.to_vec();
    }
    let lines: Vec<&[u8]> = lines
        .into_iter()
        .map(|line| {
            if trim(line).is_empty() {
                line
            } else {
                &line[shortest.min(line.len())..]
            }
        })
        .collect();
    lines.join(&b'\n')
}

fn reindent(text: &[u8], prefix: &[u8]) -> Vec<u8> {
    let lines: Vec<Vec<u8>> = text
        .split(|&b| b == b'\n')
        .map(|line| {
            if line.is_empty() {
                line.to_vec()
            } else {
                [prefix, line].concat()
            }
        })
        .collect();
    lines.join(&b'\n')
}
