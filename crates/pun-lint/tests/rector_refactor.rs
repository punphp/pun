//! Rector 2.8.4's own fixtures for the ported `refactor` codemods and the other Rector rules of the batch, copied verbatim from rector-src (rules-tests/...)
//! into tests/rector/<Rector class>/Fixture, with the fixture's `Source/` classes. Each rule is configured through
//! `Rule::configure` with the options `pun migrate` writes for the fixture's config/configured_rule.php. Oracle/ holds
//! extra inputs with what Rector 2.8.4 (`.oracle/rector-ecosystem`, the rule alone with the same configuration,
//! `phpVersion(80400)`) wrote for them; Pun/ holds the cases where Pun keeps the meaning or valid code where Rector
//! does not, each with its reason in the file. RenameCastRector's `fixture.php.inc` is left out: it casts with
//! `(real)`, which PHP 8.0 removed and Mago's parser rejects, so no file of it can be linted; Oracle/casts covers the
//! rest of it. RenameMethodRector's Fixture/Enum/rename_enum_method.php.inc is Fixture/Enum_rename_enum_method.php.inc,
//! the runner reading one directory. A fixture must pass as `pun lint --fix == expected` and
//! `format(pun lint --fix) == format(expected)` with format being the laravel preset.
//!
//! Not yet re-derived from Rector 2.8.4: most of RenameClassRector/Oracle* and AnnotationToAttributeRector/Oracle* hold
//! what Rector 2.6.1 wrote. Rector 2.8.4 prints a changed docblock with phpdoc-parser's format-preserving `Printer`
//! and reads Doctrine annotations with phpdoc-parser's own parser (`NodeDecorator/DoctrineAnnotationDecorator`),
//! which Pun does not port yet; on those cases its output differs (context/plan/lint.md). The oracle run of both
//! rules autoloads the rule's Stubs/ as well as Source/: without the Doctrine classes there, Rector leaves every
//! annotation alone. Re-derived so far: RenameClassRector/Oracle/doc_callable_templates.php.inc, doc_type_callable.php.inc,
//! OracleAutoImportNames/doc_type_callable.php.inc and AnnotationToAttributeRector/Oracle/values_more.php.inc; the
//! `callable<T of ...>` line of RenameClassRector/Oracle/doc_printing.php.inc and OracleAutoImportNames/doc_printing.php.inc
//! and the `A5` line of OracleAutoImportNames/doc_type_alias_types.php.inc; `C5` of
//! AnnotationToAttributeRector/Oracle/strings_more.php.inc and `C1` of values_edge.php.inc (an annotation with a
//! single-quoted value stays, rector-src 9a6fdb061a).

mod lint_c;

use lint_c::{run_class, run_class_at, run_class_with, run_dirs};
use mago_syntax::cst::Node;
use pun_lint::rules::{
    AddOverrideAttribute, AnnotationToAttribute, NoExtraArguments, PreferClassNameConstant,
    RemoveFunctionArgument, RemoveFunctionCall, RenameCast, RenameClass, RenameClassConstant,
    RenameConstant, RenameFunction, RenameMethod, ReplaceFunctionWithConstant,
    ReplaceFunctionWithNew, ReplaceFunctionWithStatic, RequireStrictInArray,
};
use pun_lint::{Linter, Rule};
use serde_json::{Value, json};
use std::path::Path;

fn configured<R: Rule + 'static>(mut rule: R, options: &Value) -> Box<dyn Rule> {
    rule.configure(options).unwrap();
    Box::new(rule)
}

fn check<R: Rule + 'static>(make: fn() -> R, options: Value, class: &str) {
    run_class(vec![configured(make(), &options)], class);
    everywhere(|| configured(make(), &options), class, 80400);
}

/// A rule called on every node, not only the kinds of its `targets()`.
struct Everywhere(Box<dyn Rule>);

impl Rule for Everywhere {
    fn id(&self) -> &'static str {
        self.0.id()
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &pun_lint::Context<'_>,
        out: &mut Vec<pun_lint::Report>,
    ) {
        self.0.visit(node, ancestors, ctx, out);
    }
}

/// `targets()` only spares the engine work: called on every node, the rule reports on each input of `class` what it
/// reports when called on its targets.
fn everywhere(make: impl Fn() -> Box<dyn Rule>, class: &str, php_version: u32) {
    let targeted = Linter::new(vec![make()], php_version);
    let everywhere = Linter::new(vec![Box::new(Everywhere(make()))], php_version);
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector")
        .join(class);
    for dir in ["Fixture", "Oracle", "Pun"] {
        let Ok(entries) = std::fs::read_dir(root.join(dir)) else {
            continue;
        };
        for path in entries.map(|e| e.unwrap().path()).filter(|p| p.is_file()) {
            let content = std::fs::read_to_string(&path).unwrap();
            let input = content.split("-----\n").next().unwrap();
            assert_eq!(
                everywhere.check(&path, input).unwrap(),
                targeted.check(&path, input).unwrap(),
                "{}",
                path.display()
            );
        }
    }
}

