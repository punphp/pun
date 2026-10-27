//! Rector 2.6.1, PHP 8.4: moved closures and anonymous-class late-static constants.
use pun_lint::rules::facts::{project_codebase, set_project_codebase};
use pun_lint::rules::{
    AddArrowFunctionReturn, AddReturnTypeFromConstant, AddStringReturn, NoElseAfterReturn,
};
use pun_lint::{Linter, Rule};
use std::path::Path;

#[test]
fn project_fix_rounds_keep_the_moved_arrow_typed() {
    let source = include_str!("fixtures/moved-closure/f.php");
    let expected = include_str!("fixtures/moved-closure/f.expected.php");
    set_project_codebase(Some(project_codebase(
        &[("f.php".into(), source.into())],
        80400,
    )));
    let linter = Linter::new(
        vec![
            Box::new(NoElseAfterReturn),
            Box::new(AddArrowFunctionReturn),
        ],
        80400,
    );
    let fixed = linter.fix(Path::new("f.php"), source).unwrap();
    set_project_codebase(None);
    assert_eq!(fixed.source, expected);
    assert_eq!(
        linter.fix(Path::new("f.php"), expected).unwrap().source,
        expected
    );

    // Identical file bytes under a different project must not reuse the preceding
    // project's facts. The expected declarations follow the external PHP signatures.
    let caller = "<?php $f = fn () => external();";
    for declaration in ["int", "string", "int"] {
        let library =
            format!("<?php function external(): {declaration} {{ throw new \\Exception(); }}");
        let project = project_codebase(
            &[
                ("caller.php".into(), caller.into()),
                ("library.php".into(), library),
            ],
            80400,
        );
        set_project_codebase(Some(project));
        let fixed = linter.fix(Path::new("caller.php"), caller).unwrap();
        assert_eq!(
            fixed.source,
            format!("<?php $f = fn (): {declaration} => external();")
        );
    }
    set_project_codebase(None);
    assert_eq!(
        linter.fix(Path::new("caller.php"), caller).unwrap().source,
        caller,
        "removing the project must also remove its external declaration"
    );

    // Reuse Rector's late-static constant oracle without changing its expectations.
    // Rector 2.6.1 also produces these bytes with the native string-return rule.
    let oracle = include_str!(
        "rector/ReturnTypeFromStrictConstantReturnRector/Oracle/late_static_constants.php.inc"
    );
    let (input, expected) = oracle.split_once("-----\n").unwrap();
    let anonymous_section = |code: &str| {
        let body = code.split_once("class ObjectConstants\n").unwrap().1;
        let body = body.split_once("final class FinalStatic\n").unwrap().0;
        format!("<?php\n\nclass ObjectConstants\n{body}")
    };
    let input = anonymous_section(input);
    let expected = anonymous_section(expected);
    set_project_codebase(Some(project_codebase(
        &[("anonymous.php".into(), input.clone())],
        80400,
    )));
    let rules: [Box<dyn Rule>; 2] = [
        Box::new(AddReturnTypeFromConstant),
        Box::new(AddStringReturn),
    ];
    for rule in rules {
        let id = rule.id();
        let linter = Linter::new(vec![rule], 80400);
        for (input, expected) in [
            (input.clone(), expected.clone()),
            (
                input.replacen("return new class", "return\n\n        new class", 1),
                expected.replacen("return new class", "return\n\n        new class", 1),
            ),
        ] {
            let path = Path::new("anonymous.php");
            let fixed = linter.fix(path, &input).unwrap();
            assert_eq!(fixed.source, expected, "{id}: anonymous class finality");
            assert_eq!(linter.fix(path, &expected).unwrap().source, expected);
        }
    }
    set_project_codebase(None);
}
