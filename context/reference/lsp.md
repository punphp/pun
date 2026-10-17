# `pun lsp` = PHPantom + Pun

Goal: one language server. [PHPantom](https://github.com/PHPantom-dev/phpantom_lsp) (MIT) supplies code intelligence
(completion, hover, definitions, references, symbols, rename, …); Pun supplies Pint-identical formatting, diagnostics
(PHP's syntax errors, `pun lint --mago` rules; `pun analyse` later) and fixes.

Status: prototype in `crates/pun-lsp` (package `punphp-lsp`, binary `pun-lsp`, `pun_lsp::run()`). Not wired into
`pun lsp` yet (pun-cli is being cleaned up); see "Wiring into `pun lsp`" below.

## Decision: embed PHPantom in-process (option a), routed at the JSON-RPC level

```
editor ──stdio── pun-lsp router ──in-process jsonrpc──▶ PHPantom Backend (its own tower-lsp LspService)
                     │   ◀────── PHPantom's outgoing requests / notifications (ClientSocket) ──┘
                     └── Pun: formatting, diagnostics, fix-all (pun-config / pun-format / pun-fixers / mago-linter)
```

- PHPantom is a library dependency (`phpantom_lsp`, git, pinned rev). Its `pub struct Backend` (`Backend::new(Client)`,
  `impl tower_lsp::LanguageServer`) runs inside `LspService::build(Backend::new).finish()`, exactly as its own
  `main.rs` does; we keep the service and the `ClientSocket` instead of handing them to `tower_lsp::Server`.
- The router reads the editor's messages (lsp-server framing) and:
  - `textDocument/formatting`: Pun answers; PHPantom never sees it.
  - `initialize`: Pun loads the project's pint.json / pun.toml; the client's `textDocument.diagnostic` capability is
    removed before PHPantom sees it (so PHPantom pushes), and the reply's capabilities are merged: full document
    sync, Pun's formatting provider, no range formatting (Pint has none; PHPantom's would run another formatter),
    `source.fixAll.pun` added to the code action kinds.
  - `textDocument/codeAction`: PHPantom answers; Pun's fix-all action is appended.
  - `didOpen` / `didChange` / `didSave` / `didClose`: Pun updates its copy and its diagnostics, then forwards.
  - everything else, and the editor's replies to PHPantom's requests: forwarded untouched. New PHPantom features need
    no code here.
- PHPantom's outgoing messages are forwarded, except `textDocument/publishDiagnostics`: the router keeps PHPantom's
  set per URI (minus its `syntax_error`s: Pun's carry PHP's exact message, PHPantom's mago-parser ones differ and add
  follow-on errors PHP doesn't report) and publishes it together with Pun's. Pun republishes on its own changes, so
  the editor always gets the union.
- Calls into PHPantom run concurrently and are polled in arrival order (`for_each_concurrent`, as tower-lsp's own
  server does), so a handler waiting on an editor reply (e.g. `window/workDoneProgress/create`) doesn't block the loop.

Why not the other shapes:

- Trait-level wrapper (`impl LanguageServer for Pun` delegating ~45 methods to `Backend`): fine for formatting and
  code actions, but PHPantom publishes diagnostics itself through its `Client` (push) or serves them from its own
  cache (pull). `publishDiagnostics` replaces the whole set per URI, so a second publisher would overwrite PHPantom's
  (or be overwritten). Without an upstream hook there is no way to add diagnostics at that level.
- Child process + proxy (option b): same routing code, plus a second binary to ship and locate, a pipe hop per message
  and two processes' memory. Not needed: embedding works with no upstream change.

## Evaluation

### Version and the mago pin

| | mago pins | usable |
| --- | --- | --- |
| `phpantom_lsp` 0.11.1 (crates.io, tag `0.11.1`, 2026-10-07) | `=1.47.5` | no: same major as Pun's `=1.57.0`, Cargo allows one semver-compatible version per crate, resolution fails |
| `main` at `86e6c49d4` (2026-10-16, 126 commits after 0.11.1) | `=1.57.0` | yes |

So the prototype uses `phpantom_lsp = { git = "https://github.com/PHPantom-dev/phpantom_lsp", rev = "86e6c49d4",
default-features = false }`. A crates.io dependency needs a PHPantom release with mago 1.57 (ask upstream; until
then `cargo publish` of a crate depending on it isn't possible, a git dependency can't be published).

`.oracle/phpantom` is a clone at tag `0.11.1` checked out at `86e6c49d4`; its `Cargo.toml` gets an empty `[workspace]`
table locally (the clone sits inside Pun's workspace, so cargo would otherwise refuse to build it standalone).

### Library API

- `src/lib.rs` is a real library: `pub struct Backend`, `Backend::new(client)`, the `LanguageServer` impl, plus public
  modules for its CLI tools (`analyse`, `fix`, `format_cli`, `completion`, `config`, …). Most of the server internals
  (`backend`, `server`, `diagnostics` delivery, `definition`, `hover`) are private; the `LanguageServer` trait is the
  usable surface.
- No hook to add diagnostics, add capabilities or intercept requests. None needed with the router (see the optional
  patch below for what would make it cleaner).
- `#[global_allocator]` (mimalloc) is declared in `lib.rs` under the default `mimalloc` feature, so an embedder
  inherits it as its global allocator. Pun uses `default-features = false` (glibc malloc, as Pun has today).
- Its `build.rs` downloads JetBrains phpstorm-stubs from GitHub at build time (pinned in `stubs.lock`, checksummed)
  into `OUT_DIR` and embeds them (~13.5 MB of PHP, 1051 files). First builds need network; the `offline-stubs` feature
  builds without (no built-in symbols then). Release/CI builds should cache `~/.cargo/git` + the build dir or vendor.

### Dependencies and lockfile

- Adds tower-lsp 0.20 (with lsp-types 0.94, alongside Pun's lsp-types 0.97), tokio (`full`), tower, futures, clap,
  sqlparser, regex, globset, memmap2, ustr, parking_lot, tempfile, etcetera, tracing(-subscriber), and its self-update
  stack (ureq + rustls + webpki-roots, tar, flate2, zip, self-replace; not optional, unused by Pun, mostly dropped by
  the linker).
- Cargo.lock: +116 packages, no existing package changed version, mago stays a single `=1.57.0` copy (PHPantom adds
  mago-composer and mago-phpdoc-syntax). New duplicate versions: lsp-types 0.94/0.97, toml 0.9/1, similar 2/3,
  hashbrown, getrandom, nix, object, miniz_oxide (and Windows target crates).

### Build time and size

All on this box (2 cores shared with 2 other agents, `CARGO_BUILD_JOBS=1`, `nice`), so treat as upper bounds.

| build | time | peak rustc RSS | binary | stripped |
| --- | --- | --- | --- | --- |
| PHPantom alone, `--release` from scratch (231 crates) | 24m25s (1100 s user) | 2.6 GB | 58 MB | 45 MB |
| pun-lsp, `--profile profiling` from scratch (~250 crates) | ~24.5 min (21.5 min to PHPantom + deps, 3 min for Pun's crates) | 3.0 GB | 242 MB (line tables) | 47 MB |
| pun-lsp, debug tests from scratch | 9m57s | | | |
| `pun` today (`target-perf/profiling/pun`) | | | 56 MB | 8.0 MB |

Debug from scratch with `cargo build --timings` (8m35s wall): of 514 s of compile units, ~330 s are crates only
PHPantom brings (`phpantom_lsp` itself 182 s, sqlparser 23 s, rustls 22 s, lsp-types 0.94 10 s), so a clean build
of pun-lsp takes ~2.8x what Pun's own crates and deps take (~160 s).

So `pun` with PHPantom grows from ~8 MB to ~47 MB stripped: ~14 MB embedded stubs, the rest PHPantom's code (it is
a large crate) and tokio/tower-lsp. Incremental builds of Pun crates don't recompile PHPantom (a leaf dependency);
they pay only the bigger link.

### Configuration

- PHPantom: optional `.phpantom.toml` per project plus a global `$XDG_CONFIG_HOME/phpantom_lsp/.phpantom.toml`
  (merged key by key, watched and reloaded). Sections: `[php] version`, `[diagnostics]` toggles and
  `[[diagnostics.ignore]]`, `[indexing]`, `[semantic-tokens]`, `[formatting]` (pint / php-cs-fixer / phpcbf paths;
  irrelevant under pun-lsp, formatting is Pun's), `[phpstan]`, `[phpcs]`, `[phpmd]`, `[mago]` (proxies the `mago` CLI
  when the project has `mago.toml`), `[laravel]`. Also reads `composer.json` (autoload, PHP version). Works with no
  config at all.
- Pun: its usual pint.json / pun.toml, found from the workspace folder (or `rootUri`) of `initialize`.
- Nothing to reconcile in the prototype; each side reads its own files.

### Shared parse / database

Not shared. PHPantom keeps its own per-file mago-syntax ASTs, symbol index, classmap scan and stubs behind private
modules. Pun's formatter works on its own token model (pun-tokens, PHP-CS-Fixer semantics) and its lint re-parses the
open file with mago-syntax. The sharing is at the code level: one compiled copy of mago-syntax/database/names/span.
Pun re-parsing the one open file per change costs milliseconds; sharing PHPantom's AST would need upstream API and
gains little.

### Features on the corpus (`scripts/lsp-bench.py`)

`scripts/lsp-bench.py ROOT FQCN -- SERVER` starts the server in ROOT, initializes (with work-done progress),
waits for PHPantom's indexing progress to end, opens an unsaved `__lsp_bench.php` (`use FQCN; $x = new X(); $x->`)
and asks for completion and the class's definition. Both projects have no `vendor/` (no `composer install` in the
corpus), so only project classes resolve.

| project (PHP files) | completion on `$x->` | definition of the class |
| --- | --- | --- |
| corpus/pinkary (489), `App\Models\User` | 56-57 items: `avatar`, `avatar_updated_at`, `avatar_url`, `bio`, `created_at`, … (columns from migrations) | `app/Models/User.php` line 67 (`final class User …`) |
| corpus/laravel-framework (3167), `Illuminate\Support\Collection` | 218 items: `average`, `avg`, `contains`, `doesntContain`, `each`, … | `src/Illuminate/Collections/Collection.php` line 24 (`class Collection …`) |

Same answers from PHPantom alone and from pun-lsp. Checked by hand against the files.

## Benchmarks: startup and memory

Three runs each, release builds (PHPantom's own `--release` with mimalloc; pun-lsp `--profile profiling`, glibc
malloc). "ready" = `initialize` sent → PHPantom's indexing progress ended. RSS is the server's VmHWM at the end
(after the completion and definition requests; pun-lsp has also linted the opened file).

| project | server | ready (s) | completion (ms) | definition (ms) | peak RSS (MB) |
| --- | --- | --- | --- | --- | --- |
| pinkary | PHPantom alone | 0.05-0.14 | 6.2-7.1 | 0.4-1.8 | 50-51 |
| pinkary | pun-lsp | 0.05-0.07 | 4.8-10.4 | 0.4-3.3 | 41-43 |
| laravel-framework | PHPantom alone | 0.10-0.40 | 50-70 | 0.5-2.1 | 57-60 |
| laravel-framework | pun-lsp | 0.10-0.12 | 55-84 | 0.5-0.9 | 51-52 |

The router costs nothing measurable; pun-lsp's lower RSS is the allocator (mimalloc reserves more), not less work.
Pun's own startup (config + pipeline for Pinkary's pint.json) is inside the same "ready" time.

## Optional upstream patch (not needed for the prototype; not opened)

A public hook so an embedder hands its diagnostics to PHPantom instead of intercepting `publishDiagnostics`. With it,
pun-lsp could drop the interception, keep pull diagnostics for clients that support them, and Pun's findings would
honour `@phpantom-ignore` and `[[diagnostics.ignore]]`. Against `86e6c49d4`; the test passes there (`cargo test
--lib embedder_diagnostics`). Branch `embedder-diagnostics` in `.oracle/phpantom` (commit `3f3bcfd40`); the test fails with the merge line removed.
Use from pun-lsp: `phpantom.inner().set_embedder_diagnostics(uri, diags).await` (`LspService::inner()`).

```diff
diff --git a/src/diagnostics/delivery.rs b/src/diagnostics/delivery.rs
index 9fd6a3c17..98a79578a 100644
--- a/src/diagnostics/delivery.rs
+++ b/src/diagnostics/delivery.rs
@@ -285,6 +285,12 @@ impl Backend {
                 full.extend(mago_diags.iter().cloned());
             }
         }
+        {
+            let cache = self.diag.embedder.lock();
+            if let Some(embedder_diags) = cache.get(uri_str) {
+                full.extend(embedder_diags.iter().cloned());
+            }
+        }
 
         // ── Suppress imprecise overlaps and filter ──────────────────
         suppression::suppress_imprecise_overlaps(&mut full);
@@ -368,6 +374,23 @@ impl Backend {
         }
     }
 
+    /// Set the diagnostics an embedding server computed itself for
+    /// `uri` (its own linter, say), replacing the previous set; an empty
+    /// set clears them.
+    ///
+    /// They are merged into PHPantom's own set and delivered with it in
+    /// both push and pull mode, after the same suppression
+    /// (`@phpantom-ignore`, `[[diagnostics.ignore]]`), so an embedder
+    /// never has to publish over PHPantom's diagnostics or proxy the
+    /// pull requests.
+    pub async fn set_embedder_diagnostics(&self, uri: &str, diagnostics: Vec<Diagnostic>) {
+        self.diag
+            .embedder
+            .lock()
+            .insert(uri.to_string(), diagnostics);
+        self.assemble_and_refresh(uri).await;
+    }
+
     /// Ask the editor to re-pull diagnostics.
     ///
     /// A no-op for push-mode clients, which receive their diagnostics
diff --git a/src/diagnostics/mod.rs b/src/diagnostics/mod.rs
index d32a0e882..999a8e124 100644
--- a/src/diagnostics/mod.rs
+++ b/src/diagnostics/mod.rs
@@ -514,6 +514,45 @@ mod tests {
     /// the previous result exactly, so bumping unconditionally cost a
     /// workspace-wide re-pull per run (a clean open file burned several
     /// of them during startup alone).
+    /// An embedder's diagnostics join PHPantom's in the pulled set, go
+    /// through the same comment suppression, and are replaced (not
+    /// accumulated) by the next call.
+    #[tokio::test]
+    async fn embedder_diagnostics_merge_into_the_pulled_set() {
+        let backend = Backend::new_test();
+        backend
+            .supports_pull_diagnostics
+            .store(true, Ordering::Release);
+        let uri = "file:///test.php";
+        backend.open_files.write().insert(
+            uri.to_string(),
+            Arc::new("<?php\n$a = 1;\n// @phpantom-ignore pun.rule\n$b = 2;\n".to_string()),
+        );
+        let diag = |line: u32, code: &str| Diagnostic {
+            range: Range::new(Position::new(line, 0), Position::new(line, 5)),
+            severity: Some(DiagnosticSeverity::WARNING),
+            code: Some(NumberOrString::String(code.to_string())),
+            source: Some("embedder".to_string()),
+            message: code.to_string(),
+            ..Default::default()
+        };
+        let native = diag(1, "native");
+        backend
+            .diag
+            .last_fast
+            .lock()
+            .insert(uri.to_string(), vec![native.clone()]);
+        let pulled = || backend.diag.last_full.lock().get(uri).cloned();
+
+        backend
+            .set_embedder_diagnostics(uri, vec![diag(1, "pun.other"), diag(3, "pun.rule")])
+            .await;
+        assert_eq!(pulled(), Some(vec![native.clone(), diag(1, "pun.other")]));
+
+        backend.set_embedder_diagnostics(uri, vec![]).await;
+        assert_eq!(pulled(), Some(vec![native]));
+    }
+
     #[tokio::test]
     async fn assemble_and_push_reports_change_only_on_difference() {
         let backend = Backend::new_test();
diff --git a/src/diagnostics/state.rs b/src/diagnostics/state.rs
index 601caf35a..576457103 100644
--- a/src/diagnostics/state.rs
+++ b/src/diagnostics/state.rs
@@ -95,6 +95,9 @@ pub(crate) struct DiagnosticState {
     /// snapshot and the one that started reading first can still finish
     /// writing last, clobbering a fresher merge with a stale one.
     pub(crate) assemble_locks: Arc<Mutex<HashMap<String, Arc<Mutex<()>>>>>,
+    /// Diagnostics an embedding server computed itself, per file URI
+    /// (see [`crate::Backend::set_embedder_diagnostics`]).
+    pub(crate) embedder: Arc<Mutex<HashMap<String, Vec<Diagnostic>>>>,
 }
 
 impl DiagnosticState {
@@ -122,6 +125,7 @@ impl DiagnosticState {
             decl_baselines: Arc::new(Mutex::new(HashMap::new())),
             bodies_read_from: Arc::new(Mutex::new(HashMap::new())),
             assemble_locks: Arc::new(Mutex::new(HashMap::new())),
+            embedder: Arc::new(Mutex::new(HashMap::new())),
         }
     }
 }
```

Other upstream asks, in order of value:

1. A release with mago 1.57 (blocks a crates.io dependency).
2. Move `#[global_allocator]` from `lib.rs` to `main.rs` (embedders shouldn't inherit an allocator through a default
   feature).
3. Make the self-update stack (ureq/rustls/zip/…) a default feature so embedders can drop it.

## Prototype: what works

- `crates/pun-lsp/src/lib.rs`: `run()` serves over stdio; router, capability merge, diagnostics merge, fix-all.
- `crates/pun-lsp/src/pun.rs`: formatting with the project's config through the native pipeline (`Pipeline::run` +
  annotation check, exclusions, panics → no edits), diagnostics (PHP's syntax error on its line; the five
  `pun lint --mago` rules with Mago's level mapped to warning / information / hint), one whole-document edit.
- `crates/pun-lsp/tests/lsp.rs` (stdio, like an editor): capabilities merged; completion and definition on a fixture
  Composer project; formatting and the fix-all edit equal real Pint's stdin output (UTF-16 end position; clean,
  excluded and unparsable files get no edits); a syntax error carries PHP's message and replaces PHPantom's; a Mago
  finding is published together with PHPantom's own and goes away when fixed; levels; indexing progress round-trips
  through the editor's reply; unported rules fail formatting instead of half-fixing; stdin EOF ends the server.
  100% line coverage, clippy clean.

## Wiring into `pun lsp` (after the pun-cli cleanup)

1. `pun lsp` calls `pun_lsp::run()`; delete `crates/pun-cli/src/lsp.rs` and move what `tests/lsp.rs` there checks
   that isn't covered here (config reload, broken pint.json message) to pun-lsp.
2. Share pun-cli's formatting engine instead of pun-lsp's subset: move `Engine2` / `fix_text` / `isolated` and the LSP
   config reload (`setup` / `stamp`) into a library (pun-format or a small `pun-engine`) used by `pun format`,
   `--stdin` and pun-lsp. That brings PHP-CS-Fixer delegation (unported rules, `.php-cs-fixer.php`) and pint.json
   reload on change to the LSP. pun-lsp currently refuses delegated configs with an error.
3. Share `pun lint --mago`'s rule list (and `[lint.mago]` config once it exists) from a library instead of the copy in
   `pun.rs`.
4. `pun analyse` diagnostics: add a source next to the Mago rules in `Pun::diagnostics` (or run on save only if slow).
5. Per-rule quick fixes: none of the five Mago rules carries edits; when Pun gains fixable lint rules (Rector-style
   `pun lint --fix`), offer them as `quickfix` actions on their diagnostics.
6. Release: decide git dependency vs. waiting for a PHPantom release with mago 1.57 (crates.io publishing of punphp
   needs the latter); network for the stubs download in dist builds; binary grows ~8 → ~47 MB.
7. Optional: upstream the diagnostics hook above, then drop the `publishDiagnostics` interception and stop hiding
   pull diagnostics from PHPantom.
