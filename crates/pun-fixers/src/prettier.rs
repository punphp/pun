//! Pint's `App\Support\Prettier`: the package probe and Blade formatting share their first Node start.
use anyhow::{Result, bail};
use pun_tokens::php_compat::strpos;
use std::cell::RefCell;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

const WORKER: &str = include_str!("fixers/pint_laravel_blade/worker.cjs");
const PROBE: &str = include_str!("fixers/pint_laravel_blade/probe.cjs");
const PRETTIERRC: &str = include_str!("fixers/pint_laravel_blade/prettierrc.json");
const WORKER_IDLE_TIMEOUT: Duration = Duration::from_secs(30);

enum Chunk {
    Out(Vec<u8>),
    Err(Vec<u8>),
    Closed,
}

struct Worker {
    child: Child,
    stdin: ChildStdin,
    events: Receiver<Chunk>,
}

impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

// Only the unclaimed probe worker is shared. The first formatting thread takes ownership, so formatting remains
// parallel instead of serializing all Blade files behind a process-wide mutex.
static PROBED_WORKER: Mutex<Option<Worker>> = Mutex::new(None);

fn probed_worker() -> std::sync::MutexGuard<'static, Option<Worker>> {
    PROBED_WORKER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

thread_local! {
    static WORKER_PROCESS: RefCell<Option<Worker>> = const { RefCell::new(None) };
}

/// EnsurePrettierIsConfigured's package versions, from a preload in the worker's own Node start. A missing package
/// is `None`, an unnamed manifest is `Some("")`; failure to start Node or read the probe is outer `None`.
pub fn probe(packages: &[&str]) -> Option<Vec<Option<String>>> {
    stop();
    probe_with(start(Some(packages)).ok()?, WORKER_IDLE_TIMEOUT)
}

fn probe_with(worker: Worker, idle_timeout: Duration) -> Option<Vec<Option<String>>> {
    let mut line = Vec::new();
    let mut open = 2;
    while open > 0 && !line.contains(&b'\n') {
        match worker.events.recv_timeout(idle_timeout) {
            Ok(Chunk::Out(c)) => line.extend(c),
            Ok(Chunk::Err(_)) => {}
            Ok(Chunk::Closed) => open -= 1,
            Err(_) => break,
        }
    }
    let probes = line
        .split(|b| *b == b'\n')
        .next()
        .and_then(|l| serde_json::from_slice(l).ok());
    if probes.is_some() {
        *probed_worker() = Some(worker);
    }
    probes
}

/// Stop the probe's worker if no formatting thread claimed it. Safe to call when no probe ran.
pub fn stop() {
    drop(probed_worker().take());
}

/// The scripts and config are content-addressed, resolved once per process and written via unique temporary
/// filenames and rename, so concurrent writers never expose partial files.
fn resources() -> Result<PathBuf> {
    static RESOURCES: OnceLock<Result<PathBuf, String>> = OnceLock::new();
    RESOURCES
        .get_or_init(|| {
            let mut hasher = std::hash::DefaultHasher::new();
            std::hash::Hash::hash(&(WORKER, PROBE, PRETTIERRC), &mut hasher);
            let hash = std::hash::Hasher::finish(&hasher);
            resources_in(&std::env::temp_dir().join(format!("pun-blade-{hash:016x}")))
                .map_err(|e| e.to_string())
        })
        .clone()
        .map_err(anyhow::Error::msg)
}

fn resources_in(dir: &Path) -> Result<PathBuf> {
    let files = [
        ("prettierrc.json", PRETTIERRC),
        ("probe.cjs", PROBE),
        ("worker.cjs", WORKER),
    ];
    if !files.iter().all(|(name, _)| dir.join(name).exists()) {
        std::fs::create_dir_all(dir)?;
        for (name, body) in files {
            static WRITES: AtomicUsize = AtomicUsize::new(0);
            let n = WRITES.fetch_add(1, Ordering::Relaxed);
            let tmp = dir.join(format!("{name}.{}-{n}", std::process::id()));
            std::fs::write(&tmp, body)?;
            std::fs::rename(&tmp, dir.join(name))?;
        }
    }
    Ok(dir.to_path_buf())
}

