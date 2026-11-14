//! `pun self-update`: replaces this binary with the latest release, or the one named, checked as the curl installer
//! (docs/public/install) checks it: HTTPS only, the release's SHA-256, the GitHub build attestation when a signed-in
//! `gh` can verify it (required with `PUN_REQUIRE_ATTESTATION=1`), an archive holding only `punphp-<target>/` files,
//! and a binary that reports the version asked for. The new binary is written beside this one and renamed over it,
//! so a running `pun` never sees half a file. A copy installed through Composer is updated by Composer instead.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Component, Path};
use std::process::Command;

const REPO: &str = "punphp/pun";

#[derive(clap::Args)]
pub(crate) struct SelfUpdateArgs {
    /// The release to install, such as `v0.2.0` (default: the latest).
    version: Option<String>,
    /// Only report whether a newer release is out.
    #[arg(long)]
    check: bool,
}

pub(crate) fn run(a: SelfUpdateArgs) -> Result<u8> {
    let exe = std::env::current_exe()?
        .canonicalize()
        .context("can't find the running pun")?;
    if exe.file_name().is_none_or(|n| n != "pun") {
        bail!("this pun was installed with Composer; update it with `composer update punphp/pun`");
    }
    let current = env!("CARGO_PKG_VERSION");
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .https_only(true)
        .user_agent(format!("pun/{current}"))
        .build()
        .into();
    let tag = match &a.version {
        Some(v) => format!("v{}", v.trim_start_matches('v')),
        None => latest(&agent)?,
    };
    if !valid_tag(&tag) {
        bail!("not a release version: {tag}");
    }
    let version = &tag[1..];
    if a.version.is_none() && pun_packages::version::compare(version, current) <= 0 {
        eprintln!("pun {current} is up to date (the latest release is {version})");
        return Ok(0);
    }
    if a.check {
        eprintln!("pun {version} is out (this is {current}); run `pun self-update` to install it");
        return Ok(0);
    }

    let target = target()?;
    let archive = format!("punphp-{target}.tar.gz");
    let url = format!("https://github.com/{REPO}/releases/download/{tag}/{archive}");
    eprintln!("Downloading pun {version} for {target}");
    let bytes = get(&agent, &url)?;
    let sums = String::from_utf8(get(&agent, &format!("{url}.sha256"))?).unwrap_or_default();
    let expected = sums.split_whitespace().next().unwrap_or_default();
    let actual = hex(&Sha256::digest(&bytes));
    if expected.is_empty() || actual != expected {
        bail!(
            "checksum mismatch for {archive} (expected {expected}, got {actual}); the download may be corrupt"
        );
    }
    eprintln!(" ✔ verified the SHA-256 checksum");
    attestation(&bytes, &archive, &tag)?;
    let binary = extract(&bytes, &target).with_context(|| format!("unpacking {archive}"))?;

    let dir = exe.parent().context("the running pun has no directory")?;
    let staged = dir.join(format!(".pun.{}", std::process::id()));
    std::fs::write(&staged, &binary)
        .with_context(|| format!("can't write to {}", dir.display()))?;
    let result = (|| {
        std::fs::set_permissions(&staged, std::fs::Permissions::from_mode(0o755))?;
        let out = Command::new(&staged)
            .arg("--version")
            .output()
            .context("the downloaded pun does not run on this machine")?;
        let reported = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if reported != format!("pun {version}") {
            bail!("the downloaded pun reports '{reported}', not 'pun {version}'");
        }
        std::fs::rename(&staged, &exe).with_context(|| format!("can't replace {}", exe.display()))
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&staged);
    }
    result?;
    eprintln!("Updated pun {current} → {version} at {}", exe.display());
    Ok(0)
}

/// A plain release version: `v` and three dot-separated numbers, then only letters, digits, dots and dashes.
fn valid_tag(tag: &str) -> bool {
    let Some(rest) = tag.strip_prefix('v') else {
        return false;
    };
    let mut parts = rest.splitn(3, '.');
    let (Some(major), Some(minor), Some(patch)) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let patch_start = patch
        .split(|c: char| !c.is_ascii_digit())
        .next()
        .unwrap_or_default();
    digits(major)
        && digits(minor)
        && digits(patch_start)
        && rest
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
}

/// The latest stable release, else the newest pre-release; the releases feed when the API's rate limit is used up.
fn latest(agent: &ureq::Agent) -> Result<String> {
    let api = format!("https://api.github.com/repos/{REPO}/releases");
    for url in [format!("{api}/latest"), format!("{api}?per_page=1")] {
        let Ok(body) = get(agent, &url) else { continue };
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap_or_default();
        let release = if json.is_array() { &json[0] } else { &json };
        if let Some(tag) = release["tag_name"].as_str() {
            return Ok(tag.to_string());
        }
    }
    let feed = String::from_utf8(get(
        agent,
        &format!("https://github.com/{REPO}/releases.atom"),
    )?)
    .unwrap_or_default();
    (feed.split("/releases/tag/").nth(1))
        .and_then(|rest| rest.split(['"', '<']).next())
        .map(String::from)
        .context("could not find the latest release; name one: `pun self-update v0.1.0`")
}

