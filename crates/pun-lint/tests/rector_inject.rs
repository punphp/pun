//! The rules that inject a constructor dependency through Rector's `ClassDependencyManipulator`:
//! `refactor/replace-static-with-method` (Rector 2.6.1's `StaticCallToMethodCallRector`) and
//! `laravel/prefer-constructor-injection` (rector-laravel 2.5.0's `ArgumentFuncCallToMethodCallRector`).
//!
//! Fixture/ holds the upstream fixtures verbatim (rector-src rules-tests/Transform/Rector/StaticCall/
//! StaticCallToMethodCallRector with its Source/, rector-laravel tests/Rector/FuncCall/
//! ArgumentFuncCallToMethodCallRector), each rule configured through `Rule::configure` with the options of the
//! fixture's config/configured_rule.php, at its PHP version: StaticCallToMethodCallRector's Fixture at 8.0
//! (`PhpVersionFeature::PROPERTY_PROMOTION`), its FixturePhp74 at 7.4 with the `App` item only, rector-laravel's at
//! 8.4. Oracle/ holds extra inputs with what Rector wrote for them (`.oracle/rector-ecosystem`, the rule alone with the
//! fixture's configuration and Source/ autoloaded), at the PHP version of the rule's Fixture; Oracle84/, Oracle74/ and
//! Oracle73/ at 8.4, 7.4 and 7.3. ArgumentFuncCallToMethodCallRector's Oracle/rector_error_* are files Rector stops on
//! with an error and leaves as they were (a first-class callable, arguments without `method-if-args`).
//!
//! Pun/ holds the cases where Rector writes broken code and Pun leaves the class alone, each with its reason in the
//! file's first comment:
//! - StaticCallToMethodCallRector/Pun/nested_class: `$this->application` in a nested class that has no such property.
//! - StaticCallToMethodCallRector/Pun/parent_parameter_of_the_name: no dependency added, `$this->application` read.
//! - StaticCallToMethodCallRector/Pun/trait_constructor: `parent::__construct()` in a class without a parent.
//! - StaticCallToMethodCallRector/Pun/two_calls_in_constructor: the parameter `$application` declared twice.
//! - StaticCallToMethodCallRector/Pun/variadic_parent_constructor: a parameter after the variadic one.
//! - ArgumentFuncCallToMethodCallRector/Pun/parent_private_method: the parent's private method called.
//! - ArgumentFuncCallToMethodCallRector/Pun/static_property: a static property read as `$this->s`.

mod lint_c;

use lint_c::{run_class, run_class_at, run_dirs};
use pun_lint::Rule;
use pun_lint::rules::{PreferConstructorInjection, ReplaceStaticWithMethod};
use serde_json::{Value, json};

fn configured<R: Rule + 'static>(mut rule: R, options: &Value) -> Box<dyn Rule> {
    rule.configure(options).unwrap();
    Box::new(rule)
}

/// `configure` rejects each of `bad` with the message `error`.
fn rejects(mut rule: impl Rule, bad: &[(Value, &str)]) {
    for (options, error) in bad {
        assert_eq!(rule.configure(options).unwrap_err(), *error, "{options}");
    }
    // No options: the rule does nothing.
    rule.configure(&Value::Null).unwrap();
}

const STATIC: &str = "StaticCallToMethodCallRector";

fn static_options() -> Value {
    json!({"items": [
        {"static-class": "Nette\\Utils\\FileSystem", "static-method": "write", "class-type": "Rector\\Tests\\Transform\\Rector\\StaticCall\\StaticCallToMethodCallRector\\Source\\TargetFileSystem", "method-name": "dumpFile"},
        {"static-class": "Illuminate\\Support\\Facades\\Response", "static-method": "*", "class-type": "Illuminate\\Contracts\\Routing\\ResponseFactory", "method-name": "*"},
        {"static-class": "Illuminate\\Support\\Facades\\App", "static-method": "*", "class-type": "Illuminate\\Foundation\\Application", "method-name": "*"},
    ]})
}

