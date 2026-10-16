<?php

declare(strict_types=1);

/*
 * Regenerates the expected results of crates/pun-phpstan's config tests with the real PHPStan (scripts/phpstan-config.php):
 *
 *   tests/fixtures/config/<case>/case.json    -> expected.json   (edge-case configs; phar = .oracle/phpstan's)
 *   tests/fixtures/config-corpus/<name>.json  -> "expected" key   (corpus/<project> through a symlink farm whose
 *                                                                  vendor/ is .oracle/phpstan/vendor)
 *
 *   php scripts/export-phpstan-config-fixtures.php [case-or-corpus-name...]
 *
 * case.json: {"args": [phpstan-config.php args after <cwd>], "env": {"NAME": "value"}}.
 * Paths are written with placeholders: {cwd} (the case/farm directory), {root} (the oracle phar's directory), {tmp}
 * (TMPDIR, target-lintcfg/tmp/sys); the environment is only PATH, TMPDIR and the case's env.
 */

$root = dirname(__DIR__);
$fixtures = $root.'/crates/pun-phpstan/tests/fixtures';
$phar = $root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar';
$only = array_slice($argv, 1);

$tmp = $root.'/target-lintcfg/tmp/sys';
@mkdir($tmp, 0777, true);

function oracle(string $phar, string $cwd, array $args, array $env): array
{
    global $tmp;
    $cmd = array_merge(['env', '-i', 'PATH='.getenv('PATH'), 'TMPDIR='.$tmp], array_map(fn ($k, $v) => "$k=$v", array_keys($env), $env),
        ['php', __DIR__.'/phpstan-config.php', $phar, $cwd], $args);
    $out = shell_exec(implode(' ', array_map('escapeshellarg', $cmd)));

    return json_decode(substr($out, strpos($out, "{\n")), true, flags: JSON_THROW_ON_ERROR); // extensions may print warnings first
}

function placeholders(mixed $v, array $map): mixed
{
    if (is_array($v)) {
        return array_map(fn ($x) => placeholders($x, $map), $v);
    }

    return is_string($v) ? strtr($v, $map) : $v;
}

function write(string $file, mixed $data): void
{
    file_put_contents($file, json_encode($data, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_PRESERVE_ZERO_FRACTION)."\n");
}

foreach (glob($fixtures.'/config/*/case.json') as $caseFile) {
    $dir = dirname($caseFile);
    if ($only !== [] && !in_array(basename($dir), $only, true)) {
        continue;
    }
    $case = json_decode(file_get_contents($caseFile), true);
    $before = scandir($dir);
    $res = oracle($phar, $dir, $case['args'] ?? [], $case['env'] ?? []);
    foreach (array_diff(scandir($dir), $before, ['expected.json']) as $created) { // e.g. a relative tmpDir
        system('rm -rf '.escapeshellarg($dir.'/'.$created));
    }
    write($dir.'/expected.json', placeholders($res, [$dir => '{cwd}', dirname($phar) => '{root}', $tmp => '{tmp}']));
    echo basename($dir), ': ', isset($res['error']) ? 'error' : 'ok', "\n";
}

foreach (glob($fixtures.'/config-corpus/*.json') as $caseFile) {
    $name = basename($caseFile, '.json');
    if ($only !== [] && !in_array($name, $only, true)) {
        continue;
    }
    $case = json_decode(file_get_contents($caseFile), true);
    $farm = $root.'/target-lintcfg/tmp/farm/'.$name;
    system('rm -rf '.escapeshellarg($farm));
    mkdir($farm, 0777, true);
    foreach (scandir($root.'/corpus/'.$case['project']) as $entry) {
        if ($entry !== '.' && $entry !== '..' && $entry !== 'vendor') {
            symlink($root.'/corpus/'.$case['project'].'/'.$entry, $farm.'/'.$entry);
        }
    }
    symlink($root.'/.oracle/phpstan/vendor', $farm.'/vendor');
    $res = oracle($farm.'/vendor/phpstan/phpstan/phpstan.phar', $farm, $case['args'] ?? [], $case['env'] ?? []);
    $case['expected'] = placeholders($res, [$farm => '{cwd}', $tmp => '{tmp}']);
    write($caseFile, $case);
    echo $name, ': ', isset($res['error']) ? 'error: '.substr($res['error'], 0, 200) : 'ok', "\n";
}
