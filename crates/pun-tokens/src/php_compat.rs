use std::cmp::Ordering;
use std::sync::LazyLock;

static ESCAPES: LazyLock<pcre2::bytes::Regex> = LazyLock::new(|| {
    pcre2::bytes::Regex::new(r"\\([\\nrtfve]|[xX][0-9a-fA-F]{1,2}|[0-7]{1,3}|u\{([0-9a-fA-F]+)\})")
        .unwrap()
});
static SUPPORTS_R: LazyLock<bool> = LazyLock::new(|| pcre2::bytes::Regex::new("(?r)").is_ok());

pub fn strlen(s: &[u8]) -> usize {
    s.len()
}

pub fn strtolower(s: &[u8]) -> Vec<u8> {
    s.to_ascii_lowercase()
}

pub fn unescape_phpdoc_string(quoted: &[u8]) -> Vec<u8> {
    let inner = &quoted[1..quoted.len() - 1];
    if quoted[0] == b'\'' {
        return str_replace(b"\\'", b"'", &str_replace(b"\\\\", b"\\", inner));
    }
    let inner = str_replace(b"\\\"", b"\"", inner);
    let mut out = Vec::with_capacity(inner.len());
    let mut from = 0;
    for found in ESCAPES.captures_iter(&inner) {
        let found = found.unwrap();
        let full = found.get(0).unwrap();
        let escape = found.get(1).unwrap().as_bytes();
        out.extend_from_slice(&inner[from..full.start()]);
        match escape {
            b"\\" => out.push(b'\\'),
            b"n" => out.push(b'\n'),
            b"r" => out.push(b'\r'),
            b"t" => out.push(b'\t'),
            b"f" => out.push(0x0c),
            b"v" => out.push(0x0b),
            b"e" => out.push(0x1b),
            hex if matches!(hex[0], b'x' | b'X') => {
                out.push(u8::from_str_radix(std::str::from_utf8(&hex[1..]).unwrap(), 16).unwrap());
            }
            unicode if unicode[0] == b'u' => {
                let code = u32::from_str_radix(
                    std::str::from_utf8(found.get(2).unwrap().as_bytes()).unwrap(),
                    16,
                )
                .unwrap_or(u32::MAX);
                match code {
                    0..=0x7f => out.push(code as u8),
                    0x80..=0x7ff => out
                        .extend_from_slice(&[(code >> 6) as u8 + 0xc0, (code & 0x3f) as u8 + 0x80]),
                    0x800..=0xffff => out.extend_from_slice(&[
                        (code >> 12) as u8 + 0xe0,
                        ((code >> 6) & 0x3f) as u8 + 0x80,
                        (code & 0x3f) as u8 + 0x80,
                    ]),
                    0x10000..=0x1fffff => out.extend_from_slice(&[
                        (code >> 18) as u8 + 0xf0,
                        ((code >> 12) & 0x3f) as u8 + 0x80,
                        ((code >> 6) & 0x3f) as u8 + 0x80,
                        (code & 0x3f) as u8 + 0x80,
                    ]),
                    _ => out.extend_from_slice("�".as_bytes()),
                }
            }
            octal => {
                out.push(
                    u32::from_str_radix(std::str::from_utf8(octal).unwrap(), 8).unwrap() as u8,
                );
            }
        }
        from = full.end();
    }
    out.extend_from_slice(&inner[from..]);
    out
}

pub fn mb_strtolower(s: &[u8]) -> Vec<u8> {
    mb_convert_case(s, str::to_lowercase)
}

pub fn mb_strtoupper(s: &[u8]) -> Vec<u8> {
    mb_convert_case(s, str::to_uppercase)
}

fn mb_convert_case(s: &[u8], convert: fn(&str) -> String) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    for chunk in s.utf8_chunks() {
        out.extend_from_slice(convert(chunk.valid()).as_bytes());
        if !chunk.invalid().is_empty() {
            out.push(b'?');
        }
    }
    out
}

fn sign(o: Ordering) -> i32 {
    o as i32
}

pub fn strcasecmp(a: &[u8], b: &[u8]) -> i32 {
    for (x, y) in a.iter().zip(b) {
        let (x, y) = (x.to_ascii_lowercase(), y.to_ascii_lowercase());
        if x != y {
            return i32::from(x) - i32::from(y);
        }
    }
    sign(a.len().cmp(&b.len()))
}

