//! PHP string semantics fixers rely on: byte length, ASCII-only case mapping, and the
//! comparison functions with PHP 8's normalized -1/0/1 results. Exact ports of
//! ext/standard (string.c, strnatcmp.c) in the "C" locale PHP 8 uses.

use std::cmp::Ordering;

pub fn strlen(s: &[u8]) -> usize {
    s.len()
}

pub fn strtolower(s: &[u8]) -> Vec<u8> {
    s.to_ascii_lowercase()
}

pub fn strtoupper(s: &[u8]) -> Vec<u8> {
    s.to_ascii_uppercase()
}

/// `mb_strtolower($s)` (UTF-8): full Unicode lowercasing, with each maximal invalid UTF-8 subpart replaced by `?`.
pub fn mb_strtolower(s: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    for chunk in s.utf8_chunks() {
        out.extend_from_slice(chunk.valid().to_lowercase().as_bytes());
        if !chunk.invalid().is_empty() {
            out.push(b'?');
        }
    }
    out
}

fn sign(o: Ordering) -> i32 {
    o as i32
}

/// Sign only: PHP returns glibc memcmp's raw value here (e.g. 32 or 32769), which callers
/// (usort comparators) only ever use by sign.
pub fn strcmp(a: &[u8], b: &[u8]) -> i32 {
    sign(a.cmp(b))
}

/// zend_binary_strcasecmp: difference of the first differing lowercased bytes, else
/// the length comparison (-1/0/1).
pub fn strcasecmp(a: &[u8], b: &[u8]) -> i32 {
    for (x, y) in a.iter().zip(b) {
        let (x, y) = (x.to_ascii_lowercase(), y.to_ascii_lowercase());
        if x != y {
            return x as i32 - y as i32;
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

/// Default character list of trim()/rtrim()/ltrim().
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

/// `str_replace($search, $replace, $subject)` for one non-empty needle.
pub fn str_replace(search: &[u8], replace: &[u8], subject: &[u8]) -> Vec<u8> {
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

pub fn trim(s: &[u8]) -> &[u8] {
    ltrim_chars(rtrim(s), TRIM_CHARS)
}

fn isspace(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}

/// Port of strnatcmp_ex. Reads past the end yield NUL, as on PHP's NUL-terminated strings.
fn strnatcmp_ex(a: &[u8], b: &[u8], ci: bool) -> i32 {
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
        if ci {
            ca = ca.to_ascii_uppercase();
            cb = cb.to_ascii_uppercase();
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

/// `var_export($s, true)` of a string: single-quoted, with `\\` and `'` backslash-escaped and each NUL byte spliced
/// in as `' . "\0" . '`. Other bytes, invalid UTF-8 included, are copied as they are.
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

/// PHP's `(string) $float` (`precision` = 14, `%.14G` with PHP's `.0` and `E+` exponent spelling) for finite
/// floats (the ones JSON can hold).
pub fn float_to_string(f: f64) -> String {
    if f == 0.0 {
        return if f.is_sign_negative() { "-0" } else { "0" }.into();
    }
    let s = format!("{:.13e}", f.abs());
    let (mantissa, exp) = s.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let digits = digits.trim_end_matches('0');
    let sign = if f < 0.0 { "-" } else { "" };
    if !(-4..14).contains(&exp) {
        let (head, tail) = digits.split_at(1);
        let tail = if tail.is_empty() { "0" } else { tail };
        let esign = if exp < 0 { '-' } else { '+' };
        return format!("{sign}{head}.{tail}E{esign}{}", exp.abs());
    }
    if exp < 0 {
        return format!("{sign}0.{}{digits}", "0".repeat((-exp - 1) as usize));
    }
    let int_len = exp as usize + 1;
    if digits.len() <= int_len {
        format!("{sign}{digits}{}", "0".repeat(int_len - digits.len()))
    } else {
        format!("{sign}{}.{}", &digits[..int_len], &digits[int_len..])
    }
}

/// PHP's `intval()` on a string: the leading numeric part (whitespace, sign, digits, fraction, exponent), truncated
/// toward zero and saturating; 0 when there is none.
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
    t[..i].parse::<f64>().map_or(0, |f| f as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected values from `php -r 'echo intval($s);'` (PHP 8.4).
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
    fn ascii_only_case() {
        assert_eq!(strtolower("ÉA".as_bytes()), "Éa".as_bytes());
        assert_eq!(strtoupper(b"a\xff"), b"A\xff");
        assert_eq!(strlen("é".as_bytes()), 2);
        assert_eq!(rtrim(b"x \n\0"), b"x");
    }

    // mb_strtolower($s) on PHP 8.5.
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

    // var_export($s, true) on PHP 8.5.
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
