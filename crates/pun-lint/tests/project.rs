//! A project's codebase set with `set_project_codebase`: a file sees the symbols of the project's other files. The
//! expected code is Rector's (.oracle/rector-bin, `ReturnTypeFromStrictTypedPropertyRector` alone, PHP 8.4) for the
//! two classes in one file, tests/rector/ReturnTypeFromStrictTypedPropertyRector/Oracle/property_fetches.php.inc.
//! Its own binary: the project codebase is process-wide.

use std::path::Path;
use std::sync::Mutex;

use pun_lint::Linter;
use pun_lint::rules::AddReturnTypeFromProperty;
use pun_lint::rules::facts::{project_codebase, set_project_codebase};

const ITEM: &str = "<?php\n\nclass Item\n{\n    public string $name = '';\n}\n";
const HOLDER: &str = "<?php\n\nfinal class Holder\n{\n    private Item $item;\n\n    public function __construct()\n    {\n        $this->item = new Item();\n    }\n\n    public function name()\n    {\n        return $this->item->name;\n    }\n}\n";

/// The tests take turns, as each sets the process-wide codebase.
static LOCK: Mutex<()> = Mutex::new(());

fn fix() -> String {
    let linter = Linter::new(vec![Box::new(AddReturnTypeFromProperty)], 80400);
    linter.fix(Path::new("holder.php"), HOLDER).unwrap().source
}

#[test]
fn a_file_sees_the_project_codebase() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
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

/// A deferred scan waits for the first rule that reads facts: `no-goto` reads none, so linting with it leaves the
/// project unscanned, and the property rule then sees `Item` from the scan (the same expected code as above).
#[test]
fn a_deferred_scan_runs_when_a_rule_reads_facts() {
    use pun_lint::rules::NoGoto;
    use pun_lint::rules::facts::set_project_scan;
    use std::sync::atomic::{AtomicBool, Ordering};
    static SCANNED: AtomicBool = AtomicBool::new(false);
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    set_project_scan(|| {
        SCANNED.store(true, Ordering::SeqCst);
        let files = [
            ("item.php".to_string(), ITEM.to_string()),
            ("holder.php".to_string(), HOLDER.to_string()),
        ];
        project_codebase(&files, 80400)
    });
    let goto = Linter::new(vec![Box::new(NoGoto)], 80400);
    assert!(
        goto.check(Path::new("holder.php"), HOLDER)
            .unwrap()
            .is_empty()
    );
    let scanned_before = SCANNED.load(Ordering::SeqCst);
    let typed = fix();
    set_project_codebase(None);
    assert!(!scanned_before);
    assert!(SCANNED.load(Ordering::SeqCst));
    assert_eq!(
        typed,
        HOLDER.replace("public function name()", "public function name(): string")
    );
}

/// A fix round lints a file changed after the project's codebase was scanned, so Mago's analysis gives `$this` no
/// type there. PHPStan types it as the class (`ThisType`), so the private method `$this` calls stays used. Expected:
/// the file unchanged, as Rector 2.6.1 (`RemoveUnusedPrivateMethodRector` and `AppToFacadeRector`) keeps
/// `getRecentlyViewed()` in Pinkary's app/Jobs/IncrementViews.php after rewriting its `app()` call.
#[test]
fn a_changed_file_keeps_its_methods_called_on_this() {
    use pun_lint::rules::NoUnusedPrivateMethod;
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let changed = "<?php\n\nfinal class Views\n{\n    public function handle(): int\n    {\n        return $this->recent();\n    }\n\n    private function recent(): int\n    {\n        return 1;\n    }\n}\n";
    let scanned = changed.replace(
        "return $this->recent();",
        "$n = $this->recent();\n\n        return $n;",
    );
    let files = [("/p/views.php".to_string(), scanned)];
    set_project_codebase(Some(project_codebase(&files, 80400)));
    let linter = Linter::new(vec![Box::new(NoUnusedPrivateMethod)], 80400);
    let fixed = linter
        .fix(Path::new("/p/views.php"), changed)
        .unwrap()
        .source;
    set_project_codebase(None);
    assert_eq!(fixed, changed);
}

/// Classes Composer autoloads are scanned on demand, and `ClassMethodReturnTypeOverrideGuard` lets a method override
/// a parent's untyped method only when both are in vendor/ or neither is. Expected: Rector 2.6.1
/// (`ReturnTypeFromStrictConstantReturnRector` alone, PHP 8.4, `--autoload-file vendor/autoload.php` of a project
/// whose psr-4 maps `Acme\` to vendor/acme/pkg/src/) types only the method whose parent is the project's.
#[test]
fn autoloaded_vendor_parents_guard_overrides() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
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

/// Two class names that autoload to a file declaring neither: the walk scans that file for the first, and again for
/// the second (the files are read and scanned ahead of the walk, once each). Expected (by hand, as in the test
/// above): the method overriding the vendor parent's untyped method stays untyped.
#[test]
fn a_file_autoloaded_for_two_names_is_scanned_for_each() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    use pun_lint::rules::AddReturnTypeFromConstant;
    use pun_lint::rules::facts::project_codebase_autoloaded;
    let commands = "<?php\n\nnamespace App;\n\nfinal class FromVendor extends \\Acme\\Base\n{\n    public const OK = 1;\n\n    public function run()\n    {\n        return self::OK;\n    }\n\n    public function take(\\Acme\\Missing $a, \\Acme\\Gone $b): void\n    {\n    }\n}\n";
    let base = "<?php\n\nnamespace Acme;\n\nclass Base\n{\n    public function run()\n    {\n        return 0;\n    }\n}\n";
    let files = [(
        "/project/app/Commands.php".to_string(),
        commands.to_string(),
    )];
    let autoload = |class: &[u8]| {
        matches!(class, b"acme\\base" | b"acme\\missing" | b"acme\\gone").then(|| {
            (
                "/project/vendor/acme/pkg/src/Base.php".to_string(),
                base.to_string(),
            )
        })
    };
    set_project_codebase(Some(project_codebase_autoloaded(&files, 80400, &autoload)));
    let linter = Linter::new(vec![Box::new(AddReturnTypeFromConstant)], 80400);
    let fixed = linter
        .fix(Path::new("/project/app/Commands.php"), commands)
        .unwrap()
        .source;
    set_project_codebase(None);
    assert_eq!(fixed, commands);
}