fn static_rule(options: &Value) -> Box<dyn Rule> {
    configured(ReplaceStaticWithMethod::default(), options)
}

#[test]
fn static_call_to_method_call() {
    let options = static_options();
    run_class_at(vec![static_rule(&options)], STATIC, 80000);
    let at = |sub: &str, version| run_dirs(vec![static_rule(&options)], STATIC, &[sub], version, &[]);
    at("Oracle84", 80400);
    at("Oracle74", 70400);
    at("Oracle73", 70300);
    let php74 = json!({"items": [
        {"static-class": "Illuminate\\Support\\Facades\\App", "static-method": "*", "class-type": "Illuminate\\Foundation\\Application", "method-name": "*"},
    ]});
    run_dirs(vec![static_rule(&php74)], STATIC, &["FixturePhp74"], 70400, &[]);
    rejects(
        ReplaceStaticWithMethod::default(),
        &[
            (
                json!({"items": [{"static-class": "A", "static-method": "b", "class-type": "C"}]}),
                "each of `items` needs `method-name`",
            ),
            (
                json!({"items": [{"static-class": "1A", "static-method": "b", "class-type": "C", "method-name": "d"}]}),
                "\"1A\" is not a valid class name",
            ),
            (
                json!({"items": [{"static-class": "A", "static-method": "b-c", "class-type": "C", "method-name": "d"}]}),
                "\"b-c\" is not a valid method name",
            ),
            (
                json!({"items": [{"static-class": "A", "static-method": "*", "class-type": "C-", "method-name": "d"}]}),
                "\"C-\" is not a valid class name",
            ),
            (
                json!({"items": [{"static-class": "A", "static-method": "*", "class-type": "C", "method-name": "d-"}]}),
                "\"d-\" is not a valid method name",
            ),
        ],
    );
}

const LARAVEL: &str = "ArgumentFuncCallToMethodCallRector";

fn laravel_options() -> Value {
    json!({"items": [
        {"function": "auth", "class": "Illuminate\\Contracts\\Auth\\Guard"},
        {"function": "view", "class": "Illuminate\\Contracts\\View\\Factory", "method-if-args": "make"},
        {"function": "route", "class": "Illuminate\\Routing\\UrlGenerator", "method-if-args": "route"},
        {"function": "back", "class": "Illuminate\\Routing\\Redirector", "method-if-args": "back", "method-if-no-args": "back"},
        {"function": "broadcast", "class": "Illuminate\\Contracts\\Broadcasting\\Factory", "method-if-args": "event"},
        {"function": "config", "class": "Illuminate\\Contracts\\Config\\Repository", "array-method": "set", "non-array-method": "get"},
        {"function": "session", "class": "Illuminate\\Session\\SessionManager", "array-method": "put", "non-array-method": "get"},
    ]})
}

#[test]
fn argument_func_call_to_method_call() {
    let options = laravel_options();
    run_class(
        vec![configured(PreferConstructorInjection::default(), &options)],
        LARAVEL,
    );
    rejects(
        PreferConstructorInjection::default(),
        &[
            (
                json!({"items": [{"function": "f"}]}),
                "each of `items` needs `class`",
            ),
            (
                json!({"items": [{"function": "f", "class": "C", "array-method": "a"}]}),
                "each of `items` needs `non-array-method`",
            ),
            (
                json!({"items": [{"function": "f", "class": "C", "array-method": "a-", "non-array-method": "b"}]}),
                "\"a-\" is not a valid method name",
            ),
            (
                json!({"items": [{"function": "f", "class": "C", "array-method": "a", "non-array-method": "b-"}]}),
                "\"b-\" is not a valid method name",
            ),
            (
                json!({"items": [{"function": "f", "class": "C", "method-if-args": 1}]}),
                "`method-if-args` must be a string",
            ),
            (
                json!({"items": [{"function": "f", "class": "C-"}]}),
                "\"C-\" is not a valid class name",
            ),
            (
                json!({"items": [{"function": "f-", "class": "C"}]}),
                "\"f-\" is not a valid function name",
            ),
        ],
    );
}
