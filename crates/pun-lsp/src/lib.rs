//! `pun lsp`: PHPantom's language server (completion, hover, definitions, references, symbols, rename, …) with
//! Pun's own features on top: Pint formatting, diagnostics (PHP's syntax errors, `pun lint --mago` rules) and a
//! `source.fixAll.pun` code action.
//!
//! PHPantom's `Backend` runs in-process in its own tower-lsp `LspService`. This loop sits between the editor (stdio)
//! and that service at the JSON-RPC level: Pun answers formatting, appends its action to PHPantom's code actions and
//! merges its diagnostics into the ones PHPantom publishes; every other message passes through untouched, so PHPantom
//! features need no code here. PHPantom is told the client has no pull diagnostics, so all diagnostics are pushed
//! and Pun can merge them; documents sync in full, so both sides hold the same text.

mod pun;

use anyhow::Result;
use futures::future::BoxFuture;
use futures::{SinkExt, StreamExt};
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, Response};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tower::{Service, ServiceExt};
use tower_lsp::lsp_types::{CodeAction, CodeActionKind, Url, WorkspaceEdit};
use tower_lsp::{LspService, jsonrpc};

pub fn run() -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let r = rt.block_on(serve());
    // PHPantom's background work (indexing, workers) must not hold up the exit.
    rt.shutdown_background();
    r
}

#[derive(Default)]
struct State {
    pun: Option<pun::Pun>,
    docs: HashMap<String, String>,
    /// Per URI, the diagnostics last published by PHPantom (its syntax errors dropped: Pun's carry PHP's message)
    /// and Pun's own.
    phpantom: HashMap<String, Vec<Value>>,
    own: HashMap<String, Vec<Value>>,
}

type Shared = Arc<Mutex<State>>;
type Out = crossbeam_channel::Sender<Message>;

async fn serve() -> Result<()> {
    let (conn, _io) = Connection::stdio();
    let state = Shared::default();
    let (mut phpantom, socket) = LspService::build(phpantom_lsp::Backend::new).finish();
    let (from_phpantom, mut to_phpantom) = socket.split();

    // PHPantom -> editor: its requests and notifications, diagnostics merged with Pun's.
    let (out, st) = (conn.sender.clone(), state.clone());
    tokio::spawn(from_phpantom.for_each(move |req| {
        if req.method() == "textDocument/publishDiagnostics" {
            let p = req.params().cloned().unwrap_or_default();
            let uri = p["uri"].as_str().unwrap_or_default().to_owned();
            let diags = p["diagnostics"].as_array().into_iter().flatten();
            let diags = diags
                .filter(|d| d["code"] != "syntax_error")
                .cloned()
                .collect();
            let mut s = st.lock().unwrap();
            s.phpantom.insert(uri.clone(), diags);
            publish(&out, &s, &uri);
        } else if let Ok(m) = serde_json::from_value(serde_json::to_value(req).unwrap()) {
            let _ = out.send(m);
        }
        futures::future::ready(())
    }));

    // Calls into PHPantom run concurrently, polled in arrival order (as tower-lsp's own server does), so its
    // handlers can wait on editor replies that this loop keeps reading.
    let (jobs, queue) = futures::channel::mpsc::unbounded::<BoxFuture<'static, ()>>();
    tokio::spawn(queue.for_each_concurrent(None, |f| f));

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let receiver = conn.receiver.clone();
    std::thread::spawn(move || {
        receiver
            .into_iter()
            .try_for_each(|m| tx.send(m).map_err(drop))
    });

    while let Some(msg) = rx.recv().await {
        let (method, id, params) = match msg {
            // Replies to PHPantom's requests (Pun sends none).
            Message::Response(r) => {
                // lsp-server leaves `jsonrpc` to its writer; tower-lsp requires it.
                let mut r = serde_json::to_value(r)?;
                r["jsonrpc"] = json!("2.0");
                let _ = to_phpantom.send(serde_json::from_value(r)?).await;
                continue;
            }
            Message::Notification(n) => {
                notification(&conn.sender, &state, &n);
                (n.method, None, n.params)
            }
            Message::Request(r) if r.method == "textDocument/formatting" => {
                conn.sender.send(formatting(&state, r).into())?;
                continue;
            }
            Message::Request(r) => (r.method, Some(r.id), r.params),
        };
        let exit = method == "exit";
        let mut params = params;
        let mut actions = vec![];
        match method.as_str() {
            "initialize" => initialize(&state, &mut params),
            "textDocument/codeAction" => actions = fix_all(&state, &params),
            _ => {}
        }
        let mut rpc = json!({"jsonrpc": "2.0", "method": method});
        if let Some(id) = &id {
            rpc["id"] = serde_json::to_value(id)?;
        }
        // `shutdown` / `exit` / `initialized` take no params, and tower-lsp rejects `null`.
        if !params.is_null() {
            rpc["params"] = params;
        }
        let call = phpantom
            .ready()
            .await?
            .call(serde_json::from_value::<jsonrpc::Request>(rpc)?);
        let out = conn.sender.clone();
        let _ = jobs.unbounded_send(Box::pin(async move {
            let Ok(Some(resp)) = call.await else { return };
            let mut v = serde_json::to_value(resp).unwrap();
            if let Some(caps) = v.pointer_mut("/result/capabilities") {
                merge_capabilities(caps);
            }
            if method == "textDocument/codeAction" && v.get("result").is_some() {
                let mut all = v["result"].as_array().cloned().unwrap_or_default();
                all.extend(actions);
                v["result"] = all.into();
            }
            if let Ok(m) = serde_json::from_value::<Message>(v) {
                let _ = out.send(m);
            }
        }));
        if exit {
            break;
        }
    }
    Ok(())
}

