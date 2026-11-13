//! The renderer against output.md §1's samples, copied from the plan by hand, with and without colour. Colour codes
//! are written out from design/tokens.json by hand (`#FF7AB8` is `255;122;184`).

use super::*;
use pun_problem::{Edit, Fix, Span};
use std::io::Write;
use std::sync::{Arc, Mutex};

/// `gum`, `mint`, `ok`, `ink-3` and `zap`, dark variants, at 24 bits; and the reset.
const E: &str = "\x1b[38;2;255;122;184m";
const OK: &str = "\x1b[38;2;142;240;200m";
const R: &str = "\x1b[38;2;143;184;255m";
const D: &str = "\x1b[38;2;156;146;174m";
const W: &str = "\x1b[38;2;255;229;61m";
const Z: &str = "\x1b[0m";

fn colour() -> Look {
    Look {
        depth: Depth::TrueColor,
        theme: Theme::Dark,
        unicode: true,
        links: false,
    }
}

fn ascii() -> Look {
    Look {
        unicode: false,
        ..Look::plain()
    }
}

fn plain() -> Renderer {
    Renderer::new(Look::plain(), "/project")
}

fn painted() -> Renderer {
    Renderer::new(colour(), "/project")
}

fn problem(
    rule: &str,
    path: &str,
    span: (usize, usize),
    message: &str,
    help: Option<&str>,
    level: Level,
) -> Problem {
    Problem {
        rule: rule.into(),
        path: path.into(),
        span: Span::new(span.0, span.1),
        message: message.into(),
        help: help.map(Into::into),
        level,
        fix: None,
    }
}

fn safe_fix() -> Option<Fix> {
    Some(Fix {
        safety: Safety::Safe,
        edits: vec![Edit::new(Span::new(0, 0), "")],
    })
}

/// 40 filler lines, then lines 41 and 42 of PostController (indented 8 spaces), then a closing brace.
fn post_controller() -> (String, Problem) {
    let mut source = "<?php\n".repeat(40);
    source.push_str("        $post = Post::find($id);\n        dd($post);\n}\n");
    let start = source.find("dd(").unwrap();
    let mut p = problem(
        "lint/laravel/no-debug-calls",
        "app/Http/Controllers/PostController.php",
        (start, start + "dd($post)".len()),
        "dd() left in the code",
        Some("remove the dd() call, or use Log::debug()"),
        Level::Error,
    );
    p.fix = safe_fix();
    (source, p)
}

/// Line 22 is blank, line 23 is `\treturn $user->fullName();` (a tab is one byte, so `fullName` is column 16).
fn profile_controller() -> (String, Problem) {
    let mut source = "<?php\n".repeat(21);
    source.push_str("\n\treturn $user->fullName();\n");
    let start = source.find("fullName").unwrap();
    let p = problem(
        "analyse/undefined/method",
        "app/Http/Controllers/ProfileController.php",
        (start, start + "fullName".len()),
        r"Call to undefined method App\Models\User::fullName()",
        Some("did you mean `full_name` (a column on users)?"),
        Level::Error,
    );
    (source, p)
}

fn render(r: &Renderer, (source, p): &(String, Problem)) -> String {
    r.problem(p, source, &LineIndex::new(source))
}

#[test]
fn file_line_sample() {
    let files = [FileLine {
        path: "app/Models/User.php".into(),
        status: FileStatus::Fixed,
        rules: vec![
            "format/imports/ordered".into(),
            "lint/dead-code/no-unused-private-method".into(),
        ],
    }];
    assert_eq!(
        plain().file_lines(&files).concat(),
        " ✔ app/Models/User.php        format/imports/ordered, lint/dead-code/no-unused-private-method\n"
    );
    assert_eq!(
        painted().file_lines(&files).concat(),
        format!(
            " {OK}✔{Z} app/Models/User.php        {R}format/imports/ordered{Z}, {R}lint/dead-code/no-unused-private-method{Z}\n"
        )
    );
}

