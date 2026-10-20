//! Which files are analysed and which are only scanned for symbols, as PHPStan decides it: `paths` minus
//! `excludePaths`, plus `scanFiles`, `scanDirectories` and what Composer autoloads (the project's composer.json and
//! vendor/composer/installed.json, as PHPStan's ComposerJsonAndInstalledJsonSourceLocatorMaker reads them).

use std::path::{Path, PathBuf};

use pun_phpstan::Array;
use pun_phpstan::Value;
use pun_phpstan::ignore::IgnoredErrors;
use pun_phpstan::php::FileExcluder;
use rayon::prelude::*;
use serde_json::Value as Json;

use crate::relative_path::{RelativePath, absolute};
use crate::{display, php_files};

pub struct Project {
    /// Reported on.
    pub analysed: Vec<PathBuf>,
    /// Symbols only.
    pub scanned: Vec<PathBuf>,
    /// Composer's psr-4 and psr-0 files: symbols only, scanned when an analysed or scanned file names the class their
    /// path holds, as PHPStan's PSR locators find classes on demand.
    pub autoloaded: Vec<Autoloaded>,
    /// `PHP_VERSION_ID`.
    pub php_version: u32,
    /// `reportUnmatchedIgnoredErrors`.
    pub report_unmatched_ignores: bool,
    /// `reportIgnoresWithoutComments`.
    pub report_ignores_without_comments: bool,
    /// `ignoreErrors` (with the baseline's entries).
    pub ignore_errors: IgnoredErrors,
    /// PHPStan's `checkClassCaseSensitivity` (see `rules::Checks`): false, as at level 0, until the presets set it.
    pub check_class_case_sensitivity: bool,
    /// The rule level: `level` (`max` is 10), 0 when unset.
    pub level: u8,
    /// `featureToggles.checkGenericIterableClasses` (on with bleedingEdge).
    pub check_generic_iterable_classes: bool,
    /// `featureToggles.internalTag` (on with bleedingEdge): the `@internal` usage rules.
    pub internal_tag: bool,
    /// `featureToggles.newStaticInAbstractClassStaticMethod` (on with bleedingEdge).
    pub new_static_in_abstract_class_static_method: bool,
    /// `featureToggles.reportPreciseLineForUnusedFunctionParameter` (on with bleedingEdge): unused closure uses and
    /// constructor parameters are reported on their own line, not the function's.
    pub precise_unused_parameter_line: bool,
    /// Every analysed path is a file (FileFinderResult's `isOnlyFiles()`).
    pub only_files: bool,
    /// `featureToggles.bleedingEdge`.
    pub bleeding_edge: bool,
    /// `featureToggles.unusedLabel` (on with bleedingEdge).
    pub unused_label: bool,
    /// `featureToggles.switchConditionAlwaysFalse` (on with bleedingEdge): SwitchConditionRule.
    pub switch_condition: bool,
    /// `featureToggles.sortWithoutEffect` (on with bleedingEdge): SortWithoutEffectRule.
    pub sort_without_effect: bool,
    /// `checkUninitializedProperties`: UninitializedPropertyRule (config.neon's conditional rule).
    pub check_uninitialized_properties: bool,
    /// `deprecationRulesInstalled`: the config includes phpstan-deprecation-rules' rules.neon.
    pub deprecation_rules: bool,
    /// The `strictRules` switches of phpstan-strict-rules' rules.neon that Pun ports (all off without it).
    pub strict_rules: StrictRules,
    /// How anonymous class names print their file: relative to the working directory and the analysed paths.
    pub relative_path: RelativePath,
    /// Larastan's rules and their switches, when the config includes Larastan's extension.neon.
    pub larastan: Option<crate::Larastan>,
    /// `typeAliases`: the global type aliases (name, type string).
    pub type_aliases: Vec<(String, String)>,
    /// `stubFiles`: scanned as Mago patches, whose declarations override the symbols they name.
    pub stubs: Vec<PathBuf>,
    /// Which identifiers the run reports, `None` for all: the others are dropped before any ignore sees them, as
    /// PHPStan never reports the errors of a rule it doesn't run.
    pub reported: Option<Reported>,
}

/// [`Project::reported`].
pub type Reported = std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync>;

