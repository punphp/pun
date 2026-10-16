//! Nette DI's config machinery as PHPStan 2.2.7 uses it: `Nette\Schema\Helpers::merge`, `Nette\DI\Helpers::expand`
//! / `escape`, `Nette\DI\Config\Loader` (includes) and `PHPStan\DependencyInjection\NeonAdapter::process` (`!`
//! replacing, statements, relative paths, `(?)` optional excludePaths).

use crate::neon;
use crate::php::{absolutize_path, normalize_path};
use crate::value::{Array, Entity, Key, Value};

pub const PREVENT_MERGING: &str = "_prevent_merging";

fn prevent_key() -> Key {
    Key::Str(PREVENT_MERGING.into())
}

/// `isset($value['_prevent_merging'])`
fn has_prevent(a: &Array) -> bool {
    a.get(&prevent_key()).is_some_and(|v| *v != Value::Null)
}

/// `Nette\Schema\Helpers::merge($value, $base)`: `$value` wins; its list items are appended to `$base`.
pub fn merge(value: Value, base: Value) -> Value {
    match (value, base) {
        (Value::Array(v), Value::Array(b)) => Value::Array(merge_arrays(v, b)),
        (Value::Array(mut v), _) => {
            v.remove(&prevent_key());
            Value::Array(v)
        }
        (Value::Null, Value::Array(b)) => Value::Array(b),
        (v, _) => v,
    }
}

/// [`merge`] of two arrays.
pub fn merge_arrays(mut v: Array, mut b: Array) -> Array {
    if has_prevent(&v) {
        v.remove(&prevent_key());
        return v;
    }
    let mut index = 0;
    for (k, val) in v {
        if k == Key::Int(index) {
            b.push(val);
            index += 1;
        } else {
            let old = b.get_mut(&k).map(std::mem::take).unwrap_or(Value::Null);
            b.set(k, merge(val, old));
        }
    }
    b
}

/// `Nette\DI\Helpers::escape()`.
pub fn escape(v: Value) -> Value {
    match v {
        Value::Array(a) => {
            let mut res = Array::new();
            for (k, x) in a {
                let k = match k {
                    Key::Str(s) => Key::Str(s.replace('%', "%%")),
                    k => k,
                };
                res.set(k, escape(x));
            }
            Value::Array(res)
        }
        Value::Str(s) => {
            let s = s.replace('%', "%%");
            Value::Str(if s.starts_with('@') {
                format!("@{s}")
            } else {
                s
            })
        }
        v => v,
    }
}

/// Only `%` doubling: for values PHPStan passes as runtime (dynamic) parameters, which expansion never touches.
pub fn escape_percent(v: Value) -> Value {
    match v {
        Value::Array(a) => {
            let mut res = Array::new();
            for (k, x) in a {
                res.set(k, escape_percent(x));
            }
            Value::Array(res)
        }
        Value::Str(s) => Value::Str(s.replace('%', "%%")),
        v => v,
    }
}

fn statement(entity: Value, arguments: Array) -> Value {
    Value::Entity(Box::new(Entity {
        value: entity,
        attributes: arguments,
    }))
}

/// `Nette\DI\Helpers::expand($var, $params, $recursive)`; `recursive` is `None` for a plain expansion.
pub fn expand(var: &Value, params: &Array, recursive: Option<&[String]>) -> Result<Value, String> {
    match var {
        Value::Array(a) => Ok(Value::Array(expand_array(a, params, recursive)?)),
        Value::Entity(e) => Ok(statement(
            expand(&e.value, params, recursive)?,
            expand_array(&e.attributes, params, recursive)?,
        )),
        Value::Str(s) if s == "%parameters%" && !params.contains(&Key::from("parameters")) => {
            if recursive.is_some() {
                expand(&Value::Array(params.clone()), params, recursive)
            } else {
                Ok(Value::Array(params.clone()))
            }
        }
        Value::Str(s) => expand_string(s, params, recursive, false),
        v => Ok(v.clone()),
    }
}