#[test]
fn file_lines_align_rules_after_the_longest_path() {
    let files = [
        FileLine {
            path: "app/Http/Controllers/PostController.php".into(),
            status: FileStatus::NeedsFix,
            rules: vec!["format/imports/ordered".into()],
        },
        FileLine {
            path: "app/Kernel.php".into(),
            status: FileStatus::NeedsFix,
            rules: vec!["format/whitespace/no-trailing".into()],
        },
        FileLine {
            path: "app/Clean.php".into(),
            status: FileStatus::Unchanged,
            rules: vec![],
        },
    ];
    assert_eq!(
        plain().file_lines(&files).concat(),
        concat!(
            " ✖ app/Http/Controllers/PostController.php  format/imports/ordered\n",
            " ✖ app/Kernel.php                           format/whitespace/no-trailing\n",
            " ✔ app/Clean.php\n",
        )
    );
    assert_eq!(
        painted().file_lines(&files[2..]).concat(),
        format!(" {D}✔{Z} {D}app/Clean.php{Z}\n")
    );
    assert_eq!(
        Renderer::new(ascii(), "/p")
            .file_lines(&files[1..])
            .concat(),
        " x app/Kernel.php             format/whitespace/no-trailing\n ok app/Clean.php\n"
    );
}

#[test]
fn problem_block_sample_with_context_and_fixable() {
    let input = post_controller();
    assert_eq!(
        render(&plain(), &input),
        concat!(
            " ✖ lint/laravel/no-debug-calls  dd() left in the code                        fixable\n",
            "   ╭─ app/Http/Controllers/PostController.php:42:9\n",
            " 41│     $post = Post::find($id);\n",
            " 42│     dd($post);\n",
            "   │     ━━━━━━━━━\n",
            "   ╰─ help: remove the dd() call, or use Log::debug()\n",
        )
    );
    assert_eq!(
        render(&painted(), &input),
        format!(
            " {E}✖{Z} {R}lint/laravel/no-debug-calls{Z}  dd() left in the code                        {D}fixable{Z}\n\
             \x20  {D}╭─{Z} app/Http/Controllers/PostController.php:42:9\n\
             \x20{D}41{Z}{D}│{Z}     $post = Post::find($id);\n\
             \x20{D}42{Z}{D}│{Z}     dd($post);\n\
             \x20  {D}│{Z}     {E}━━━━━━━━━{Z}\n\
             \x20  {D}╰─{Z} help: remove the dd() call, or use Log::debug()\n"
        )
    );
}

#[test]
fn problem_block_sample_warning_header() {
    let mut source = "<?php\n".repeat(16);
    source.push_str("    if ($paid) {\n");
    let start = source.find("if").unwrap();
    let p = problem(
        "lint/style/prefer-early-return",
        "app/Services/Billing.php",
        (start, start + 2),
        "return early instead of nesting the if",
        None,
        Level::Warning,
    );
    let index = LineIndex::new(&source);
    assert!(plain().problem(&p, &source, &index).starts_with(concat!(
        " ⚠ lint/style/prefer-early-return  return early instead of nesting the if\n",
        "   ╭─ app/Services/Billing.php:17:5\n",
    )));
    assert!(painted().problem(&p, &source, &index).starts_with(&format!(
        " {W}⚠{Z} {R}lint/style/prefer-early-return{Z}  return early instead of nesting the if\n\
         \x20  {D}╭─{Z} app/Services/Billing.php:17:5\n"
    )));
}

#[test]
fn problem_block_sample_tab_and_no_context_after_a_blank_line() {
    let input = profile_controller();
    assert_eq!(
        render(&plain(), &input),
        concat!(
            " ✖ analyse/undefined/method  Call to undefined method App\\Models\\User::fullName()\n",
            "   ╭─ app/Http/Controllers/ProfileController.php:23:16\n",
            " 23│     return $user->fullName();\n",
            "   │                   ━━━━━━━━\n",
            "   ╰─ help: did you mean `full_name` (a column on users)?\n",
        )
    );
    assert_eq!(
        render(&painted(), &input),
        format!(
            " {E}✖{Z} {R}analyse/undefined/method{Z}  Call to undefined method App\\Models\\User::fullName()\n\
             \x20  {D}╭─{Z} app/Http/Controllers/ProfileController.php:23:16\n\
             \x20{D}23{Z}{D}│{Z}     return $user->fullName();\n\
             \x20  {D}│{Z}                   {E}━━━━━━━━{Z}\n\
             \x20  {D}╰─{Z} help: did you mean `full_name` (a column on users)?\n"
        )
    );
}

#[test]
fn problems_are_separated_by_a_blank_line() {
    let (a, b) = (post_controller(), profile_controller());
    let (ia, ib) = (LineIndex::new(&a.0), LineIndex::new(&b.0));
    let out = plain().problems([(&a.1, a.0.as_str(), &ia), (&b.1, b.0.as_str(), &ib)]);
    assert!(
        out.contains("Log::debug()\n\n ✖ analyse/undefined/method"),
        "{out}"
    );
}

