//! Rector 2.6.1's own fixtures for the ported `refactor` codemods and the other Rector rules of the batch, copied verbatim from rector-src (rules-tests/...)
//! into tests/rector/<Rector class>/Fixture, with the fixture's `Source/` classes. Each rule is configured through
//! `Rule::configure` with the options `pun migrate` writes for the fixture's config/configured_rule.php. Oracle/ holds
//! extra inputs with what Rector 2.6.1 (`.oracle/rector-ecosystem`, the rule alone with the same configuration,
//! `phpVersion(80400)`) wrote for them; Pun/ holds the cases where Pun keeps the meaning or valid code where Rector
//! does not, each with its reason in the file. RenameCastRector's `fixture.php.inc` is left out: it casts with
//! `(real)`, which PHP 8.0 removed and Mago's parser rejects, so no file of it can be linted; Oracle/casts covers the
//! rest of it. A fixture must pass as `pun lint --fix == expected` and
//! `format(pun lint --fix) == format(expected)` with format being the laravel preset.

mod lint_c;

use lint_c::{run_class, run_class_with};
use pun_lint::Rule;
use pun_lint::rules::{
    RemoveFunctionArgument, RemoveFunctionCall, RenameCast, RenameClassConstant, RenameConstant,
    PreferClassNameConstant, RenameFunction, RequireStrictInArray, ReplaceFunctionWithConstant, ReplaceFunctionWithNew, ReplaceFunctionWithStatic,
};
use serde_json::{Value, json};

fn check(mut rule: impl Rule + 'static, options: Value, class: &str) {
    rule.configure(&options).unwrap();
    run_class(vec![Box::new(rule)], class);
}

/// `configure` rejects each of `bad` with the message `error`.
fn rejects(mut rule: impl Rule, bad: &[(Value, &str)]) {
    for (options, error) in bad {
        assert_eq!(rule.configure(options).unwrap_err(), *error, "{options}");
    }
    // No options: the rule does nothing.
    rule.configure(&Value::Null).unwrap();
}

#[test]
fn remove_func_call_arg() {
    check(
        RemoveFunctionArgument::default(),
        json!({"items": [{"function": "ldap_first_attribute", "argument-position": 2}]}),
        "RemoveFuncCallArgRector",
    );
    rejects(
        RemoveFunctionArgument::default(),
        &[
            (json!(["f"]), "options must be a table"),
            (json!({"f": 1}), "unknown option `f`; the list goes under `items`"),
            (json!({"items": 1}), "`items` must be an array"),
            (json!({"items": [1]}), "each of `items` must be a table"),
            (
                json!({"items": [{"function": "f"}]}),
                "each of `items` needs `argument-position`",
            ),
            (
                json!({"items": [{"function": "f", "argument-position": 1, "x": 1}]}),
                "unknown key `x` in `items`; the keys are function, argument-position",
            ),
            (
                json!({"items": [{"function": 1, "argument-position": 1}]}),
                "`function` must be a string",
            ),
            (
                json!({"items": [{"function": "f", "argument-position": "1"}]}),
                "`argument-position` must be an integer",
            ),
            (
                json!({"items": [{"function": "1f", "argument-position": 1}]}),
                "\"1f\" is not a valid function name",
            ),
        ],
    );
}

#[test]
fn remove_func_call() {
    check(
        RemoveFunctionCall::default(),
        json!({"items": ["var_dump"]}),
        "RemoveFuncCallRector",
    );
    rejects(
        RemoveFunctionCall::default(),
        &[(json!({"items": [1]}), "each of `items` must be a string")],
    );
}

/// The laravel preset's `short_scalar_cast` writes the same short casts.
#[test]
fn rename_cast() {
    let mut rule = RenameCast::default();
    rule.configure(&json!({"items": [
            {"from-cast-expr-class": "PhpParser\\Node\\Expr\\Cast\\Double", "from-cast-kind": 3, "to-cast-kind": 2},
            {"from-cast-expr-class": "PhpParser\\Node\\Expr\\Cast\\Double", "from-cast-kind": 1, "to-cast-kind": 2},
            {"from-cast-expr-class": "PhpParser\\Node\\Expr\\Cast\\Int_", "from-cast-kind": 2, "to-cast-kind": 1},
        ]}))
        .unwrap();
    run_class_with(
        vec![Box::new(rule)],
        "RenameCastRector",
        80400,
        &["Fixture/keep_parentheses.php.inc", "Oracle/casts.php.inc"],
    );
    rejects(
        RenameCast::default(),
        &[(
            json!({"items": [{"from-cast-expr-class": "PhpParser\\Node\\Expr\\Variable", "from-cast-kind": 1, "to-cast-kind": 2}]}),
            "\"PhpParser\\Node\\Expr\\Variable\" is not a subclass of \"PhpParser\\Node\\Expr\\Cast\"",
        )],
    );
}

