//! RegisterFacades' alias identities, without booting PHP. The installed Facade::defaultAliases(), app config
//! (a replacement array or the standard defaults/merge/toArray chain), config cache, and PackageManifest are the
//! sources of truth. Package aliases override app aliases. An unreadable/dynamic value contributes no aliases.
//!
//! This does not resolve facade accessors or forward methods to container services. Programmatic AliasLoader
//! changes, custom bootstrap/config/cache/vendor paths and real-time `Facades\...` generation need more than this
//! literal reader; never guess their aliases. The normal codex scanner/populator owns class_alias identity.
//! Its real declarations win, matching an already-loaded native class. Runtime load order (including an alias
//! beating an unloaded real class, or differently cased aliases to different targets) is not simulated here.

use std::collections::BTreeMap;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_database::file::FileType;
use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use serde_json::Value as Json;

use crate::codebase::{parse, read};

type Aliases = BTreeMap<String, String>;

#[derive(Clone)]
enum Value {
    Array(Vec<(String, Value)>),
    String(String),
    Unknown,
}

impl Value {
    fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Array(items) => items.iter().rev().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    fn aliases(&self) -> Aliases {
        let mut aliases = Aliases::new();
        match self {
            Value::Array(items) => {
                for (k, v) in items {
                    if let Value::String(v) = v
                        && !v.is_empty()
                        && v != "0"
                    {
                        aliases.insert(k.clone(), v.clone());
                    } else {
                        aliases.remove(k);
                    }
                }
            }
            _ => {}
        }
        aliases
    }
}

/// Alias spelling is retained: AliasLoader's cold autoload lookup is case-sensitive, unlike PHP's loaded classes.
pub(crate) fn load(root: &Path) -> Vec<(String, String)> {
    let installed = json(&root.join("vendor/composer/installed.json"));
    let packages = installed.get("packages").unwrap_or(&installed).as_array();
    let framework = packages
        .into_iter()
        .flatten()
        .find(|p| p["name"] == "laravel/framework")
        .and_then(|p| p["install-path"].as_str())
        .map(|p| root.join("vendor/composer").join(p))
        .unwrap_or_else(|| root.join("vendor/laravel/framework"));
    let defaults = returned(
        &framework.join("src/Illuminate/Support/Facades/Facade.php"),
        &[],
        true,
    )
    .unwrap_or(Value::Unknown);
    let defaults = match defaults {
        Value::Array(a) => a,
        _ => Vec::new(),
    };
    let cached = root.join("bootstrap/cache/config.php");
    let mut aliases = if cached.is_file() {
        returned(&cached, &defaults, false)
            .and_then(|v| v.get("app")?.get("aliases").cloned())
            .unwrap_or(Value::Unknown)
            .aliases()
    } else {
        let app = root.join("config/app.php");
        let app = if app.is_file() {
            returned(&app, &defaults, false).unwrap_or(Value::Unknown)
        } else {
            Value::Array(Vec::new())
        };
        match app {
            Value::Unknown => Aliases::new(),
            _ => app
                .get("aliases")
                .cloned()
                .unwrap_or_else(|| {
                    returned(&framework.join("config/app.php"), &defaults, false)
                        .and_then(|v| v.get("aliases").cloned())
                        .unwrap_or(Value::Unknown)
                })
                .aliases(),
        }
    };
    let manifest = root.join("bootstrap/cache/packages.php");
    let mut package_aliases = Vec::new();
    if manifest.is_file() {
        if let Some(Value::Array(packages)) = returned(&manifest, &defaults, false) {
            for (_, package) in packages {
                if let Some(Value::Array(a)) = package.get("aliases") {
                    package_aliases.extend(a.clone());
                }
            }
        }
    } else {
        let composer = json(&root.join("composer.json"));
        let mut ignored = strings(&composer["extra"]["laravel"]["dont-discover"]);
        let ignore_all = ignored.iter().any(|p| p == "*");
        // PackageManifest collects every package's exclusions before rejecting any package.
        // Its wildcard switch is computed from the root package only, before those additions.
        for package in packages.into_iter().flatten() {
            ignored.extend(strings(&package["extra"]["laravel"]["dont-discover"]));
        }
        for package in packages.into_iter().flatten() {
            if ignore_all || ignored.iter().any(|p| package["name"].as_str() == Some(p)) {
                continue;
            }
            for (alias, target) in package["extra"]["laravel"]["aliases"]
                .as_object()
                .into_iter()
                .flatten()
            {
                package_aliases.push((
                    alias.clone(),
                    target
                        .as_str()
                        .map(|s| Value::String(s.to_string()))
                        .unwrap_or(Value::Unknown),
                ));
            }
        }
    }
    aliases.extend(Value::Array(package_aliases).aliases());
    aliases.into_iter().collect()
}

