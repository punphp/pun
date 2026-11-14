//! Package versions as Composer reads them: the normalized form `installed.json` and `installed.php` store
//! (`v1.2` → `1.2.0.0`, `5.x-dev` → `5.9999999.9999999.9999999-dev`), branch aliases, PHP's `version_compare`, and the
//! lowest version a constraint admits, which `platform_check.php` tests the running PHP against.

use anyhow::{Result, bail};
use pcre2::bytes::{Captures, Regex};
use std::cmp::Ordering;
use std::sync::LazyLock;

const MODIFIER: &str =
    r"[._-]?(?:(stable|beta|b|RC|alpha|a|patch|pl|p)((?:[.-]?\d+)*+)?)?([.-]?dev)?";

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("valid pattern")
}

fn group<'a>(c: &'a Captures, i: usize) -> &'a str {
    c.get(i)
        .map_or("", |m| std::str::from_utf8(m.as_bytes()).unwrap_or(""))
}

static ALIAS: LazyLock<Regex> = LazyLock::new(|| re(r"^([^,\s]++) ++as ++([^,\s]++)$"));
static FLAG: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)@(?:stable|RC|beta|alpha|dev)$"));
static META: LazyLock<Regex> = LazyLock::new(|| re(r"^([^,\s+]++)\+[^\s]++$"));
static CLASSIC: LazyLock<Regex> = LazyLock::new(|| {
    re(&format!(
        r"(?i)^v?(\d{{1,5}}+)(\.\d++)?(\.\d++)?(\.\d++)?{MODIFIER}$"
    ))
});
static DATE: LazyLock<Regex> = LazyLock::new(|| {
    re(&format!(
        r"(?i)^v?(\d{{4}}(?:[.:-]?\d{{2}}){{1,6}}(?:[.:-]?\d{{1,3}}){{0,2}}){MODIFIER}$"
    ))
});
static DEV: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)(.*?)[.-]?dev$"));
static BRANCH: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)^v?(\d++)(\.(?:\d++|[xX*]))?(\.(?:\d++|[xX*]))?(\.(?:\d++|[xX*]))?$")
});
static NUMERIC_ALIAS: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)^((\d++\.)*\d++)(?:\.x)?-dev$"));

fn stability(s: &str) -> String {
    match s.to_ascii_lowercase().as_str() {
        "a" => "alpha".into(),
        "b" => "beta".into(),
        "p" | "pl" => "patch".into(),
        "rc" => "RC".into(),
        other => other.into(),
    }
}

/// The normalized form of a version (`VersionParser::normalize`): four numeric parts and a stability suffix, a
/// `dev-` branch kept as it is, aliases (`x as y`), stability flags (`@dev`) and build metadata (`+foo`) dropped.
pub fn normalize(version: &str) -> Result<String> {
    let original = version.trim();
    let mut v = original.to_string();
    if let Some(c) = ALIAS.captures(v.as_bytes())? {
        v = group(&c, 1).to_string();
    }
    if let Some(m) = FLAG.find(v.as_bytes())? {
        v.truncate(m.start());
    }
    if matches!(v.as_str(), "master" | "trunk" | "default") {
        v = format!("dev-{v}");
    }
    if v.len() >= 4 && v[..4].eq_ignore_ascii_case("dev-") {
        return Ok(format!("dev-{}", &v[4..]));
    }
    if let Some(c) = META.captures(v.as_bytes())? {
        v = group(&c, 1).to_string();
    }
    let matched = if let Some(c) = CLASSIC.captures(v.as_bytes())? {
        let part = |i| match group(&c, i) {
            "" => ".0".to_string(),
            p => p.to_string(),
        };
        Some((
            format!("{}{}{}{}", group(&c, 1), part(2), part(3), part(4)),
            c,
            5,
        ))
    } else if let Some(c) = DATE.captures(v.as_bytes())? {
        let base = group(&c, 1)
            .chars()
            .map(|ch| if ch.is_ascii_digit() { ch } else { '.' })
            .collect();
        Some((base, c, 2))
    } else {
        None
    };
    if let Some((mut out, c, i)) = matched {
        let modifier = group(&c, i);
        if !modifier.is_empty() {
            if modifier == "stable" {
                return Ok(out);
            }
            out = format!(
                "{out}-{}{}",
                stability(modifier),
                group(&c, i + 1).trim_start_matches(['.', '-'])
            );
        }
        if !group(&c, i + 2).is_empty() {
            out.push_str("-dev");
        }
        return Ok(out);
    }
    if let Some(c) = DEV.captures(v.as_bytes())? {
        let branch = normalize_branch(group(&c, 1));
        if !branch.starts_with("dev-") {
            return Ok(branch);
        }
    }
    bail!("Invalid version string \"{original}\"")
}

