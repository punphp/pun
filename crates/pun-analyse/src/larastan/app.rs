//! What Larastan's extensions ask the booted Laravel app (`HasContainer::resolve()`, the `config` repository,
//! `now()`), read from the installed framework and the app's sources instead (Pun never runs PHP).
//!
//! - The container: `Application::registerCoreContainerAliases()`, each alias resolving to its key's first class.
//!   A key whose classes are all interfaces resolves to what the framework's provider binds by default:
//!   `auth.driver` to the default guard's driver (GuardDynamicStaticMethodReturnTypeExtension's
//!   `findTypeFromGuardDriver()`), `filesystem.disk` and `filesystem.cloud` to a `FilesystemAdapter`. Bindings that
//!   service providers add are not seen.
//! - The `auth` config: LoadConfiguration's framework `config/auth.php` merged with the app's (`guards`,
//!   `providers` and `passwords` merged again), or `bootstrap/cache/config.php`; `env()` reads `.env`.
//! - `get_class(now())`: `Illuminate\Support\Carbon`, unless a service provider in `app/Providers` calls
//!   `Date::use(X::class)` (DateFactory::use()).

use std::collections::HashMap;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_database::file::FileType;
use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use crate::codebase::{parse, read};

/// A config value as far as a literal reader can tell.
#[derive(Clone, Debug, PartialEq)]
enum Value {
    Array(Vec<(String, Value)>),
    String(String),
    Null,
    Unknown,
}

impl Value {
    fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Array(items) => items.iter().rev().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// PHP's truthiness of a string (`! $provider`): neither empty nor `'0'`.
    fn truthy_str(&self) -> Option<&str> {
        self.as_str().filter(|s| !s.is_empty() && *s != "0")
    }

    /// `array_merge()` of two string-keyed arrays.
    fn merge(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Array(a), Value::Array(b)) => {
                let mut items = a.clone();
                for (k, v) in b {
                    match items.iter_mut().find(|(key, _)| key == k) {
                        Some(item) => item.1 = v.clone(),
                        None => items.push((k.clone(), v.clone())),
                    }
                }
                Value::Array(items)
            }
            _ => Value::Unknown,
        }
    }
}

/// The app as Larastan's extensions see it.
#[derive(Clone, Debug, Default)]
pub(crate) struct App {
    /// Lowercase alias or class name to the class the container makes for it.
    container: HashMap<String, String>,
    /// The `auth` config.
    auth: Option<Value>,
    /// `get_class(now())`.
    pub(crate) date_class: String,
    /// `version_compare(LARAVEL_VERSION, '12.0.0', '>=')`.
    pub(crate) laravel_12: bool,
}

impl App {
    pub(crate) fn load(root: &Path) -> App {
        let framework = root.join("vendor/laravel/framework");
        let env = dot_env(root);
        let auth = auth_config(root, &framework, &env);
        let mut app = App {
            container: HashMap::new(),
            auth,
            date_class: date_class(root).unwrap_or_else(|| "Illuminate\\Support\\Carbon".to_string()),
            laravel_12: super::stubs::laravel_version(root).is_some_and(|v| super::stubs::compare(&v, "12.0.0").is_ge()),
        };
        let aliases = core_aliases(&framework.join("src/Illuminate/Foundation/Application.php"));
        for (key, classes) in aliases {
            let concrete = match key.as_str() {
                "auth.driver" => app.guard_class(None).map(str::to_string),
                "filesystem.disk" | "filesystem.cloud" => {
                    Some("Illuminate\\Filesystem\\FilesystemAdapter".to_string())
                }
                _ => classes.first().cloned(),
            };
            let Some(concrete) = concrete else { continue };
            for class in classes.iter().chain([&key]) {
                app.container.insert(class.to_ascii_lowercase(), concrete.clone());
            }
        }
        app
    }

    /// `HasContainer::resolve()`: the class of what the container makes for `abstract`, `None` when it can't.
    /// The caller checks that the class exists.
    pub(crate) fn resolve(&self, abstract_: &str) -> Option<&str> {
        self.container
            .get(&abstract_.trim_start_matches('\\').to_ascii_lowercase())
            .map(String::as_str)
    }

