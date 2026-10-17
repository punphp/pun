//! pun-lsp driven over stdio like an editor. Formatting expectations come from the real Pint
//! (`php .oracle/pint/builds/pint --stdin-filename=… -`; those checks are skipped without the oracle), syntax error
//! text from PHP's `token_get_all(…, TOKEN_PARSE)`, and the completion / definition answers were checked by hand on
//! the fixture project below.
use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, Instant};

fn oracle() -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.oracle/pint/builds/pint");
    p.exists().then(|| p.canonicalize().unwrap())
}

/// A Composer project (`App\` in app/) with these files.
fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let d = std::env::temp_dir().join(format!("pun-lsp-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    std::fs::write(
        d.join("composer.json"),
        r#"{"autoload":{"psr-4":{"App\\":"app/"}}}"#,
    )
    .unwrap();
    for (f, text) in files {
        std::fs::create_dir_all(d.join(f).parent().unwrap()).unwrap();
        std::fs::write(d.join(f), text).unwrap();
    }
    d.canonicalize().unwrap()
}

/// Pint's stdin output for `code` named `name` in `dir`.
fn pint(dir: &Path, name: &str, code: &str) -> String {
    let mut c = Command::new("php")
        .args([
            oracle().unwrap().to_str().unwrap(),
            &format!("--stdin-filename={name}"),
            "-",
        ])
        .current_dir(dir)
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap())
        .env("HOME", dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    c.stdin.take().unwrap().write_all(code.as_bytes()).unwrap();
    String::from_utf8(c.wait_with_output().unwrap().stdout).unwrap()
}

struct Server {
    child: Child,
    stdin: ChildStdin,
    out: Receiver<Value>,
    id: i64,
    root: PathBuf,
    /// Notifications seen while waiting for responses.
    seen: Vec<Value>,
}

impl Server {
    /// Initialized with these client capabilities.
    fn start(root: &Path, capabilities: Value) -> Server {
        let mut c = Command::new(env!("CARGO_BIN_EXE_pun-lsp"));
        c.current_dir("/")
            .env_clear()
            .env("PATH", std::env::var("PATH").unwrap())
            .env("HOME", root);
        if let Ok(v) = std::env::var("LLVM_PROFILE_FILE") {
            c.env("LLVM_PROFILE_FILE", v);
        }
        let mut child = c
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let stdin = child.stdin.take().unwrap();
        let mut stdout = BufReader::new(child.stdout.take().unwrap());
        let (tx, out) = channel();
        std::thread::spawn(move || {
            loop {
                let mut len = 0;
                loop {
                    let mut line = String::new();
                    if stdout.read_line(&mut line).unwrap() == 0 {
                        return;
                    }
                    match line.trim_end() {
                        "" => break,
                        h => {
                            len = h
                                .strip_prefix("Content-Length: ")
                                .map_or(len, |n| n.parse().unwrap())
                        }
                    }
                }
                let mut body = vec![0; len];
                stdout.read_exact(&mut body).unwrap();
                tx.send(serde_json::from_slice(&body).unwrap()).unwrap();
            }
        });
        let mut s = Server {
            child,
            stdin,
            out,
            id: 0,
            root: root.to_owned(),
            seen: vec![],
        };
        let init = s.request(
            "initialize",
            json!({"capabilities": capabilities, "rootUri": s.uri("")}),
        );
        let caps = &init["result"]["capabilities"];
        // Merged: PHPantom's providers, Pun's formatting, full sync, no range formatting, Pun's fix-all kind.
        assert_eq!(
            (
                caps["completionProvider"].is_object(),
                caps["definitionProvider"].clone()
            ),
            (true, json!(true)),
            "{caps}"
        );
        assert_eq!(
            (
                caps["textDocumentSync"]["change"].clone(),
                caps["documentFormattingProvider"].clone()
            ),
            (json!(1), json!(true))
        );
        assert_eq!(caps.get("documentRangeFormattingProvider"), None);
        assert!(
            caps["codeActionProvider"]["codeActionKinds"]
                .as_array()
                .unwrap()
                .contains(&json!("source.fixAll.pun")),
            "{caps}"
        );
        s.notify("initialized", json!({}));
        s
    }

    fn uri(&self, rel: &str) -> String {
        format!("file://{}/{rel}", self.root.display())
    }

