//! Resolving a project's PHPStan configuration the way `phpstan analyse` does (PHPStan 2.2.7): the config half of
//! `CommandHelper::begin()` and `ContainerFactory::create()`, down to the container's parameters, including the
//! validation errors PHPStan prints for broken configs. Services are not built.

use crate::di::{
    self, Files, Loader, assert_list, dirname, escape, escape_percent, expand,
    expand_included_file, merge,
};
use crate::php::{absolutize_path, is_fnmatch_pattern, normalize_path, parent_relative_path, preg};
use crate::schema;
use crate::value::{Array, Key, Value};

macro_rules! conf {
    ($($name:literal),* $(,)?) => {
        &[$(($name, include_str!(concat!("../conf/", $name)))),*]
    };
}

/// PHPStan 2.2.7's `conf/` directory (from the PHAR).
pub const CONF: &[(&str, &str)] = conf![
    "bleedingEdge.neon",
    "config.level0.neon",
    "config.level1.neon",
    "config.level2.neon",
    "config.level3.neon",
    "config.level4.neon",
    "config.level5.neon",
    "config.level6.neon",
    "config.level7.neon",
    "config.level8.neon",
    "config.level9.neon",
    "config.level10.neon",
    "config.levelmax.neon",
    "config.neon",
    "config.stubValidator.neon",
    "parametersSchema.neon",
    "parsers.neon",
    "services.neon",
];

/// The PHPStan extensions registered with `#[ContainerExtension]`, in registration (name) order.
const ATTRIBUTE_EXTENSIONS: [&str; 11] = [
    "autowiredAttributeServices",
    "autowiredExtensions",
    "conditionalTags",
    "expandRelativePaths",
    "fnsr",
    "parametersSchema",
    "preloadParameters",
    "rules",
    "validateExcludePaths",
    "validateIgnoredErrors",
    "validateServiceTags",
];

const SENSITIVE_ENV: [&str; 4] = [
    "GITHUB_TOKEN",
    "CI_JOB_TOKEN",
    "PRIVATE-TOKEN",
    "TIDEWAYS_APIKEY",
];

/// Inputs `phpstan analyse` gets from its command line and environment.
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// Working directory.
    pub cwd: String,
    /// `-c`.
    pub config: Option<String>,
    /// `-l`.
    pub level: Option<String>,
    /// Analysed paths from the command line.
    pub paths: Vec<String>,
    /// `--generate-baseline`.
    pub generate_baseline: Option<String>,
    /// `-a`.
    pub autoload_file: Option<String>,
    /// Where `phpstan.phar` is (`%rootDir%` is its directory); usually `vendor/phpstan/phpstan/phpstan.phar`.
    pub phar: String,
    /// phpstan/extension-installer's `GeneratedConfig.php`, when installed.
    pub generated_config: Option<String>,
    /// `sys_get_temp_dir()`.
    pub sys_temp_dir: String,
    /// `getenv()`.
    pub env: Vec<(String, String)>,
    /// Projects whose Composer autoloader PHPStan's bin script loaded.
    pub composer_autoloader_project_paths: Vec<String>,
    /// `ini_get('register_argc_argv') === '1'`.
    pub register_argc_argv: bool,
}

impl Options {
    /// Defaults for a project directory: config discovery, `vendor/phpstan/phpstan/phpstan.phar`, the
    /// extension-installer's GeneratedConfig if present, this process's environment and temp dir.
    pub fn for_project(cwd: &str) -> Options {
        let generated = format!("{cwd}/vendor/phpstan/extension-installer/src/GeneratedConfig.php");
        Options {
            cwd: cwd.to_string(),
            phar: format!("{cwd}/vendor/phpstan/phpstan/phpstan.phar"),
            generated_config: std::path::Path::new(&generated)
                .is_file()
                .then_some(generated),
            sys_temp_dir: std::env::temp_dir()
                .to_string_lossy()
                .trim_end_matches('/')
                .to_string(),
            env: std::env::vars().collect(),
            ..Options::default()
        }
    }
}