#[test]
fn concise_prints_one_line_per_problem() {
    let (a, b) = (post_controller(), profile_controller());
    let (ia, ib) = (LineIndex::new(&a.0), LineIndex::new(&b.0));
    let mut r = plain();
    r.concise = true;
    assert_eq!(
        r.problems([(&a.1, a.0.as_str(), &ia), (&b.1, b.0.as_str(), &ib)]),
        concat!(
            " ✖ app/Http/Controllers/PostController.php:42:9  lint/laravel/no-debug-calls  dd() left in the code  fixable\n",
            " ✖ app/Http/Controllers/ProfileController.php:23:16  analyse/undefined/method  Call to undefined method App\\Models\\User::fullName()\n",
        )
    );
    let mut r = painted();
    r.concise = true;
    assert_eq!(
        render(&r, &a),
        format!(
            " {E}✖{Z} app/Http/Controllers/PostController.php:42:9  {R}lint/laravel/no-debug-calls{Z}  dd() left in the code  {D}fixable{Z}\n"
        )
    );
}

#[test]
fn ascii_frame() {
    let (source, mut p) = post_controller();
    p.fix.as_mut().unwrap().safety = Safety::Unsafe;
    p.help = None;
    p.message = "a message longer than the label column leaves two spaces before the label".into();
    assert_eq!(
        Renderer::new(ascii(), "/p").problem(&p, &source, &LineIndex::new(&source)),
        concat!(
            " x lint/laravel/no-debug-calls  a message longer than the label column leaves two spaces before the label  fixable (unsafe)\n",
            "   ,- app/Http/Controllers/PostController.php:42:9\n",
            " 41|     $post = Post::find($id);\n",
            " 42|     dd($post);\n",
            "   |     ^^^^^^^^^\n",
            "   `-\n",
        )
    );
}

#[test]
fn multi_line_span_marks_each_line_from_its_code() {
    // Lines 1-3; the span runs from `foo` on line 1 to `)` on line 3. Indentation is 2, under the 4-column cut.
    let source = "  $a = foo(\n    1,\n  );\n";
    let p = problem("lint/x/y", "a.php", (7, 22), "m", None, Level::Error);
    assert_eq!(
        plain().problem(&p, source, &LineIndex::new(source)),
        concat!(
            " ✖ lint/x/y  m\n",
            "  ╭─ a.php:1:8\n",
            " 1│   $a = foo(\n",
            "  │        ━━━━\n",
            " 2│     1,\n",
            "  │     ━━\n",
            " 3│   );\n",
            "  │   ━\n",
            "  ╰─\n",
        )
    );
}

#[test]
fn long_span_shows_first_two_lines_and_the_last() {
    let source = "<?php\nf(\n1,\n2,\n3,\n);\n";
    let p = problem("lint/x/y", "a.php", (6, 20), "m", None, Level::Warning);
    let index = LineIndex::new(source);
    assert_eq!(
        plain().problem(&p, source, &index),
        concat!(
            " ⚠ lint/x/y  m\n",
            "  ╭─ a.php:2:1\n",
            " 1│ <?php\n",
            " 2│ f(\n",
            "  │ ━━\n",
            " 3│ 1,\n",
            "  │ ━━\n",
            " …\n",
            " 6│ );\n",
            "  │ ━━\n",
            "  ╰─\n",
        )
    );
    assert!(
        Renderer::new(ascii(), "/p")
            .problem(&p, source, &index)
            .contains("\n ...\n")
    );
    assert!(
        painted()
            .problem(&p, source, &index)
            .contains(&format!("\n {D}…{Z}\n"))
    );
}

#[test]
fn empty_span_and_span_past_the_line_end_mark_one_column() {
    let source = "<?php\n$a = 1\n";
    let index = LineIndex::new(source);
    let at_end = problem("lint/x/y", "a.php", (12, 13), "m", None, Level::Error);
    let empty = problem("lint/x/y", "a.php", (6, 6), "m", None, Level::Error);
    assert!(
        plain()
            .problem(&at_end, source, &index)
            .contains(" 2│ $a = 1\n  │       ━\n")
    );
    assert!(
        plain()
            .problem(&empty, source, &index)
            .contains(" 2│ $a = 1\n  │ ━\n")
    );
}