/// Before PHPantom sees `initialize`: load the project's Pun config, and hide pull diagnostics from PHPantom.
fn initialize(state: &Shared, params: &mut Value) {
    let uri = params
        .pointer("/workspaceFolders/0/uri")
        .or(params.get("rootUri"))
        .and_then(Value::as_str);
    let root = uri
        .and_then(|u| Url::parse(u).ok()?.to_file_path().ok())
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_default();
    // Pint and the fixers read the project relative to the working directory, as `pun format` does.
    let _ = std::env::set_current_dir(&root);
    state.lock().unwrap().pun = Some(pun::Pun::load(root));
    if let Some(td) = params
        .pointer_mut("/capabilities/textDocument")
        .and_then(Value::as_object_mut)
    {
        td.remove("diagnostic");
    }
}

fn merge_capabilities(caps: &mut Value) {
    caps["textDocumentSync"]["change"] = json!(1);
    caps["documentFormattingProvider"] = json!(true);
    // Pint has no range formatting; PHPantom's would run a different formatter.
    caps.as_object_mut()
        .unwrap()
        .remove("documentRangeFormattingProvider");
    if let Some(kinds) = caps
        .pointer_mut("/codeActionProvider/codeActionKinds")
        .and_then(Value::as_array_mut)
    {
        kinds.push(json!("source.fixAll.pun"));
    }
}

fn notification(out: &Out, state: &Shared, n: &Notification) {
    let uri = n
        .params
        .pointer("/textDocument/uri")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    let mut s = state.lock().unwrap();
    match n.method.as_str() {
        "textDocument/didOpen" => {
            s.docs.insert(
                uri.clone(),
                n.params["textDocument"]["text"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            );
        }
        "textDocument/didChange" => {
            if let Some(text) = n.params["contentChanges"]
                .as_array()
                .and_then(|c| c.last())
                .and_then(|c| c["text"].as_str())
            {
                s.docs.insert(uri.clone(), text.to_owned());
            }
        }
        "textDocument/didSave" => {}
        "textDocument/didClose" => {
            s.docs.remove(&uri);
            s.own.remove(&uri);
            return;
        }
        _ => return,
    }
    // Only documents on disk: their path picks the config, exclusions and rules.
    let diags = (|| Some(s.pun.as_ref()?.diagnostics(&path(&uri)?, s.docs.get(&uri)?)))();
    let diags = diags
        .into_iter()
        .flatten()
        .map(|d| serde_json::to_value(d).unwrap())
        .collect();
    s.own.insert(uri.clone(), diags);
    publish(out, &s, &uri);
}

fn publish(out: &Out, s: &State, uri: &str) {
    let all: Vec<&Value> = s
        .phpantom
        .get(uri)
        .into_iter()
        .chain(s.own.get(uri))
        .flatten()
        .collect();
    let _ = out.send(
        Notification::new(
            "textDocument/publishDiagnostics".into(),
            json!({"uri": uri, "diagnostics": all}),
        )
        .into(),
    );
}

/// The open document at `params.textDocument.uri`, Pint-fixed.
fn fixed(
    state: &Shared,
    params: &Value,
) -> Result<(Url, Vec<tower_lsp::lsp_types::TextEdit>), String> {
    let uri = params
        .pointer("/textDocument/uri")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let s = state.lock().unwrap();
    let text = s
        .docs
        .get(uri)
        .ok_or_else(|| format!("{uri} is not open"))?;
    let path = path(uri).ok_or_else(|| format!("{uri} is not a file"))?;
    let edits = s
        .pun
        .as_ref()
        .ok_or("not initialized")?
        .format(&path, text)?;
    Ok((Url::parse(uri).unwrap(), edits))
}

fn formatting(state: &Shared, r: Request) -> Response {
    match fixed(state, &r.params) {
        Ok((_, edits)) => Response::new_ok(r.id, edits),
        Err(e) => Response::new_err(r.id, ErrorCode::RequestFailed as i32, e),
    }
}

/// The whole-file Pint fix as a `source.fixAll.pun` action, when there is one.
fn fix_all(state: &Shared, params: &Value) -> Vec<Value> {
    let fix = fixed(state, params)
        .into_iter()
        .filter(|(_, edits)| !edits.is_empty());
    fix.map(|(uri, edits)| {
        let action = CodeAction {
            title: "Fix with Pun (Pint rules)".into(),
            kind: Some(CodeActionKind::new("source.fixAll.pun")),
            edit: Some(WorkspaceEdit {
                changes: Some([(uri, edits)].into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        serde_json::to_value(action).unwrap()
    })
    .collect()
}

fn path(uri: &str) -> Option<PathBuf> {
    Url::parse(uri).ok()?.to_file_path().ok()
}
