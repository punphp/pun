//! Runs PHP-CS-Fixer's own test cases (exported to crates/pun-harness/fixtures/<name>.json) against each ported fixer.
//! `PUN_FIXERS_ONLY=name cargo test -p punphp-fixers --test fixtures -- --nocapture`
//!
//! Mirrors AbstractFixerTestCase::doTest: a fresh fixer per case, configured from the `configuration` arg
//! (or a literal `configure([...])` / `$this->testFix($expected, $input, [...])` in the test body), whitespace
//! config from the `whitespacesConfig` arg or a literal `new WhitespacesFixerConfig(...)` in the body;
//! input must be a candidate and fix to expected (and mark tokens changed); expected must stay unchanged.
extern crate punphp_fixers as pun_fixers;
use pun_format::{Fixer, WhitespacesFixerConfig};
use serde_json::Value;
use std::path::{Path, PathBuf};

fn fresh(name: &str) -> Box<dyn Fixer> {
    pun_fixers::all().into_iter().find(|f| f.name() == name).unwrap()
}

/// Tiny PHP literal parser: strings, ints, bools, null, `[...]` / `array(...)` with optional keys.
struct Lit<'a>(&'a [u8], usize);

impl Lit<'_> {
    fn ws(&mut self) {
        while self.1 < self.0.len() && self.0[self.1].is_ascii_whitespace() {
            self.1 += 1;
        }
    }
    fn eat(&mut self, s: &str) -> bool {
        self.ws();
        if self.0[self.1..].starts_with(s.as_bytes()) {
            self.1 += s.len();
            true
        } else {
            false
        }
    }
    fn value(&mut self) -> Option<Value> {
        self.ws();
        let c = *self.0.get(self.1)?;
        if c == b'[' || self.0[self.1..].starts_with(b"array(") {
            let close = if c == b'[' { self.1 += 1; "]" } else { self.1 += 6; ")" };
            let (mut list, mut map, mut is_map) = (Vec::new(), serde_json::Map::new(), false);
            while !self.eat(close) {
                let v = self.value()?;
                if self.eat("=>") {
                    is_map = true;
                    let key = match v { Value::String(s) => s, v => v.to_string() };
                    map.insert(key, self.value()?);
                } else {
                    map.insert(list.len().to_string(), v.clone());
                    list.push(v);
                }
                self.eat(",");
            }
            return Some(if is_map { Value::Object(map) } else { Value::Array(list) });
        }
        if c == b'\'' || c == b'"' {
            self.1 += 1;
            let mut s = Vec::new();
            while self.0[self.1] != c {
                let mut b = self.0[self.1];
                if b == b'\\' {
                    self.1 += 1;
                    b = self.0[self.1];
                    if c == b'"' {
                        b = match b { b'n' => b'\n', b'r' => b'\r', b't' => b'\t', b'v' => 0x0B, b'0' => 0, b => b };
                    } else if b != b'\'' && b != b'\\' {
                        s.push(b'\\');
                    }
                }
                s.push(b);
                self.1 += 1;
            }
            self.1 += 1;
            return Some(Value::String(String::from_utf8(s).ok()?));
        }
        let start = self.1;
        while self.1 < self.0.len() && (self.0[self.1].is_ascii_alphanumeric() || self.0[self.1] == b'_' || self.0[self.1] == b'-') {
            self.1 += 1;
        }
        let word = std::str::from_utf8(&self.0[start..self.1]).ok()?.to_ascii_lowercase();
        match word.as_str() {
            "true" => Some(Value::Bool(true)),
            "false" => Some(Value::Bool(false)),
            "null" => Some(Value::Null),
            w => w.parse::<i64>().ok().map(Value::from),
        }
    }
}

/// The first PHP literal argument after `marker` in `body`, e.g. `configure(` -> `['a' => 1]`.
fn literal_after(body: &str, marker: &str, skip_args: usize) -> Option<Value> {
    let at = body.find(marker)? + marker.len();
    let rest = &body.as_bytes()[at..];
    let mut lit = Lit(rest, 0);
    for _ in 0..skip_args {
        // skip `$expected, ` style args
        while lit.1 < rest.len() && rest[lit.1] != b',' {
            lit.1 += 1;
        }
        lit.1 += 1;
    }
    lit.value()
}

fn whitespaces_from_body(body: &str) -> Option<WhitespacesFixerConfig> {
    // `setWhitespacesConfig(` may be followed by a line break before `new WhitespacesFixerConfig(`.
    let at = body.find("new WhitespacesFixerConfig(")? + "new WhitespacesFixerConfig(".len();
    let rest = &body.as_bytes()[at..];
    let mut lit = Lit(rest, 0);
    let indent = match lit.value() { Some(Value::String(s)) => s, _ => "    ".into() };
    lit.eat(",");
    let eol = match lit.value() { Some(Value::String(s)) => s, _ => "\n".into() };
    Some(WhitespacesFixerConfig::new(&indent, &eol).unwrap())
}

