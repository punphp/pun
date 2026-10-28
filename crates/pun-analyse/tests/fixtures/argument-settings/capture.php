<?php
// php capture.php /absolute/path/to/phpstan.phar
// Complete public PHPStan 2.3.1 diagnostics, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);

$phar = realpath($argv[1]);
$root = __DIR__;
foreach (['AMP_CURRENT_THREAD_ID', 'AUGMENT_AGENT', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT',
    'GEMINI_CLI', 'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE',
    'CLAUDE_CODE', 'REPL_ID'] as $name) {
    putenv($name);
}
putenv('XDEBUG_MODE=off');
$expected = [];
foreach ([0, 5] as $level) {
    foreach (['default' => null, 'TT' => [true, true], 'TF' => [true, false],
        'FT' => [false, true], 'FF' => [false, false]] as $flags => $checks) {
        foreach (['default' => null, 'this' => true, 'all' => false] as $receiver => $thisOnly) {
            $case = "$level-$flags-$receiver";
            $config = "parameters:\n    level: $level\n    phpVersion: 80500\n    paths:\n        - "
                . json_encode("$root/matrix.php", JSON_UNESCAPED_SLASHES) . "\n";
            if ($checks !== null) {
                $config .= '    checkFunctionArgumentTypes: ' . ($checks[0] ? 'true' : 'false') . "\n"
                    . '    checkArgumentsPassedByReference: ' . ($checks[1] ? 'true' : 'false') . "\n";
            }
            if ($thisOnly !== null) {
                $config .= '    checkThisOnly: ' . ($thisOnly ? 'true' : 'false') . "\n";
            }
            $reserved = tempnam(sys_get_temp_dir(), 'pun-argument-settings-');
            $tmp = "$reserved.neon";
            unlink($reserved);
            file_put_contents($tmp, $config);
            try {
                $process = proc_open(['php', '-d', 'phpstan.restarted=1', $phar, 'analyse', '--debug',
                    '-c', $tmp, '--error-format=json', '--no-progress'], [1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
                $out = stream_get_contents($pipes[1]);
                $err = stream_get_contents($pipes[2]);
                fclose($pipes[1]);
                fclose($pipes[2]);
                $status = proc_close($process);
                if ($status > 1 || !str_contains($out, '{')) {
                    throw new RuntimeException("$case: $out $err");
                }
                $result = json_decode(substr($out, strpos($out, '{')), true, flags: JSON_THROW_ON_ERROR);
                if ($result['errors'] !== []) {
                    throw new RuntimeException(json_encode($result['errors']));
                }
                $expected[$case] = [];
                foreach ($result['files'] as $file => $data) {
                    foreach ($data['messages'] as $message) {
                        $expected[$case][] = ['file' => str_replace($root, '$ROOT', $file)] + $message;
                    }
                }
                echo "$case: ", count($expected[$case]), " diagnostics\n";
            } finally {
                unlink($tmp);
            }
        }
    }
}
file_put_contents($argv[2] ?? "$root/expected.json", json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
