//! The Composer package's shim (`composer/bin/pun`) runs the release binary of the workspace version
//! with the caller's arguments, stdin and exit code. The download itself needs a published release, so the binary is
//! put where the shim keeps it (next to the shim when Composer's bin dir is unknown) before it runs. Skipped without
//! `php`.
mod common;

use common::{PUN, scratch};

use std::path::Path;
use std::process::{Command, Output};

fn run(cmd: &mut Command) -> Output {
    cmd.env("PUN_AGENT", "0").output().unwrap()
}

#[test]
fn shim_runs_the_workspace_version_with_its_arguments() {
    if Command::new("php").arg("-v").output().is_err() {
        return;
    }
    let dir = scratch("composer", "shim");
    let shim = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../composer/bin/pun");
    std::fs::copy(&shim, dir.join("pun")).unwrap();
    // a shim whose fallback version drifted from the workspace's would download another release instead
    let os = match std::env::consts::OS {
        "macos" => "apple-darwin",
        _ => "unknown-linux-musl",
    };
    let binary = dir.join(format!(
        "pun-{}-{}-{os}",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::ARCH
    ));
    std::fs::copy(PUN, &binary).unwrap();
    std::fs::write(dir.join("Bad.php"), "<?php\n$a = array( 1 );\n").unwrap();

    for args in [
        &["--version"][..],
        &["format", "--test", "Bad.php"],
        &["no-such-command"],
    ] {
        let via_shim = run(Command::new("php")
            .arg(dir.join("pun"))
            .args(args)
            .current_dir(&dir));
        let direct = run(Command::new(&binary).args(args).current_dir(&dir));
        assert_eq!(
            via_shim.status.code(),
            direct.status.code(),
            "exit code of {args:?}"
        );
        assert_eq!(via_shim.stdout, direct.stdout, "stdout of {args:?}");
    }

    // A binary that cannot start (another platform's, or a broken download) is named, not left to a PHP warning.
    std::fs::write(&binary, "not a binary").unwrap();
    let broken = run(Command::new("php").arg(dir.join("pun")).current_dir(&dir));
    assert_eq!(broken.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&broken.stderr),
        format!(
            "pun: could not start {}; delete it to download it again\n",
            binary.display()
        )
    );
}
