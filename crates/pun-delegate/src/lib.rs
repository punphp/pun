//! Delegate mode: run the real PHP-CS-Fixer in long-lived `php` workers (one per thread that asks), fed files over
//! stdin/stdout, for configs Pun can't run natively (unported rules, `.php-cs-fixer.php` with custom fixers).
//!
//! Whole file, not per fixer: PHP-CS-Fixer runs every fixer once, in priority order, on one token list, and unported
//! rules sit between native ones (laravel/framework's pint.json interleaves 8 of them from priority 27 to -41).
//! Splitting the list would mean printing and re-lexing mid-run, which isn't provably identical (adjacent whitespace
//! tokens merge, transformer kinds are recomputed), so when anything is delegated, every fixer runs in PHP.
//!
//! ponytail: system `php` + the project's PHP-CS-Fixer (vendor/laravel/pint PHAR or vendor/autoload.php). The plan's
//! embedded libphp and a downloaded, pinned Pint PHAR replace `runtime()` without changing the protocol.

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;

const WORKER: &str = include_str!("worker.php");

/// What the workers configure PHP-CS-Fixer from.
pub enum Source {
    /// Pint's merged rule map (preset + pint.json, `false` entries and `@Set`s included), as Pint's Config gets it.
    Rules(Value),
    /// A `.php-cs-fixer(.dist).php` file, loaded by PHP-CS-Fixer's own ConfigurationResolver.
    Config(PathBuf),
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Unchanged,
    /// New code and the fixers that changed it, in run order.
    Fixed(Vec<u8>, Vec<&'static str>),
    /// A fixer threw (Runner reports the file as an error and goes on).
    Failed(String),
}

pub struct Delegate {
    root: PathBuf,
    /// The Pint PHAR or `vendor/autoload.php` the workers load.
    pub runtime: PathBuf,
    mode: &'static str,
    arg: String,
    /// Enabled fixers in run order, with priorities (from the first worker).
    pub fixers: Vec<(&'static str, i32)>,
    idle: Mutex<Vec<Worker>>,
}

struct Worker {
    child: Child,
    /// `None` once dropped: EOF on stdin is the worker's signal to exit.
    stdin: Option<ChildStdin>,
    stdout: BufReader<ChildStdout>,
}

impl Drop for Worker {
    /// Close stdin and let PHP exit on its own: a killed PHP leaks its multi-MB PHAR temp files in /tmp.
    fn drop(&mut self) {
        self.stdin = None;
        let _ = self.child.wait();
    }
}

impl Worker {
    /// One reply: `<kind> <head> <len>\n<payload>`.
    fn reply(&mut self) -> Result<(String, String, Vec<u8>)> {
        let mut line = String::new();
        self.stdout.read_line(&mut line)?;
        let mut parts = line.split_whitespace();
        let (Some(kind), Some(head), Some(len)) = (parts.next(), parts.next(), parts.next().and_then(|n| n.parse().ok())) else {
            bail!("the PHP worker exited unexpectedly (see its output above)");
        };
        let mut body = vec![0; len];
        self.stdout.read_exact(&mut body)?;
        if kind == "A" {
            bail!("{}", String::from_utf8_lossy(&body));
        }
        Ok((kind.to_string(), head.to_string(), body))
    }
}

/// Where PHP-CS-Fixer comes from: `pinned` (the CLI passes `$PUN_PINT`), the project's Pint PHAR, then its Composer
/// autoloader when it has PHP-CS-Fixer. A `.php-cs-fixer.php` config always needs the autoloader (its custom fixers).
fn runtime(root: &Path, source: &Source, pinned: Option<PathBuf>) -> Result<PathBuf> {
    let autoload = root.join("vendor/autoload.php");
    let has_fixer = root.join("vendor/friendsofphp/php-cs-fixer").is_dir();
    let found = match source {
        Source::Config(_) => Some(autoload).filter(|_| has_fixer),
        Source::Rules(_) => pinned
            .or_else(|| Some(root.join("vendor/laravel/pint/builds/pint")).filter(|p| p.is_file()))
            .or_else(|| Some(autoload).filter(|_| has_fixer)),
    };
    found.context(match source {
        Source::Config(_) => "no vendor/friendsofphp/php-cs-fixer: run `composer install` so Pun can load the config's fixers",
        Source::Rules(_) => "no PHP-CS-Fixer to delegate to: run `composer require --dev laravel/pint`, set PUN_PINT to a Pint PHAR, or pass --native-only",
    })
}

impl Delegate {
    /// Find a runtime, start the first worker and read its fixer list (config errors surface here).
    pub fn start(root: &Path, source: Source, pinned: Option<PathBuf>) -> Result<Self> {
        let runtime = runtime(root, &source, pinned)?;
        let (mode, arg) = match source {
            Source::Rules(r) => ("rules", r.to_string()),
            Source::Config(p) => ("config", p.to_string_lossy().into_owned()),
        };
        let d = Delegate { root: root.to_path_buf(), runtime, mode, arg, fixers: vec![], idle: Mutex::default() };
        let (w, list) = d.spawn()?;
        let list: Vec<(String, i32)> = serde_json::from_slice(&list)?;
        // Leaked once per run: fixer names are reported as &'static str like native ones.
        let fixers = list.into_iter().map(|(n, p)| (&*Box::leak(n.into_boxed_str()), p)).collect();
        d.idle.lock().unwrap().push(w);
        Ok(Delegate { fixers, ..d })
    }

