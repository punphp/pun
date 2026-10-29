<?php
// php capture.php /absolute/path/to/phpstan.phar [output.json]
// PHPStan 2.3.1; phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);
$phar = realpath($argv[1]);
$root = __DIR__;
foreach (['AMP_CURRENT_THREAD_ID', 'AUGMENT_AGENT', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT',
    'GEMINI_CLI', 'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE',
    'CLAUDE_CODE', 'REPL_ID'] as $name) { putenv($name); }
putenv('XDEBUG_MODE=off');
$expected = [];
$cases = array_map(fn ($level) => ['default', $level], [0, 1, 2, 4, 5, 7]);
foreach (['types-only', 'references-only', 'neither', 'this-only', 'no-calls'] as $mode) { $cases[] = [$mode, 5]; }
$cases[] = ['all-receivers', 0];
foreach ([2, 5, 7] as $level) { $cases[] = ['boundaries-default', $level]; }
$cases[] = ['boundaries-case', 5];
foreach ([2, 5, 9] as $level) { $cases[] = ['guards-default', $level]; }
foreach ([2, 5, 9] as $level) { $cases[] = ['nullsafe-default', $level]; }
foreach ($cases as [$mode, $level]) {
    $key = "$mode-$level";
    $file = 'calls';
    foreach (['boundaries', 'guards', 'nullsafe'] as $prefix) {
        if (str_starts_with($mode, "$prefix-")) {
            $file = $prefix;
            $mode = substr($mode, strlen("$prefix-"));
            break;
        }
    }
    $config = "parameters:\n    level: $level\n    phpVersion: 80500\n    paths:\n        - "
        . json_encode("$root/$file.php", JSON_UNESCAPED_SLASHES) . "\n";
    if (in_array($mode, ['references-only', 'neither'], true)) { $config .= "    checkFunctionArgumentTypes: false\n"; }
    if (in_array($mode, ['types-only', 'neither'], true)) { $config .= "    checkArgumentsPassedByReference: false\n"; }
    if ($mode === 'this-only') { $config .= "    checkThisOnly: true\n"; }
    if ($mode === 'all-receivers') { $config .= "    checkThisOnly: false\n"; }
    if ($mode === 'case') { $config .= "    checkFunctionNameCase: true\n"; }
    if ($mode === 'no-calls') {
        $config .= "    customRulesetUsed: true\nautowiredAttributeServices:\n    level: null\n";
    }
    $reserved = tempnam(sys_get_temp_dir(), 'pun-dynamic-methods-');
    $path = "$reserved.neon";
    unlink($reserved);
    file_put_contents($path, $config);
    try {
        $process = proc_open(['php', '-d', 'phpstan.restarted=1', $phar, 'analyse', '--debug', '-c', $path,
            '--error-format=json', '--no-progress'], [1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes);
        $out = stream_get_contents($pipes[1]);
        $err = stream_get_contents($pipes[2]);
        fclose($pipes[1]); fclose($pipes[2]);
        $status = proc_close($process);
        if ($status > 1 || !str_contains($out, '{')) { throw new RuntimeException("$key: $out $err"); }
        $result = json_decode(substr($out, strpos($out, '{')), true, flags: JSON_THROW_ON_ERROR);
        if ($result['errors'] !== []) { throw new RuntimeException(json_encode($result['errors'])); }
        $expected[$key] = [];
        foreach ($result['files'] as $file => $data) {
            foreach ($data['messages'] as $message) {
                $expected[$key][] = ['file' => str_replace($root, '$ROOT', $file)] + $message;
            }
        }
        echo "$key: ", count($expected[$key]), " complete diagnostics\n";
    } finally { unlink($path); }
}
file_put_contents($argv[2] ?? "$root/expected.json", json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
