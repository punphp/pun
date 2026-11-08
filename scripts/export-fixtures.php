<?php

// Export PHP-CS-Fixer's per-fixer data-provider test cases to JSON.
//
//   php scripts/export-fixtures.php scripts/data/pinkary-effective.json crates/pun-fixers/tests/fixtures
//
// For every fixer in the effective list, reflects its *FixerTest class from the
// pinned php-cs-fixer checkout (.oracle/php-cs-fixer, with dev deps), and for
// every test method carrying #[DataProvider], records the method signature, its
// #[RequiresPhp] constraint, its body source, and every case the provider yields.
// Objects in case data are dumped as {"__class": ..., <props>}; non-UTF-8
// strings as {"__base64": ...}.

[$_, $effective, $outDir] = $argv + [null, null, null];
$root = dirname(__DIR__).'/.oracle/php-cs-fixer';
require $root.'/vendor/autoload.php';

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\RequiresPhp;

function dump_value(mixed $v): mixed
{
    if (is_array($v)) {
        return array_map('dump_value', $v);
    }
    if (is_object($v)) {
        $props = match (true) {
            $v instanceof SplFileInfo => ['pathname' => $v->getPathname()],
            (new ReflectionObject($v))->isInternal() => (array) $v,
            default => (fn () => get_object_vars($this))->call($v),
        };

        return ['__class' => get_class($v)] + array_map('dump_value', $props);
    }

    if (is_string($v) && ! mb_check_encoding($v, 'UTF-8')) {
        return ['__base64' => base64_encode($v)];
    }

    return $v;
}

@mkdir($outDir, 0777, true);
$summary = [];
foreach (json_decode(file_get_contents($effective), true)['fixers'] as $fixer) {
    $testClass = preg_replace('/^PhpCsFixer\\\\Fixer\\\\/', 'PhpCsFixer\\Tests\\Fixer\\', $fixer['class']).'Test';
    if (! str_starts_with($fixer['class'], 'PhpCsFixer\\') || ! class_exists($testClass)) {
        fwrite(STDERR, "skip {$fixer['name']}: no upstream test class\n");
        continue;
    }
    $rc = new ReflectionClass($testClass);
    $tests = [];
    $total = 0;
    foreach ($rc->getMethods() as $m) {
        if (! str_starts_with($m->getDeclaringClass()->getName(), 'PhpCsFixer\\Tests\\Fixer\\')) {
            continue;
        }
        $providers = array_map(fn ($a) => $a->getArguments()[0], $m->getAttributes(DataProvider::class));
        if ($providers === []) {
            continue;
        }
        $requires = array_map(fn ($a) => $a->getArguments()[0], $m->getAttributes(RequiresPhp::class));
        $cases = [];
        foreach ($providers as $provider) {
            foreach ($rc->getMethod($provider)->invoke(null) as $key => $args) {
                $cases[] = ['provider' => $provider, 'key' => $key, 'args' => dump_value($args)];
            }
        }
        $total += count($cases);
        $tests[] = [
            'method' => $m->getName(),
            'params' => array_map(fn ($p) => [
                'name' => $p->getName(),
                'type' => (string) $p->getType(),
                'default' => $p->isDefaultValueAvailable() ? dump_value($p->getDefaultValue()) : null,
            ], $m->getParameters()),
            'requires_php' => $requires[0] ?? null,
            'declared_in' => $m->getDeclaringClass()->getName(),
            'body' => implode('', array_slice(file($m->getFileName()), $m->getStartLine() - 1, $m->getEndLine() - $m->getStartLine() + 1)),
            'cases' => $cases,
        ];
    }
    file_put_contents("$outDir/{$fixer['name']}.json", json_encode([
        'fixer' => $fixer['name'],
        'test_class' => $testClass,
        'test_file' => 'php-cs-fixer/'.substr($rc->getFileName(), strlen(realpath($root)) + 1),
        'case_count' => $total,
        'tests' => $tests,
    ], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
    $summary[$fixer['name']] = $total;
}
echo json_encode($summary, JSON_PRETTY_PRINT)."\n";