/// Real files, with `phar://…/phpstan.phar/conf/*` served from [`CONF`].
pub struct ProjectFiles {
    phar: String,
}

impl ProjectFiles {
    pub fn new(phar: &str) -> ProjectFiles {
        ProjectFiles {
            phar: normalize_path(phar),
        }
    }

    fn embedded(&self, path: &str) -> Option<&'static str> {
        let n = normalize_path(path);
        let rel = n
            .strip_prefix(&format!("phar://{}/", self.phar))
            .or_else(|| n.strip_prefix("phar://phpstan.phar/"))?;
        let name = rel.strip_prefix("conf/")?;
        CONF.iter().find(|(f, _)| *f == name).map(|(_, c)| *c)
    }
}

impl Files for ProjectFiles {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        if path.starts_with("phar://") {
            return self.embedded(path).map(|c| c.as_bytes().to_vec());
        }
        std::path::Path::new(path)
            .is_file()
            .then(|| std::fs::read(path).ok())
            .flatten()
    }
}

/// The resolved container parameters (`$container->getParameters()`, without `env`).
#[derive(Debug)]
pub struct Resolved {
    pub parameters: Array,
}

fn s(v: &str) -> Value {
    Value::str(v)
}

fn list(items: &[String]) -> Value {
    Value::Array(Array::list(items.iter().map(|i| s(i))))
}

fn opt(v: &Option<String>) -> Value {
    v.as_deref().map_or(Value::Null, s)
}

fn arr(pairs: Vec<(&str, Value)>) -> Array {
    let mut a = Array::new();
    for (k, v) in pairs {
        a.set(Key::from(k), v);
    }
    a
}

fn get<'a>(a: &'a Array, path: &[&str]) -> Option<&'a Value> {
    let mut v = a.get_str(path[0])?;
    for k in &path[1..] {
        v = v.as_array()?.get_str(k)?;
    }
    Some(v).filter(|v| **v != Value::Null)
}

/// `CommandHelper::begin()` up to the container, then `ContainerFactory::create()`. `Err` is what PHPStan prints
/// (console tags stripped) before exiting.
pub fn resolve(o: &Options) -> Result<Resolved, String> {
    // PHP opens a PHAR through its real path, so `%rootDir%` has symlinks resolved.
    let phar = std::fs::canonicalize(&o.phar)
        .map_or_else(|_| o.phar.clone(), |p| p.to_string_lossy().into_owned());
    let files = ProjectFiles::new(&phar);
    resolve_with(&Options { phar, ..o.clone() }, &files)
}

