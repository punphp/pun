//! Paths as the generated PHP files spell them: normalized with `/`, and written relative to the file that holds
//! them (`../laravel/sail/bin`, `__DIR__ . '/..'`) so `vendor/` keeps working when the project moves.

use crate::php::export_str;

/// `path` with `\` as `/`, `.` and empty segments dropped, `..` applied where it can be, and no trailing `/`.
pub fn normalize(path: &str) -> String {
    let path = path.replace('\\', "/");
    let (absolute, rest) = match path.strip_prefix('/') {
        Some(rest) => ("/", rest),
        None => ("", path.as_str()),
    };
    let mut parts: Vec<&str> = vec![];
    let mut up = false;
    for chunk in rest.split('/') {
        if chunk == ".." && (!absolute.is_empty() || up) {
            parts.pop();
            up = !(parts.is_empty() || parts.last() == Some(&".."));
        } else if chunk != "." && !chunk.is_empty() {
            parts.push(chunk);
            up = chunk != "..";
        }
    }
    format!("{absolute}{}", parts.join("/"))
}

/// The parent of an absolute path (Composer's `findShortestPath` takes only absolute ones).
fn dirname(p: &str) -> &str {
    match p.rfind('/') {
        Some(i) if i > 0 => &p[..i],
        _ => "/",
    }
}

fn basename(p: &str) -> &str {
    p.rsplit('/').next().unwrap_or(p)
}

/// The longest directory both paths share (`/` at least), with a trailing `/`.
fn common(from: &str, to: &str) -> String {
    let mut common = to.to_string();
    while !format!("{from}/").starts_with(&format!("{common}/")) && common != "/" {
        common = dirname(&common).to_string();
    }
    format!("{}/", common.trim_end_matches('/'))
}

/// `to` relative to `from` (both absolute): `from` is a file, or a directory when `directory`.
pub fn shortest(from: &str, to: &str, directory: bool) -> String {
    let mut from = normalize(from);
    let to = normalize(to);
    if directory {
        from = format!("{}/dummy_file", from.trim_end_matches('/'));
    }
    if dirname(&from) == dirname(&to) {
        return format!("./{}", basename(&to));
    }
    let common = common(&from, &to);
    let depth = from[common.len()..].matches('/').count();
    // Two top-level directories (`/foo` and `/bar`, common in containers) stay absolute.
    if common == "/" && depth > 1 {
        return to;
    }
    let result = format!(
        "{}{}",
        "../".repeat(depth),
        &to[common.len().min(to.len())..]
    );
    if result.is_empty() {
        "./".into()
    } else {
        result
    }
}

/// PHP code for `to` seen from the file `from` (or the directory, when `directory`): `__DIR__ . '/..'.'/x'`, or with
/// `dirname()` calls instead of `/..` when not `static_code`.
pub fn shortest_code(from: &str, to: &str, directory: bool, static_code: bool) -> String {
    let from = normalize(from);
    let to = normalize(to);
    if from == to {
        return if directory { "__DIR__" } else { "__FILE__" }.into();
    }
    let common = common(&from, &to);
    if let Some(rest) = to.strip_prefix(&format!("{from}/")) {
        return format!("__DIR__ . {}", export_str(&format!("/{rest}")));
    }
    let depth = from[common.len()..].matches('/').count() + usize::from(directory);
    if common == "/" && depth > 1 {
        return export_str(&to);
    }
    let up = if static_code {
        format!("__DIR__ . '{}'", "/..".repeat(depth))
    } else {
        format!("{}__DIR__{}", "dirname(".repeat(depth), ")".repeat(depth))
    };
    let rel = &to[common.len().min(to.len())..];
    if rel.is_empty() {
        up
    } else {
        format!("{up}.{}", export_str(&format!("/{rel}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected values from Composer 2.11.2's `Filesystem` (`normalizePath`, `findShortestPath`,
    /// `findShortestPathCode`) run in PHP 8.5.
    #[test]
    fn paths_match_composer() {
        assert_eq!(normalize("/a/./b//c/../d/"), "/a/b/d");
        assert_eq!(normalize("../a/../../b"), "../../b");
        assert_eq!(normalize("a\\b"), "a/b");
        let v = "/p/vendor";
        assert_eq!(
            shortest(
                "/p/vendor/bin/sail",
                "/p/vendor/laravel/sail/bin/sail",
                false
            ),
            "../laravel/sail/bin/sail"
        );
        assert_eq!(
            shortest("/p/vendor/composer", "/p/vendor/acme/lib", true),
            "../acme/lib"
        );
        assert_eq!(shortest("/p/vendor/composer", "/p", true), "../../");
        assert_eq!(shortest("/p", "/p", true), "./");
        assert_eq!(shortest("/p/a", "/p/b", false), "./b");
        assert_eq!(shortest("/foo/a/b", "/bar/c", false), "/bar/c");
        assert_eq!(
            shortest_code(v, "/p/vendor/composer", true, false),
            "__DIR__ . '/composer'"
        );
        assert_eq!(
            shortest_code("/p/vendor/composer", v, true, false),
            "dirname(__DIR__)"
        );
        assert_eq!(shortest_code(v, "/p", true, false), "dirname(__DIR__)");
        assert_eq!(
            shortest_code("/p/vendor/composer", "/p", true, true),
            "__DIR__ . '/../..'"
        );
        assert_eq!(
            shortest_code(
                "/p/vendor/bin/pest",
                "/p/vendor/pestphp/pest/bin/pest",
                false,
                true
            ),
            "__DIR__ . '/..'.'/pestphp/pest/bin/pest'"
        );
        assert_eq!(shortest_code(v, v, true, true), "__DIR__");
        assert_eq!(shortest_code("/p/x", "/p/x", false, true), "__FILE__");
        assert_eq!(shortest_code("/foo/a/b", "/bar/c", false, true), "'/bar/c'");
    }
}
