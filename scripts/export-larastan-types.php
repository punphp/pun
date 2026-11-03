<?php

declare(strict_types=1);

/*
 * Larastan 3.10.0's type tests as oracle fixtures for `pun analyse` (tests/larastan_types.rs).
 *
 * Larastan's tests/Type/*Test.php (TypeInferenceTestCase) gather the `assertType()`, `assertNativeType()`,
 * `assertSuperType()` and `assertVariableCertainty()` calls of their data files, and tests/Integration/IntegrationTest.php
 * analyses its data files with PHPStan's level-8 test container. Each test class's files go through the real
 * PHPStan 2.3.1 with Larastan from inside the Larastan checkout (so Larastan boots its Testbench application, as in
 * export-larastan-fixtures.php), with the test's getAdditionalConfigFiles() at level 8 (PHPStanTestCase's
 * config.level8.neon). FileAssertRule runs at every level, so the assertions that do not hold are among the errors,
 * and every error is recorded.
 *
 * <larastan> is a checkout of larastan/larastan v3.10.0 with `composer install` done and PHPStan 2.3.1 in
 * vendor/phpstan/phpstan (LARASTAN_DIR, default target-bench/larastan). Writes
 * crates/pun-analyse/tests/fixtures/larastan-types/<Dir>/<TestClass>.json, with each file's assertion lines, and copies
 * the checkout's files Pun needs below crates/pun-analyse/tests/fixtures/larastan-types/root/: tests/ (the data files,
 * Larastan's test application and configs), src/, extension.neon and stubs, Testbench's skeleton, and the vendor files
 * of the list in vendor.txt (the files Pun runs on the full checkout read, refreshed with `--vendor <file of paths>`),
 * with their function bodies cut. tests/larastan_types.rs with PUN_LARASTAN_TYPES_ROOT=<larastan> checks that the copy
 * gives the full checkout's result.
 *
 *   php scripts/export-larastan-types.php [--vendor paths.txt] [TestClassShortName...]
 *
 * (A name no test has, such as `none`, only copies the files.)
 */

$root = dirname(__DIR__);
if (($argv[1] ?? '') === '--collect') {
    collect($argv[2], $argv[3]);
    exit(0);
}
$larastan = realpath(getenv('LARASTAN_DIR') ?: $root.'/target-bench/larastan');
if ($larastan === false || ! is_file($larastan.'/vendor/bin/phpstan')) {
    fwrite(STDERR, "no Larastan checkout with vendor/: set LARASTAN_DIR\n");
    exit(1);
}
// php-parser, for the declared names and the cut bodies
require_once $larastan.'/vendor/autoload.php';
$tests = $larastan.'/tests';
$out = $root.'/crates/pun-analyse/tests/fixtures/larastan-types';
$copyRoot = $out.'/root';
$work = $root.'/target-analyse/tmp/larastan-types';
@mkdir($work.'/cache', 0777, true);

$args = array_slice($argv, 1);
$vendorList = null;
$vendor = null;
if (($args[0] ?? '') === '--vendor') {
    $vendorList = $args[1];
    $args = array_slice($args, 2);
}
$only = $args;

// PHPStan 2.2 switches to its agent output format when it sees one of these (src/Internal/AgentDetector.php).
const AGENT_UNSET = '-u AUGMENT_AGENT -u AMP_CURRENT_THREAD_ID -u AI_AGENT -u CURSOR_TRACE_ID -u CURSOR_AGENT -u GEMINI_CLI -u CODEX_SANDBOX -u CODEX_THREAD_ID -u OPENCODE_CLIENT -u OPENCODE -u CLAUDECODE -u CLAUDE_CODE -u REPL_ID';

// the testing functions FileAssertRule checks (TypeInferenceTestCase::gatherAssertTypes())
const ASSERTS = ['asserttype', 'assertnativetype', 'assertsupertype', 'assertvariablecertainty'];

$copy = function (string $file) use ($larastan, $copyRoot): string {
    $real = realpath($file);
    if ($real === false || ! str_starts_with($real, $larastan.'/')) {
        throw new RuntimeException("$file is outside the checkout");
    }
    $rel = substr($real, strlen($larastan) + 1);
    @mkdir(dirname($copyRoot.'/'.$rel), 0777, true);
    copy($real, $copyRoot.'/'.$rel);

    return $rel;
};

