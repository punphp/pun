<?php

declare(strict_types=1);

/*
 * Runs PHPStan 2.3.5's real BaselineNeonErrorFormatter / BaselinePhpErrorFormatter (from the phar) on the error lists
 * of phpstan-src's own formatter tests (their data providers, called directly) plus extra cases, and writes
 * crates/pun-phpstan/tests/fixtures/baseline.json with the exact output and what PHPStan reads back from it
 * (Neon::decode for .neon, `require` for .php).
 *
 *   php scripts/export-baseline-fixtures.php
 *
 * Paths are made relative with ParentDirectoryRelativePathHelper(dir), what `analyse --generate-baseline` uses.
 */

namespace PHPStan\Testing {
    // The providers are static; their test classes only need to load.
    abstract class PHPStanTestCase {}
    abstract class ErrorFormatterTestCase extends PHPStanTestCase
    {
        protected const DIRECTORY_PATH = '/data/folder/with space/and unicode 😃/project';
    }
}

namespace {
    use PHPStan\Analyser\Error;
    use PHPStan\Command\AnalysisResult;
    use PHPStan\Command\ErrorFormatter\BaselineNeonErrorFormatter;
    use PHPStan\Command\ErrorFormatter\BaselinePhpErrorFormatter;
    use PHPStan\File\ParentDirectoryRelativePathHelper;