/// phpstan-strict-rules' `strictRules.*` switches of the ported rules.
#[derive(Clone, Copy, Default)]
pub struct StrictRules {
    /// `closureUsesThis`: ClosureUsesThisRule.
    pub closure_uses_this: bool,
    /// `requireParentConstructorCall`: RequireParentConstructCallRule.
    pub require_parent_constructor_call: bool,
    /// `matchingInheritedMethodNames`: WrongCaseOfInheritedMethodRule.
    pub matching_inherited_method_names: bool,
    /// `dynamicCallOnStaticMethod`: DynamicCallOnStaticMethodsRule and DynamicCallOnStaticMethodsCallableRule.
    pub dynamic_call_on_static_method: bool,
    /// `booleansInConditions`: BooleanInBooleanAndRule, BooleanInBooleanOrRule and BooleanInBooleanNotRule.
    pub booleans_in_conditions: bool,
    /// `numericOperandsInArithmeticOperators`: OperandsInArithmetic*Rule and OperandInArithmetic*Rule.
    pub numeric_operands_in_arithmetic_operators: bool,
}

impl StrictRules {
    fn from_params(params: &Array) -> StrictRules {
        let strict = params.get_str("strictRules").and_then(Value::as_array);
        let on = |k: &str| strict.and_then(|s| s.get_str(k)) == Some(&Value::Bool(true));
        StrictRules {
            closure_uses_this: on("closureUsesThis"),
            require_parent_constructor_call: on("requireParentConstructorCall"),
            matching_inherited_method_names: on("matchingInheritedMethodNames"),
            dynamic_call_on_static_method: on("dynamicCallOnStaticMethod"),
            booleans_in_conditions: on("booleansInConditions"),
            numeric_operands_in_arithmetic_operators: on("numericOperandsInArithmeticOperators"),
        }
    }
}

/// PHP 8.5.11, the oracle's runtime, which PHPStan uses when `phpVersion` is null.
pub(crate) const RUNTIME_PHP_VERSION: u32 = 80511;

