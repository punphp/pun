<?php

// php capture.php /absolute/path/to/phpstan.phar [output.json]
// php capture.php /absolute/path/to/phpstan.phar --argument-names [output-directory]
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// calls.php is byte-identical to the original /tmp/pun-dynamic-named-proof probe.
declare(strict_types=1);

$phar = realpath($argv[1]);
$argumentNames = ($argv[2] ?? null) === '--argument-names';
$root = $argumentNames ? dirname(__DIR__) . '/argument-names' : __DIR__;
foreach (['AMP_CURRENT_THREAD_ID', 'AUGMENT_AGENT', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT',
    'GEMINI_CLI', 'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE',
    'CLAUDE_CODE', 'REPL_ID'] as $name) {
    putenv($name);
}
putenv('XDEBUG_MODE=off');
$expected = [];
foreach ($argumentNames ? ['no-named'] : ['calls', 'boundaries', 'nullable-chain'] as $file) {
    $levels = $argumentNames ? [0, 1, 4, 5] : ($file === 'nullable-chain' ? [4, 5, 8] : [4, 5]);
    foreach ($levels as $level) {
        $mode = "$file-$level";
        $reserved = tempnam(sys_get_temp_dir(), 'pun-filtered-named-');
        $config = "$reserved.neon";
        unlink($reserved);
        file_put_contents($config, "parameters:\n    level: $level\n    phpVersion: 80500\n    paths:\n        - "
            . json_encode("$root/$file.php", JSON_UNESCAPED_SLASHES) . "\n");
        try {
            $process = proc_open(['php', '-d', 'phpstan.restarted=1', $phar, 'analyse', '--debug',
                '-c', $config, '--error-format=json', '--no-progress'], [1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
            $out = stream_get_contents($pipes[1]);
            $err = stream_get_contents($pipes[2]);
            fclose($pipes[1]);
            fclose($pipes[2]);
            $status = proc_close($process);
            if ($status > 1 || !str_contains($out, '{')) {
                throw new RuntimeException("$mode: $out $err");
            }
            $result = json_decode(substr($out, strpos($out, '{')), true, flags: JSON_THROW_ON_ERROR);
            if ($result['errors'] !== []) {
                throw new RuntimeException(json_encode($result['errors']));
            }
            $expected[$mode] = [];
            foreach ($result['files'] as $path => $data) {
                foreach ($data['messages'] as $message) {
                    $expected[$mode][] = $argumentNames
                        ? $message
                        : ['file' => str_replace($root, '$ROOT', $path)] + $message;
                }
            }
            echo "$mode: ", count($expected[$mode]), " complete diagnostics\n";
        } finally {
            unlink($config);
        }
    }
}
if ($argumentNames) {
    foreach ([0, 1, 4, 5] as $level) {
        $output = ($argv[3] ?? $root) . "/expected-no-named-level$level.json";
        $json = json_encode($expected["no-named-$level"], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE);
        $json = preg_replace_callback('/^ +/m', static fn(array $match): string => str_repeat(' ', intdiv(strlen($match[0]), 2)), $json);
        file_put_contents($output, $json . "\n");
    }
} else {
    file_put_contents($argv[2] ?? "$root/expected.json", json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
}
