//! Getting a package's files into `vendor/<name>`: its dist archive from Composer's download cache (shared with
//! Composer, so either tool reuses the other's downloads) or the network, checked against the lock's `shasum`, then
//! unpacked with the archive's single top-level directory dropped. Path dists are symlinked.

use crate::php::sha1;
use anyhow::{Context, Result, bail};
use pcre2::bytes::Regex;
use serde_json::{Map, Value};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// Credentials from `auth.json` (the project's, then `COMPOSER_HOME`'s) and `COMPOSER_AUTH`, by host.
#[derive(Debug, Default, Clone)]
pub struct Auth {
    headers: Vec<(String, String)>,
}

impl Auth {
    /// Reads every source: `env` (`COMPOSER_AUTH`'s value), the project's auth.json, then the one in Composer's home;
    /// a later one does not override a host an earlier one set.
    pub fn load(project: &Path, home: Option<&Path>, env: Option<String>) -> Auth {
        let mut auth = Auth::default();
        let mut texts: Vec<String> = env.into_iter().collect();
        texts.extend(std::fs::read_to_string(project.join("auth.json")));
        if let Some(home) = home {
            texts.extend(std::fs::read_to_string(home.join("auth.json")));
        }
        for text in texts {
            if let Ok(Value::Object(json)) = serde_json::from_str::<Value>(&text) {
                auth.add(&json);
            }
        }
        auth
    }

    fn add(&mut self, json: &Map<String, Value>) {
        let entries = |key: &str| {
            json.get(key)
                .and_then(Value::as_object)
                .into_iter()
                .flatten()
        };
        for (host, token) in entries("github-oauth") {
            if let Some(token) = token.as_str() {
                self.set(host, format!("token {token}"));
                if host == "github.com" {
                    self.set("api.github.com", format!("token {token}"));
                }
            }
        }
        for (host, token) in entries("gitlab-token") {
            match token {
                Value::String(t) => self.set(host, format!("Bearer {t}")),
                Value::Object(t) => self.set(host, basic(t.get("username"), t.get("token"))),
                _ => {}
            }
        }
        for (host, token) in entries("bearer") {
            if let Some(token) = token.as_str() {
                self.set(host, format!("Bearer {token}"));
            }
        }
        for (host, cred) in entries("http-basic") {
            if let Value::Object(c) = cred {
                self.set(host, basic(c.get("username"), c.get("password")));
            }
        }
    }

    fn set(&mut self, host: &str, header: String) {
        if !self.headers.iter().any(|h| h.0 == host) {
            self.headers.push((host.to_string(), header));
        }
    }

    fn header(&self, url: &str) -> Option<&str> {
        let host = url.split("://").nth(1)?.split(['/', '?', '#']).next()?;
        let host = host.rsplit_once('@').map_or(host, |h| h.1);
        let bare = host.split(':').next().unwrap_or(host);
        self.headers
            .iter()
            .find(|h| h.0 == host || h.0 == bare)
            .map(|h| h.1.as_str())
    }
}

fn basic(user: Option<&Value>, password: Option<&Value>) -> String {
    let text = |v: Option<&Value>| v.and_then(Value::as_str).unwrap_or_default().to_string();
    format!(
        "Basic {}",
        base64(format!("{}:{}", text(user), text(password)).as_bytes())
    )
}

fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | u32::from(*b) << (16 - 8 * i));
        for i in 0..4 {
            out.push(if i <= chunk.len() {
                ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char
            } else {
                '='
            });
        }
    }
    out
}

/// Composer's home (where `auth.json` and the global config live) and its download cache, as
/// `Factory::getHomeDir` and `getCacheDir` find them for this process; `None` without `HOME` or `COMPOSER_HOME`.
pub fn composer_dirs() -> Option<(PathBuf, PathBuf)> {
    let xdg_env = std::env::vars_os().any(|(k, _)| k.to_string_lossy().starts_with("XDG_"));
    let xdg = Path::new("/etc/xdg").is_dir() || xdg_env;
    let env = |k: &str| std::env::var(k).ok();
    dirs_in(&env, xdg, cfg!(target_os = "macos"))
}

