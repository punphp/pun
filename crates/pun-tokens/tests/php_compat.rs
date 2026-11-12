//! `php_compat` against real PHP functions and pinned upstream PCRE fixtures.
#![allow(
    clippy::unicode_not_nfc,
    reason = "the inputs test non-NFC bytes on purpose"
)]

mod common;

use pun_tokens::php_compat::*;
use std::io::Write;
use std::process::Stdio;

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

/// Every pair, compared against `php`.
#[test]
fn matches_php() {
    let script = r#"
        $in = json_decode(stream_get_contents(STDIN));
        foreach ($in as $a) foreach ($in as $b)
            echo strcasecmp($a,$b), ' ', strnatcmp($a,$b), "\n";
        foreach ($in as $a) echo json_encode([ltrim($a, " \t0a"), rtrim($a, " \t7"), rtrim($a), trim($a), str_replace("a", "XY", $a), str_replace("00", "0", $a), str_replace("", "Z", $a)]), "\n";
    "#;
    let json = format!(
        "[{}]",
        INPUTS
            .iter()
            .map(|s| format!("{s:?}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let mut child = common::php()
        .args(["-r", script])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
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
            let got = format!("{} {}", strcasecmp(a, b), strnatcmp(a, b));
            let line = lines.next().expect("php output");
            assert_eq!(got, line, "{a:?} vs {b:?}");
        }
    }
    for a in INPUTS {
        let want: Vec<String> = serde_json::from_str(lines.next().expect("php output")).unwrap();
        let a = a.as_bytes();
        let got = [
            ltrim_chars(a, b" \t0a"),
            rtrim_chars(a, b" \t7"),
            rtrim(a),
            trim(a),
            &str_replace(b"a", b"XY", a),
            &str_replace(b"00", b"0", a),
            &str_replace(b"", b"Z", a),
        ]
        .map(|x| String::from_utf8(x.to_vec()).unwrap());
        assert_eq!(got.to_vec(), want, "{a:?}");
    }
}

#[test]
fn float_to_string_matches_php() {
    let floats = [
        0.0,
        -0.0,
        1.0,
        2.0,
        -2.5,
        0.1,
        1.5,
        1e20,
        1e-5,
        0.0001,
        123456789012345.0,
        12345678901234.0,
        2f64.powi(63),
        -(2f64.powi(63)),
        1.0 / 3.0,
        2.0 / 3.0,
        1e14,
        99999999999999.5,
        1e100,
        5e-324,
        f64::MAX,
        0.30000000000000004,
        1234.5678,
    ];
    let json = serde_json::to_string(&floats.map(|f| format!("{f:e}"))).unwrap();
    let out = common::php()
        .args([
            "-r",
            "foreach (json_decode($argv[1]) as $f) echo (string) (float) $f, \"\\n\";",
            &json,
        ])
        .output()
        .unwrap();
    let want = String::from_utf8(out.stdout).unwrap();
    let got: String = floats.iter().map(|&f| float_to_string(f) + "\n").collect();
    assert_eq!(got, want);
}

/// `php` run on `script` with `input` on stdin; one JSON value per output line.
fn php_lines(script: &str, input: &serde_json::Value) -> Vec<serde_json::Value> {
    let mut child = common::php()
        .args(["-r", script])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let json = serde_json::to_vec(input).unwrap();
    child.stdin.take().unwrap().write_all(&json).unwrap();
    let out = String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap();
    out.lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

/// `false` or the offset, as PHP's search functions return it.
fn pos(p: Option<usize>) -> serde_json::Value {
    p.map_or(serde_json::Value::Bool(false), std::convert::Into::into)
}

fn utf8(b: &[u8]) -> String {
    String::from_utf8(b.to_vec()).unwrap()
}

/// strpos, stripos, strrpos, substr, substr_count, explode and preg_quote on every haystack and needle.
#[test]
fn search_split_and_quote_match_php() {
    let haystacks = [
        "",
        "a",
        "abcabc",
        "aaaa",
        "AbC aBc",
        "x::y::",
        "::",
        "a.b/c#d\0e",
    ];
    let needles = ["", "a", "bc", "aa", "B", "::", "abcabcd", "/", "\0"];
    let script = r#"
        $in = json_decode(stream_get_contents(STDIN), true);
        foreach ($in['h'] as $h) foreach ($in['n'] as $n) {
            $r = [];
            foreach ([0, 1, strlen($h)] as $o) if ($o <= strlen($h)) { $r[] = strpos($h, $n, $o); $r[] = stripos($h, $n, $o); }
            $r[] = strrpos($h, $n);
            if ($n !== '') { $r[] = substr_count($h, $n); $r[] = explode($n, $h); }
            echo json_encode($r), "\n";
        }
        foreach ($in['h'] as $h) foreach ([0, 2, 9] as $s) foreach ([null, 0, 2, 50] as $l) echo json_encode(substr($h, $s, $l)), "\n";
        foreach ($in['h'] as $h) foreach ([null, '/', '#', 'a'] as $d) echo json_encode(preg_quote($h, $d)), "\n";
    "#;
    let want = php_lines(script, &serde_json::json!({"h": haystacks, "n": needles}));
    let mut got = Vec::new();
    for h in haystacks.map(str::as_bytes) {
        for n in needles.map(str::as_bytes) {
            let mut r = Vec::new();
            for o in [0, 1, h.len()] {
                if o <= h.len() {
                    r.push(pos(strpos(h, n, o)));
                    r.push(pos(stripos(h, n, o)));
                }
            }
            r.push(pos(strrpos(h, n)));
            if !n.is_empty() {
                r.push(substr_count(h, n).into());
                r.push(explode(n, h).into_iter().map(utf8).collect());
            }
            got.push(serde_json::Value::Array(r));
        }
    }
    for h in haystacks.map(str::as_bytes) {
        for s in [0, 2, 9] {
            for l in [None, Some(0), Some(2), Some(50)] {
                got.push(utf8(substr(h, s, l)).into());
            }
        }
    }
    for h in haystacks.map(str::as_bytes) {
        for d in [None, Some(b'/'), Some(b'#'), Some(b'a')] {
            got.push(utf8(&preg_quote(h, d)).into());
        }
    }
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(&want) {
        assert_eq!(g, w);
    }
}

#[test]
#[should_panic(expected = "substr_count(): Argument #2 ($needle) cannot be empty")]
fn substr_count_refuses_an_empty_needle() {
    substr_count(b"a", b"");
}

#[test]
#[should_panic(expected = "explode(): Argument #1 ($separator) cannot be empty")]
fn explode_refuses_an_empty_separator() {
    explode(b"", b"a");
}

/// fnmatch() with no flags, against the same C library PHP calls.
#[test]
fn fnmatch_matches_php() {
    let cases = [
        ("*.php", "a.php"),
        ("*", "/a/.b"),
        ("/a/*", "/a/b/c"),
        ("[ab]?", "bc"),
        ("a\\*", "a*"),
        ("a\\*", "ab"),
        ("a[", "a["),
        ("[]a]", "]"),
        ("[!]a]", "b"),
        ("[a-]", "-"),
        ("[\\]]", "]"),
        ("[a\\]", "\\"),
        ("\\", "\\"),
        ("[[:alpha:]]", "a"),
        ("[b-a]", "a"),
        ("[!a]", "/"),
    ];
    let want = php_lines(
        r#"foreach (json_decode(stream_get_contents(STDIN)) as [$p, $s]) echo json_encode(@fnmatch($p, $s) === true), "\n";"#,
        &serde_json::json!(cases.as_slice()),
    );
    let got: Vec<serde_json::Value> = cases
        .iter()
        .map(|(p, s)| fnmatch(p.as_bytes(), s.as_bytes()).into())
        .collect();
    assert_eq!(got, want);
    // PHP throws a ValueError for a NUL byte; nothing matches here.
    assert!(!fnmatch(b"a\0", b"a\0"));
}

/// preg_match() of each pattern: whether it matched, or PHP's warning.
#[test]
fn preg_compile_matches_php() {
    let cases = [
        ("#^Foo$#", "Foo"),
        ("#^Foo$#", "foo"),
        ("#^foo$#i", "FOO"),
        ("/a.c/s", "a\nc"),
        ("/a.c/", "a\nc"),
        ("/^b$/m", "a\nb"),
        ("/a b/x", "ab"),
        ("/^.$/u", "é"),
        ("/^.$/", "é"),
        ("/b/A", "ab"),
        ("/a+?$/U", "aaa"),
        ("/(?<n>a)|(?<n>b)/J", "b"),
        ("/(a)/n", "a"),
        ("/a/SXr", "a"),
        ("/a/SX", "a"),
        ("/a/r", "a"),
        ("/a/rr", "a"),
        ("/a(/r", "a"),
        ("/a(/rZ", "a"),
        ("/a(/Zr", "a"),
        ("/^k$/iu", "\u{212A}"),
        ("/^k$/iur", "\u{212A}"),
        ("/^\\x{212A}$/iur", "k"),
        ("/^s$/iur", "ſ"),
        ("/^\\x{017F}$/iur", "s"),
        ("/^[^k]$/iur", "\u{212A}"),
        ("/^k$/iur", "K"),
        ("/^é$/iur", "É"),
        ("/(*UTF)(?i)^k$/r", "\u{212A}"),
        ("/^(?-r:k)$/iur", "\u{212A}"),
        ("/k(?^i)k/iur", "K\u{212A}"),
        ("/k(?^i)k/iur", "\u{212A}\u{212A}"),
        ("/(?r)a/", "a"),
        ("/s(?r)s(?-r)s(?r:s)s/iu", "ſSſSſ"),
        ("/(*UTF)a(/r", "a"),
        ("/(*FOO)a/r", "a"),
        ("  \n/a/ \n\r", "a"),
        ("(a(b)c)", "abc"),
        ("[a]i", "A"),
        ("{a{1}}", "a"),
        ("<a>", "a"),
        ("#a\\#b#", "a#b"),
        ("/a/b/", "a"),
        ("", "a"),
        ("   ", "a"),
        ("abc", "a"),
        ("\\a\\", "a"),
        ("/abc", "a"),
        ("(abc", "a"),
        ("/a/Z", "a"),
        ("/a/\0", "a"),
        ("/a(/", "a"),
        ("/a(/A", "a"),
        ("/x(/i", "a"),
        ("/a #c/xA", "a"),
        ("/a #c/xA", "ba"),
        ("#\\Qa)#A", "a)"),
        ("#\\Qa\\Eb#A", "ab"),
        ("#\\\\Qa#A", "\\Qa"),
        ("#(*ANYCRLF)a#A", "ba"),
        ("#(*ANYCRLF)(*UCP)a#AU", "a"),
        ("#(*FOO)a#U", "a"),
        ("#(*CR)a(#U", "a"),
        ("#(*FAIL)|a#A", "a"),
        ("#(*COMMIT)a|b#A", "b"),
    ];
    let want = php_lines(
        r#"foreach (json_decode(stream_get_contents(STDIN)) as [$p, $s]) {
            $m = @preg_match($p, $s);
            echo json_encode($m === false ? substr(error_get_last()['message'], strlen('preg_match(): ')) : $m === 1), "\n";
        }"#,
        &serde_json::json!(cases.as_slice()),
    );
    let got: Vec<serde_json::Value> = cases
        .iter()
        .map(|(p, s)| match preg_compile(p) {
            Ok(re) => re.is_match(s.as_bytes()).unwrap().into(),
            Err(e) => e.into(),
        })
        .collect();
    // `r` needs PCRE2 10.43. When this PHP's engine and Pun's disagree on it, PHP is no oracle for the `r` cases;
    // preg_caseless_restrict_matches_upstream checks them against upstream's own expectations.
    let r = cases.iter().position(|c| *c == ("/a/r", "a")).unwrap();
    let same_engine = got[r] == want[r];
    for ((g, w), c) in got.iter().zip(&want).zip(&cases) {
        if same_engine || !c.0.contains('r') {
            assert_eq!(g, w, "{c:?}");
        }
    }
    // PHP accepts `D` (PCRE2_DOLLAR_ENDONLY); the pcre2 crate can't set it, so only the parse is checked.
    assert!(preg_parse(b"/a$/D").unwrap().1.dollar_end_only);
}

#[test]
fn preg_caseless_restrict_matches_upstream() {
    // Probe raw PCRE independently of preg_parse/preg_compile: PHP and Rust can link different engines.
    let supports_r = pcre2::bytes::Regex::new("(?r)").is_ok();
    // PHP php-8.5.12 at 136e049ac10b032c83018e65b63b6df858943750:
    // ext/pcre/tests/preg_match_caseless_restrict.phpt, including its EXPECT values.
    // Exact matched substrings: PCRE2 10.43 at 3864abdb713f78831dd12d898ab31bbb0fa630b6,
    // testdata/testoutput5:5042-5107 and 5210-5215.
    let cases = [
        ("/AskZ/iur", "AskZ", Some("AskZ")),
        ("/AskZ/iur", "aSKz", Some("aSKz")),
        ("/AskZ/iur", "AſkZ", None),
        ("/AskZ/iur", "As\u{212A}Z", None),
        ("/k/iur", "\u{212A}", None),
        ("/A\\x{17f}\\x{212a}Z/iur", "AskZ", None),
        ("/[AskZ]+/iur", "AſkZ", Some("A")),
        ("/[AskZ]+/iur", "As\u{212A}Z", Some("As")),
        ("/[\\x{17f}\\x{212a}]+/iur", "AskZ", None),
        ("/[^s]+/iur", "AſZ", Some("AſZ")),
        ("/k(?^i)k/iur", "K\u{212A}", Some("K\u{212A}")),
        ("/k(?^i)k/iur", "\u{212A}\u{212A}", None),
        // Anchored, inline and ordinary-case controls follow the same ASCII/non-ASCII boundary contract:
        // PCRE2 10.43, doc/pcre2api.3:2005-2014; (?r) does not enable caseless matching itself.
        ("/^k$/iur", "\u{212A}", None),
        ("/^\\x{212A}$/iur", "k", None),
        ("/^s$/iur", "ſ", None),
        ("/^\\x{017F}$/iur", "s", None),
        ("/^[^k]$/iur", "\u{212A}", Some("\u{212A}")),
        ("/^k$/iur", "K", Some("K")),
        ("/^é$/iur", "É", Some("É")),
        ("/^k$/ur", "K", None),
        ("/a/SXr", "a", Some("a")),
        ("/a/rr", "a", Some("a")),
        ("/(*UTF)(?i)^k$/r", "\u{212A}", None),
        ("/(*UTF)(?i)^k$/r", "K", Some("K")),
        ("/(*UTF)(?i)k/rA", "aK", None),
        ("/(*UTF)(?i)k/rA", "K", Some("K")),
        ("/(*UTF)(?i)k/rA", "\u{212A}", None),
        ("/^(?-r:k)$/iur", "\u{212A}", Some("\u{212A}")),
    ];
    for (pattern, subject, want) in cases {
        if supports_r {
            assert!(preg_parse(pattern.as_bytes()).unwrap().1.caseless_restrict);
            let re = preg_compile(pattern).unwrap();
            let got = re.find(subject.as_bytes()).unwrap();
            assert_eq!(
                got.map(|m| m.as_bytes()),
                want.map(str::as_bytes),
                "{pattern}: {subject}"
            );
        } else {
            assert_eq!(
                preg_parse(pattern.as_bytes()).unwrap_err(),
                "Unknown modifier 'r'",
                "{pattern}"
            );
            assert_eq!(
                preg_compile(pattern).unwrap_err(),
                "Unknown modifier 'r'",
                "{pattern}"
            );
        }
    }
    // The PHP fixture's inline enable/disable/scoped controls: lines 55-57 and EXPECT lines 102-104.
    for (subject, want) in [("ſSſSſ", Some("ſSſSſ")), ("ſſſSſ", None), ("ſSſſſ", None)]
    {
        let re = preg_compile("/s(?r)s(?-r)s(?r:s)s/iu");
        if supports_r {
            let re = re.unwrap();
            assert_eq!(
                re.find(subject.as_bytes()).unwrap().map(|m| m.as_bytes()),
                want.map(str::as_bytes)
            );
        } else {
            // PHP 8.5.12 / PCRE2 10.42: the unknown inline option is a compile error, not a modifier error.
            assert_eq!(
                re.unwrap_err(),
                "Compilation failed: unrecognized character after (? or (?- at offset 3"
            );
        }
    }
    // PHP 8.5.12 / PCRE2 10.42 supplied these body errors without `r`; adding a supported `r` preserves them.
    for (pattern, error) in [
        (
            "/a(/r",
            "Compilation failed: missing closing parenthesis at offset 2",
        ),
        (
            "/(*UTF)a(/r",
            "Compilation failed: missing closing parenthesis at offset 8",
        ),
        (
            "/(*FOO)a/r",
            "Compilation failed: (*VERB) not recognized or malformed at offset 5",
        ),
        ("/a(/rZ", "Unknown modifier 'Z'"),
    ] {
        let want = if supports_r {
            error
        } else {
            "Unknown modifier 'r'"
        };
        assert_eq!(preg_compile(pattern).unwrap_err(), want, "{pattern}");
    }
    assert_eq!(preg_compile("/a(/Zr").unwrap_err(), "Unknown modifier 'Z'");
}

/// INF, -INF and NAN, which JSON cannot carry to PHP: `(string) INF` etc. on PHP 8.5.
#[test]
fn float_to_string_of_specials() {
    assert_eq!(float_to_string(f64::INFINITY), "INF");
    assert_eq!(float_to_string(f64::NEG_INFINITY), "-INF");
    assert_eq!(float_to_string(f64::NAN), "NAN");
}