    /// ContractsMethodsExtension: an `Illuminate\Contracts` interface has the methods of the class the container makes
    /// for it, which Mago's method resolution reaches as a mixin of that class (one Mago has loaded).
    pub(crate) fn contract_mixins(&self, metadata: &mut mago_codex::metadata::CodebaseMetadata) {
        use mago_codex::metadata::ttype::TypeMetadata;
        use mago_codex::ttype::atomic::TAtomic;
        use mago_codex::ttype::atomic::object::{TObject, named::TNamedObject};
        use mago_codex::ttype::union::TUnion;
        let concretes: Vec<_> = metadata
            .class_likes
            .iter()
            .filter(|(_, k)| k.kind.is_interface() && k.original_name.as_bytes().starts_with(b"Illuminate\\Contracts\\"))
            .filter_map(|(key, k)| {
                let concrete = self.resolve(&String::from_utf8_lossy(k.original_name.as_bytes()))?;
                let concrete = metadata.get_class_like(concrete.as_bytes())?;
                (concrete.name != k.name).then(|| (*key, concrete.original_name))
            })
            .collect();
        for (key, concrete) in concretes {
            if let Some(k) = metadata.class_likes.get_mut(&key) {
                let object = TAtomic::Object(TObject::Named(TNamedObject::new(concrete)));
                k.mixins.push(TypeMetadata::new(TUnion::from_atomic(object), k.span));
            }
        }
    }

    /// LoadsAuthModel::getAuthModels(): the models of the guard's provider, or of every guard's.
    pub(crate) fn auth_models(&self, guard: Option<&str>) -> Vec<String> {
        let Some(auth) = &self.auth else { return Vec::new() };
        let (Some(Value::Array(guards)), Some(providers @ Value::Array(_))) =
            (auth.get("guards"), auth.get("providers"))
        else {
            return Vec::new();
        };
        let names: Vec<&str> = match guard {
            Some(g) => vec![g],
            None => guards.iter().map(|(k, _)| k.as_str()).collect(),
        };
        let mut models: Vec<String> = Vec::new();
        for name in names {
            let Some(provider) = auth.get("guards").and_then(|g| g.get(name)?.get("provider")?.truthy_str()) else {
                continue;
            };
            let Some(model) = providers.get(provider).and_then(|p| p.get("model")?.truthy_str()) else {
                continue;
            };
            if !models.iter().any(|m| m == model) {
                models.push(model.to_string());
            }
        }
        models
    }

    /// GuardDynamicStaticMethodReturnTypeExtension::findTypeFromGuardDriver() of the guard's driver (the default
    /// guard's for `None`).
    pub(crate) fn guard_class(&self, guard: Option<&str>) -> Option<&'static str> {
        let auth = self.auth.as_ref()?;
        let guard = match guard {
            Some(g) => g,
            None => auth.get("defaults")?.get("guard")?.as_str()?,
        };
        driver_class(auth.get("guards")?.get(guard)?.get("driver")?.as_str()?)
    }
}

/// GuardDynamicStaticMethodReturnTypeExtension::findTypeFromGuardDriver().
pub(crate) fn driver_class(driver: &str) -> Option<&'static str> {
    match driver {
        "session" => Some("Illuminate\\Auth\\SessionGuard"),
        "token" => Some("Illuminate\\Auth\\TokenGuard"),
        "passport" => Some("Illuminate\\Auth\\RequestGuard"),
        _ => None,
    }
}

