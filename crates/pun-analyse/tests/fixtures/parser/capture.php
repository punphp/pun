<?php

// php capture.php /absolute/path/to/phpstan.phar > expected.json
// Public Analyser, not the CLI: AnalyseCommand hides valid-file diagnostics when another file cannot parse.
// Inputs are analysed together at level 0; no rule/identifier filter changes this capture.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-parser-oracle-' . getmypid();
mkdir($root);
try {
    $files = [];
    foreach (json_decode(file_get_contents(__DIR__ . '/inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $files[] = $file = "$root/$name.php";
        file_put_contents($file, "<?php\n$code\n");
    }
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache",
        ['phar://' . $phar . '/conf/config.level0.neon'],
        $files,
    );
    // CommandHelper does this separately from Analyser::setAnalysedFiles, before reflection can clean bodies.
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
    $errors = [];
    foreach ($result->getErrors() as $error) {
        $errors[] = [
            'file' => basename($error->getFile()),
            'line' => $error->getLine(),
            'identifier' => $error->getIdentifier(),
            'message' => $error->getMessage(),
            'tip' => $error->getTip(),
            'ignorable' => $error->canBeIgnored(),
        ];
    }
    usort($errors, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
    echo json_encode($errors, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
