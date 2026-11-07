<?php

declare(strict_types=1);

/*
 * End-to-end oracle for `@phpstan-ignore*` comments: runs the real PHPStan 2.3.5 (`analyse`, level 0, JSON output) on
 * crates/pun-phpstan/tests/fixtures/local/*.php
 *
 *   - with the comments blanked out (same lines): the raw errors before any local ignore;
 *   - as they are, with reportUnmatchedIgnoredErrors: true and reportIgnoresWithoutComments false / true;
 *
 * and writes tests/fixtures/local.json, keyed by file name.
 *
 *   php scripts/export-local-ignore-fixtures.php
 */

$root = dirname(__DIR__);
$phar = $root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar';
$src = $root.'/crates/pun-phpstan/tests/fixtures/local';
$work = $root.'/target-lintcfg/tmp/local-ignores';
system('rm -rf '.escapeshellarg($work));
mkdir($work.'/blank', 0777, true);
mkdir($work.'/tmp', 0777, true);

function analyse(string $phar, string $work, string $dir, bool $withoutComments): array
{
    $config = $work.'/phpstan.neon';
    file_put_contents($config, "parameters:\n\tlevel: 0\n\treportUnmatchedIgnoredErrors: true\n\treportIgnoresWithoutComments: ".($withoutComments ? 'true' : 'false')."\n\ttmpDir: $work/tmp\n\tparallel:\n\t\tmaximumNumberOfProcesses: 1\n");
    $cmd = ['php', $phar, 'analyse', '-c', $config, '--no-progress', '--error-format=json', '--memory-limit=1G', $dir];
    $out = shell_exec(implode(' ', array_map('escapeshellarg', $cmd)).' 2>/dev/null');
    $result = json_decode($out, true, flags: JSON_THROW_ON_ERROR);
    $files = [];
    foreach ($result['files'] as $file => $data) {
        $files[basename($file)] = $data['messages'];
    }

    return $files;
}

foreach (glob($src.'/*.php') as $file) {
    $blank = '';
    foreach (token_get_all(file_get_contents($file)) as $token) {
        $text = is_array($token) ? $token[1] : $token;
        $blank .= is_array($token) && in_array($token[0], [T_COMMENT, T_DOC_COMMENT], true) ? preg_replace('~[^\n]~', ' ', $text) : $text;
    }
    file_put_contents($work.'/blank/'.basename($file), $blank);
}

$raw = analyse($phar, $work, $work.'/blank', false);
$plain = analyse($phar, $work, $src, false);
$noComments = analyse($phar, $work, $src, true);
$out = [];
foreach (glob($src.'/*.php') as $file) {
    $name = basename($file);
    $out[$name] = ['raw' => $raw[$name] ?? [], 'reported' => $plain[$name] ?? [], 'reportedWithoutComments' => $noComments[$name] ?? []];
}
file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/local.json', json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
echo count($out), " files\n";
