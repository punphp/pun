<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-finite-array-literal-keys-' . getmypid();
mkdir($root);
$files = [__DIR__ . '/matrix.php'];
function scalars(PHPStan\Type\Type $type): array
{
    if ($type instanceof PHPStan\Type\UnionType) {
        $values = array_merge(...array_map(scalars(...), $type->getTypes()));
    } elseif ($type instanceof PHPStan\Type\IntegerRangeType) {
        $values = array_map(static fn($i) => "int:$i", range($type->getMin(), $type->getMax()));
    } elseif ($type instanceof PHPStan\Type\NullType) {
        $values = ['null'];
    } elseif ($type instanceof PHPStan\Type\ConstantScalarType) {
        $value = $type->getValue();
        $values = [match (true) {
            is_string($value) => 'string:' . $value,
            is_int($value) => 'int:' . $value,
            is_bool($value) => 'bool:' . (int) $value,
        }];
    } else {
        $values = [$type->describe(PHPStan\Type\VerbosityLevel::precise())];
    }
    sort($values, SORT_STRING);
    return array_values(array_unique($values));
}
try {
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level5.neon', "$root/config.neon"], $files,
        usedLevel: '5',
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles($files);
    $types = $descriptions = [];
    $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $files[0], [$files[0] => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$types, &$descriptions): void {
            if (!$node instanceof PhpParser\Node\Expr\FuncCall || !$node->name instanceof PhpParser\Node\Name || $node->name->getLast() !== 'observe') return;
            $type = $scope->getType($node->getArgs()[0]->value);
            $line = $node->getStartLine();
            $descriptions[$line] = $type->describe(PHPStan\Type\VerbosityLevel::precise());
            $expr = $node->getArgs()[0]->value;
            $record = ['line' => $line, 'span' => [$expr->getStartFilePos(), $expr->getEndFilePos() + 1], 'non_empty' => $type->isIterableAtLeastOnce()->yes()];
            $record['is_list'] = $type->isList()->yes();
            $size = $type->getArraySize();
            $record['count'] = $size instanceof PHPStan\Type\Constant\ConstantIntegerType ? $size->getValue() : null;
            $arrays = $type->getConstantArrays();
            if ($arrays !== []) {
                $array = $arrays[0];
                $slots = [];
                foreach ($array->getKeyTypes() as $i => $key) {
                    $slots[] = [scalars($key)[0], $array->isOptionalKey($i), scalars($array->getValueTypes()[$i])];
                }
                usort($slots, static fn($a, $b) => strcmp($a[0], $b[0]));
                $record['slots'] = $slots;
            } else {
                $record['keys'] = scalars($type->getIterableKeyType());
                $record['values'] = scalars($type->getIterableValueType());
            }
            $types[] = $record;
        },
    );
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
    $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
        ->finalize($result, onlyFiles: false, debug: true)->getAnalyserResult();
    if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan reported an internal error');
    $errors = array_map(static fn($error) => [
        $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
    ], $result->getUnorderedErrors());
    sort($errors);
    echo json_encode(['types' => $types, 'diagnostics' => $errors, 'descriptions' => $descriptions], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
