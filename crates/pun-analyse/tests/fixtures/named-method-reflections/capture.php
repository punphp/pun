<?php

// php capture.php /absolute/path/to/phpstan.phar [output.json]
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);

$phar = realpath($argv[1]);
foreach (['AMP_CURRENT_THREAD_ID', 'AUGMENT_AGENT', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT',
    'GEMINI_CLI', 'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE',
    'CLAUDE_CODE', 'REPL_ID'] as $name) {
    putenv($name);
}
putenv('XDEBUG_MODE=off');
$expected = [];
foreach (['intersection', 'union', 'structural'] as $file) {
    $modes = $file === 'structural' ? ['4', '5', '8', '5-TT', '5-TF', '5-FT', '5-FF'] : ['4', '5'];
    foreach ($modes as $settings) {
        [$level, $switches] = array_pad(explode('-', $settings), 2, null);
        $mode = "$file-$settings";
        $reserved = tempnam(sys_get_temp_dir(), 'pun-named-reflections-');
        $config = "$reserved.neon";
        unlink($reserved);
        $options = $switches === null ? '' : '    checkFunctionArgumentTypes: ' . ($switches[0] === 'T' ? 'true' : 'false')
            . "\n    checkArgumentsPassedByReference: " . ($switches[1] === 'T' ? 'true' : 'false') . "\n";
        file_put_contents($config, "parameters:\n    level: $level\n    phpVersion: 80500\n$options    paths:\n        - "
            . json_encode(__DIR__ . "/$file.php", JSON_UNESCAPED_SLASHES) . "\n");
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
                    $expected[$mode][] = ['file' => str_replace(__DIR__, '$ROOT', $path)] + $message;
                }
            }
            echo "$mode: ", count($expected[$mode]), " complete diagnostics\n";
        } finally {
            unlink($config);
        }
    }
}
file_put_contents($argv[2] ?? __DIR__ . '/expected.json', json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
