//! Type strings in a project scope and `decideType()`, against PHPStan 2.2.7
//! (`php scripts/export-phpstan-symbols.php probes` writes the fixtures).

mod common;

use std::collections::HashMap;
use std::sync::Arc;

use common::fixture;
use pun_phpstan::phpdoc;
use pun_phpstan::resolve::{
    Scope, const_expr, has_global_alias, resolve_in, resolve_str, with_type_aliases,
};
use pun_phpstan::symbols::{self, Class, Template};
use pun_phpstan::types::{Level, Type, ancestor_args, decide_type, is_super_type_of};

/// `new NameScope('PunProbe', [], $class)`
struct InClass<'a>(&'a str);

impl Scope for InClass<'_> {
    fn class(&self) -> Option<&str> {
        Some(self.0)
    }
    fn parent(&self) -> Option<String> {
        (self.0 == "PunProbe\\Child").then(|| "PunProbe\\Coll".to_string())
    }
    fn resolve_name(&self, name: &str) -> String {
        qualify(name)
    }
    fn constant_names(&self, name: &str) -> Vec<String> {
        constant_names(name)
    }
}

/// `new NameScope('PunProbe', [])`: no class, so the trait's defaults answer `class()` and `parent()`.
struct InNamespace;

impl Scope for InNamespace {
    fn parent(&self) -> Option<String> {
        None
    }
    fn resolve_name(&self, name: &str) -> String {
        qualify(name)
    }
    fn constant_names(&self, name: &str) -> Vec<String> {
        constant_names(name)
    }
}

fn qualify(name: &str) -> String {
    match name.strip_prefix('\\') {
        Some(full) => full.to_string(),
        None => format!("PunProbe\\{name}"),
    }
}

fn constant_names(name: &str) -> Vec<String> {
    match name.strip_prefix('\\') {
        Some(full) => vec![full.to_string()],
        None => vec![format!("PunProbe\\{name}"), name.to_string()],
    }
}

fn template(name: &str) -> Template {
    Template {
        name: name.to_string(),
        bound: "mixed".to_string(),
        default: None,
    }
}

/// tests/fixtures/scope-classes.php as PHPStan's reflection sees it.
fn class(name: &str) -> Option<Arc<Class>> {
    let class =
        |name: &str, ancestors: &[&str], templates, constants: &[(&str, &str)], generics| Class {
            name: name.to_string(),
            ancestors: ancestors.iter().map(|a| a.to_string()).collect(),
            templates,
            cases: Vec::new(),
            is_interface: false,
            is_trait: false,
            is_enum: false,
            is_final: false,
            properties: Vec::new(),
            constants: constants
                .iter()
                .map(|(n, t)| (n.to_string(), t.to_string()))
                .collect(),
            methods: HashMap::new(),
            iterable: None,
            generics,
            property_docs: HashMap::new(),
        };
    match name {
        "PunProbe\\Coll" => Some(Arc::new(class(
            name,
            &[],
            vec![template("TKey"), template("TValue")],
            &[("A", "1"), ("B", "'b'")],
            HashMap::new(),
        ))),
        "PunProbe\\Child" => Some(Arc::new(class(
            name,
            &["PunProbe\\Coll"],
            vec![template("T")],
            &[("C", "2")],
            HashMap::from([(
                "PunProbe\\Coll".to_string(),
                vec![
                    "int".to_string(),
                    "__template<T, 'class PunProbe\\Child', mixed>".to_string(),
                ],
            )]),
        ))),
        // `extends Coll` without `@extends`: no arguments named for it
        "PunProbe\\Plain" => Some(Arc::new(class(
            name,
            &["PunProbe\\Coll"],
            Vec::new(),
            &[],
            HashMap::new(),
        ))),
        "PunProbe\\Suit" => Some(Arc::new(Class {
            cases: vec!["Hearts".to_string()],
            is_enum: true,
            is_final: true,
            ..class(name, &[], Vec::new(), &[("X", "1")], HashMap::new())
        })),
        "PunProbe\\Tr" => Some(Arc::new(Class {
            is_trait: true,
            ..class(name, &[], Vec::new(), &[], HashMap::new())
        })),
        "PunProbe\\Defaults" => Some(Arc::new(class(
            name,
            &[],
            vec![
                template("A"),
                template("B"),
                Template {
                    default: Some("int".to_string()),
                    ..template("C")
                },
                template("D"),
            ],
            &[],
            HashMap::new(),
        ))),
        _ => None,
    }
}

/// `new NameScope('PunProbe', [], 'PunProbe\Child', null, $templates)` with the probe templates `T`, `U` of
/// `int|string` and `V` of `Countable` of `function probe()`.
struct WithTemplates;