#[test]
fn span_inside_deep_indentation_keeps_the_indentation() {
    // The span starts at column 1 of a line indented 12 spaces, so nothing is cut.
    let source = "            foo();\n";
    let p = problem("lint/x/y", "a.php", (0, 18), "m", None, Level::Error);
    assert_eq!(
        plain().problem(&p, source, &LineIndex::new(source)),
        concat!(
            " ✖ lint/x/y  m\n",
            "  ╭─ a.php:1:1\n",
            " 1│             foo();\n",
            "  │ ━━━━━━━━━━━━━━━━━━\n",
            "  ╰─\n",
        )
    );
}

#[test]
fn blank_line_inside_a_span_gets_no_marker() {
    // Indented 8, cut to 4; line 2 is empty.
    let source = "        foo(\n\n        );\n";
    let p = problem("lint/x/y", "a.php", (8, 24), "m", None, Level::Error);
    assert_eq!(
        plain().problem(&p, source, &LineIndex::new(source)),
        concat!(
            " ✖ lint/x/y  m\n",
            "  ╭─ a.php:1:9\n",
            " 1│     foo(\n",
            "  │     ━━━━\n",
            " 2│\n",
            " 3│     );\n",
            "  │     ━━\n",
            "  ╰─\n",
        )
    );
}

#[test]
fn summary_samples() {
    let failing = Summary {
        errors: 3,
        warnings: 1,
        fixable: 2,
        fix_command: "pun check --fix",
        need_formatting: 2,
        files: 412,
        elapsed: Duration::from_millis(310),
    };
    let passing = Summary {
        errors: 0,
        warnings: 0,
        fixable: 0,
        need_formatting: 0,
        elapsed: Duration::from_millis(80),
        ..failing.clone()
    };
    assert_eq!(
        plain().summary(&failing),
        "✖ 3 errors · 1 warning · 2 fixable (run pun check --fix) · 2 files need formatting · 412 files in 0.31s\n"
    );
    assert_eq!(
        plain().summary(&passing),
        "✔ No problems · 412 files in 0.08s\n"
    );
    let dot = format!(" {D}·{Z} ");
    assert_eq!(
        painted().summary(&failing),
        format!(
            "{E}✖{Z} {E}3 errors{Z}{dot}{W}1 warning{Z}{dot}2 fixable (run pun check --fix){dot}2 files need formatting{dot}{D}412 files in 0.31s{Z}\n"
        )
    );
    assert_eq!(
        painted().summary(&passing),
        format!("{OK}✔{Z} {OK}No problems{Z}{dot}{D}412 files in 0.08s{Z}\n")
    );
}

#[test]
fn summary_singulars_and_warnings_only() {
    let warnings = Summary {
        errors: 0,
        warnings: 1,
        fixable: 0,
        fix_command: "pun lint --fix",
        need_formatting: 0,
        files: 1,
        elapsed: Duration::from_millis(12),
    };
    assert_eq!(
        plain().summary(&warnings),
        "⚠ 1 warning · 1 file in 0.01s\n"
    );
    let formatting = Summary {
        warnings: 0,
        need_formatting: 1,
        ..warnings.clone()
    };
    assert_eq!(
        Renderer::new(ascii(), "/p").summary(&formatting),
        "x 1 file needs formatting - 1 file in 0.01s\n"
    );
    let errors = Summary {
        warnings: 0,
        errors: 1,
        ..warnings
    };
    assert_eq!(plain().summary(&errors), "✖ 1 error · 1 file in 0.01s\n");
}

/// init.md §2's "✔ Wrote pun.toml": only the glyph is coloured, `ok` without Unicode.
#[test]
fn done_line() {
    assert_eq!(plain().done("Wrote pun.toml"), "✔ Wrote pun.toml\n");
    assert_eq!(
        painted().done("Wrote pun.toml"),
        format!("{OK}✔{Z} Wrote pun.toml\n")
    );
    let ascii = Renderer::new(ascii(), "/p");
    assert_eq!(ascii.done("Wrote pun.toml"), "ok Wrote pun.toml\n");
}

