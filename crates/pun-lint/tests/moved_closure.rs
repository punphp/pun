//! Rector 2.6.1, RemoveAlwaysElseRector + AddArrowFunctionReturnTypeRector, PHP 8.4.
use pun_lint::Linter;
use pun_lint::rules::facts::{project_codebase, set_project_codebase};
use pun_lint::rules::{AddArrowFunctionReturn, NoElseAfterReturn};
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
}
