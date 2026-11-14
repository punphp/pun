//! `pun install` on a project whose two dependencies come from a path repository, against what Composer 2.11.2 wrote
//! for it: `composer update --no-scripts` in a copy of `fixtures/project` gave its composer.lock and every file in
//! `fixtures/project-expected` (vendor/autoload.php, vendor/composer/*, vendor/bin/*). `fixtures/edge` and
//! `edge-expected` are the same for the autoloader's and bin installer's corner cases, from
//! `composer update --no-scripts --ignore-platform-req=php --ignore-platform-req=ext-sodium-x`.

use pun_packages::fetch::{Auth, Fetcher};
use pun_packages::install::{Options, Report, install};
use pun_packages::lock::Lock;
use serde_json::{Map, Value};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        let kind = entry.file_type().unwrap();
        if kind.is_symlink() {
            std::os::unix::fs::symlink(std::fs::read_link(entry.path()).unwrap(), &target).unwrap();
        } else if kind.is_dir() {
            copy(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).unwrap();
        }
    }
}

fn try_run(dir: &Path, lock: &Lock, o: &Options) -> anyhow::Result<Report> {
    let manifest: Map<String, Value> =
        serde_json::from_str(&std::fs::read_to_string(dir.join("composer.json")).unwrap()).unwrap();
    let fetcher = Fetcher::new(Some(dir.join("cache")), Auth::default(), true);
    let mut events = vec![];
    let report = install(dir, &manifest, lock, &fetcher, o, &mut |e: &str| {
        events.push(e.to_string());
        Ok(())
    })?;
    let expected: &[&str] = if o.no_autoloader {
        &[]
    } else {
        &["pre-autoload-dump", "post-autoload-dump"]
    };
    assert_eq!(events, expected);
    Ok(report)
}

fn run(dir: &Path, lock: &Lock, o: &Options) -> Report {
    try_run(dir, lock, o).unwrap()
}

/// Every file in `fixtures/<expected>` matches the one pun wrote under `dir/vendor`, and nothing else is in `vendor/bin`.
fn same_as_composer(dir: &Path, expected: &str) {
    let expected = Path::new(FIXTURES).join(expected);
    let mut files = vec![("autoload.php".to_string(), expected.join("autoload.php"))];
    for sub in ["composer", "bin"] {
        for entry in std::fs::read_dir(expected.join(sub)).unwrap() {
            let entry = entry.unwrap();
            files.push((
                format!("{sub}/{}", entry.file_name().to_string_lossy()),
                entry.path(),
            ));
        }
    }
    // Without --apcu-autoloader-prefix each dump picks a random one: 20 hex digits.
    let apcu = |text: String| {
        let Some(at) = text.find("setApcuPrefix('") else {
            return text;
        };
        let prefix = &text[at + 15..at + 35];
        assert!(prefix.bytes().all(|b| b.is_ascii_hexdigit()) && text[at + 35..].starts_with("')"));
        text.replace(prefix, "<random>")
    };
    for (name, want) in &files {
        let got = std::fs::read_to_string(dir.join("vendor").join(name))
            .unwrap_or_else(|_| panic!("vendor/{name} missing"));
        assert_eq!(
            apcu(got),
            apcu(std::fs::read_to_string(want).unwrap()),
            "vendor/{name}"
        );
    }
    let bins = std::fs::read_dir(dir.join("vendor/bin")).unwrap().count();
    assert_eq!(
        bins,
        std::fs::read_dir(expected.join("bin")).unwrap().count()
    );
}