    fn send(&mut self, msg: Value) {
        let body = serde_json::to_vec(&msg).unwrap();
        write!(self.stdin, "Content-Length: {}\r\n\r\n", body.len()).unwrap();
        self.stdin.write_all(&body).unwrap();
        self.stdin.flush().unwrap();
    }

    fn notify(&mut self, method: &str, params: Value) {
        self.send(json!({"jsonrpc": "2.0", "method": method, "params": params}));
    }

    /// The next message from the server, answering its own requests (progress tokens, registrations) on the way.
    fn next(&mut self) -> Value {
        let m = self
            .out
            .recv_timeout(Duration::from_secs(120))
            .expect("a message from the server");
        if m.get("method").is_some() && m.get("id").is_some() {
            self.send(json!({"jsonrpc": "2.0", "id": m["id"], "result": null}));
        } else if m.get("method").is_some() {
            self.seen.push(m.clone());
        }
        m
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        self.id += 1;
        let id = self.id;
        self.send(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}));
        loop {
            let m = self.next();
            if m.get("method").is_none() && m["id"] == id {
                return m;
            }
        }
    }

    /// The first diagnostics published for `rel` that satisfy `done`.
    fn diagnostics(&mut self, rel: &str, done: impl Fn(&[Value]) -> bool) -> Vec<Value> {
        let uri = self.uri(rel);
        let is = |m: &Value| {
            m["method"] == "textDocument/publishDiagnostics" && m["params"]["uri"] == uri.as_str()
        };
        let mut i = 0;
        loop {
            while i < self.seen.len() {
                let m = &self.seen[i];
                i += 1;
                if is(m) && done(m["params"]["diagnostics"].as_array().unwrap()) {
                    return m["params"]["diagnostics"].as_array().unwrap().clone();
                }
            }
            self.next();
        }
    }

    fn open(&mut self, rel: &str, text: &str) {
        let uri = self.uri(rel);
        self.notify(
            "textDocument/didOpen",
            json!({"textDocument": {"uri": uri, "languageId": "php", "version": 1, "text": text}}),
        );
    }

    fn at(&mut self, method: &str, rel: &str, line: u32, character: u32) -> Value {
        let uri = self.uri(rel);
        self.request(method, json!({"textDocument": {"uri": uri}, "position": {"line": line, "character": character}}))
    }

    /// shutdown + exit: a clean exit 0.
    fn stop(mut self) {
        assert_eq!(self.request("shutdown", Value::Null)["result"], Value::Null);
        self.notify("exit", Value::Null);
        assert_eq!(self.child.wait().unwrap().code(), Some(0));
    }
}

const GREETER: &str = "<?php\n\nnamespace App;\n\nclass Greeter\n{\n    public function hello(): string\n    {\n        return 'hi';\n    }\n}\n";

#[test]
fn phpantom_answers_completion_and_definition() {
    let dir = project("intel", &[("app/Greeter.php", GREETER)]);
    // PHPantom only reports indexing progress once the editor's reply to window/workDoneProgress/create is back.
    let mut s = Server::start(&dir, json!({"window": {"workDoneProgress": true}}));
    let ended = |s: &Server| {
        s.seen
            .iter()
            .any(|m| m["method"] == "$/progress" && m["params"]["value"]["kind"] == "end")
    };
    while !ended(&s) {
        s.next();
    }
    s.open(
        "app/Use.php",
        "<?php\n\nnamespace App;\n\n$g = new Greeter();\n$g->\n",
    );
    // `$g->|`: Greeter's method.
    let r = s.at("textDocument/completion", "app/Use.php", 5, 4);
    let items = r["result"]
        .get("items")
        .unwrap_or(&r["result"])
        .as_array()
        .unwrap()
        .clone();
    let hello = items
        .iter()
        .find(|i| i["label"].as_str().unwrap().starts_with("hello"))
        .unwrap_or_else(|| panic!("{r}"));
    assert_eq!(hello["kind"], 2, "a method: {hello}");
    // `new Gre|eter()`: the class declaration, line 5 of Greeter.php.
    let r = s.at("textDocument/definition", "app/Use.php", 4, 12);
    let loc = r["result"]
        .as_array()
        .map_or(&r["result"], |a| &a[0])
        .clone();
    assert_eq!(loc["uri"], s.uri("app/Greeter.php"), "{r}");
    assert_eq!(loc["range"]["start"]["line"], 4, "{r}");
    s.stop();
}