impl Scope for WithTemplates {
    fn class(&self) -> Option<&str> {
        Some("PunProbe\\Child")
    }
    fn parent(&self) -> Option<String> {
        Some("PunProbe\\Coll".to_string())
    }
    fn resolve_name(&self, name: &str) -> String {
        qualify(name)
    }
    fn template(&self, name: &str) -> Option<Type> {
        let bound = match name {
            "T" => "mixed",
            "U" => "int|string",
            "V" => "Countable",
            _ => return None,
        };
        Some(resolve_str(
            &format!("__template<{name}, 'function probe()', {bound}>"),
            None,
        ))
    }
}

#[test]
fn project_scope() {
    let f = fixture("type-scope.json");
    let mut failures = Vec::new();
    symbols::with_classes(&class, || {
        for row in f.as_array().unwrap() {
            let input = row[1].as_str().unwrap();
            let want = row[2].as_str().unwrap();
            let node = phpdoc::parse(input).unwrap();
            let ty = match row[0].as_str() {
                Some("templates") => resolve_in(&node, &WithTemplates),
                Some(class) => resolve_in(&node, &InClass(class)),
                None => resolve_in(&node, &InNamespace),
            };
            let got = ty.describe(Level::Precise);
            if got != want {
                failures.push(format!("{} {input}: want {want}, got {got}", row[0]));
            }
        }
    });
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn decide_types() {
    let f = fixture("type-decide.json");
    let mut failures = Vec::new();
    for row in f.as_array().unwrap() {
        // no native type: an implicit mixed
        let native = row[0].as_str();
        let phpdoc = row[1].as_str();
        let want = row[2].as_str().unwrap();
        let native_type = native.map_or(Type::Mixed { explicit: false }, |n| resolve_str(n, None));
        let got =
            decide_type(native_type, phpdoc.map(|p| resolve_str(p, None))).describe(Level::Precise);
        if got != want {
            failures.push(format!("{native:?} / {phpdoc:?}: want {want}, got {got}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn project_super_types() {
    let f = fixture("type-project-super.json");
    let mut failures = Vec::new();
    symbols::with_classes(&class, || {
        for row in f.as_array().unwrap() {
            let [a, b, want] = [0, 1, 2].map(|i| row[i].as_str().unwrap());
            let resolve = |s: &str| resolve_in(&phpdoc::parse(s).unwrap(), &InNamespace);
            let got = format!("{:?}", is_super_type_of(&resolve(a), &resolve(b)));
            if got != want {
                failures.push(format!("{a} isSuperTypeOf {b}: want {want}, got {got}"));
            }
        }
    });
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// `TypeCombinator::addNull()` and `removeNull()`.
#[test]
fn add_and_remove_null() {
    let f = fixture("type-null.json");
    let mut failures = Vec::new();
    for row in f.as_array().unwrap() {
        let [input, added, removed] = [0, 1, 2].map(|i| row[i].as_str().unwrap());
        let ty = resolve_str(input, None);
        let got = [
            ty.clone().add_null().describe(Level::Precise),
            ty.remove_null().describe(Level::Precise),
        ];
        if got != [added, removed] {
            failures.push(format!("{input}: want {added} / {removed}, got {got:?}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// `getIterableKeyType()`, `getIterableValueType()` and `getAncestorWithClassName('PunProbe\Coll')`.
#[test]
fn project_iterables_and_ancestors() {
    let f = fixture("type-project.json");
    let mut failures = Vec::new();
    symbols::with_classes(&class, || {
        for row in f.as_array().unwrap() {
            let input = row[0].as_str().unwrap();
            let ty = resolve_in(&phpdoc::parse(input).unwrap(), &InNamespace);
            let ancestor = match &ty {
                // a class is its own ancestor
                Type::Object(name, _) if name == "PunProbe\\Coll" => {
                    Some(ty.describe(Level::Precise))
                }
                Type::Object(name, args) => symbols::lookup(name)
                    .and_then(|c| ancestor_args(&c, "PunProbe\\Coll", args))
                    .map(|args| {
                        Type::Object("PunProbe\\Coll".to_string(), args).describe(Level::Precise)
                    }),
                _ => None,
            };
            let got = (
                ty.iterable_key_type().describe(Level::Precise),
                ty.iterable_value_type().describe(Level::Precise),
                ancestor,
            );
            let want = (
                row[1].as_str().unwrap().to_string(),
                row[2].as_str().unwrap().to_string(),
                row[3].as_str().map(str::to_string),
            );
            if got != want {
                failures.push(format!("{input}: want {want:?}, got {got:?}"));
            }
        }
    });
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// `TypeTraverser::map()` with a callback that turns `int` into `string`.
#[test]
fn traverse() {
    // `instanceof IntegerType`: ranges and constants too
    fn map(t: &Type) -> Type {
        if matches!(t, Type::Int | Type::IntRange(..) | Type::ConstInt(_)) {
            Type::String
        } else {
            t.traverse(&map)
        }
    }
    let f = fixture("type-traverse.json");
    let mut failures = Vec::new();
    symbols::with_classes(&class, || {
        for row in f.as_array().unwrap() {
            let [input, want] = [0, 1].map(|i| row[i].as_str().unwrap());
            let ty = resolve_in(&phpdoc::parse(input).unwrap(), &WithTemplates);
            let got = map(&ty).describe(Level::Precise);
            if got != want {
                failures.push(format!("{input}: want {want}, got {got}"));
            }
        }
    });
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// phpdoc-parser reads a tag value's type and stops before the rest (`@param array<int, string> $x text`: the
/// type is the first 18 bytes); a tag value has no `*NEVER*`, which is Pun's own syntax.
#[test]
fn tag_value_prefix() {
    let value = "array<int, string> $x text";
    let (node, offset) = phpdoc::parse_prefix(value).unwrap();
    assert_eq!(node, phpdoc::parse("array<int, string>").unwrap());
    // the rest starts after the type (the offset may include the whitespace after it)
    assert_eq!(value[offset..].trim_start(), "$x text");
    assert!(phpdoc::parse_prefix("*NEVER* $x").is_err());
}

/// `parseGenericTypeArgument()` accepts a call-site variance keyword before an argument.
#[test]
fn generic_argument_variance() {
    assert_eq!(
        phpdoc::parse("ArrayIterator<int, covariant string>").unwrap(),
        phpdoc::parse("ArrayIterator<int, string>").unwrap()
    );
}

/// `tryResolvePseudoTypeClassType()`: `use Foo as integer;` makes `integer` that class; in a namespace a keyword
/// that names no class of it stays the keyword.
#[test]
fn pseudo_type_keywords_imported_or_namespaced() {
    struct Imported;
    impl Scope for Imported {
        fn parent(&self) -> Option<String> {
            None
        }
        fn resolve_name(&self, name: &str) -> String {
            qualify(name)
        }
        fn has_use_alias(&self, name: &str) -> bool {
            name == "integer"
        }
        fn in_namespace(&self) -> bool {
            true
        }
    }
    let describe =
        |s: &str| resolve_in(&phpdoc::parse(s).unwrap(), &Imported).describe(Level::Precise);
    assert_eq!(describe("integer"), "PunProbe\\integer");
    assert_eq!(describe("scalar"), "bool|float|int|string");
}

#[test]
fn object_class_names() {
    let f = fixture("type-object-class-names.json");
    symbols::with_classes(&class, || {
        for row in f.as_array().unwrap() {
            let input = row[0].as_str().unwrap();
            let want: Vec<String> = row[1]
                .as_array()
                .unwrap()
                .iter()
                .map(|n| n.as_str().unwrap().to_string())
                .collect();
            let ty = resolve_in(&phpdoc::parse(input).unwrap(), &InNamespace);
            assert_eq!(ty.object_class_names(), want, "{input}");
        }
    });
}

/// ConstExprNodeResolver on a `@method` parameter's default value.
#[test]
fn constant_expressions() {
    let f = fixture("type-const-expr.json");
    let mut failures = Vec::new();
    symbols::with_classes(&class, || {
        for row in f.as_array().unwrap() {
            let input = row[1].as_str().unwrap();
            let want = row[2].as_str().unwrap();
            let tags = phpdoc::tags(&format!("/** @method void m($a = {input}) */")).unwrap();
            let phpdoc::TagValue::Method { params, .. } = &tags[0].value else {
                panic!("{input}: no method tag");
            };
            let default = params[0].default.as_ref().unwrap();
            let ty = match row[0].as_str() {
                Some(class) => const_expr(default, &InClass(class)),
                None => const_expr(default, &InNamespace),
            };
            let got = ty.describe(Level::Precise);
            if got != want {
                failures.push(format!("{} {input}: want {want}, got {got}", row[0]));
            }
        }
    });
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The `typeAliases` parameter's global type aliases.
#[test]
fn global_type_aliases() {
    let f = fixture("type-global-aliases.json");
    for row in f.as_array().unwrap() {
        let aliases: Vec<(String, String)> = row[0]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();
        let input = row[1].as_str().unwrap();
        let got = with_type_aliases(&aliases, || {
            assert!(has_global_alias("PunAlias"));
            resolve_in(&phpdoc::parse(input).unwrap(), &InNamespace).describe(Level::Precise)
        });
        assert_eq!(got, row[2].as_str().unwrap(), "{input}");
    }
    assert!(!has_global_alias("PunAlias"));
}