fn start(probe: Option<&[&str]>) -> Result<Worker> {
    let project_root = std::env::current_dir()?; // Project::path() is getcwd()
    let dir = resources()?;
    let mut node = Command::new(runtime_binary(&project_root));
    if let Some(packages) = probe {
        node.arg("--require")
            .arg(dir.join("probe.cjs"))
            .env("PUN_PROBE", packages.join(","));
    }
    node.arg(dir.join("worker.cjs"))
        .arg(&project_root)
        .arg(dir.join("prettierrc.json"))
        .current_dir(&project_root);
    spawn(node)
}

/// `NodePackageManager::detect($projectRoot)->runtimeBinary()`: Bun for a project with a Bun lock file, else Node.
fn runtime_binary(project_root: &Path) -> &'static str {
    match project_root.join("bun.lock").exists() || project_root.join("bun.lockb").exists() {
        true => "bun",
        false => "node",
    }
}

fn spawn(mut command: Command) -> Result<Worker> {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdin = child.stdin.take().unwrap();
    let (tx, events) = channel();
    for (mut pipe, is_err) in [
        (
            Box::new(child.stdout.take().unwrap()) as Box<dyn Read + Send>,
            false,
        ),
        (Box::new(child.stderr.take().unwrap()), true),
    ] {
        let tx = tx.clone();
        std::thread::spawn(move || {
            #[expect(
                clippy::large_stack_arrays,
                reason = "a pipe-read buffer on its own thread's 2 MiB stack"
            )]
            let mut buf = [0u8; 65536];
            loop {
                match pipe.read(&mut buf) {
                    Ok(0) | Err(_) => {
                        let _ = tx.send(Chunk::Closed);
                        return;
                    }
                    Ok(n) => {
                        let chunk = buf[..n].to_vec();
                        let _ = tx.send(if is_err {
                            Chunk::Err(chunk)
                        } else {
                            Chunk::Out(chunk)
                        });
                    }
                }
            }
        });
    }
    Ok(Worker {
        child,
        stdin,
        events,
    })
}

const START: &[u8] = b"[PINT_PRETTIER_WORKER_START]";
const END: &[u8] = b"[PINT_PRETTIER_WORKER_END]";

/// Format with the probe's worker or this thread's own worker, starting a fresh one after a worker failure.
pub fn format(path: &str, content: &[u8]) -> Result<Vec<u8>> {
    WORKER_PROCESS.with(|w| {
        let mut guard = w.borrow_mut();
        if guard.is_none() {
            let probed = probed_worker().take();
            *guard = Some(match probed {
                Some(worker) => worker,
                None => start(None)?,
            });
        }
        format_with(&mut guard, path, content, WORKER_IDLE_TIMEOUT)
    })
}