/// A branch name as a version: a numeric branch (`5.x`, `2.1`) becomes `5.9999999.9999999.9999999-dev`, any other
/// name `dev-<name>`.
pub fn normalize_branch(name: &str) -> String {
    let name = name.trim();
    let Ok(Some(c)) = BRANCH.captures(name.as_bytes()) else {
        return format!("dev-{name}");
    };
    let mut v = String::new();
    for i in 1..5 {
        match c.get(i) {
            Some(_) => v.push_str(&group(&c, i).replace(['*', 'X'], "x")),
            None => v.push_str(".x"),
        }
    }
    format!("{}-dev", v.replace('x', "9999999"))
}

/// The alias Composer installs a dev version under, as (normalized, pretty): `extra.branch-alias` for the version
/// (`dev-main` → `1.x-dev`), else `9999999-dev` for a non-numeric default branch.
pub fn branch_alias(package: &serde_json::Value) -> Option<(String, String)> {
    let version = package.get("version")?.as_str()?;
    if !version.starts_with("dev-") && !version.ends_with("-dev") {
        return None;
    }
    let numeric = |b: &str| -> Option<String> {
        let c = NUMERIC_ALIAS.captures(b.as_bytes()).ok()??;
        Some(format!("{}.", group(&c, 1)))
    };
    let aliases = package
        .pointer("/extra/branch-alias")
        .and_then(|a| a.as_object());
    for (source, target) in aliases.into_iter().flatten() {
        let Some(target) = target.as_str() else {
            continue;
        };
        let Some(stem) = target.strip_suffix("-dev") else {
            continue;
        };
        let validated = if target == "9999999-dev" {
            target.to_string()
        } else {
            normalize_branch(stem)
        };
        if !validated.ends_with("-dev") || !version.eq_ignore_ascii_case(source) {
            continue;
        }
        if let (Some(s), Some(t)) = (numeric(source), numeric(target))
            && !t.to_ascii_lowercase().starts_with(&s.to_ascii_lowercase())
        {
            continue;
        }
        return Some(pretty_alias(validated));
    }
    let default = package
        .get("default-branch")
        .and_then(serde_json::Value::as_bool)
        == Some(true);
    (default && numeric(version.trim_start_matches('v')).is_none())
        .then(|| pretty_alias("9999999-dev".into()))
}

fn pretty_alias(normalized: String) -> (String, String) {
    static NINES: LazyLock<Regex> = LazyLock::new(|| re(r"(\.9{7})+"));
    let mut pretty = String::new();
    let mut at = 0;
    for m in NINES.find_iter(normalized.as_bytes()).flatten() {
        pretty.push_str(&normalized[at..m.start()]);
        pretty.push_str(".x");
        at = m.end();
    }
    pretty.push_str(&normalized[at..]);
    (normalized, pretty)
}

/// `BasePackage::packageNameToRegexp`: whether `name` matches `pattern`, where `*` matches any run of characters
/// (`/` included), case-insensitively. Composer's `allow-plugins` and `--ignore-platform-req` match names this way.
pub fn name_matches(pattern: &str, name: &str) -> bool {
    let (pattern, name) = (pattern.to_ascii_lowercase(), name.to_ascii_lowercase());
    let mut parts = pattern.split('*');
    let first = parts.next().unwrap_or_default();
    let Some(mut rest) = name.strip_prefix(first) else {
        return false;
    };
    let parts: Vec<&str> = parts.collect();
    for (i, part) in parts.iter().enumerate() {
        if i + 1 == parts.len() {
            return rest.len() >= part.len() && rest.ends_with(part);
        }
        match rest.find(part) {
            Some(at) => rest = &rest[at + part.len()..],
            None => return false,
        }
    }
    rest.is_empty()
}