    fn spawn(&self) -> Result<(Worker, Vec<u8>)> {
        let mut child = Command::new("php")
            .args(["-r", WORKER, "--"])
            .arg(&self.runtime)
            .args([self.mode, &self.arg])
            .current_dir(&self.root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("delegate mode needs `php` on PATH")?;
        let (stdin, stdout) = (child.stdin.take(), BufReader::new(child.stdout.take().unwrap()));
        let mut w = Worker { child, stdin, stdout };
        let (kind, _, list) = w.reply()?;
        if kind == "E" {
            bail!("{}", String::from_utf8_lossy(&list));
        }
        Ok((w, list))
    }

    /// Send one request to an idle worker (or a new one). After an error (which ends the run) the worker is dropped.
    fn call(&self, req: &[&[u8]]) -> Result<(String, String, Vec<u8>)> {
        let idle = self.idle.lock().unwrap().pop();
        let mut w = match idle {
            Some(w) => w,
            None => self.spawn()?.0,
        };
        let stdin = w.stdin.as_mut().unwrap();
        for part in req {
            stdin.write_all(part)?;
        }
        stdin.flush()?;
        let r = w.reply();
        if r.is_ok() {
            self.idle.lock().unwrap().push(w);
        }
        r
    }

    /// Runner::fixFile on `code`. `Err` aborts the run, as the same failure aborts PHP-CS-Fixer's (a crashed worker,
    /// an `@php-cs-fixer-ignore` for a rule that isn't enabled).
    pub fn fix(&self, file: &Path, code: &[u8]) -> Result<Outcome> {
        let path = file.as_os_str().as_encoded_bytes();
        let head = format!("F {} {}\n", path.len(), code.len());
        let (kind, applied, body) = self.call(&[head.as_bytes(), path, code])?;
        Ok(match kind.as_str() {
            "S" => Outcome::Unchanged,
            "E" => Outcome::Failed(String::from_utf8_lossy(&body).into_owned()),
            _ => Outcome::Fixed(body, applied.split(',').map(|i| self.fixers[i.parse::<usize>().unwrap()].0).collect()),
        })
    }

    /// The config's Finder, as PHP-CS-Fixer resolves it (`.php-cs-fixer.php` only).
    pub fn files(&self) -> Result<Vec<PathBuf>> {
        let (_, _, list) = self.call(&[b"L\n"])?;
        Ok(String::from_utf8_lossy(&list).lines().map(PathBuf::from).collect())
    }

    /// What delegated results depend on besides Pun's own signature: the runtime, the config file and the installed
    /// packages. ponytail: like PHP-CS-Fixer's own cache, edits to project-local custom fixer classes aren't seen.
    pub fn signature(&self) -> Value {
        let stamp = |p: &Path| std::fs::metadata(p).ok().map(|m| format!("{} {:?}", m.len(), m.modified().ok()));
        let config = if self.mode == "config" { std::fs::read(&self.arg).ok() } else { None };
        json!({
            "runtime": [self.runtime, stamp(&self.runtime)],
            "installed": stamp(&self.root.join("vendor/composer/installed.json")),
            "config": config.map(|c| String::from_utf8_lossy(&c).into_owned()),
        })
    }
}
