//! Branches of the declaration, keyword, assignment and type rules that PHPStan's own rule fixtures miss (other
//! `phpVersion`s, declare values, trait aliases, interface constructors, by-reference arguments, destructuring
//! targets, use statements, promoted and hooked property types, final and abstract parents, traits used in a trait).
//! Expected: PHPStan's run on tests/fixtures/statements for each
//! phpstan*.neon there, restricted to the identifiers Pun reports. tests/fixtures/declarations holds the class
//! constant, PHP version and literal array checks, run the same way for each `phpVersion` that changes them.

use std::path::Path;

use pun_analyse::{Project, analyse, reports, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

fn check(neon: &str) {
    check_in("statements", neon);
}

fn check_in(fixtures: &str, neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixtures);
    let dir = root.to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{dir}/{neon}.neon")),
        ..Options::for_project(&dir)
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    let prefix = format!("{dir}/");
    let mut got: Vec<Key> = analyse(&root, &config)
        .unwrap()
        .into_iter()
        .map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
            (file, e.line, e.identifier, e.message, e.tip)
        })
        .collect();
    got.sort();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("expected-{neon}.json"))).unwrap())
            .unwrap();
    let mut want: Vec<Key> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| reports(e["identifier"].as_str().unwrap_or("")))
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn php_8_5() {
    check("phpstan");
}

/// Built-in functions, classes and methods new in PHP 8.5 exist only from `phpVersion: 80500` (expected: the
/// PHPStan phar run outside any Composer project, so no polyfill declares them).
#[test]
fn builtins_follow_php_version() {
    check_in("builtins", "phpstan");
    check_in("builtins", "phpstan-php84");
}

/// The casts and the backtick are allowed before PHP 8.5, the (unset) cast before 8.0.
#[test]
fn php_7_4() {
    check("phpstan-php74");
}

/// VoidCastRuleTest::testSupport below PHP 8.5 (the oracle's PHP 8.4 cannot parse `(void)`, so the expectation is
/// the upstream test's).
#[test]
fn void_cast_before_php_8_5() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/rules/data/Cast/data/void-cast-support.php");
    let mut project = Project::new(vec![file], Vec::new());
    project.php_version = 80400;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message))
        .collect();
    assert_eq!(
        got,
        [(
            10,
            "cast.voidNotSupported".to_string(),
            "The (void) cast is supported only on PHP 8.5 and later.".to_string()
        )]
    );
}

/// Every version gate below 8.0: non-capturing catches, throw expressions, first-class callables (whose other checks
/// are then skipped), final and typed constants, constants in traits (once per using class), and interface constants
/// final before 8.1.
#[test]
fn declarations_php_7_4() {
    check_in("declarations", "phpstan-70400");
}

/// Type aliases that PHPStan's LocalTypeAliases fixtures miss: duplicates of an interface, trait or enum, invalid tag
/// values, aliases with invalid types, circular imports, `parent` in aliases, and aliases of a trait, interface and enum.
#[test]
fn type_aliases() {
    check_in("type_aliases", "phpstan");
}

/// A PHP_VERSION_ID guard does not lift the non-capturing catch check; 8.0 allows catches and throw expressions.
#[test]
fn declarations_php_8_0() {
    check_in("declarations", "phpstan-80000");
}

/// Readonly classes before 8.2, anonymous ones before 8.3.
#[test]
fn declarations_php_8_1() {
    check_in("declarations", "phpstan-80100");
}

#[test]
fn declarations_php_8_2() {
    check_in("declarations", "phpstan-80200");
}

/// Overridden and trait constants, literal array keys and references to properties with narrower write visibility.
#[test]
fn declarations_php_8_5() {
    check_in("declarations", "phpstan-80500");
}

/// Below PHP 8.1 PHPStan cannot parse a readonly class, so ReadOnlyClassRule reports nothing (only parse errors).
#[test]
fn declarations_readonly_php_8_0() {
    check_in("declarations", "phpstan-80000-readonly");
}

/// Anonymous class names (`class@anonymous/<path>:<line>`, `:<index>` for several on a line, the parent or first
/// interface's name) in messages and in a trait's `in context of` label.
#[test]
fn anonymous_classes() {
    check_in("anonymous", "phpstan");
}

/// AccessPropertiesRule, AccessPropertiesInAssignRule and AccessStaticPropertiesRule: undefined, parent-private and static
/// properties, the contexts that allow an undefined one (`isset()`, `??`, `unset()`), narrowing conditions,
/// assignments, closures, magic and dynamic classes, generic classes and traits.
#[test]
fn property_access() {
    check_in("properties", "phpstan");
}

/// Before PHP 8.2 every class allows dynamic properties, so `isset()` on a final class's missing one is allowed.
#[test]
fn property_access_php_8_1() {
    check_in("properties", "phpstan-81");
}

/// PHPStan 2.3.1's original AccessStaticPropertiesInAssignRuleTest::testRuleExpressionNames fixture,
/// unchanged: the finite foreach keys include a missing static property (fresh level-0 oracle capture).
#[test]
fn static_property_expression_names_original() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "tests/fixtures/rules/data/Properties/data/properties-from-array-into-static-object.php",
    );
    let got: Vec<_> = run(&Project::new(vec![file], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip))
        .collect();
    assert_eq!(
        got,
        [(
            29,
            "staticProperty.notFound".to_string(),
            "Access to an undefined static property PropertiesFromArrayIntoStaticObject\\Foo::$noop."
                .to_string(),
            None,
        )]
    );
}

/// Finite dynamic selectors use the same receiver, visibility and undefined-property checks as direct names.
/// Expectations are fresh PHPStan 2.3.1 captures, at levels 0 and 2 (including non-stringable-name checks).
#[test]
fn static_property_expression_names() {
    check_in("static-property-names", "phpstan");
    check_in("static-property-names", "phpstan-level2");
}

/// Writes and isset guards keep selector identity, branch joins and invalidation; multiline diagnostics point
/// at the name expression. Inc/dec produces both a read check and a PropertyAssignNode check upstream.
#[test]
fn static_property_expression_name_boundaries() {
    check_in("static-property-names", "phpstan-boundaries");
}

/// AllowedSubTypesRule (`@phpstan-sealed`, `@psalm-inheritors`), MethodVisibilityInInterfaceRule and
/// DuplicateTraitDeclarationRule (reported in each using class's context). The anonymous class's hashed name is the
/// oracle's from this crate's directory, the working directory of `cargo test`.
#[test]
fn hierarchy() {
    check_in("hierarchy", "phpstan");
}

/// AccessPrivatePropertyThroughStaticRule, CallPrivateMethodThroughStaticRule and
/// AccessPrivateConstantThroughStaticRule at level 2: own, trait and parent members, final classes.
#[test]
fn through_static() {
    check_in("through-static", "phpstan");
}

/// RestrictedMethodUsageRule and RestrictedStaticMethodUsageRule with the `@internal` extension (level 2,
/// `featureToggles.internalTag`): internal methods, methods of internal class-likes, the root-namespace exemption.
#[test]
fn internal_methods() {
    check_in("internal", "phpstan");
}

/// RestrictedFunctionUsageRule and RestrictedFunctionCallableUsageRule with the `@internal` extension (level 0,
/// `featureToggles.internalTag`): calls, first-class callables and pipes, the root-namespace and same-named-class
/// exemptions (upstream's function-internal-tag.php plus functions.php).
#[test]
fn internal_functions() {
    check_in("restricted_functions", "phpstan");
}
