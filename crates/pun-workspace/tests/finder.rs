//! Discovery parity with the real Pint (`php .oracle/pint/builds/pint`): every scenario builds a tree whose PHP
//! files all start with `<?PHP` (pint.json enables only `full_opening_tag`), so `pint --test --format=txt` lists
//! every discovered regular file ("N) path") and counts every discovered file, symlinks included ("Found x of N").
//! Our side must produce the same set and count. Skipped when the oracle is not installed.
extern crate punphp_workspace as pun_workspace;
use pun_workspace::{GitSelect, discover, git_paths, is_path_excluded};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

const BAD: &str = "<?PHP\necho 1;\n";

fn oracle() -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.oracle/pint/builds/pint");
    p.exists().then_some(p)
}

/// A temp project with `files` (each `BAD`) and `pint.json` = `pint_json` merged over the base rules.
fn tree(files: &[&str], pint_json: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let root = std::env::temp_dir().join(format!(
        "pun-finder-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&root);
    for f in files {
        let p = root.join(f);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, BAD).unwrap();
    }
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("pint.json"), pint_json).unwrap();
    root.canonicalize().unwrap()
}

fn json(extra: &str) -> String {
    format!(r#"{{"preset":"empty","rules":{{"full_opening_tag":true}}{extra}}}"#)
}

fn cmd(program: &Path, dir: &Path) -> Command {
    let mut c = Command::new(program);
    // No coding-agent variables: Pint would switch to its agent reporter.
    c.current_dir(dir)
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap())
        .env("HOME", std::env::var("HOME").unwrap())
        .env("COLUMNS", "80");
    c
}

type Listing = Result<(BTreeSet<String>, usize), String>;

fn pint(dir: &Path, args: &[&str]) -> Listing {
    let out = cmd(Path::new("php"), dir)
        .arg(oracle().unwrap())
        .args(["--test", "--format=txt"])
        .args(args)
        .output()
        .unwrap();
    let text =
        String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
    let Some(found) = text.lines().find(|l| l.starts_with("Found ")) else {
        return Err(text);
    };
    let total = found
        .split(" of ")
        .nth(1)
        .unwrap()
        .split(' ')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let files = text
        .lines()
        .filter_map(|l| {
            l.trim_start()
                .split_once(") ")
                .filter(|(n, _)| n.parse::<u32>().is_ok())
                .map(|(_, f)| f.to_string())
        })
        .collect();
    Ok((files, total))
}