/// [`composer_dirs`] for an environment: `xdg` when the system uses XDG directories, `macos` on macOS.
fn dirs_in(
    env: &dyn Fn(&str) -> Option<String>,
    xdg: bool,
    macos: bool,
) -> Option<(PathBuf, PathBuf)> {
    let env = |k: &str| env(k).filter(|v| !v.is_empty());
    let composer_home = env("COMPOSER_HOME");
    let user = || env("HOME").map(|h| h.trim_end_matches('/').to_string());
    let home = match &composer_home {
        Some(home) => home.clone(),
        None => {
            let user = user()?;
            let mut dirs = vec![];
            if xdg {
                let config = env("XDG_CONFIG_HOME").unwrap_or_else(|| format!("{user}/.config"));
                dirs.push(format!("{config}/composer"));
            }
            dirs.push(format!("{user}/.composer"));
            (dirs.iter().find(|d| Path::new(d).is_dir()))
                .unwrap_or(&dirs[0])
                .clone()
        }
    };
    let cache = if let Some(dir) = env("COMPOSER_CACHE_DIR") {
        dir
    } else if let Some(home) = &composer_home {
        format!("{home}/cache")
    } else {
        let user = user()?;
        if macos {
            format!("{user}/Library/Caches/composer")
        } else if home == format!("{user}/.composer") && Path::new(&home).join("cache").is_dir() {
            format!("{home}/cache")
        } else if xdg {
            let cache = env("XDG_CACHE_HOME").unwrap_or_else(|| format!("{user}/.cache"));
            format!("{cache}/composer")
        } else {
            format!("{home}/cache")
        }
    };
    Some((PathBuf::from(home), PathBuf::from(cache)))
}

/// The dist URL with its reference updated, as Composer requests (and caches) it.
pub fn dist_url(url: &str, reference: Option<&str>) -> String {
    static RULES: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
        [
            (r"(?i)^https?://(?:www\.)?github\.com/([^/]+)/([^/]+)/(zip|tar)ball/(.+)$", "github"),
            (r"(?i)^https?://(?:www\.)?github\.com/([^/]+)/([^/]+)/archive/.+\.(zip|tar)(?:\.gz)?$", "github"),
            (r"(?i)^https?://api\.github\.com/repos/([^/]+)/([^/]+)/(zip|tar)ball(?:/.+)?$", "github"),
            (r"(?i)^https?://(?:www\.)?bitbucket\.org/([^/]+)/([^/]+)/get/(.+)\.(zip|tar\.gz|tar\.bz2)$", "bitbucket"),
            (r"(?i)^https?://(?:www\.)?gitlab\.com/api/v[34]/projects/([^/]+)/repository/archive\.(zip|tar\.gz|tar\.bz2|tar)\?sha=.+$", "gitlab"),
        ]
        .into_iter()
        .map(|(p, kind)| (Regex::new(p).expect("valid pattern"), kind))
        .collect()
    });
    let Some(reference) = reference.filter(|r| !r.is_empty()) else {
        return url.to_string();
    };
    for (re, kind) in RULES.iter() {
        let Ok(Some(c)) = re.captures(url.as_bytes()) else {
            continue;
        };
        let g = |i: usize| String::from_utf8_lossy(&c[i]).into_owned();
        return match *kind {
            "github" => format!(
                "https://api.github.com/repos/{}/{}/{}ball/{reference}",
                g(1),
                g(2),
                g(3)
            ),
            "bitbucket" => format!(
                "https://bitbucket.org/{}/{}/get/{reference}.{}",
                g(1),
                g(2),
                g(4)
            ),
            _ => format!(
                "https://gitlab.com/api/v4/projects/{}/repository/archive.{}?sha={reference}",
                g(1),
                g(2)
            ),
        };
    }
    url.to_string()
}

pub struct Fetcher {
    agent: ureq::Agent,
    cache: Option<PathBuf>,
    auth: Auth,
    secure_http: bool,
}

impl Fetcher {
    /// `secure_http` is `config.secure-http` (Composer's default, true): no `http://` URL, and no redirect to one.
    pub fn new(cache: Option<PathBuf>, auth: Auth, secure_http: bool) -> Fetcher {
        let config = ureq::Agent::config_builder()
            .user_agent(format!("pun/{}", env!("CARGO_PKG_VERSION")))
            .https_only(secure_http)
            .http_status_as_error(false)
            .build();
        Fetcher {
            agent: config.into(),
            cache,
            auth,
            secure_http,
        }
    }