fn get(agent: &ureq::Agent, url: &str) -> Result<Vec<u8>> {
    let mut res = agent
        .get(url)
        .call()
        .with_context(|| format!("download failed: {url}"))?;
    let mut body = vec![];
    res.body_mut()
        .as_reader()
        .read_to_end(&mut body)
        .with_context(|| format!("download failed: {url}"))?;
    Ok(body)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The release archive this machine runs; a shell under Rosetta still gets the Apple silicon build.
fn target() -> Result<String> {
    let mut arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        other => {
            bail!("unsupported CPU architecture {other} (Pun ships x86_64 and aarch64 binaries)")
        }
    };
    match std::env::consts::OS {
        "macos" => {
            let translated = Command::new("sysctl")
                .args(["-n", "sysctl.proc_translated"])
                .output();
            if arch == "x86_64" && translated.is_ok_and(|o| o.stdout.trim_ascii() == b"1") {
                arch = "aarch64";
            }
            Ok(format!("{arch}-apple-darwin"))
        }
        "linux" => Ok(format!("{arch}-unknown-linux-musl")),
        other => bail!("unsupported OS {other} (Pun ships Linux and macOS binaries)"),
    }
}

/// GitHub's build attestation: proof the archive came from the release workflow for `tag` on a GitHub-hosted runner,
/// which the checksum (published beside the archive) cannot give.
fn attestation(bytes: &[u8], archive: &str, tag: &str) -> Result<()> {
    let required = std::env::var("PUN_REQUIRE_ATTESTATION").is_ok_and(|v| v == "1");
    let gh = Command::new("gh")
        .args(["auth", "status"])
        .output()
        .is_ok_and(|o| o.status.success());
    if !gh {
        if required {
            bail!("PUN_REQUIRE_ATTESTATION=1 needs the GitHub CLI (gh), signed in");
        }
        eprintln!(" · skipped the build attestation (needs a signed-in gh)");
        return Ok(());
    }
    let file = std::env::temp_dir().join(format!("pun-{}-{archive}", std::process::id()));
    std::fs::write(&file, bytes)?;
    let status = Command::new("gh")
        .args(["attestation", "verify"])
        .arg(&file)
        .args([
            "--repo",
            REPO,
            "--signer-workflow",
            &format!("github.com/{REPO}/.github/workflows/release.yml"),
        ])
        .args([
            "--source-ref",
            &format!("refs/tags/{tag}"),
            "--deny-self-hosted-runners",
        ])
        .output();
    let _ = std::fs::remove_file(&file);
    if !status.is_ok_and(|o| o.status.success()) {
        bail!(
            "the build attestation of {archive} does not verify; the download may have been tampered with"
        );
    }
    eprintln!(" ✔ verified the GitHub build attestation");
    Ok(())
}

/// The `pun` binary from the archive, which may hold only regular files and directories under `punphp-<target>/`.
fn extract(bytes: &[u8], target: &str) -> Result<Vec<u8>> {
    let top = format!("punphp-{target}");
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(bytes));
    let mut binary = None;
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        let kind = entry.header().entry_type();
        if !(kind.is_file() || kind.is_dir()) {
            bail!(
                "it holds an entry that is not a file or directory: {}",
                path.display()
            );
        }
        let mut parts = path.components();
        let inside = parts.next() == Some(Component::Normal(top.as_ref()))
            && parts.all(|c| matches!(c, Component::Normal(_)));
        if !inside {
            bail!("it holds an unexpected path: {}", path.display());
        }
        if kind.is_file() && path == Path::new(&top).join("pun") {
            let mut data = vec![];
            entry.read_to_end(&mut data)?;
            binary = Some(data);
        }
    }
    binary.context("it holds no pun binary")
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: &str = "x86_64-unknown-linux-musl";

    /// A gzip tar of (path, entry type, contents); the path bytes go into the header as they are, `..` included.
    fn archive(entries: &[(&str, tar::EntryType, &[u8])]) -> Vec<u8> {
        let mut out = tar::Builder::new(flate2::write::GzEncoder::new(
            vec![],
            flate2::Compression::fast(),
        ));
        for (path, kind, data) in entries {
            let mut h = tar::Header::new_old();
            h.as_old_mut().name[..path.len()].copy_from_slice(path.as_bytes());
            h.set_entry_type(*kind);
            h.set_size(data.len() as u64);
            h.set_mode(0o755);
            h.set_cksum();
            out.append(&h, *data).unwrap();
        }
        out.into_inner().unwrap().finish().unwrap()
    }

    #[test]
    fn extract_takes_only_the_release_layout() {
        use tar::EntryType::{Directory, Regular, Symlink};
        let top = format!("punphp-{T}/");
        let pun = format!("punphp-{T}/pun");
        let good = archive(&[
            (&top, Directory, b""),
            (&format!("{top}LICENSE"), Regular, b"MIT"),
            (&pun, Regular, b"ELF"),
        ]);
        assert_eq!(extract(&good, T).unwrap(), b"ELF");
        let error = |entries: &[(&str, tar::EntryType, &[u8])]| {
            format!("{:#}", extract(&archive(entries), T).unwrap_err())
        };
        assert!(error(&[(&pun, Symlink, b"")]).contains("not a file or directory"));
        assert!(error(&[(&format!("{top}../pun"), Regular, b"x")]).contains("unexpected path"));
        assert!(
            error(&[("punphp-aarch64-apple-darwin/pun", Regular, b"x")])
                .contains("unexpected path")
        );
        assert!(error(&[("/etc/pun", Regular, b"x")]).contains("unexpected path"));
        assert!(error(&[(&format!("{top}LICENSE"), Regular, b"MIT")]).contains("no pun binary"));
    }

    #[test]
    fn tags_must_be_release_versions() {
        for tag in ["v0.1.0", "v12.30.4", "v0.2.0-beta.1"] {
            assert!(valid_tag(tag), "{tag}");
        }
        for tag in [
            "vlatest",
            "0.1.0",
            "v1.2",
            "v1.2.x",
            "v1.2.3/../x",
            "v1.2.3?a=b",
            "v.1.2",
        ] {
            assert!(!valid_tag(tag), "{tag}");
        }
    }
}