/// One request to the started worker in `guard`; a failure tears it down (the next file starts a fresh one).
fn format_with(
    guard: &mut Option<Worker>,
    path: &str,
    content: &[u8],
    idle_timeout: Duration,
) -> Result<Vec<u8>> {
    let worker = guard.as_mut().unwrap();
    // json_encode(..., JSON_INVALID_UTF8_SUBSTITUTE)
    let request = serde_json::json!({ "path": path, "content": String::from_utf8_lossy(content) });
    let _ = writeln!(worker.stdin, "{request}").and_then(|()| worker.stdin.flush());

    let mut formatted: Vec<u8> = Vec::new();
    let failure = loop {
        if strpos(&formatted, END, 0).is_some() {
            // Str::after(START)->before(END), after checking both markers are present.
            let Some(start) = strpos(&formatted, START, 0) else {
                bail!("Laravel Pint's Prettier worker did not return a valid response.");
            };
            let body = &formatted[start + START.len()..];
            return Ok(body[..strpos(body, END, 0).unwrap_or(body.len())].to_vec());
        }
        match worker.events.recv_timeout(idle_timeout) {
            Ok(Chunk::Out(c)) => formatted.extend(c),
            event => break event,
        }
    };
    std::thread::sleep(Duration::from_millis(100));
    let timed_out = matches!(failure, Err(RecvTimeoutError::Timeout));
    let error: Vec<u8> = std::iter::once(failure.ok())
        .flatten()
        .chain(worker.events.try_iter())
        .flat_map(|e| if let Chunk::Err(c) = e { c } else { Vec::new() })
        .collect();
    drop(guard.take());
    let message = if timed_out {
        format!("Laravel Pint's Prettier worker timed out while formatting [{path}].")
    } else if error.is_empty() {
        "Laravel Pint's Prettier worker terminated unexpectedly.".into()
    } else {
        String::from_utf8_lossy(&error).into_owned()
    };
    bail!("{message}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sh(script: &str) -> Worker {
        let mut sh = Command::new("sh");
        sh.args(["-c", script]);
        spawn(sh).unwrap()
    }

    /// Pint's Prettier::format() failures: silence, exiting, and error output.
    #[test]
    fn worker_failures() {
        for (script, idle, message) in [
            (
                "cat >/dev/null",
                Duration::from_millis(50),
                "Laravel Pint's Prettier worker timed out while formatting [a.blade.php].",
            ),
            (
                "exit 0",
                WORKER_IDLE_TIMEOUT,
                "Laravel Pint's Prettier worker terminated unexpectedly.",
            ),
            ("echo boom >&2; sleep 5", WORKER_IDLE_TIMEOUT, "boom\n"),
        ] {
            let mut guard = Some(sh(script));
            let err = format_with(&mut guard, "a.blade.php", b"x", idle).unwrap_err();
            assert_eq!(err.to_string(), message);
            assert!(guard.is_none());
        }
    }

    /// Preserve Str::after(START)->before(END), including a stray earlier end marker.
    #[test]
    fn worker_replies() {
        let mut guard = Some(sh("printf 'x[PINT_PRETTIER_WORKER_END]'; cat >/dev/null"));
        let err = format_with(&mut guard, "a.blade.php", b"x", WORKER_IDLE_TIMEOUT).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Laravel Pint's Prettier worker did not return a valid response."
        );
        assert!(guard.is_some());
        let mut guard = Some(sh(
            "printf 'x[PINT_PRETTIER_WORKER_END]y[PINT_PRETTIER_WORKER_START]abc'; cat >/dev/null",
        ));
        assert_eq!(
            format_with(&mut guard, "a.blade.php", b"x", WORKER_IDLE_TIMEOUT).unwrap(),
            b"abc"
        );
    }

    #[test]
    fn resources_written_concurrently() {
        let dir = std::env::temp_dir().join(format!("pun-blade-race-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let barrier = std::sync::Barrier::new(16);
        std::thread::scope(|s| {
            for _ in 0..16 {
                s.spawn(|| {
                    barrier.wait();
                    resources_in(&dir).unwrap();
                });
            }
        });
        for (name, body) in [
            ("worker.cjs", WORKER),
            ("probe.cjs", PROBE),
            ("prettierrc.json", PRETTIERRC),
        ] {
            assert_eq!(std::fs::read_to_string(dir.join(name)).unwrap(), body);
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn resources_are_shared() {
        let dir = resources().unwrap();
        assert_ne!(
            dir.file_name().unwrap().to_string_lossy(),
            format!("pun-blade-{}", std::process::id())
        );
        let dir = std::env::temp_dir().join(format!("pun-blade-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        resources_in(&dir).unwrap();
        for (name, body) in [
            ("worker.cjs", WORKER),
            ("probe.cjs", PROBE),
            ("prettierrc.json", PRETTIERRC),
        ] {
            assert_eq!(std::fs::read_to_string(dir.join(name)).unwrap(), body);
        }
        std::fs::write(dir.join("worker.cjs"), "kept").unwrap();
        resources_in(&dir).unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join("worker.cjs")).unwrap(),
            "kept"
        );
        for (name, body) in [("prettierrc.json", PRETTIERRC), ("probe.cjs", PROBE)] {
            std::fs::remove_file(dir.join(name)).unwrap();
            resources_in(&dir).unwrap();
            assert_eq!(std::fs::read_to_string(dir.join(name)).unwrap(), body);
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
