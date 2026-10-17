//! Run errors as Pint shows them: Symfony's renderThrowable box on stdout, exit 1, with the "In X.php line N:" header
//! of the PHP exception that carries the same message.

use crate::{console, report};

/// The box for `message` (one line in agent mode), coloured like Symfony's configureIO decides (raw `--ansi`/`--no-ansi` in argv, else TTY).
/// `-v` (also from raw argv) adds the exception class above the message and the trace below the box. Of the trace,
/// Pun prints only its first line, the throw site, as a path inside the PHAR (`phar://pint/vendor/...`, where Pint
/// has its absolute PHAR path); the PHP call frames under it are left out, as is the requirements check the PHAR's
/// stub prints on stderr at `-vv`.
pub fn render(message: &str) -> String {
    if crate::agent::enabled() {
        return crate::agent::error(message);
    }
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
        out += &report::abort_box(
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

/// A run that aborts on a file (unreadable, unwritable) has already printed the progress of the files before it.
pub fn progress_before_abort<T, U>(
    results: &[anyhow::Result<(report::FileResult, T, bool, U)>],
    dry_run: bool,
    ansi: bool,
    no_ansi: bool,
) {
    let Some(i) = results.iter().position(Result::is_err) else {
        return;
    };
    if crate::agent::enabled() {
        return;
    }
    let decorated = console::decorated(1, ansi, no_ansi);
    let done = results[..i].iter().flatten().map(|r| &r.0);
    print!(
        "{}",
        console::format(
            &report::progress(done, console::width(), dry_run, decorated),
            decorated
        )
    );
}
