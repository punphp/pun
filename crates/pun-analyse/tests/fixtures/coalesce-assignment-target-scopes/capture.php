<?php
declare(strict_types=1);

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = realpath($argv[1]);
$root = sys_get_temp_dir().'/pun-coalesce-assignment-target-scopes-'.getmypid();
mkdir($root);
try {
    copy(__DIR__.'/input.php', "$root/input.php");
    file_put_contents("$root/phpstan.neon", "parameters:\n    level: 0\n    phpVersion: 80500\n    tmpDir: $root/cache\n");
    $command = escapeshellarg(PHP_BINARY).' '.escapeshellarg($phar)
        .' analyse --no-progress --error-format=json -c '.escapeshellarg("$root/phpstan.neon")
        .' '.escapeshellarg("$root/input.php");
    exec($command, $output, $status);
    if ($status !== 1) {
        throw new RuntimeException('oracle exit '.$status);
    }
    $result = json_decode(implode("\n", $output), true, flags: JSON_THROW_ON_ERROR);
    if ($result['errors'] !== []) {
        throw new RuntimeException('oracle run errors');
    }
    $records = [];
    foreach ($result['files'] as $file => $data) {
        foreach ($data['messages'] as $e) {
            $records[] = [basename($file), $e['line'], $e['identifier'], $e['message'], $e['tip'] ?? null, $e['ignorable']];
        }
    }
    echo json_encode(['phar_sha256' => hash_file('sha256', $phar), 'records' => $records],
        JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
