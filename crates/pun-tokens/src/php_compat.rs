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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTS: &[&str] = &[
        "",
        "a",
        "A",
        "b",
        "B",
        "_",
        "a1",
        "a01",
        "a001",
        "a2",
        "a10",
        "a 10",
        "a  2",
        "img12",
        "img10",
        "IMG12",
        "x2-g8",
        "x2-y7",
        "x2-y08",
        "x8-y8",
        "1.010",
        "1.002",
        "1.1",
        "1.02",
        "0",
        "00",
        "007",
        "07",
        " 7",
        "7 ",
        "abc",
        "ABC",
        "Abc",
        "abcd",
        "ab c",
        "é",
        "É",
        "\u{ff}",
        "Foo\\Bar",
        "Foo\\bar",
        "foo_bar",
        "fooBar",
        "\t1",
        "1\t",
        "0a",
        "0b",
        "a0",
        "a00",
        "12345678901234567890",
        "12345678901234567891",
    ];

    /// Every pair, compared against `php` (skipped if no php binary is available).
    #[test]
    fn matches_php() {
        let script = r#"
            $in = json_decode(stream_get_contents(STDIN));
            foreach ($in as $a) foreach ($in as $b)
                echo strcmp($a,$b) <=> 0, ' ', strcasecmp($a,$b), ' ', strnatcmp($a,$b), ' ', strnatcasecmp($a,$b), "\n";
            foreach ($in as $a) echo json_encode([ltrim($a, " \t0a"), rtrim($a, " \t7"), rtrim($a), trim($a)]), "\n";
        "#;
        let json = format!(
            "[{}]",
            INPUTS
                .iter()
                .map(|s| format!("{:?}", s))
                .collect::<Vec<_>>()
                .join(",")
        );
        let mut child = std::process::Command::new("php")
            .args(["-r", script])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("php (the oracle) must be on PATH");
        use std::io::Write;
        child
            .stdin
            .take()
            .unwrap()
            .write_all(json.as_bytes())
            .unwrap();
        let out = String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap();
        let mut lines = out.lines();
        for a in INPUTS {
            for b in INPUTS {
                let (a, b) = (a.as_bytes(), b.as_bytes());
                let got = format!(
                    "{} {} {} {}",
                    strcmp(a, b),
                    strcasecmp(a, b),
                    strnatcmp(a, b),
                    strnatcasecmp(a, b)
                );
                let line = lines.next().expect("php output");
                assert_eq!(got, line, "{:?} vs {:?}", a, b);
            }
        }
        for a in INPUTS {
            let want: Vec<String> =
                serde_json::from_str(lines.next().expect("php output")).unwrap();
            let a = a.as_bytes();
            let got = [
                ltrim_chars(a, b" \t0a"),
                rtrim_chars(a, b" \t7"),
                rtrim(a),
                trim(a),
            ]
            .map(|x| String::from_utf8(x.to_vec()).unwrap());
            assert_eq!(got.to_vec(), want, "{a:?}");
        }
    }

    #[test]
    fn ascii_only_case() {
        assert_eq!(strtolower("ÉA".as_bytes()), "Éa".as_bytes());
        assert_eq!(strtoupper(b"a\xff"), b"A\xff");
        assert_eq!(strlen("é".as_bytes()), 2);
        assert_eq!(rtrim(b"x \n\0"), b"x");
    }
}
