//! Shared by the tests that run `pun format` next to Pint. Human output is Pun's own (output.md §1), so a human run
//! that ends in a summary (Pint's Termwind box, Pun's summary line) is compared by the files each tool names; abort
//! boxes, `-q`, stdin and every `--format` stay byte for byte.

/// Whether `args` print human output: no `--format`, no `-q` (alone or in a short-option group), no stdin.
pub fn human(args: &[&str]) -> bool {
    !args.iter().any(|a| {
        a.starts_with("--format")
            || a.starts_with("--stdin")
            || *a == "--quiet"
            || *a == "-"
            || (a.starts_with('-') && !a.starts_with("--") && a.contains('q'))
    })
}

/// Whether `line` is Pun's summary line, which always ends with the run's time (`0.12s`).
fn pun_summary(line: &str) -> bool {
    let time = line.strip_suffix('s').and_then(|l| l.rsplit(' ').next());
    time.is_some_and(|t| {
        t.len() >= 4 && t.as_bytes()[t.len() - 3] == b'.' && t.parse::<f64>().is_ok()
    })
}

/// Pint's and Pun's stdout of one human run, made comparable: when Pint printed its summary and Pun its summary line,
/// each becomes the files it names (Pint's issue lines `⨯`, `✓`, `!`; Pun's file lines and error frames, but not its
/// dim `-v` lines for unchanged files, which Pint doesn't list), followed by what Pun printed after its summary (an
/// abort box) when Pint's output ends with it too. Anything else (an abort box alone) stays as it is.
pub fn human_pair<'a>(
    pint: &str,
    pun: &str,
    pint_files: impl IntoIterator<Item = &'a String>,
    pun_files: impl IntoIterator<Item = &'a String>,
) -> (String, String) {
    let plain = strip_sgr(pun);
    let Some(end) = plain
        .lines()
        .collect::<Vec<_>>()
        .into_iter()
        .rposition(pun_summary)
    else {
        return (pint.to_string(), pun.to_string());
    };
    if !pint.contains("────") {
        return (pint.to_string(), pun.to_string());
    }
    let tail: String = pun.split_inclusive('\n').skip(end + 1).collect();
    let pint_tail = if pint.ends_with(&tail) {
        tail.clone()
    } else {
        "(Pint's output ends differently)".to_string()
    };
    let head: Vec<&str> = plain.lines().take(end).collect();
    (
        format!("{}\n{pint_tail}", named(&strip_sgr(pint), pint_files)),
        format!("{}\n{tail}", named(&head.join("\n"), pun_files)),
    )
}

/// The files of `files` that `text` names after a space, before a space, a line end or a `:`.
fn named<'a>(text: &str, files: impl IntoIterator<Item = &'a String>) -> String {
    let lines: Vec<&str> = (text.lines())
        .filter(|l| {
            let bare = l.strip_prefix(" ✔ ").or_else(|| l.strip_prefix(" ok "));
            bare.is_none_or(|rest| rest.contains("  "))
        })
        .collect();
    let named: Vec<&str> = (files.into_iter())
        .filter(|rel| {
            lines.iter().any(|l| {
                [" ", "\n", ":"]
                    .iter()
                    .any(|end| format!("{l}\n").contains(&format!(" {rel}{end}")))
            })
        })
        .map(String::as_str)
        .collect();
    format!("named: {}", named.join(", "))
}

/// `text` without SGR sequences (`ESC [ … m`).
pub fn strip_sgr(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(i) = rest.find("\x1b[") {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        rest = rest.find('m').map_or("", |m| &rest[m + 1..]);
    }
    out.push_str(rest);
    out
}