#[test]
fn install_matches_composer() {
    let tmp = std::env::temp_dir().join(format!("pun-install-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    copy(&Path::new(FIXTURES).join("project"), &tmp);
    let lock =
        Lock::from_composer_lock(&std::fs::read_to_string(tmp.join("composer.lock")).unwrap())
            .unwrap();

    let report = run(&tmp, &lock, &Options::default());
    assert!(!report.stale_lock);
    assert_eq!(
        report.installed,
        [
            ("acme/lib".to_string(), "1.2.0".to_string()),
            ("acme/tools".to_string(), "0.3.1".to_string())
        ]
    );
    same_as_composer(&tmp, "project-expected");
    assert_eq!(
        std::fs::read_link(tmp.join("vendor/acme/lib")).unwrap(),
        Path::new("../../packages/lib/")
    );
    for bin in [
        "vendor/bin/lib",
        "vendor/bin/tool.sh",
        "packages/lib/bin/lib",
    ] {
        assert_eq!(
            std::fs::metadata(tmp.join(bin))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o755,
            "{bin}"
        );
    }

    // Again: nothing to fetch.
    let again = run(&tmp, &lock, &Options::default());
    assert_eq!((again.kept, again.installed.len()), (2, 0));

    // Without dev packages, acme/tools and its bin go; the path package it linked to stays.
    let no_dev = run(
        &tmp,
        &lock,
        &Options {
            no_dev: true,
            ..Options::default()
        },
    );
    assert_eq!(no_dev.removed, ["acme/tools"]);
    assert!(std::fs::symlink_metadata(tmp.join("vendor/acme/tools")).is_err());
    assert!(!tmp.join("vendor/bin/tool.sh").exists());
    assert!(tmp.join("packages/tools/bin/tool.sh").exists());
    let classmap =
        std::fs::read_to_string(tmp.join("vendor/composer/autoload_classmap.php")).unwrap();
    assert!(
        !classmap.contains("Legacy_Old")
            && !classmap.contains("KernelTest")
            && classmap.contains("App\\\\Kernel")
    );
    std::fs::remove_dir_all(&tmp).unwrap();
}

#[test]
fn autoloader_and_bin_corner_cases_match_composer() {
    let tmp = std::env::temp_dir().join(format!("pun-install-edge-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    copy(&Path::new(FIXTURES).join("edge"), &tmp);
    let lock =
        Lock::from_composer_lock(&std::fs::read_to_string(tmp.join("composer.lock")).unwrap())
            .unwrap();
    let o = Options {
        ignore_platform: vec!["php".into(), "ext-sodium-*".into()],
        ..Options::default()
    };
    let report = run(&tmp, &lock, &o);
    same_as_composer(&tmp, "edge-expected");
    // Composer's warnings, in its order; its paths are /tmp/edge.
    let at = tmp.to_string_lossy();
    let warnings: Vec<String> = report
        .warnings
        .iter()
        .map(|w| w.replace(&*at, "/tmp/edge"))
        .collect();
    assert_eq!(
        warnings,
        [
            "Skipped installation of bin bin/nofile for package acme/legacy: file not found in package",
            "Skipped installation of bin bin/ for package acme/legacy: found a directory at that path",
            "Skipped installation of bin bin/escape for package acme/legacy: the bin resolves to a path outside of the package directory",
            "Skipped installation of bin bin/bad$name for package acme/legacy: the bin path contains invalid characters",
            "Skipped installation of bin bin/dup for package acme/legacy: name conflicts with an existing file",
            "Ambiguous class resolution, \"Dup\\Thing\" was found 3x: in \"/tmp/edge/classes/Thing.php\" and \"/tmp/edge/dup/Thing.php\", \"/tmp/edge/dup/Thing2.php\", the first will be used.",
            "Ambiguous class resolution, \"Dup\\Pair\" was found in both \"/tmp/edge/classes/Pair.php\" and \"/tmp/edge/dup/Pair.php\", the first will be used.",
        ]
    );

    // Composer stops at a file it can't read and at a classmap path that isn't there (`composer dump-autoload`).
    let pair = tmp.join("classes/Pair.php");
    std::fs::set_permissions(&pair, std::fs::Permissions::from_mode(0o000)).unwrap();
    let error = try_run(&tmp, &lock, &o).unwrap_err().to_string();
    std::fs::set_permissions(&pair, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(
        error,
        format!(
            "File at \"{}\" is not readable, check its permissions",
            pair.display()
        )
    );
    let mut missing = lock;
    let legacy = (missing.packages.iter_mut())
        .find(|p| p["name"] == "acme/legacy")
        .unwrap();
    legacy["autoload"]["classmap"] = serde_json::json!(["code/", "gone/"]);
    assert_eq!(
        try_run(&tmp, &missing, &o).unwrap_err().to_string(),
        format!(
            "Could not scan for classes inside \"{}/vendor/acme/legacy/gone/\" which does not appear to be a file nor a folder",
            tmp.display()
        )
    );
    std::fs::remove_dir_all(&tmp).unwrap();
}

#[test]
fn lock_changes_plugins_and_failures() {
    let tmp = std::env::temp_dir().join(format!("pun-install-changes-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    copy(&Path::new(FIXTURES).join("project"), &tmp);
    let lock =
        Lock::from_composer_lock(&std::fs::read_to_string(tmp.join("composer.lock")).unwrap())
            .unwrap();
    run(&tmp, &lock, &Options::default());
    let package = |v: Value| v.as_object().unwrap().clone();

    // A new version of a locked package is an update; a metapackage installs no files.
    let mut changed = lock.clone();
    changed.packages[0]["version"] = "1.2.1".into();
    changed.packages.push(package(
        serde_json::json!({"name": "acme/meta", "version": "1.0.0", "type": "metapackage"}),
    ));
    let report = run(&tmp, &changed, &Options::default());
    assert_eq!(
        report.updated,
        [("acme/lib".into(), "1.2.0".into(), "1.2.1".into())]
    );
    assert_eq!(report.installed, [("acme/meta".into(), "1.0.0".into())]);
    assert!(!tmp.join("vendor/acme/meta").exists());

    // A zip dist comes from the cache when its checksum matches (Composer's cache layout: the URL's sha1).
    let zip = {
        use std::io::Write;
        let mut w = zip::ZipWriter::new(std::io::Cursor::new(vec![]));
        w.start_file(
            "acme-zipped-1/src/Z.php",
            zip::write::SimpleFileOptions::default(),
        )
        .unwrap();
        w.write_all(b"<?php class Z {}\n").unwrap();
        w.finish().unwrap().into_inner()
    };
    let url = "https://repo.acme.test/zipped.zip";
    let cache = tmp.join("cache/files/acme/zipped");
    std::fs::create_dir_all(&cache).unwrap();
    std::fs::write(
        cache.join(format!("{}.zip", pun_packages::php::sha1(url.as_bytes()))),
        &zip,
    )
    .unwrap();
    let mut zipped = lock.clone();
    zipped.packages.push(package(serde_json::json!({"name": "acme/zipped", "version": "1.0.0", "dist": {"type": "zip", "url": url, "shasum": pun_packages::php::sha1(&zip)}})));
    let report = run(&tmp, &zipped, &Options::default());
    assert_eq!(report.cached, 1);
    assert!(tmp.join("vendor/acme/zipped/src/Z.php").exists());

    // Every package that can't be placed is reported, in lock order.
    let mut failing = lock.clone();
    failing.packages.push(package(serde_json::json!({"name": "acme/zip", "version": "1.0.0", "dist": {"type": "zip", "url": "http://example.com/zip.zip"}})));
    failing.packages.push(package(serde_json::json!({"name": "acme/src", "version": "1.0.0", "source": {"type": "git", "url": "https://example.com/src.git", "reference": "abc"}})));
    assert_eq!(
        try_run(&tmp, &failing, &Options::default())
            .unwrap_err()
            .to_string(),
        "installing acme/zip: Your configuration does not allow connections to http://example.com/zip.zip. See https://getcomposer.org/doc/06-config.md#secure-http for details.\n\
         installing acme/src: it has no dist archive, and installing from git sources is not supported yet"
    );
    let mut bad = lock.clone();
    bad.packages[0]["name"] = "Acme/Not Valid".into();
    assert_eq!(
        try_run(&tmp, &bad, &Options::default())
            .unwrap_err()
            .to_string(),
        "pun.lock has a package named `acme/not valid`, which is not a valid package name"
    );

    // PSR violations are reported only outside vendor/, and an absolute `files` path stays absolute (Composer's
    // `composer install` on the same changes).
    std::fs::write(
        tmp.join("packages/tools/lib/Acme/Tools/Bad.php"),
        "<?php class Nope {}\n",
    )
    .unwrap();
    std::fs::write(
        tmp.join("src/Wrong.php"),
        "<?php\nnamespace App;\nclass Nope {}\n",
    )
    .unwrap();
    let manifest_path = tmp.join("composer.json");
    let original = std::fs::read_to_string(&manifest_path).unwrap();
    let mut manifest: Value = serde_json::from_str(&original).unwrap();
    manifest["autoload"]["files"] = serde_json::json!(["/dev/null"]);
    std::fs::write(&manifest_path, manifest.to_string()).unwrap();
    let report = run(&tmp, &lock, &Options::default());
    assert_eq!(
        report.warnings,
        [
            "Class App\\Nope located in ./src/Wrong.php does not comply with psr-4 autoloading standard (rule: App\\ => ./src). Skipping."
        ]
    );
    let files = std::fs::read_to_string(tmp.join("vendor/composer/autoload_files.php")).unwrap();
    assert!(
        files.contains("    '63a99e721f6076a94999e44885027d8b' => '/dev/null',\n"),
        "{files}"
    );
    std::fs::write(&manifest_path, original).unwrap();
    std::fs::remove_file(tmp.join("packages/tools/lib/Acme/Tools/Bad.php")).unwrap();
    std::fs::remove_file(tmp.join("src/Wrong.php")).unwrap();

    // With `php` ignored nothing is left to check, so there is no platform_check.php (Composer's
    // `--ignore-platform-req=php` on the same project).
    let ignore_php = Options {
        ignore_platform: vec!["php".into()],
        ..Options::default()
    };
    run(&tmp, &lock, &ignore_php);
    assert!(!tmp.join("vendor/composer/platform_check.php").exists());
    assert!(
        !std::fs::read_to_string(tmp.join("vendor/composer/autoload_real.php"))
            .unwrap()
            .contains("platform_check")
    );

    // A removed package whose installed.json path leaves vendor/ is dropped from the list but its files stay.
    let installed = tmp.join("vendor/composer/installed.json");
    let mut json: Value =
        serde_json::from_str(&std::fs::read_to_string(&installed).unwrap()).unwrap();
    json["packages"].as_array_mut().unwrap().push(serde_json::json!({"name": "evil/escape", "version": "1.0.0", "install-path": "../../packages/lib"}));
    std::fs::write(&installed, json.to_string()).unwrap();
    let report = run(&tmp, &lock, &Options::default());
    assert_eq!(report.removed, ["evil/escape"]);
    assert!(tmp.join("packages/lib/composer.json").exists());

    // Without `files` autoloads there is no autoload_files.php; `--no-autoloader` leaves the autoloader alone.
    let mut no_files = lock.clone();
    no_files.packages.clear();
    run(&tmp, &no_files, &Options::default());
    assert!(!tmp.join("vendor/composer/autoload_files.php").exists());
    assert!(
        !std::fs::read_to_string(tmp.join("vendor/composer/autoload_real.php"))
            .unwrap()
            .contains("$filesToLoad")
    );
    let no_autoloader = Options {
        no_autoloader: true,
        ..Options::default()
    };
    run(&tmp, &lock, &no_autoloader);
    assert!(!tmp.join("vendor/composer/autoload_files.php").exists());

    // `platform-check: false` drops platform_check.php; allowed plugins run when ported and are reported when not;
    // without `optimize-autoloader` only the classmap directories are scanned.
    let mut manifest: Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();
    manifest["config"]["platform-check"] = false.into();
    manifest["config"]
        .as_object_mut()
        .unwrap()
        .remove("optimize-autoloader");
    manifest["config"]["allow-plugins"] =
        serde_json::json!({"acme/plugin": true, "pestphp/pest-plugin": true});
    std::fs::write(&manifest_path, manifest.to_string()).unwrap();
    let mut plugins = lock;
    for name in ["acme/plugin", "pestphp/pest-plugin"] {
        plugins.packages.push(package(serde_json::json!({"name": name, "version": "1.0.0", "type": "composer-plugin", "dist": {"type": "path", "url": "packages/lib", "reference": "x"}})));
    }
    let report = run(&tmp, &plugins, &Options::default());
    assert!(!tmp.join("vendor/composer/platform_check.php").exists());
    assert!(tmp.join("vendor/pest-plugins.json").exists());
    assert!(report.warnings.contains(&"pun does not run the Composer plugin acme/plugin yet, so the files it writes on install are missing".to_string()));
    let classmap =
        std::fs::read_to_string(tmp.join("vendor/composer/autoload_classmap.php")).unwrap();
    assert!(
        classmap.contains("KernelTest") && !classmap.contains("App\\\\Kernel'"),
        "{classmap}"
    );
    std::fs::remove_file(tmp.join("vendor/pest-plugins.json")).unwrap();
    let no_plugins = Options {
        no_plugins: true,
        ..Options::default()
    };
    let report = run(&tmp, &plugins, &no_plugins);
    assert!(!tmp.join("vendor/pest-plugins.json").exists() && report.warnings.is_empty());
    std::fs::remove_dir_all(&tmp).unwrap();
}
