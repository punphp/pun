<?php

declare(strict_types=1);

/*
 * PHPStan's own rule tests as level-0 oracle fixtures for `pun analyse`.
 *
 * For every RuleTestCase in .oracle/phpstan-src/tests/PHPStan/Rules whose rule is registered at level 0
 * (#[RegisteredRule(level: 0)] in src/Rules), each test method's `analyse([...files], ...)` call (data providers
 * expanded) becomes one case. The test's own expectations are not used: tests construct rules with non-default
 * flags, while the target is what `phpstan analyse --level 0` reports. So each case's files go through the real
 * PHAR at level 0 (one run per case, files that the method `require`s become scanFiles), and every error it
 * reports, from any rule, is recorded with its identifier and tip.
 *
 * Writes crates/pun-analyse/tests/fixtures/rules/<Dir>/<TestClass>.json and copies the PHP files used into
 * crates/pun-analyse/tests/fixtures/rules/data/ (same paths as under tests/PHPStan/Rules).
 *
 *   php scripts/export-rule-fixtures.php [--resume] [--budget=SECONDS] [--level=N] [TestClassShortName...]
 *
 * --resume skips test classes whose JSON exists; --budget stops starting new classes after that many seconds (the
 * full export is a few thousand PHPStan runs, so it is run in chunks). --level=N (default 0) exports the tests of
 * the rules registered at level N, run at level N, to crates/pun-analyse/tests/fixtures/levels/<N>/ instead (the PHP
 * files still go to rules/data/). --from=M (default N) takes the tests of the rules registered at level M instead, so
 * a level-0 rule whose checks grow with the level (FunctionCallParametersCheck, RuleLevelHelper) is exported at N.
 */

$root = dirname(__DIR__);
$src = realpath($root.'/.oracle/phpstan-src') ?: $root.'/.oracle/phpstan-src'; // .oracle may be a link (worktrees)
$rulesTests = $src.'/tests/PHPStan/Rules';
$phar = $root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar';
$out = $root.'/crates/pun-analyse/tests/fixtures/rules';
$work = $root.'/target-analyse/tmp/rule-fixtures-'.getmypid(); // per process, so exports can run side by side

if (($argv[1] ?? '') === '--collect') {
    collect($argv[2]);
    exit(0);
}

@mkdir($work.'/cache', 0777, true);
@mkdir($work.'/ini', 0777, true);
file_put_contents($work.'/ini/pun.ini', "max_memory_limit = -1\n"); // php.ini's cap also binds PHPStan's workers

// PHPStan 2.2 switches to its agent output format when it sees one of these (src/Internal/AgentDetector.php).
const AGENT_UNSET = '-u AUGMENT_AGENT -u AMP_CURRENT_THREAD_ID -u AI_AGENT -u CURSOR_TRACE_ID -u CURSOR_AGENT -u GEMINI_CLI -u CODEX_SANDBOX -u CODEX_THREAD_ID -u OPENCODE_CLIENT -u OPENCODE -u CLAUDECODE -u CLAUDE_CODE -u REPL_ID';

/** @return list<string> test files of the rules registered at $level (any rule's, for the ones named in $only) */
function levelTests(string $src, int $level, array $only = []): array
{
    $tests = [];
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($src.'/src/Rules'));
    foreach ($it as $f) {
        // a rule config.levelN.neon registers (TooWide*ThrowTypeRule) has no RegisteredRule attribute: name its test
        $named = in_array(basename($f->getFilename(), '.php').'Test', $only, true);
        if (! str_ends_with($f->getFilename(), 'Rule.php') || (! $named && ! str_contains(file_get_contents($f->getPathname()), '#[RegisteredRule(level: '.$level.')]'))) {
            continue;
        }
        $test = str_replace([$src.'/src/Rules', '.php'], [$src.'/tests/PHPStan/Rules', 'Test.php'], $f->getPathname());
        if (is_file($test)) {
            $tests[] = $test;
        }
    }
    sort($tests);

    return $tests;
}