#[test]
fn run_error_sample() {
    let e = RunError {
        path: Some("pun.toml".into()),
        line: Some(7),
        message: "unknown rule `lint/style/no-short-ternry`".into(),
        hint: Some("did you mean `lint/style/no-short-ternary`?".into()),
    };
    assert_eq!(
        plain().run_error(&e),
        "✖ pun.toml:7  unknown rule `lint/style/no-short-ternry`\n  did you mean `lint/style/no-short-ternary`?\n"
    );
    assert_eq!(
        painted().run_error(&e),
        format!(
            "{E}✖ pun.toml:7  unknown rule `lint/style/no-short-ternry`{Z}\n  did you mean `lint/style/no-short-ternary`?\n"
        )
    );
    let no_line = RunError {
        line: None,
        hint: None,
        message: "cannot read the file".into(),
        ..e
    };
    assert_eq!(
        plain().run_error(&no_line),
        "✖ pun.toml  cannot read the file\n"
    );
    let no_path = RunError {
        path: None,
        ..no_line
    };
    assert_eq!(
        Renderer::new(ascii(), "/p").run_error(&no_path),
        "x cannot read the file\n"
    );
    let two_lines = RunError {
        hint: Some("first\nsecond".into()),
        ..no_path
    };
    assert_eq!(
        plain().run_error(&two_lines),
        "✖ cannot read the file\n  first\n  second\n"
    );
}

#[test]
fn links_wrap_paths_and_rule_ids() {
    let look = Look {
        links: true,
        ..Look::plain()
    };
    let r = Renderer::new(look, "/my project");
    let files = [FileLine {
        path: "a.php".into(),
        status: FileStatus::Fixed,
        rules: vec!["format/arrays/syntax".into(), "syntax".into()],
    }];
    let path = "\x1b]8;;file:///my%20project/a.php\x1b\\a.php\x1b]8;;\x1b\\";
    // A rule links to its docs page; a name that is not a rule has no page.
    let rule = "\x1b]8;;https://pun.laravel.cloud/docs/rules/format/arrays/syntax\x1b\\format/arrays/syntax\x1b]8;;\x1b\\, syntax";
    assert_eq!(
        r.file_lines(&files).concat(),
        format!(" ✔ {path}{}{rule}\n", " ".repeat(22))
    );
    let e = RunError {
        path: Some("a.php".into()),
        line: Some(3),
        message: "m".into(),
        hint: None,
    };
    assert_eq!(
        r.run_error(&e),
        "✖ \x1b]8;;file:///my%20project/a.php\x1b\\a.php:3\x1b]8;;\x1b\\  m\n"
    );
    // Without links, plain text.
    let unlinked = Renderer::new(Look::plain(), "/p");
    assert_eq!(
        unlinked.file_lines(&files).concat(),
        " ✔ a.php                      format/arrays/syntax, syntax\n"
    );
}

#[test]
fn file_url_percent_encodes() {
    assert_eq!(
        file_url(Path::new("/a b/ü~_.-/x.php")),
        "file:///a%20b/%C3%BC~_.-/x.php"
    );
}

#[test]
fn colour_depths_and_themes() {
    let at = |depth, theme| Look {
        depth,
        theme,
        ..Look::plain()
    };
    // 16 colours: the standard codes.
    let basic = at(Depth::Ansi16, Theme::Dark);
    let codes: Vec<String> = [Role::Error, Role::Warning, Role::Ok, Role::Rule, Role::Dim]
        .iter()
        .map(|&r| basic.paint(r, "x"))
        .collect();
    assert_eq!(
        codes,
        [
            "\x1b[31mx\x1b[0m",
            "\x1b[33mx\x1b[0m",
            "\x1b[32mx\x1b[0m",
            "\x1b[36mx\x1b[0m",
            "\x1b[2mx\x1b[0m"
        ]
    );
    // Light variants at 24 bits; warnings and ✔ fall back to the terminal's yellow and green.
    let light = at(Depth::TrueColor, Theme::Light);
    assert_eq!(
        light.paint(Role::Error, "x"),
        "\x1b[38;2;185;58;10mx\x1b[0m"
    );
    assert_eq!(light.paint(Role::Ok, "x"), "\x1b[32mx\x1b[0m");
    assert_eq!(light.paint(Role::Rule, "x"), "\x1b[38;2;29;95;204mx\x1b[0m");
    assert_eq!(light.paint(Role::Dim, "x"), "\x1b[38;2;107;98;128mx\x1b[0m");
    assert_eq!(light.paint(Role::Warning, "x"), "\x1b[33mx\x1b[0m");
    // 256 colours: gum (255,122,184) -> cube (5,2,3) = 211; zap (255,229,61) -> (5,4,1) = 221; ink-3
    // (156,146,174) -> grey 158 = 247, nearer than the cube's (175,135,175).
    let xterm = at(Depth::Ansi256, Theme::Dark);
    assert_eq!(xterm.paint(Role::Error, "x"), "\x1b[38;5;211mx\x1b[0m");
    assert_eq!(xterm.paint(Role::Warning, "x"), "\x1b[38;5;221mx\x1b[0m");
    assert_eq!(xterm.paint(Role::Dim, "x"), "\x1b[38;5;247mx\x1b[0m");
    assert_eq!(nearest_256((0, 0, 0)), 16);
    assert_eq!(nearest_256((255, 255, 255)), 231);
    assert_eq!(nearest_256((128, 128, 128)), 244);
}

