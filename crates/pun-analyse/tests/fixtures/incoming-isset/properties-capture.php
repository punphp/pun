<?php

// php properties-capture.php /absolute/path/to/phpstan.phar [output.json] [fixture.php]
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);

$phar = realpath($argv[1]);
$root = __DIR__;
$file = $argv[3] ?? 'properties.php';
foreach (['AMP_CURRENT_THREAD_ID', 'AUGMENT_AGENT', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT',
    'GEMINI_CLI', 'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE',
    'CLAUDE_CODE', 'REPL_ID'] as $name) {
    putenv($name);
}
putenv('XDEBUG_MODE=off');
$expected = [];
foreach ([1, 4, 5] as $level) {
    $reserved = tempnam(sys_get_temp_dir(), 'pun-isset-properties-');
    $path = "$reserved.neon";
    unlink($reserved);
    file_put_contents($path, "parameters:\n    level: $level\n    phpVersion: 80500\n    paths:\n        - "
        . json_encode("$root/$file", JSON_UNESCAPED_SLASHES) . "\n");
    try {
        $process = proc_open(['php', '-d', 'phpstan.restarted=1', $phar, 'analyse', '--debug',
            '-c', $path, '--error-format=json', '--no-progress'], [1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
        $out = stream_get_contents($pipes[1]);
        $err = stream_get_contents($pipes[2]);
        fclose($pipes[1]);
        fclose($pipes[2]);
        $status = proc_close($process);
        if ($status > 1 || !str_contains($out, '{')) {
            throw new RuntimeException("level $level: $out $err");
        }
        $result = json_decode(substr($out, strpos($out, '{')), true, flags: JSON_THROW_ON_ERROR);
        if ($result['errors'] !== []) {
            throw new RuntimeException(json_encode($result['errors']));
        }
        $expected[$level] = [];
        foreach ($result['files'] as $reportedFile => $data) {
            foreach ($data['messages'] as $message) {
                $expected[$level][] = ['file' => str_replace($root, '$ROOT', $reportedFile)] + $message;
            }
        }
        echo "level $level: ", count($expected[$level]), " complete diagnostics\n";
    } finally {
        unlink($path);
    }
}
file_put_contents($argv[2] ?? "$root/properties-phpstan.json", json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