/// `registerCoreContainerAliases()`'s keys and their classes.
fn core_aliases(path: &Path) -> Vec<(String, Vec<String>)> {
    let Some(file) = read(path, FileType::Vendored).ok().flatten() else {
        return Vec::new();
    };
    let arena = LocalArena::new();
    let (program, names) = parse(&arena, &file);
    struct Find<'n, 'arena> {
        names: &'n ResolvedNames<'arena>,
        found: Vec<(String, Vec<String>)>,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Find<'_, 'arena> {
        fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
            if !m.name.value.eq_ignore_ascii_case(b"registerCoreContainerAliases") {
                return;
            }
            let MethodBody::Concrete(body) = &m.body else { return };
            for s in body.statements.iter() {
                let Statement::Foreach(f) = s else { continue };
                let Expression::Array(a) = f.expression else { continue };
                for e in a.elements.iter() {
                    let ArrayElement::KeyValue(kv) = e else { continue };
                    let Expression::Literal(Literal::String(key)) = kv.key else { continue };
                    let Some(key) = key.value else { continue };
                    let Expression::Array(classes) = kv.value else { continue };
                    let classes = classes
                        .elements
                        .iter()
                        .filter_map(|c| match c {
                            ArrayElement::Value(v) => class_constant(self.names, v.value),
                            _ => None,
                        })
                        .collect();
                    self.found.push((String::from_utf8_lossy(key).into_owned(), classes));
                }
            }
        }
    }
    let mut find = Find { names: &names, found: Vec::new() };
    find.walk_program(program, &mut ());
    find.found
}

/// The class of `X::class` (`self::class` is the Application).
fn class_constant(names: &ResolvedNames, e: &Expression) -> Option<String> {
    let Expression::Access(Access::ClassConstant(c)) = e else { return None };
    let ClassLikeConstantSelector::Identifier(k) = &c.constant else { return None };
    if !k.value.eq_ignore_ascii_case(b"class") {
        return None;
    }
    match c.class {
        Expression::Identifier(id) => Some(String::from_utf8_lossy(names.get(&id.span())).into_owned()),
        Expression::Self_(_) => Some("Illuminate\\Foundation\\Application".to_string()),
        _ => None,
    }
}

/// The `.env` file's variables (Dotenv's simple `KEY=value` lines, quotes removed).
fn dot_env(root: &Path) -> HashMap<String, String> {
    let Ok(text) = std::fs::read_to_string(root.join(".env")) else {
        return HashMap::new();
    };
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            let line = line.strip_prefix("export ").unwrap_or(line);
            if line.starts_with('#') {
                return None;
            }
            let (k, v) = line.split_once('=')?;
            let v = v.trim();
            let v = match v.as_bytes().first() {
                Some(q @ (b'"' | b'\'')) => {
                    let rest = &v[1..];
                    rest[..rest.find(*q as char).unwrap_or(rest.len())].to_string()
                }
                _ => v.split(" #").next().unwrap_or(v).trim().to_string(),
            };
            Some((k.trim().to_string(), v))
        })
        .collect()
}

/// The `auth` config LoadConfiguration builds.
fn auth_config(root: &Path, framework: &Path, env: &HashMap<String, String>) -> Option<Value> {
    let cached = root.join("bootstrap/cache/config.php");
    if cached.is_file() {
        return returned(&cached, env)?.get("auth").cloned();
    }
    let base = returned(&framework.join("config/auth.php"), env);
    let app = root.join("config/auth.php");
    let app = if app.is_file() { returned(&app, env) } else { None };
    match (base, app) {
        (Some(base), Some(app)) => {
            let mut config = base.merge(&app);
            for option in ["guards", "providers", "passwords"] {
                if let (Some(b), Some(a)) = (base.get(option), app.get(option)) {
                    let merged = b.merge(a);
                    if let Value::Array(items) = &mut config {
                        for item in items.iter_mut().filter(|(k, _)| k == option) {
                            item.1 = merged.clone();
                        }
                    }
                }
            }
            Some(config)
        }
        (base, app) => app.or(base),
    }
}

/// A config file's top-level `return`.
fn returned(path: &Path, env: &HashMap<String, String>) -> Option<Value> {
    let file = read(path, FileType::Vendored).ok()??;
    let arena = LocalArena::new();
    let (program, names) = parse(&arena, &file);
    program.statements.iter().find_map(|s| match s {
        Statement::Return(r) => r.value.map(|e| value(e, &names, env)),
        _ => None,
    })
}

