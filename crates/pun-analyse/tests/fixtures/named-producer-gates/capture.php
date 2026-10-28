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
$groups = [
    '' => ['contracts.php', 'ignored.php'],
    'boundaries-' => ['boundaries.php'],
    'constructor-' => ['constructor-only.php'],
];
foreach ($groups as $prefix => $files) {
    foreach (['all', 'no-calls', 'no-overrides', 'no-constructors', 'no-join'] as $mode) {
        $case = $prefix . $mode;
        $enabled = array_filter($rules, fn ($class, $name) => $mode !== "no-$name", ARRAY_FILTER_USE_BOTH);
        $config = "parameters:\n    level: 5\n    customRulesetUsed: true\n    phpVersion: 80500\n"
            . "    reportUnmatchedIgnoredErrors: true\n";
        if ($prefix === 'boundaries-') {
            $config .= "    featureToggles:\n        checkNonStringableDynamicAccess: true\n";
        }
        $config .= "    paths:\n";
        foreach ($files as $file) {
            $config .= '        - ' . json_encode("$root/$file", JSON_UNESCAPED_SLASHES) . "\n";
        }
        $config .= "autowiredAttributeServices:\n    level: null\nservices:\n";
        foreach ($enabled as $name => $class) {
            $config .= "    -\n        class: $class\n        tags:\n            - phpstan.rules.rule\n";
            if ($name === 'overrides') {
                $config .= "        arguments:\n            checkPhpDocMethodSignatures: %checkPhpDocMethodSignatures%\n"
                    . "            checkMissingOverrideMethodAttribute: %checkMissingOverrideMethodAttribute%\n";
            }
        }
        $reserved = tempnam(sys_get_temp_dir(), 'pun-named-gates-');
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
        echo "$case: ", count($expected[$case]), " complete diagnostics\n";
    }
}
file_put_contents("$root/expected.json", json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
