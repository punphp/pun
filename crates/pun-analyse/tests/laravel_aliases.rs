//! Alias identity, not facade-service forwarding. Expectations come from PHPStan 2.3.1 with Laravel 13.23.0's
//! AliasLoader/Facade/PackageManifest (Pinkary's installed vendor), and the reduced installed defaults below.
//! Runtime cold-case and loaded-class collision controls use a fresh PHP process for each AliasLoader probe.
//! Mago's constructors print canonical ReflectionClass::getName() identities; PHPStan's dumpType and Mago's
//! signature types retain the written alias. Do not require invalid abstract constructions to infer a useful type.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use pun_analyse::{Larastan, Laravel, Project, run, run_mago};
use pun_phpstan::config::{Options, resolve};

struct App(PathBuf);

impl App {
    fn new(probe: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "pun-aliases-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let app = Self(root);
        app.put(
            "composer.json",
            r#"{"autoload":{"psr-4":{"App\\":"app/"}}}"#,
        );
        app.put("vendor/composer/installed.json", r#"{"packages":[{"name":"laravel/framework","install-path":"../laravel/framework","autoload":{"psr-4":{"Illuminate\\":"src/Illuminate/"}}}]}"#);
        app.put(
            "vendor/laravel/framework/src/Illuminate/Support/Facades/Facade.php",
            include_str!("fixtures/laravel-aliases/Facade.php"),
        );
        app.put("vendor/laravel/framework/config/app.php", "<?php return ['aliases' => Illuminate\\Support\\Facades\\Facade::defaultAliases()->merge([])->toArray()];");
        app.put(
            "vendor/laravel/framework/src/Illuminate/Support/Facades/Blade.php",
            "<?php namespace Illuminate\\Support\\Facades; class Blade extends Facade {} ",
        );
        app.put(
            "vendor/laravel/framework/src/Illuminate/Support/Arr.php",
            "<?php namespace Illuminate\\Support; class Arr {} ",
        );
        app.put(
            "vendor/laravel/framework/src/Illuminate/Database/Eloquent/Model.php",
            "<?php namespace Illuminate\\Database\\Eloquent; abstract class Model {} ",
        );
        app.put("app/Custom.php", "<?php namespace App; class Custom { public static function native(): int { return 1; } } ");
        app.put("app/Other.php", "<?php namespace App; class Other { public static function native(): string { return 'other'; } } ");
        app.put("probe.php", probe);
        app.put(
            "phpstan.neon",
            "parameters:\n    level: 0\n    paths:\n        - probe.php\n",
        );
        app
    }

    fn put(&self, path: &str, code: &str) {
        let path = self.0.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, code).unwrap();
    }

    fn project(&self) -> Project {
        let config = resolve(&Options {
            config: Some(self.0.join("phpstan.neon").to_string_lossy().into_owned()),
            ..Options::for_project(&self.0.to_string_lossy())
        })
        .unwrap();
        let mut project = Project::from_config(&self.0, &config.parameters);
        project.larastan = Some(Larastan::new(&self.0));
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_CLASS_CASE_SENSITIVITY,
            true,
        );
        project
    }

    fn unknown(&self) -> Vec<String> {
        run(&self.project())
            .unwrap()
            .into_iter()
            // Dump probes also feed `types()`; this assertion checks the other alias diagnostics.
            .filter(|e| e.identifier != "phpstan.dumpType")
            .map(|e| e.message)
            .collect()
    }

    fn types(&self) -> Vec<String> {
        run_mago(&self.project(), None)
            .unwrap()
            .dumped
            .into_iter()
            .map(|d| d.type_id)
            .collect()
    }
}