fn strings<'a>(params: &'a Array, key: &str) -> Vec<&'a str> {
    params
        .get_str(key)
        .and_then(Value::as_array)
        .map(|a| a.values().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

impl Project {
    pub fn from_config(root: &Path, params: &Array) -> Project {
        let extensions: Vec<String> = strings(params, "fileExtensions")
            .into_iter()
            .map(str::to_string)
            .collect();
        let excludes = params.get_str("excludePaths").and_then(Value::as_array);
        let group = |k: &str| {
            excludes
                .and_then(|e| e.get_str(k))
                .and_then(Value::as_array)
                .map(|a| a.values().filter_map(Value::as_str).collect::<Vec<_>>())
                .unwrap_or_default()
        };
        let both = FileExcluder::new(group("analyseAndScan"));
        let analyse_only = FileExcluder::new(group("analyse"));
        let mut analysed = Vec::new();
        let mut scanned = Vec::new();
        let paths: Vec<String> = strings(params, "paths")
            .into_iter()
            .map(|p| absolute(Path::new(p)))
            .collect();
        for path in strings(params, "paths") {
            let p = Path::new(path);
            let found = if p.is_file() {
                vec![p.to_path_buf()]
            } else {
                php_files(p, &extensions)
            };
            for f in found {
                let name = display(&f);
                if both.is_excluded(&name) {
                    continue;
                }
                if analyse_only.is_excluded(&name) {
                    scanned.push(f);
                } else {
                    analysed.push(f);
                }
            }
        }
        scanned.extend(strings(params, "scanFiles").into_iter().map(PathBuf::from));
        for dir in strings(params, "scanDirectories") {
            scanned.extend(php_files(Path::new(dir), &extensions));
        }
        // PHPStan's PSR locators look classes up whatever excludePaths says
        let (composer, autoloaded) = composer_files(root, &extensions);
        scanned.extend(composer);
        scanned.retain(|f| !both.is_excluded(&display(f)));
        let php_version = params
            .get_str("phpVersion")
            // the schema's `{min, max}` structure requires both as ints
            .and_then(|v| match v {
                Value::Array(a) => a.get_str("min"),
                v => Some(v),
            })
            .and_then(|v| match v {
                Value::Int(i) => u32::try_from(*i).ok(),
                _ => None,
            })
            .unwrap_or(RUNTIME_PHP_VERSION);
        let flag = |k: &str| params.get_str(k) == Some(&Value::Bool(true));
        let toggle = |k: &str| {
            params
                .get_str("featureToggles")
                .and_then(Value::as_array)
                .and_then(|t| t.get_str(k))
                == Some(&Value::Bool(true))
        };
        let report_unmatched_ignores = flag("reportUnmatchedIgnoredErrors");
        let empty = Array::new();
        let ignore_errors = params
            .get_str("ignoreErrors")
            .and_then(Value::as_array)
            .unwrap_or(&empty);
        let level = match params.get_str("level") {
            Some(Value::Int(i)) => u8::try_from(*i).unwrap_or(10).min(10),
            Some(Value::Str(s)) if s == "max" => 10,
            _ => 0,
        };
        Project {
            analysed,
            scanned,
            autoloaded,
            php_version,
            report_unmatched_ignores,
            report_ignores_without_comments: flag("reportIgnoresWithoutComments"),
            ignore_errors: IgnoredErrors::new(ignore_errors, report_unmatched_ignores),
            // config.neon's false, which config.level2.neon turns on
            check_class_case_sensitivity: flag("checkClassCaseSensitivity"),
            level,
            check_generic_iterable_classes: toggle("checkGenericIterableClasses"),
            internal_tag: toggle("internalTag"),
            new_static_in_abstract_class_static_method: toggle("newStaticInAbstractClassStaticMethod"),
            only_files: strings(params, "paths").iter().all(|p| Path::new(p).is_file()),
            bleeding_edge: toggle("bleedingEdge"),
            unused_label: toggle("unusedLabel"),
            switch_condition: toggle("switchConditionAlwaysFalse"),
            sort_without_effect: toggle("sortWithoutEffect"),
            check_uninitialized_properties: flag("checkUninitializedProperties"),
            deprecation_rules: flag("deprecationRulesInstalled"),
            strict_rules: StrictRules::from_params(params),
            precise_unused_parameter_line: toggle("reportPreciseLineForUnusedFunctionParameter"),
            relative_path: RelativePath::of(&paths),
            larastan: crate::Larastan::from_params(root, params),
            type_aliases: params
                .get_str("typeAliases")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(|(k, v)| Some((k.as_string(), v.as_str()?.to_string())))
                        .collect()
                })
                .unwrap_or_default(),
            stubs: strings(params, "stubFiles")
                .into_iter()
                .map(PathBuf::from)
                .collect(),
            reported: None,
        }
    }

    /// PHPStan's defaults (no `ignoreErrors`, unmatched ignores reported) over the given files.
    pub fn new(analysed: Vec<PathBuf>, scanned: Vec<PathBuf>) -> Project {
        let paths: Vec<String> = analysed.iter().map(|p| absolute(p)).collect();
        Project {
            relative_path: RelativePath::of(&paths),
            analysed,
            scanned,
            autoloaded: Vec::new(),
            php_version: RUNTIME_PHP_VERSION,
            report_unmatched_ignores: true,
            report_ignores_without_comments: false,
            ignore_errors: IgnoredErrors::new(&Array::new(), true),
            check_class_case_sensitivity: false,
            level: 0,
            check_generic_iterable_classes: false,
            internal_tag: false,
            new_static_in_abstract_class_static_method: false,
            only_files: true,
            bleeding_edge: false,
            unused_label: false,
            switch_condition: false,
            sort_without_effect: false,
            check_uninitialized_properties: false,
            deprecation_rules: false,
            strict_rules: StrictRules::default(),
            precise_unused_parameter_line: false,
            larastan: None,
            type_aliases: Vec::new(),
            stubs: Vec::new(),
            reported: None,
        }
    }
}

/// A psr-4 or psr-0 file and the class its path holds.
pub struct Autoloaded {
    pub path: PathBuf,
    /// The class name, ASCII-lowercased: the psr-4 prefix and the path below the directory, or for psr-0 the path
    /// alone, with `\` for `/` and without the extension.
    pub class: Vec<u8>,
    /// psr-0, where `_` in the class's own name is a directory too.
    pub psr0: bool,
}

