//! Run errors as Pint shows them: Symfony's renderThrowable box on stdout, exit 1, with the "In X.php line N:" header
//! of the PHP exception that carries the same message.

use crate::console::{self, escape};
use std::fmt::Write as _;

/// The box for `message`, coloured like Symfony's configureIO decides (raw `--ansi`/`--no-ansi` in argv, else TTY).
/// `-v` (also from raw argv) adds the exception class above the message and the trace below the box. Of the trace,
/// Pun prints only its first line, the throw site, as a path inside the PHAR (`phar://pint/vendor/...`, where Pint
/// has its absolute PHAR path); the PHP call frames under it are left out, as is the requirements check the PHAR's
/// stub prints on stderr at `-vv`.
pub fn render(message: &str) -> String {
    let args: Vec<String> = std::env::args().skip(1).take_while(|a| a != "--").collect();
    let has = |flag: &str| args.iter().any(|a| a == flag);
    let decorated = console::decorated(1, has("--ansi"), has("--no-ansi"));
    // configureIO: quiet wins; hasParameterOption matches a short option by prefix (`-qv` is quiet, `-vq` verbose).
    let opt = |long: &str, short: &str| {
        args.iter()
            .any(|a| a == long || a.starts_with(&format!("{long}=")) || a.starts_with(short))
    };
    let verbose = !opt("--quiet", "-q") && opt("--verbose", "-v");
    let mut out = String::from("\n");
    // Symfony also renders the previous exception: FixerAnnotationAnalyzer's, wrapped by Runner::fixFile.
    let nested = message
        .strip_prefix("Error while analysing file \"")
        .and_then(|m| m.split_once("\": "))
        .map(|(_, inner)| inner);
    for (i, m) in std::iter::once(message).chain(nested).enumerate() {
        let (file, line, class, console_error) = origin(m, i > 0);
        // Console exceptions (option errors, `abort()`) name where they were thrown only with -v.
        if verbose || !console_error {
            let name = file.rsplit('/').next().unwrap_or(file);
            out += &console::format(
                &format!("<comment>In {name} line {line}:</comment>\n"),
                decorated,
            );
        }
        out += &abort_box(
            verbose.then(|| format!("  [{class}]  ")).as_deref(),
            m,
            decorated,
        );
        if verbose {
            out += &console::format(
                &format!(
                    "<comment>Exception trace:</comment>\n  at <info>phar://pint/{file}:{line}</info>\n\n"
                ),
                decorated,
            );
        }
    }
    out
}

/// Where Pint throws the exception with this message: the file in the PHAR, line, class (with a non-zero code) and
/// whether it is a Symfony Console exception. Anything else is Laravel Zero's `abort()`.
fn origin(m: &str, nested: bool) -> (&'static str, u32, &'static str, bool) {
    let between = |a: &str, b: &str| m.starts_with(a) && m.ends_with(b);
    let at = |file: &'static str, line, class| (file, line, class, false);
    if between("The path \"", "\" is not readable.") {
        at(
            "vendor/friendsofphp/php-cs-fixer/src/Console/ConfigurationResolver.php",
            409,
            "PhpCsFixer\\ConfigurationException\\InvalidConfigurationException (16)",
        )
    } else if between("The \"", "\" directory does not exist.") {
        at(
            "vendor/symfony/finder/Finder.php",
            648,
            "Symfony\\Component\\Finder\\Exception\\DirectoryNotFoundException",
        )
    } else if between("Failed to read content from \"", "\".") {
        at(
            "vendor/friendsofphp/php-cs-fixer/src/FileReader.php",
            63,
            "RuntimeException",
        )
    } else if between("Cannot write to file \"", "\" as it is not writable.") {
        at(
            "vendor/friendsofphp/php-cs-fixer/src/Runner/Runner.php",
            732,
            "Symfony\\Component\\Filesystem\\Exception\\IOException",
        )
    } else if m.starts_with("@php-cs-fixer-ignore annotation(s) used for rules that are not in") {
        at(
            "vendor/friendsofphp/php-cs-fixer/src/Runner/Runner.php",
            271,
            "RuntimeException",
        )
    } else if m.starts_with("Error while analysing file \"") {
        at(
            "vendor/friendsofphp/php-cs-fixer/src/Runner/Runner.php",
            598,
            "RuntimeException",
        )
    } else if nested {
        at(
            "vendor/friendsofphp/php-cs-fixer/src/Tokenizer/Analyzer/FixerAnnotationAnalyzer.php",
            115,
            "RuntimeException",
        )
    } else if m == "Undefined array key 1" {
        at(
            "vendor/friendsofphp/php-cs-fixer/src/Tokenizer/Analyzer/FixerAnnotationAnalyzer.php",
            103,
            "ErrorException",
        )
    } else if m.starts_with("file_put_contents(") {
        at("app/Actions/ElaborateSummary.php", 106, "ErrorException")
    } else {
        (
            "vendor/laravel-zero/framework/src/Application.php",
            154,
            "LaravelZero\\Framework\\Exceptions\\ConsoleException",
            true,
        )
    }
}

/// Symfony's doRenderThrowable box (without the blank line before it), on stdout; `title` is `-v`'s exception class line.
fn abort_box(title: Option<&str>, message: &str, decorated: bool) -> String {
    let width = match console::width() {
        0 => usize::MAX,
        w => w - 1,
    };
    let chunk = width.saturating_sub(4).max(1);
    let mut lines: Vec<String> = vec![];
    for line in message
        .split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
    {
        // splitStringByWidth: every piece is padded to the chunk width once a line needs splitting.
        let chars: Vec<char> = line.chars().collect();
        if chars.len() <= chunk {
            lines.push(line.to_string());
        } else {
            lines.extend(
                chars
                    .chunks(chunk)
                    .map(|c| format!("{:chunk$}", c.iter().collect::<String>())),
            );
        }
    }
    let title_len = title.map_or(0, |t| t.chars().count());
    let len = lines
        .iter()
        .map(|l| l.chars().count() + 4)
        .max()
        .unwrap_or(0)
        .max(title_len);
    let empty = format!("<error>{}</error>", " ".repeat(len));
    let mut o = format!("{empty}\n");
    if let Some(t) = title {
        let _ = writeln!(o, "<error>{t}{}</error>", " ".repeat(len - title_len));
    }
    for l in &lines {
        let _ = writeln!(
            o,
            "<error>  {}  {}</error>",
            escape(l),
            " ".repeat(len - l.chars().count() - 4)
        );
    }
    let _ = write!(o, "{empty}\n\n");
    console::format(&o, decorated)
}
