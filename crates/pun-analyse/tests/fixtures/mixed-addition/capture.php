<?php

// PHPStan 2.3.1 (phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea).
// php capture.php /absolute/path/to/phpstan.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-mixed-addition-' . getmypid();
mkdir($temporary);
try {
    $file = __DIR__ . '/frontend.php';
    file_put_contents("$temporary/config.neon", "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$temporary/types", ["phar://$phar/conf/config.level0.neon", "$temporary/config.neon"], [$file],
    );
    $container->getByType(PHPStan\Reflection\ReflectionProvider::class);
    $resolver = $container->getByType(PHPStan\Reflection\InitializerExprTypeResolver::class);
    $mixed = new PHPStan\Type\MixedType();
    $array = new PHPStan\Type\ArrayType($mixed, $mixed);
    $types = [
        'mixed' => $mixed,
        'explicit' => new PHPStan\Type\MixedType(true),
        'error' => new PHPStan\Type\ErrorType(),
        'not-array' => $mixed->subtract($array),
        'not-iterable' => $mixed->subtract(new PHPStan\Type\IterableType($mixed, $mixed)),
        'not-array-null' => $mixed->subtract(PHPStan\Type\TypeCombinator::union($array, new PHPStan\Type\NullType())),
        'not-empty-array' => $mixed->subtract(new PHPStan\Type\Constant\ConstantArrayType([], [])),
        'not-int-array' => $mixed->subtract(new PHPStan\Type\ArrayType(new PHPStan\Type\IntegerType(), $mixed)),
        'not-nonempty-array' => $mixed->subtract(PHPStan\Type\TypeCombinator::intersect($array, new PHPStan\Type\Accessory\NonEmptyArrayType())),
        'not-string' => $mixed->subtract(new PHPStan\Type\StringType()),
        'int' => new PHPStan\Type\IntegerType(),
        'float' => new PHPStan\Type\FloatType(),
        'empty-array' => new PHPStan\Type\Constant\ConstantArrayType([], []),
        'never' => new PHPStan\Type\NeverType(),
    ];
    $pairs = [
        ['mixed', 'mixed'], ['explicit', 'explicit'], ['explicit', 'mixed'], ['mixed', 'explicit'],
        ['error', 'error'], ['error', 'mixed'], ['mixed', 'error'],
        ['not-array', 'not-array'], ['not-iterable', 'not-array'], ['not-array', 'not-iterable'],
        ['not-array-null', 'not-array'], ['not-array', 'not-array-null'],
        ['not-array', 'mixed'], ['mixed', 'not-array'], ['not-array', 'error'], ['error', 'not-array'],
        ['not-array', 'not-empty-array'], ['not-empty-array', 'not-array'],
        ['not-array', 'not-int-array'], ['not-int-array', 'not-array'],
        ['not-array', 'not-nonempty-array'], ['not-nonempty-array', 'not-array'],
        ['not-string', 'not-string'], ['int', 'int'], ['float', 'int'],
        ['empty-array', 'empty-array'], ['never', 'mixed'], ['mixed', 'never'],
    ];
    $left = new PhpParser\Node\Expr\Variable('left');
    $right = new PhpParser\Node\Expr\Variable('right');
    $out = ['plus' => [], 'diagnostics' => []];
    foreach ($pairs as [$a, $b]) {
        $result = $resolver->getPlusType($left, $right, static fn($node) => $node === $left ? $types[$a] : $types[$b]);
        $out['plus'][] = [
            'left' => $a, 'right' => $b,
            'type' => $result->describe(PHPStan\Type\VerbosityLevel::precise()),
            'benevolent' => $result instanceof PHPStan\Type\BenevolentUnionType,
        ];
    }
    foreach ([0, 8, 9, 10] as $level) {
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$temporary/level$level", ["phar://$phar/conf/config.level$level.neon", "$temporary/config.neon"], [$file],
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
            ->finalize($result, onlyFiles: true, debug: true)->getAnalyserResult();
        if ($result->getInternalErrors() !== []) {
            throw new RuntimeException('PHPStan reported an internal error.');
        }
        $records = array_map(static fn($error) => [
            basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
            $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
        ], $result->getUnorderedErrors());
        sort($records);
        $out['diagnostics'][(string) $level] = $records;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($temporary);
}
