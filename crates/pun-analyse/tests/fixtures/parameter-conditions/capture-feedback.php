<?php
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture-feedback.php /absolute/path/to/phpstan.phar > feedback-types.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-capture-feedback-' . getmypid();
mkdir($temporary);
try {
    $file = __DIR__ . '/feedback.php';
    $source = file_get_contents($file);
    file_put_contents("$temporary/config.neon", "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$temporary/cache", ['phar://' . $phar . '/conf/config.level0.neon', "$temporary/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $types = [];
    $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$types, $source): void {
            if (!$node instanceof PhpParser\Node\Expr\Variable) return;
            $tail = substr($source, $node->getEndFilePos() + 1);
            if (!preg_match('~^(?:; //| /\*) observe ([a-z-]+)~', $tail, $match)) return;
            $types[$match[1]] = [
                'start' => $node->getStartFilePos(),
                'end' => $node->getEndFilePos() + 1,
                'full' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
            ];
        },
    );
    if ($result->getErrors() !== []) throw new RuntimeException('Oracle internal error');
    ksort($types);
    echo json_encode($types, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($temporary);
}