// the start line of each testing-function call, by token: a name of one of them followed by `(`, not a declaration,
// a member or an import
$assertLines = function (string $file): array {
    $tokens = token_get_all(file_get_contents($file));
    $lines = [];
    $previous = null;
    foreach ($tokens as $i => $t) {
        if (is_array($t) && in_array($t[0], [T_WHITESPACE, T_COMMENT, T_DOC_COMMENT], true)) {
            continue;
        }
        if (is_array($t) && in_array($t[0], [T_STRING, T_NAME_QUALIFIED, T_NAME_FULLY_QUALIFIED], true)) {
            $name = strtolower(substr($t[1], (int) strrpos('\\'.$t[1], '\\')));
            $j = $i + 1;
            while (is_array($tokens[$j] ?? null) && $tokens[$j][0] === T_WHITESPACE) {
                $j++;
            }
            $member = is_array($previous) && in_array($previous[0], [T_FUNCTION, T_OBJECT_OPERATOR, T_NULLSAFE_OBJECT_OPERATOR, T_DOUBLE_COLON, T_NEW], true);
            if (in_array($name, ASSERTS, true) && ($tokens[$j] ?? null) === '(' && ! $member) {
                $lines[] = $t[2];
            }
        }
        $previous = $t;
    }

    return $lines;
};

// one PHPStan run: errors by file (relative to the checkout), sorted, or null when it crashed
$run = function (array $configs, array $files) use ($larastan, $work): ?array {
    $neon = "includes:\n".implode('', array_map(fn ($c) => "\t- '$c'\n", $configs));
    $neon .= "parameters:\n\tlevel: 8\n\ttmpDir: $work/cache\n\tparallel:\n\t\tmaximumNumberOfProcesses: 1\n";
    file_put_contents($work.'/phpstan.neon', $neon);
    $cmd = array_merge(['php', '-d', 'memory_limit=-1', $larastan.'/vendor/bin/phpstan', 'analyse', '-c', $work.'/phpstan.neon', '--no-progress', '--error-format=json'], array_map(fn ($f) => $larastan.'/'.$f, $files));
    $output = shell_exec('cd '.escapeshellarg($larastan).' && nice env '.AGENT_UNSET.' XDEBUG_MODE=off '.implode(' ', array_map('escapeshellarg', $cmd)).' 2>/dev/null');
    $r = json_decode((string) $output, true);
    if (! is_array($r)) {
        return null;
    }
    foreach ($r['errors'] as $error) {
        fwrite(STDERR, "general error: $error\n");
    }
    $errors = array_fill_keys($files, []);
    foreach ($r['files'] as $file => $d) {
        $rel = str_replace($larastan.'/', '', $file);
        foreach ($d['messages'] as $msg) {
            $errors[$rel][] = ['line' => $msg['line'] ?? null, 'identifier' => $msg['identifier'] ?? null, 'message' => $msg['message'], 'tip' => $msg['tip'] ?? null];
        }
    }
    foreach ($errors as &$list) {
        usort($list, fn ($a, $b) => [$a['line'], $a['identifier'], $a['message']] <=> [$b['line'], $b['identifier'], $b['message']]);
    }

    return $errors;
};

// phpunit.xml.dist's bootstrap: the test application's migrations, schema, config, resources and commands go into
// Testbench's skeleton, the app Larastan boots
shell_exec('php '.escapeshellarg($larastan.'/phpunit-bootstrap.php'));