fn detect(choice: ColorChoice, tty: bool, vars: &[(&str, &str)]) -> Look {
    let vars: Vec<(String, String)> = vars
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    Look::detect(choice, tty, &move |k| {
        vars.iter()
            .find(|(name, _)| name == k)
            .map(|(_, v)| v.clone())
    })
}

#[test]
fn colour_is_on_for_a_terminal_and_off_for_a_pipe() {
    let utf8 = [("LANG", "en_US.UTF-8")];
    assert_eq!(detect(ColorChoice::Auto, true, &utf8).depth, Depth::Ansi16);
    assert_eq!(detect(ColorChoice::Auto, false, &utf8).depth, Depth::None);
    assert_eq!(
        detect(ColorChoice::Always, false, &utf8).depth,
        Depth::Ansi16
    );
    assert_eq!(detect(ColorChoice::Never, true, &utf8).depth, Depth::None);
    assert_eq!(
        detect(ColorChoice::Auto, true, &[("NO_COLOR", "1")]).depth,
        Depth::None
    );
    assert_eq!(
        detect(ColorChoice::Auto, true, &[("NO_COLOR", "")]).depth,
        Depth::Ansi16
    );
    assert_eq!(
        detect(ColorChoice::Always, true, &[("NO_COLOR", "1")]).depth,
        Depth::Ansi16
    );
    assert_eq!(
        detect(ColorChoice::Auto, true, &[("TERM", "dumb")]).depth,
        Depth::None
    );
    assert_eq!(Look::stdout(ColorChoice::Never).depth, Depth::None);
}

#[test]
fn colour_depth_from_colorterm_and_term() {
    let depth = |vars: &[(&str, &str)]| detect(ColorChoice::Auto, true, vars).depth;
    assert_eq!(depth(&[("COLORTERM", "truecolor")]), Depth::TrueColor);
    assert_eq!(
        depth(&[("COLORTERM", "24bit"), ("TERM", "xterm-256color")]),
        Depth::TrueColor
    );
    assert_eq!(
        depth(&[("COLORTERM", "yes"), ("TERM", "xterm-256color")]),
        Depth::Ansi256
    );
    assert_eq!(depth(&[("TERM", "xterm")]), Depth::Ansi16);
}

#[test]
fn theme_from_colorfgbg() {
    let theme = |v: &str| detect(ColorChoice::Auto, true, &[("COLORFGBG", v)]).theme;
    assert_eq!(theme("0;15"), Theme::Light);
    assert_eq!(theme("0;default;7"), Theme::Light);
    assert_eq!(theme("15;0"), Theme::Dark);
    assert_eq!(theme("7;8"), Theme::Dark);
    assert_eq!(theme("garbage"), Theme::Dark);
    assert_eq!(detect(ColorChoice::Auto, true, &[]).theme, Theme::Dark);
}

#[test]
fn unicode_needs_a_utf8_locale() {
    let unicode = |vars: &[(&str, &str)]| detect(ColorChoice::Auto, false, vars).unicode;
    assert!(unicode(&[("LANG", "en_US.UTF-8")]));
    assert!(unicode(&[("LC_ALL", "C.utf8"), ("LANG", "C")]));
    assert!(!unicode(&[("LC_ALL", "C"), ("LANG", "en_US.UTF-8")]));
    assert!(unicode(&[("LC_CTYPE", "UTF-8")]));
    assert!(!unicode(&[]));
    assert!(!unicode(&[("TERM", "linux"), ("LANG", "en_US.UTF-8")]));
    assert!(unicode(&[("WT_SESSION", "1")]));
}

