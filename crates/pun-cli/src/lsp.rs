//! `pun lsp`: a formatting-only language server on stdio (lsp-server's two threads, no async runtime).
//! Documents sync in full; `textDocument/formatting` runs the `--stdin` pipeline with the document's path, so the
//! project's pint.json / pun.toml and their exclusions apply, and answers with one whole-document edit (no edits
//! when nothing changes, the path is excluded or the code doesn't parse).
//! Config files are checked (content hashes) on each formatting request and reloaded when they changed.

use crate::{Engine, FormatArgs, engine, fix_text, load_config, root};
use anyhow::Result;
use lsp_server::{Connection, ErrorCode, Message, Response};
use lsp_types::notification::{
    DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification as _,
};
use lsp_types::request::{Formatting, Request as _};
use lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DocumentFormattingParams, InitializeParams, OneOf, Position, Range, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Uri,
};
use pun_config::Config;
use serde_json::from_value;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn run() -> Result<u8> {
    let (conn, io) = Connection::stdio();
    let caps = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        document_formatting_provider: Some(OneOf::Left(true)),
        ..Default::default()
    };
    let init: InitializeParams = from_value(conn.initialize(serde_json::to_value(caps)?)?)?;
    // The workspace is the project (its config, its exclusions), wherever the editor started the server.
    #[allow(deprecated)]
    let workspace = init
        .workspace_folders
        .and_then(|f| f.into_iter().next())
        .map(|f| f.uri)
        .or(init.root_uri);
    if let Some(dir) = workspace.as_ref().and_then(file_path) {
        std::env::set_current_dir(dir)?;
    }
    let mut setup = setup();
    let mut seen = stamp(&setup);
    let mut docs: HashMap<String, String> = HashMap::new();
    for msg in &conn.receiver {
        match msg {
            Message::Request(req) => {
                if conn.handle_shutdown(&req)? {
                    break;
                }
                // A failed load can't list the files it read (an `extend`ed one), so it is retried on every request.
                if req.method == Formatting::METHOD && (setup.is_err() || stamp(&setup) != seen) {
                    setup = self::setup();
                    seen = stamp(&setup);
                }
                let resp = match (req.method.as_str(), &setup) {
                    (Formatting::METHOD, Ok((cfg, engine))) => {
                        match from_value::<DocumentFormattingParams>(req.params) {
                            Ok(p) => match formatting(cfg, engine, &docs, &p.text_document.uri) {
                                Ok(edits) => Response::new_ok(req.id, edits),
                                Err(e) => Response::new_err(
                                    req.id,
                                    ErrorCode::RequestFailed as i32,
                                    e.to_string(),
                                ),
                            },
                            Err(e) => Response::new_err(
                                req.id,
                                ErrorCode::InvalidParams as i32,
                                e.to_string(),
                            ),
                        }
                    }
                    // A broken pint.json fails every request with Pint's message.
                    (Formatting::METHOD, Err(e)) => {
                        Response::new_err(req.id, ErrorCode::RequestFailed as i32, e.to_string())
                    }
                    _ => Response::new_err(
                        req.id,
                        ErrorCode::MethodNotFound as i32,
                        format!("pun lsp only formats; no {}", req.method),
                    ),
                };
                conn.sender.send(resp.into())?;
            }
            Message::Notification(n) => match n.method.as_str() {
                DidOpenTextDocument::METHOD => {
                    if let Ok(p) = from_value::<DidOpenTextDocumentParams>(n.params) {
                        docs.insert(
                            p.text_document.uri.as_str().to_owned(),
                            p.text_document.text,
                        );
                    }
                }
                DidChangeTextDocument::METHOD => {
                    if let Ok(p) = from_value::<DidChangeTextDocumentParams>(n.params)
                        && let Some(c) = p.content_changes.into_iter().last()
                    {
                        docs.insert(p.text_document.uri.as_str().to_owned(), c.text);
                    }
                }
                DidCloseTextDocument::METHOD => {
                    if let Ok(p) = from_value::<DidCloseTextDocumentParams>(n.params) {
                        docs.remove(p.text_document.uri.as_str());
                    }
                }
                _ => {}
            },
            Message::Response(_) => {}
        }
    }
    // The writer thread ends once every sender is gone.
    drop(conn);
    io.join()?;
    Ok(0)
}

fn setup() -> Result<(Config, Engine)> {
    let a = FormatArgs::default();
    let root = root()?;
    let cfg = load_config(&a, false, &root)?;
    let engine = engine(&a, &cfg, &root)?;
    Ok((cfg, engine))
}

/// A hash of each config file (`None`: absent): the ones a project can add, plus what the loaded config read (an
/// `extend`ed file). Content, not mtime + size: a same-size rewrite within one mtime tick is still a change.
fn stamp(setup: &Result<(Config, Engine)>) -> Vec<Option<u64>> {
    let root = root().unwrap_or_default();
    let mut files: Vec<PathBuf> = [
        "composer.json",
        "pint.json",
        "pun.toml",
        ".php-cs-fixer.php",
        ".php-cs-fixer.dist.php",
    ]
    .iter()
    .map(|f| root.join(f))
    .collect();
    if let Ok((cfg, _)) = setup {
        files.extend(cfg.sources.iter().cloned());
    }
    files
        .iter()
        .map(|f| {
            std::fs::read(f)
                .ok()
                .map(|c| xxhash_rust::xxh3::xxh3_64(&c))
        })
        .collect()
}

fn formatting(
    cfg: &Config,
    engine: &Engine,
    docs: &HashMap<String, String>,
    uri: &Uri,
) -> Result<Vec<TextEdit>> {
    let text = docs
        .get(uri.as_str())
        .ok_or_else(|| anyhow::anyhow!("{} is not open", uri.as_str()))?;
    let path = file_path(uri).ok_or_else(|| anyhow::anyhow!("{} is not a file", uri.as_str()))?;
    let Some(new) = fix_text(cfg, engine, &path, true, text.as_bytes())? else {
        return Ok(vec![]);
    };
    // Positions count UTF-16 units (the protocol's default encoding); the range ends after the last character.
    // Line breaks are \n, \r\n and a lone \r.
    let breaks =
        text.matches('\n').count() + text.matches('\r').count() - text.matches("\r\n").count();
    let end = Position {
        line: breaks as u32,
        character: text
            .rsplit(['\r', '\n'])
            .next()
            .unwrap_or("")
            .encode_utf16()
            .count() as u32,
    };
    Ok(vec![TextEdit {
        range: Range {
            start: Position::default(),
            end,
        },
        new_text: String::from_utf8_lossy(&new).into_owned(),
    }])
}

/// `file:///a/b%20c.php` -> `/a/b c.php`.
fn file_path(uri: &Uri) -> Option<PathBuf> {
    if uri.scheme().map(|s| s.as_str()) != Some("file") {
        return None;
    }
    let path = uri
        .path()
        .as_estr()
        .decode()
        .into_string()
        .ok()?
        .into_owned();
    // `file:///C:/x` on Windows.
    #[cfg(windows)]
    let path = path.strip_prefix('/').map(str::to_string).unwrap_or(path);
    Some(PathBuf::from(path))
}
