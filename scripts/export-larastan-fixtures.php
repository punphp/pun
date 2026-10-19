<?php

declare(strict_types=1);

/*
 * Larastan 3.10.0's own rule tests as oracle fixtures for `pun analyse` (context/plan/analyse.md §8).
 *
 * For every RuleTestCase under <larastan>/tests/Rules, each test method's `analyse([...files], ...)` call becomes
 * one case (collected by export-rule-fixtures.php --collect). Its files go through the real PHPStan 2.2.7 with
 * Larastan, at level 0, from inside the Larastan checkout (so Larastan boots its Testbench application), with the
 * test's own config files and every opt-in Larastan rule switched on. Every error it reports is recorded.
 *
 * <larastan> is a checkout of larastan/larastan v3.10.0 with `composer install` done (LARASTAN_DIR, default
 * target-bench/larastan, then ~/Projects/pun-wf/scratch/larastan). Writes
 * crates/pun-analyse/tests/fixtures/larastan/<Dir>/<TestClass>.json and copies the PHP files used into
 * crates/pun-analyse/tests/fixtures/larastan/data/ (same paths as under <larastan>/tests), and the test application's
 * migrations into crates/pun-analyse/tests/fixtures/larastan/application/.
 *
 *   php scripts/export-larastan-fixtures.php [TestClassShortName...]
 */

$root = dirname(__DIR__);
if (($argv[1] ?? '') === '--collect') {
    collect($argv[2]);
    exit(0);
}
$larastan = getenv('LARASTAN_DIR') ?: null;
foreach ([$root.'/target-bench/larastan', getenv('HOME').'/Projects/pun-wf/scratch/larastan'] as $candidate) {
    if ($larastan === null && is_file($candidate.'/vendor/bin/phpstan')) {
        $larastan = $candidate;
    }
}
if ($larastan === null || ! is_file($larastan.'/vendor/bin/phpstan')) {
    fwrite(STDERR, "no Larastan checkout with vendor/: set LARASTAN_DIR\n");
    exit(1);
}
$larastan = realpath($larastan);
$tests = $larastan.'/tests';
$out = $root.'/crates/pun-analyse/tests/fixtures/larastan';
$data = $out.'/data';
$work = $root.'/target-analyse/tmp/larastan-fixtures';
@mkdir($work.'/cache', 0777, true);

// PHPStan 2.2 switches to its agent output format when it sees one of these (src/Internal/AgentDetector.php).
const AGENT_UNSET = '-u AUGMENT_AGENT -u AMP_CURRENT_THREAD_ID -u AI_AGENT -u CURSOR_TRACE_ID -u CURSOR_AGENT -u GEMINI_CLI -u CODEX_SANDBOX -u CODEX_THREAD_ID -u OPENCODE_CLIENT -u OPENCODE -u CLAUDECODE -u CLAUDE_CODE -u REPL_ID';

// the opt-in rules' switches (extension.neon's `phpstan.rules.rule: %...%` tags that default to false)
const OPT_IN = ['checkOctaneCompatibility', 'noUnnecessaryEnumerableToArrayCalls', 'checkUnusedViews', 'checkMissingTranslations', 'checkModelMethodVisibility', 'checkAuthCallsWhenInRequestScope'];

// what a test's getRule() or setUp() configures beyond the defaults, as parameters (relative to <larastan>/tests/Rules)
const PARAMETERS = [
    'NoEnvCallsOutsideOfConfigRuleTest' => ['configDirectories' => ['data/config', 'data/module/*/config']],
    // the Testbench application's migrations, which the test boots with and the command line does not
    'ModelAppendsRuleTest' => ['databaseMigrationsPath' => ['../application/database/migrations']],
];

$only = array_slice($argv, 1);
$it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tests.'/Rules'));
$testFiles = [];
foreach ($it as $f) {
    if (str_ends_with($f->getFilename(), 'RuleTest.php')) {
        $testFiles[] = $f->getPathname();
    }
}
sort($testFiles);

