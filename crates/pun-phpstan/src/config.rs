use std::fmt;
use std::fmt::Write as _;

use crate::di::{
    self, Files, Loader, assert_list, dirname, escape, escape_percent, expand,
    expand_included_file, merge, merge_arrays,
};
use crate::php::{absolutize_path, is_fnmatch_pattern, normalize_path, parent_relative_path, preg};
use crate::value::{Array, Key, Value};
use crate::{neon, phpfile, schema};

/// Why a PHPStan configuration could not be resolved, displayed as PHPStan (or Nette DI) words it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Unreadable(String),
    Neon {
        file: String,
        error: neon::Error,
    },
    UnsupportedPhp {
        file: String,
        error: phpfile::Error,
    },
    UnknownExtension(String),
    RecursiveInclude(String),
    NotAList {
        label: String,
        given: String,
    },
    ReplacingNonArray(String),
    MissingParameter(String),
    CircularReference(Vec<String>),
    NonScalarConcatenation {
        parameter: String,
        into: String,
    },
    MissingProjectConfig(String),
    MissingLevelConfig(String),
    GeneratedConfig(phpfile::Error),
    ExtensionIncludeNotString {
        package: String,
        given: &'static str,
    },
    MissingExtensionConfig(String),
    IncludedMultipleTimes(Vec<String>),
    Schema(schema::Error),
    /// PHPStan's `InvalidConfigurationException`s, each printed as `Invalid configuration:\n<message>`.
    Invalid(Vec<String>),
    InvalidExcludePaths(String),
    InvalidIgnoreErrors(String),
    UnresolvedStatement,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unreadable(file) => write!(f, "File '{file}' is missing or is not readable."),
            Error::Neon { file, error } => write!(f, "Error while loading {file}: {error}"),
            Error::UnsupportedPhp { file, error } => {
                write!(f, "Unsupported PHP config {file}: {error}")
            }
            Error::UnknownExtension(file) => write!(f, "Unknown file extension '{file}'."),
            Error::RecursiveInclude(file) => write!(f, "Recursive included file '{file}'"),
            Error::NotAList { label, given } => {
                write!(f, "The {label} expects to be list, {given} given.")
            }
            Error::ReplacingNonArray(item) => write!(
                f,
                "Replacing operator is available only for arrays, item '{item}' is not array."
            ),
            Error::MissingParameter(name) => write!(f, "Missing parameter '{name}'."),
            Error::CircularReference(names) => write!(
                f,
                "Circular reference detected for parameters: %{}%",
                names.join("%, %")
            ),
            Error::NonScalarConcatenation { parameter, into } => write!(
                f,
                "Unable to concatenate non-scalar parameter '{parameter}' into '{into}'."
            ),
            Error::MissingProjectConfig(file) => {
                write!(f, "Project config file at path {file} does not exist.")
            }
            Error::MissingLevelConfig(file) => write!(f, "Level config file {file} was not found."),
            Error::GeneratedConfig(error) => write!(f, "{error}"),
            Error::ExtensionIncludeNotString { package, given } => write!(
                f,
                "Cannot include config from package {package}, expecting string file path but got {given}"
            ),
            Error::MissingExtensionConfig(file) => {
                write!(f, "Config file {file} does not exist or isn't readable")
            }
            Error::IncludedMultipleTimes(files) => {
                let what = if files.len() == 1 {
                    "This file is included multiple times:"
                } else {
                    "These files are included multiple times:"
                };
                write!(f, "{what}\n- {}", files.join("\n- "))
            }
            Error::Schema(error) => write!(f, "{error}"),
            Error::Invalid(messages) => {
                let messages: Vec<String> = messages
                    .iter()
                    .map(|m| format!("Invalid configuration:\n{m}"))
                    .collect();
                f.write_str(&messages.join("\n"))
            }
            Error::InvalidExcludePaths(message) | Error::InvalidIgnoreErrors(message) => {
                f.write_str(message)
            }
            Error::UnresolvedStatement => {
                f.write_str("Class \"Nette\\DI\\ServiceCreationException\" not found")
            }
        }
    }
}

impl std::error::Error for Error {}

macro_rules! conf {
    ($($name:literal),* $(,)?) => {
        &[$(($name, include_str!(concat!("../conf/", $name)))),*]
    };
}

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