/// Whether a package name is the platform's (PHP, an extension, a library, Composer's APIs), not a package.
pub fn is_platform(name: &str) -> bool {
    static PLATFORM: LazyLock<Regex> = LazyLock::new(|| {
        re(
            r"(?i)^(?:php(?:-64bit|-ipv6|-zts|-debug)?|hhvm|(?:ext|lib)-[a-z0-9](?:[_.-]?[a-z0-9]+)*|composer(?:-(?:plugin|runtime)-api)?)$",
        )
    });
    PLATFORM.is_match(name.as_bytes()).unwrap_or(false)
}

/// PHP's `version_compare($a, $b)`: -1, 0 or 1.
pub fn compare(a: &str, b: &str) -> i32 {
    if a.is_empty() || b.is_empty() {
        return match (a.is_empty(), b.is_empty()) {
            (true, true) => 0,
            (true, false) => -1,
            _ => 1,
        };
    }
    let (a, b) = (canonical(a), canonical(b));
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let (mut p1, mut p2) = (0usize, 0usize);
    let (mut n1, mut n2): (Option<usize>, Option<usize>) = (Some(0), Some(0));
    let mut result = 0;
    let dot = |s: &[u8], from: usize| s[from..].iter().position(|&c| c == b'.').map(|i| from + i);
    while p1 < a.len() && p2 < b.len() && n1.is_some() && n2.is_some() {
        n1 = dot(a, p1);
        n2 = dot(b, p2);
        let s1 = &a[p1..n1.unwrap_or(a.len())];
        let s2 = &b[p2..n2.unwrap_or(b.len())];
        result = segment(s1, s2);
        if result != 0 {
            break;
        }
        if let Some(n) = n1 {
            p1 = n + 1;
        }
        if let Some(n) = n2 {
            p2 = n + 1;
        }
    }
    if result == 0 {
        if n1.is_some() {
            result = if a.get(p1).is_some_and(u8::is_ascii_digit) {
                1
            } else {
                compare(&String::from_utf8_lossy(&a[p1..]), "#N#")
            };
        } else if n2.is_some() {
            result = if b.get(p2).is_some_and(u8::is_ascii_digit) {
                -1
            } else {
                compare("#N#", &String::from_utf8_lossy(&b[p2..]))
            };
        }
    }
    result
}

/// One segment against another: numbers by value, words by their rank (`dev` < `alpha` < `beta` < `RC` < a number <
/// `pl`), anything else below `dev`.
fn segment(s1: &[u8], s2: &[u8]) -> i32 {
    let digit = |s: &[u8]| s.first().is_some_and(u8::is_ascii_digit);
    let number = |s: &[u8]| -> i64 {
        s.iter()
            .take_while(|c| c.is_ascii_digit())
            .fold(0i64, |n, &c| {
                n.saturating_mul(10).saturating_add(i64::from(c - b'0'))
            })
    };
    match (digit(s1), digit(s2)) {
        (true, true) => number(s1).cmp(&number(s2)) as i32,
        (false, false) => rank(s1).cmp(&rank(s2)) as i32,
        (true, false) => rank(b"#N#").cmp(&rank(s2)) as i32,
        (false, true) => rank(s1).cmp(&rank(b"#N#")) as i32,
    }
}

fn rank(form: &[u8]) -> i32 {
    const FORMS: [(&[u8], i32); 10] = [
        (b"dev", 0),
        (b"alpha", 1),
        (b"a", 1),
        (b"beta", 2),
        (b"b", 2),
        (b"RC", 3),
        (b"rc", 3),
        (b"#", 4),
        (b"pl", 5),
        (b"p", 5),
    ];
    FORMS
        .iter()
        .find(|(name, _)| form.starts_with(name))
        .map_or(-1, |f| f.1)
}

