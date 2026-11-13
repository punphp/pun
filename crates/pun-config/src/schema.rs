//! The `[files]`, `[lint]` and `[analyse]` parts of pun.toml (context/plan/config.md §3 and §5): what pun.toml says,
//! resolved against rules/mapping.toml and what composer.lock locks into the rules each command runs.

use crate::composer::{Composer, Version};
use crate::{fixer_of, options, renamed, rules};
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

/// The lint sets (`[lint] sets`), Rector's prepared sets.
pub const LINT_SETS: [&str; 8] = [
    "dead-code",
    "modernize",
    "code-quality",
    "types",
    "early-return",
    "privatization",
    "naming",
    "coding-style",
];

/// The format presets pun.toml and `--preset` take (Pint's `laravel`, `psr12` and `empty`; `per` and `symfony` are
/// only `pun migrate` input). Reads as its name (`&*preset`, `preset == "laravel"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatPreset {
    Laravel,
    Psr12,
    Empty,
}

impl FormatPreset {
    pub const ALL: [Self; 3] = [Self::Laravel, Self::Psr12, Self::Empty];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Laravel => "laravel",
            Self::Psr12 => "psr12",
            Self::Empty => "empty",
        }
    }

    /// The preset of a name, `None` for any other.
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.name() == s)
    }
}

impl std::ops::Deref for FormatPreset {
    type Target = str;

    fn deref(&self) -> &str {
        self.name()
    }
}

impl PartialEq<&str> for FormatPreset {
    fn eq(&self, other: &&str) -> bool {
        self.name() == *other
    }
}

impl std::fmt::Display for FormatPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl Serialize for FormatPreset {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.name())
    }
}

/// How safe a rule's fix is (mapping.toml's `fix`): `mixed` has safe and unsafe parts, `none` is report-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixSafety {
    Safe,
    Unsafe,
    Mixed,
    None,
}

impl FixSafety {
    pub(crate) fn parse(s: &str) -> Self {
        match s {
            "safe" => Self::Safe,
            "unsafe" => Self::Unsafe,
            "mixed" => Self::Mixed,
            _ => Self::None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Unsafe => "unsafe",
            Self::Mixed => "mixed",
            Self::None => "none",
        }
    }
}

/// The analyse presets, each including the ones before it (their order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AnalysePreset {
    Basic,
    Essential,
    Recommended,
    Strict,
    Max,
}

impl AnalysePreset {
    pub const ALL: [Self; 5] = [
        Self::Basic,
        Self::Essential,
        Self::Recommended,
        Self::Strict,
        Self::Max,
    ];

    /// The name pun.toml writes.
    pub fn name(self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Essential => "essential",
            Self::Recommended => "recommended",
            Self::Strict => "strict",
            Self::Max => "max",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.name() == s)
    }

    /// The preset that holds a PHPStan level (0 to 10).
    pub fn of_level(level: usize) -> Self {
        Self::ALL[[0, 0, 1, 1, 2, 2, 3, 3, 3, 4, 4][level]]
    }
}

impl std::fmt::Display for AnalysePreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// The type switches each analyse preset turns on, cumulative (PHPStan levels 7-10; mapping.toml `preset_switches`).
const SWITCHES: [(AnalysePreset, &str); 4] = [
    (AnalysePreset::Strict, "union-types"),
    (AnalysePreset::Strict, "nullables"),
    (AnalysePreset::Max, "explicit-mixed"),
    (AnalysePreset::Max, "implicit-mixed"),
];

/// An ecosystem group: its rules run when composer.lock has the package. Nothing detects `doctrine` yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Ecosystem {
    Laravel,
    Pest,
    Phpunit,
    Doctrine,
}

impl Ecosystem {
    const ALL: [Self; 4] = [Self::Laravel, Self::Pest, Self::Phpunit, Self::Doctrine];

    /// The group name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Laravel => "laravel",
            Self::Pest => "pest",
            Self::Phpunit => "phpunit",
            Self::Doctrine => "doctrine",
        }
    }
}

/// `[files]`: what every command checks.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Files {
    pub paths: Vec<String>,
    pub exclude: Vec<String>,
}

/// A lint level. A rule that is `"off"` is not in the resolved list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Warn,
    Error,
}

