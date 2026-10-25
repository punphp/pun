<?php
// Regenerate only with pinned PHPStan 2.3.1 (phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea).
// From the repository cwd: php <this-file> <absolute-path-to-phpstan.phar>
// Each case becomes an absolute-path config: analysed => paths, scanned => scanFiles, PHP 8.5.
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
foreach (json_decode(file_get_contents("$root/cases.json"), true, flags: JSON_THROW_ON_ERROR) as $name => $case) {
    $config = "parameters:\n    level: {$case['level']}\n    phpVersion: 80500\n";
    foreach (['analysed' => 'paths', 'scanned' => 'scanFiles'] as $key => $parameter) {
        if ($case[$key] === []) {
            continue;
        }
        $config .= "    $parameter:\n";
        foreach ($case[$key] as $file) {
            $config .= '        - ' . json_encode("$root/$file", JSON_UNESCAPED_SLASHES) . "\n";
        }
    }
    if ($case['explainIgnores'] ?? false) {
        $config .= "    reportIgnoresWithoutComments: true\n";
    }
    $reserved = tempnam(sys_get_temp_dir(), 'pun-renamed-');
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
            throw new RuntimeException("$name: $out $err");
        }
        $result = json_decode(substr($out, strpos($out, '{')), true, flags: JSON_THROW_ON_ERROR);
        if ($result['errors'] !== []) {
            throw new RuntimeException(json_encode($result['errors']));
        }
        $expected[$name] = [];
        foreach ($result['files'] as $file => $data) {
            foreach ($data['messages'] as $message) {
                $identifier = $message['identifier'] ?? '';
                if ($identifier !== 'argument.parameterRenamedInSubtype' && !str_starts_with($identifier, 'ignore.')) {
                    continue;
                }
                $expected[$name][] = ['file' => str_replace($root, '$ROOT', $file)] + $message;
            }
        }
        echo "$name: ", count($expected[$name]), " diagnostics\n";
    } finally {
        unlink($tmp);
    }
}
file_put_contents("$root/expected.json", json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