$testFiles = array_merge(glob($tests.'/Type/*Test.php'), [$tests.'/Integration/IntegrationTest.php']);
sort($testFiles);
$runs = 0;
foreach ($testFiles as $test) {
    $short = basename($test, '.php');
    if ($only !== [] && ! in_array($short, $only, true)) {
        continue;
    }
    $json = shell_exec(implode(' ', array_map('escapeshellarg', ['php', '-d', 'display_errors=stderr', __FILE__, '--collect', $larastan, $test])).' 2>/dev/null');
    $collected = json_decode((string) $json, true);
    if (! is_array($collected)) {
        fwrite(STDERR, "$short: could not collect\n");
        continue;
    }
    if ($collected['files'] === []) {
        // a unit test of the extensions' classes, without data files
        continue;
    }
    $configs = $collected['configs'];
    $files = array_values(array_unique(array_map($copy, array_keys($collected['files']))));
    $runs++;
    $errors = $run($configs, $files);
    if ($errors === null) {
        fwrite(STDERR, "$short: phpstan crashed\n");
        continue;
    }
    $name = substr(dirname($test), strlen($tests) + 1).'/'.$short;
    $result = [
        'test' => $name,
        'configs' => array_map(fn ($c) => substr(realpath($c), strlen($larastan) + 1), $configs),
        'cases' => [],
    ];
    foreach ($collected['files'] as $file => $larastanErrors) {
        $rel = substr(realpath($file), strlen($larastan) + 1);
        $case = ['file' => $rel, 'declares' => declared_names($file)];
        if ($larastanErrors === null) {
            $case['asserts'] = $assertLines($file);
        } else {
            // the integration test's own expectation: its errors by line, null for none
            $case['larastan'] = $larastanErrors;
        }
        $case['errors'] = $errors[$rel];
        $result['cases'][] = $case;
    }
    foreach ($configs as $config) {
        $copy($config);
    }
    @mkdir(dirname($out.'/'.$name), 0777, true);
    file_put_contents($out.'/'.$name.'.json', json_encode($result, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
    fwrite(STDERR, sprintf("%s: %d files\n", $name, count($result['cases'])));
}

// what a Pun run on the copy needs beyond the data: the rest of tests/ (Composer's classmap, with the test
// application and the configs' includes and bootstrap files), Larastan's extension and stubs, and the vendor files Pun
// reads
// Composer's metadata, and the app files Larastan reads beside the PHP it scans: Testbench's skeleton (the app it boots,
// with phpunit-bootstrap.php's copies) and the framework's default config
$needed = ['composer.json', 'extension.neon', 'bootstrap.php', 'vendor/composer/installed.json'];
foreach (['src', 'stubs', 'tests', 'vendor/orchestra/testbench-core/laravel', 'vendor/laravel/framework/config'] as $dir) {
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($larastan.'/'.$dir, FilesystemIterator::SKIP_DOTS));
    foreach ($it as $f) {
        $path = substr($f->getPathname(), strlen($larastan) + 1);
        // the skeleton's vendor is a link to the checkout's; a .gitignore would hide the copies from git
        if (! str_starts_with($path, 'vendor/orchestra/testbench-core/laravel/vendor/') && $f->getFilename() !== '.gitignore') {
            $needed[] = $path;
        }
    }
}
if ($vendorList !== null) {
    $vendor = array_values(array_filter(array_map('trim', file($vendorList)), fn ($p) => str_starts_with($p, 'vendor/')));
    sort($vendor);
    file_put_contents($out.'/vendor.txt', implode("\n", array_unique($vendor))."\n");
}
foreach (is_file($out.'/vendor.txt') ? file($out.'/vendor.txt', FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES) : [] as $path) {
    $needed[] = $path;
}
foreach ($needed as $path) {
    if (is_file($larastan.'/'.$path)) {
        $copy($larastan.'/'.$path);
    }
}
// the skeleton's vendor, as Testbench's CreateVendorSymlink links it
@unlink($copyRoot.'/vendor/orchestra/testbench-core/laravel/vendor');
symlink('../../..', $copyRoot.'/vendor/orchestra/testbench-core/laravel/vendor');
// the vendor classes' and functions' bodies, which Pun never analyses, are cut to `{}` (`{ yield; }` for a generator),
// except those Pun reads as the booted app: registerCoreContainerAliases() and Facade::defaultAliases()
const READ_BODIES = ['vendor/laravel/framework/src/Illuminate/Foundation/Application.php', 'vendor/laravel/framework/src/Illuminate/Support/Facades/Facade.php'];
$parser = (new PhpParser\ParserFactory())->createForHostVersion();
foreach ($vendor ?? file($out.'/vendor.txt', FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES) as $path) {
    if (str_ends_with($path, '.php') && is_file($copyRoot.'/'.$path) && ! in_array($path, READ_BODIES, true)) {
        file_put_contents($copyRoot.'/'.$path, strip_bodies($parser, file_get_contents($copyRoot.'/'.$path)));
    }
}
fwrite(STDERR, "$runs phpstan runs\n");

/*
 * Child process: load one test class over stub PHPStan test cases, run its data providers and print the files they
 * gather, with getAdditionalConfigFiles(). Type tests gather assertion files (null); the integration test's provider
 * yields each file with its expected errors by line.
 */
function collect(string $larastan, string $testFile): void
{
    eval(<<<'PHP'
        namespace PHPStan\Testing {
            abstract class PHPStanTestCase {
                public static array $gathered = [];
                public static function getContainer() { return null; }
                public static function __callStatic($n, $a) { return null; }
                public function __call($n, $a) { return null; }
            }
            abstract class TypeInferenceTestCase extends PHPStanTestCase {
                public static function gatherAssertTypes(string $file): array {
                    self::$gathered[$file] = null;
                    return [];
                }
            }
        }
        PHP);
    require $larastan.'/vendor/autoload.php';
    $before = get_declared_classes();
    require_once $testFile;
    $class = null;
    foreach (array_diff(get_declared_classes(), $before) as $c) {
        if (str_ends_with($testFile, '/'.substr($c, strrpos($c, '\\') + 1).'.php')) {
            $class = $c;
        }
    }
    $rc = new ReflectionClass($class);
    if (! $rc->isSubclassOf(PHPStan\Testing\PHPStanTestCase::class)) {
        echo json_encode(['files' => [], 'configs' => []]);

        return;
    }
    foreach ($rc->getMethods(ReflectionMethod::IS_STATIC) as $m) {
        if (! str_starts_with($m->getName(), 'data') || $m->getDeclaringClass()->getName() !== $class) {
            continue;
        }
        try {
            foreach ($m->invoke(null) as $row) {
                // IntegrationTest: [file, expected errors by line]
                if (is_array($row) && is_string($row[0] ?? null) && is_file($row[0])) {
                    PHPStan\Testing\PHPStanTestCase::$gathered[$row[0]] = $row[1] ?? [];
                }
            }
        } catch (Throwable) {
            // a provider that needs PHPStan's container gathers no files
        }
    }
    echo json_encode(['files' => PHPStan\Testing\PHPStanTestCase::$gathered, 'configs' => $class::getAdditionalConfigFiles()], JSON_UNESCAPED_SLASHES);
}

/** `$source` with each function's and method's body replaced, everything else byte for byte. */
function strip_bodies(PhpParser\Parser $parser, string $source): string
{
    try {
        $stmts = $parser->parse($source);
    } catch (PhpParser\Error) {
        return $source;
    }
    $finder = new PhpParser\NodeFinder();
    $bodies = [];
    $functions = $finder->find($stmts, fn ($n) => ($n instanceof PhpParser\Node\Stmt\ClassMethod || $n instanceof PhpParser\Node\Stmt\Function_) && $n->stmts !== null);
    foreach ($functions as $fn) {
        $generator = yields($fn->stmts);
        // the body's `{`: the first after the signature (its return type, else its last parameter, else its name)
        $params = $fn->getParams();
        $signature = $fn->returnType ?? ($params === [] ? $fn->name : end($params));
        $bodies[] = [strpos($source, '{', $signature->getEndFilePos() + 1), $fn->getEndFilePos(), $generator];
    }
    usort($bodies, fn ($a, $b) => $a[0] <=> $b[0]);
    $out = '';
    $at = 0;
    foreach ($bodies as [$start, $end, $generator]) {
        // a function declared inside a stripped body
        if ($start < $at) {
            continue;
        }
        $out .= substr($source, $at, $start - $at).($generator ? '{ yield; }' : '{}');
        $at = $end + 1;
    }

    return $out.substr($source, $at);
}

/** Whether `$nodes` yield, outside the closures and classes they declare. */
function yields(array $nodes): bool
{
    foreach ($nodes as $node) {
        if ($node instanceof PhpParser\Node\Expr\Yield_ || $node instanceof PhpParser\Node\Expr\YieldFrom) {
            return true;
        }
        if (! $node instanceof PhpParser\Node || $node instanceof PhpParser\Node\FunctionLike || $node instanceof PhpParser\Node\Stmt\ClassLike) {
            continue;
        }
        foreach ($node->getSubNodeNames() as $name) {
            $sub = $node->$name;
            if (yields(is_array($sub) ? $sub : [$sub])) {
                return true;
            }
        }
    }

    return false;
}

/**
 * The functions and class-likes `$file` declares, lowercased: Pun analyses files that declare the same name in separate
 * runs, as PHPStan's own scope of each file has its own declaration.
 */
function declared_names(string $file): array
{
    static $parser = null;
    $parser ??= (new PhpParser\ParserFactory())->createForHostVersion();
    $traverser = new PhpParser\NodeTraverser(new PhpParser\NodeVisitor\NameResolver());
    $stmts = $traverser->traverse($parser->parse(file_get_contents($file)));
    $names = [];
    foreach ((new PhpParser\NodeFinder())->find($stmts, fn ($n) => ($n instanceof PhpParser\Node\Stmt\Function_ || $n instanceof PhpParser\Node\Stmt\ClassLike) && isset($n->namespacedName)) as $node) {
        $names[] = strtolower($node->namespacedName->toString());
    }
    sort($names);

    return array_values(array_unique($names));
}
