<?php

// php oracle.php /absolute/path/to/phpstan/bootstrap.php [source.php ...]
declare(strict_types=1);

require $argv[1];

use PhpParser\Node;
use PhpParser\Node\Expr;
use PHPStan\Analyser\FileAnalyser;
use PHPStan\Analyser\NodeScopeResolver;
use PHPStan\Analyser\Scope;
use PHPStan\Collectors\Registry as CollectorRegistry;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Rules\DirectRegistry as RuleRegistry;
use PHPStan\Type\MixedType;
use PHPStan\Type\Type;
use PHPStan\Type\VerbosityLevel;

$files = array_map('realpath', array_slice($argv, 2));
if ($files === []) {
    $files = [__DIR__.'/declarations.php'];
}
$tmp = dirname(__DIR__, 5).'/target/oracle-explicit-mixed/'.md5(realpath($argv[1]));
$container = (new ContainerFactory(__DIR__))->create($tmp, [__DIR__.'/phpstan.neon'], $files);
$container->getService('pathRoutingParser')->setAnalysedFiles($files);
$resolver = $container->getByType(NodeScopeResolver::class);
$resolver->setAnalysedFiles($files);
$parser = $container->getService('defaultAnalysisParser');
$record = static fn (Type $type): array => [
    'type' => $type->describe(VerbosityLevel::precise()),
    'explicit' => $type instanceof MixedType ? $type->isExplicitMixed() : null,
];
$out = [];
foreach ($files as $file) {
    $source = file_get_contents($file);
    $nodes = $parser->parseFile($file);
    $wanted = [];
    $find = static function (Node $node) use (&$wanted): void {
        if ($node instanceof Expr\ArrowFunction) {
            $wanted[$node->expr->getStartFilePos()] = $node->expr->getEndFilePos();
        }
    };
    $visit = static function (array $nodes) use (&$visit, $find): void {
        foreach ($nodes as $node) {
            if (!$node instanceof Node) {
                continue;
            }
            $find($node);
            foreach ($node->getSubNodeNames() as $name) {
                $child = $node->$name;
                $visit(is_array($child) ? $child : [$child]);
            }
        }
    };
    $visit($nodes);
    $offset = 0;
    foreach (explode("\n", $source) as $line) {
        if (str_contains($line, '// mixed:') && !str_contains($line, '=>')) {
            $expression = rtrim(explode('// mixed:', $line)[0]);
            $expression = rtrim($expression, ';');
            $start = $offset + strlen($expression) - strlen(ltrim($expression));
            $wanted[$start] = $offset + strlen($expression) - 1;
        }
        $offset += strlen($line) + 1;
    }
    $result = $container->getByType(FileAnalyser::class)->analyseFile($file, array_fill_keys($files, true), new RuleRegistry([]), new CollectorRegistry([]), static function (Node $node, Scope $scope) use (&$out, $file, $source, $wanted, $record): void {
        if (!$node instanceof Expr || ($wanted[$node->getStartFilePos()] ?? null) !== $node->getEndFilePos()) {
            return;
        }
        $out[basename($file)][(string) $node->getStartFilePos()] = [
            'line' => $node->getStartLine(),
            'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
            'full' => $record($scope->getType($node)),
            'native' => $record($scope->getNativeType($node)),
        ];
    });
    foreach ($result->getErrors() as $error) {
        throw new RuntimeException($error->getMessage());
    }
    if (count($out[basename($file)] ?? []) !== count($wanted)) {
        throw new RuntimeException('Missing observations for '.$file);
    }
    ksort($out[basename($file)], SORT_NUMERIC);
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES), "\n";
