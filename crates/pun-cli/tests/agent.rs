//! Agent mode (docs/PLAN.md "Agent-optimised output"): detection priority and the exact lines of every command.
//! Expected rule IDs and summaries come from rules/mapping.toml, the fixers each input triggers from Pint
//! (`php .oracle/pint/builds/pint --format=agent` on the same files), the env list from the agent-detector crate
//! 0.2.1's src/agents.rs plus laravel/agent-detector's AgentDetector::AGENT_ENV_VARS (380e6e4). The run time in the summary line is replaced by `T`.
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

const PUN: &str = env!("CARGO_BIN_EXE_pun");

fn project(files: &[(&str, &str)]) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!(
        "pun-agent-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&d);
    for (f, c) in files {
        std::fs::create_dir_all(d.join(f).parent().unwrap()).unwrap();
        std::fs::write(d.join(f), c).unwrap();
    }
    d.canonicalize().unwrap()
}

/// stdout (time normalised) and exit code; only PATH (and coverage's variable) survive from the environment.
fn run(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (String, Option<i32>) {
    let out = Command::new(PUN)
        .args(args)
        .current_dir(dir)
        .env_clear()
        .envs(std::env::vars().filter(|(k, _)| k == "PATH" || k == "LLVM_PROFILE_FILE"))
        .env("COLUMNS", "80")
        .envs(env.iter().copied())
        .output()
        .unwrap();
    let out_s = String::from_utf8(out.stdout).unwrap();
    let (first, rest) = out_s.split_once('\n').unwrap_or((&out_s, ""));
    let first = match first.strip_suffix("s)") {
        Some(head) if first.starts_with("pun ") => {
            let i = head.rfind(['(', ' ']).unwrap();
            format!("{}Ts)", &head[..=i])
        }
        _ => first.to_string(),
    };
    let sep = if out_s.contains('\n') { "\n" } else { "" };
    (format!("{first}{sep}{rest}"), out.status.code())
}

/// The agent's view: PUN_AGENT=1.
fn agent(dir: &Path, args: &[&str]) -> (String, Option<i32>) {
    run(dir, args, &[("PUN_AGENT", "1")])
}

const TOML: &str = "[format]\npreset = \"empty\"\n\n[format.rules]\narray-syntax = true\n\n[lint.rules]\nno-alias-function = { fix = \"apply\" }\nno-self-assignment = true\n";
const ARRAY: &str = "<?php\n$a = array(1);\n";
const ALIAS: &str = "<?php\n$n = sizeof(array());\n";
const SELF: &str = "<?php\n$a = 1;\n$a = $a;\n";
const CLEAN: &str = "<?php\n$c = [1];\n";

const ALIAS_LINE: &str = "app/B.php lint.no-alias-function [fix:unsafe] Master functions shall be used instead of aliases.";

#[test]
fn detection_priority() {
    let d = project(&[("pun.toml", TOML), ("app/A.php", ARRAY)]);
    let is_agent = |args: &[&str], env: &[(&str, &str)]| {
        let mut a = vec!["format", "--test"];
        a.extend(args);
        run(&d, &a, env).0.starts_with("pun format: fail")
    };
    // The agent-detector crate (0.2.1, env only): each variable of its agents.rs, set to a non-empty value.
    for var in [
        "CURSOR_AGENT",
        "GEMINI_CLI",
        "CODEX_SANDBOX",
        "CODEX_CI",
        "CODEX_THREAD_ID",
        "ANTIGRAVITY_AGENT",
        "AUGMENT_AGENT",
        "OPENCODE_CLIENT",
        "CLAUDECODE",
        "CLAUDE_CODE",
        "CLINE_ACTIVE",
        "GOOSE_TERMINAL",
        "OPENCLAW_SHELL",
        "PI_CODING_AGENT",
        "ROO_ACTIVE",
        "TRAE_AI_SHELL_ID",
        "REPL_ID",
        "COPILOT_MODEL",
        "COPILOT_ALLOW_ALL",
        "COPILOT_GITHUB_TOKEN",
        "ASTRBOT_CLI",
        "AUTOHAND_CLI",
        "BASH_AGENT_HOME",
        "CLAWHUB_WORKDIR",
        "AI_AGENT",
        "AGENT",
        // laravel/agent-detector's AGENT_ENV_VARS entries the crate lacks.
        "GROK_PLUGIN_ROOT",
        "GROK_PLUGIN_DATA",
        "CODEX_SANDBOX_NETWORK_DISABLED",
        "OPENCODE",
        "AMP_CURRENT_THREAD_ID",
        "COPILOT_CLI",
        "ANTIGRAVITY_CLI_ALIAS",
        "MATTERHORN_SESSION_ID",
        "JUNIE_DATA",
        "JUNIE_SHIM_PATH",
        "KIRO_AGENT_PATH",
        "KIMI_PLUGIN_ROOT",
        "ZCODE_ENV",
    ] {
        assert!(is_agent(&[], &[(var, "1")]), "{var}");
        // Empty or blank values don't count.
        assert!(!is_agent(&[], &[(var, " ")]), "{var}");
    }
    // No agent; no TTY (the test's pipes) is not an agent.
    assert!(!is_agent(&[], &[]));
    // Cursor sets CURSOR_TRACE_ID in human terminals too (upstream dropped it for CURSOR_AGENT); it alone is no agent,
    // but Cursor's agent terminals (CURSOR_AGENT) and other agents run inside Cursor still count.
    assert!(!is_agent(&[], &[("CURSOR_TRACE_ID", "1")]));
    assert!(is_agent(
        &[],
        &[("CURSOR_TRACE_ID", "1"), ("CURSOR_AGENT", "1")]
    ));
    assert!(is_agent(
        &[],
        &[("CURSOR_TRACE_ID", "1"), ("CLINE_ACTIVE", "1")]
    ));
    assert!(is_agent(&[], &[("AI_AGENT", "cursor")]));
    assert!(is_agent(
        &[],
        &[("CURSOR_EXTENSION_HOST_ROLE", "agent-exec")]
    ));
    assert!(!is_agent(&[], &[("CURSOR_EXTENSION_HOST_ROLE", "ui")]));
    assert!(!is_agent(&[], &[("AIDER", "1")]));
    // PAO: PAO_DISABLE beats the env list and PAO_FORCE; PAO_FORCE turns it on; falsy values fall through.
    assert!(!is_agent(&[], &[("CLAUDECODE", "1"), ("PAO_DISABLE", "1")]));
    assert!(!is_agent(
        &[],
        &[("PAO_FORCE", "true"), ("PAO_DISABLE", "on")]
    ));
    assert!(is_agent(&[], &[("CLAUDECODE", "1"), ("PAO_DISABLE", "0")]));
    assert!(is_agent(&[], &[("PAO_FORCE", "YES")]));
    assert!(!is_agent(&[], &[("PAO_FORCE", "off")]));
    // PUN_AGENT beats PAO and the env list; empty or junk falls through.
    assert!(!is_agent(&[], &[("PUN_AGENT", "0"), ("PAO_FORCE", "1")]));
    assert!(!is_agent(
        &[],
        &[("PUN_AGENT", "false"), ("CLAUDECODE", "1")]
    ));
    assert!(is_agent(&[], &[("PUN_AGENT", "1"), ("PAO_DISABLE", "1")]));
    assert!(is_agent(&[], &[("PUN_AGENT", ""), ("CLAUDECODE", "1")]));
    assert!(is_agent(
        &[],
        &[("PUN_AGENT", "maybe"), ("CLAUDECODE", "1")]
    ));
    // The flags beat everything.
    assert!(is_agent(&["--agent"], &[("PUN_AGENT", "0")]));
    assert!(!is_agent(&["--no-agent"], &[("PUN_AGENT", "1")]));
    assert!(!is_agent(&["--no-agent"], &[("CLAUDECODE", "1")]));
    assert_eq!(
        run(&d, &["format", "--agent", "--no-agent"], &[]).1,
        Some(2)
    );
    // --no-agent under an agent is the human output byte for byte.
    assert_eq!(
        run(
            &d,
            &["format", "--test", "--no-agent"],
            &[("CLAUDECODE", "1")]
        ),
        run(&d, &["format", "--test"], &[])
    );
}

#[test]
fn format() {
    let d = project(&[
        ("pun.toml", TOML),
        ("app/A.php", ARRAY),
        ("app/C.php", CLEAN),
    ]);
    assert_eq!(
        agent(&d, &["format", "--test"]),
        (
            "pun format: fail, 1 file needs formatting (2 checked, Ts)\napp/A.php format.array-syntax\n".into(),
            Some(1)
        )
    );
    // -v: the bare diff under each file, paths relative.
    assert_eq!(
        agent(&d, &["format", "--test", "-v"]).0,
        "pun format: fail, 1 file needs formatting (2 checked, Ts)\napp/A.php format.array-syntax\n--- app/A.php\n+++ app/A.php\n@@ -1,2 +1,2 @@\n <?php\n-$a = array(1);\n+$a = [1];\n"
    );
    // An explicit --format keeps its reporter; --format=agent is Pint's JSON agent reporter.
    let (json, _) = agent(&d, &["format", "--test", "--format=json"]);
    assert!(json.starts_with(r#"{"about":"PHP CS Fixer"#), "{json}");
    assert_eq!(
        agent(&d, &["format", "--test", "--format=agent"]).0,
        r#"{"tool":"pint","result":"fail","files":[{"path":"app\/A.php","fixers":["array_syntax"]}]}"#
    );
    // -q doesn't silence the summary.
    assert_eq!(
        agent(&d, &["format", "--repair", "-q"]),
        (
            "pun format: fixed, 1 file formatted (2 checked, Ts)\napp/A.php format.array-syntax\n"
                .into(),
            Some(1)
        )
    );
    assert_eq!(
        std::fs::read_to_string(d.join("app/A.php")).unwrap(),
        CLEAN.replace("$c", "$a")
    );
    assert_eq!(
        agent(&d, &["format"]),
        ("pun format: pass (2 checked, Ts)\n".into(), Some(0))
    );
}

#[test]
fn caps_long_lists() {
    let mut files = vec![("pun.toml".to_string(), TOML.to_string())];
    for n in 1..=23 {
        files.push((format!("app/F{n:02}.php"), ARRAY.into()));
    }
    let files: Vec<(&str, &str)> = files
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    let d = project(&files);
    let line = |n: usize| format!("app/F{n:02}.php format.array-syntax\n");
    let head = "pun format: fail, 23 files need formatting (23 checked, Ts)\n";
    let first: String = (1..=20).map(line).collect();
    assert_eq!(
        agent(&d, &["format", "--test"]).0,
        format!("{head}{first}+3 more; run with -v for all\n")
    );
    let all: String = (1..=23)
        .map(|n| {
            format!(
                "{}--- app/F{n:02}.php\n+++ app/F{n:02}.php\n@@ -1,2 +1,2 @@\n <?php\n-$a = array(1);\n+$a = [1];\n",
                line(n)
            )
        })
        .collect();
    assert_eq!(
        agent(&d, &["format", "--test", "-v"]).0,
        format!("{head}{all}")
    );
}

#[test]
fn lint_and_check() {
    let d = project(&[
        ("pun.toml", TOML),
        ("app/A.php", ARRAY),
        ("app/B.php", ALIAS),
        ("app/S.php", SELF),
    ]);
    // A rule without a summary in the mapping prints none.
    let lint = format!("{ALIAS_LINE}\napp/S.php lint.no-self-assignment [fix:safe]\n");
    assert_eq!(
        agent(&d, &["lint"]),
        (
            format!("pun lint: fail, 2 files with fixes (3 checked, Ts)\n{lint}"),
            Some(1)
        )
    );
    // One file with format and lint rules: the format IDs on one line, each lint rule on its own.
    assert_eq!(
        agent(&d, &["check"]),
        (
            format!(
                "pun check: fail, 3 files with fixes (3 checked, Ts)\napp/A.php format.array-syntax\napp/B.php format.array-syntax\n{lint}"
            ),
            Some(1)
        )
    );
    // --fix applies only `fix = "apply"` rules; the others are reported as remaining, like `--test` reports them,
    // and fail the run.
    let remaining = "app/S.php lint.no-self-assignment [fix:safe] [remaining]\n";
    assert_eq!(
        agent(&d, &["lint", "--fix"]),
        (
            format!(
                "pun lint: fail, 1 file fixed, 1 file with remaining fixes (3 checked, Ts)\n{ALIAS_LINE}\n{remaining}"
            ),
            Some(1)
        )
    );
    assert_eq!(
        std::fs::read_to_string(d.join("app/B.php")).unwrap(),
        "<?php\n$n = count(array());\n"
    );
    assert_eq!(
        read(&d, "app/S.php"),
        SELF,
        "remaining fixes are not written"
    );
    // Again (the cache must not hide them): nothing left to fix, the same rule remains.
    assert_eq!(
        agent(&d, &["lint", "--fix"]),
        (
            format!("pun lint: fail, 1 file with remaining fixes (3 checked, Ts)\n{remaining}"),
            Some(1)
        )
    );
    // `pun check --fix` too, after the format rules ran.
    assert_eq!(
        agent(&d, &["check", "--fix"]),
        (
            format!(
                "pun check: fail, 2 files fixed, 1 file with remaining fixes (3 checked, Ts)\napp/A.php format.array-syntax\napp/B.php format.array-syntax\n{remaining}"
            ),
            Some(1)
        )
    );
    // Humans get Pint's summary: FAIL, the fixed files (✓), then the remaining ones (⨯), as `--test` lists them.
    let (out, code) = run(&d, &["lint", "--fix"], &[]);
    let lines: Vec<&str> = out.lines().map(str::trim).collect();
    assert_eq!(code, Some(1), "{out}");
    assert!(
        lines
            .iter()
            .any(|l| l.starts_with("FAIL") && l.ends_with(" 3 files, 1 style issue remaining")),
        "{out}"
    );
    assert!(
        lines
            .iter()
            .any(|l| l.starts_with("⨯ app/S.php ") && l.ends_with(" Pun/no_self_assignment")),
        "{out}"
    );
    // Without remaining rules --fix passes as before.
    std::fs::write(d.join("app/S.php"), CLEAN).unwrap();
    assert_eq!(
        agent(&d, &["lint", "--fix"]),
        ("pun lint: pass (3 checked, Ts)\n".to_string(), Some(0))
    );
}

fn read(d: &Path, f: &str) -> String {
    std::fs::read_to_string(d.join(f)).unwrap()
}

#[test]
fn errors() {
    let d = project(&[
        ("pun.toml", TOML),
        ("app/A.php", ARRAY),
        ("app/D.php", "<?php\n$a = (;\n"),
    ]);
    assert_eq!(
        agent(&d, &["format", "--test"]),
        (
            "pun format: fail, 1 error, 1 file needs formatting (2 checked, Ts)\napp/A.php format.array-syntax\napp/D.php:2 syntax-error Parse error: syntax error, unexpected token \";\" on line 2.\n".into(),
            Some(1)
        )
    );
    // Errors that abort the run (Pint's boxes) are one line, with no progress before them.
    std::fs::write(d.join("app/D.php"), CLEAN).unwrap();
    let locked = d.join("app/A.php");
    std::fs::set_permissions(&locked, std::os::unix::fs::PermissionsExt::from_mode(0o000)).unwrap();
    let got = agent(&d, &["format"]);
    std::fs::set_permissions(&locked, std::os::unix::fs::PermissionsExt::from_mode(0o644)).unwrap();
    assert_eq!(
        got,
        (
            format!(
                "pun format: error: Failed to read content from \"{}\".\n",
                locked.display()
            ),
            Some(1)
        )
    );
    assert_eq!(
        agent(&d, &["format", "app/nope.php"]),
        (
            "pun format: error: The path \"app/nope.php\" is not readable.\n".into(),
            Some(1)
        )
    );
    // Config errors too, for every command.
    let empty = project(&[("app/A.php", ARRAY)]);
    for cmd in ["format", "lint", "config"] {
        assert_eq!(
            agent(&empty, &[cmd]),
            (
                format!("pun {cmd}: error: No pun.toml found. Run `pun migrate`.\n"),
                Some(1)
            )
        );
    }
}

/// --dirty and --diff list only the files git selects.
#[test]
fn git_selection() {
    let d = project(&[
        ("pun.toml", TOML),
        ("app/A.php", CLEAN),
        ("app/C.php", CLEAN),
    ]);
    let git = |args: &[&str]| {
        let ok = Command::new("git")
            .args([
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "-c",
                "init.defaultBranch=main",
            ])
            .args(args)
            .current_dir(&d)
            .output()
            .unwrap()
            .status
            .success();
        assert!(ok, "{args:?}");
    };
    git(&["init", "-q"]);
    git(&["add", "."]);
    git(&["commit", "-qm", "init"]);
    git(&["checkout", "-qb", "topic"]);
    std::fs::write(d.join("app/A.php"), ARRAY).unwrap();
    let want = "pun format: fail, 1 file needs formatting (1 checked, Ts)\napp/A.php format.array-syntax\n";
    assert_eq!(agent(&d, &["format", "--dirty", "--test"]).0, want);
    git(&["commit", "-qam", "array"]);
    assert_eq!(
        agent(&d, &["format", "--dirty", "--test"]).0,
        "pun format: pass (0 checked, Ts)\n"
    );
    assert_eq!(agent(&d, &["format", "--diff=main", "--test"]).0, want);
}

#[test]
fn config() {
    let d = project(&[
        ("pun.toml", TOML),
        ("composer.json", r#"{"require":{"php":"^8.5"}}"#),
    ]);
    let (human, _) = run(&d, &["config"], &[]);
    // The summary line, then the same TOML a human gets.
    assert_eq!(
        agent(&d, &["config"]),
        (
            format!(
                "pun config: ok, preset empty, PHP 8.4, 1 format rule, 2 lint rules (from composer.json, pun.toml, Ts)\n{human}"
            ),
            Some(0)
        )
    );
    assert_eq!(
        agent(&d, &["config", "--format=json"]),
        run(&d, &["config", "--format=json"], &[])
    );
}

#[test]
fn migrate() {
    let d = project(&[
        (
            "pint.json",
            r#"{"preset":"empty","rules":{"array_syntax":true,"no_alias_functions":true}}"#,
        ),
        ("app/A.php", "<?php\n// @php-cs-fixer-ignore array_syntax\n"),
        ("app/U.php", "<?php\n// @php-cs-fixer-ignore Acme/rule\n"),
    ]);
    let summary = "pun.toml (preset empty, 1 format rule, 1 lint rule), ignore comments";
    // --dry-run: the summary first, then the diffs.
    let (out, code) = agent(&d, &["migrate", "--dry-run"]);
    assert_eq!(code, Some(0));
    assert!(
        out.starts_with(&format!(
            "pun migrate: dry-run, {} to rewrite in 1 file (from pint.json, Ts)\npint.json migrate\napp/U.php ignore-not-migrated `@php-cs-fixer-ignore Acme/rule` left as is (no Pun rule for it)\n--- pun.toml\n+++ pun.toml\n",
            summary.replacen("pun.toml", "pun.toml to write", 1)
        )),
        "{out}"
    );
    assert!(!d.join("pun.toml").exists());
    assert_eq!(
        agent(&d, &["migrate"]),
        (
            format!(
                "pun migrate: ok, {} rewritten in 1 file (from pint.json, Ts)\npint.json migrated\napp/U.php ignore-not-migrated `@php-cs-fixer-ignore Acme/rule` left as is (no Pun rule for it)\n",
                summary.replacen("pun.toml", "pun.toml written", 1)
            ),
            Some(0)
        )
    );
    assert_eq!(
        agent(&d, &["migrate"]).0,
        format!(
            "pun migrate: ok, {} rewritten in 0 files (from pint.json, Ts)\npint.json migrated\napp/U.php ignore-not-migrated `@php-cs-fixer-ignore Acme/rule` left as is (no Pun rule for it)\n",
            summary.replacen("pun.toml", "pun.toml up to date", 1)
        )
    );
    // A pun.toml that differs: the error as one line.
    std::fs::write(d.join("pint.json"), r#"{"preset":"psr12"}"#).unwrap();
    assert_eq!(
        agent(&d, &["migrate"]),
        (
            "pun migrate: error: pun.toml exists and differs from what migrate writes; move it away first (`pun migrate --dry-run` shows the difference)\n".into(),
            Some(1)
        )
    );
    // Unsupported rules: each one a line, nothing written.
    std::fs::remove_file(d.join("pun.toml")).unwrap();
    std::fs::write(
        d.join("pint.json"),
        r#"{"preset":"empty","rules":{"Acme/x":true}}"#,
    )
    .unwrap();
    assert_eq!(
        agent(&d, &["migrate"]),
        (
            "pun migrate: fail, 1 unsupported rule, nothing written (from pint.json, Ts)\npint.json unsupported-rule Acme/x: not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set)\n".into(),
            Some(1)
        )
    );
    assert!(!d.join("pun.toml").exists());
}

/// No config at all: pun.toml's comment, the human lines and the agent summary say so (a starter, not "from pint.json").
#[test]
fn migrate_without_a_config() {
    let d = project(&[("app/A.php", CLEAN)]);
    let nothing = "no pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found";
    assert_eq!(
        agent(&d, &["migrate"]),
        (
            format!(
                "pun migrate: ok, starter pun.toml written (preset laravel, 126 format rules, 4 lint rules), ignore comments rewritten in 0 files ({nothing}, Ts)\n"
            ),
            Some(0)
        )
    );
    let toml = std::fs::read_to_string(d.join("pun.toml")).unwrap();
    assert!(
        toml.starts_with(&format!(
            "# Written by `pun migrate`: {nothing}; starter config.\npreset = \"recommended\"\n\n[format]\n"
        )),
        "{toml}"
    );
    assert_eq!(
        run(&d, &["migrate"], &[]),
        (
            format!(
                "pun migrate: {nothing}.\npun migrate: starter pun.toml already up to date, ignore comments rewritten in 0 file(s).\n"
            ),
            Some(0)
        )
    );
    std::fs::remove_file(d.join("pun.toml")).unwrap();
    assert_eq!(
        run(&d, &["migrate"], &[]).0,
        format!(
            "pun migrate: {nothing}.\npun migrate: starter pun.toml written, ignore comments rewritten in 0 file(s).\n"
        )
    );
}

/// The prettier pre-flight (`--blade`) under an agent: its error line, without Pint's yellow warning.
#[test]
fn prettier_preflight() {
    if Command::new("node").arg("-v").output().is_err() {
        return;
    }
    let d = project(&[("pun.toml", TOML), ("app/A.php", CLEAN)]);
    assert_eq!(
        agent(&d, &["format", "--blade"]),
        (
            "pun format: error: The rules enabled in your pint configuration require the following prettier dependencies to be installed: prettier, prettier-plugin-blade, prettier-plugin-tailwindcss.\n".into(),
            Some(1)
        )
    );
}