#[test]
fn pun_diagnostics_merge_with_phpantom() {
    let dir = project("diag", &[]);
    // A client that supports pull diagnostics still gets them pushed.
    let mut s = Server::start(
        &dir,
        json!({"textDocument": {"diagnostic": {"dynamicRegistration": false}}}),
    );
    // PHP's message (token_get_all TOKEN_PARSE: `syntax error, unexpected token ";"`, line 3), on that whole line,
    // instead of PHPantom's own syntax error; its other findings stay.
    s.open("app/Bad.php", "<?php\nnew Missing();\n$a = ;\n");
    let d = s.diagnostics("app/Bad.php", |d| {
        d.iter().any(|d| d["source"] == "pun") && d.iter().any(|d| d["code"] == "unknown_class")
    });
    let syntax: Vec<&Value> = d
        .iter()
        .filter(|d| d["severity"] == 1 && d["code"] != "unknown_class")
        .collect();
    assert_eq!(
        syntax,
        [
            &json!({"range": {"start": {"line": 2, "character": 0}, "end": {"line": 2, "character": 6}}, "severity": 1, "code": "syntax",
            "source": "pun", "message": "Parse error: syntax error, unexpected token \";\" on line 3."})
        ],
        "{d:?}"
    );

    // A Mago rule (`$request->all()`, the call's span: line 10, 15..30) next to PHPantom's own finding (Request is
    // unknown: no vendor/), published together.
    let code = "<?php\n\nnamespace App\\Http;\n\nuse Illuminate\\Http\\Request;\n\nclass Store\n{\n    public function store(Request $request): array\n    {\n        return $request->all();\n    }\n}\n";
    s.open("app/Http/Store.php", code);
    let d = s.diagnostics("app/Http/Store.php", |d| {
        d.iter().any(|d| d["code"] == "unknown_class") && d.iter().any(|d| d["source"] == "pun")
    });
    let mago = d
        .iter()
        .find(|d| d["code"] == "no-request-all")
        .unwrap_or_else(|| panic!("{d:?}"));
    assert_eq!(
        mago["range"],
        json!({"start": {"line": 10, "character": 15}, "end": {"line": 10, "character": 30}})
    );
    assert_eq!(mago["severity"], 2);

    // Mago's Note and Help levels: information and hint.
    let model = "<?php\n\nnamespace App;\n\nuse Illuminate\\Database\\Eloquent\\Model;\n\nclass User extends Model\n{\n    protected $casts = ['a' => 'boolean'];\n}\n\nfunction v() { return view('a')->with(['x' => 1]); }\n";
    s.open("app/User.php", model);
    let d = s.diagnostics("app/User.php", |d| d.iter().any(|d| d["source"] == "pun"));
    let level = |code: &str| {
        d.iter()
            .find(|d| d["code"] == code)
            .map(|d| d["severity"].clone())
    };
    assert_eq!(
        (level("prefer-casts-method"), level("prefer-view-array")),
        (Some(json!(3)), Some(json!(4))),
        "{d:?}"
    );

    // Fixed in the editor: Pun's finding goes, PHPantom's stays.
    let uri = s.uri("app/Http/Store.php");
    let fixed = code.replace("->all()", "->only(['a'])");
    s.notify(
        "textDocument/didChange",
        json!({"textDocument": {"uri": uri, "version": 2}, "contentChanges": [{"text": fixed}]}),
    );
    s.seen.clear();
    let d = s.diagnostics("app/Http/Store.php", |d| {
        d.iter().any(|d| d["code"] == "unknown_class")
    });
    assert!(!d.iter().any(|d| d["source"] == "pun"), "{d:?}");
    // Saved, then closed: nothing left.
    s.notify(
        "textDocument/didSave",
        json!({"textDocument": {"uri": uri}}),
    );
    s.notify(
        "textDocument/didClose",
        json!({"textDocument": {"uri": uri}}),
    );
    s.seen.clear();
    s.diagnostics("app/Http/Store.php", <[Value]>::is_empty);
    s.stop();
}