pub fn strnatcmp(a: &[u8], b: &[u8]) -> i32 {
    strnatcmp_ex(a, b, false)
}

pub fn strnatcasecmp(a: &[u8], b: &[u8]) -> i32 {
    strnatcmp_ex(a, b, true)
}

pub const TRIM_CHARS: &[u8] = b" \t\n\r\0\x0B";

pub fn rtrim(s: &[u8]) -> &[u8] {
    rtrim_chars(s, TRIM_CHARS)
}

pub fn rtrim_chars<'a>(s: &'a [u8], chars: &[u8]) -> &'a [u8] {
    let end = s
        .iter()
        .rposition(|c| !chars.contains(c))
        .map_or(0, |i| i + 1);
    &s[..end]
}

pub fn ltrim_chars<'a>(s: &'a [u8], chars: &[u8]) -> &'a [u8] {
    let start = s.iter().position(|c| !chars.contains(c)).unwrap_or(s.len());
    &s[start..]
}

pub fn str_replace(search: &[u8], replace: &[u8], subject: &[u8]) -> Vec<u8> {
    if search.is_empty() {
        return subject.to_vec();
    }
    let mut out = Vec::with_capacity(subject.len());
    let mut i = 0;
    while i < subject.len() {
        if subject[i..].starts_with(search) {
            out.extend_from_slice(replace);
            i += search.len();
        } else {
            out.push(subject[i]);
            i += 1;
        }
    }
    out
}

pub fn strpos(haystack: &[u8], needle: &[u8], offset: usize) -> Option<usize> {
    let Some((&first, tail)) = needle.split_first() else {
        return Some(offset);
    };
    let mut at = offset;
    loop {
        at += haystack[at..].iter().position(|&c| c == first)?;
        if haystack[at + 1..].starts_with(tail) {
            return Some(at);
        }
        at += 1;
    }
}

pub fn stripos(haystack: &[u8], needle: &[u8], offset: usize) -> Option<usize> {
    if needle.is_empty() {
        return Some(offset);
    }
    haystack[offset..]
        .windows(needle.len())
        .position(|w| w.eq_ignore_ascii_case(needle))
        .map(|p| p + offset)
}

pub fn strrpos(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let last = haystack.len().checked_sub(needle.len())?;
    (0..=last)
        .rev()
        .find(|&i| haystack[i..].starts_with(needle))
}

pub fn substr(s: &[u8], start: usize, length: Option<usize>) -> &[u8] {
    let start = start.min(s.len());
    let end = length.map_or(s.len(), |l| start.saturating_add(l).min(s.len()));
    &s[start..end]
}

pub fn substr_count(haystack: &[u8], needle: &[u8]) -> usize {
    assert!(
        !needle.is_empty(),
        "substr_count(): Argument #2 ($needle) cannot be empty"
    );
    let (mut n, mut i) = (0, 0);
    while let Some(p) = strpos(haystack, needle, i) {
        n += 1;
        i = p + needle.len();
    }
    n
}

pub fn explode<'a>(separator: &[u8], s: &'a [u8]) -> Vec<&'a [u8]> {
    assert!(
        !separator.is_empty(),
        "explode(): Argument #1 ($separator) cannot be empty"
    );
    let mut parts = Vec::new();
    let mut start = 0;
    while let Some(p) = strpos(s, separator, start) {
        parts.push(&s[start..p]);
        start = p + separator.len();
    }
    parts.push(&s[start..]);
    parts
}

pub fn trim(s: &[u8]) -> &[u8] {
    ltrim_chars(rtrim(s), TRIM_CHARS)
}

pub fn isspace(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}