fn listing(root: &Path, files: anyhow::Result<Vec<PathBuf>>) -> Listing {
    let files = files.map_err(|e| e.to_string())?;
    let total = files.len();
    let regular = files
        .iter()
        .filter(|p| !p.is_symlink())
        .map(|p| {
            p.strip_prefix(root)
                .unwrap_or(p)
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    Ok((regular, total))
}

/// What Pun runs for `root`'s pint.json (or `config`): the pun.toml `pun migrate` writes from it.
fn load(root: &Path, config: Option<&str>) -> Result<pun_config::Config, String> {
    let pint = root.join(config.unwrap_or("pint.json"));
    pun_config::migrate::from_pint(&pint, None, &|_| true)
        .and_then(|m| pun_config::from_toml(root, &m.toml))
        .map_err(|e| e.to_string())
}

fn ours(root: &Path, config: Option<&str>, paths: &[&str]) -> Listing {
    let cfg = load(root, config)?;
    let paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    listing(root, discover(&cfg, &paths))
}

/// Same listing, or (for errors) our message appears in Pint's output.
fn check(root: &Path, config: Option<&str>, paths: &[&str]) {
    let mut args: Vec<String> = paths.iter().map(|p| p.to_string()).collect();
    if let Some(c) = config {
        args.push(format!("--config={c}"));
    }
    let want = pint(root, &args.iter().map(String::as_str).collect::<Vec<_>>());
    let got = ours(root, config, paths);
    match (&want, &got) {
        (Err(w), Err(g)) => assert!(
            w.contains(g.as_str()),
            "{root:?} {paths:?}: pint said\n{w}\nwe said\n{g}"
        ),
        _ => assert_eq!(got, want, "{root:?} {paths:?} {config:?}"),
    }
}

const COMMON: &[&str] = &[
    "app/A.php",
    "app/v.blade.php",
    "app/build/B.php",
    "app/.hidden/C.php",
    "app/.D.php",
    "vendor/x/E.php",
    "app/vendor/E2.php",
    "x/bootstrap/cache/F.php",
    "bootstrap/cache/F2.php",
    "bootstrap/app.php",
    "storage/G.php",
    "app/storage/G2.php",
    "app/_ide_helper.php",
    "_ide_helper_models.php",
    "a/_ide_helper_actions.php",
    ".phpstorm.meta.php",
    "app/skip/H.php",
    "app/I.txt",
    "app/notme.php",
    "app/U.PHP",
    "app/with space.php",
    "app/.git/K.php",
    "app/CVS/K.php",
    "node_modules/n/N.php",
    "src/Sub/Deep/T.php",
    "src/Sub/ZTest.php",
    "src/a.php",
    "src/b.php",
    "src/ax.php",
    "src/README",
];

fn with_links(root: &Path) {
    std::os::unix::fs::symlink(root.join("app"), root.join("linkdir")).unwrap();
    std::os::unix::fs::symlink(root.join("app/A.php"), root.join("app/L.php")).unwrap();
    std::os::unix::fs::symlink(root.join("missing.php"), root.join("app/dangling.php")).unwrap();
}

#[test]
fn finder_matches_pint() {
    if oracle().is_none() {
        return;
    }
    // Pint's defaults; .gitignore is not honoured; symlinked dirs are not followed, symlinked files count.
    let root = tree(COMMON, &json(""));
    with_links(&root);
    std::fs::write(root.join(".gitignore"), "app/A.php\n").unwrap();
    check(&root, None, &[]);
    // Explicit arguments: a file bypasses every filter, a dir gets the Pint filters, duplicates collapse.
    check(
        &root,
        None,
        &["app", "vendor/x/E.php", "app/A.php", "storage/G.php"],
    );
    check(&root, None, &["nope"]);
    check(&root, None, &["."]);

    // pint.json finder keys: exclude (name / path), notName (glob, brace, regex), notPath (plain, regex).
    let root = tree(
        COMMON,
        &json(
            r##","exclude":["skip","src/Sub/Deep/"],"notName":["*Test.php","{a,b}.php","/^ax\\.php$/i","notme.php"],"notPath":["#^app/with#","build"]"##,
        ),
    );
    check(&root, None, &[]);
    check(&root, None, &["app/build/B.php"]);
    let root = tree(
        COMMON,
        &json(r#","notName":"[ab].php","notPath":"/^SRC/i","exclude":"app""#),
    );
    check(&root, None, &[]);

    // Glob features (Symfony Glob::toRegex) and PHP regexes with every delimiter pair and modifier.
    let files = [
        "g/ax.php",
        "g/bx.php",
        "g/cx.php",
        "g/x.php",
        "g/xx.php",
        "g/a{b.php",
        "g/a*.php",
        "g/a\\b.php",
        "g/.x.php",
        "g/T.php",
        "g/ZTest.php",
        "g/a.php",
        "g/ä.php",
        "g/Mm.php",
        "g/ab",
        "g/p.php",
        "g/q.php",
        "g/r.php",
        "g/s.php",
    ];
    let root = tree(
        &files,
        &json(
            r#","notName":["?x.php","**/xx.php","a\\{b.php","a\\*.php","a\\\\b.php","[b]x.php","(^cx\\.php$)","<^ZTest\\.php$>","{^t\\.php$}i","[^Mm\\.php$]","/^a . php$/x","/^ä/u","/^p.php$/s","/^q.php$/m","/^r.php$/U","~s.php~","ab","g/**"]"#,
        ),
    );
    check(&root, None, &[]);

    // `in`: replaces the cwd default; with path arguments the configured finder is dropped entirely.
    let root = tree(COMMON, &json(r#","in":["src","app"],"exclude":["Deep"]"#));
    check(&root, None, &[]);
    check(&root, None, &["storage"]);
    let root = tree(COMMON, &json(r#","in":["nodir"]"#));
    check(&root, None, &[]);

    // Blade discovery needs the rule (and prettier, which Pint checks for up front).
    let modules = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/pinkary/node_modules");
    if modules.exists() {
        let files: Vec<&str> = COMMON
            .iter()
            .copied()
            .filter(|f| !f.starts_with("node_modules/"))
            .collect();
        let root = tree(
            &files,
            &json("").replace(
                r#""full_opening_tag":true"#,
                r#""full_opening_tag":true,"Pint/laravel_blade":true"#,
            ),
        );
        std::os::unix::fs::symlink(&modules, root.join("node_modules")).unwrap();
        check(&root, None, &[]);
    }

    // Non-ASCII globs and names (UTF-8 bytes on both sides).
    let root = tree(
        &["app/Café.php", "app/Cafe.php", "app/né/X.php", "app/ü.php"],
        &json(r#","notName":["*é.php","?.php"],"notPath":["né"]"#),
    );
    check(&root, None, &[]);

    // --config=<other file>.
    let root = tree(COMMON, &json(r#","notName":["*.php"]"#));
    std::fs::write(root.join("other.json"), json(r#","exclude":["app"]"#)).unwrap();
    check(&root, Some("other.json"), &[]);
}

/// `--stdin-filename`: Pint echoes the input unchanged for excluded paths and fixes it otherwise.
#[test]
fn stdin_exclusion_matches_pint() {
    if oracle().is_none() {
        return;
    }
    let root = tree(
        &[],
        &json(
            r#","exclude":["legacy","app/Old"],"notName":["*Stub.php","{x,y}.php"],"notPath":["gen/*","s/?.php","t/[!x]*.php","u/[a-c].php","v/\\*.php","w/[\\].php"]"#,
        ),
    );
    let cfg = load(&root, None).unwrap();
    let mut blade_cfg = cfg.clone();
    blade_cfg.blade = true;
    for name in [
        "app/A.php",
        "storage/x.php",
        "storage",
        "app/storage/x.php",
        "bootstrap/cache/c.php",
        "build/b.php",
        "node_modules/n.php",
        "legacy/l.php",
        "app/legacy/l.php",
        "app/Old/o.php",
        "app/Older/o.php",
        "FooStub.php",
        "src/FooStub.php",
        "x.php",
        "z.php",
        "gen/a/b.php",
        "src/gen/a.php",
        "v.blade.php",
        "_ide_helper.php",
        "app/.phpstorm.meta.php",
        "[a].php",
        "s/a.php",
        "s/ab.php",
        "t/a.php",
        "t/x.php",
        "u/b.php",
        "u/d.php",
        "v/*.php",
        "v/a.php",
        "w/\\.php",
        "w/a.php",
    ] {
        let out = cmd(Path::new("php"), &root)
            .arg(oracle().unwrap())
            .arg(format!("--stdin-filename={name}"))
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut c| {
                use std::io::Write;
                c.stdin.take().unwrap().write_all(BAD.as_bytes())?;
                c.wait_with_output()
            })
            .unwrap();
        let excluded = out.stdout == BAD.as_bytes();
        assert_eq!(
            is_path_excluded(&cfg, Path::new(name)),
            excluded,
            "relative {name}"
        );
        assert_eq!(
            is_path_excluded(&cfg, &root.join(name)),
            excluded,
            "absolute {name}"
        );
        if name.ends_with(".blade.php") {
            assert!(
                !is_path_excluded(&blade_cfg, Path::new(name)),
                "--blade keeps {name}"
            );
        }
    }
}

fn git(dir: &Path, args: &[&str]) {
    let ok = cmd(Path::new("git"), dir)
        .args([
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "-c",
            "init.defaultBranch=main",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .unwrap()
        .status
        .success();
    assert!(ok, "git {args:?}");
}

/// `--dirty` / `--diff`: git's list intersected with the finder. Pint aborts with "No dirty files found." (an empty run
/// for us) and refuses outside git; `--staged` is Pun's own (git diff --cached).
#[test]
fn git_selection_matches_pint() {
    if oracle().is_none() {
        return;
    }
    let root = tree(
        &[
            "app/Clean.php",
            "app/Mod.php",
            "app/Del.php",
            "app/Ren.php",
            "app/Staged.php",
            "vendor/V.php",
            "storage/S.php",
        ],
        &json(""),
    );
    let ours = |sel: &GitSelect| listing(&root, git_paths(&load(&root, None).unwrap(), sel));
    // Not a repository.
    assert!(pint(&root, &["--dirty"]).is_err() || pint(&root, &["--dirty"]).unwrap().1 == 0);
    // Pint swallows the "only available when using Git" abort: nothing selected.
    for sel in [
        GitSelect::Dirty,
        GitSelect::Diff("main".into()),
        GitSelect::Staged,
        GitSelect::Since("main".into()),
    ] {
        assert_eq!(ours(&sel).unwrap(), (BTreeSet::new(), 0));
    }

    git(&root, &["init", "-q"]);
    git(&root, &["add", "."]);
    git(&root, &["commit", "-qm", "init"]);
    // Clean tree: Pint aborts "No dirty files found." with exit 0 and no listing.
    assert_eq!(ours(&GitSelect::Dirty).unwrap(), (BTreeSet::new(), 0));
    // Only an excluded file is dirty: nothing is selected (not a full discovery).
    std::fs::write(root.join("vendor/V.php"), "<?PHP\necho 0;\n").unwrap();
    assert_eq!(ours(&GitSelect::Dirty), pint(&root, &["--dirty"]));
    git(&root, &["checkout", "-q", "vendor/V.php"]);

    git(&root, &["checkout", "-qb", "feature"]);
    std::fs::write(root.join("app/Mod.php"), "<?PHP\necho 2;\n").unwrap();
    std::fs::remove_file(root.join("app/Del.php")).unwrap();
    git(&root, &["mv", "app/Ren.php", "app/Renamed.php"]);
    std::fs::write(root.join("app/Staged.php"), "<?PHP\necho 3;\n").unwrap();
    git(&root, &["add", "app/Staged.php"]);
    std::fs::write(root.join("app/Staged.php"), "<?PHP\necho 4;\n").unwrap();
    std::fs::write(root.join("app/New.php"), BAD).unwrap();
    std::fs::write(root.join("vendor/V.php"), "<?PHP\necho 5;\n").unwrap();
    std::fs::write(root.join("app/notes.txt"), "x").unwrap();
    assert_eq!(ours(&GitSelect::Dirty), pint(&root, &["--dirty"]));
    git(&root, &["add", "-A"]);
    git(&root, &["commit", "-qm", "feature"]);
    std::fs::write(root.join("app/Clean.php"), "<?PHP\necho 6;\n").unwrap();
    std::fs::write(root.join("app/Untracked.php"), BAD).unwrap();
    assert_eq!(
        ours(&GitSelect::Diff("main".into())),
        pint(&root, &["--diff=main"])
    );
    assert_eq!(
        ours(&GitSelect::Diff("nope".into())),
        pint(&root, &["--diff=nope"])
    );
    git(&root, &["add", "app/Clean.php"]);
    let staged = ours(&GitSelect::Staged).unwrap();
    assert_eq!(staged, (["app/Clean.php".to_string()].into(), 1));
    // --since=main: everything feature changed (the commit, the staged Clean.php) plus untracked; Del.php is gone.
    let since: BTreeSet<String> = [
        "app/Clean.php",
        "app/Mod.php",
        "app/New.php",
        "app/Renamed.php",
        "app/Staged.php",
        "app/Untracked.php",
    ]
    .map(String::from)
    .into();
    assert_eq!(ours(&GitSelect::Since("main".into())).unwrap(), (since, 6));
}