const ATTRIBUTE_EXTENSIONS: [&str; 13] = [
    "attributeServicesDirectories",
    "autowiredAttributeServices",
    "autowiredExtensions",
    "conditionalTags",
    "expandRelativePaths",
    "extensionInterfaces",
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

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub cwd: String,
    pub config: Option<String>,
    pub level: Option<String>,
    pub paths: Vec<String>,
    pub generate_baseline: Option<String>,
    pub autoload_file: Option<String>,
    pub phar: String,
    pub generated_config: Option<String>,
    pub sys_temp_dir: String,
    pub env: Vec<(String, String)>,
    pub composer_autoloader_project_paths: Vec<String>,
    pub register_argc_argv: bool,
}

impl Options {
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

    fn is_file(&self, path: &str) -> bool {
        if path.starts_with("phar://") {
            return self.embedded(path).is_some();
        }
        std::path::Path::new(path).is_file()
    }
}

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

fn opt(v: Option<&str>) -> Value {
    v.map_or(Value::Null, s)
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

pub fn resolve(o: &Options) -> Result<Resolved, Error> {
    let phar = std::fs::canonicalize(&o.phar)
        .map_or_else(|_| o.phar.clone(), |p| p.to_string_lossy().into_owned());
    let files = ProjectFiles::new(&phar);
    resolve_with(&Options { phar, ..o.clone() }, &files)
}

pub fn resolve_with(o: &Options, files: &dyn Files) -> Result<Resolved, Error> {
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
            return Err(Error::MissingProjectConfig(config.clone()));
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
            paths.clone_from(&paths_from_config);
        }
    }

    let mut additional = Vec::new();
    if let Some(l) = &level {
        let file = format!("{config_dir}/config.level{l}.neon");
        if !files.is_file(&file) {
            return Err(Error::MissingLevelConfig(file));
        }
        additional.push(file);
    }
    if let Some(generated) = &o.generated_config {
        let php = files
            .read(generated)
            .ok_or_else(|| Error::Unreadable(generated.clone()))?;
        let dir = dirname(generated);
        let extensions =
            phpfile::generated_config_extensions(&String::from_utf8_lossy(&php), &dir, files)
                .map_err(Error::GeneratedConfig)?;
        for (name, ext) in extensions.iter() {
            let Some(ext) = ext.as_array() else { continue };
            let includes = get(ext, &["extra", "includes"])
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            for inc in includes.values() {
                let Value::Str(inc) = inc else {
                    return Err(Error::ExtensionIncludeNotString {
                        package: name.as_string(),
                        given: inc.gettype(),
                    });
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
                    return Err(Error::MissingExtensionConfig(path));
                }
                additional.push(path);
            }
        }
    }
    if let Some(config) = &config {
        additional.push(config.clone());
    }
    let tmp_dir = tmp_dir.unwrap_or_else(|| format!("{}/phpstan", o.sys_temp_dir));

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
        return Err(Error::IncludedMultipleTimes(
            duplicates.into_iter().cloned().collect(),
        ));
    }
    let project_config = project_config.as_array().cloned().expect("merged arrays");
    let attribute_services_directories = attribute_services_directories(
        project_config.get_str("attributeServicesDirectories"),
        &cwd,
    )?;
    let expand_relative: Vec<String> = project_config
        .get_str("expandRelativePaths")
        .and_then(Value::as_array)
        .map_or_default(|a| a.values().filter_map(Value::to_php_string).collect());

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
        ("generateBaselineFile", opt(generate_baseline.as_deref())),
        ("usedLevel", s(level.as_deref().unwrap_or("0"))),
        ("cliAutoloadFile", opt(o.autoload_file.as_deref())),
        (
            "attributeServicesDirectories",
            list(&attribute_services_directories),
        ),
        ("attributeServicesClasses", list(&[])),
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
            .map_or(&[][..], |(_, l)| l.as_slice())
    };

    let mut flat: Option<Value> = None;
    for data in section("parameters") {
        flat = Some(match flat {
            None => data.clone(),
            Some(base) => merge(data.clone(), base),
        });
    }
    let flat = flat
        .and_then(|f| f.as_array().cloned())
        .expect("static parameters merged last");
    let mut params = merge_arrays(flat, Array::new());
    for (k, v) in dynamic {
        params.set(k, escape_percent(v));
    }
    for v in params.values_mut() {
        sys_get_temp_dir_calls(v, &o.sys_temp_dir);
    }
    let mut params = di::expand_array(&params, &params, Some(&[]))?;
    for (name, datas) in &sections {
        if *name != Key::from("services") {
            for d in datas {
                expand(d, &params, None)?;
            }
        }
    }

    let mut known: Vec<String> = ["services", "parameters", "php", "extensions"]
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    known.extend(
        ATTRIBUTE_EXTENSIONS
            .iter()
            .map(std::string::ToString::to_string),
    );
    for ext in section("extensions").iter().filter_map(Value::as_array) {
        known.extend(
            ext.keys()
                .filter(|k| matches!(k, Key::Str(_)))
                .map(Key::as_string),
        );
    }

    let validate = params.get_str("__validate").is_some_and(truthy);
    validate_exclude_paths(&mut params, validate, config.as_deref())?;
    if validate {
        validate_ignored_errors(&params)?;
    }
    for defs in section("services").iter().filter_map(Value::as_array) {
        for def in defs.values() {
            expand(def, &params, None)?;
        }
    }
    if let Some((extra, _)) = sections
        .iter()
        .find(|(k, _)| !known.contains(&k.as_string()))
        .filter(|(k, _)| *k != Key::Int(0))
    {
        let extra = extra.as_string();
        let hint = schema::suggestion(&known, &extra);
        return Err(Error::Invalid(vec![format!(
            "Found section '{extra}' in configuration, but corresponding extension is missing{}",
            hint.map_or(".".to_string(), |h| format!(", did you mean '{h}'?"))
        )]));
    }

    if validate {
        let schema_section = project_config
            .get_str("parametersSchema")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let errors = schema::parameters_schema(&schema_section)
            .map_err(Error::Schema)?
            .validate(&Value::Array(params.clone()), &["parameters"]);
        if !errors.is_empty() {
            return Err(Error::Invalid(errors));
        }
        validate_php_version(&params)?;
        validate_ignore_error_entries(&params)?;
    }
    params.remove(&Key::from("env"));
    if contains_statement(&Value::Array(params.clone())) {
        return Err(Error::UnresolvedStatement);
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

fn config_files(
    files: &dyn Files,
    file: &str,
    loader_params: &Array,
) -> Result<(Vec<String>, Array), Error> {
    let mut data = if file.ends_with(".php") {
        di::load_file(files, file, &[], &Array::new())?
    } else {
        di::load_neon(
            files,
            file,
            &["[attributeServicesDirectories][]".to_string()],
            &Array::new(),
        )?
    };
    let mut all = vec![file.to_string()];
    if let Some(includes) = data
        .get_str("includes")
        .filter(|v| **v != Value::Null)
        .cloned()
    {
        assert_list(&includes, &format!("section 'includes' in file '{file}'"))?;
        let includes = expand(&includes, loader_params, None)?;
        for include in includes.as_array().expect("a list").values() {
            let include = expand_included_file(&include.to_php_string().unwrap_or_default(), file);
            let (found, inc) = config_files(files, &include, loader_params)?;
            all.extend(found);
            data = merge_arrays(inc, data);
        }
    }
    Ok((all, data))
}

fn attribute_services_directories(
    section: Option<&Value>,
    cwd: &str,
) -> Result<Vec<String>, Error> {
    let entries = match section {
        None | Some(Value::Null) => return Ok(Vec::new()),
        Some(Value::Array(a)) if a.is_empty() => return Ok(Vec::new()),
        Some(Value::Array(a)) => a,
        Some(_) => {
            return Err(Error::Invalid(vec![
                "The attributeServicesDirectories section must contain a list of directory paths."
                    .to_string(),
            ]));
        }
    };
    let mut errors = Vec::new();
    let mut directories = Vec::new();
    for entry in entries.values() {
        let Value::Str(entry) = entry else {
            errors.push(
                "The attributeServicesDirectories section must contain a list of directory paths."
                    .to_string(),
            );
            continue;
        };
        if entry.contains('%') {
            errors.push(format!("Entry {entry} in the attributeServicesDirectories section must be a plain path - % parameters are not supported."));
            continue;
        }
        if entry.starts_with('*') {
            errors.push(format!("Entry {entry} in the attributeServicesDirectories section must be a plain path - wildcards are not supported."));
            continue;
        }
        let dir = normalize_path(&absolutize_path(cwd, entry));
        directories.push(dir.trim_end_matches('/').to_string());
    }
    directories.sort_by_key(String::len);
    let mut kept: Vec<String> = Vec::new();
    for d in directories {
        if !kept
            .iter()
            .any(|k| d == *k || d.starts_with(&format!("{k}/")))
        {
            kept.push(d);
        }
    }
    kept.sort();
    for d in &kept {
        if !std::path::Path::new(d).is_dir() {
            errors.push(format!(
                "Directory {d} from the attributeServicesDirectories section does not exist."
            ));
        }
    }
    if errors.is_empty() {
        Ok(kept)
    } else {
        Err(Error::Invalid(vec![errors.join("\n\n")]))
    }
}

fn sys_get_temp_dir_calls(v: &mut Value, tmp: &str) {
    match v {
        Value::Entity(e)
            if e.value == Value::str("::sys_get_temp_dir") && e.attributes.is_empty() =>
        {
            *v = Value::Str(tmp.replace('%', "%%"));
        }
        Value::Array(a) => a.values_mut().for_each(|x| sys_get_temp_dir_calls(x, tmp)),
        _ => {}
    }
}

fn path_problem(path: &str) -> Option<String> {
    let exists = path.starts_with('/') && std::path::Path::new(path).exists();
    (!exists && !is_fnmatch_pattern(path)).then(|| {
        format!("Path \"{path}\" is neither a directory, nor a file path, nor a fnmatch pattern.")
    })
}

fn validate_exclude_paths(
    params: &mut Array,
    validate: bool,
    config: Option<&str>,
) -> Result<(), Error> {
    let exclude = params
        .get_str("excludePaths")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
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
                .map_or_default(|a| a.values().cloned().collect::<Vec<_>>())
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
            let _ = writeln!(out, "\t\t{key}:");
            for p in paths {
                let p = config.map_or_else(|| p.clone(), |c| parent_relative_path(&dirname(c), p));
                let _ = writeln!(out, "\t\t\t- {p} (?)");
            }
        }
        return Err(Error::InvalidExcludePaths(out));
    }
    let mut unwrapped = Array::new();
    for (k, paths) in new {
        let paths = paths
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|(_, p)| match p {
                Value::OptionalPath(p) => Value::Str(p),
                p => p,
            });
        unwrapped.set(k, Value::Array(Array::list(paths)));
    }
    params.set(Key::from("excludePaths"), Value::Array(unwrapped));
    Ok(())
}