$args = array_slice($argv, 1);
$resume = in_array('--resume', $args, true);
$budget = null;
foreach ($args as $a) {
    if (str_starts_with($a, '--budget=')) {
        $budget = (int) substr($a, 9);
    }
}
$level = 0;
foreach ($args as $a) {
    if (str_starts_with($a, '--level=')) {
        $level = (int) substr($a, 8);
    }
}
$data = $out.'/data';
if ($level > 0) {
    $out = dirname($out).'/levels/'.$level;
    $work .= '-level'.$level;
    @mkdir($work.'/cache', 0777, true);
    @mkdir($work.'/ini', 0777, true);
    file_put_contents($work.'/ini/pun.ini', "max_memory_limit = -1\n");
}
$from = $level;
foreach ($args as $a) {
    if (str_starts_with($a, '--from=')) {
        $from = (int) substr($a, 7);
    }
}
$only = array_values(array_filter($args, fn ($a) => ! str_starts_with($a, '--')));
$started = time();
$cases = [];
foreach (levelTests($src, $from, $only) as $test) {
    $short = basename($test, '.php');
    if ($only !== [] && ! in_array($short, $only, true)) {
        continue;
    }
    $json = shell_exec(implode(' ', array_map('escapeshellarg', ['php', __FILE__, '--collect', $test])).' 2>/dev/null');
    $collected = json_decode((string) $json, true);
    if (! is_array($collected)) {
        fwrite(STDERR, "$short: could not collect\n");
        continue;
    }
    $rel = substr(dirname($test), strlen($rulesTests) + 1);
    if ($resume && is_file($out.'/'.$rel.'/'.$short.'.json')) {
        continue;
    }
    $cases[$rel.'/'.$short] = $collected;
}

/** Real PHPStan, at $level, on $files with $scan as scanFiles. */
function oracle(string $phar, string $work, array $files, array $scan, int $level): array
{
    $neon = "parameters:\n\tlevel: $level\n\ttmpDir: $work/cache\n\tparallel:\n\t\tmaximumNumberOfProcesses: 1\n";
    if ($scan !== []) {
        $neon .= "\tscanFiles:\n".implode('', array_map(fn ($f) => "\t\t- '$f'\n", $scan));
    }
    file_put_contents($work.'/phpstan.neon', $neon);
    $cmd = array_merge(['php', $phar, 'analyse', '-c', $work.'/phpstan.neon', '--no-progress', '--error-format=json', '--memory-limit=1G'], $files);
    $json = shell_exec('nice env '.AGENT_UNSET.' PHP_INI_SCAN_DIR=:'.escapeshellarg($work.'/ini').' '.implode(' ', array_map('escapeshellarg', $cmd)).' 2>/dev/null');
    $result = json_decode((string) $json, true);
    if (! is_array($result)) {
        return ['crashed' => true];
    }
    $errors = [];
    foreach ($result['files'] as $file => $data) {
        foreach ($data['messages'] as $m) {
            $errors[] = ['file' => $file, 'line' => $m['line'] ?? null, 'identifier' => $m['identifier'] ?? null, 'message' => $m['message'], 'tip' => $m['tip'] ?? null];
        }
    }

    return ['errors' => $errors, 'general' => $result['errors']];
}

$copied = [];
$copy = function (string $file) use ($rulesTests, $data, &$copied): ?string {
    $real = realpath($file);
    if ($real === false || ! str_starts_with($real, $rulesTests.'/')) {
        return null;
    }
    $rel = substr($real, strlen($rulesTests) + 1);
    if (! isset($copied[$rel])) {
        @mkdir(dirname($data.'/'.$rel), 0777, true);
        copy($real, $data.'/'.$rel);
        $copied[$rel] = true;
    }

    return $rel;
};