#[test]
fn formatting_and_fix_all_match_pint() {
    let Some(_) = oracle() else { return };
    let dir = project(
        "fmt",
        &[
            (
                "pint.json",
                r#"{"preset":"laravel","notName":["Skip.php"]}"#,
            ),
            ("pun.toml", "[format]\npreset = \"laravel\"\nnot-name = [\"Skip.php\"]\n"),
        ],
    );
    let mut s = Server::start(&dir, json!({}));
    // One whole-document edit ending after the last character in UTF-16 units ("é" 1, "😀" 2).
    let code = "<?php\n$a = array(1,2);\n// é😀 tail";
    let want = pint(&dir, "app/A.php", code);
    assert_ne!(want, code);
    s.open("app/A.php", code);
    let uri = s.uri("app/A.php");
    let edit = json!({"range": {"start": {"line": 0, "character": 0}, "end": {"line": 2, "character": 11}}, "newText": want});
    let r = s.request(
        "textDocument/formatting",
        json!({"textDocument": {"uri": uri}, "options": {"tabSize": 4, "insertSpaces": true}}),
    );
    assert_eq!(r["result"], json!([edit]), "{r}");

    // The same edit as a fix-all action, next to whatever PHPantom offers.
    let range = json!({"start": {"line": 1, "character": 0}, "end": {"line": 1, "character": 0}});
    let r = s.request(
        "textDocument/codeAction",
        json!({"textDocument": {"uri": uri}, "range": range, "context": {"diagnostics": []}}),
    );
    let fix = r["result"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["kind"] == "source.fixAll.pun")
        .unwrap_or_else(|| panic!("{r}"));
    assert_eq!(fix["edit"]["changes"][&uri], json!([edit]));

    // Clean code: no edits and no action; excluded (notName) and unparsable files: Pint echoes them.
    let clean = s.uri("app/Clean.php");
    s.notify(
        "textDocument/didChange",
        json!({"textDocument": {"uri": uri, "version": 2}, "contentChanges": [{"text": want}]}),
    );
    assert_eq!(pint(&dir, "app/A.php", &want), want);
    for (rel, code) in [
        ("app/A.php", want.as_str()),
        ("app/Skip.php", code),
        ("app/Bad.php", "<?php\n$a = array(\n"),
    ] {
        assert_eq!(pint(&dir, rel, code), code, "{rel}");
        if rel != "app/A.php" {
            s.open(rel, code);
        }
        let u = s.uri(rel);
        assert_eq!(
            s.request(
                "textDocument/formatting",
                json!({"textDocument": {"uri": u}, "options": {"tabSize": 4, "insertSpaces": true}})
            )["result"],
            json!([]),
            "{rel}"
        );
    }
    let r = s.request(
        "textDocument/codeAction",
        json!({"textDocument": {"uri": uri}, "range": range, "context": {"diagnostics": []}}),
    );
    assert!(
        !r["result"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|a| a["kind"] == "source.fixAll.pun"),
        "{r}"
    );
    // Not open: an error, not an empty answer.
    let r = s.request(
        "textDocument/formatting",
        json!({"textDocument": {"uri": clean}, "options": {"tabSize": 4, "insertSpaces": true}}),
    );
    assert_eq!(r["error"]["message"], format!("{clean} is not open"));
    s.stop();
}

#[test]
fn unported_rules_fail_formatting_instead_of_half_fixing() {
    let dir = project(
        "unported",
        &[(
            "pun.toml",
            "[format]\npreset = \"empty\"\n[format.rules]\narray-syntax = true\nassign-null-coalescing-to-coalesce-equal = true\n",
        )],
    );
    let mut s = Server::start(&dir, json!({}));
    s.open("app/A.php", "<?php\n$a = array(1);\n");
    let uri = s.uri("app/A.php");
    let r = s.request(
        "textDocument/formatting",
        json!({"textDocument": {"uri": uri}, "options": {"tabSize": 4, "insertSpaces": true}}),
    );
    assert_eq!(r["error"]["code"], -32803, "{r}");
    assert!(
        r["error"]["message"]
            .as_str()
            .unwrap()
            .contains("not ported: assign_null_coalescing_to_coalesce_equal"),
        "{r}"
    );
    s.stop();
}

#[test]
fn stdin_eof_without_exit_ends_the_server() {
    let dir = project("eof", &[]);
    let s = Server::start(&dir, json!({}));
    let Server {
        mut child, stdin, ..
    } = s;
    drop(stdin);
    let t = Instant::now();
    assert_eq!(child.wait().unwrap().code(), Some(0));
    assert!(t.elapsed() < Duration::from_secs(30));
}