fn ignore_errors(params: &Array) -> Array {
    params
        .get_str("ignoreErrors")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

fn validate_ignored_errors(params: &Array) -> Result<(), Error> {
    let ignore = ignore_errors(params);
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
    Err(Error::InvalidIgnoreErrors(out))
}

fn validate_php_version(params: &Array) -> Result<(), Error> {
    if let Some(Value::Array(v)) = params.get_str("phpVersion")
        && let (Some(Value::Int(min)), Some(Value::Int(max))) = (v.get_str("min"), v.get_str("max"))
        && max < min
    {
        return Err(Error::Invalid(vec!["Invalid PHP version range: phpVersion.max should be greater or equal to phpVersion.min.".into()]));
    }
    Ok(())
}

pub fn validate_ignore_errors(params: &Array) -> Result<(), Error> {
    validate_ignored_errors(params)?;
    validate_ignore_error_entries(params)
}

fn validate_ignore_error_entries(params: &Array) -> Result<(), Error> {
    let ignore = ignore_errors(params);
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
        if !at_least.iter().any(|k| has(k)) && !has("rule") {
            return Err(Error::Invalid(vec![format!(
                "An ignoreErrors entry must contain at least one of the following fields: {}.",
                at_least.join(", ")
            )]));
        }
        for group in [
            &["rawMessage", "rawMessages", "message", "messages"][..],
            &["identifier", "identifiers"],
            &["path", "paths"],
        ] {
            for (i, a) in group.iter().enumerate() {
                for b in &group[i + 1..] {
                    if has(a) && has(b) {
                        return Err(Error::Invalid(vec![format!(
                            "An ignoreErrors entry cannot contain both {a} and {b} fields."
                        )]));
                    }
                }
            }
        }
        if has("count") && !has("path") {
            return Err(Error::Invalid(vec![
                "An ignoreErrors entry with count field must also contain path field.".into(),
            ]));
        }
    }
    Ok(())
}