/// The PHP files under `dir`, recursively: `(path, code)`.
fn php_files(dir: &Path, out: &mut Vec<(String, String)>) {
    for path in std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()) {
        if path.is_dir() {
            php_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "php") {
            out.push((
                path.to_string_lossy().into_owned(),
                std::fs::read_to_string(&path).unwrap(),
            ));
        }
    }
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
        RemoveFunctionArgument::default,
        json!({"items": [{"function": "ldap_first_attribute", "argument-position": 2}]}),
        "RemoveFuncCallArgRector",
    );
    rejects(
        RemoveFunctionArgument::default(),
        &[
            (json!(["f"]), "options must be a table"),
            (
                json!({"f": 1}),
                "unknown option `f`; the list goes under `items`",
            ),
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
        RemoveFunctionCall::default,
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
    let options = json!({"items": [
        {"from-cast-expr-class": "PhpParser\\Node\\Expr\\Cast\\Double", "from-cast-kind": 3, "to-cast-kind": 2},
        {"from-cast-expr-class": "PhpParser\\Node\\Expr\\Cast\\Double", "from-cast-kind": 1, "to-cast-kind": 2},
        {"from-cast-expr-class": "PhpParser\\Node\\Expr\\Cast\\Int_", "from-cast-kind": 2, "to-cast-kind": 1},
    ]});
    everywhere(
        || configured(RenameCast::default(), &options),
        "RenameCastRector",
        80400,
    );
    run_class_with(
        vec![configured(RenameCast::default(), &options)],
        "RenameCastRector",
        80400,
        &["Fixture/keep_parentheses.php.inc", "Oracle/casts.php.inc"],
    );
    rejects(
        RenameCast::default(),
        &[
            (
                json!({"items": [{"from-cast-expr-class": "PhpParser\\Node\\Expr\\Variable", "from-cast-kind": 1, "to-cast-kind": 2}]}),
                "\"PhpParser\\Node\\Expr\\Variable\" is not a subclass of \"PhpParser\\Node\\Expr\\Cast\"",
            ),
            (
                json!({"items": [{"from-cast-expr-class": "PhpParser\\Node\\Expr\\Cast\\Double", "from-cast-kind": 2, "to-cast-kind": 3}]}),
                "`to-cast-kind` 3 of \"PhpParser\\Node\\Expr\\Cast\\Double\" prints `(real)`, which PHP 8 removed",
            ),
        ],
    );
}

/// What Rector 2.8.4 wrote with `RenameCast` `Bool_` 2 to 1, `Int_` 1 to 2, `String_` 2 to 1 and `Double` 2 to 1: the
/// other kind of each spelling, `(int)` printed as it was.
#[test]
fn rename_cast_kinds() {
    let cast = |class: &str, from: i64, to: i64| json!({"from-cast-expr-class": format!("PhpParser\\Node\\Expr\\Cast\\{class}"), "from-cast-kind": from, "to-cast-kind": to});
    let options = json!({"items": [cast("Bool_", 2, 1), cast("Int_", 1, 2), cast("String_", 2, 1), cast("Double", 2, 1)]});
    let code = "<?php\n\n$a = (boolean) $x;\n$b = (bool) $x;\n$c = (int) $x;\n$d = (binary) $x;\n$e = (string) $x;\n$f = (float) $x;\n$g = (array) $x;\n$h = (object) $x;\n$i = (integer) $x;\n";
    let fixed = Linter::new(vec![configured(RenameCast::default(), &options)], 80400)
        .fix(Path::new("f.php"), code)
        .unwrap();
    assert_eq!(
        fixed.source,
        "<?php\n\n$a = (bool) $x;\n$b = (bool) $x;\n$c = (int) $x;\n$d = (string) $x;\n$e = (string) $x;\n$f = (double) $x;\n$g = (array) $x;\n$h = (object) $x;\n$i = (integer) $x;\n"
    );
}

#[test]
fn rename_constant() {
    check(
        RenameConstant::default,
        json!({"MYSQL_ASSOC": "MYSQLI_ASSOC", "OLD_CONSTANT": "NEW_CONSTANT"}),
        "RenameConstantRector",
    );
    rejects(
        RenameConstant::default(),
        &[
            (json!({"A": 1}), "`A` must be a string"),
            (
                json!({"A\\B": "C"}),
                "\"A\\B\" is not a valid constant name",
            ),
        ],
    );
}

/// `in_for` renames `sizeof` to `count`, which the laravel preset's `no_alias_functions` does too.
#[test]
fn rename_function() {
    let options = json!({"view": "Laravel\\Templating\\render", "sprintf": "Safe\\sprintf", "abc2": "abc", "sizeof": "count"});
    everywhere(
        || configured(RenameFunction::default(), &options),
        "RenameFunctionRector",
        80400,
    );
    run_class_with(
        vec![configured(RenameFunction::default(), &options)],
        "RenameFunctionRector",
        80400,
        &["Fixture/in_for.php.inc"],
    );
}

#[test]
fn rename_class_const_fetch() {
    let source =
        "Rector\\Tests\\Renaming\\Rector\\ClassConstFetch\\RenameClassConstFetchRector\\Source";
    check(
        RenameClassConstant::default,
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
        &[
            (
                json!({"items": [{"old-class": "A", "old-constant": "B", "new-class": "1", "new-constant": "C"}]}),
                "\"1\" is not a valid class name",
            ),
            (
                json!({"items": [{"old-class": "A", "new-constant": "C"}]}),
                "each of `items` needs `old-constant`",
            ),
        ],
    );
}

#[test]
fn func_call_to_const_fetch() {
    check(
        ReplaceFunctionWithConstant::default,
        json!({"php_sapi_name": "PHP_SAPI", "pi": "M_PI"}),
        "FuncCallToConstFetchRector",
    );
}

#[test]
fn func_call_to_new() {
    check(
        ReplaceFunctionWithNew::default,
        json!({"collection": "Collection"}),
        "FuncCallToNewRector",
    );
}

#[test]
fn func_call_to_static_call() {
    check(
        ReplaceFunctionWithStatic::default,
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
        PreferClassNameConstant::default,
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
    check(|| RequireStrictInArray, Value::Null, "StrictInArrayRector");
}

/// From PHP 8.5 (`OVERRIDE_ATTRIBUTE_ON_PROPERTIES`); Oracle/names at `phpVersion(80400)` was left as it is.
#[test]
fn add_override_attribute_to_overridden_properties() {
    run_class_at(
        vec![Box::new(AddOverrideAttribute)],
        "AddOverrideAttributeToOverriddenPropertiesRector",
        80500,
    );
    everywhere(
        || Box::new(AddOverrideAttribute),
        "AddOverrideAttributeToOverriddenPropertiesRector",
        80500,
    );
    let code =
        "<?php\n\nclass B\n{\n    public $a;\n}\n\nclass C extends B\n{\n    public $a;\n}\n";
    let found = Linter::new(vec![Box::new(AddOverrideAttribute)], 80400)
        .check(std::path::Path::new("f.php"), code)
        .unwrap();
    assert!(found.is_empty());
}

/// config/configured_rule.php.
#[test]
fn rename_method() {
    let source = "Rector\\Tests\\Renaming\\Rector\\MethodCall\\RenameMethodRector\\Source\\";
    let fixture = "Rector\\Tests\\Renaming\\Rector\\MethodCall\\RenameMethodRector\\Fixture\\";
    let rename = |class: String, old: &str, new: &str| json!({"class": class, "old-method": old, "new-method": new});
    let mut keyed = rename("Nette\\Utils\\Html".into(), "addToArray", "addToHtmlArray");
    keyed["array-key"] = json!("hey");
    check(
        RenameMethod::default,
        json!({"items": [
            rename(format!("{source}AbstractType"), "setDefaultOptions", "configureOptions"),
            rename("Nette\\Utils\\Html".into(), "add", "addHtml"),
            rename(format!("{source}CustomType"), "notify", "__invoke"),
            rename(format!("{fixture}CustomType"), "notify", "__invoke"),
            rename(format!("{source}SomeSubscriber"), "old", "new"),
            rename(format!("{source}Foo"), "old", "new"),
            rename(format!("{source}NewInterface"), "some_old", "some_new"),
            rename(format!("{source}DifferentInterface"), "renameMe", "toNewVersion"),
            rename("ReflectionFunctionAbstract".into(), "getTentativeReturnType", "getReturnType"),
            keyed,
            rename("Symfony\\Component\\Workflow\\DefinitionBuilder".into(), "reset", "clear"),
            rename(format!("{source}Enum\\SomeEnumWithMethod"), "oldEnumMethod", "newEnumMethod"),
            rename(format!("{source}SomeTrait"), "_test", "test"),
            rename(format!("{fixture}RenameTraitMethod"), "run", "execute"),
            rename(format!("{fixture}RenameTraitMethodCall"), "method1", "method3"),
        ]}),
        "RenameMethodRector",
    );
    rejects(
        RenameMethod::default(),
        &[
            (
                json!({"items": [{"class": "A", "old-method": "a"}]}),
                "each of `items` needs `new-method`",
            ),
            (
                json!({"items": [{"class": "A", "old-method": "a", "new-method": "b", "array-key": []}]}),
                "`array-key` must be a string or an integer",
            ),
            (
                json!({"items": [{"class": "A", "old-method": "a-b", "new-method": "b"}]}),
                "\"a-b\" is not a valid method name",
            ),
        ],
    );
}

/// What Rector 2.8.4 wrote for this code with `MethodCallRename('App\\Child', 'old', 'new')` and
/// `MethodCallRenameWithArrayKey('App\\K', 'a', 'b', 3)`: an interface keeps a method its rename class's parent
/// interface declares, and an integer key is read as written.
#[test]
fn rename_method_parent_interface_and_integer_key() {
    let mut rule = RenameMethod::default();
    rule.configure(&json!({"items": [
        {"class": "App\\Child", "old-method": "old", "new-method": "new"},
        {"class": "App\\K", "old-method": "a", "new-method": "b", "array-key": 3},
    ]}))
    .unwrap();
    let interfaces = "<?php\n\nnamespace App;\n\ninterface Base\n{\n    public function old();\n}\n\ninterface Child extends Base\n{\n    public function old();\n}\n\ninterface Sub extends Child\n{\n    public function old();\n}\n\n";
    let code = format!(
        "{interfaces}class K\n{{\n    public function a() {{}}\n}}\n\nfunction f(K $k)\n{{\n    return $k->a();\n}}\n"
    );
    let fixed = Linter::new(vec![Box::new(rule)], 80400)
        .fix(std::path::Path::new("f.php"), &code)
        .unwrap();
    let expected = format!(
        "{interfaces}class K\n{{\n    public function b() {{}}\n}}\n\nfunction f(K $k)\n{{\n    return $k->b()[3];\n}}\n"
    );
    assert_eq!(fixed.source, expected);
}

#[test]
fn remove_extra_parameters() {
    check(
        || NoExtraArguments,
        Value::Null,
        "RemoveExtraParametersRector",
    );
}

/// What Rector 2.8.4 wrote with `RenameFunctionRector` `['strlen' => 'STRLEN']` and `RenameConstantRector`
/// `['my_const' => 'MY_CONST']`: every spelling is renamed, and the one already written as the new name is left as it is
/// (no report: the diff does not touch it).
#[test]
fn rename_to_another_case() {
    let rules = || -> Vec<Box<dyn Rule>> {
        vec![
            configured(RenameFunction::default(), &json!({"strlen": "STRLEN"})),
            configured(RenameConstant::default(), &json!({"my_const": "MY_CONST"})),
        ]
    };
    let code = "<?php\n\nstrlen('a');\nSTRLEN('b');\nStrlen('c');\necho my_const;\necho MY_CONST;\necho My_Const;\n";
    let path = Path::new("f.php");
    let found = Linter::new(rules(), 80400).check(path, code).unwrap();
    assert_eq!(found.len(), 4);
    let fixed = Linter::new(rules(), 80400).fix(path, code).unwrap();
    assert_eq!(
        fixed.source,
        "<?php\n\nSTRLEN('a');\nSTRLEN('b');\nSTRLEN('c');\necho MY_CONST;\necho MY_CONST;\necho MY_CONST;\n"
    );
}

/// A configurable rule without options (pun.toml turns it on with none) reports nothing.
#[test]
fn unconfigured_rules_do_nothing() {
    let cases: Vec<(Box<dyn Rule>, &str)> = vec![
        (
            Box::new(RemoveFunctionArgument::default()),
            "RemoveFuncCallArgRector",
        ),
        (
            Box::new(RemoveFunctionCall::default()),
            "RemoveFuncCallRector",
        ),
        (Box::new(RenameCast::default()), "RenameCastRector"),
        (Box::new(RenameClass::default()), "RenameClassRector"),
        (
            Box::new(RenameClassConstant::default()),
            "RenameClassConstFetchRector",
        ),
        (Box::new(RenameConstant::default()), "RenameConstantRector"),
        (Box::new(RenameFunction::default()), "RenameFunctionRector"),
        (Box::new(RenameMethod::default()), "RenameMethodRector"),
        (
            Box::new(ReplaceFunctionWithConstant::default()),
            "FuncCallToConstFetchRector",
        ),
        (
            Box::new(ReplaceFunctionWithNew::default()),
            "FuncCallToNewRector",
        ),
        (
            Box::new(ReplaceFunctionWithStatic::default()),
            "FuncCallToStaticCallRector",
        ),
    ];
    for (rule, class) in cases {
        let linter = Linter::new(vec![rule], 80400);
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/rector")
            .join(class);
        for dir in ["Fixture", "Oracle"] {
            for path in std::fs::read_dir(root.join(dir))
                .unwrap()
                .map(|e| e.unwrap().path())
            {
                let content = std::fs::read_to_string(&path).unwrap();
                let input = content.split("-----\n").next().unwrap();
                assert_eq!(
                    linter.check(&path, input).unwrap(),
                    [],
                    "{}",
                    path.display()
                );
            }
        }
    }
}

/// What Rector 2.8.4 wrote with `RemoveFuncCallArg('foo', 0)`: a first-class callable loses its `...`.
#[test]
fn remove_func_call_arg_of_first_class_callable() {
    let options = json!({"items": [{"function": "foo", "argument-position": 0}]});
    let fixed = Linter::new(
        vec![configured(RemoveFunctionArgument::default(), &options)],
        80400,
    )
    .fix(Path::new("f.php"), "<?php\n\n$f = foo(...);\nfoo(1, 2);\n")
    .unwrap();
    assert_eq!(fixed.source, "<?php\n\n$f = foo();\nfoo(2);\n");
}

/// What Rector 2.8.4 wrote with `RenameClassConstFetch('App\\A', 'x', 'X')`: the constant written another way is
/// renamed, the one already written as the new name and a dynamic `A::{$n}` stay (no report for them).
#[test]
fn rename_class_const_fetch_to_another_case() {
    let rule = || {
        let options =
            json!({"items": [{"old-class": "App\\A", "old-constant": "x", "new-constant": "X"}]});
        configured(RenameClassConstant::default(), &options)
    };
    let code = "<?php\n\nnamespace App;\n\nclass A\n{\n    const X = 1;\n}\n\necho A::X;\necho A::x;\necho A::{$n};\n";
    let path = Path::new("f.php");
    assert_eq!(
        Linter::new(vec![rule()], 80400)
            .check(path, code)
            .unwrap()
            .len(),
        1
    );
    let fixed = Linter::new(vec![rule()], 80400).fix(path, code).unwrap();
    assert_eq!(fixed.source, code.replace("A::x", "A::X"));
}

/// RenameMethodRector/OracleConfigured/skips: what Rector 2.8.4 wrote with `MethodCallRename('App\\B', 'old', 'new')`,
/// `('App\\Suit', 'oldCase', 'newCase')` and `('App\\Q', 'old', 'new')`. A call stays when the receiver's class
/// declares the old method (privately too) and has the new one, its own or an interface's; an enum case is matched
/// on an enum named as written, not on `$class::Case`.
#[test]
fn rename_method_skips() {
    let rename = |class: &str, old: &str, new: &str| json!({"class": class, "old-method": old, "new-method": new});
    let options = json!({"items": [
        rename("App\\B", "old", "new"),
        rename("App\\Suit", "oldCase", "newCase"),
        rename("App\\Q", "old", "new"),
    ]});
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector/RenameMethodRector/OracleConfigured/skips.php.inc");
    let content = std::fs::read_to_string(&path).unwrap();
    let (input, expected) = content.split_once("-----\n").unwrap();
    let fixed = Linter::new(vec![configured(RenameMethod::default(), &options)], 80400)
        .fix(&path, input)
        .unwrap();
    assert_eq!(fixed.source, expected);
}

/// StringClassNameToClassConstantRector/OracleConfigured/patterns: what Rector 2.8.4 wrote with the skip patterns
/// below, matched as `fnmatch` matches them (`[!...]`, `[^...]`, a leading `]`, ranges, `?`, an unclosed `[`).
#[test]
fn string_class_name_skip_patterns() {
    let options = json!({"items": [
        "Random\\[!E]*", "Random\\Engine\\?t*", "Dom\\[A-G]*", "Random\\Engine\\[Ss-x]ecure*",
        "Dom\\[^]A-S]ext*", "Dom\\No[de*", "Dom\\Doc[^u]*", "Dom\\El[e]ment*",
    ]});
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "tests/rector/StringClassNameToClassConstantRector/OracleConfigured/patterns.php.inc",
    );
    let content = std::fs::read_to_string(&path).unwrap();
    let (input, expected) = content.split_once("-----\n").unwrap();
    let fixed = Linter::new(
        vec![configured(PreferClassNameConstant::default(), &options)],
        80400,
    )
    .fix(&path, input)
    .unwrap();
    assert_eq!(fixed.source, expected);
}

/// RenameClassRector, each upstream config on its own directories: Fixture, Oracle and Pun with
/// config/configured_rule.php, FixtureAutoImportNames, OracleAutoImportNames and PunAutoImportNames with config/auto_import_names.php,
/// OracleNoShortClasses with that config and `importShortClasses(false)`,
/// FixtureRenameClassAndMethod with config/complex_rename.php and its RenameMethodRector beside the rule.
/// FixtureAutoImportNames/SomeShort.php, a class the fixtures name, is Stubs/SomeShort.php. FixtureSkipAfterDefined
/// and FixtureSkipUsedTracking are left out: they test Rector's `skip()` paths, which pun.toml's own ignores replace.
/// Fixture/skip_rename_docblock_imported_by_last_group_use and skip_rename_docblock_via_same_namespace_by_last_group_in_namespace
/// are FixtureUnformattable/: Pint (and so the laravel format) fails on their `use \Company2\{ Bar };`, so they run
/// without the format step. Fixture/class_annotations_serializer_type
/// is Oracle/: Rector 2.8.4 keeps `array<OldClass>` in the JMS `Type`, its pattern wanting a `b` after the old name.
#[test]
fn rename_class() {
    let source = "Rector\\Tests\\Renaming\\Rector\\Name\\RenameClassRector\\Source\\";
    let fixture = "Rector\\Tests\\Renaming\\Rector\\Name\\RenameClassRector\\Fixture\\";
    let mut options = serde_json::Map::new();
    for (old, new) in [
        (format!("{source}OldClass"), format!("{source}NewClass")),
        (
            format!("{source}InterfaceAndClass\\SomeBasicDateTime"),
            format!("{source}InterfaceAndClass\\SomeBasicDateTimeInterface"),
        ),
        (
            format!("{source}OldClassWithTypo"),
            format!("{source}NewClassWithoutTypo"),
        ),
        ("Countable".into(), "stdClass".into()),
        (
            "Twig_Extension_Sandbox".into(),
            "Twig\\Extension\\SandboxExtension".into(),
        ),
        (
            "MyNamespace\\MylegacyClass".into(),
            "MyNewNamespace\\MyNewClass".into(),
        ),
        (
            format!("{fixture}SingularClass"),
            format!("{fixture}DuplicatedClass"),
        ),
        (
            format!("{source}Contract\\FirstInterface"),
            format!("{source}Contract\\ThirdInterface"),
        ),
        (
            format!("{source}Contract\\SecondInterface"),
            format!("{source}Contract\\ThirdInterface"),
        ),
        (
            "Acme\\Foo\\DoNotUpdateExistingTargetNamespace".into(),
            "Acme\\Bar\\DoNotUpdateExistingTargetNamespace".into(),
        ),
        (
            format!("{source}SomeNonFinalClass"),
            format!("{source}SomeFinalClass"),
        ),
        (
            "Doctrine\\DBAL\\DBALException".into(),
            "Doctrine\\DBAL\\Exception".into(),
        ),
        ("NotExistsClass".into(), "NewClass".into()),
        ("foo".into(), "Bar\\Foo".into()),
        (
            "Sensio\\Bundle\\FrameworkExtraBundle\\Configuration\\IsGranted".into(),
            "Symfony\\Component\\Security\\Http\\Attribute\\IsGranted".into(),
        ),
        (
            "Symfony\\Component\\Serializer\\Normalizer\\ObjectNormalizer".into(),
            "Symfony\\Component\\Serializer\\Normalizer\\NormalizerInterface".into(),
        ),
        (
            "MyNamespace\\AbstractMigration".into(),
            "MyNamespace\\BaseMigration".into(),
        ),
    ] {
        options.insert(old, json!(new));
    }
    options.insert("remove-unused-imports".into(), json!(true));
    let options = Value::Object(options);
    run_dirs(
        vec![configured(RenameClass::default(), &options)],
        "RenameClassRector",
        &["Fixture", "Oracle", "Pun"],
        80400,
        &[],
    );
    everywhere(
        || configured(RenameClass::default(), &options),
        "RenameClassRector",
        80400,
    );
    let linter = Linter::new(vec![configured(RenameClass::default(), &options)], 80400);
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector/RenameClassRector/FixtureUnformattable");
    for path in std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()) {
        let input = std::fs::read_to_string(&path).unwrap();
        assert_eq!(
            linter.fix(&path, &input).unwrap().source,
            input,
            "{}",
            path.display()
        );
    }

    let auto = json!({
        format!("{source}OldClass"): format!("{source}NewClass"),
        format!("{source}FirstNamespace\\SomeServiceClass"): format!("{source}SecondNamespace\\SomeServiceClass"),
        "Storage": "Illuminate\\Support\\Facades\\Storage",
        "Queue": "Illuminate\\Support\\Facades\\Queue",
        "Some\\Storage": "Illuminate\\Support\\Facades\\Storage",
        "Some2\\Storage": "Foo2\\Storage",
        "import-names": true,
        "import-short-classes": true,
        "remove-unused-imports": true,
    });
    run_dirs(
        vec![configured(RenameClass::default(), &auto)],
        "RenameClassRector",
        &[
            "FixtureAutoImportNames",
            "OracleAutoImportNames",
            "PunAutoImportNames",
        ],
        80400,
        &[],
    );
    let mut no_short = auto.clone();
    no_short["import-short-classes"] = json!(false);
    run_dirs(
        vec![configured(RenameClass::default(), &no_short)],
        "RenameClassRector",
        &["OracleNoShortClasses"],
        80400,
        &[],
    );

    let complex = json!({
        format!("{source}OldClassWithMethod"): format!("{source}NewClassWithNewMethod"),
        "import-names": true,
        "remove-unused-imports": true,
    });
    let method = json!({"items": [{
        "class": format!("{source}NewClassWithNewMethod"), "old-method": "someMethod", "new-method": "someNewMethod",
    }]});
    run_dirs(
        vec![
            configured(RenameClass::default(), &complex),
            configured(RenameMethod::default(), &method),
        ],
        "RenameClassRector",
        &["FixtureRenameClassAndMethod"],
        80400,
        &[],
    );

    // OracleCrlf*: files whose lines end in `\r\n`, read as written (the runner makes them `\n`), without the format
    // step (the laravel format makes them `\n` too)
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector/RenameClassRector");
    let mut extra = Vec::new();
    for dir in ["Source", "Stubs"] {
        php_files(&root.join(dir), &mut extra);
    }
    for (dir, options) in [
        ("OracleCrlf", &options),
        ("OracleCrlfAutoImportNames", &auto),
    ] {
        let linter = Linter::new(vec![configured(RenameClass::default(), options)], 80400);
        for path in std::fs::read_dir(root.join(dir))
            .unwrap()
            .map(|e| e.unwrap().path())
        {
            let content = std::fs::read_to_string(&path).unwrap();
            let (input, expected) = content.split_once("-----\n").unwrap();
            let fixed =
                pun_lint::rules::facts::with_sources(extra.clone(), || linter.fix(&path, input))
                    .unwrap();
            assert_eq!(fixed.source, expected, "{}", path.display());
        }
    }

    rejects(
        RenameClass::default(),
        &[
            (json!([]), "options must be a table"),
            (json!({"A": 1}), "`A` must be a string"),
            (
                json!({"import-names": "yes"}),
                "`import-names` must be a boolean",
            ),
        ],
    );
}