/// `php_canonicalize_version`: `-`, `_`, `+` and other punctuation become `.`, and a `.` goes between a digit and a
/// letter.
fn canonical(v: &str) -> String {
    let b = v.as_bytes();
    let mut out = vec![b[0]];
    let mut last = b[0];
    let digit = |c: u8| c.is_ascii_digit() && c != b'.';
    let other = |c: u8| !c.is_ascii_digit() && c != b'.';
    for &c in &b[1..] {
        let dot = |out: &mut Vec<u8>| {
            if out.last() != Some(&b'.') {
                out.push(b'.');
            }
        };
        if matches!(c, b'-' | b'_' | b'+') {
            dot(&mut out);
        } else if (other(last) && digit(c)) || (digit(last) && other(c)) {
            dot(&mut out);
            out.push(c);
        } else if !c.is_ascii_alphanumeric() {
            dot(&mut out);
        } else {
            out.push(c);
        }
        last = c;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// The lowest version a constraint admits: (normalized version, inclusive), as `getLowerBound()` of the parsed
/// constraint gives it. `0.0.0.0-dev` inclusive is no bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {
    pub version: String,
    pub inclusive: bool,
}

impl Bound {
    pub fn zero() -> Bound {
        Bound {
            version: "0.0.0.0-dev".into(),
            inclusive: true,
        }
    }

    pub fn is_zero(&self) -> bool {
        *self == Bound::zero()
    }

    /// `Bound::compareTo($other, '>')`: higher version, or the same version where only `other` includes it.
    pub fn above(&self, other: &Bound) -> bool {
        if self == other {
            return false;
        }
        match compare(&self.version, &other.version) {
            0 => other.inclusive,
            c => c == 1,
        }
    }
}

static TILDE: LazyLock<Regex> = LazyLock::new(|| re(&format!("(?i)^~>?{}$", version_regex())));
static CARET: LazyLock<Regex> = LazyLock::new(|| re(&format!("(?i)^\\^{}($)", version_regex())));
static WILDCARD: LazyLock<Regex> =
    LazyLock::new(|| re(r"^v?(\d++)(?:\.(\d++))?(?:\.(\d++))?(?:\.[xX*])++$"));
static HYPHEN: LazyLock<Regex> = LazyLock::new(|| {
    let v = version_regex();
    re(&format!("(?i)^(?P<from>{v}) +- +(?P<to>{v})($)"))
});
static OPERATOR: LazyLock<Regex> = LazyLock::new(|| re(r"^(<>|!=|>=?|<=?|==?)?\s*(.*)"));
static STABILITY_FLAG: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?i)^([^,\s]*?)@(stable|RC|beta|alpha|dev)$"));
static ANY: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)^(v)?[xX*](\.[xX*])*$"));
static OR: LazyLock<Regex> = LazyLock::new(|| re(r"\s*\|\|?\s*"));
static AND: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?<!^|as|[=>< ,]) *(?<!-)[, ](?!-) *(?!,|as|$)"));
static MODIFIER_END: LazyLock<Regex> = LazyLock::new(|| re(&format!("(?i)-{MODIFIER}$")));

fn version_regex() -> String {
    format!(
        r"v?(\d++)(?:\.(\d++))?(?:\.(\d++))?(?:\.(\d++))?(?:{MODIFIER}|\.([xX*][.-]?dev))(?:\+[^\s]+)?"
    )
}

fn split<'a>(re: &Regex, s: &'a str) -> Vec<&'a str> {
    let mut out = vec![];
    let mut start = 0;
    for m in re.find_iter(s.as_bytes()).flatten() {
        out.push(&s[start..m.start()]);
        start = m.end();
    }
    out.push(&s[start..]);
    out
}

