//! `vendor/bin`: one proxy per package binary, as Composer 2 writes them. A PHP binary gets a PHP proxy that includes
//! it (so `php vendor/bin/x` works with any PHP), anything else a shell proxy that `exec`s it.

use crate::paths::{shortest, shortest_code};
use pcre2::bytes::Regex;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::LazyLock;

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("valid pattern")
}

/// Characters no legitimate bin path holds; they only make a proxy unsafe to generate.
static UNSAFE: LazyLock<Regex> = LazyLock::new(|| re(r#"[*$`"&^|<>()%!;\x00-\x1f\x7f]"#));

/// A proxy to write: its name in the bin dir and its text.
#[derive(Debug, PartialEq, Eq)]
pub struct Proxy {
    /// The package's `bin` entry.
    pub bin: String,
    pub name: String,
    pub text: String,
    /// The package file the proxy runs, made executable along with it.
    pub target: String,
}

/// The proxies for one package's `bin` entries, with a warning for each one skipped.
pub fn proxies(
    package: &str,
    install: &str,
    bins: &[String],
    bin_dir: &str,
    vendor: &str,
) -> (Vec<Proxy>, Vec<String>) {
    let (mut out, mut warnings) = (vec![], vec![]);
    let skip = |bin: &str, why: &str| {
        format!("Skipped installation of bin {bin} for package {package}: {why}")
    };
    for bin in bins {
        if UNSAFE.is_match(bin.as_bytes()).unwrap_or(true) {
            warnings.push(skip(bin, "the bin path contains invalid characters"));
            continue;
        }
        let path = format!("{install}/{bin}");
        let file = Path::new(&path);
        if !file.exists() {
            warnings.push(skip(bin, "file not found in package"));
            continue;
        }
        if file.is_dir() {
            warnings.push(skip(bin, "found a directory at that path"));
            continue;
        }
        let inside = (std::fs::canonicalize(file).ok())
            .zip(std::fs::canonicalize(install).ok())
            .is_some_and(|(real, root)| real.starts_with(&root) && real != root);
        if !inside {
            warnings.push(skip(
                bin,
                "the bin resolves to a path outside of the package directory",
            ));
            continue;
        }
        let name = bin.rsplit('/').next().unwrap_or(bin).to_string();
        let link = format!("{bin_dir}/{name}");
        let head = std::fs::read(file)
            .map(|b| b[..b.len().min(500)].to_vec())
            .unwrap_or_default();
        out.push(Proxy {
            bin: bin.clone(),
            text: proxy(&path, &link, &head, vendor),
            name,
            target: path,
        });
    }
    (out, warnings)
}

fn quote_sh(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// The proxy text for the binary `bin` linked at `link`, given the binary's first 500 bytes.
fn proxy(bin: &str, link: &str, head: &[u8], vendor: &str) -> String {
    static PHP_START: LazyLock<Regex> = LazyLock::new(|| re(r"^(#!.*\r?\n)?[\r\n\t ]*<\?php"));
    let relative = shortest(link, bin, false);
    let Ok(Some(found)) = PHP_START.captures(head) else {
        return sh_proxy(&relative);
    };
    let shebang = found.get(1).map(|m| {
        String::from_utf8_lossy(m.as_bytes())
            .trim_end_matches([' ', '\t', '\r', '\n'])
            .to_string()
    });
    let opener = match shebang.as_deref().and_then(shebang_caller) {
        Some(caller) if is_php(&caller) => shebang.unwrap_or_default(),
        _ => "#!/usr/bin/env php".to_string(),
    };
    let exported = shortest_code(link, bin, false, true);
    let comment = relative.replace(['\r', '\n'], " ").replace("*/", "* /");
    let mut globals = "$GLOBALS['_composer_bin_dir'] = __DIR__;\n".to_string();
    let autoload = std::fs::canonicalize(vendor)
        .map_or_else(|_| vendor.to_string(), |p| p.to_string_lossy().into_owned());
    globals.push_str(&format!(
        "$GLOBALS['_composer_autoload_path'] = {};\n",
        shortest_code(link, &format!("{autoload}/autoload.php"), false, true)
    ));
    let phpunit = crate::paths::normalize(bin)
        == crate::paths::normalize(&format!("{vendor}/phpunit/phpunit/phpunit"));
    if phpunit {
        globals.push_str(&format!(
            "$GLOBALS['__PHPUNIT_ISOLATION_EXCLUDE_LIST'] = $GLOBALS['__PHPUNIT_ISOLATION_BLACKLIST'] = array(realpath({exported}));\n"
        ));
    }
    let (mut hint, mut wrapper) = (String::new(), String::new());
    if String::from_utf8_lossy(found.get(0).map_or(&[][..], |m| m.as_bytes())).trim() != "<?php" {
        hint = " using a stream wrapper to prevent the shebang from being output on PHP<8\n *"
            .to_string();
        let (open, read) = if phpunit {
            (
                "'phpvfscomposer://'.",
                "\n                $data = str_replace('__DIR__', var_export(dirname($this->realpath), true), $data);\n                $data = str_replace('__FILE__', var_export($this->realpath, true), $data);",
            )
        } else {
            ("", "")
        };
        wrapper = include_str!("../assets/bin-stream-wrapper.php")
            .replace("{phpunit_open}", open)
            .replace("{phpunit_read}", read)
            .replace("{bin_path}", &exported);
    }
    format!(
        "{opener}\n<?php\n\n/**\n * Proxy PHP file generated by Composer\n *\n * This file includes the referenced bin path ({comment})\n *{hint}\n * @generated\n */\n\nnamespace Composer;\n\n{globals}\n{wrapper}\nreturn include {exported};\n"
    )
}

/// The command a shebang runs, with its arguments, when it is safe to carry over; `env` itself is looked through.
fn shebang_caller(shebang: &str) -> Option<String> {
    static SHEBANG: LazyLock<Regex> = LazyLock::new(|| {
        re(r"^#!(?:/[a-zA-Z0-9_.+-]+)*/([a-zA-Z0-9_.+][a-zA-Z0-9_.+-]*(?:[ \t].*)?)$")
    });
    static ENV: LazyLock<Regex> = LazyLock::new(|| re(r"^env[ \t]+(?:-S[ \t]*)?"));
    let caps = SHEBANG.captures(shebang.as_bytes()).ok()??;
    let command = String::from_utf8_lossy(&caps[1]).into_owned();
    if UNSAFE.is_match(command.as_bytes()).unwrap_or(true) {
        return None;
    }
    let caller = match ENV.find(command.as_bytes()) {
        Ok(Some(m)) => command[m.end()..].to_string(),
        _ => command,
    };
    match caller.as_str() {
        "" | "env" => None,
        c if c.starts_with('-') => Some(format!("env {c}")),
        c => Some(c.to_string()),
    }
}

fn is_php(caller: &str) -> bool {
    static PHP: LazyLock<Regex> = LazyLock::new(|| re(r"^php[0-9.]*(?:[ \t]|$)"));
    PHP.is_match(caller.as_bytes()).unwrap_or(false)
}

fn sh_proxy(relative: &str) -> String {
    let (dir, file) = relative.rsplit_once('/').unwrap_or((".", relative));
    let (dir, file) = (
        quote_sh(if dir.is_empty() { "/" } else { dir }),
        quote_sh(file),
    );
    format!(
        r#"#!/usr/bin/env sh

# Support bash to support `source` with fallback on $0 if this does not run with bash
# https://stackoverflow.com/a/35006505/6512
selfArg="$BASH_SOURCE"
if [ -z "$selfArg" ]; then
    selfArg="$0"
fi

self=$(realpath "$selfArg" 2> /dev/null)
if [ -z "$self" ]; then
    self="$selfArg"
fi

dir=$(cd "${{self%[/\\]*}}" > /dev/null; cd {dir} && pwd)

if [ -d /proc/cygdrive ]; then
    case $(which php) in
        $(readlink -n /proc/cygdrive)/*)
            # We are in Cygwin using Windows php, so the path must be translated
            dir=$(cygpath -m "$dir");
            ;;
    esac
fi

export COMPOSER_RUNTIME_BIN_DIR="$(cd "${{self%[/\\]*}}" > /dev/null; pwd)"

# If bash is sourcing this file, we have to source the target as well
bashSource="$BASH_SOURCE"
if [ -n "$bashSource" ]; then
    if [ "$bashSource" != "$0" ]; then
        source "${{dir}}/"{file} "$@"
        return
    fi
fi

exec "${{dir}}/"{file} "$@"
"#
    )
}

/// Writes the proxy and makes it and its target executable (`0777 & ~umask`, as 0755 under the usual umask).
pub fn write(proxy: &Proxy, bin_dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(bin_dir)?;
    let link = bin_dir.join(&proxy.name);
    if std::fs::read(&link).ok().as_deref() != Some(proxy.text.as_bytes()) {
        std::fs::write(&link, &proxy.text)?;
    }
    for path in [link.as_path(), Path::new(&proxy.target)] {
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}