fn strnatcmp_ex(a: &[u8], b: &[u8], fold: bool) -> i32 {
    if a.is_empty() || b.is_empty() {
        return sign(a.len().cmp(&b.len()));
    }
    let at = |s: &[u8], i: usize| *s.get(i).unwrap_or(&0);
    let (aend, bend) = (a.len(), b.len());
    let (mut ap, mut bp) = (0usize, 0usize);
    let (mut ca, mut cb) = (a[0], b[0]);
    while ca == b'0' && ap + 1 < aend && a[ap + 1].is_ascii_digit() {
        ap += 1;
        ca = a[ap];
    }
    while cb == b'0' && bp + 1 < bend && b[bp + 1].is_ascii_digit() {
        bp += 1;
        cb = b[bp];
    }
    loop {
        while isspace(ca) {
            ap += 1;
            ca = at(a, ap);
        }
        while isspace(cb) {
            bp += 1;
            cb = at(b, bp);
        }
        if ca.is_ascii_digit() && cb.is_ascii_digit() {
            let fractional = ca == b'0' || cb == b'0';
            let result = if fractional {
                compare_left(a, &mut ap, b, &mut bp)
            } else {
                compare_right(a, &mut ap, b, &mut bp)
            };
            if result != 0 {
                return result;
            } else if ap == aend && bp == bend {
                return 0;
            } else if ap == aend {
                return -1;
            } else if bp == bend {
                return 1;
            }
            ca = at(a, ap);
            cb = at(b, bp);
        }
        if fold {
            (ca, cb) = (ca.to_ascii_uppercase(), cb.to_ascii_uppercase());
        }
        if ca < cb {
            return -1;
        } else if ca > cb {
            return 1;
        }
        ap += 1;
        bp += 1;
        if ap >= aend && bp >= bend {
            return 0;
        } else if ap >= aend {
            return -1;
        } else if bp >= bend {
            return 1;
        }
        ca = a[ap];
        cb = b[bp];
    }
}

fn digit_at(s: &[u8], i: usize) -> bool {
    i < s.len() && s[i].is_ascii_digit()
}

fn compare_right(a: &[u8], ap: &mut usize, b: &[u8], bp: &mut usize) -> i32 {
    let mut bias = 0;
    loop {
        let (da, db) = (digit_at(a, *ap), digit_at(b, *bp));
        if !da && !db {
            return bias;
        } else if !da {
            return -1;
        } else if !db {
            return 1;
        } else if a[*ap] < b[*bp] {
            if bias == 0 {
                bias = -1;
            }
        } else if a[*ap] > b[*bp] && bias == 0 {
            bias = 1;
        }
        *ap += 1;
        *bp += 1;
    }
}

fn compare_left(a: &[u8], ap: &mut usize, b: &[u8], bp: &mut usize) -> i32 {
    loop {
        let (da, db) = (digit_at(a, *ap), digit_at(b, *bp));
        if !da && !db {
            return 0;
        } else if !da {
            return -1;
        } else if !db {
            return 1;
        } else if a[*ap] < b[*bp] {
            return -1;
        } else if a[*ap] > b[*bp] {
            return 1;
        }
        *ap += 1;
        *bp += 1;
    }
}

pub fn preg_quote(s: &[u8], delimiter: Option<u8>) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    for &b in s {
        match b {
            0 => out.extend_from_slice(b"\\000"),
            b'.' | b'\\' | b'+' | b'*' | b'?' | b'[' | b'^' | b']' | b'$' | b'(' | b')' | b'{'
            | b'}' | b'=' | b'!' | b'>' | b'<' | b'|' | b':' | b'-' | b'#' => {
                out.extend([b'\\', b]);
            }
            _ if Some(b) == delimiter => out.extend([b'\\', b]),
            _ => out.push(b),
        }
    }
    out
}

pub fn fnmatch(pattern: &[u8], string: &[u8]) -> bool {
    use std::ffi::CString;
    CString::new(pattern)
        .ok()
        .zip(CString::new(string).ok())
        // SAFETY: both are valid NUL-terminated C strings for the duration of the call.
        .is_some_and(|(p, s)| unsafe { libc::fnmatch(p.as_ptr(), s.as_ptr(), 0) == 0 })
}