#[test]
fn links_on_terminals_that_support_them() {
    let links = |choice, tty, vars: &[(&str, &str)]| detect(choice, tty, vars).links;
    assert!(links(
        ColorChoice::Auto,
        true,
        &[("TERM_PROGRAM", "iTerm.app")]
    ));
    assert!(!links(
        ColorChoice::Auto,
        false,
        &[("TERM_PROGRAM", "iTerm.app")]
    ));
    assert!(!links(
        ColorChoice::Never,
        true,
        &[("TERM_PROGRAM", "iTerm.app")]
    ));
    assert!(links(
        ColorChoice::Always,
        false,
        &[("TERM_PROGRAM", "WezTerm")]
    ));
    assert!(!links(
        ColorChoice::Auto,
        true,
        &[("TERM_PROGRAM", "Apple_Terminal")]
    ));
    assert!(links(ColorChoice::Auto, true, &[("WT_SESSION", "x")]));
    assert!(links(
        ColorChoice::Auto,
        true,
        &[("KONSOLE_VERSION", "230800")]
    ));
    assert!(links(ColorChoice::Auto, true, &[("VTE_VERSION", "7600")]));
    assert!(!links(ColorChoice::Auto, true, &[("VTE_VERSION", "4600")]));
    assert!(links(ColorChoice::Auto, true, &[("TERM", "xterm-kitty")]));
    assert!(links(ColorChoice::Auto, true, &[("FORCE_HYPERLINK", "1")]));
    assert!(!links(
        ColorChoice::Auto,
        true,
        &[("FORCE_HYPERLINK", "0"), ("TERM_PROGRAM", "iTerm.app")]
    ));
}

/// A `Write` the test can read back while the spinner thread holds the other end.
#[derive(Clone, Default)]
struct Shared(Arc<Mutex<Vec<u8>>>);