fn value(e: &Expression, names: &ResolvedNames, env: &HashMap<String, String>) -> Value {
    match e {
        Expression::Literal(Literal::String(s)) => {
            s.value.map(|s| Value::String(String::from_utf8_lossy(s).into_owned())).unwrap_or(Value::Unknown)
        }
        Expression::Literal(Literal::Null(_)) => Value::Null,
        Expression::Access(Access::ClassConstant(_)) => {
            class_constant(names, e).map(Value::String).unwrap_or(Value::Unknown)
        }
        Expression::Parenthesized(p) => value(p.expression, names, env),
        Expression::Array(a) => array(&a.elements, names, env),
        Expression::LegacyArray(a) => array(&a.elements, names, env),
        Expression::Call(Call::Function(call)) => {
            let Expression::Identifier(id) = call.function else { return Value::Unknown };
            let name = names.get(&id.span());
            let name = name.rsplit(|&b| b == b'\\').next().unwrap_or(name);
            if !name.eq_ignore_ascii_case(b"env") {
                return Value::Unknown;
            }
            let args = &call.argument_list.arguments;
            let Some(Argument::Positional(key)) = args.first() else { return Value::Unknown };
            let Value::String(key) = value(key.value, names, env) else { return Value::Unknown };
            match env.get(&key) {
                // Env::get(): the `true`, `false`, `null` and `empty` strings (parenthesized too) are converted
                Some(v) => match v.to_ascii_lowercase().trim_matches(|c| c == '(' || c == ')') {
                    "null" => Value::Null,
                    "empty" => Value::String(String::new()),
                    "true" | "false" => Value::Unknown,
                    _ => Value::String(v.clone()),
                },
                None => match args.get(1) {
                    Some(Argument::Positional(d)) => value(d.value, names, env),
                    _ => Value::Null,
                },
            }
        }
        _ => Value::Unknown,
    }
}

fn array(
    elements: &TokenSeparatedSequence<ArrayElement>,
    names: &ResolvedNames,
    env: &HashMap<String, String>,
) -> Value {
    let mut items = Vec::new();
    let mut next = 0;
    for element in elements.iter() {
        match element {
            ArrayElement::KeyValue(kv) => {
                let Value::String(key) = value(kv.key, names, env) else {
                    return Value::Unknown;
                };
                items.push((key, value(kv.value, names, env)));
            }
            ArrayElement::Value(v) => {
                items.push((next.to_string(), value(v.value, names, env)));
                next += 1;
            }
            _ => return Value::Unknown,
        }
    }
    Value::Array(items)
}

/// The class a service provider passes to `Date::use()`, the last one found.
fn date_class(root: &Path) -> Option<String> {
    let mut found = None;
    let mut files: Vec<_> = std::fs::read_dir(root.join("app/Providers"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "php"))
        .collect();
    files.sort();
    for path in files {
        let Some(file) = read(&path, FileType::Host).ok().flatten() else { continue };
        let arena = LocalArena::new();
        let (program, names) = parse(&arena, &file);
        struct Find<'n, 'arena> {
            names: &'n ResolvedNames<'arena>,
            found: Option<String>,
        }
        impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Find<'_, 'arena> {
            fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
                let Expression::Identifier(class) = call.class else { return };
                let class = self.names.get(&class.span());
                let date = class.eq_ignore_ascii_case(b"Illuminate\\Support\\Facades\\Date")
                    || class.eq_ignore_ascii_case(b"Illuminate\\Support\\DateFactory");
                let ClassLikeMemberSelector::Identifier(m) = &call.method else { return };
                if date
                    && (m.value.eq_ignore_ascii_case(b"use") || m.value.eq_ignore_ascii_case(b"useClass"))
                    && let Some(Argument::Positional(arg)) = call.argument_list.arguments.first()
                    && let Some(class) = class_constant(self.names, arg.value)
                {
                    self.found = Some(class);
                }
            }
        }
        let mut find = Find { names: &names, found: None };
        find.walk_program(program, &mut ());
        found = find.found.or(found);
    }
    found
}