    /// The package's dist archive bytes: from the cache when its sha1 matches `shasum` (or there is none), else
    /// downloaded, checked, and cached. Returns whether it came from the cache.
    pub fn archive(&self, package: &Map<String, Value>) -> Result<(Vec<u8>, bool)> {
        let name = package["name"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let dist = package
            .get("dist")
            .and_then(Value::as_object)
            .context("no dist")?;
        let kind = dist.get("type").and_then(Value::as_str).unwrap_or("zip");
        let raw = dist
            .get("url")
            .and_then(Value::as_str)
            .context("dist without a url")?;
        let url = dist_url(raw, dist.get("reference").and_then(Value::as_str));
        let shasum = dist
            .get("shasum")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty());
        let cached = (self.cache.as_ref()).map(|c| {
            c.join("files")
                .join(&name)
                .join(format!("{}.{kind}", sha1(url.as_bytes())))
        });
        if let Some(path) = &cached
            && let Ok(bytes) = std::fs::read(path)
            && shasum.is_none_or(|s| s == sha1(&bytes))
        {
            return Ok((bytes, true));
        }
        let scheme = url
            .split("://")
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if self.secure_http
            && url.contains("://")
            && ["http", "git", "ftp", "svn"].contains(&scheme.as_str())
        {
            bail!(
                "Your configuration does not allow connections to {url}. See https://getcomposer.org/doc/06-config.md#secure-http for details."
            );
        }
        let bytes = self
            .get(&url)
            .with_context(|| format!("downloading {name} from {url}"))?;
        if let Some(want) = shasum {
            let got = sha1(&bytes);
            if got != want {
                bail!(
                    "the {name} archive from {url} has sha1 {got}, but pun.lock expects {want}; it may have been tampered with"
                );
            }
        }
        if let Some(path) = &cached
            && let Some(dir) = path.parent()
            && std::fs::create_dir_all(dir).is_ok()
        {
            let tmp = path.with_extension(format!("{kind}.{}.tmp", std::process::id()));
            if std::fs::write(&tmp, &bytes).is_ok() {
                let _ = std::fs::rename(&tmp, path);
            }
        }
        Ok((bytes, false))
    }

    /// GET with the host's credentials; a 429, a 5xx or a failed connection is tried again, up to three times.
    fn get(&self, url: &str) -> Result<Vec<u8>> {
        let mut attempt = 0;
        loop {
            let mut req = self.agent.get(url);
            if let Some(h) = self.auth.header(url) {
                req = req.header("Authorization", h);
            }
            let error = match req.call() {
                Ok(mut res) if res.status().is_success() => {
                    let mut body = vec![];
                    res.body_mut().as_reader().read_to_end(&mut body)?;
                    return Ok(body);
                }
                Ok(res) => {
                    let status = res.status().as_u16();
                    if !(status == 429 || status >= 500) {
                        bail!("HTTP {status}");
                    }
                    anyhow::anyhow!("HTTP {status}")
                }
                Err(e) => e.into(),
            };
            attempt += 1;
            if attempt == 3 {
                return Err(error);
            }
            std::thread::sleep(std::time::Duration::from_millis(500 << (attempt - 1)));
        }
    }
}

static METADATA: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\._.*|\.DS_Store|__MACOSX|\.AppleDouble|\.fseventsd|\.Spotlight-V100|\.Trashes|(?i:thumbs\.db|desktop\.ini))$")
        .expect("valid pattern")
});