/// One enabled lint rule.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LintRule {
    /// `lint/<group>/<name>`.
    pub id: String,
    pub level: Level,
    /// `false` for `fix = "off"`: the rule only reports.
    pub fix: bool,
    /// Options from pun.toml (as the PHP-CS-Fixer fixer takes them when one runs the rule), `Null` for none.
    pub options: Value,
    /// The PHP-CS-Fixer fixer that runs it in the token pass, if any.
    #[serde(skip)]
    pub fixer: Option<&'static str>,
    /// Named in `[lint.rules]`, not only enabled by a set or a group. A named rule Pun can't run stops the run; a set
    /// skips such rules.
    #[serde(skip)]
    pub named: bool,
}

impl LintRule {
    /// The rule as pun.toml writes it: its level, or a table when it has options or `fix = "off"` (with `level` only
    /// when it is `warn`, the default being `error`).
    pub fn value(&self) -> Value {
        lint_value(&self.id, Some(self.level), self.fix, &self.options)
    }
}

impl LintSetting {
    /// The line as pun.toml writes it, `"off"` included.
    pub fn value(&self) -> Value {
        lint_value(&self.id, self.level, self.fix, &self.options)
    }
}

fn lint_value(id: &str, level: Option<Level>, fix: bool, options: &Value) -> Value {
    let level = Value::from(match level {
        None => "off",
        Some(Level::Warn) => "warn",
        Some(Level::Error) => "error",
    });
    if fix && options.is_null() {
        return level;
    }
    let mut t = serde_json::Map::new();
    if level != "error" {
        t.insert("level".into(), level);
    }
    if !fix {
        t.insert("fix".into(), "off".into());
    }
    // A fixer's options came through `to_fixer`, so they translate back.
    let options = match fixer_of("lint", id) {
        Some(f) if options.is_object() => options::to_pun(f, id, options).unwrap(),
        _ => options.clone(),
    };
    if let Value::Object(o) = options {
        t.extend(o);
    }
    Value::Object(t)
}

/// A rule's line in `[lint.rules]` or an override: `level` is `None` for `"off"`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LintSetting {
    pub id: String,
    pub level: Option<Level>,
    pub fix: bool,
    pub options: Value,
}

/// One `[[lint.overrides]]`: rule levels for the files its globs match.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Override {
    pub files: Vec<String>,
    pub rules: Vec<LintSetting>,
}

/// The resolved `[lint]`.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Lint {
    pub sets: Vec<String>,
    /// `[files] exclude` plus `[lint] exclude`.
    pub exclude: Vec<String>,
    /// The ecosystems the lint rules run for: detected in composer.lock and not switched off.
    pub ecosystems: Vec<Ecosystem>,
    /// Every enabled rule, by ID.
    pub rules: Vec<LintRule>,
    pub overrides: Vec<Override>,
}

/// One enabled analyse rule.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseRule {
    /// `analyse/<group>/<name>`.
    pub id: String,
    /// Options from pun.toml, `Null` for none.
    pub options: Value,
}

/// The resolved `[analyse]`.
#[derive(Debug, Clone, PartialEq)]
pub struct Analyse {
    pub preset: AnalysePreset,
    /// `[analyse] paths`, or `[files] paths` when it has none.
    pub paths: Vec<String>,
    /// `[files] exclude` plus `[analyse] exclude`.
    pub exclude: Vec<String>,
    pub baseline: Option<String>,
    /// `[analyse] stubs`: PHPStan's `stubFiles`, relative to the project.
    pub stubs: Vec<String>,
    /// `[[analyse.ignore]]`: PHPStan's `ignoreErrors`, in pun.toml order.
    pub ignore: Vec<Ignore>,
    /// The type switches the preset turns on.
    pub switches: Vec<&'static str>,
    pub ecosystems: Vec<Ecosystem>,
    /// Larastan's knowledge and rules: the `laravel` ecosystem is on and composer.lock pins a Laravel that Larastan
    /// supports.
    pub larastan: bool,
    /// Every enabled rule, by ID.
    pub rules: Vec<AnalyseRule>,
    /// `[analyse] bleeding-edge`: PHPStan's bleeding-edge type inference (`bleedingEdge.neon`'s inference toggles).
    pub bleeding_edge: bool,
}

