//! `pint` mode errors as Pint shows them: Symfony's renderThrowable box on stdout, exit 1. ArgvInput's option
//! errors replace clap's usage errors (exit 2); run errors get the "In X.php line N:" header of the PHP exception
//! that carries the same message.
//! ponytail: `-v` adds the exception class and a phar stack trace in Pint; Pun prints the non-verbose box.

use crate::{console, report};
use clap::error::{ContextKind, ContextValue, ErrorKind};

/// ArgvInput's message for a clap parse error; `None` for help/version and Pun-only value checks (clap prints those).
pub fn from_clap(e: &clap::Error) -> Option<String> {
    let Some(ContextValue::String(arg)) = e.get(ContextKind::InvalidArg) else { return None };
    // "--x=1" -> "--x", "--config <CONFIG>" -> "--config"
    let name = arg.split('=').next().unwrap_or(arg).split(" <").next().unwrap_or(arg);
    let empty_value = matches!(e.get(ContextKind::InvalidValue), Some(ContextValue::String(v)) if v.is_empty());
    Some(match e.kind() {
        ErrorKind::UnknownArgument => format!("The \"{name}\" option does not exist."),
        ErrorKind::TooManyValues => format!("The \"{name}\" option does not accept a value."),
        ErrorKind::InvalidValue if empty_value => format!("The \"{name}\" option requires a value."),
        _ => return None,
    })
}

/// The box for `message`, coloured like Symfony's configureIO decides (raw `--ansi`/`--no-ansi` in argv, else TTY).
pub fn render(message: &str) -> String {
    let has = |flag: &str| std::env::args().skip(1).any(|a| a == flag);
    let decorated = console::decorated(1, has("--ansi"), has("--no-ansi"));
    let header = header(message).map(|h| console::format(&format!("\n<comment>In {h}:</comment>"), decorated)).unwrap_or_default();
    let mut out = header + &report::abort_box(message, decorated);
    // Symfony also renders the previous exception: FixerAnnotationAnalyzer's, wrapped by Runner::fixFile.
    if let Some((_, inner)) = message.strip_prefix("Error while analysing file \"").and_then(|m| m.split_once("\": ")) {
        out += &console::format("<comment>In FixerAnnotationAnalyzer.php line 115:</comment>", decorated);
        out += &report::abort_box(inner, decorated);
    }
    out
}

/// Where Pint throws the exception with this message; `None` for Pint's `abort()`s and option errors (no header).
fn header(m: &str) -> Option<&'static str> {
    let between = |a: &str, b: &str| m.starts_with(a) && m.ends_with(b);
    Some(if between("The path \"", "\" is not readable.") {
        "ConfigurationResolver.php line 409"
    } else if between("The \"", "\" directory does not exist.") {
        "Finder.php line 648"
    } else if m == "Pint configuration cannot extend from more than 1 file." {
        "ConfigurationJsonRepository.php line 149"
    } else if between("Failed to read content from \"", "\".") {
        "FileReader.php line 63"
    } else if between("Cannot write to file \"", "\" as it is not writable.") {
        "Runner.php line 732"
    } else if m.starts_with("@php-cs-fixer-ignore annotation(s) used for rules that are not in") {
        "Runner.php line 271"
    } else if m.starts_with("Error while analysing file \"") {
        "Runner.php line 598"
    } else if m == "Undefined array key 1" {
        "FixerAnnotationAnalyzer.php line 103"
    } else if m.starts_with('[') && m.contains(" must be enabled (true)") {
        "RuleSet.php line 62"
    } else {
        return None;
    })
}

/// A run that aborts on a file (unreadable, unwritable) has already printed the progress of the files before it.
pub fn progress_before_abort<T>(results: &[anyhow::Result<(report::FileResult, T, bool)>], dry_run: bool, ansi: bool, no_ansi: bool) {
    let Some(i) = results.iter().position(Result::is_err) else { return };
    if report::running_in_agent() {
        return;
    }
    let decorated = console::decorated(1, ansi, no_ansi);
    let done = results[..i].iter().flatten().map(|r| &r.0);
    print!("{}", console::format(&report::progress(done, console::width(), dry_run, decorated), decorated));
}