/// Unpacks `bytes` (a `zip` or `tar` dist) into `target`, replacing what was there. A single top-level directory
/// (metadata files beside it aside) is unwrapped.
pub fn unpack(kind: &str, bytes: &[u8], target: &Path) -> Result<()> {
    let parent = target.parent().context("target without a parent")?;
    std::fs::create_dir_all(parent)?;
    let tmp = parent.join(format!(
        ".pun-{}-{}",
        target.file_name().unwrap_or_default().to_string_lossy(),
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp)?;
    let result = (|| -> Result<()> {
        match kind {
            "zip" => unzip(bytes, &tmp)?,
            "tar" => {
                let gz = bytes.starts_with(&[0x1f, 0x8b]);
                let reader: Box<dyn Read> = if gz {
                    Box::new(flate2::read::GzDecoder::new(bytes))
                } else {
                    Box::new(bytes)
                };
                tar::Archive::new(reader).unpack(&tmp)?;
            }
            other => bail!("dist type `{other}` is not supported yet"),
        }
        let entries: Vec<PathBuf> = std::fs::read_dir(&tmp)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        let content: Vec<&PathBuf> = (entries.iter())
            .filter(|p| {
                !METADATA
                    .is_match(p.file_name().unwrap_or_default().as_encoded_bytes())
                    .unwrap_or(false)
            })
            .collect();
        let from = match content.as_slice() {
            [only] if only.is_dir() => (*only).clone(),
            _ => tmp.clone(),
        };
        remove(target)?;
        std::fs::rename(&from, target)?;
        Ok(())
    })();
    let _ = std::fs::remove_dir_all(&tmp);
    result
}

/// Extracts a zip into `dir`, its entries in parallel (one large package would otherwise take a core for a second).
/// Entry names that leave `dir` are skipped; symlinks are made last, and only those that point inside `dir`.
fn unzip(bytes: &[u8], dir: &Path) -> Result<()> {
    use rayon::prelude::*;
    use std::os::unix::fs::PermissionsExt;
    let archive = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    let links = std::sync::Mutex::new(vec![]);
    (0..archive.len()).into_par_iter().try_for_each_init(
        || archive.clone(),
        |archive, i| -> Result<()> {
            let mut entry = archive.by_index(i)?;
            let Some(name) = entry.enclosed_name() else {
                return Ok(());
            };
            let path = dir.join(name);
            if entry.is_dir() {
                std::fs::create_dir_all(&path)?;
                return Ok(());
            }
            std::fs::create_dir_all(path.parent().unwrap_or(dir))?;
            let mode = entry.unix_mode();
            if entry.is_symlink() {
                let mut target = String::new();
                entry.read_to_string(&mut target)?;
                (links
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner))
                .push((path, target));
                return Ok(());
            }
            let mut out = std::fs::File::create(&path)?;
            std::io::copy(&mut entry, &mut out)?;
            if let Some(mode) = mode.filter(|m| m & 0o111 != 0) {
                std::fs::set_permissions(
                    &path,
                    std::fs::Permissions::from_mode((mode & 0o777) | 0o600),
                )?;
            }
            Ok(())
        },
    )?;
    for (path, target) in links
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
    {
        let relative = path
            .strip_prefix(dir)
            .ok()
            .and_then(Path::parent)
            .unwrap_or(Path::new(""));
        let resolved = crate::paths::normalize(&format!("{}/{target}", relative.to_string_lossy()));
        if !target.starts_with('/') && !resolved.starts_with("..") && !resolved.contains("/../") {
            let _ = std::os::unix::fs::symlink(&target, &path);
        }
    }
    Ok(())
}

/// A `path` dist: a symlink to the directory (relative when it can be), as Composer's path repository makes by default.
pub fn link_path(url: &str, base: &Path, target: &Path) -> Result<()> {
    let source = if Path::new(url).is_absolute() {
        PathBuf::from(url)
    } else {
        base.join(url)
    };
    let source = std::fs::canonicalize(&source)
        .with_context(|| format!("path repository {url} does not exist"))?;
    remove(target)?;
    std::fs::create_dir_all(target.parent().context("target without a parent")?)?;
    let parent = std::fs::canonicalize(target.parent().unwrap_or(target))?;
    // Composer's relative link ends in a slash.
    let relative =
        crate::paths::shortest(&parent.to_string_lossy(), &source.to_string_lossy(), true);
    std::os::unix::fs::symlink(format!("{}/", relative.trim_end_matches('/')), target)?;
    Ok(())
}