/// PHP's `glob($pattern)` without flags: the C library's sorted matches, or an empty list when
/// nothing matches. Without `GLOB_ERR`, the C library skips unreadable folders, so PHP's `false`
/// comes only from a pattern with a NUL byte (a `ValueError` in PHP) or from no memory.
pub fn glob(pattern: &[u8]) -> Vec<Vec<u8>> {
    use std::ffi::{CStr, CString};
    let Ok(pattern) = CString::new(pattern) else {
        return Vec::new();
    };
    // SAFETY: glob_t is a plain C struct, and all zeros is its empty state.
    let mut found: libc::glob_t = unsafe { std::mem::zeroed() };
    // SAFETY: pattern is NUL-terminated, and found is a valid glob_t that globfree releases below.
    unsafe { libc::glob(pattern.as_ptr(), 0, None, &raw mut found) };
    let paths = (0..found.gl_pathc)
        .map(|i| {
            // SAFETY: glob filled gl_pathv with gl_pathc NUL-terminated strings (none on no match).
            let path = unsafe { CStr::from_ptr(*found.gl_pathv.add(i)) };
            path.to_bytes().to_vec()
        })
        .collect();
    // SAFETY: found came from one glob call and is released once.
    unsafe { libc::globfree(&raw mut found) };
    paths
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PregModifiers {
    pub caseless: bool,
    pub multi_line: bool,
    pub dotall: bool,
    pub extended: bool,
    pub anchored: bool,
    pub dollar_end_only: bool,
    pub ungreedy: bool,
    pub utf: bool,
    pub dup_names: bool,
    pub no_auto_capture: bool,
    pub caseless_restrict: bool,
}

pub fn preg_parse(pattern: &[u8]) -> Result<(&[u8], PregModifiers), String> {
    let start = pattern.iter().position(|&c| !isspace(c));
    let Some(start) = start else {
        return Err("Empty regular expression".into());
    };
    let delimiter = pattern[start];
    if delimiter.is_ascii_alphanumeric() || delimiter == b'\\' || delimiter == 0 {
        return Err("Delimiter must not be alphanumeric, backslash, or NUL byte".into());
    }
    let end_delimiter = match delimiter {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        d => d,
    };
    let (mut p, mut depth) = (start + 1, 1);
    while p < pattern.len() {
        let c = pattern[p];
        if c == b'\\' && p + 1 < pattern.len() {
            p += 1;
        } else if c == end_delimiter {
            depth -= 1;
            if depth == 0 {
                break;
            }
        } else if c == delimiter {
            depth += 1;
        }
        p += 1;
    }
    if p >= pattern.len() {
        let end = String::from_utf8_lossy(&[end_delimiter]).into_owned();
        return Err(if end_delimiter == delimiter {
            format!("No ending delimiter '{end}' found")
        } else {
            format!("No ending matching delimiter '{end}' found")
        });
    }
    let mut m = PregModifiers::default();
    for &c in &pattern[p + 1..] {
        match c {
            b'i' => m.caseless = true,
            b'm' => m.multi_line = true,
            b's' => m.dotall = true,
            b'x' => m.extended = true,
            b'A' => m.anchored = true,
            b'D' => m.dollar_end_only = true,
            b'U' => m.ungreedy = true,
            b'u' => m.utf = true,
            b'J' => m.dup_names = true,
            b'n' => m.no_auto_capture = true,
            b'r' if *SUPPORTS_R => {
                m.caseless_restrict = true;
            }
            b'S' | b'X' | b' ' | b'\n' | b'\r' => {}
            0 => return Err("NUL byte is not a valid modifier".into()),
            c => {
                let c = String::from_utf8_lossy(&[c]).into_owned();
                return Err(format!("Unknown modifier '{c}'"));
            }
        }
    }
    Ok((&pattern[start + 1..p], m))
}

pub fn preg_compile(pattern: &str) -> Result<pcre2::bytes::Regex, String> {
    let (body, m) = preg_parse(pattern.as_bytes())?;
    let body = String::from_utf8_lossy(body);
    let mut b = pcre2::bytes::RegexBuilder::new();
    b.caseless(m.caseless)
        .multi_line(m.multi_line)
        .dotall(m.dotall)
        .extended(m.extended);
    if m.utf {
        b.utf(true).ucp(true);
    }
    let flags = [
        (m.ungreedy, "(?U)"),
        (m.dup_names, "(?J)"),
        (m.no_auto_capture, "(?n)"),
        (m.caseless_restrict, "(?r)"),
    ]
    .iter()
    .filter(|f| f.0)
    .map(|f| f.1)
    .collect::<String>();
    let mut verbs = 0;
    while let Some(rest) = body[verbs..].strip_prefix("(*")
        && let Some(end) = rest.find(')')
        && rest[..end]
            .bytes()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || b"_=".contains(&c))
        && !["ACCEPT", "FAIL", "F", "COMMIT", "PRUNE", "SKIP", "THEN"].contains(&&rest[..end])
    {
        verbs += end + 3;
    }
    let (verbs, body) = body.split_at(verbs);
    let build = |pattern: String| {
        b.build(&pattern).map_err(|e| {
            let at = e.offset().unwrap_or(0);
            let at = if at <= verbs.len() {
                at
            } else {
                at - flags.len()
            };
            let text = e.to_string();
            let msg = text
                .split_once("pattern at offset ")
                .and_then(|(_, r)| r.split_once(": "))
                .map_or(text.as_str(), |(_, m)| m);
            format!("Compilation failed: {msg} at offset {at}")
        })
    };
    let re = build(format!("{verbs}{flags}{body}"))?;
    if !m.anchored {
        return Ok(re);
    }
    let unquote = if open_quote(body) { "\\E" } else { "" };
    let close = if m.extended { "\n)" } else { ")" };
    build(format!("{verbs}{flags}\\G(?:{body}{unquote}{close}"))
}