/// Where Composer's autoloader finds files, in its order.
enum Source {
    /// A psr-4 directory and its namespace prefix.
    Psr4(String, PathBuf),
    /// A psr-0 directory.
    Psr0(PathBuf),
    /// A classmap entry: a directory or a file.
    Classmap(PathBuf),
    /// A `files` entry.
    File(PathBuf),
}

/// Files Composer can autoload, of the root package (with autoload-dev) and every installed package: classmap entries
/// and `files`, then the psr-4 and psr-0 directories' files, by class.
fn composer_files(root: &Path, extensions: &[String]) -> (Vec<PathBuf>, Vec<Autoloaded>) {
    let read =
        |p: PathBuf| -> Option<Json> { serde_json::from_slice(&std::fs::read(p).ok()?).ok() };
    let mut sources = Vec::new();
    let mut add = |base: &Path, autoload: &Json| {
        for kind in ["psr-4", "psr-0"] {
            for (prefix, dirs) in autoload[kind].as_object().into_iter().flatten() {
                let dirs = match dirs {
                    Json::Array(a) => a.iter().filter_map(Json::as_str).collect(),
                    d => d.as_str().into_iter().collect::<Vec<_>>(),
                };
                for d in dirs {
                    let dir = base.join(d);
                    sources.push(if kind == "psr-4" {
                        Source::Psr4(prefix.clone(), dir)
                    } else {
                        Source::Psr0(dir)
                    });
                }
            }
        }
        for entry in autoload["classmap"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Json::as_str)
        {
            sources.push(Source::Classmap(base.join(entry)));
        }
        for f in autoload["files"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Json::as_str)
        {
            sources.push(Source::File(base.join(f)));
        }
    };
    if let Some(composer) = read(root.join("composer.json")) {
        add(root, &composer["autoload"]);
        add(root, &composer["autoload-dev"]);
    }
    let installed_dir = root.join("vendor/composer");
    if let Some(installed) = read(installed_dir.join("installed.json")) {
        let packages = installed.get("packages").unwrap_or(&installed);
        for p in packages.as_array().into_iter().flatten() {
            if let Some(path) = p["install-path"].as_str() {
                add(&installed_dir.join(path), &p["autoload"]);
            }
        }
    }
    // every package's directories are walked at once
    let found: Vec<(Vec<PathBuf>, Vec<Autoloaded>)> = sources
        .into_par_iter()
        .map(|source| match source {
            Source::Psr4(prefix, d) => (
                Vec::new(),
                by_class(&d, prefix.as_bytes(), false, extensions),
            ),
            Source::Psr0(d) => (Vec::new(), by_class(&d, b"", true, extensions)),
            Source::Classmap(p) if p.is_dir() => (php_files(&p, extensions), Vec::new()),
            Source::Classmap(p) | Source::File(p) => (vec![p], Vec::new()),
        })
        .collect();
    let mut out = Vec::new();
    let mut autoloaded = Vec::new();
    for (files, classes) in found {
        out.extend(files);
        autoloaded.extend(classes);
    }
    // Composer's own classes, in every generated classmap (PHPStan finds it through the project's autoloader)
    // ponytail: other classes only the runtime autoloader knows (autoload_classmap.php) aren't scanned
    for class in ["InstalledVersions.php", "ClassLoader.php"] {
        let path = installed_dir.join(class);
        if path.is_file() {
            out.push(path);
        }
    }
    (out, autoloaded)
}

/// The files below a psr-4 or psr-0 directory, each with the class its path holds.
fn by_class(dir: &Path, prefix: &[u8], psr0: bool, extensions: &[String]) -> Vec<Autoloaded> {
    php_files(dir, extensions)
        .into_iter()
        .map(|path| {
            let mut class = prefix.to_ascii_lowercase();
            let rel = path.strip_prefix(dir).unwrap_or(&path).with_extension("");
            for (i, part) in rel.iter().enumerate() {
                if i > 0 {
                    class.push(b'\\');
                }
                class.extend(part.as_encoded_bytes().iter().map(u8::to_ascii_lowercase));
            }
            Autoloaded { path, class, psr0 }
        })
        .collect()
}