fn json(path: &Path) -> Json {
    std::fs::read(path)
        .ok()
        .and_then(|s| serde_json::from_slice(&s).ok())
        .unwrap_or(Json::Null)
}

fn strings(value: &Json) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Json::as_str)
        .map(str::to_string)
        .collect()
}

fn returned(path: &Path, defaults: &[(String, Value)], method: bool) -> Option<Value> {
    let file = read(path, FileType::Vendored).ok()??;
    let arena = LocalArena::new();
    let (program, names) = parse(&arena, &file);
    if program.has_errors() {
        return None;
    }
    if method {
        struct DefaultAliases<'a, 'arena> {
            names: &'a ResolvedNames<'arena>,
            value: Option<Value>,
        }
        impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for DefaultAliases<'_, 'arena> {
            fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
                if m.name.value.eq_ignore_ascii_case(b"defaultAliases")
                    && let MethodBody::Concrete(b) = &m.body
                    && let [Statement::Return(r)] = b.statements.as_slice()
                    && let Some(Expression::Instantiation(i)) = r.value
                    && let Expression::Identifier(id) = i.class
                    && self
                        .names
                        .get(&id.span())
                        .eq_ignore_ascii_case(b"Illuminate\\Support\\Collection")
                    && let Some(args) = &i.argument_list
                    && let [Argument::Positional(arg)] = args.arguments.as_slice()
                {
                    self.value = Some(value(arg.value, self.names, &[]));
                }
            }
        }
        let mut found = DefaultAliases {
            names: &names,
            value: None,
        };
        found.walk_program(program, &mut ());
        found.value
    } else {
        // Only a top-level return: do not take a closure's return or a branch of dynamic bootstrap code.
        program.statements.iter().find_map(|s| match s {
            Statement::Return(r) => r.value.map(|e| value(e, &names, defaults)),
            _ => None,
        })
    }
}

fn value(e: &Expression, names: &ResolvedNames, defaults: &[(String, Value)]) -> Value {
    match e {
        Expression::Literal(Literal::String(s)) => s
            .value
            .map(|s| Value::String(String::from_utf8_lossy(s).into_owned()))
            .unwrap_or(Value::Unknown),
        Expression::Access(Access::ClassConstant(c)) => {
            if let Expression::Identifier(id) = c.class
                && let ClassLikeConstantSelector::Identifier(k) = &c.constant
                && k.value.eq_ignore_ascii_case(b"class")
            {
                Value::String(String::from_utf8_lossy(names.get(&id.span())).into_owned())
            } else {
                Value::Unknown
            }
        }
        Expression::Array(a) => array(&a.elements, names, defaults),
        Expression::LegacyArray(a) => array(&a.elements, names, defaults),
        Expression::Parenthesized(p) => value(p.expression, names, defaults),
        Expression::Call(Call::Method(c))
            if c.argument_list.arguments.is_empty()
                && matches!(&c.method, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"toArray") || id.value.eq_ignore_ascii_case(b"all")) =>
        {
            collection(c.object, names, defaults)
                .map(Value::Array)
                .unwrap_or(Value::Unknown)
        }
        _ => Value::Unknown,
    }
}

fn array(
    elements: &TokenSeparatedSequence<ArrayElement>,
    names: &ResolvedNames,
    defaults: &[(String, Value)],
) -> Value {
    let mut items = Vec::new();
    for element in elements.iter() {
        let ArrayElement::KeyValue(kv) = element else {
            return Value::Unknown;
        };
        let Value::String(key) = value(kv.key, names, defaults) else {
            return Value::Unknown;
        };
        items.push((key, value(kv.value, names, defaults)));
    }
    Value::Array(items)
}

fn collection(
    e: &Expression,
    names: &ResolvedNames,
    defaults: &[(String, Value)],
) -> Option<Vec<(String, Value)>> {
    match e {
        Expression::Call(Call::StaticMethod(c))
            if c.argument_list.arguments.is_empty()
                && matches!(&c.method, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"defaultAliases"))
                && matches!(c.class, Expression::Identifier(id) if names.get(&id.span()).eq_ignore_ascii_case(b"Illuminate\\Support\\Facades\\Facade")) =>
        {
            Some(defaults.to_vec())
        }
        Expression::Call(Call::Method(c)) if matches!(&c.method, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"merge")) =>
        {
            let [Argument::Positional(arg)] = c.argument_list.arguments.as_slice() else {
                return None;
            };
            let mut items = collection(c.object, names, defaults)?;
            let Value::Array(extra) = value(arg.value, names, defaults) else {
                return None;
            };
            items.extend(extra);
            Some(items)
        }
        _ => None,
    }
}
