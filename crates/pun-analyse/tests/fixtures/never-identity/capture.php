<?php

// php capture.php /absolute/path/to/phpstan/bootstrap.php > expected-scopes.json
require $argv[1];

use PhpParser\Node;
use PHPStan\Analyser\FileAnalyser;
use PHPStan\Analyser\NodeScopeResolver;
use PHPStan\Analyser\Scope;
use PHPStan\Collectors\Registry as CollectorRegistry;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Reflection\ReflectionProvider;
use PHPStan\Rules\DirectRegistry;
use PHPStan\Type\NeverType;
use PHPStan\Type\NonAcceptingNeverType;
use PHPStan\Type\VerbosityLevel;

$tmp = sys_get_temp_dir().'/pun-never-identity-'.getmypid();
mkdir($tmp);
try {
    $config = $tmp.'/config.neon';
    file_put_contents($config, "parameters:\n    phpVersion: 80500\n");
    $file = __DIR__.'/types.php';
    $container = (new ContainerFactory(__DIR__))->create($tmp.'/cache', [$config], [$file]);
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $nodes = $container->getService('defaultAnalysisParser')->parseFile($file);
    $provider = $container->getByType(ReflectionProvider::class);
    $record = static function ($type) use (&$record): array {
        $value = [
            'type' => $type->describe(VerbosityLevel::precise()),
            'kind' => $type instanceof NonAcceptingNeverType ? 'nonaccepting'
                : ($type instanceof NeverType ? ($type->isExplicit() ? 'explicit' : 'implicit') : null),
        ];
        if ($type instanceof PHPStan\Type\ClosureType) {
            $value['return'] = $record($type->getReturnType());
        }
        if ($type instanceof PHPStan\Type\ConditionalTypeForParameter || $type instanceof PHPStan\Type\ConditionalType) {
            $value['if'] = $record($type->getIf());
            $value['else'] = $record($type->getElse());
        }
        return $value;
    };
    $out = [];
    foreach ($nodes as $node) {
        if ($node instanceof Node\Stmt\Function_) {
            $variant = $provider->getFunction(new Node\Name($node->name->toString()), null)->getVariants()[0];
            $out['declarations'][$node->name->toString()] = $record($variant->getReturnType());
        }
    }
    $source = file_get_contents($file);
    $result = $container->getByType(FileAnalyser::class)->analyseFile($file, [$file => true], new DirectRegistry([]), new CollectorRegistry([]), static function (Node $node, Scope $scope) use (&$out, $source, $record): void {
        if (!$node instanceof Node\Expr\FuncCall || !$node->name instanceof Node\Name || !str_starts_with($node->name->toString(), 'choose')) {
            return;
        }
        $out['calls'][(string) $node->getStartFilePos()] = [
            'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
            ...$record($scope->getType($node)),
        ];
    });
    if ($result->getErrors() !== [] || count($out['calls'] ?? []) !== 8) {
        throw new RuntimeException('Missing or invalid oracle observations');
    }
    ksort($out['calls'], SORT_NUMERIC);
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($tmp);
}