fn is_empty_config(c: &Value) -> bool {
    c.is_null() || c.as_array().is_some_and(|a| a.is_empty()) || c.as_object().is_some_and(|o| o.is_empty())
}

#[test]
fn upstream_fixtures() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../pun-harness/fixtures");
    let verbose = std::env::var("PUN_FIXTURE_VERBOSE").is_ok();
    let (mut pass, mut fail) = (0, 0);
    for fixer in pun_fixers::all() {
        let name = fixer.name();
        let Ok(raw) = std::fs::read(dir.join(format!("{}.json", name.replace('/', "_")))) else { continue };
        let doc: Value = serde_json::from_slice(&raw).unwrap();
        let (mut p, mut f) = (0, 0);
        for test in doc["tests"].as_array().into_iter().flatten() {
            let method = test["method"].as_str().unwrap_or("");
            if !method.starts_with("testFix") && method != "testWithWhitespacesConfig" { continue }
            // #[RequiresPhp('< 8.0.0')] etc.: the oracle is PHP 8.5, so such cases never run upstream either.
            if test["requires_php"].as_str().is_some_and(|r| r.starts_with('<')) { continue }
            let body = test["body"].as_str().unwrap_or("");
            let params: Vec<&str> = test["params"].as_array().into_iter().flatten().filter_map(|p| p["name"].as_str()).collect();
            let arg_index = |n: &[&str]| params.iter().position(|p| n.contains(p));
            for case in test["cases"].as_array().into_iter().flatten() {
                let args = case["args"].as_array().cloned().unwrap_or_default();
                let arg = |n: &[&str]| arg_index(n).and_then(|i| args.get(i)).cloned().unwrap_or(Value::Null);
                let mut expected = arg(&["expected"]).as_str().unwrap_or("").to_string();
                let mut input = arg(&["input"]).as_str().map(String::from);
                // `$expected = str_replace("\n", $whitespacesConfig->getLineEnding(), $expected);` (and the same for input)
                if let Some(eol) = arg(&["whitespacesConfig"]).get("lineEnding").and_then(Value::as_str) {
                    if body.contains(r#"str_replace("\n", $whitespacesConfig->getLineEnding(), $expected)"#) {
                        expected = expected.replace('\n', eol);
                        input = input.map(|i| i.replace('\n', eol));
                    }
                }
                let mut config = arg(&["configuration", "config"]);
                if is_empty_config(&config) {
                    config = literal_after(body, "->configure(", 0)
                        .or_else(|| literal_after(body, "$this->testFix(", 2))
                        .unwrap_or(Value::Null);
                }
                if is_empty_config(&config) { config = Value::Null }
                let ws = match arg(&["whitespacesConfig"]) {
                    Value::Object(o) => WhitespacesFixerConfig::new(o["indent"].as_str().unwrap(), o["lineEnding"].as_str().unwrap()).unwrap(),
                    _ => whitespaces_from_body(body).unwrap_or_default(),
                };
                let file = match arg(&["file"]) {
                    Value::Object(o) => PathBuf::from(o["pathname"].as_str().unwrap()),
                    _ => PathBuf::from("test.php"),
                };
                let mut fixer = fresh(name);
                if let Err(e) = fixer.configure(&config) {
                    f += 1;
                    if verbose { eprintln!("--- {name} {method} case {}: configure({config}) failed: {e}", case["key"]) }
                    continue;
                }
                fixer.set_whitespaces_config(&ws);
                let supported = fixer.supports(&file);
                let mut problems = Vec::new();
                let mut got_text = Vec::new();
                if let Some(input) = &input {
                    let mut tokens = pun_tokens::Tokens::from_code(input.as_bytes());
                    if supported {
                        if !fixer.is_candidate(&tokens) { problems.push("not a candidate for input") }
                        fixer.fix(&file, &mut tokens);
                    }
                    let got = tokens.generate_code();
                    if got != expected.as_bytes() { problems.push("input did not fix to expected") }
                    if !tokens.is_changed() { problems.push("tokens not marked changed") }
                    got_text = got;
                }
                let mut tokens = pun_tokens::Tokens::from_code(expected.as_bytes());
                // AbstractFixer::fix: applyFix only for non-empty, candidate, supported input.
                if supported && tokens.count() > 0 && fixer.is_candidate(&tokens) { fixer.fix(&file, &mut tokens); }
                let got2 = tokens.generate_code();
                if got2 != expected.as_bytes() { problems.push("expected code changed"); if input.is_none() { got_text = got2 } }
                else if tokens.is_changed() { problems.push("expected tokens marked changed") }
                if problems.is_empty() { p += 1 } else {
                    f += 1;
                    if verbose {
                        eprintln!("--- {name} {method} case {} ({}) ---\nexpected:\n{expected}\ngot:\n{}", case["key"], problems.join(", "), String::from_utf8_lossy(&got_text));
                    }
                }
            }
        }
        eprintln!("{name:40} {p:4} pass {f:4} fail");
        pass += p; fail += f;
    }
    eprintln!("TOTAL {pass} pass, {fail} fail");
    assert_eq!(fail, 0);
}