/// One `[[analyse.ignore]]`: an `ignoreErrors` entry of PHPStan's under a Pun rule. An error is ignored when it
/// matches every key given: its rule, its PHPStan identifier, its message (`message` a PCRE pattern as in PHPStan,
/// `raw-message` the exact text) and one of `paths` (fnmatch patterns or paths, relative to the project). `count`
/// ignores exactly that many and reports the rest, as in PHPStan.
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Ignore {
    /// `analyse/<group>/<name>` once resolved; pun.toml may leave out `analyse/`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct FilesToml {
    #[serde(default)]
    paths: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintToml {
    #[serde(default)]
    sets: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default)]
    groups: toml::Table,
    #[serde(default)]
    rules: toml::Table,
    #[serde(default)]
    overrides: Vec<OverrideToml>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OverrideToml {
    files: Vec<String>,
    #[serde(default)]
    rules: toml::Table,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct AnalyseToml {
    preset: Option<toml::Value>,
    paths: Option<Vec<String>>,
    #[serde(default)]
    exclude: Vec<String>,
    baseline: Option<String>,
    #[serde(default)]
    stubs: Vec<String>,
    #[serde(default)]
    ignore: Vec<Ignore>,
    #[serde(default)]
    rules: toml::Table,
    #[serde(default, rename = "bleeding-edge")]
    bleeding_edge: bool,
    /// PHPStan's `level`: an error naming the preset.
    level: Option<toml::Value>,
}

impl Files {
    pub(crate) fn from(t: FilesToml) -> Files {
        Files {
            paths: t.paths,
            exclude: t.exclude,
        }
    }
}

/// A PHP version as `major.minor`: `[php] version`, or the lowest composer.json's `require.php` admits. The one
/// representation of the target PHP version: `to_id()` is PHP's `PHP_VERSION_ID` form that pun-lint and
/// pun-analyse compare with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhpVersion {
    pub major: u8,
    pub minor: u8,
}

impl PhpVersion {
    /// Pun's floor: the version a project without `[php] version` or `require.php` targets.
    pub const FLOOR: PhpVersion = PhpVersion { major: 8, minor: 4 };
    /// The newest PHP release: what PHPStan reads as its runtime version when nothing else names one.
    pub const NEWEST: PhpVersion = PhpVersion { major: 8, minor: 5 };

    /// `PHP_VERSION_ID` of the `.0` release: 8.4 is 80400.
    pub fn to_id(self) -> u32 {
        u32::from(self.major) * 10_000 + u32::from(self.minor) * 100
    }
}

impl std::str::FromStr for PhpVersion {
    type Err = String;

    /// `8.4`: two parts, each a number.
    fn from_str(s: &str) -> Result<Self, String> {
        let bad = || format!("expected a PHP version as major.minor, such as \"8.4\"; got \"{s}\"");
        let (major, minor) = s.split_once('.').ok_or_else(bad)?;
        Ok(PhpVersion {
            major: major.parse().map_err(|_| bad())?,
            minor: minor.parse().map_err(|_| bad())?,
        })
    }
}

impl std::fmt::Display for PhpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl Serialize for PhpVersion {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

/// What gates the rules of ecosystem and upgrade groups: the PHP version and the ecosystems that are on.
pub(crate) struct Gates<'a> {
    pub php: PhpVersion,
    pub composer: &'a Composer,
}

impl Gates<'_> {
    /// The locked version of an ecosystem's package composer.lock has, `None` when it has none.
    fn locked(&self, ecosystem: Ecosystem) -> Option<Option<Version>> {
        let l = match ecosystem {
            Ecosystem::Laravel => &self.composer.laravel,
            Ecosystem::Pest => &self.composer.pest,
            Ecosystem::Phpunit => &self.composer.phpunit,
            Ecosystem::Doctrine => return None,
        };
        l.as_ref().map(|l| l.semver)
    }

    /// The ecosystems composer.lock has, minus those `off` switches off (by group name).
    pub(crate) fn ecosystems(&self, off: impl Fn(&str) -> bool) -> Vec<Ecosystem> {
        (Ecosystem::ALL.into_iter())
            .filter(|e| self.locked(*e).is_some() && !off(e.name()))
            .collect()
    }

    /// Whether a rule of `group` runs when only a set or a preset turns it on: an ecosystem group needs its ecosystem
    /// on, an upgrade group (`php-8.4`, `laravel-11`) needs that PHP version or that locked Laravel version.
    pub(crate) fn allows(&self, group: &str, on: &[Ecosystem]) -> bool {
        if let Some(v) = group.strip_prefix("php-") {
            return v.parse().is_ok_and(|v: PhpVersion| self.php >= v);
        }
        if let Some(v) = group.strip_prefix("laravel-") {
            let locked = self.locked(Ecosystem::Laravel).flatten();
            return on.contains(&Ecosystem::Laravel)
                && version(v)
                    .zip(locked)
                    .is_some_and(|(v, l)| (l.major, l.minor) >= v);
        }
        if let Some(e) = Ecosystem::ALL.into_iter().find(|e| e.name() == group) {
            return on.contains(&e);
        }
        true
    }
}

