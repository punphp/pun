//! A project's codebase set with `set_project_codebase`: a file sees the symbols of the project's other files. The
//! expected code is Rector's (.oracle/rector-bin, `ReturnTypeFromStrictTypedPropertyRector` alone, PHP 8.4) for the
//! two classes in one file, tests/rector/ReturnTypeFromStrictTypedPropertyRector/Oracle/property_fetches.php.inc.
//! Its own binary: the project codebase is process-wide.

use std::path::Path;

use pun_lint::Linter;
use pun_lint::rules::AddReturnTypeFromProperty;
use pun_lint::rules::facts::{project_codebase, set_project_codebase};

const ITEM: &str = "<?php\n\nclass Item\n{\n    public string $name = '';\n}\n";
const HOLDER: &str = "<?php\n\nfinal class Holder\n{\n    private Item $item;\n\n    public function __construct()\n    {\n        $this->item = new Item();\n    }\n\n    public function name()\n    {\n        return $this->item->name;\n    }\n}\n";

fn fix() -> String {
    let linter = Linter::new(vec![Box::new(AddReturnTypeFromProperty)], 80400);
    linter.fix(Path::new("holder.php"), HOLDER).unwrap().source
}

#[test]
fn a_file_sees_the_project_codebase() {
    let files = [
        ("item.php".to_string(), ITEM.to_string()),
        ("holder.php".to_string(), HOLDER.to_string()),
    ];
    set_project_codebase(Some(project_codebase(&files, 80400)));
    let typed = fix();
    set_project_codebase(None);
    assert_eq!(
        typed,
        HOLDER.replace("public function name()", "public function name(): string")
    );
}

/// Classes Composer autoloads are scanned on demand, and `ClassMethodReturnTypeOverrideGuard` lets a method override
/// a parent's untyped method only when both are in vendor/ or neither is. Expected: Rector 2.6.1
/// (`ReturnTypeFromStrictConstantReturnRector` alone, PHP 8.4, `--autoload-file vendor/autoload.php` of a project
/// whose psr-4 maps `Acme\` to vendor/acme/pkg/src/) types only the method whose parent is the project's.
#[test]
fn autoloaded_vendor_parents_guard_overrides() {
    use pun_lint::rules::AddReturnTypeFromConstant;
    use pun_lint::rules::facts::project_codebase_autoloaded;
    let base = |ns: &str| {
        format!(
            "<?php\n\nnamespace {ns};\n\nclass {}\n{{\n    public function run()\n    {{\n        return 0;\n    }}\n}}\n",
            if ns == "Acme" { "Base" } else { "LocalBase" }
        )
    };
    let commands = "<?php\n\nnamespace App;\n\nfinal class FromVendor extends \\Acme\\Base\n{\n    public const OK = 1;\n\n    public function run()\n    {\n        return self::OK;\n    }\n}\n\nfinal class FromProject extends LocalBase\n{\n    public const OK = 1;\n\n    public function run()\n    {\n        return self::OK;\n    }\n}\n";
    let files = [(
        "/project/app/Commands.php".to_string(),
        commands.to_string(),
    )];
    let autoload = |class: &[u8]| match class {
        b"acme\\base" => Some((
            "/project/vendor/acme/pkg/src/Base.php".to_string(),
            base("Acme"),
        )),
        b"app\\localbase" => Some(("/project/app/LocalBase.php".to_string(), base("App"))),
        _ => None,
    };
    set_project_codebase(Some(project_codebase_autoloaded(&files, 80400, &autoload)));
    let linter = Linter::new(vec![Box::new(AddReturnTypeFromConstant)], 80400);
    let fixed = linter
        .fix(Path::new("/project/app/Commands.php"), commands)
        .unwrap()
        .source;
    set_project_codebase(None);
    let at = commands.rfind("public function run()").unwrap();
    let expected = format!(
        "{}public function run(): int{}",
        &commands[..at],
        &commands[at + 21..]
    );
    assert_eq!(fixed, expected);
}