/// `preg_split('#%([\w.-]*)%#i', $s, -1, PREG_SPLIT_DELIM_CAPTURE)`
fn split_placeholders(s: &str) -> Vec<&str> {
    let b = s.as_bytes();
    let mut parts = Vec::new();
    let (mut last, mut i) = (0, 0);
    while i < b.len() {
        if b[i] == b'%' {
            let mut j = i + 1;
            while j < b.len() && (b[j].is_ascii_alphanumeric() || b"_.-".contains(&b[j])) {
                j += 1;
            }
            if j < b.len() && b[j] == b'%' {
                parts.push(&s[last..i]);
                parts.push(&s[i + 1..j]);
                last = j + 1;
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    parts.push(&s[last..]);
    parts
}

fn expand_string(
    s: &str,
    params: &Array,
    recursive: Option<&[String]>,
    only_string: bool,
) -> Result<Value, String> {
    let parts = split_placeholders(s);
    let mut res: Vec<Value> = Vec::new();
    let mut dynamic = false;
    for (n, part) in parts.iter().enumerate() {
        if n % 2 == 0 {
            res.push(Value::str(*part));
        } else if part.is_empty() {
            res.push(Value::str("%"));
        } else {
            let val = expand_parameter(part, params, recursive, only_string)?;
            if part.len() + 2 == s.len() {
                return Ok(val);
            } else if matches!(val, Value::Entity(_)) {
                dynamic = true;
            } else if !val.is_scalar() {
                return Err(format!(
                    "Unable to concatenate non-scalar parameter '{part}' into '{s}'."
                ));
            }
            res.push(val);
        }
    }
    if dynamic {
        return Ok(statement(
            Value::str("::implode"),
            Array::list([Value::str(""), Value::Array(Array::list(res))]),
        ));
    }
    Ok(Value::Str(
        res.iter()
            .map(|v| v.to_php_string().unwrap_or_default())
            .collect(),
    ))
}

fn expand_parameter(
    parameter: &str,
    params: &Array,
    recursive: Option<&[String]>,
    only_string: bool,
) -> Result<Value, String> {
    let mut val = Value::Array(params.clone());
    let mut path: Vec<&str> = Vec::new();
    let keys: Vec<&str> = parameter.split('.').collect();
    for (i, key) in keys.iter().enumerate() {
        path.push(key);
        let found = match &val {
            Value::Array(a) => a.get(&Key::from(*key)).cloned(),
            _ => None,
        };
        if let Some(v) = found {
            val = v;
            let full_expand = !only_string && i == keys.len() - 1;
            if let Some(rec) = recursive
                && (full_expand || matches!(val, Value::Str(_)))
            {
                let path_str = path.join(".");
                if rec.contains(&path_str) {
                    return Err(format!(
                        "Circular reference detected for parameters: %{}%",
                        rec.join("%, %")
                    ));
                }
                let mut rec = rec.to_vec();
                rec.push(path_str);
                val = match &val {
                    Value::Str(s) if !full_expand => expand_string(s, params, Some(&rec), true)?,
                    v => expand(v, params, Some(&rec))?,
                };
            }
        } else if matches!(val, Value::Entity(_)) {
            val = statement(Value::str("(?)[?]"), Array::list([val, Value::str(*key)]));
        } else {
            return Err(format!("Missing parameter '{parameter}'."));
        }
    }
    Ok(val)
}

/// [`expand`] of an array (keys are expanded too).
pub fn expand_array(
    a: &Array,
    params: &Array,
    recursive: Option<&[String]>,
) -> Result<Array, String> {
    let mut res = Array::new();
    for (k, v) in a.iter() {
        let k = match k {
            Key::Str(s) => match expand_string(s, params, recursive, false)? {
                Value::Int(i) => Key::Int(i),
                other => Key::from_string(other.to_php_string().unwrap_or_default()),
            },
            k => k.clone(),
        };
        res.set(k, expand(v, params, recursive)?);
    }
    Ok(res)
}

/// PHP `dirname()` for `/`-separated paths.
pub fn dirname(path: &str) -> String {
    // (only ever given absolute file paths: no trailing slash, no bare name)
    let parent = std::path::Path::new(path)
        .parent()
        .map(|p| p.to_string_lossy().into_owned());
    parent.unwrap_or_else(|| "/".into())
}

/// Source of config files: the filesystem, plus PHPStan's own `conf/` inside the PHAR (embedded in Pun).
pub trait Files {
    fn read(&self, path: &str) -> Option<Vec<u8>>;
    fn is_file(&self, path: &str) -> bool {
        self.read(path).is_some()
    }
}

/// `PHPStan\DependencyInjection\NeonAdapter::load()`.
pub fn load_neon(
    files: &dyn Files,
    file: &str,
    expand_relative: &[String],
) -> Result<Array, String> {
    let contents = files
        .read(file)
        .ok_or_else(|| format!("File '{file}' is missing or is not readable."))?;
    let decoded = neon::decode_bytes(&contents)
        .map_err(|e| format!("Error while loading {file}: {}", e.0))?;
    let arr = match decoded {
        Value::Array(a) => a,
        Value::Null => Array::new(),
        other => Array::list([other]),
    };
    process(arr, "", file, expand_relative)
}

const EXCLUDE_PATH_KEYS: [&str; 3] = [
    "[parameters][excludePaths][]",
    "[parameters][excludePaths][analyse][]",
    "[parameters][excludePaths][analyseAndScan][]",
];

fn relative_to(file: &str, path: &str) -> String {
    let dir = normalize_path(&dirname(file));
    normalize_path(&absolutize_path(&dir, path))
}

fn process(
    arr: Array,
    file_key: &str,
    file: &str,
    expand_relative: &[String],
) -> Result<Array, String> {
    let mut res = Array::new();
    for (key, mut val) in arr {
        let mut key = key;
        if let Key::Str(s) = &key
            && let Some(stripped) = s.strip_suffix('!')
        {
            let stripped = stripped.to_string();
            match &mut val {
                Value::Array(a) => a.set(prevent_key(), Value::Bool(true)),
                Value::Null => {
                    let mut a = Array::new();
                    a.set(prevent_key(), Value::Bool(true));
                    val = Value::Array(a);
                }
                _ => {
                    return Err(format!(
                        "Replacing operator is available only for arrays, item '{s}' is not array."
                    ));
                }
            }
            key = Key::from_string(stripped);
        }
        let int_key = matches!(key, Key::Int(_));
        let segment = |open: char, close: char| match &key {
            Key::Int(_) => format!("{file_key}{open}{close}"),
            Key::Str(s) => format!("{file_key}{open}{s}{close}"),
        };
        let key_to_resolve = segment('[', ']');
        val = match val {
            Value::Array(a) => Value::Array(process(a, &segment('[', ']'), file, expand_relative)?),
            Value::Entity(e) => {
                let pass = if int_key {
                    format!("{file_key}()")
                } else {
                    segment('(', ')')
                };
                // (NeonAdapter folds `a()::b()` chains into nested statements; Pun only reads parameters, where any
                // statement ends resolution (see config.rs), so chains stay as decoded.)
                if EXCLUDE_PATH_KEYS.contains(&key_to_resolve.as_str())
                    && e.attributes.len() == 1
                    && e.attributes.get(&Key::Int(0)) == Some(&Value::str("?"))
                    && matches!(&e.value, Value::Str(s) if !s.contains('%') && !s.starts_with('*'))
                {
                    Value::OptionalPath(relative_to(file, e.value.as_str().expect("string")))
                } else {
                    let tmp = process(Array::list([e.value]), &pass, file, expand_relative)?;
                    let entity = tmp
                        .into_iter()
                        .next()
                        .map(|(_, v)| v)
                        .unwrap_or(Value::Null);
                    statement(entity, process(e.attributes, &pass, file, expand_relative)?)
                }
            }
            v => v,
        };
        if expand_relative.contains(&key_to_resolve)
            && let Value::Str(s) = &val
            && !s.contains('%')
            && !s.starts_with('*')
        {
            val = Value::Str(relative_to(file, s));
        }
        if key_to_resolve == "[parameters][excludePaths]"
            && let Value::Array(a) = &val
            && a.is_list()
        {
            let mut wrapped = Array::new();
            wrapped.set(Key::from("analyseAndScan"), val);
            wrapped.set(Key::from("analyse"), Value::Array(Array::new()));
            val = Value::Array(wrapped);
        }
        res.set(key, val);
    }
    Ok(res)
}

/// `Nette\Utils\Validators::assert($value, 'list', $label)` message.
pub fn assert_list(v: &Value, label: &str) -> Result<(), String> {
    if let Value::Array(a) = v
        && a.is_list()
    {
        return Ok(());
    }
    let ty = match v {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "bool".to_string(),
        Value::Int(_) | Value::Float(_) => format!(
            "{} {}",
            if matches!(v, Value::Int(_)) {
                "int"
            } else {
                "float"
            },
            v.var_export()
        ),
        Value::Str(s) if s.len() < 40 => format!("string {}", v.var_export()),
        Value::Str(_) => "string".to_string(),
        Value::Array(_) => "array".to_string(),
        // (the phar's scoped class name)
        _ => "object _PHPStan_88a9c7bcc\\Nette\\DI\\Definitions\\Statement".to_string(),
    };
    Err(format!("The {label} expects to be list, {ty} given."))
}

/// `Loader::expandIncludedFile()` / `ContainerFactory::expandIncludedFile()`: `#([a-z]+:)?[/\\\\]#Ai`.
pub fn expand_included_file(included: &str, main: &str) -> String {
    let rest = match included.find(':') {
        Some(i) if i > 0 && included[..i].bytes().all(|c| c.is_ascii_alphabetic()) => {
            &included[i + 1..]
        }
        _ => included,
    };
    if rest.starts_with(['/', '\\']) || included.starts_with(['/', '\\']) {
        included.to_string()
    } else {
        format!("{}/{included}", dirname(main))
    }
}

/// Reads one config file: `.neon`/`.dist` through PHPStan's NeonAdapter, `.php` through the baseline-shaped PHP
/// reader (anything else in a `.php` config needs PHP).
pub fn load_file(
    files: &dyn Files,
    file: &str,
    expand_relative: &[String],
) -> Result<Array, String> {
    let ext = file
        .rsplit_once('.')
        .map(|(_, e)| e.to_ascii_lowercase())
        .filter(|e| !e.contains('/'))
        .unwrap_or_default();
    match ext.as_str() {
        "neon" | "dist" => load_neon(files, file, expand_relative),
        "php" => {
            let contents = files
                .read(file)
                .ok_or_else(|| format!("File '{file}' is missing or is not readable."))?;
            crate::phpfile::load_config(&String::from_utf8_lossy(&contents), &dirname(file))
                .map_err(|e| format!("Unsupported PHP config {file}: {e}"))
        }
        _ => Err(format!("Unknown file extension '{file}'.")),
    }
}

/// `Nette\DI\Config\Loader` with PHPStan's `NeonLoader`: returns every loaded file's data, includes first (the
/// `$merge = false` form the compiler consumes).
pub struct Loader<'a> {
    pub files: &'a dyn Files,
    pub parameters: Array,
    pub expand_relative: Vec<String>,
    pub generate_baseline: Option<String>,
    loaded: Vec<String>,
}

impl<'a> Loader<'a> {
    pub fn new(
        files: &'a dyn Files,
        parameters: Array,
        expand_relative: Vec<String>,
        generate_baseline: Option<String>,
    ) -> Loader<'a> {
        Loader {
            files,
            parameters,
            expand_relative,
            generate_baseline,
            loaded: Vec::new(),
        }
    }

    /// `Loader::load($file, false)`.
    pub fn load(&mut self, file: &str) -> Result<Vec<Array>, String> {
        self.load_inner(file, false)
    }

    /// `Loader::load($file)` (merged).
    pub fn load_merged(&mut self, file: &str) -> Result<Array, String> {
        Ok(self.load_inner(file, true)?.pop().unwrap_or_default())
    }

    fn load_inner(&mut self, file: &str, merged: bool) -> Result<Vec<Array>, String> {
        if self.generate_baseline.as_deref() == Some(normalize_path(file).as_str()) {
            return Ok(Vec::new());
        }
        if !self.files.is_file(file) {
            return Err(format!("File '{file}' is missing or is not readable."));
        }
        if self.loaded.iter().any(|f| f == file) {
            return Err(format!("Recursive included file '{file}'"));
        }
        self.loaded.push(file.to_string());
        let mut data = load_file(self.files, file, &self.expand_relative)?;
        let mut list = Vec::new();
        let mut res = Array::new();
        if let Some(includes) = data
            .get_str("includes")
            .filter(|v| **v != Value::Null)
            .cloned()
        {
            assert_list(&includes, &format!("section 'includes' in file '{file}'"))?;
            for include in expand(&includes, &self.parameters, None)?
                .as_array()
                .expect("a list")
                .values()
            {
                let include =
                    expand_included_file(&include.to_php_string().unwrap_or_default(), file);
                let loaded = self.load_inner(&include, merged)?;
                if merged {
                    res = merge_arrays(loaded.into_iter().next().unwrap_or_default(), res);
                } else {
                    list.extend(loaded);
                }
            }
        }
        data.remove(&Key::from("includes"));
        self.loaded.retain(|f| f != file);
        if merged {
            return Ok(vec![merge_arrays(data, res)]);
        }
        list.push(data);
        Ok(list)
    }
}