/// `8.4` or `13` as `(major, minor)`.
pub(crate) fn version(v: &str) -> Option<(u64, u64)> {
    let (major, minor) = v.split_once('.').unwrap_or((v, "0"));
    Some((major.parse().ok()?, minor.parse().ok()?))
}

/// The group list of mapping.toml (`[meta.groups]`), in its order.
pub fn groups() -> &'static Vec<String> {
    static T: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
        (crate::mapping::section(&["meta", "groups"]).table().keys())
            .cloned()
            .collect()
    });
    &T
}

/// The group of a rule ID (`lint/dead-code/x` -> `dead-code`).
fn group_of(id: &str) -> &str {
    id.split('/').nth(1).unwrap()
}

/// The full ID of `name` under `[<command>.rules]`: a full ID or `<group>/<name>`. Anything else is an error, naming
/// the ID to use when the name is an old or upstream one.
pub(crate) fn rule_id(path: &Path, command: &str, name: &str) -> Result<String> {
    let id = match name.strip_prefix(command).and_then(|r| r.strip_prefix('/')) {
        Some(_) => name.to_string(),
        None => format!("{command}/{name}"),
    };
    // The token fixers' rules are found without parsing the whole mapping.
    if fixer_of(command, name).is_some() || rules().iter().any(|(r, _)| *r == id) {
        return Ok(id);
    }
    let other = if command == "format" {
        "lint"
    } else {
        "format"
    };
    let found = (fixer_of(other, name).and_then(crate::rule_of)).or_else(|| renamed(name));
    match found {
        Some(id) => bail!("{}: `{name}` is `{id}` in pun.toml", path.display()),
        None => {
            let a = if command == "analyse" { "an" } else { "a" };
            bail!("{}: `{name}` is not {a} {command} rule", path.display())
        }
    }
}

/// A level string of `[lint.groups]`, `[lint.rules]` or an override.
fn level(path: &Path, key: &str, s: &str) -> Result<Option<Level>> {
    Ok(match s {
        "off" => None,
        "warn" => Some(Level::Warn),
        "error" => Some(Level::Error),
        "info" => bail!(
            "{}: `{key}` = \"info\": the `info` level was removed; use \"warn\"",
            path.display()
        ),
        _ => bail!(
            "{}: `{key}` = \"{s}\": expected \"error\", \"warn\" or \"off\"",
            path.display()
        ),
    })
}

/// A lint rule's line: a level, or a table with `level`, `fix = "off"` and the rule's options.
fn setting(path: &Path, name: &str, v: toml::Value) -> Result<LintSetting> {
    let id = rule_id(path, "lint", name)?;
    let shown = |v: &toml::Value| v.to_string();
    let (level, fix, opts) = match v {
        toml::Value::String(s) => (level(path, name, &s)?, true, toml::Table::new()),
        toml::Value::Boolean(b) => bail!(
            "{}: `{name}` = {b}: lint rules take a level; use \"{}\"",
            path.display(),
            if b { "error" } else { "off" }
        ),
        toml::Value::Table(mut t) => {
            let level = match t.remove("level") {
                None => Some(Level::Error),
                Some(toml::Value::String(s)) => level(path, name, &s)?,
                Some(v) => bail!(
                    "{}: `{name}`: level = {}: expected \"error\", \"warn\" or \"off\"",
                    path.display(),
                    shown(&v)
                ),
            };
            if t.contains_key("severity") {
                bail!(
                    "{}: `{name}`: `severity` was removed; use `level` (\"error\", \"warn\" or \"off\")",
                    path.display()
                );
            }
            let fix = match t.remove("fix") {
                None => true,
                Some(f) => match f.as_str() {
                    Some("off") => false,
                    Some("apply") => bail!(
                        "{}: `{name}`: fix = \"apply\" was removed; `--fix` applies the fix of every enabled rule, so leave `fix` out",
                        path.display()
                    ),
                    Some("safe-only") => bail!(
                        "{}: `{name}`: fix = \"safe-only\" was removed; `--fix` applies every fix, and fix = \"off\" keeps a rule report-only",
                        path.display()
                    ),
                    _ => bail!(
                        "{}: `{name}`: fix = {}: the only value is \"off\"",
                        path.display(),
                        shown(&f)
                    ),
                },
            };
            (level, fix, t)
        }
        v => bail!(
            "{}: `{name}` = {}: expected \"error\", \"warn\", \"off\" or a table",
            path.display(),
            shown(&v)
        ),
    };
    let options = if opts.is_empty() {
        Value::Null
    } else if fixer_of("lint", &id).is_some() {
        // The fixer's options under their Pun names (`[options."lint/..."]` of mapping.toml).
        options::to_fixer(name, &id, toml::Value::Table(opts))
            .map_err(|e| anyhow::anyhow!("{}: {e}", path.display()))?
    } else {
        serde_json::to_value(opts).unwrap()
    };
    Ok(LintSetting {
        id,
        level,
        fix,
        options,
    })
}