fn open_quote(body: &str) -> bool {
    let (mut quoted, mut rest) = (false, body.as_bytes());
    while let Some((&c, tail)) = rest.split_first() {
        rest = tail;
        match (c, quoted, tail.first()) {
            (b'\\', true, Some(b'E')) => quoted = false,
            (b'\\', false, next) => quoted = next == Some(&b'Q'),
            _ => continue,
        }
        rest = &tail[1..];
    }
    quoted
}

pub fn var_export_string(s: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len() + 2);
    out.push(b'\'');
    for &b in s {
        match b {
            b'\'' | b'\\' => out.extend([b'\\', b]),
            0 => out.extend_from_slice(b"' . \"\\0\" . '"),
            _ => out.push(b),
        }
    }
    out.push(b'\'');
    out
}

pub fn float_to_string(f: f64) -> String {
    if f.is_nan() {
        return "NAN".into();
    }
    if f.is_infinite() {
        return if f > 0.0 { "INF" } else { "-INF" }.into();
    }
    sprintf_g(f, 14)
}

pub fn sprintf_g(value: f64, precision: usize) -> String {
    if value == 0.0 {
        return if value.is_sign_negative() { "-0" } else { "0" }.to_string();
    }
    let scientific = format!("{:.*e}", precision - 1, value);
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let exponent: i32 = exponent.parse().unwrap_or(0);
    let (sign, mantissa) = match mantissa.strip_prefix('-') {
        Some(m) => ("-", m),
        None => ("", mantissa),
    };
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    if exponent < -4 || exponent >= precision as i32 {
        let digits = digits.trim_end_matches('0');
        let (first, rest) = digits.split_at(1);
        let rest = if rest.is_empty() { "0" } else { rest };
        let exponent_sign = if exponent < 0 { '-' } else { '+' };
        return format!("{sign}{first}.{rest}E{exponent_sign}{}", exponent.abs());
    }
    let fixed = if exponent < 0 {
        format!("0.{}{digits}", "0".repeat((-exponent - 1) as usize))
    } else {
        let point = exponent as usize + 1;
        format!("{}.{}", &digits[..point], &digits[point..])
    };
    let fixed = fixed.trim_end_matches('0').trim_end_matches('.');
    format!("{sign}{fixed}")
}

pub fn filter_var_bool(v: &str) -> bool {
    ["1", "true", "on", "yes"].contains(&v.trim().to_ascii_lowercase().as_str())
}