impl Write for Shared {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Shared {
    fn text(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
    }
}

#[test]
fn spinner_stays_hidden_for_a_short_run() {
    let out = Shared::default();
    let spinner = Spinner::start(Box::new(out.clone()), 10, true, Duration::from_secs(60));
    spinner.tick();
    drop(spinner);
    assert_eq!(out.text(), "");
    assert!(Spinner::stderr(10, &Look::plain(), false).is_none());
    drop(Spinner::stderr(10, &Look::plain(), true).expect("a spinner on a terminal"));
}

#[test]
fn spinner_counts_files_then_clears_its_line() {
    for (unicode, first) in [(true, '⠋'), (false, '|')] {
        let out = Shared::default();
        let spinner = Spinner::start(Box::new(out.clone()), 10, unicode, Duration::ZERO);
        for _ in 0..3 {
            spinner.tick();
        }
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let wait_for = |count: &str| {
            while !out.text().contains(count) && std::time::Instant::now() < deadline {
                std::thread::yield_now();
            }
        };
        wait_for("3/10 files");
        // A later frame shows the new count.
        spinner.tick();
        wait_for("4/10 files");
        drop(spinner);
        let text = out.text();
        assert!(text.starts_with(&format!("\r{first} ")), "{text:?}");
        assert!(text.contains(" 3/10 files"), "{text:?}");
        assert!(text.contains(" 4/10 files"), "{text:?}");
        assert!(text.ends_with("\r\x1b[2K"), "{text:?}");
    }
}

/// format.md §9's `-v` diff, hand-derived from a two-hunk unified diff: the header and `\ No newline` markers are
/// left out, each line carries its old and new number (blank where the line doesn't exist on that side), a blank
/// context line ends at the bar, and `…` separates the hunks.
#[test]
fn diff_numbers_both_sides_and_separates_hunks() {
    let unified = "--- /p/a.php\n+++ /p/a.php\n@@ -1,3 +1,3 @@\n <?php\n-$a = array(1);\n+$a = [1];\n \n@@ -9,2 +9,2 @@\n x\r\n-y\n+z\n\\ No newline at end of file\n";
    assert_eq!(
        plain().diff(unified),
        concat!(
            "    1  1 │  <?php\n",
            "    2    │ -$a = array(1);\n",
            "       2 │ +$a = [1];\n",
            "    3  3 │\n",
            "   …\n",
            "    9  9 │  x\n",
            "   10    │ -y\n",
            "      10 │ +z\n",
        )
    );
    assert_eq!(
        Renderer::new(ascii(), "/p").diff("@@ -1 +1 @@\n-a\n+b\n"),
        "   1   | -a\n     1 | +b\n"
    );
    assert_eq!(
        painted().diff("@@ -1,2 +1,2 @@\n x\n-a\n+b\n"),
        format!(
            "   {D}1 1{Z} {D}│{Z}  {D}x{Z}\n   {D}2  {Z} {D}│{Z} {E}-a{Z}\n   {D}  2{Z} {D}│{Z} {OK}+b{Z}\n"
        )
    );
}

/// format.md §9's summary wording, hand-derived: fixing, `--test` with and without files to format, errors in red,
/// singulars, ASCII.
#[test]
fn format_summary_samples() {
    let fixed = FormatSummary {
        formatted: 2,
        unchanged: 482,
        errors: 0,
        files: 484,
        test: false,
        elapsed: Duration::from_millis(210),
    };
    assert_eq!(
        plain().format_summary(&fixed),
        "Formatted 2 files, 482 unchanged · 484 files in 0.21s\n"
    );
    let test = FormatSummary {
        test: true,
        elapsed: Duration::from_millis(180),
        ..fixed
    };
    assert_eq!(
        plain().format_summary(&test),
        "2 of 484 files need formatting · run `pun format` to fix · 0.18s\n"
    );
    let failing = FormatSummary {
        formatted: 0,
        errors: 1,
        files: 1,
        ..test
    };
    assert_eq!(
        painted().format_summary(&failing),
        format!("{E}1 error{Z} {D}·{Z} {D}1 file in 0.18s{Z}\n")
    );
    let clean = FormatSummary {
        formatted: 0,
        ..test
    };
    assert_eq!(
        Renderer::new(ascii(), "/p").format_summary(&clean),
        "All 484 files formatted - 0.18s\n"
    );
    let one = FormatSummary {
        formatted: 1,
        unchanged: 0,
        errors: 2,
        files: 3,
        ..fixed
    };
    assert_eq!(
        plain().format_summary(&one),
        "Formatted 1 file, 0 unchanged · 2 errors · 3 files in 0.21s\n"
    );
}

#[test]
fn check_summary_and_pills() {
    let at = |depth, theme| Look {
        depth,
        theme,
        unicode: true,
        links: false,
    };
    let dark = at(Depth::TrueColor, Theme::Dark);
    // The summary: bold zap on dark, bold only on light, bold yellow at 16 colours.
    assert_eq!(
        dark.paint(Role::Summary, "x"),
        "\x1b[1;38;2;255;229;61mx\x1b[0m"
    );
    assert_eq!(
        at(Depth::TrueColor, Theme::Light).paint(Role::Summary, "x"),
        "\x1b[1mx\x1b[0m"
    );
    assert_eq!(
        at(Depth::Ansi16, Theme::Dark).paint(Role::Summary, "x"),
        "\x1b[1;33mx\x1b[0m"
    );
    // Pills: the label centred in 9 columns, bold code-bg ink on the step's colour; plain without colour.
    assert_eq!(
        dark.pill(Step::Lint),
        "\x1b[1;38;2;23;18;31;48;2;255;122;184m  lint   \x1b[0m"
    );
    assert_eq!(
        at(Depth::Ansi16, Theme::Dark).pill(Step::Format),
        "\x1b[1;30;103m format  \x1b[0m"
    );
    assert_eq!(Look::plain().pill(Step::Analyse), " analyse ");

    let r = Renderer::new(Look::plain(), "/project");
    let items = [CheckItem {
        path: "app/Models/Post.php".into(),
        at: ":18:16".into(),
        rules: vec!["analyse/undefined/method".into()],
        message: Some("Call to an undefined method App\\Models\\Post::publsh().".into()),
        below: String::new(),
    }];
    let line = |step, errors, fixable, text: Option<&str>, items| StepLine {
        step,
        errors,
        warnings: 0,
        fixable,
        files: errors,
        text: text.map(String::from),
        failed: false,
        items,
    };
    let out = [
        r.check_step(&line(Step::Format, 0, 0, Some("All files formatted"), &[])),
        r.check_step(&line(Step::Analyse, 1, 1, None, &items)),
        r.check_summary(&Summary {
            errors: 2,
            warnings: 0,
            fixable: 1,
            fix_command: "pun check --fix",
            need_formatting: 0,
            files: 2,
            elapsed: Duration::from_millis(310),
        }),
    ]
    .concat();
    assert_eq!(
        out,
        "\
✔  format    All files formatted
✖  analyse   1 error in 1 file · 1 fixable
   app/Models/Post.php:18:16  analyse/undefined/method
   Call to an undefined method App\\Models\\Post::publsh().

✖ 2 errors · 1 fixable (run pun check --fix)
"
    );
}