/// `[lint]`: sets turn rules on at `error`, a group's level wins over a set's, a rule's own line over its group's.
pub(crate) fn lint(path: &Path, t: LintToml, files: &Files, gates: &Gates) -> Result<Lint> {
    for s in &t.sets {
        if !LINT_SETS.contains(&s.as_str()) {
            bail!(
                "{}: [lint] sets: `{s}` is not a lint set; the sets are {}",
                path.display(),
                list(&LINT_SETS)
            );
        }
    }
    let mut groups = BTreeMap::new();
    for (g, v) in t.groups {
        if !groups_contains(&g) {
            bail!(
                "{}: [lint.groups]: `{g}` is not a group (see [meta.groups] in crates/pun-config/rules/mapping.toml)",
                path.display()
            );
        }
        let toml::Value::String(s) = v else {
            bail!(
                "{}: [lint.groups] `{g}` = {v}: expected \"error\", \"warn\" or \"off\"",
                path.display()
            );
        };
        groups.insert(g.clone(), level(path, &g, &s)?);
    }
    let named = (t.rules.into_iter())
        .map(|(name, v)| setting(path, &name, v))
        .collect::<Result<Vec<_>>>()?;
    let overrides = (t.overrides.into_iter())
        .map(|o| {
            Ok(Override {
                files: o.files,
                rules: (o.rules.into_iter())
                    .map(|(name, v)| setting(path, &name, v))
                    .collect::<Result<_>>()?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let ecosystems = gates.ecosystems(|e| groups.get(e) == Some(&None));
    // Sets and groups need every rule's data; a pun.toml with neither only runs what it names.
    let rule = |id: String, level, fix, options, named| LintRule {
        fixer: fixer_of("lint", &id),
        id,
        level,
        fix,
        options,
        named,
    };
    let mut on: BTreeMap<String, LintRule> = BTreeMap::new();
    if !t.sets.is_empty() || !groups.is_empty() {
        // a rule without a native port (`ported = false`, one upstream added since the last port) is in no set
        // or group Pun runs
        let ported =
            |e: &toml::Value| e.get("ported").and_then(toml::Value::as_bool) != Some(false);
        for (id, e) in rules()
            .iter()
            .filter(|(id, e)| id.starts_with("lint/") && ported(e))
        {
            let group = group_of(id);
            let level = match groups.get(group) {
                Some(l) => *l,
                None => {
                    let in_set = (e.get("sets").and_then(toml::Value::as_array)).is_some_and(|s| {
                        s.iter()
                            .any(|s| t.sets.iter().any(|t| s.as_str() == Some(t)))
                    });
                    (in_set && gates.allows(group, &ecosystems)).then_some(Level::Error)
                }
            };
            if let Some(level) = level {
                on.insert(
                    id.clone(),
                    rule(id.clone(), level, true, Value::Null, false),
                );
            }
        }
    }
    for s in named {
        match s.level {
            Some(level) => drop(on.insert(s.id.clone(), rule(s.id, level, s.fix, s.options, true))),
            None => drop(on.remove(&s.id)),
        }
    }
    Ok(Lint {
        sets: t.sets,
        exclude: [&files.exclude[..], &t.exclude[..]].concat(),
        ecosystems,
        rules: on.into_values().collect(),
        overrides,
    })
}

fn groups_contains(g: &str) -> bool {
    groups().iter().any(|x| x == g)
}

fn list(items: &[&str]) -> String {
    let quoted: Vec<String> = items.iter().map(|s| format!("`{s}`")).collect();
    quoted.join(", ")
}

/// `[analyse]`: a preset turns on its rules and those of the presets below it; a group's or a rule's line turns
/// them on (`"on"` or a table of options) or off.
pub(crate) fn analyse(
    path: &Path,
    t: AnalyseToml,
    files: &Files,
    gates: &Gates,
) -> Result<Analyse> {
    if let Some(level) = t.level {
        let hint = level
            .as_integer()
            .and_then(|l| usize::try_from(l).ok())
            .filter(|l| *l <= 10)
            .map(|l| format!(": level {l} is preset = \"{}\"", AnalysePreset::of_level(l)))
            .unwrap_or_default();
        let presets: Vec<String> = (AnalysePreset::ALL.iter())
            .map(|p| format!("\"{p}\""))
            .collect();
        bail!(
            "{}: [analyse] level: PHPStan's levels are presets in Pun; use preset = {}{hint}",
            path.display(),
            presets.join(" | ")
        );
    }
    let preset = match t.preset {
        None => AnalysePreset::Recommended,
        Some(v) if let Some(p) = v.as_str().and_then(AnalysePreset::parse) => p,
        Some(toml::Value::Integer(l)) if (0..=10).contains(&l) => bail!(
            "{}: [analyse] preset = {l}: PHPStan level {l} is preset = \"{}\"",
            path.display(),
            AnalysePreset::of_level(l as usize)
        ),
        Some(v) => bail!(
            "{}: [analyse] preset = {v}: expected {}",
            path.display(),
            list(&AnalysePreset::ALL.map(AnalysePreset::name))
        ),
    };

    let mut groups = BTreeMap::new();
    let mut named = vec![];
    for (name, v) in t.rules {
        let on = match v {
            toml::Value::String(s) if s == "on" => Some(Value::Null),
            toml::Value::String(s) if s == "off" => None,
            toml::Value::Table(t) => Some(serde_json::to_value(t).unwrap()),
            v => bail!(
                "{}: [analyse.rules] `{name}` = {v}: analyse rules are \"on\" or \"off\" (or a table of options); there are no levels",
                path.display()
            ),
        };
        if groups_contains(&name) {
            if !matches!(on, None | Some(Value::Null)) {
                bail!(
                    "{}: [analyse.rules] `{name}` is a group: it takes \"on\" or \"off\"",
                    path.display()
                );
            }
            groups.insert(name, on.is_some());
        } else {
            named.push((rule_id(path, "analyse", &name)?, on));
        }
    }
    let ecosystems = gates.ecosystems(|e| groups.get(e) == Some(&false));
    let mut on: BTreeMap<String, Value> = BTreeMap::new();
    for (id, e) in rules().iter().filter(|(id, _)| id.starts_with("analyse/")) {
        let group = group_of(id);
        let enabled = match groups.get(group) {
            Some(g) => *g,
            None => {
                (e.get("preset").and_then(toml::Value::as_str))
                    .and_then(AnalysePreset::parse)
                    .is_some_and(|p| p <= preset)
                    && gates.allows(group, &ecosystems)
            }
        };
        if enabled {
            on.insert(id.clone(), Value::Null);
        }
    }
    for (id, v) in named {
        match v {
            Some(v) => drop(on.insert(id, v)),
            None => drop(on.remove(&id)),
        }
    }
    let mut ignore = t.ignore;
    for (n, i) in ignore.iter_mut().enumerate() {
        let at = || format!("{}: [[analyse.ignore]] #{}", path.display(), n + 1);
        if let Some(r) = &i.rule {
            i.rule = Some(rule_id(path, "analyse", r)?);
        }
        if i.message.is_some() && i.raw_message.is_some() {
            bail!("{}: message and raw-message: give one", at());
        }
        if i.rule.is_none()
            && i.identifier.is_none()
            && i.message.is_none()
            && i.raw_message.is_none()
        {
            bail!("{}: name a rule, an identifier or a message", at());
        }
    }
    Ok(Analyse {
        switches: (SWITCHES.iter())
            .filter(|(p, _)| *p <= preset)
            .map(|(_, s)| *s)
            .collect(),
        preset,
        paths: t.paths.unwrap_or_else(|| files.paths.clone()),
        exclude: [&files.exclude[..], &t.exclude[..]].concat(),
        baseline: t.baseline,
        stubs: t.stubs,
        ignore,
        larastan: ecosystems.contains(&Ecosystem::Laravel)
            && crate::phpstan_parameters::larastan_supports(
                gates.composer.laravel.as_ref().and_then(|l| l.semver),
            ),
        ecosystems,
        rules: (on.into_iter())
            .map(|(id, options)| AnalyseRule { id, options })
            .collect(),
        bleeding_edge: t.bleeding_edge,
    })
}
