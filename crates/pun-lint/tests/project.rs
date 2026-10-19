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