pub fn intval(s: &str) -> i64 {
    let t = s.trim_start_matches([' ', '\t', '\n', '\r', '\x0b', '\x0c']);
    let b = t.as_bytes();
    let digits = |i: &mut usize| {
        let start = *i;
        while b.get(*i).is_some_and(u8::is_ascii_digit) {
            *i += 1;
        }
        *i > start
    };
    let mut i = usize::from(matches!(b.first(), Some(b'+' | b'-')));
    let int = digits(&mut i);
    let mut j = i;
    if b.get(j) == Some(&b'.') {
        j += 1;
        if digits(&mut j) {
            i = j;
        } else if !int {
            return 0;
        }
    } else if !int {
        return 0;
    }
    if matches!(b.get(i), Some(b'e' | b'E')) {
        let mut k = i + 1 + usize::from(matches!(b.get(i + 1), Some(b'+' | b'-')));
        if digits(&mut k) {
            i = k;
        }
    }
    if let Ok(int) = t[..i].parse::<i64>() {
        return int;
    }
    t[..i]
        .parse::<f64>()
        .map_or(0, |f| if f.is_finite() { f as i64 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strnatcasecmp_matches_php() {
        for (a, b, want) in [
            ("a", "B", -1),
            ("img12", "IMG10", 1),
            ("Laravel/x", "laravel/y", -1),
            ("b", "A", 1),
        ] {
            assert_eq!(strnatcasecmp(a.as_bytes(), b.as_bytes()), want, "{a} {b}");
        }
        assert_eq!(strnatcmp(b"a", b"B"), 1);
    }

    #[test]
    fn intval_matches_php() {
        for (s, want) in [
            ("3", 3),
            ("abc", 0),
            (" 2x", 2),
            ("-1", -1),
            ("0", 0),
            ("+4", 4),
            ("1e3", 1000),
            ("1e3x", 1000),
            ("2.9", 2),
            (".5", 0),
            ("  7 ", 7),
            ("\t5", 5),
            ("4abc", 4),
            ("1e", 1),
            ("1e+2", 100),
            ("99999999999999999999", i64::MAX),
            ("9007199254740993", 9007199254740993),
            ("1e1000", 0),
            ("-1e1000", 0),
            ("1.9e1", 19),
            ("-0", 0),
            ("", 0),
            ("-", 0),
            ("3.", 3),
            (".", 0),
        ] {
            assert_eq!(intval(s), want, "intval({s:?})");
        }
    }

    #[test]
    fn filter_var_bool_matches_php() {
        assert!(
            filter_var_bool(" Yes")
                && filter_var_bool("on")
                && filter_var_bool("1")
                && filter_var_bool("TRUE")
        );
        assert!(!filter_var_bool("0") && !filter_var_bool("y") && !filter_var_bool(""));
    }

    #[test]
    fn ascii_only_case() {
        assert_eq!(strtolower("ÉA".as_bytes()), "Éa".as_bytes());
        assert_eq!(strlen("é".as_bytes()), 2);
        assert_eq!(rtrim(b"x \n\0"), b"x");
    }

    #[test]
    fn mb_strtolower_matches_php() {
        let cases: &[(&[u8], &[u8])] = &[
            (b"\xc3\x89COLE\xff", b"\xc3\xa9cole?"),
            (b"A\xe2\x82B", b"a?b"),
            (b"\xff\xfeX", b"??x"),
            ("ΣΑΣ".as_bytes(), "σας".as_bytes()),
            ("İ".as_bytes(), b"i\xcc\x87"),
            (b"\xc3", b"?"),
            (b"AB\xf0\x9f\x98", b"ab?"),
        ];
        for (input, expected) in cases {
            assert_eq!(mb_strtolower(input), *expected, "{input:?}");
        }
    }

    #[test]
    fn mb_strtoupper_matches_php() {
        assert_eq!(
            mb_strtoupper(b"stra\xc3\x9fe_\xc3\xa9\xffx"),
            "STRASSE_É?X".as_bytes()
        );
    }

    #[test]
    fn var_export_string_matches_php() {
        let cases: &[(&[u8], &[u8])] = &[
            (b"", b"''"),
            (b"a'b\\c", b"'a\\'b\\\\c'"),
            (b"x\0y", b"'x' . \"\\0\" . 'y'"),
            (b"\0", b"'' . \"\\0\" . ''"),
            (b"\xff\xc3\xa9", b"'\xff\xc3\xa9'"),
            (b"\\Foo\\Bar<", b"'\\\\Foo\\\\Bar<'"),
        ];
        for (input, expected) in cases {
            assert_eq!(var_export_string(input), *expected, "{input:?}");
        }
    }
}
