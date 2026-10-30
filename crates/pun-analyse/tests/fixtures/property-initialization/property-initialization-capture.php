<?php
// php property-initialization-capture.php /absolute/phpstan.phar input.php > expected.json
// PHPStan 2.3.1 / phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);

foreach (['AMP_CURRENT_THREAD_ID', 'AUGMENT_AGENT', 'AI_AGENT', 'CURSOR_TRACE_ID', 'CURSOR_AGENT',
    'GEMINI_CLI', 'CODEX_SANDBOX', 'CODEX_THREAD_ID', 'OPENCODE_CLIENT', 'OPENCODE', 'CLAUDECODE',
    'CLAUDE_CODE', 'REPL_ID'] as $name) {
    putenv($name);
}
putenv('XDEBUG_MODE=off');
$phar = realpath($argv[1]);
require "phar://$phar/vendor/autoload.php";
$file = realpath($argv[2]);
$tmp = sys_get_temp_dir() . '/pun-property-initialization-' . getmypid();
mkdir($tmp);
try {
    file_put_contents("$tmp/config.neon", "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(dirname($file)))->create(
        "$tmp/cache", ["phar://$phar/conf/config.level4.neon", "$tmp/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $records = [];
    $source = file_get_contents($file);
    $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$records, $source): void {
            if (!$node instanceof PhpParser\Node\Expr\Isset_) return;
            foreach ($node->vars as $fetch) {
                if (!$fetch instanceof PhpParser\Node\Expr\PropertyFetch
                    || !$fetch->name instanceof PhpParser\Node\Identifier) continue;
                $property = $fetch->name->toString();
                $records[] = [
                    'start' => $fetch->getStartFilePos(), 'end' => $fetch->getEndFilePos() + 1,
                    'line' => $fetch->getStartLine(), 'property' => $property,
                    'expression' => substr($source, $fetch->getStartFilePos(), $fetch->getEndFilePos() - $fetch->getStartFilePos() + 1),
                    'class' => $scope->isInClass() ? $scope->getClassReflection()->getName() : null,
                    'method' => $scope->getFunctionName(),
                    'certainty' => $scope->hasExpressionType(new PHPStan\Node\Expr\PropertyInitializationExpr($property))->describe(),
                ];
            }
        },
    );
    if ($result->getErrors() !== []) throw new RuntimeException('PHPStan observation capture failed');
    usort($records, static fn($a, $b) => $a['start'] <=> $b['start']);
    echo json_encode($records, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($tmp);
}