pub fn resolve_with(o: &Options, files: &dyn Files) -> Result<Resolved, String> {
    let cwd = normalize_path(&o.cwd);
    let mut env = Array::new();
    for (k, v) in &o.env {
        if !SENSITIVE_ENV.contains(&k.as_str()) {
            env.set(Key::from_string(k.clone()), s(v));
        }
    }
    let config = match &o.config {
        None => [
            ".phpstan.neon",
            "phpstan.neon",
            ".phpstan.neon.dist",
            "phpstan.neon.dist",
            ".phpstan.dist.neon",
            "phpstan.dist.neon",
        ]
        .iter()
        .map(|n| format!("{cwd}/{n}"))
        .find(|f| files.is_file(f)),
        Some(c) => Some(absolutize_path(&cwd, c)),
    };
    let generate_baseline = o
        .generate_baseline
        .as_ref()
        .map(|g| normalize_path(&absolutize_path(&cwd, g)));
    let mut level = o.level.clone();
    if config.is_none() && level.is_none() {
        level = Some("0".into());
    }
    let mut paths: Vec<String> = o
        .paths
        .iter()
        .map(|p| normalize_path(&absolutize_path(&cwd, p)))
        .collect();
    let root_dir = normalize_path(&dirname(&o.phar));
    let phar_root = normalize_path(&format!("phar://{}", o.phar));
    let config_dir = format!("{phar_root}/conf");
    let loader_params = arr(vec![
        ("rootDir", s(&root_dir)),
        ("currentWorkingDirectory", s(&cwd)),
        ("env", Value::Array(env.clone())),
    ]);

    let mut tmp_dir = None;
    let mut paths_from_config: Vec<String> = Vec::new();
    if let Some(config) = &config {
        if !files.is_file(config) {
            return Err(format!(
                "Project config file at path {config} does not exist."
            ));
        }
        let expand_relative = vec![
            "[parameters][paths][]".to_string(),
            "[parameters][tmpDir]".to_string(),
        ];
        let project = Loader::new(
            files,
            loader_params.clone(),
            expand_relative,
            generate_baseline.clone(),
        )
        .load_merged(config)?;
        if let Some(t) = get(&project, &["parameters", "tmpDir"]) {
            tmp_dir = expand(t, &loader_params, None)?.to_php_string();
        }
        if level.is_none()
            && let Some(l) = get(&project, &["parameters", "level"])
        {
            level = l.to_php_string();
        }
        if let Some(p) = get(&project, &["parameters", "paths"])
            && let Value::Array(a) = expand(p, &loader_params, None)?
        {
            paths_from_config = a.values().filter_map(Value::to_php_string).collect();
        }
        if paths.is_empty() {
            paths = paths_from_config.clone();
        }
    }

    let mut additional = Vec::new();
    if let Some(l) = &level {
        let file = format!("{config_dir}/config.level{l}.neon");
        if !files.is_file(&file) {
            return Err(format!("Level config file {file} was not found."));
        }
        additional.push(file);
    }
    if let Some(generated) = &o.generated_config {
        let php = files
            .read(generated)
            .ok_or_else(|| format!("File '{generated}' is missing or is not readable."))?;
        let dir = dirname(generated);
        let extensions =
            crate::phpfile::generated_config_extensions(&String::from_utf8_lossy(&php), &dir)?;
        for (name, ext) in extensions.iter() {
            let Some(ext) = ext.as_array() else { continue };
            let includes = get(ext, &["extra", "includes"])
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            for inc in includes.values() {
                let Value::Str(inc) = inc else {
                    return Err(format!(
                        "Cannot include config from package {}, expecting string file path but got {}",
                        name.as_string(),
                        inc.gettype()
                    ));
                };
                let mut path = get(ext, &["relative_install_path"])
                    .and_then(Value::to_php_string)
                    .map(|rel| format!("{dir}/{rel}/{inc}"))
                    .filter(|p| files.is_file(p));
                if path.is_none() {
                    let install = get(ext, &["install_path"])
                        .and_then(Value::to_php_string)
                        .unwrap_or_default();
                    path = Some(format!("{install}/{inc}"));
                }
                let path = path.expect("set");
                if !files.is_file(&path) {
                    return Err(format!(
                        "Config file {path} does not exist or isn't readable"
                    ));
                }
                additional.push(path);
            }
        }
    }
    if let Some(config) = &config {
        additional.push(config.clone());
    }
    let tmp_dir = tmp_dir.unwrap_or_else(|| format!("{}/phpstan", o.sys_temp_dir));

    // ContainerFactory::create()
    let schema_file =
        format!("{phar_root}/src/DependencyInjection/../../conf/parametersSchema.neon");
    let mut all_config_files = Vec::new();
    let mut project_config = Value::Array(Array::new());
    for file in std::iter::once(&schema_file).chain(&additional) {
        let (found, data) = config_files(files, file, &loader_params)?;
        all_config_files.extend(found);
        project_config = merge(Value::Array(data), project_config);
    }
    let all_config_files: Vec<String> =
        all_config_files.iter().map(|f| normalize_path(f)).collect();
    let mut seen: Vec<&String> = Vec::new();
    let mut duplicates: Vec<&String> = Vec::new();
    for f in &all_config_files {
        if seen.contains(&f) {
            if !duplicates.contains(&f) {
                duplicates.push(f);
            }
        } else {
            seen.push(f);
        }
    }
    if !duplicates.is_empty() {
        let what = if duplicates.len() == 1 {
            "This file is included multiple times:"
        } else {
            "These files are included multiple times:"
        };
        return Err(format!(
            "{what}\n- {}",
            duplicates
                .iter()
                .map(|d| d.as_str())
                .collect::<Vec<_>>()
                .join("\n- ")
        ));
    }
    let Value::Array(project_config) = project_config else {
        unreachable!()
    };
    let expand_relative: Vec<String> = project_config
        .get_str("expandRelativePaths")
        .and_then(Value::as_array)
        .map(|a| a.values().filter_map(Value::to_php_string).collect())
        .unwrap_or_default();

    let static_params = arr(vec![
        ("debugMode", Value::Bool(true)),
        ("productionMode", Value::Bool(false)),
        ("tempDir", s(&tmp_dir)),
        ("rootDir", s(&root_dir)),
        ("currentWorkingDirectory", s(&cwd)),
        (
            "cliArgumentsVariablesRegistered",
            Value::Bool(o.register_argc_argv),
        ),
        ("tmpDir", s(&tmp_dir)),
        ("additionalConfigFiles", list(&additional)),
        ("allConfigFiles", list(&all_config_files)),
        (
            "composerAutoloaderProjectPaths",
            list(&o.composer_autoloader_project_paths),
        ),
        ("generateBaselineFile", opt(&generate_baseline)),
        ("usedLevel", s(level.as_deref().unwrap_or("0"))),
        ("cliAutoloadFile", opt(&o.autoload_file)),
        ("env", Value::Array(env)),
    ]);
    let dynamic = arr(vec![
        ("singleReflectionFile", Value::Null),
        ("singleReflectionInsteadOfFile", Value::Null),
        ("analysedPaths", list(&paths)),
        ("analysedPathsFromConfig", list(&paths_from_config)),
    ]);

    let mut loader = Loader::new(
        files,
        static_params.clone(),
        expand_relative,
        generate_baseline,
    );
    let mut datas = loader.load(&format!("{config_dir}/config.neon"))?;
    for file in &additional {
        datas.extend(loader.load(file)?);
    }
    datas.push(arr(vec![(
        "parameters",
        escape(Value::Array(static_params)),
    )]));

    let mut sections: Vec<(Key, Vec<Value>)> = Vec::new();
    for data in datas {
        for (k, v) in data {
            match sections.iter_mut().find(|(s, _)| *s == k) {
                Some((_, list)) => list.push(v),
                None => sections.push((k, vec![v])),
            }
        }
    }
    let section = |name: &str| {
        sections
            .iter()
            .find(|(k, _)| *k == Key::from(name))
            .map(|(_, l)| l.as_slice())
            .unwrap_or(&[])
    };

    // ParametersExtension: Expect::array() over every `parameters:` section, then expansion.
    let mut flat: Option<Value> = None;
    for data in section("parameters") {
        flat = Some(match flat {
            None => data.clone(),
            Some(base) => merge(data.clone(), base),
        });
    }
    let mut params = match flat.unwrap_or(Value::Null) {
        Value::Null => Array::new(),
        Value::Array(a) => {
            let Value::Array(a) = merge(Value::Array(a), Value::Array(Array::new())) else {
                unreachable!()
            };
            a
        }
        other => {
            return Err(format!(
                "Invalid configuration:\nThe item 'parameters' expects to be array, {} given.",
                schema::format_value(&other)
            ));
        }
    };
    for (k, v) in dynamic {
        params.set(k, escape_percent(v));
    }
    let params = sys_get_temp_dir_calls(Value::Array(params), &o.sys_temp_dir);
    let Value::Array(params) = params else {
        unreachable!()
    };
    let Value::Array(mut params) = expand(&Value::Array(params.clone()), &params, Some(&[]))?
    else {
        unreachable!()
    };
    // ParametersExtension then expands every other section with them (services later, per definition); only the
    // errors matter here.
    for (name, datas) in &sections {
        if *name != Key::from("services") {
            for d in datas {
                expand(d, &params, None)?;
            }
        }
    }

    let mut known: Vec<String> = ["services", "parameters", "php", "extensions"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    known.extend(ATTRIBUTE_EXTENSIONS.iter().map(|s| s.to_string()));
    for ext in section("extensions") {
        if let Value::Array(a) = ext {
            known.extend(
                a.keys()
                    .filter(|k| matches!(k, Key::Str(_)))
                    .map(Key::as_string),
            );
        }
    }

    let validate = params.get_str("__validate").is_some_and(truthy);
    validate_exclude_paths(&mut params, validate, config.as_deref())?;
    if validate {
        validate_ignored_errors(&params)?;
    }
    for d in section("services") {
        if let Value::Array(defs) = d {
            for def in defs.values() {
                expand(def, &params, None)?;
            }
        }
    }
    // `elseif ($extra = key(array_diff_key(...)))`: only the first unknown section counts, and section 0 is falsy.
    if let Some((extra, _)) = sections
        .iter()
        .find(|(k, _)| !known.contains(&k.as_string()))
        .filter(|(k, _)| *k != Key::Int(0))
    {
        let extra = extra.as_string();
        let hint = schema::suggestion(&known, &extra);
        return Err(format!(
            "Invalid configuration:\nFound section '{extra}' in configuration, but corresponding extension is missing{}",
            hint.map_or(".".to_string(), |h| format!(", did you mean '{h}'?"))
        ));
    }

    if validate {
        let schema_section = project_config
            .get_str("parametersSchema")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let errors = schema::parameters_schema(&schema_section)?
            .validate(&Value::Array(params.clone()), &["parameters"]);
        if !errors.is_empty() {
            return Err(errors
                .iter()
                .map(|e| format!("Invalid configuration:\n{e}"))
                .collect::<Vec<_>>()
                .join("\n"));
        }
        validate_php_version(&params)?;
        validate_ignore_error_entries(&params)
            .map_err(|e| format!("Invalid configuration:\n{e}"))?;
    }
    params.remove(&Key::from("env"));
    // A statement left in the parameters becomes a dynamic parameter, whose generated code references the unprefixed
    // exception class: every `phpstan` run then dies with this message.
    // ponytail: PHP would evaluate a plain function call (`::strtoupper(a)`) fine; only sys_get_temp_dir() is.
    if contains_statement(&Value::Array(params.clone())) {
        return Err("Class \"Nette\\DI\\ServiceCreationException\" not found".into());
    }
    Ok(Resolved { parameters: params })
}

fn contains_statement(v: &Value) -> bool {
    match v {
        Value::Entity(_) => true,
        Value::Array(a) => a.values().any(contains_statement),
        _ => false,
    }
}

fn truthy(v: &Value) -> bool {
    !matches!(v, Value::Null | Value::Bool(false) | Value::Int(0))
        && *v != Value::str("")
        && *v != Value::str("0")
}

/// `ContainerFactory::getConfigFiles()`: the files a config pulls in (missing ones skipped) and their data, merged
/// with each include taking precedence over its parent.
fn config_files(
    files: &dyn Files,
    file: &str,
    loader_params: &Array,
) -> Result<(Vec<String>, Array), String> {
    if !files.is_file(file) {
        return Ok((Vec::new(), Array::new()));
    }
    let mut data = if file.ends_with(".php") {
        di::load_file(files, file, &[])?
    } else {
        di::load_neon(files, file, &[])?
    };
    let mut all = vec![file.to_string()];
    if let Some(includes) = data
        .get_str("includes")
        .filter(|v| **v != Value::Null)
        .cloned()
    {
        assert_list(&includes, &format!("section 'includes' in file '{file}'"))?;
        let Value::Array(includes) = expand(&includes, loader_params, None)? else {
            unreachable!()
        };
        for include in includes.values() {
            let include = expand_included_file(&include.to_php_string().unwrap_or_default(), file);
            let (found, inc) = config_files(files, &include, loader_params)?;
            all.extend(found);
            let Value::Array(d) = merge(Value::Array(inc), Value::Array(data)) else {
                unreachable!()
            };
            data = d;
        }
    }
    Ok((all, data))
}

/// `sysGetTempDir: ::sys_get_temp_dir()` is the one statement among PHPStan's own parameters; it is evaluated like the
/// container does at runtime.
fn sys_get_temp_dir_calls(v: Value, tmp: &str) -> Value {
    match v {
        Value::Entity(e)
            if e.value == Value::str("::sys_get_temp_dir") && e.attributes.is_empty() =>
        {
            Value::Str(tmp.replace('%', "%%"))
        }
        Value::Array(a) => {
            let mut res = Array::new();
            for (k, x) in a {
                res.set(k, sys_get_temp_dir_calls(x, tmp));
            }
            Value::Array(res)
        }
        v => v,
    }
}

fn path_problem(path: &str) -> Option<String> {
    let exists = path.starts_with('/') && std::path::Path::new(path).exists();
    (!exists && !is_fnmatch_pattern(path)).then(|| {
        format!("Path \"{path}\" is neither a directory, nor a file path, nor a fnmatch pattern.")
    })
}

/// `ValidateExcludePathsExtension`: rewrites `excludePaths` to its `analyseAndScan`/`analyse` keys without the
/// `(?)` markers, or fails on paths that don't exist.
fn validate_exclude_paths(
    params: &mut Array,
    validate: bool,
    config: Option<&str>,
) -> Result<(), String> {
    let Some(Value::Array(exclude)) = params.get_str("excludePaths").cloned() else {
        return Ok(());
    };
    let mut new = Array::new();
    for key in ["analyseAndScan", "analyse"] {
        if let Some(v) = exclude.get_str(key) {
            new.set(Key::from(key), v.clone());
        }
    }
    let mut errors = Vec::new();
    let mut suggest: Vec<(String, Vec<String>)> = Vec::new();
    if validate {
        for (key, paths) in new.iter() {
            for p in paths
                .as_array()
                .map(|a| a.values().cloned().collect::<Vec<_>>())
                .unwrap_or_default()
            {
                let Value::Str(p) = p else { continue };
                if let Some(e) = path_problem(&p) {
                    errors.push(e);
                    match suggest.iter_mut().find(|(k, _)| *k == key.as_string()) {
                        Some((_, l)) => l.push(p),
                        None => suggest.push((key.as_string(), vec![p])),
                    }
                }
            }
        }
    }
    if !errors.is_empty() {
        let mut out = format!(
            "Invalid {} in excludePaths:\n",
            if errors.len() == 1 {
                "entry"
            } else {
                "entries"
            }
        );
        for e in &errors {
            out.push_str(e);
            out.push_str("\n\n");
        }
        out.push_str("If the excluded path can sometimes exist, append (?)\nto its config entry to mark it as optional. Example:\n\n");
        out.push_str("parameters:\n\texcludePaths:\n");
        for (key, paths) in &suggest {
            out.push_str(&format!("\t\t{key}:\n"));
            for p in paths {
                let p = config.map_or_else(|| p.clone(), |c| parent_relative_path(&dirname(c), p));
                out.push_str(&format!("\t\t\t- {p} (?)\n"));
            }
        }
        return Err(out);
    }
    let mut unwrapped = Array::new();
    for (k, paths) in new {
        let paths = match paths {
            Value::Array(a) => Value::Array(Array::list(a.into_iter().map(|(_, p)| match p {
                Value::OptionalPath(p) => Value::Str(p),
                p => p,
            }))),
            p => p,
        };
        unwrapped.set(k, paths);
    }
    params.set(Key::from("excludePaths"), Value::Array(unwrapped));
    Ok(())
}

/// `ValidateIgnoredErrorsExtension` (regex compilation and, with `reportUnmatchedIgnoredErrors`, paths).
/// ponytail: IgnoredRegexValidator's "unescaped `|` ignores types" / "anchor in the middle" checks need PHPStan's
/// type parser and are not ported yet.
fn validate_ignored_errors(params: &Array) -> Result<(), String> {
    let Some(Value::Array(ignore)) = params.get_str("ignoreErrors") else {
        return Ok(());
    };
    if ignore.is_empty() {
        return Ok(());
    }
    let mut errors = Vec::new();
    for entry in ignore.values() {
        let messages: Vec<String> = match entry {
            Value::Array(a) if a.get_str("count").is_some_and(|c| *c != Value::Null) => continue,
            Value::Array(a) => match (get(a, &["messages"]), get(a, &["message"])) {
                (Some(Value::Array(m)), _) => m.values().filter_map(Value::to_php_string).collect(),
                (_, Some(m)) => m.to_php_string().into_iter().collect(),
                _ => continue,
            },
            other => other.to_php_string().into_iter().collect(),
        };
        for m in messages {
            if let Err(e) = preg(&m) {
                errors.push(e);
            }
        }
    }
    if params
        .get_str("reportUnmatchedIgnoredErrors")
        .is_some_and(truthy)
    {
        for entry in ignore.values() {
            let Value::Array(a) = entry else { continue };
            let paths: Vec<Value> = if let Some(p) = get(a, &["path"]) {
                if !matches!(p, Value::Str(_)) {
                    errors.push(format!("Key 'path' of ignoreErrors expects a string, {} given. Did you mean 'paths'?", p.gettype()));
                    continue;
                }
                vec![p.clone()]
            } else if let Some(p) = get(a, &["paths"]) {
                let Value::Array(p) = p else {
                    errors.push(format!("Key 'paths' of ignoreErrors expects an array, {} given. Did you mean 'path'?", p.gettype()));
                    continue;
                };
                p.values().cloned().collect()
            } else {
                continue;
            };
            for p in paths {
                if let Some(e) = path_problem(&p.to_php_string().unwrap_or_default()) {
                    errors.push(e);
                }
            }
        }
    }
    if errors.is_empty() {
        return Ok(());
    }
    let mut out = format!(
        "Invalid {} in ignoreErrors:\n",
        if errors.len() == 1 {
            "entry"
        } else {
            "entries"
        }
    );
    for e in &errors {
        out.push_str(e);
        out.push_str("\n\n");
    }
    out.push_str("To ignore non-existent paths in ignoreErrors,\nset reportUnmatchedIgnoredErrors: false in your configuration file.\n");
    Err(out)
}

fn validate_php_version(params: &Array) -> Result<(), String> {
    if let Some(Value::Array(v)) = params.get_str("phpVersion")
        && let (Some(Value::Int(min)), Some(Value::Int(max))) = (v.get_str("min"), v.get_str("max"))
        && max < min
    {
        return Err("Invalid configuration:\nInvalid PHP version range: phpVersion.max should be greater or equal to phpVersion.min.".into());
    }
    Ok(())
}

/// `ContainerFactory::validateParameters()`'s `ignoreErrors` entry rules.
fn validate_ignore_error_entries(params: &Array) -> Result<(), String> {
    let Some(Value::Array(ignore)) = params.get_str("ignoreErrors") else {
        return Ok(());
    };
    let at_least = [
        "message",
        "messages",
        "rawMessage",
        "rawMessages",
        "identifier",
        "identifiers",
        "path",
        "paths",
    ];
    for entry in ignore.values() {
        let Value::Array(e) = entry else { continue };
        let has = |k: &str| e.contains(&Key::from(k));
        if !at_least.iter().any(|k| has(k)) {
            return Err(format!(
                "An ignoreErrors entry must contain at least one of the following fields: {}.",
                at_least.join(", ")
            ));
        }
        for group in [
            &["rawMessage", "rawMessages", "message", "messages"][..],
            &["identifier", "identifiers"],
            &["path", "paths"],
        ] {
            for (i, a) in group.iter().enumerate() {
                for b in &group[i + 1..] {
                    if has(a) && has(b) {
                        return Err(format!(
                            "An ignoreErrors entry cannot contain both {a} and {b} fields."
                        ));
                    }
                }
            }
        }
        if has("count") && !has("path") {
            return Err(
                "An ignoreErrors entry with count field must also contain path field.".into(),
            );
        }
    }
    Ok(())
}
