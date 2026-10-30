<?php

// php capture.php /absolute/path/to/phpstan.phar [output.json]
// Pinned PHPStan 2.3.1. Retain every diagnostic field, without identifier filtering.
declare(strict_types=1);

$phar = realpath($argv[1]);
$root = __DIR__;
foreach (['AMP_CURRENT_THREAD_ID', 'AUGMENT_AGENT', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT',
    'GEMINI_CLI', 'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE',
    'CLAUDE_CODE', 'REPL_ID'] as $name) {
    putenv($name);
}
putenv('XDEBUG_MODE=off');
$cases = [];
foreach (['callable-values' => [2, 5], 'guards' => [0, 5], 'anonymous-values' => [0, 5]] as $group => $levels) {
    foreach ($levels as $level) {
        $cases[] = ['id' => "$group-$level", 'group' => $group, 'level' => $level,
            'paths' => ["$group.php"], 'scan' => [], 'names' => false];
    }
}
foreach ([false, true] as $names) {
    $cases[] = ['id' => 'nullsafe-' . (int) $names, 'group' => 'nullsafe', 'level' => 5,
        'paths' => ['nullsafe.php'], 'scan' => [], 'names' => $names];
}
$cases[] = ['id' => 'nullsafe-unregistered', 'group' => 'nullsafe', 'level' => 5,
    'paths' => ['nullsafe.php'], 'scan' => [], 'names' => true, 'calls' => false];
$cases[] = ['id' => 'retained-scanned', 'group' => 'retained', 'level' => 5,
    'paths' => ['host.php'], 'scan' => ['retained.php'], 'names' => false];
$cases[] = ['id' => 'retained-analysed', 'group' => 'retained', 'level' => 5,
    'paths' => ['host.php', 'retained.php'], 'scan' => [], 'names' => false];
foreach ($cases as &$case) {
    $config = "parameters:\n    level: {$case['level']}\n    phpVersion: 80500\n    paths:\n";
    foreach ($case['paths'] as $file) {
        $config .= '        - ' . json_encode("$root/$file", JSON_UNESCAPED_SLASHES) . "\n";
    }
    if ($case['scan'] !== []) {
        $config .= "    scanFiles:\n";
        foreach ($case['scan'] as $file) {
            $config .= '        - ' . json_encode("$root/$file", JSON_UNESCAPED_SLASHES) . "\n";
        }
    }
    $config .= "    featureToggles:\n        checkNonStringableDynamicAccess: "
        . ($case['names'] ? 'true' : 'false') . "\n";
    if (!($case['calls'] ?? true)) {
        $config .= "    customRulesetUsed: true\nautowiredAttributeServices:\n    level: null\n";
    }
    $reserved = tempnam(sys_get_temp_dir(), 'pun-dispatch-contracts-');
    $path = "$reserved.neon";
    unlink($reserved);
    file_put_contents($path, $config);
    try {
        $process = proc_open(['php', '-d', 'phpstan.restarted=1', $phar, 'analyse', '--debug', '-c', $path,
            '--error-format=json', '--no-progress'], [1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
        $stdout = stream_get_contents($pipes[1]);
        $stderr = stream_get_contents($pipes[2]);
        fclose($pipes[1]);
        fclose($pipes[2]);
        $status = proc_close($process);
        if ($status > 1 || !str_contains($stdout, '{')) {
            throw new RuntimeException("{$case['id']}: $stdout $stderr");
        }
        $result = json_decode(substr($stdout, strpos($stdout, '{')), true, flags: JSON_THROW_ON_ERROR);
        $case['general'] = $result['errors'];
        $case['messages'] = [];
        foreach ($result['files'] as $file => $data) {
            foreach ($data['messages'] as $message) {
                $case['messages'][] = ['file' => str_replace($root, '$ROOT', $file)] + $message;
            }
        }
        echo $case['id'], ': ', count($case['messages']), " complete diagnostics\n";
    } finally {
        unlink($path);
    }
}
unset($case);
file_put_contents($argv[2] ?? "$root/expected.json", json_encode($cases, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