$runs = 0;
foreach ($cases as $name => $collected) {
    if ($budget !== null && time() - $started > $budget) {
        fwrite(STDERR, "budget reached; rerun with --resume\n");
        break;
    }
    $result = ['test' => $name, 'cases' => []];
    foreach ($collected as $case) {
        $files = array_values(array_filter(array_map($copy, $case['files'])));
        $scan = array_values(array_filter(array_map($copy, $case['scan'])));
        if ($files === [] || count($files) !== count($case['files'])) {
            continue; // files outside tests/PHPStan/Rules (or missing): not a portable case
        }
        $r = oracle($phar, $work, array_map(fn ($f) => $rulesTests.'/'.$f, $files), array_map(fn ($f) => $rulesTests.'/'.$f, $scan), $level);
        $runs++;
        if (isset($r['crashed'])) {
            fwrite(STDERR, "$name::{$case['method']}: phpstan crashed\n");
            continue;
        }
        foreach ($r['errors'] as &$e) {
            $e['file'] = substr($e['file'], strlen($rulesTests) + 1);
        }
        unset($e);
        usort($r['errors'], fn ($a, $b) => [$a['file'], $a['line'], $a['identifier'], $a['message']] <=> [$b['file'], $b['line'], $b['identifier'], $b['message']]);
        $result['cases'][] = ['method' => $case['method'], 'files' => $files, 'scan' => $scan] + $r;
    }
    @mkdir(dirname($out.'/'.$name), 0777, true);
    file_put_contents($out.'/'.$name.'.json', json_encode($result, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
    fwrite(STDERR, sprintf("%s: %d cases\n", $name, count($result['cases'])));
}
fwrite(STDERR, "$runs phpstan runs\n");

/*
 * Child process: load one test class over stub PHPStan/PHPUnit base classes, call every test method (data
 * providers expanded) and print the analyse() calls as JSON. Fixture files the method body `require`s are read
 * from its source, so a file another method already included is still recorded.
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
            class Skip extends \Exception {}
            abstract class PHPStanTestCase extends \PHPUnit\Framework\TestCase {
                public static function getContainer() { throw new Skip(); }
                public static function createReflectionProvider() { throw new Skip(); }
                public static function getAdditionalConfigFiles(): array { return []; }
            }
            abstract class RuleTestCase extends PHPStanTestCase {
                public array $calls = [];
                public function analyse(array $files, array $expectedErrors): void { $this->calls[] = $files; }
                public function gatherAnalyserErrors(array $files): array { $this->calls[] = $files; return []; }
                public function fix(string $file, string $expectedFile): void {}
            }
        }
        PHP);
    spl_autoload_register(function (string $class) use ($testFile): void {
        // sibling test classes (shared bases) live next to the test
        $f = dirname($testFile).'/'.substr($class, strrpos($class, '\\') + 1).'.php';
        if (is_file($f)) {
            require_once $f;
        }
    });
    // PHPStan's own classes the expectations name (MissingTypehintCheck::MISSING_ITERABLE_VALUE_TYPE_TIP)
    spl_autoload_register(function (string $class): void {
        $f = dirname(__DIR__).'/.oracle/phpstan-src/src/'.str_replace('\\', '/', substr($class, 8)).'.php';
        if (str_starts_with($class, 'PHPStan\\') && is_file($f)) {
            require_once $f;
        }
    });
    $before = get_declared_classes();
    require_once $testFile;
    $class = null;
    foreach (array_diff(get_declared_classes(), $before) as $c) {
        if (str_ends_with($testFile, str_replace('\\', '/', substr($c, strrpos($c, '\\') + 1)).'.php')) {
            $class = $c;
        }
    }
    $rc = new ReflectionClass($class);
    $lines = file($rc->getFileName());
    $cases = [];
    foreach ($rc->getMethods(ReflectionMethod::IS_PUBLIC) as $m) {
        if (! str_starts_with($m->getName(), 'test') || $m->getDeclaringClass()->getName() !== $class) {
            continue;
        }
        if (! phpOk($m)) {
            continue;
        }
        $body = implode('', array_slice($lines, $m->getStartLine() - 1, $m->getEndLine() - $m->getStartLine() + 1));
        preg_match_all("~require(?:_once)?\s*\(?\s*__DIR__\s*\.\s*'([^']+)'~", $body, $req);
        $scan = array_map(fn ($r) => dirname($testFile).$r, $req[1]);
        $argsList = [[]];
        foreach ($m->getAttributes() as $a) {
            if (str_ends_with($a->getName(), 'DataProvider')) {
                $provider = $a->getArguments()[0];
                try {
                    $argsList = [];
                    foreach ($class::$provider() as $args) {
                        $argsList[] = array_values($args);
                    }
                } catch (Throwable) {
                    $argsList = [];
                }
            }
        }
        foreach ($argsList as $i => $args) {
            $t = $rc->newInstanceWithoutConstructor();
            try {
                $m->invokeArgs($t, $args);
            } catch (Throwable) {
            }
            foreach ($t->calls as $j => $files) {
                $cases[] = ['method' => $m->getName().(count($argsList) > 1 ? "#$i" : '').(count($t->calls) > 1 ? "/$j" : ''), 'files' => $files, 'scan' => $scan];
            }
        }
    }
    echo json_encode($cases, JSON_UNESCAPED_SLASHES);
}

function phpOk(ReflectionMethod $m): bool
{
    foreach ($m->getAttributes() as $a) {
        if (str_ends_with($a->getName(), 'RequiresPhp')) {
            [$op, $v] = preg_match('~^\s*(<=|>=|<|>|==|=)?\s*(\S+)~', $a->getArguments()[0], $mm) ? [$mm[1] ?: '>=', $mm[2]] : ['>=', '0'];
            if (! version_compare(PHP_VERSION, $v, $op)) {
                return false;
            }
        }
    }

    return true;
}