/// The lower bound of a version constraint (`^8.4`, `>=8.1 <9`, `^7.2 || ^8.0`): of an AND the highest, of an OR the
/// lowest.
pub fn lower_bound(constraint: &str) -> Result<Bound> {
    let mut low: Option<Bound> = None;
    for alternative in split(&OR, constraint.trim()) {
        let mut high: Option<Bound> = None;
        for part in split(&AND, alternative) {
            let b = atom_bound(part)?;
            high = Some(match high {
                Some(h) if !b.above(&h) => h,
                _ => b,
            });
        }
        let b = high.unwrap_or_else(Bound::zero);
        low = Some(match low {
            Some(l) if !l.above(&b) => l,
            _ => b,
        });
    }
    Ok(low.unwrap_or_else(Bound::zero))
}

/// The lower bound of one constraint without `,`, ` ` or `|`.
fn atom_bound(constraint: &str) -> Result<Bound> {
    let ge = |version: String| Bound {
        version,
        inclusive: true,
    };
    let mut c = constraint.to_string();
    if let Some(m) = ALIAS.captures(c.as_bytes())? {
        c = group(&m, 1).to_string();
    }
    let mut flag = String::new();
    if let Some(m) = STABILITY_FLAG.captures(c.as_bytes())? {
        flag = if group(&m, 2).eq_ignore_ascii_case("stable") {
            String::new()
        } else {
            group(&m, 2).to_string()
        };
        c = match group(&m, 1) {
            "" => "*".to_string(),
            v => v.to_string(),
        };
    }
    if let Some(m) = ANY.captures(c.as_bytes())? {
        let _ = m;
        return Ok(Bound::zero());
    }
    let dev_suffix = |m: &Captures, a: usize, b: usize, d: usize| {
        if group(m, a).is_empty() && group(m, b).is_empty() && group(m, d).is_empty() {
            "-dev"
        } else {
            ""
        }
    };
    if let Some(m) = TILDE.captures(c.as_bytes())? {
        if c.starts_with("~>") {
            bail!(
                "Could not parse version constraint {c}: Invalid operator \"~>\", you probably meant to use the \"~\" operator"
            );
        }
        return Ok(ge(normalize(&format!(
            "{}{}",
            &c[1..],
            dev_suffix(&m, 5, 7, 8)
        ))?));
    }
    if let Some(m) = CARET.captures(c.as_bytes())? {
        return Ok(ge(normalize(&format!(
            "{}{}",
            &c[1..],
            dev_suffix(&m, 5, 7, 8)
        ))?));
    }
    if let Some(m) = WILDCARD.captures(c.as_bytes())? {
        let parts: Vec<&str> = (1..4).map(|i| group(&m, i)).collect();
        let position = parts
            .iter()
            .rposition(|p| !p.is_empty())
            .map_or(1, |p| p + 1);
        let low: Vec<&str> = (0..4)
            .map(|i| if i < position { parts[i] } else { "0" })
            .collect();
        return Ok(ge(format!("{}-dev", low.join("."))));
    }
    if let Some(m) = HYPHEN.captures(c.as_bytes())? {
        let from = std::str::from_utf8(m.name("from").unwrap().as_bytes())?;
        return Ok(ge(format!(
            "{}{}",
            normalize(from)?,
            dev_suffix(&m, 6, 8, 9)
        )));
    }
    let m = OPERATOR
        .captures(c.as_bytes())?
        .expect("matches any string");
    let (op, rest) = (group(&m, 1), group(&m, 2));
    let mut version = match normalize(rest) {
        Ok(v) => v,
        Err(_)
            if rest.ends_with("-dev")
                && rest
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"-./".contains(&b)) =>
        {
            normalize(&format!("dev-{}", &rest[..rest.len() - 4]))?
        }
        Err(e) => bail!("Could not parse version constraint {c}: {e}"),
    };
    if version.starts_with("dev-") {
        return Ok(Bound::zero());
    }
    if !matches!(op, "==" | "=" | "") && !flag.is_empty() && !version.contains('-') {
        version = format!("{version}-{flag}");
    } else if op == ">=" && !MODIFIER_END.is_match(rest.as_bytes())? && !rest.starts_with("dev-") {
        version.push_str("-dev");
    }
    Ok(match op {
        "" | "=" | "==" | ">=" => ge(version),
        ">" => Bound {
            version,
            inclusive: false,
        },
        _ => Bound::zero(),
    })
}

