//! Agent mode (context/PLAN.md "Agent-optimised output"): under a coding agent every command prints a summary line,
//! then one compiler-style line per finding (`path:line:col rule [fix:safe] message`), capped unless `-v`. No
//! colours, boxes or progress. Exit codes don't change. Humans (no agent detected) get the Pint output as before.
use std::fmt::Write as _;
use std::sync::OnceLock;
use std::time::Instant;

/// Findings shown without `-v`.
const CAP: usize = 20;

struct Mode {
    on: bool,
    cmd: &'static str,
    start: Instant,
}

static MODE: OnceLock<Mode> = OnceLock::new();

/// Decide once per run: `--agent`/`--no-agent`, then `PUN_AGENT`, then PAO's `PAO_DISABLE`/`PAO_FORCE`, then
/// the agent-detector crate (env variables only). A missing TTY alone is not an agent.
pub fn init(flag: Option<bool>, cmd: &'static str) {
    let on = flag
        .or_else(|| boolean("PUN_AGENT"))
        .or_else(|| boolean("PAO_DISABLE").filter(|d| *d).map(|_| false))
        .or_else(|| boolean("PAO_FORCE").filter(|f| *f))
        .unwrap_or_else(agent_detector::is_agent);
    let _ = MODE.set(Mode {
        on,
        cmd,
        start: Instant::now(),
    });
}

/// PHP's FILTER_VALIDATE_BOOLEAN (as PAO reads its variables); `None` when unset, empty or neither true nor false.
fn boolean(var: &str) -> Option<bool> {
    let v = std::env::var(var).ok()?.trim().to_ascii_lowercase();
    match v.as_str() {
        "1" | "true" | "on" | "yes" => Some(true),
        "0" | "false" | "off" | "no" => Some(false),
        _ => None,
    }
}

pub fn enabled() -> bool {
    MODE.get().is_some_and(|m| m.on)
}

fn mode() -> &'static Mode {
    MODE.get().expect("agent::init runs first")
}

pub fn cmd() -> &'static str {
    mode().cmd
}

/// `pun <cmd>: <result>[, <detail>] ([<checked>, ]0.21s)` and the findings, capped at CAP unless `all`.
pub fn render(
    result: &str,
    detail: &[String],
    checked: &str,
    items: &[String],
    all: bool,
) -> String {
    let m = mode();
    let mut o = format!("pun {}: {result}", m.cmd);
    for d in detail {
        let _ = write!(o, ", {d}");
    }
    let sep = if checked.is_empty() { "" } else { ", " };
    let _ = writeln!(
        o,
        " ({checked}{sep}{:.2}s)",
        m.start.elapsed().as_secs_f64()
    );
    let shown = if all {
        items.len()
    } else {
        items.len().min(CAP)
    };
    for i in &items[..shown] {
        let _ = writeln!(o, "{i}");
    }
    if shown < items.len() {
        let _ = writeln!(o, "+{} more; run with -v for all", items.len() - shown);
    }
    o
}

/// A run error, as one line.
pub fn error(message: &str) -> String {
    let one: Vec<&str> = message
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    format!("pun {}: error: {}\n", cmd(), one.join(" "))
}

/// `n file` / `n files`.
pub fn count(n: usize, what: &str) -> String {
    format!("{n} {what}{}", if n == 1 { "" } else { "s" })
}