$copied = [];
$copy = function (string $file) use ($tests, $data, &$copied): ?string {
    $real = realpath($file);
    if ($real === false || ! str_starts_with($real, $tests.'/')) {
        return null;
    }
    $rel = substr($real, strlen($tests) + 1);
    if (! isset($copied[$rel])) {
        @mkdir(dirname($data.'/'.$rel), 0777, true);
        copy($real, $data.'/'.$rel);
        $copied[$rel] = true;
    }

    return $rel;
};

// one PHPStan run over files under <larastan>/tests: its errors, sorted, or null when it crashed
$run = function (array $configs, array $params, array $files, array $bootstrap = []) use ($larastan, $tests, $work): ?array {
    $neon = "includes:\n".implode('', array_map(fn ($c) => "\t- '$c'\n", $configs));
    $neon .= "parameters:\n\tlevel: 0\n\ttmpDir: $work/cache\n\tparallel:\n\t\tmaximumNumberOfProcesses: 1\n";
    $neon .= $bootstrap === [] ? '' : "\tbootstrapFiles:\n".implode('', array_map(fn ($f) => "\t\t- '$tests/$f'\n", $bootstrap));
    foreach (OPT_IN as $switch) {
        $neon .= "\t$switch: true\n";
    }
    foreach ($params as $key => $paths) {
        $neon .= "\t$key:\n".implode('', array_map(fn ($p) => "\t\t- '$tests/Rules/$p'\n", $paths));
    }
    file_put_contents($work.'/phpstan.neon', $neon);
    $cmd = array_merge([$larastan.'/vendor/bin/phpstan', 'analyse', '-c', $work.'/phpstan.neon', '--no-progress', '--error-format=json', '--memory-limit=2G'], array_map(fn ($f) => $tests.'/'.$f, $files));
    $output = shell_exec('cd '.escapeshellarg($larastan).' && nice env '.AGENT_UNSET.' XDEBUG_MODE=off '.implode(' ', array_map('escapeshellarg', $cmd)).' 2>/dev/null');
    $r = json_decode((string) $output, true);
    if (! is_array($r)) {
        return null;
    }
    $errors = [];
    foreach ($r['files'] as $file => $d) {
        foreach ($d['messages'] as $msg) {
            $errors[] = ['file' => str_replace($tests.'/', '', $file), 'line' => $msg['line'] ?? null, 'identifier' => $msg['identifier'] ?? null, 'message' => $msg['message'], 'tip' => $msg['tip'] ?? null];
        }
    }
    usort($errors, fn ($a, $b) => [$a['file'], $a['line'], $a['identifier'], $a['message']] <=> [$b['file'], $b['line'], $b['identifier'], $b['message']]);

    return $errors;
};

// the test application's migrations, for the model rules (Pun's tests use application/ as the Laravel root)
foreach (glob($tests.'/application/database/migrations/*.php') as $migration) {
    @mkdir($out.'/application/database/migrations', 0777, true);
    copy($migration, $out.'/application/database/migrations/'.basename($migration));
}