impl Drop for App {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn installed_defaults_share_target_identity_with_class_checks_and_inference() {
    let app = App::new(
        "<?php\nnamespace { use Blade as Template; \n\\PHPStan\\dumpType(new \\Blade);\n\\PHPStan\\dumpType(new Template);\n\\PHPStan\\dumpType(new \\Arr);\nnew \\Eloquent;\n\\PHPStan\\dumpType(new \\Illuminate\\Support\\Facades\\Blade);\n}\nnamespace App { new Blade; }\n",
    );
    assert_eq!(
        app.unknown(),
        [
            "Instantiated class Illuminate\\Database\\Eloquent\\Model is abstract.",
            "Instantiated class App\\Blade not found."
        ]
    );
    assert_eq!(
        app.types(),
        [
            "Illuminate\\Support\\Facades\\Blade",
            "Illuminate\\Support\\Facades\\Blade",
            "Illuminate\\Support\\Arr",
            "Illuminate\\Support\\Facades\\Blade"
        ]
    );
}

#[test]
fn literal_replacement_empty_and_standard_defaults_merge() {
    let app = App::new("<?php new \\Blade; \\PHPStan\\dumpType(new \\CustomAlias); ");
    app.put(
        "config/app.php",
        "<?php use App\\Custom; return ['aliases' => ['CustomAlias' => Custom::class]];",
    );
    assert_eq!(app.unknown(), ["Instantiated class Blade not found."]);
    assert_eq!(app.types(), ["App\\Custom"]);
    app.put("config/app.php", "<?php return ['aliases' => []];");
    assert_eq!(
        app.unknown(),
        [
            "Instantiated class Blade not found.",
            "Instantiated class CustomAlias not found."
        ]
    );
    app.put("config/app.php", "<?php use Illuminate\\Support\\Facades\\Facade; use App\\Custom; return ['aliases' => Facade::defaultAliases()->merge(['CustomAlias' => Custom::class, 'Blade' => Custom::class])->toArray()];");
    assert_eq!(app.unknown(), Vec::<String>::new());
    assert_eq!(app.types(), ["App\\Custom"]);
}

#[test]
fn cached_config_and_package_manifest_override_source_configuration() {
    let app = App::new(
        "<?php\n\\PHPStan\\dumpType(new \\Blade);\n\\PHPStan\\dumpType(new \\PackageAlias);\n\\PHPStan\\dumpType(CachedAlias::native());\n",
    );
    app.put("config/app.php", "<?php return ['aliases' => []];");
    app.put(
        "bootstrap/cache/config.php",
        "<?php return ['app' => ['aliases' => ['Blade' => 'App\\\\Custom', 'CachedAlias' => App\\Custom::class]]];",
    );
    app.put("bootstrap/cache/packages.php", "<?php return ['vendor/package' => ['aliases' => ['PackageAlias' => App\\Other::class, 'Blade' => App\\Other::class], 'providers' => []]];");
    assert_eq!(app.unknown(), Vec::<String>::new());
    assert_eq!(app.types(), ["App\\Other", "App\\Other", "int"]);
}

#[test]
fn package_discovery_honours_root_and_package_dont_discover() {
    let app =
        App::new("<?php new \\Blade; \\PHPStan\\dumpType(new \\PackageAlias); new \\IgnoredAlias;");
    app.put("vendor/composer/installed.json", r#"[{"name":"vendor/one","extra":{"laravel":{"aliases":{"PackageAlias":"App\\Custom"},"dont-discover":["vendor/two"]}}},{"name":"vendor/two","extra":{"laravel":{"aliases":{"IgnoredAlias":"App\\Other"}}}}]"#);
    // Discovery still finds installed targets through the root package's Composer declarations.
    app.put("composer.json", r#"{"autoload":{"psr-4":{"App\\":"app/","Illuminate\\":"vendor/laravel/framework/src/Illuminate/"}}}"#);
    assert_eq!(
        app.unknown(),
        ["Instantiated class IgnoredAlias not found."]
    );
    assert_eq!(app.types(), ["App\\Custom"]);
    // Laravel computes ignoreAll from the root before collecting package exclusions. A package's '*' is
    // not a wildcard, although its explicit package exclusions still apply (PackageManifest::build()).
    app.put("vendor/composer/installed.json", r#"[{"name":"vendor/one","extra":{"laravel":{"aliases":{"PackageAlias":"App\\Custom"},"dont-discover":["*","vendor/two"]}}},{"name":"vendor/two","extra":{"laravel":{"aliases":{"IgnoredAlias":"App\\Other"}}}}]"#);
    assert_eq!(
        app.unknown(),
        ["Instantiated class IgnoredAlias not found."]
    );
    assert_eq!(app.types(), ["App\\Custom"]);
    app.put("composer.json", r#"{"autoload":{"psr-4":{"App\\":"app/","Illuminate\\":"vendor/laravel/framework/src/Illuminate/"}},"extra":{"laravel":{"dont-discover":["vendor/one"]}}}"#);
    assert_eq!(
        app.unknown(),
        [
            "Instantiated class IgnoredAlias not found.",
            "Instantiated class PackageAlias not found."
        ]
    );
    app.put("composer.json", r#"{"autoload":{"psr-4":{"App\\":"app/","Illuminate\\":"vendor/laravel/framework/src/Illuminate/"}},"extra":{"laravel":{"dont-discover":["*"]}}}"#);
    assert_eq!(
        app.unknown(),
        [
            "Instantiated class IgnoredAlias not found.",
            "Instantiated class PackageAlias not found."
        ]
    );
}

#[test]
fn cold_case_and_real_class_collision_do_not_invent_aliases() {
    let app = App::new("<?php new \\blade;");
    assert_eq!(app.unknown(), ["Instantiated class blade not found."]);
    app.put(
        "probe.php",
        "<?php new \\Illuminate\\Support\\Facades\\Blade; new \\blade;",
    );
    assert_eq!(app.unknown(), ["Instantiated class blade not found."]);
    app.put("probe.php", "<?php new \\Blade; new \\blade;");
    assert_eq!(app.unknown(), Vec::<String>::new());
    // Composer autoload.files are executed before the alias loader boots: an already-loaded real class wins.
    app.put(
        "app/Native.php",
        "<?php class Blade { public static function native(): bool { return true; } }",
    );
    app.put(
        "composer.json",
        r#"{"autoload":{"psr-4":{"App\\":"app/"},"files":["app/Native.php"]}}"#,
    );
    app.put("probe.php", "<?php \\PHPStan\\dumpType(Blade::native());");
    assert_eq!(app.unknown(), Vec::<String>::new());
    assert_eq!(app.types(), ["bool"]);
}

#[test]
fn namespace_aliases_chains_and_native_members_share_the_codex() {
    let app = App::new(
        "<?php\nnamespace Consumer;\nuse Tools\\{Alias as Imported};\n\\PHPStan\\dumpType(Imported::native());\n\\PHPStan\\dumpType(new \\Chain);\nnew Alias;\n",
    );
    app.put("config/app.php", "<?php return ['aliases' => ['Tools\\\\Alias' => App\\Custom::class, 'Chain' => 'Tools\\\\Alias']];");
    assert_eq!(
        app.unknown(),
        ["Instantiated class Consumer\\Alias not found."]
    );
    assert_eq!(app.types(), ["int", "App\\Custom"]);
    app.put("probe.php", "<?php new \\CycleA; new \\CycleB;");
    app.put(
        "config/app.php",
        "<?php return ['aliases' => ['CycleA' => 'CycleB', 'CycleB' => 'CycleA']];",
    );
    assert_eq!(
        app.unknown(),
        [
            "Instantiated class CycleA not found.",
            "Instantiated class CycleB not found."
        ]
    );
}

#[test]
fn unused_alias_targets_remain_lazy_and_no_context_means_no_laravel_aliases() {
    let app = App::new("<?php namespace UnusedAlias; \\PHPStan\\dumpType(\\UsedAlias::native());");
    app.put("config/app.php", "<?php return ['aliases' => ['UsedAlias' => App\\Custom::class, 'UnusedAlias' => App\\Other::class]];");
    let mut project = app.project();
    let unused = project
        .autoloaded
        .iter_mut()
        .find(|a| a.class == b"app\\other")
        .unwrap();
    unused.path = app.0.clone(); // A directory errors if read, even as root.
    assert_eq!(
        run(&project)
            .unwrap()
            .into_iter()
            .map(|e| e.message)
            .collect::<Vec<_>>(),
        ["Dumped type: int"]
    );
    assert_eq!(run_mago(&project, None).unwrap().dumped[0].type_id, "int");
    app.put("probe.php", "<?php new \\UnusedAlias;");
    assert_eq!(run(&project).unwrap_err().path, app.0);
    project.larastan = None;
    assert_eq!(
        run(&project).unwrap()[0].message,
        "Instantiated class UnusedAlias not found."
    );
}

#[test]
fn aliases_in_signatures_and_phpdoc_are_visible_to_inference() {
    let app = App::new(
        "<?php\n/** @param Blade $blade */\nfunction identity($blade): \\Blade { return $blade; }\n\\PHPStan\\dumpType(identity(new \\Blade));\n",
    );
    assert_eq!(app.unknown(), Vec::<String>::new());
    assert_eq!(app.types(), ["Blade"]);
    // Only the PHPDoc asks for the alias; it must not depend on an executable reference warming it first.
    app.put("probe.php", "<?php\n/** @return Blade */\nfunction identity() { throw new \\Exception; }\n\\PHPStan\\dumpType(identity());\n");
    assert_eq!(app.unknown(), Vec::<String>::new());
    assert_eq!(app.types(), ["Blade"]);
    app.put(
        "config/app.php",
        "<?php return ['aliases' => ['Blade' => App\\Custom::class]];",
    );
    app.put("probe.php", "<?php\n/** @return Blade */\nfunction identity() { throw new \\Exception; }\n\\PHPStan\\dumpType(identity()::native());\n");
    assert_eq!(app.unknown(), Vec::<String>::new());
    assert_eq!(app.types(), ["int"]);
    app.put("config/app.php", "<?php return ['aliases' => ['Tools\\\\Alias' => App\\Custom::class, 'Consumer\\\\Local' => App\\Other::class]];");
    app.put("probe.php", "<?php\nnamespace Consumer;\nuse Tools as T;\n/** @return T\\Alias */\nfunction imported() { throw new \\Exception; }\n/** @return Local */\nfunction local() { throw new \\Exception; }\n\\PHPStan\\dumpType(imported()::native());\n\\PHPStan\\dumpType(local()::native());\n");
    assert_eq!(app.unknown(), Vec::<String>::new());
    assert_eq!(app.types(), ["int", "string"]);
}

#[test]
fn a_package_manifest_is_authoritative_even_when_discovery_would_find_more() {
    let app = App::new("<?php new PackageAlias; \\PHPStan\\dumpType(Blade::native());");
    app.put(
        "config/app.php",
        "<?php return ['aliases' => ['Blade' => App\\Custom::class]];",
    );
    app.put("vendor/composer/installed.json", r#"{"packages":[{"name":"vendor/package","extra":{"laravel":{"aliases":{"PackageAlias":"App\\Other"}}}}]}"#);
    app.put("bootstrap/cache/packages.php", "<?php return [];");
    assert_eq!(
        app.unknown(),
        ["Instantiated class PackageAlias not found."]
    );
    assert_eq!(app.types(), ["int"]);
    // flatMap() overwrites equal keys before filter(); an empty later package alias removes the package key.
    app.put("bootstrap/cache/packages.php", "<?php return ['z-first' => ['aliases' => ['Blade' => App\\Other::class]], 'a-last' => ['aliases' => ['Blade' => '']]];");
    assert_eq!(app.types(), ["int"]);
    app.put(
        "bootstrap/cache/packages.php",
        "<?php return app('manifest');",
    );
    assert_eq!(
        app.unknown(),
        ["Instantiated class PackageAlias not found."]
    );
}

#[test]
fn dynamic_configuration_and_missing_targets_stay_unknown() {
    let app = App::new("<?php new \\Blade; new \\CustomAlias;");
    app.put(
        "config/app.php",
        "<?php return ['aliases' => app('aliases')];",
    );
    assert_eq!(
        app.unknown(),
        [
            "Instantiated class Blade not found.",
            "Instantiated class CustomAlias not found."
        ]
    );
    app.put("config/app.php", "<?php return ['aliases' => ['Blade' => 'MissingTarget', 'CustomAlias' => 'MissingTarget']];");
    assert_eq!(
        app.unknown(),
        [
            "Instantiated class Blade not found.",
            "Instantiated class CustomAlias not found."
        ]
    );
}

#[test]
fn literal_forms_and_unknown_overrides_do_not_invent_defaults() {
    let app = App::new("<?php \\PHPStan\\dumpType(Blade::native());");
    app.put(
        "config/app.php",
        "<?php return array('aliases' => array('Blade' => (App\\Custom::class)));",
    );
    assert_eq!(app.types(), ["int"]);
    app.put("config/app.php", "<?php use Illuminate\\Support\\Facades\\Facade; return ['aliases' => Facade::defaultAliases()->merge(['Blade' => App\\Custom::class])->all()];");
    assert_eq!(app.types(), ["int"]);
    app.put("probe.php", "<?php new \\Blade;");
    // These values cannot establish a literal alias. In particular, an unknown replacement must not resurrect
    // a previous duplicate key or installed defaults. This is the native reader's conservative bootstrap limit.
    for code in [
        "<?php return ['aliases' => ['Blade' => App\\Custom::class, 'Blade' => getenv('BLADE_CLASS')]];",
        "<?php return ['aliases' => ['Blade' => $class::class]];",
        "<?php return ['aliases' => ['Blade' => App\\Custom::TARGET]];",
        "<?php return ['aliases' => [...getAliases()]];",
        "<?php return ['aliases' => [App\\Custom::class]];",
        "<?php return ['aliases' => [1 => App\\Custom::class]];",
        "<?php use Illuminate\\Support\\Facades\\Facade; return ['aliases' => Facade::defaultAliases()->merge(getAliases())->toArray()];",
        "<?php use Illuminate\\Support\\Facades\\Facade; return ['aliases' => Facade::defaultAliases()->merge()->toArray()];",
        "<?php return ['aliases' => collect([])->merge([])->toArray()];",
        "<?php return ['aliases' => collect([])->toArray()];",
        "<?php return;",
        "<?php function aliases() { return ['Blade' => App\\Custom::class]; }",
        "<?php return [;",
    ] {
        app.put("config/app.php", code);
        assert_eq!(
            app.unknown(),
            ["Instantiated class Blade not found."],
            "{code}"
        );
    }
    app.put(
        "config/app.php",
        "<?php return ['name' => 'no alias override'];",
    );
    assert_eq!(app.unknown(), Vec::<String>::new());
    std::fs::remove_file(app.0.join("vendor/laravel/framework/config/app.php")).unwrap();
    assert_eq!(app.unknown(), ["Instantiated class Blade not found."]);
    app.put(
        "config/app.php",
        "<?php return ['aliases' => ['Blade' => App\\Custom::class]];",
    );
    app.put("bootstrap/cache/config.php", "<?php return ['app' => []];");
    assert_eq!(app.unknown(), ["Instantiated class Blade not found."]);
    app.put(
        "bootstrap/cache/config.php",
        "<?php return ['app' => null];",
    );
    assert_eq!(app.unknown(), ["Instantiated class Blade not found."]);
    std::fs::remove_file(app.0.join("bootstrap/cache/config.php")).unwrap();
    std::fs::remove_file(
        app.0
            .join("vendor/laravel/framework/src/Illuminate/Support/Facades/Facade.php"),
    )
    .unwrap();
    assert_eq!(app.unknown(), Vec::<String>::new());
}

#[test]
fn explicit_laravel_context_uses_the_same_alias_registry() {
    let app = App::new("<?php \\PHPStan\\dumpType(new \\Blade);");
    let mut project = app.project();
    project.larastan = None;
    let result = run_mago(
        &project,
        Some(Laravel {
            root: Path::new(&app.0),
            parse_casts_method: false,
        }),
    )
    .unwrap();
    assert_eq!(
        result
            .dumped
            .into_iter()
            .map(|d| d.type_id)
            .collect::<Vec<_>>(),
        ["Illuminate\\Support\\Facades\\Blade"]
    );
}