/// Removes a directory tree, a file, or a symlink (without following it).
pub fn remove(path: &Path) -> std::io::Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(m) if m.is_dir() => std::fs::remove_dir_all(path),
        Ok(_) => std::fs::remove_file(path),
        Err(_) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    /// Composer 2.11.2's `Factory::getHomeDir` and `getCacheDir` (through reflection) with the same environment
    /// and directories, on Linux with `/etc/xdg`; the cases without XDG and on macOS follow its code by hand.
    #[test]
    fn composer_dirs_match_composer() {
        let t = scratch("dirs");
        for d in [
            "a",
            "b/.composer/cache",
            "c/cfg/composer",
            "d/.config/composer",
            "d/.composer",
        ] {
            std::fs::create_dir_all(t.join(d)).unwrap();
        }
        let t = t.to_string_lossy();
        let dirs = |vars: &[(&str, String)], xdg: bool, macos: bool| {
            let env = |k: &str| vars.iter().find(|v| v.0 == k).map(|v| v.1.clone());
            dirs_in(&env, xdg, macos).map(|(h, c)| {
                (
                    h.to_string_lossy().into_owned(),
                    c.to_string_lossy().into_owned(),
                )
            })
        };
        let pair = |h: &str, c: &str| Some((h.to_string(), c.to_string()));
        let home = |d: &str| ("HOME", format!("{t}/{d}"));
        let set = |k: &'static str, v: &str| (k, v.to_string());
        assert_eq!(
            dirs(
                &[
                    set("COMPOSER_HOME", "/x"),
                    set("COMPOSER_CACHE_DIR", "/c"),
                    home("a")
                ],
                true,
                false
            ),
            pair("/x", "/c")
        );
        assert_eq!(
            dirs(&[set("COMPOSER_HOME", "/x"), home("a")], true, true),
            pair("/x", "/x/cache")
        );
        assert_eq!(
            dirs(&[home("a/")], true, false),
            pair(
                &format!("{t}/a/.config/composer"),
                &format!("{t}/a/.cache/composer")
            )
        );
        assert_eq!(
            dirs(&[home("b")], true, false),
            pair(
                &format!("{t}/b/.composer"),
                &format!("{t}/b/.composer/cache")
            )
        );
        let xdg = [
            home("c"),
            ("XDG_CONFIG_HOME", format!("{t}/c/cfg")),
            ("XDG_CACHE_HOME", format!("{t}/c/cch")),
        ];
        assert_eq!(
            dirs(&xdg, true, false),
            pair(
                &format!("{t}/c/cfg/composer"),
                &format!("{t}/c/cch/composer")
            )
        );
        assert_eq!(
            dirs(&[home("d")], true, false),
            pair(
                &format!("{t}/d/.config/composer"),
                &format!("{t}/d/.cache/composer")
            )
        );
        assert_eq!(
            dirs(&[home("a")], false, false),
            pair(
                &format!("{t}/a/.composer"),
                &format!("{t}/a/.composer/cache")
            )
        );
        assert_eq!(
            dirs(&[home("b")], false, true),
            pair(
                &format!("{t}/b/.composer"),
                &format!("{t}/b/Library/Caches/composer")
            )
        );
        assert_eq!(dirs(&[set("HOME", "")], true, false), None);
        assert_eq!(dirs(&[set("COMPOSER_CACHE_DIR", "/c")], true, false), None);
        assert!(composer_dirs().is_some(), "HOME is set in tests");
    }

    fn scratch(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("pun-fetch-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    /// Expected URLs from Composer 2.11.2's `Url::updateDistReference($config, $url, 'abc123')` in PHP 8.5.
    #[test]
    fn dist_urls_match_composer() {
        for (url, want) in [
            (
                "https://github.com/acme/lib/zipball/old",
                "https://api.github.com/repos/acme/lib/zipball/abc123",
            ),
            (
                "https://www.github.com/acme/lib/tarball/v1",
                "https://api.github.com/repos/acme/lib/tarball/abc123",
            ),
            (
                "https://github.com/acme/lib/archive/v1.2.zip",
                "https://api.github.com/repos/acme/lib/zipball/abc123",
            ),
            (
                "https://api.github.com/repos/acme/lib/zipball",
                "https://api.github.com/repos/acme/lib/zipball/abc123",
            ),
            (
                "https://api.github.com/repos/Acme/Lib/zipball/abc",
                "https://api.github.com/repos/Acme/Lib/zipball/abc123",
            ),
            (
                "https://bitbucket.org/acme/lib/get/v1.tar.gz",
                "https://bitbucket.org/acme/lib/get/abc123.tar.gz",
            ),
            (
                "https://gitlab.com/api/v4/projects/acme%2Flib/repository/archive.zip?sha=old",
                "https://gitlab.com/api/v4/projects/acme%2Flib/repository/archive.zip?sha=abc123",
            ),
            (
                "https://repo.example.com/dist/acme-lib-1.0.zip",
                "https://repo.example.com/dist/acme-lib-1.0.zip",
            ),
        ] {
            assert_eq!(dist_url(url, Some("abc123")), want, "{url}");
        }
        assert_eq!(
            dist_url("https://github.com/acme/lib/zipball/old", None),
            "https://github.com/acme/lib/zipball/old"
        );
    }

    /// An HTTP server on localhost that answers `requests` requests: 200 with `body`, 404 for `/missing` or 503 for
    /// `/busy`. Returns
    /// its base URL and the Authorization headers it saw.
    fn serve(
        body: &'static [u8],
        requests: usize,
    ) -> (String, std::sync::mpsc::Receiver<Option<String>>) {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for stream in listener.incoming().take(requests) {
                let mut stream = stream.unwrap();
                let mut head = vec![];
                let mut byte = [0];
                while !head.ends_with(b"\r\n\r\n") && stream.read(&mut byte).unwrap() == 1 {
                    head.push(byte[0]);
                }
                let head = String::from_utf8_lossy(&head).into_owned();
                let auth = head.lines().find_map(|l| {
                    l.strip_prefix("authorization: ")
                        .or_else(|| l.strip_prefix("Authorization: "))
                });
                tx.send(auth.map(String::from)).unwrap();
                let response = if head.starts_with("GET /busy") {
                    b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        .to_vec()
                } else if head.starts_with("GET /missing") {
                    b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        .to_vec()
                } else {
                    [
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            body.len()
                        )
                        .as_bytes(),
                        body,
                    ]
                    .concat()
                };
                stream.write_all(&response).unwrap();
            }
        });
        (base, rx)
    }

    fn package(url: &str, shasum: &str) -> Map<String, Value> {
        json!({"name": "Acme/Lib", "dist": {"type": "zip", "url": url, "reference": "", "shasum": shasum}}).as_object().unwrap().clone()
    }

    #[test]
    fn archives_are_checked_cached_and_authorized() {
        const BODY: &[u8] = b"archive bytes";
        let dir = scratch("archive");
        std::fs::write(
            dir.join("auth.json"),
            r#"{"http-basic": {"127.0.0.1": {"username": "user", "password": "pass"}}}"#,
        )
        .unwrap();
        let auth = Auth::load(&dir, None, None);
        let (base, seen) = serve(BODY, 3);
        let url = format!("{base}/a.zip");
        let fetcher = Fetcher::new(Some(dir.join("cache")), auth.clone(), false);

        // Downloaded with the project's credentials (base64 of "user:pass"), checked, and cached under Composer's
        // name for it; the second time it comes from the cache.
        assert_eq!(
            fetcher.archive(&package(&url, &sha1(BODY))).unwrap(),
            (BODY.to_vec(), false)
        );
        assert_eq!(seen.recv().unwrap().as_deref(), Some("Basic dXNlcjpwYXNz"));
        let cached = dir
            .join("cache/files/acme/lib")
            .join(format!("{}.zip", sha1(url.as_bytes())));
        assert_eq!(std::fs::read(&cached).unwrap(), BODY);
        assert_eq!(
            fetcher.archive(&package(&url, &sha1(BODY))).unwrap(),
            (BODY.to_vec(), true)
        );

        // A cached archive that no longer matches is fetched again; a download that doesn't match is refused.
        let error = format!("{:#}", fetcher.archive(&package(&url, "0000")).unwrap_err());
        assert!(
            error.contains("has sha1")
                && error.contains("pun.lock expects 0000; it may have been tampered with"),
            "{error}"
        );
        let error = format!(
            "{:#}",
            fetcher
                .archive(&package(&format!("{base}/missing"), ""))
                .unwrap_err()
        );
        assert!(error.contains("HTTP 404"), "{error}");

        // Without a cache the archive is only downloaded; a busy server is asked three times, a missing one too.
        let (base, seen) = serve(BODY, 6);
        let uncached = Fetcher::new(None, Auth::default(), false);
        let url = format!("{base}/a.zip");
        assert_eq!(
            uncached.archive(&package(&url, "")).unwrap(),
            (BODY.to_vec(), false)
        );
        // A cache directory that is read-only, or a file, doesn't fail the download.
        let readonly = dir.join("readonly");
        std::fs::create_dir_all(readonly.join("files/acme/lib")).unwrap();
        let mode = std::os::unix::fs::PermissionsExt::from_mode(0o555);
        std::fs::set_permissions(readonly.join("files/acme/lib"), mode).unwrap();
        let fetcher = Fetcher::new(Some(readonly.clone()), Auth::default(), false);
        assert_eq!(
            fetcher.archive(&package(&url, "")).unwrap(),
            (BODY.to_vec(), false)
        );
        let mode = std::os::unix::fs::PermissionsExt::from_mode(0o755);
        std::fs::set_permissions(readonly.join("files/acme/lib"), mode).unwrap();
        std::fs::write(dir.join("not-a-dir"), "").unwrap();
        let unwritable = Fetcher::new(Some(dir.join("not-a-dir")), Auth::default(), false);
        assert_eq!(
            unwritable.archive(&package(&url, "")).unwrap(),
            (BODY.to_vec(), false)
        );
        let error = format!(
            "{:#}",
            uncached
                .archive(&package(&format!("{base}/busy"), ""))
                .unwrap_err()
        );
        assert!(error.ends_with("/busy: HTTP 503"), "{error}");
        assert_eq!(seen.iter().take(6).count(), 6);
        let closed = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap();
        let url = format!("http://{closed}/a.zip");
        let error = format!("{:#}", uncached.archive(&package(&url, "")).unwrap_err());
        assert!(
            error.starts_with(&format!("downloading acme/lib from {url}: ")),
            "{error}"
        );

        // Composer's secure-http default refuses plain HTTP before connecting.
        let secure = Fetcher::new(None, auth, true);
        let error = format!("{:#}", secure.archive(&package(&url, "")).unwrap_err());
        assert_eq!(
            error,
            format!(
                "Your configuration does not allow connections to {url}. See https://getcomposer.org/doc/06-config.md#secure-http for details."
            )
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn credentials_by_host() {
        let dir = scratch("auth");
        std::fs::write(
            dir.join("auth.json"),
            r#"{"github-oauth": {"github.com": "gh", "github.acme.test": "ghe"}, "gitlab-token": {"gitlab.com": "gl", "git.acme.test": {"username": "u", "token": "t"}}, "bearer": {"repo.acme.test": "br"}}"#,
        )
        .unwrap();
        let home = scratch("auth-home");
        std::fs::write(
            home.join("auth.json"),
            r#"{"bearer": {"repo.acme.test": "home", "other.test": "o"}}"#,
        )
        .unwrap();
        let env = r#"{"http-basic": {"basic.test": {"username": "ab", "password": "c"}, "skip.test": "x"}, "github-oauth": {"skip.test": 1}, "gitlab-token": {"skip.test": 1}, "bearer": {"skip.test": 1}}"#;
        let auth = Auth::load(&dir, Some(&home), Some(env.into()));
        // base64 of "ab:c"; entries of the wrong shape are skipped.
        assert_eq!(auth.header("https://basic.test/x"), Some("Basic YWI6Yw=="));
        assert_eq!(auth.header("https://skip.test/x"), None);
        assert_eq!(
            Auth::load(&dir, None, Some("not json".into())).header("https://other.test/x"),
            None
        );
        assert_eq!(
            auth.header("https://api.github.com/repos/a/b/zipball/c"),
            Some("token gh")
        );
        assert_eq!(auth.header("https://github.com/a/b"), Some("token gh"));
        // A GitHub Enterprise host gets its token, and no API host is guessed for it.
        assert_eq!(auth.header("https://github.acme.test/x"), Some("token ghe"));
        assert_eq!(
            auth.header("https://gitlab.com/api/v4/x"),
            Some("Bearer gl")
        );
        // base64 of "u:t"
        assert_eq!(auth.header("https://git.acme.test/x"), Some("Basic dTp0"));
        // The project's auth.json wins over COMPOSER_HOME's; a port or user in the URL doesn't hide the host.
        assert_eq!(
            auth.header("https://me@repo.acme.test:8443/p.zip"),
            Some("Bearer br")
        );
        assert_eq!(auth.header("https://other.test/x"), Some("Bearer o"));
        assert_eq!(auth.header("https://packagist.org/x"), None);
        std::fs::remove_dir_all(&dir).unwrap();
        std::fs::remove_dir_all(&home).unwrap();
    }

    /// A zip as GitHub builds them: one top-level directory, with a macOS metadata directory beside it, an
    /// executable, a symlink inside the package, and the entries an attacker adds: a `..` path and a symlink out.
    fn zip_archive() -> Vec<u8> {
        use zip::write::SimpleFileOptions;
        let mut w = zip::ZipWriter::new(std::io::Cursor::new(vec![]));
        let file = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        w.add_directory("acme-lib-1a2b/", file).unwrap();
        w.start_file("acme-lib-1a2b/src/A.php", file).unwrap();
        w.write_all(b"<?php class A {}\n").unwrap();
        w.start_file("acme-lib-1a2b/bin/run", file.unix_permissions(0o755))
            .unwrap();
        w.write_all(b"#!/bin/sh\n").unwrap();
        w.add_symlink("acme-lib-1a2b/link.php", "src/A.php", file)
            .unwrap();
        w.add_symlink("acme-lib-1a2b/escape", "../../outside", file)
            .unwrap();
        w.start_file("../evil.php", file).unwrap();
        w.write_all(b"evil").unwrap();
        w.add_directory("__MACOSX/", file).unwrap();
        w.finish().unwrap().into_inner()
    }

    #[test]
    fn unpack_unwraps_and_stays_inside() {
        let dir = scratch("unpack");
        let target = dir.join("vendor/acme/lib");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::write(target.join("stale.php"), "old").unwrap();
        unpack("zip", &zip_archive(), &target).unwrap();
        assert_eq!(
            std::fs::read_to_string(target.join("src/A.php")).unwrap(),
            "<?php class A {}\n"
        );
        assert!(!target.join("stale.php").exists());
        assert_eq!(
            std::fs::metadata(target.join("bin/run"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o755
        );
        assert_eq!(
            std::fs::read_link(target.join("link.php")).unwrap(),
            Path::new("src/A.php")
        );
        assert!(std::fs::symlink_metadata(target.join("escape")).is_err());
        assert!(
            !dir.join("vendor/acme/evil.php").exists() && !dir.join("vendor/evil.php").exists()
        );
        assert_eq!(
            std::fs::read_dir(dir.join("vendor/acme")).unwrap().count(),
            1,
            "no temporary directory is left"
        );

        // A gzip tar dist; and a dist type with no unpacker.
        let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(
            vec![],
            flate2::Compression::fast(),
        ));
        let mut h = tar::Header::new_gnu();
        h.set_size(5);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(&mut h, "pkg-v1/README", &b"hello"[..])
            .unwrap();
        let tgz = tar.into_inner().unwrap().finish().unwrap();
        unpack("tar", &tgz, &target).unwrap();
        assert_eq!(
            std::fs::read_to_string(target.join("README")).unwrap(),
            "hello"
        );
        assert!(!target.join("src").exists());
        // An entry that can't be written (a file where the archive has a directory) fails the unpack.
        let mut w = zip::ZipWriter::new(std::io::Cursor::new(vec![]));
        let file = zip::write::SimpleFileOptions::default();
        w.start_file("a", file).unwrap();
        w.start_file("a/b", file).unwrap();
        let clash = w.finish().unwrap().into_inner();
        assert!(unpack("zip", &clash, &dir.join("vendor/acme/clash")).is_err());

        // A plain tar with more than one entry at the top is unpacked as it is.
        let mut tar = tar::Builder::new(vec![]);
        for name in ["README", "LICENSE"] {
            let mut h = tar::Header::new_gnu();
            h.set_size(5);
            h.set_mode(0o644);
            h.set_cksum();
            tar.append_data(&mut h, name, &b"hello"[..]).unwrap();
        }
        unpack("tar", &tar.into_inner().unwrap(), &target).unwrap();
        assert!(target.join("README").exists() && target.join("LICENSE").exists());
        for kind in ["rar", "xz"] {
            let error = unpack(kind, b"", &target).unwrap_err().to_string();
            assert_eq!(error, format!("dist type `{kind}` is not supported yet"));
        }

        // A path dist links to the directory, given relative to the project or absolute.
        let package_dir = dir.join("packages/x");
        std::fs::create_dir_all(&package_dir).unwrap();
        let link = dir.join("vendor/acme/x");
        link_path("packages/x", &dir, &link).unwrap();
        assert_eq!(
            std::fs::read_link(&link).unwrap(),
            Path::new("../../packages/x/")
        );
        link_path(&package_dir.to_string_lossy(), Path::new("/"), &link).unwrap();
        assert_eq!(
            std::fs::read_link(&link).unwrap(),
            Path::new("../../packages/x/")
        );
        let error = link_path("packages/gone", &dir, &link)
            .unwrap_err()
            .to_string();
        assert_eq!(error, "path repository packages/gone does not exist");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
