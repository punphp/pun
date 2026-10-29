<?php

// PHPStan 2.3.1; phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/to/phpstan.phar
declare(strict_types=1);

$phar = realpath($argv[1]);
$root = __DIR__;
$rules = [
    'calls' => 'PHPStan\\Rules\\Methods\\CallMethodsRule',
    'overrides' => 'PHPStan\\Rules\\Methods\\OverridingMethodRule',
    'constructors' => 'PHPStan\\Rules\\Methods\\ConsistentConstructorRule',
    'join' => 'PHPStan\\Rules\\Methods\\MethodCallWithPossiblyRenamedNamedArgumentRule',
];
foreach (['AMP_CURRENT_THREAD_ID', 'AUGMENT_AGENT', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT',
    'GEMINI_CLI', 'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE',
    'CLAUDE_CODE', 'REPL_ID'] as $name) {
    putenv($name);
}
putenv('XDEBUG_MODE=off');
$expected = [];
// As in named-producer-gates, Pun's identifier-filter control keeps the join registered but drops its
// identifier before local ignores. Its oracle is the independently unregistered PHPStan join, not an
// output filter applied after PHPStan has already consumed the ignores.
foreach (['all', 'no-join'] as $mode) {
    $config = "parameters:\n    level: 5\n    customRulesetUsed: true\n    phpVersion: 80500\n"
        . "    reportUnmatchedIgnoredErrors: true\n    paths:\n        - "
        . json_encode("$root/calls.php", JSON_UNESCAPED_SLASHES) . "\n"
        . "autowiredAttributeServices:\n    level: null\nservices:\n";
    foreach ($rules as $name => $class) {
        if ($mode === "no-$name") {
            continue;
        }
        $config .= "    -\n        class: $class\n        tags:\n            - phpstan.rules.rule\n";
        if ($name === 'overrides') {
            $config .= "        arguments:\n            checkPhpDocMethodSignatures: %checkPhpDocMethodSignatures%\n"
                . "            checkMissingOverrideMethodAttribute: %checkMissingOverrideMethodAttribute%\n";
        }
    }
    $reserved = tempnam(sys_get_temp_dir(), 'pun-named-ignore-');
    $path = "$reserved.neon";
    unlink($reserved);
    file_put_contents($path, $config);
    $process = proc_open(['php', '-d', 'phpstan.restarted=1', $phar, 'analyse', '--debug', '-c', $path,
        '--error-format=json', '--no-progress'], [1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
    $out = stream_get_contents($pipes[1]);
    $err = stream_get_contents($pipes[2]);
    fclose($pipes[1]);
    fclose($pipes[2]);
    $status = proc_close($process);
    unlink($path);
    if ($status > 1 || !str_contains($out, '{')) {
        throw new RuntimeException("$mode: $out $err");
    }
    $result = json_decode(substr($out, strpos($out, '{')), true, flags: JSON_THROW_ON_ERROR);
    if ($result['errors'] !== []) {
        throw new RuntimeException(json_encode($result['errors']));
    }
    $case = "ignore-identity-$mode";
    $expected[$case] = [];
    foreach ($result['files'] as $file => $data) {
        foreach ($data['messages'] as $message) {
            $expected[$case][] = ['file' => str_replace($root, '$ROOT', $file)] + $message;
        }
    }
    echo "$case: ", count($expected[$case]), " complete diagnostics\n";
}
file_put_contents("$root/expected.json", json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