    $root = dirname(__DIR__);
    Phar::loadPhar($root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar', 'phpstan.phar');
    require 'phar://phpstan.phar/vendor/autoload.php';
    $tests = $root.'/.oracle/phpstan-src/tests/PHPStan/Command/ErrorFormatter';
    require $tests.'/BaselineNeonErrorFormatterTest.php';
    require $tests.'/BaselinePhpErrorFormatterTest.php';

    final class Buffer implements PHPStan\Command\Output
    {
        public string $out = '';
        public function writeFormatted(string $message): void { $this->out .= $message; }
        public function writeLineFormatted(string $message): void { $this->out .= $message."\n"; }
        public function writeRaw(string $message): void { $this->out .= $message; }
        public function getStyle(): PHPStan\Command\OutputStyle { throw new LogicException(); }
        public function isVerbose(): bool { return false; }
        public function isVeryVerbose(): bool { return false; }
        public function isDebug(): bool { return false; }
        public function isDecorated(): bool { return false; }
    }

    function tagged(mixed $v): mixed
    {
        if (is_array($v)) {
            $items = [];
            foreach ($v as $k => $x) {
                $items[] = [$k, tagged($x)];
            }

            return ['array' => $items];
        }

        return $v;
    }

    function str_replace_deep(string $from, string $to, mixed $v): mixed
    {
        return is_array($v) ? array_map(fn ($x) => str_replace_deep($from, $to, $x), $v) : (is_string($v) ? str_replace($from, $to, $v) : $v);
    }

    const DIR = '/data/folder/with space/and unicode 😃/project';

    // ErrorFormatterTestCase::getAnalysisResult()'s errors.
    function standard(): array
    {
        return [
            new Error('Foo', DIR.'/folder with unicode 😃/file name with "spaces" and unicode 😃.php', 4),
            new Error('Foo<Bar>', DIR.'/foo.php', 1),
            new Error("Bar\nBar2", DIR.'/foo.php', 5, tip: 'a tip'),
            new Error("Bar\nBar2", DIR.'/folder with unicode 😃/file name with "spaces" and unicode 😃.php', 2),
            new Error("Bar\nBar2", DIR.'/foo.php', null),
            new Error('Foobar\\Buz', DIR.'/foo.php', 5, tip: 'a tip', identifier: 'foobar.buz'),
            new Error('Error with @param or @phpstan-param and class@anonymous in the message.', DIR.'/bar.php', 5),
        ];
    }

    $lists = []; // [errors, generic errors]
    foreach (range(0, 7) as $offset) {
        foreach (range(0, 7 - $offset) as $n) {
            $lists[] = [array_slice(standard(), $offset, $n), 0];
        }
    }
    $lists[] = [[], 1];
    $lists[] = [array_slice(standard(), 0, 2), 2];
    $lists[] = [[new Error('Escape Regex with file # ~ \' ()', 'Testfile')], 0];
    foreach (['Test %value%', '@Foo', '%@x', 'tab'."\t".'and \\ back\\\\slash', '😃 unicode', '-1.5', 'yes', 'null', '[a]: {b}'] as $m) {
        $lists[] = [[new Error($m, 'Testfile')], 0];
    }
    $lists[] = [[new Error('123', DIR.'/a.php'), new Error('0', DIR.'/a.php', identifier: '42')], 0];
    $lists[] = [[new Error('In a numeric file', DIR.'/7'), new Error('Up', DIR.'/../other/x.php'), new Error('Abs', '/elsewhere/y.php')], 0];
    $lists[] = [[
        new Error('Not ignorable', DIR.'/a.php', canBeIgnored: false),
        new Error('Mixed', DIR.'/a.php', identifier: 'b.id'),
        new Error('Mixed', DIR.'/a.php'),
        new Error('Mixed', DIR.'/a.php', identifier: 'a.id'),
        new Error('Mixed', DIR.'/a.php', identifier: 'b.id'),
        new Error('Plain', DIR.'/a.php'),
        new Error('Plain', DIR.'/a.php'),
        new Error('B', DIR.'/B.php'),
        new Error('a', DIR.'/a.php'),
    ], 0];
    $lists[] = [[new Error('Not ignorable', DIR.'/a.php', canBeIgnored: false)], 0];
    foreach (PHPStan\Command\ErrorFormatter\BaselineNeonErrorFormatterTest::outputOrderingProvider() as [$errors]) {
        $lists[] = [$errors, 0];
    }
    foreach (PHPStan\Command\ErrorFormatter\BaselineNeonErrorFormatterTest::dataEscapeDiNeon() as [$error]) {
        $lists[] = [[$error], 0];
    }
    foreach (PHPStan\Command\ErrorFormatter\BaselineNeonErrorFormatterTest::dataFormatErrorsWithIdentifiers() as [$errors]) {
        $lists[] = [$errors, 0];
    }
    foreach (PHPStan\Command\ErrorFormatter\BaselinePhpErrorFormatterTest::dataFormatErrors() as [$errors]) {
        $lists[] = [$errors, 0];
    }
    $existing = ['', "\n", "\n\n", "parameters:\n\tignoreErrors: []", "parameters:\n\tignoreErrors: []\n", "parameters:\n\tignoreErrors: []\n\n\n", "x\n\ny"];

    $cases = [];
    $tmp = $root.'/target-lintcfg/tmp/baseline.php';
    foreach ($lists as $i => [$errors, $generic]) {
        $result = new AnalysisResult($errors, array_slice(['first generic error', 'second generic<error>'], 0, $generic), [], [], [], false, null, true, 0, false, []);
        $input = array_map(fn (Error $e) => ['file' => $e->getFilePath(), 'message' => $e->getMessage(), 'identifier' => $e->getIdentifier(), 'ignorable' => $e->canBeIgnored()], $errors);
        foreach ([false, true] as $raw) {
            $helper = new ParentDirectoryRelativePathHelper(DIR);
            foreach ($i % 3 === 0 ? $existing : [$existing[$i % count($existing)]] as $content) {
                $out = new Buffer();
                try {
                    (new BaselineNeonErrorFormatter($helper, $raw))->formatErrors($result, $out, $content);
                } catch (TypeError) { // preg_quote(int) of a numeric message: PHPStan crashes, Pun quotes it
                    continue 2;
                }
                $cases[] = ['format' => 'neon', 'raw' => $raw, 'errors' => $input, 'has_errors' => $result->hasErrors(), 'existing' => $content, 'output' => $out->out, 'read' => tagged(_PHPStan_95d9e8b05\Nette\Neon\Neon::decode($out->out))];
            }
            $out = new Buffer();
            (new BaselinePhpErrorFormatter($helper, $raw))->formatErrors($result, $out);
            file_put_contents($tmp, $out->out);
            $cases[] = ['format' => 'php', 'raw' => $raw, 'errors' => $input, 'has_errors' => $result->hasErrors(), 'output' => $out->out, 'read' => tagged(str_replace_deep(dirname($tmp), '{dir}', require $tmp))];
        }
    }
    unlink($tmp);

    file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/baseline.json', json_encode(['dir' => DIR, 'cases' => $cases], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
    echo count($cases), " cases\n";
}
