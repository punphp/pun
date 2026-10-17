//! `pun lsp` driven over stdio like an editor would. Formatted text comes from the real Pint
//! (`php .oracle/pint/builds/pint -`, the same stdin path with the document's name). Skipped without the oracle.
use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

fn oracle() -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.oracle/pint/builds/pint");
    p.exists().then(|| p.canonicalize().unwrap())
}

/// A project with `pint_json` for Pint and the pun.toml `pun migrate` writes from it.
fn project(name: &str, pint_json: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("pun-lsp-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(d.join("app")).unwrap();
    std::fs::write(d.join("pint.json"), pint_json).unwrap();
    migrate(&d);
    d.canonicalize().unwrap()
}

/// pun.toml from the current pint.json.
fn migrate(dir: &Path) {
    let _ = std::fs::remove_file(dir.join("pun.toml"));
    let out = Command::new(env!("CARGO_BIN_EXE_pun"))
        .arg("migrate")
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Pint's stdin output for `code` named `name` in `dir`. Each run gets its own TMPDIR: Pint names its stdin temp
/// file with uniqid(), so concurrent runs in one /tmp can read (and delete) each other's input.
fn pint(dir: &Path, name: &str, code: &str) -> String {
    let tmp = dir.with_extension("tmp");
    std::fs::create_dir_all(&tmp).unwrap();
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
        .env("TMPDIR", &tmp)
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
}

impl Server {
    fn start(cwd: &Path) -> Server {
        let mut c = Command::new(env!("CARGO_BIN_EXE_pun"));
        c.arg("lsp")
            .current_dir(cwd)
            .env_clear()
            .env("PATH", std::env::var("PATH").unwrap())
            .env("HOME", cwd);
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
        Server {
            child,
            stdin,
            out,
            id: 0,
        }
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

    /// The response to a new request.
    fn request(&mut self, method: &str, params: Value) -> Value {
        self.id += 1;
        self.send(json!({"jsonrpc": "2.0", "id": self.id, "method": method, "params": params}));
        let r = self
            .out
            .recv_timeout(Duration::from_secs(60))
            .expect("response");
        assert_eq!(r["id"], self.id, "{r}");
        r
    }

    fn format(&mut self, uri: &str) -> Value {
        self.request(
            "textDocument/formatting",
            json!({"textDocument": {"uri": uri}, "options": {"tabSize": 4, "insertSpaces": true}}),
        )
    }

    fn open(&mut self, uri: &str, text: &str) {
        self.notify(
            "textDocument/didOpen",
            json!({"textDocument": {"uri": uri, "languageId": "php", "version": 1, "text": text}}),
        );
    }

    /// shutdown + exit: a clean exit 0.
    fn stop(mut self) {
        assert_eq!(self.request("shutdown", Value::Null)["result"], Value::Null);
        self.notify("exit", Value::Null);
        assert_eq!(self.child.wait().unwrap().code(), Some(0));
    }
}

#[test]
fn formats_like_pint_stdin() {
    if oracle().is_none() {
        return;
    }
    let dir = project(
        "fmt",
        r#"{"preset":"empty","rules":{"array_syntax":true,"blank_line_after_opening_tag":true},"notName":["Skip.php"]}"#,
    );
    let file = |name: &str| format!("file://{}/{}", dir.display(), name.replace(' ', "%20"));
    // Started elsewhere: the workspace folder decides the project (its pun.toml).
    let mut s = Server::start(Path::new("/"));
    let init = s.request("initialize", json!({"capabilities": {}, "workspaceFolders": [{"uri": format!("file://{}", dir.display()), "name": "p"}]}));
    assert_eq!(
        init["result"]["capabilities"],
        json!({"textDocumentSync": 1, "documentFormattingProvider": true})
    );
    s.notify("initialized", json!({}));

    // One whole-document edit: the range ends after the last character, in UTF-16 units ("é" 1, "😀" 2).
    let code = "<?php\n$a = array(1,2);\n// é😀 tail";
    let want = pint(&dir, "app/A b.php", code);
    assert_ne!(want, code);
    s.open(&file("app/A b.php"), code);
    let r = s.format(&file("app/A b.php"));
    assert_eq!(
        r["result"],
        json!([{"range": {"start": {"line": 0, "character": 0}, "end": {"line": 2, "character": 11}}, "newText": want}]),
        "{r}"
    );

    // LSP line breaks are \n, \r\n and a lone \r: this document has 3 lines, the last one "// x".
    let code = "<?php\r\n$a = array(1,2);\r// x";
    let want = pint(&dir, "app/Cr.php", code);
    assert_ne!(want, code);
    s.open(&file("app/Cr.php"), code);
    assert_eq!(
        s.format(&file("app/Cr.php"))["result"],
        json!([{"range": {"start": {"line": 0, "character": 0}, "end": {"line": 2, "character": 4}}, "newText": want}])
    );

    // After a full-text change to code Pint leaves alone: no edits.
    s.notify("textDocument/didChange", json!({"textDocument": {"uri": file("app/A b.php"), "version": 2}, "contentChanges": [{"text": want}]}));
    assert_eq!(pint(&dir, "app/A b.php", &want), want);
    assert_eq!(s.format(&file("app/A b.php"))["result"], json!([]));

    // Excluded by the config (notName) or Pint's own list (storage), code that doesn't parse (Pint echoes those), and
    // vendor (Pint's stdin path formats it).
    for name in [
        "app/Skip.php",
        "storage/x/S.php",
        "app/Bad.php",
        "vendor/x/V.php",
    ] {
        let code = if name == "app/Bad.php" {
            "<?php\n$a = array(\n"
        } else {
            "<?php\n$a = array(1);\n"
        };
        let want = pint(&dir, name, code);
        let edits = if want == code {
            json!([])
        } else {
            json!([{"range": {"start": {"line": 0, "character": 0}, "end": {"line": 2, "character": 0}}, "newText": want}])
        };
        assert_eq!(want == code, name != "vendor/x/V.php", "{name}");
        s.open(&file(name), code);
        assert_eq!(s.format(&file(name))["result"], edits, "{name}");
    }

    // Closed or never-opened documents, non-file URIs, bad params and other methods are errors (codes from the spec).
    s.notify(
        "textDocument/didClose",
        json!({"textDocument": {"uri": file("app/A b.php")}}),
    );
    let r = s.format(&file("app/A b.php"));
    assert_eq!(
        (
            r["error"]["code"].as_i64(),
            r["error"]["message"]
                .as_str()
                .unwrap()
                .ends_with("/app/A%20b.php is not open")
        ),
        (Some(-32803), true),
        "{r}"
    );
    s.open("untitled:Untitled-1", "<?php\n$a = array(1);\n");
    assert_eq!(
        s.format("untitled:Untitled-1")["error"],
        json!({"code": -32803, "message": "untitled:Untitled-1 is not a file"})
    );
    assert_eq!(
        s.request("textDocument/formatting", json!({"nope": 1}))["error"]["code"],
        -32602
    );
    assert_eq!(
        s.request("textDocument/hover", json!({}))["error"]["code"],
        -32601
    );
    // Ignored: other notifications, unparsable ones, responses from the client.
    s.notify("$/setTrace", json!({"value": "off"}));
    s.notify("textDocument/didOpen", json!({}));
    s.notify("textDocument/didChange", json!({}));
    s.notify("textDocument/didClose", json!({}));
    s.send(json!({"jsonrpc": "2.0", "id": 99, "result": null}));
    s.stop();
}

/// Without a workspace the server's cwd is the project; a broken pun.toml fails each request with its error.
#[test]
fn broken_config_fails_requests() {
    if oracle().is_none() {
        return;
    }
    let dir = project("broken", r#"{"preset":"empty"}"#);
    std::fs::write(dir.join("pun.toml"), "[format\n").unwrap();
    let mut s = Server::start(&dir);
    s.request("initialize", json!({"capabilities": {}}));
    s.notify("initialized", json!({}));
    s.open(&format!("file://{}/app/A.php", dir.display()), "<?php\n");
    let r = s.format(&format!("file://{}/app/A.php", dir.display()));
    assert_eq!(
        r["error"],
        json!({"code": -32803, "message": format!("invalid {}/pun.toml", dir.display())})
    );
    // Fixing pun.toml, then changing its rules, applies to the next request.
    let code = "<?php\n$a = array(1);\n";
    s.open(&format!("file://{}/app/A.php", dir.display()), code);
    for cfg in [
        r#"{"preset":"empty","rules":{"array_syntax":true}}"#,
        r#"{"preset":"empty","rules":{"blank_line_after_opening_tag":true}}"#,
        r#"{"preset":"empty"}"#,
        r#"{"preset":"empty","rules":{"array_syntax":true}}"#,
        r#"{"preset":"empty","rules":{"single_quote":true}}"#,
    ] {
        // Same mtime as before (a fast filesystem, one mtime tick): only the content tells.
        let path = dir.join("pun.toml");
        let before = std::fs::metadata(&path).and_then(|m| m.modified()).unwrap();
        std::fs::write(dir.join("pint.json"), cfg).unwrap();
        migrate(&dir);
        std::fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_modified(before)
            .unwrap();
        let want = pint(&dir, "app/A.php", code);
        let edits = if want == code {
            json!([])
        } else {
            json!([{"range": {"start": {"line": 0, "character": 0}, "end": {"line": 2, "character": 0}}, "newText": want}])
        };
        assert_eq!(
            s.format(&format!("file://{}/app/A.php", dir.display()))["result"],
            edits,
            "{cfg}"
        );
    }
    s.stop();
}
