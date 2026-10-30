<?php

// php capture.php /absolute/path/to/phpstan.phar [output.json]
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);

$phar = realpath($argv[1]);
$root = __DIR__;
foreach (['AMP_CURRENT_THREAD_ID', 'AUGMENT_AGENT', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT',
    'GEMINI_CLI', 'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE',
    'CLAUDE_CODE', 'REPL_ID'] as $name) {
    putenv($name);
}
putenv('XDEBUG_MODE=off');
$reserved = tempnam(sys_get_temp_dir(), 'pun-shape-discrimination-');
$config = "$reserved.neon";
unlink($reserved);
file_put_contents($config, "parameters:\n    level: 5\n    phpVersion: 80500\n    paths:\n        - "
    . json_encode("$root/matrix.php", JSON_UNESCAPED_SLASHES) . "\n");
try {
    $process = proc_open(['php', '-d', 'phpstan.restarted=1', $phar, 'analyse', '--debug',
        '-c', $config, '--error-format=json', '--no-progress'], [1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
    $out = stream_get_contents($pipes[1]);
    $err = stream_get_contents($pipes[2]);
    fclose($pipes[1]);
    fclose($pipes[2]);
    $status = proc_close($process);
    if ($status > 1 || !str_contains($out, '{')) {
        throw new RuntimeException("$out $err");
    }
    $result = json_decode(substr($out, strpos($out, '{')), true, flags: JSON_THROW_ON_ERROR);
    if ($result['errors'] !== []) {
        throw new RuntimeException(json_encode($result['errors']));
    }
    $expected = [];
    foreach ($result['files'] as $file => $data) {
        foreach ($data['messages'] as $message) {
            $expected[] = ['file' => str_replace($root, '$ROOT', $file)] + $message;
        }
    }
    file_put_contents($argv[2] ?? "$root/expected.json", json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
    echo count($expected), " complete diagnostics\n";
} finally {
    unlink($config);
}