#[test]
fn rename_constant() {
    check(
        RenameConstant::default(),
        json!({"MYSQL_ASSOC": "MYSQLI_ASSOC", "OLD_CONSTANT": "NEW_CONSTANT"}),
        "RenameConstantRector",
    );
    rejects(
        RenameConstant::default(),
        &[
            (json!({"A": 1}), "`A` must be a string"),
            (json!({"A\\B": "C"}), "\"A\\B\" is not a valid constant name"),
        ],
    );
}

/// `in_for` renames `sizeof` to `count`, which the laravel preset's `no_alias_functions` does too.
#[test]
fn rename_function() {
    let mut rule = RenameFunction::default();
    rule.configure(&json!({"view": "Laravel\\Templating\\render", "sprintf": "Safe\\sprintf", "abc2": "abc", "sizeof": "count"}))
        .unwrap();
    run_class_with(
        vec![Box::new(rule)],
        "RenameFunctionRector",
        80400,
        &["Fixture/in_for.php.inc"],
    );
}

#[test]
fn rename_class_const_fetch() {
    let source = "Rector\\Tests\\Renaming\\Rector\\ClassConstFetch\\RenameClassConstFetchRector\\Source";
    check(
        RenameClassConstant::default(),
        json!({"items": [
            {"old-class": format!("{source}\\LocalFormEvents"), "old-constant": "PRE_BIND", "new-constant": "PRE_SUBMIT"},
            {"old-class": format!("{source}\\LocalFormEvents"), "old-constant": "BIND", "new-constant": "SUBMIT"},
            {"old-class": format!("{source}\\LocalFormEvents"), "old-constant": "POST_BIND", "new-constant": "POST_SUBMIT"},
            {"old-class": format!("{source}\\LocalFormEvents"), "old-constant": "OLD_CONSTANT", "new-class": format!("{source}\\DifferentClass"), "new-constant": "NEW_CONSTANT"},
        ]}),
        "RenameClassConstFetchRector",
    );
    rejects(
        RenameClassConstant::default(),
        &[(
            json!({"items": [{"old-class": "A", "old-constant": "B", "new-class": "1", "new-constant": "C"}]}),
            "\"1\" is not a valid class name",
        )],
    );
}

#[test]
fn func_call_to_const_fetch() {
    check(
        ReplaceFunctionWithConstant::default(),
        json!({"php_sapi_name": "PHP_SAPI", "pi": "M_PI"}),
        "FuncCallToConstFetchRector",
    );
}

#[test]
fn func_call_to_new() {
    check(
        ReplaceFunctionWithNew::default(),
        json!({"collection": "Collection"}),
        "FuncCallToNewRector",
    );
}

#[test]
fn func_call_to_static_call() {
    check(
        ReplaceFunctionWithStatic::default(),
        json!({"items": [
            {"old-func-name": "view", "new-class-name": "SomeStaticClass", "new-method-name": "render"},
            {"old-func-name": "SomeNamespaced\\view", "new-class-name": "AnotherStaticClass", "new-method-name": "render"},
        ]}),
        "FuncCallToStaticCallRector",
    );
    rejects(
        ReplaceFunctionWithStatic::default(),
        &[(
            json!({"items": [{"old-func-name": "f", "new-class-name": "C", "new-method-name": "a-b"}]}),
            "\"a-b\" is not a valid method name",
        )],
    );
}

#[test]
fn string_class_name_to_class_constant() {
    check(
        PreferClassNameConstant::default(),
        json!({"items": ["Nette\\*", "Error", "Exception"]}),
        "StringClassNameToClassConstantRector",
    );
    rejects(
        PreferClassNameConstant::default(),
        &[(json!({"items": [1]}), "each of `items` must be a string")],
    );
}

#[test]
fn strict_in_array() {
    check(RequireStrictInArray, Value::Null, "StrictInArrayRector");
}
