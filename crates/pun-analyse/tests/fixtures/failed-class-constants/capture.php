<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea, PHP 8.5.
$phar = realpath($argv[1]);
require "phar://$phar/vendor/autoload.php";

function identity(PHPStan\Type\Type $type): array
{
    $ints = array_map(static fn($value) => $value->getValue(), array_values(array_filter(
        $type->getConstantScalarTypes(), static fn($value) => $value instanceof PHPStan\Type\Constant\ConstantIntegerType,
    )));
    $strings = array_map(static fn($value) => $value->getValue(), array_values(array_filter(
        $type->getConstantScalarTypes(), static fn($value) => $value instanceof PHPStan\Type\Constant\ConstantStringType,
    )));
    $objects = $type->getObjectClassNames();
    sort($ints);
    sort($strings);
    sort($objects);
    return [
        'error' => $type instanceof PHPStan\Type\ErrorType,
        'mixed' => $type instanceof PHPStan\Type\MixedType && !$type instanceof PHPStan\Type\ErrorType,
        'never' => $type instanceof PHPStan\Type\NonAcceptingNeverType ? 'nonaccepting'
            : ($type instanceof PHPStan\Type\NeverType ? ($type->isExplicit() ? 'explicit' : 'implicit') : null),
        'ints' => $ints, 'strings' => $strings, 'objects' => $objects,
    ];
}

$tmp = sys_get_temp_dir() . '/pun-failed-class-constants-' . getmypid();
mkdir($tmp);
try {
    $out = ['phpstan_source' => 'cb3b7e99489b2e3789ee2eec10501cb7a72580ea', 'files' => []];
    foreach (['undefined-class', 'self', 'static', 'parent', 'dynamic-invalid', 'string-class', 'enum', 'controls', 'never'] as $case) {
        $file = __DIR__ . "/$case.php";
        $source = file_get_contents($file);
        $record = ['sha256' => hash('sha256', $source), 'diagnostics' => [], 'expressions' => []];
        foreach ([0, 8, 10] as $level) {
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                "$tmp/$case-$level", ["phar://$phar/conf/config.level$level.neon"], [$file],
                usedLevel: (string)$level, singleReflectionFile: $file, singleReflectionInsteadOfFile: $file,
                additionalParameters: ['phpVersion' => 80500],
            );
            $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
            $expressions = [];
            $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
                $file, [$file => true], $container->getByType(PHPStan\Rules\Registry::class),
                $container->getByType(PHPStan\Collectors\Registry::class),
                static function ($node, $scope) use (&$expressions, $source): void {
                    $marker = $node instanceof PhpParser\Node\Scalar\String_ && str_starts_with($node->value, 'after-');
                    $stop = $node instanceof PhpParser\Node\Expr\FuncCall && $node->name instanceof PhpParser\Node\Name
                        && strtolower($node->name->getLast()) === 'stop';
                    if (!$node instanceof PhpParser\Node\Expr\ClassConstFetch && !$node instanceof PhpParser\Node\Expr\Assign && !$marker && !$stop) {
                        return;
                    }
                    $offset = $node->getStartFilePos();
                    $type = $scope->getType($node);
                    $native = $scope->getNativeType($node);
                    $stored = $scope->getCurrentExpressionResultStorage()->findExpressionResult($node);
                    $entry = [
                        'expression' => substr($source, $offset, $node->getEndFilePos() - $offset + 1),
                        'line' => $node->getStartLine(), 'type' => $type->describe(PHPStan\Type\VerbosityLevel::precise()),
                        'full' => identity($type), 'native' => identity($native),
                        'kind' => $marker ? 'marker' : ($stop ? 'stop' : ($node instanceof PhpParser\Node\Expr\Assign ? 'assignment' : 'constant')),
                        'visits' => ($expressions[$offset]['visits'] ?? 0) + 1,
                    ];
                    if ($node instanceof PhpParser\Node\Expr\Assign && $node->var instanceof PhpParser\Node\Expr\Variable && is_string($node->var->name)) {
                        if ($stored === null) {
                            throw new RuntimeException('Missing stored assignment result.');
                        }
                        $entry['assigned'] = identity($stored->getScope()->getVariableType($node->var->name));
                    }
                    $expressions[$offset] = $entry;
                },
            );
            $errors = array_map(static fn($error) => [
                $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
            ], $result->getErrors());
            sort($errors);
            $record['diagnostics'][$level] = $errors;
            ksort($expressions, SORT_NUMERIC);
            if ($level === 8) {
                $record['expressions'] = $expressions;
            }
        }
        $out['files']["$case.php"] = $record;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($tmp);
}