/// PHP's `version_compare` as an [`Ordering`].
pub fn ordering(a: &str, b: &str) -> Ordering {
    compare(a, b).cmp(&0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// composer/semver 3.6.0's `VersionParserTest::successfulNormalizedVersions`, a selection covering every branch.
    #[test]
    fn normalize_matches_composer() {
        for (input, want) in [
            ("1.0.0", "1.0.0.0"),
            ("1.2.3.4", "1.2.3.4"),
            ("1.0.0RC1dev", "1.0.0.0-RC1-dev"),
            ("1.0.0-rC15-dev", "1.0.0.0-RC15-dev"),
            ("1.0.0.RC.15-dev", "1.0.0.0-RC15-dev"),
            ("1.0.0.pl3-dev", "1.0.0.0-patch3-dev"),
            ("1.0-dev", "1.0.0.0-dev"),
            ("10.4.13-b5", "10.4.13.0-beta5"),
            ("v1.13.11-beta.0", "1.13.11.0-beta0"),
            ("v1.0.0", "1.0.0.0"),
            ("2010.01.02", "2010.01.02.0"),
            ("20100102-203040", "20100102.203040"),
            ("2010-01-02-10-20-30.5", "2010.01.02.10.20.30.5"),
            ("20100102.x-dev", "20100102.9999999.9999999.9999999-dev"),
            ("master", "dev-master"),
            ("DEV-FOOBAR", "dev-FOOBAR"),
            ("1.x-dev", "1.9999999.9999999.9999999-dev"),
            ("dev-master as 1.0.0", "dev-master"),
            ("1.0.0+foo@dev", "1.0.0.0"),
            ("1.0.0-alpha.3.1+foo", "1.0.0.0-alpha3.1"),
            ("1.0.0-stable", "1.0.0.0"),
            ("041.x-dev", "041.9999999.9999999.9999999-dev"),
            (" 1.0.0", "1.0.0.0"),
            ("1.0.0a1", "1.0.0.0-alpha1"),
        ] {
            assert_eq!(normalize(input).unwrap(), want, "{input}");
        }
        for bad in [
            "",
            "a",
            "1.0.0-meh",
            "1.0.0.0.0",
            "feature-foo",
            "1.0.1-SNAPSHOT",
            "foo bar-dev",
            "^1",
        ] {
            assert!(normalize(bad).is_err(), "{bad}");
        }
        assert_eq!(
            normalize("1.0.0-meh").unwrap_err().to_string(),
            "Invalid version string \"1.0.0-meh\""
        );
    }

    /// Expected values from Composer 2.11.2's `ArrayLoader::getBranchAlias` on the same package arrays.
    #[test]
    fn branch_aliases_match_composer() {
        let alias = |v: serde_json::Value| branch_alias(&v);
        let pair = |a: &str, b: &str| Some((a.to_string(), b.to_string()));
        assert_eq!(
            alias(
                json!({"version": "dev-main", "extra": {"branch-alias": {"dev-main": "2.x-dev"}}})
            ),
            pair("2.9999999.9999999.9999999-dev", "2.x-dev")
        );
        assert_eq!(
            alias(json!({"version": "dev-main", "default-branch": true})),
            pair("9999999-dev", "9999999-dev")
        );
        assert_eq!(
            alias(json!({"version": "5.x-dev", "default-branch": true})),
            None
        );
        assert_eq!(
            alias(json!({"version": "1.0.0", "extra": {"branch-alias": {"1.0.0": "1.x-dev"}}})),
            None
        );
        // A numeric alias must extend its branch, and the target must be a -dev version.
        assert_eq!(
            alias(
                json!({"version": "2.x-dev", "extra": {"branch-alias": {"2.x-dev": "3.0.x-dev"}}})
            ),
            None
        );
        assert_eq!(
            alias(json!({"version": "dev-a", "extra": {"branch-alias": {"dev-a": "1.0", "x": 1}}})),
            None
        );
        assert_eq!(
            alias(json!({"version": "dev-a", "extra": {"branch-alias": {"dev-a": "9999999-dev"}}})),
            pair("9999999-dev", "9999999-dev")
        );
    }

    /// Expected values from PHP 8.5's `version_compare`.
    #[test]
    fn compare_matches_php() {
        for (a, b, want) in [
            ("8.4.0.0-dev", "8.4.0.0", -1),
            ("8.4.1.0-dev", "8.4.0.0", 1),
            ("1.0", "1.0.0", -1),
            ("1.0.0", "1.0", 1),
            ("1.0-dev", "1.0", -1),
            ("1.0", "1.0-dev", 1),
            ("1.0rc1", "1.0beta2", 1),
            ("1.0pl1", "1.0", 1),
            ("1.0.x", "1.0.dev", -1),
            ("1_0", "1-0", 0),
            ("", "1", -1),
            ("1", "", 1),
            ("", "", 0),
            ("1.0.1", "1.0.rc", 1),
            ("1.0.rc", "1.0.1", -1),
        ] {
            assert_eq!(compare(a, b), want, "{a} {b}");
        }
        assert_eq!(ordering("2", "10"), Ordering::Less);
    }

    /// Expected values from `getLowerBound()` of composer/semver 3.6.0's `parseConstraints`.
    #[test]
    fn lower_bounds_match_composer() {
        for (c, version, inclusive) in [
            ("^8.4", "8.4.0.0-dev", true),
            ("^8.4.1", "8.4.1.0-dev", true),
            ("~8.1.2", "8.1.2.0-dev", true),
            (">=8.1", "8.1.0.0-dev", true),
            (">=8.1-beta1", "8.1.0.0-beta1", true),
            (">8.1", "8.1.0.0", false),
            ("8.2.*", "8.2.0.0-dev", true),
            ("8.2.1", "8.2.1.0", true),
            ("^7.2 || ^8.0", "7.2.0.0-dev", true),
            (">=7.4 <9 , >=8.1", "8.1.0.0-dev", true),
            ("8.1 - 8.3", "8.1.0.0-dev", true),
            ("^8.1@beta", "8.1.0.0-dev", true),
            (">8.1@beta", "8.1.0.0-beta", false),
            ("*", "0.0.0.0-dev", true),
            ("<9", "0.0.0.0-dev", true),
            ("dev-main", "0.0.0.0-dev", true),
            ("5.x-dev", "5.9999999.9999999.9999999-dev", true),
            ("1.0 as 2.0", "1.0.0.0", true),
            ("@dev", "0.0.0.0-dev", true),
            (">=1.0@stable", "1.0.0.0-dev", true),
            ("~1.2-beta", "1.2.0.0-beta", true),
            ("^1.2-beta", "1.2.0.0-beta", true),
            ("1.2 - 1.3-beta", "1.2.0.0-dev", true),
            (">=feature/x-dev", "0.0.0.0-dev", true),
            (">=my.feature-dev", "0.0.0.0-dev", true),
        ] {
            assert_eq!(
                lower_bound(c).unwrap(),
                Bound {
                    version: version.into(),
                    inclusive
                },
                "{c}"
            );
        }
        assert!(lower_bound("~>1.2").is_err());
        assert!(lower_bound(">=foo").is_err());
        assert!(Bound::zero().is_zero());
        assert!(
            !lower_bound(">8.1")
                .unwrap()
                .above(&lower_bound(">8.1").unwrap())
        );
        assert!(
            lower_bound(">8.1")
                .unwrap()
                .above(&lower_bound("8.1").unwrap())
        );
    }

    #[test]
    fn platform_names() {
        assert!(
            is_platform("php") && is_platform("ext-mbstring") && is_platform("composer-plugin-api")
        );
        assert!(!is_platform("laravel/framework") && !is_platform("ext-"));
    }
}
