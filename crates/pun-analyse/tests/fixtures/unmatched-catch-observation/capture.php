<?php

// php capture.php /absolute/path/to/rector-bin /absolute/path/to/output.json
declare(strict_types=1);

$oracle = realpath($argv[1]);
$output = $argv[2];
$root = sys_get_temp_dir().'/pun-unmatched-catch-'.getmypid();
mkdir($root);
try {
    copy(__DIR__.'/input.php', "$root/input.php");
    $capture = __DIR__.'/CaptureRector.php';
    file_put_contents("$root/phpstan.neon", "parameters:\n    treatPhpDocTypesAsCertain: true\n");
    file_put_contents("$root/rector.php", '<?php require '.var_export($capture, true).'; return Rector\\Config\\RectorConfig::configure()->withPaths(['
        .var_export("$root/input.php", true).'])->withPhpVersion(80500)->withPHPStanConfigs(['
        .var_export("$root/phpstan.neon", true).'])->withRules([UnmatchedCatchCaptureRector::class]);');
    $process = proc_open(['php', "$oracle/vendor/bin/rector", 'process', '-c', "$root/rector.php", '--clear-cache', '--no-progress-bar', '--no-ansi'],
        [1 => ['pipe', 'w'], 2 => ['pipe', 'w']], $pipes, $oracle,
        array_merge(getenv(), ['UNMATCHED_CAPTURE_INPUT' => "$root/input.php", 'UNMATCHED_CAPTURE_OUTPUT' => "$root/records.json"]));
    $stdout = stream_get_contents($pipes[1]);
    $stderr = stream_get_contents($pipes[2]);
    fclose($pipes[1]);
    fclose($pipes[2]);
    $status = proc_close($process);
    if ($status !== 0 || !is_file("$root/records.json")) {
        throw new RuntimeException("Rector exit $status: $stdout $stderr");
    }
    if (file_get_contents("$root/input.php") !== file_get_contents(__DIR__.'/input.php')) {
        throw new RuntimeException('Observation rule modified the input');
    }
    $lock = json_decode(file_get_contents("$oracle/composer.lock"), true, flags: JSON_THROW_ON_ERROR);
    $versions = array_column($lock['packages'], 'version', 'name');
    $result = [
        'rector' => $versions['rector/rector'],
        'phpstan' => $versions['phpstan/phpstan'],
        'treatPhpDocTypesAsCertain' => true,
        'phar_sha256' => hash_file('sha256', "$oracle/vendor/phpstan/phpstan/phpstan.phar"),
        'input_sha256' => hash_file('sha256', __DIR__.'/input.php'),
        'records' => json_decode(file_get_contents("$root/records.json"), true, flags: JSON_THROW_ON_ERROR),
        'stdout' => $stdout,
        'stderr' => $stderr,
    ];
    file_put_contents($output, json_encode($result, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR)."\n");
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
