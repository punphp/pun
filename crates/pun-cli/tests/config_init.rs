use std::process::Command;

fn pun(dir: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_pun"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

#[test]
fn init_writes_pun_toml_once() {
    let d = std::env::temp_dir().join(format!("pun-cli-init-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    std::fs::write(
        d.join("pint.json"),
        r#"{"preset":"psr12","exclude":["build"],"rules":{"final_class":true}}"#,
    )
    .unwrap();

    let first = pun(&d, &["config", "--init"]);
    assert!(first.status.success(), "{first:?}");
    let toml = std::fs::read_to_string(d.join("pun.toml")).unwrap();
    assert_eq!(
        toml,
        "[format]\npreset = \"psr12\"\nexclude = [\"build\"]\n\n[format.rules]\nfinal_class = true\n"
    );

    // Never overwrites an existing pun.toml.
    std::fs::write(d.join("pun.toml"), "# mine\n").unwrap();
    let second = pun(&d, &["config", "--init"]);
    assert!(!second.status.success());
    assert!(String::from_utf8_lossy(&second.stderr).contains("pun.toml already exists."));
    assert_eq!(
        std::fs::read_to_string(d.join("pun.toml")).unwrap(),
        "# mine\n"
    );
    std::fs::remove_dir_all(&d).unwrap();
}