/// AnnotationToAttributeRector, each upstream config on its own directories: Fixture, Oracle and Pun with
/// config/configured_rule.php (OraclePhp80 with it at PHP 8.0) (Fixture/Behat/*, Fixture/Doctrine/* and Fixture/Symfony/* in their directories),
/// FixtureMultipleCall with config/multiple_call_configured_rule.php at PHP 8.0, FixturePhp81 and OraclePhp81 with
/// config/nested_attributes_php81.php at PHP 8.1. The classes Rector's test run autoloads from rector-src's stubs/
/// are in Stubs/. Fixture/Doctrine/{doctrine_entity,handle_with_api_above,nullable_bool_value} are
/// FixtureUnformattable/Doctrine_*: Pint (and so the laravel format) fails on their `\App\Some\Class::class`, so they run
/// without the format step. FixturePhp81/{unique_constraints,new_line_after_open_parentheses} are OraclePhp81/, with what the
/// rule writes alone: their change is NestedAnnotationToAttributeRector's, which the config runs beside it.
#[test]
fn annotation_to_attribute() {
    let source = "Rector\\Tests\\Php80\\Rector\\Class_\\AnnotationToAttributeRector\\Source\\";
    let options = json!({"items": [
        {"tag": "Doctrine\\ORM\\Mapping\\Embeddable"},
        {"tag": format!("{source}NotAnAttribute")},
        {"tag": format!("{source}Annotation\\OpenApi\\PastAnnotation"), "attribute-class": format!("{source}Attribute\\OpenApi\\FutureAttribute")},
        {"tag": format!("{source}Annotation\\OpenApi\\Annotation\\NestedPastAnnotation"), "attribute-class": format!("{source}Attribute\\OpenApi\\Attribute\\NestedFutureAttribute")},
        {"tag": format!("{source}GenericAnnotation")},
        {"tag": format!("{source}GenericSingleImplicitAnnotation")},
        {"tag": "Symfony\\Component\\Routing\\Annotation\\Route"},
        {"tag": "Doctrine\\ORM\\Mapping\\Entity", "class-reference-fields": ["repositoryClass"]},
        {"tag": "Doctrine\\ORM\\Mapping\\DiscriminatorMap"},
        {"tag": "Doctrine\\ORM\\Mapping\\Column"},
        {"tag": "Symfony\\Component\\Validator\\Constraints\\Choice"},
        {"tag": "Symfony\\Component\\Validator\\Constraints\\Length"},
        {"tag": "Symfony\\Component\\Validator\\Constraints\\File"},
        {"tag": format!("{source}Options\\ConstraintWithOptions")},
        {"tag": format!("{source}Options\\ConstraintWithoutOptions")},
        {"tag": "Sensio\\Bundle\\FrameworkExtraBundle\\Configuration\\ParamConverter"},
        {"tag": format!("{source}UseAlias\\TestSmth")},
        {"tag": format!("{source}UseAlias\\TestOther")},
        {"tag": "Sensio\\Bundle\\FrameworkExtraBundle\\Configuration\\Security"},
        {"tag": "Symfony\\Component\\Serializer\\Attribute\\Groups"},
        {"tag": "When", "attribute-class": "Behat\\Step\\When", "use-value-as-attribute-argument": true},
        {"tag": "Then", "attribute-class": "Behat\\Step\\Then", "use-value-as-attribute-argument": true},
        {"tag": "Transform", "attribute-class": "Behat\\Transformation\\Transform", "use-value-as-attribute-argument": true},
        {"tag": "OldName1", "attribute-class": format!("{source}Attribute\\NewName1")},
        {"tag": "OldName2", "attribute-class": format!("{source}Attribute\\NewName2")},
        {"tag": "SameName", "attribute-class": format!("{source}Attribute\\SameName")},
        {"tag": format!("{source}Attribute\\OpenApi\\Annotation\\SomeProperty"), "attribute-class": format!("{source}Attribute\\OpenApi\\Attribute\\SomeProperty")},
        {"tag": "OpenApi\\Annotations\\Property", "attribute-class": "OpenApi\\Attributes\\Property"},
        {"tag": format!("{source}Attribute\\EmptyAttribute"), "class-reference-fields": ["foo"]},
    ]});
    run_dirs(
        vec![configured(AnnotationToAttribute::default(), &options)],
        "AnnotationToAttributeRector",
        &["Fixture", "Oracle", "Pun"],
        80400,
        &[],
    );
    everywhere(
        || configured(AnnotationToAttribute::default(), &options),
        "AnnotationToAttributeRector",
        80400,
    );
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector/AnnotationToAttributeRector");
    let mut extra = Vec::new();
    for dir in ["Source", "Stubs"] {
        php_files(&root.join(dir), &mut extra);
    }
    let linter = Linter::new(
        vec![configured(AnnotationToAttribute::default(), &options)],
        80400,
    );
    for path in std::fs::read_dir(root.join("FixtureUnformattable"))
        .unwrap()
        .map(|e| e.unwrap().path())
    {
        let content = std::fs::read_to_string(&path).unwrap();
        let (input, expected) = content.split_once("-----\n").unwrap();
        let fixed =
            pun_lint::rules::facts::with_sources(extra.clone(), || linter.fix(&path, input))
                .unwrap();
        assert_eq!(fixed.source, expected, "{}", path.display());
    }
    run_dirs(
        vec![configured(AnnotationToAttribute::default(), &options)],
        "AnnotationToAttributeRector",
        &["OraclePhp80"],
        80000,
        &[],
    );
    run_dirs(
        vec![configured(AnnotationToAttribute::default(), &options)],
        "AnnotationToAttributeRector",
        &["OraclePhp74"],
        70400,
        &[],
    );
    let multiple = json!({"items": [{"tag": "Doctrine\\ORM\\Mapping\\Entity"}, {"tag": "Doctrine\\ORM\\Mapping\\ManyToMany"}]});
    run_dirs(
        vec![configured(AnnotationToAttribute::default(), &multiple)],
        "AnnotationToAttributeRector",
        &["FixtureMultipleCall"],
        80000,
        &[],
    );
    let php81 = "Rector\\Tests\\Php80\\Rector\\Class_\\AnnotationToAttributeRector\\SourcePhp81\\";
    let nested = json!({"items": [
        {"tag": format!("{php81}All")},
        {"tag": format!("{php81}Length")},
        {"tag": format!("{php81}NotNumber")},
        {"tag": format!("{source}GenericAnnotation")},
    ]});
    let source_php81 = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector/AnnotationToAttributeRector/SourcePhp81");
    run_dirs(
        vec![configured(AnnotationToAttribute::default(), &nested)],
        "AnnotationToAttributeRector",
        &["FixturePhp81", "OraclePhp81"],
        80100,
        &[&source_php81],
    );
    rejects(
        AnnotationToAttribute::default(),
        &[
            (json!([]), "options must be a table"),
            (
                json!({"items": [{"attribute-class": "A"}]}),
                "each of `items` needs `tag`",
            ),
            (
                json!({"items": [{"tag": "A b"}]}),
                "\"A b\" is not a valid class name",
            ),
            (json!({"items": [{"tag": 1}]}), "`tag` must be a string"),
            (
                json!({"items": [{"tag": "A", "class-reference-fields": [1]}]}),
                "`class-reference-fields` must be a list of strings",
            ),
            (
                json!({"items": [{"tag": "A", "class-reference-fields": "f"}]}),
                "`class-reference-fields` must be a list of strings",
            ),
            (
                json!({"items": [{"tag": "A", "use-value-as-attribute-argument": 1}]}),
                "`use-value-as-attribute-argument` must be a boolean",
            ),
        ],
    );
}