$runs = 0;
foreach ($testFiles as $test) {
    $short = basename($test, '.php');
    if ($only !== [] && ! in_array($short, $only, true)) {
        continue;
    }
    $json = shell_exec(implode(' ', array_map('escapeshellarg', ['php', '-d', 'display_errors=stderr', __FILE__, '--collect', $test])).' 2>/dev/null');
    $collected = json_decode((string) $json, true);
    if (! is_array($collected)) {
        fwrite(STDERR, "$short: could not collect\n");
        continue;
    }
    // getAdditionalConfigFiles(), else the rules' own config
    $source = file_get_contents($test);
    $configs = [];
    if (preg_match('~function getAdditionalConfigFiles\(\).*?\{(.*?)\n    \}~s', $source, $m)) {
        preg_match_all("~__DIR__\s*\.\s*'([^']+\.neon)'~", $m[1], $neons);
        $configs = array_map(fn ($n) => realpath(dirname($test).$n), $neons[1]);
    }
    if ($configs === []) {
        $configs = [$tests.'/Rules/phpstan-rules.neon'];
    }
    $name = substr(dirname($test), strlen($tests) + 1).'/'.$short;
    $result = ['test' => $name, 'cases' => []];
    foreach ($collected as $case) {
        $files = array_values(array_filter(array_map($copy, $case['files'])));
        if ($files === [] || count($files) !== count($case['files'])) {
            continue;
        }
        $runs++;
        $errors = $run($configs, PARAMETERS[$short] ?? [], $files);
        if ($errors === null) {
            fwrite(STDERR, "$name::{$case['method']}: phpstan crashed\n");
            continue;
        }
        $result['cases'][] = ['method' => $case['method'], 'files' => $files, 'errors' => $errors];
    }
    @mkdir(dirname($out.'/'.$name), 0777, true);
    file_put_contents($out.'/'.$name.'.json', json_encode($result, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
    fwrite(STDERR, sprintf("%s: %d cases\n", $name, count($result['cases'])));
}
// Pun's own cases for the branches Larastan's tests miss: each data/Pun/*.php is one case, analysed from a copy
// under <larastan>/tests/Pun (so it boots the same application) and loaded as a bootstrap file (so its classes exist
// at run time, as an app's autoloaded classes do: Larastan instantiates models)
if ($only === [] || in_array('Pun', $only, true)) {
    @mkdir($tests.'/Pun');
    $result = ['test' => 'Pun/Cases', 'cases' => []];
    $pun = glob($data.'/Pun/*.php');
    sort($pun);
    foreach ($pun as $file) {
        $rel = 'Pun/'.basename($file);
        copy($file, $tests.'/'.$rel);
        $runs++;
        $errors = $run([$tests.'/Rules/phpstan-rules.neon'], [], [$rel], [$rel]);
        if ($errors === null) {
            fwrite(STDERR, "$rel: phpstan crashed\n");
            continue;
        }
        $result['cases'][] = ['method' => basename($file, '.php'), 'files' => [$rel], 'errors' => $errors];
    }
    @mkdir($out.'/Pun');
    file_put_contents($out.'/Pun/Cases.json', json_encode($result, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
    fwrite(STDERR, sprintf("Pun/Cases: %d cases\n", count($result['cases'])));
}
fwrite(STDERR, "$runs phpstan runs\n");

/*
 * Child process: load one test class over stub PHPStan/PHPUnit base classes, call every test method (`test*` or
 * `#[Test]`, data providers expanded) and print the analyse() calls as JSON.
 */
function collect(string $testFile): void
{
    $testFile = realpath($testFile);
    eval(<<<'PHP'
        namespace PHPUnit\Framework { abstract class TestCase {
            public function __call($n, $a) { return null; }
            public static function __callStatic($n, $a) { return null; }
        } }
        namespace PHPStan\Testing {
            abstract class PHPStanTestCase extends \PHPUnit\Framework\TestCase {}
            abstract class RuleTestCase extends PHPStanTestCase {
                public array $calls = [];
                public function analyse(array $files, array $expectedErrors): void { $this->calls[] = $files; }
                public function gatherAnalyserErrors(array $files): array { $this->calls[] = $files; return []; }
                public function fix(string $file, string $expectedFile): void {}
            }
        }
        PHP);
    $before = get_declared_classes();
    require_once $testFile;
    $class = null;
    foreach (array_diff(get_declared_classes(), $before) as $c) {
        if (str_ends_with($testFile, '/'.substr($c, strrpos($c, '\\') + 1).'.php')) {
            $class = $c;
        }
    }
    $rc = new ReflectionClass($class);
    $cases = [];
    foreach ($rc->getMethods(ReflectionMethod::IS_PUBLIC) as $m) {
        $isTest = str_starts_with($m->getName(), 'test');
        foreach ($m->getAttributes() as $a) {
            $isTest = $isTest || str_ends_with($a->getName(), 'Test');
        }
        if (! $isTest || $m->getDeclaringClass()->getName() !== $class) {
            continue;
        }
        $t = $rc->newInstanceWithoutConstructor();
        try {
            $m->invoke($t);
        } catch (Throwable) {
        }
        foreach ($t->calls as $j => $files) {
            $cases[] = ['method' => $m->getName().(count($t->calls) > 1 ? "/$j" : ''), 'files' => $files];
        }
    }
    echo json_encode($cases, JSON_UNESCAPED_SLASHES);
}
