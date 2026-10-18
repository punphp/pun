//! `php_compat` against the real PHP functions: PHP's output is the expectation.

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
            echo strcmp($a,$b) <=> 0, ' ', strcasecmp($a,$b), ' ', strnatcmp($a,$b), ' ', strnatcasecmp($a,$b), "\n";
        foreach ($in as $a) echo json_encode([ltrim($a, " \t0a"), rtrim($a, " \t7"), rtrim($a), trim($a), str_replace("a", "XY", $a), str_replace("00", "0", $a)]), "\n";
    "#;
    let json = format!(
        "[{}]",
        INPUTS
            .iter()
            .map(|s| format!("{:?}", s))
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
        let want: Vec<String> = serde_json::from_str(lines.next().expect("php output")).unwrap();
        let a = a.as_bytes();
        let got = [
            ltrim_chars(a, b" \t0a"),
            rtrim_chars(a, b" \t7"),
            rtrim(a),
            trim(a),
            &str_replace(b"a", b"XY", a),
            &str_replace(b"00", b"0", a),
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
