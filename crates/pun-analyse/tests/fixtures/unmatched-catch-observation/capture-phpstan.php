<?php

// php capture-phpstan.php /absolute/path/to/phpstan.phar /absolute/path/to/output.json
declare(strict_types=1);

$phar = realpath($argv[1]);
require 'phar://'.$phar.'/vendor/autoload.php';
$root = sys_get_temp_dir().'/pun-unmatched-phpstan-'.getmypid();
mkdir($root);
$file = __DIR__.'/input.php';
try {
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n    treatPhpDocTypesAsCertain: true\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://'.$phar.'/conf/config.level0.neon', "$root/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $records = [];
    $analysis = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$records, $source): void {
            if (!$node instanceof PhpParser\Node\Expr
                || !preg_match('/^; \/\/ observe:([a-z-]+)/', substr($source, $node->getEndFilePos() + 1), $match)) {
                return;
            }
            $records[$match[1]] = [
                'start' => $node->getStartFilePos(), 'end' => $node->getEndFilePos() + 1,
                'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
                'full' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
            ];
        },
    );
    if ($analysis->getErrors() !== []) {
        throw new RuntimeException('Observation capture failed');
    }
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    if ($result->getInternalErrors() !== []) {
        throw new RuntimeException('PHPStan internal error');
    }
    $errors = array_map(static fn($error) => [
        $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
    ], $result->getUnorderedErrors());
    sort($errors);
    ksort($records);
    file_put_contents($argv[2], json_encode([
        'phar_sha256' => hash_file('sha256', $phar), 'input_sha256' => hash_file('sha256', $file),
        'records' => $records, 'errors' => $errors,
    ], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR)."\n");
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
