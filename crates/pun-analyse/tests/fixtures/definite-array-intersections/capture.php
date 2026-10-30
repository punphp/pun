<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
require 'phar://' . realpath($argv[1]) . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-definite-arrays-' . getmypid();
mkdir($root);
try {
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create($root, [], []);
    $resolver = $container->getByType(PHPStan\PhpDoc\TypeStringResolver::class);
    function describe(PHPStan\Type\Type $type): mixed {
        if ($type instanceof PHPStan\Type\NeverType) {
            return 'never';
        }
        if ($type->isArray()->yes()) {
            $arrays = $type->getConstantArrays();
            if (count($arrays) > 1) {
                throw new RuntimeException('Retained array intersection in ' . $GLOBALS['name'] . ': ' . $type->describe(PHPStan\Type\VerbosityLevel::precise()));
            }
            $items = [];
            $tail = null;
            if (count($arrays) === 1) {
                $array = $arrays[0];
                foreach ($array->getKeyTypes() as $i => $key) {
                    $items[(is_int($key->getValue()) ? 'i:' : 's:') . $key->getValue()] = [
                        $array->isOptionalKey($i), describe($array->getValueTypes()[$i]),
                    ];
                }
                $unsealed = $array->getUnsealedTypes();
                if ($unsealed !== null && !$array->isUnsealed()->no()) {
                    $tail = [describe($unsealed[0]), describe($unsealed[1])];
                }
            } else {
                $tail = [describe($type->getIterableKeyType()), describe($type->getIterableValueType())];
            }
            ksort($items);
            return [
                'items' => (object) $items, 'tail' => $tail, 'callable' => $type->isCallable()->yes(),
                'size' => describe($type->getArraySize()), 'nonempty' => $type->isIterableAtLeastOnce()->describe(),
                'constant' => count($arrays) === 1,
            ];
        }
        return str_replace(' ', '', $type->describe(PHPStan\Type\VerbosityLevel::precise()));
    }
    $out = [];
    foreach (json_decode(file_get_contents(__DIR__ . '/cases.json'), true, flags: JSON_THROW_ON_ERROR) as [$name, $a, $b]) {
        // PHPDoc notation supplies values, not literal-array sealedness: its default is Maybe.
        // Model the definite arrays emitted by ArrayHandler, retaining explicit open tails.
        $definite = static function (PHPStan\Type\Type $type, callable $traverse): PHPStan\Type\Type {
            $type = $traverse($type);
            if (!$type instanceof PHPStan\Type\Constant\ConstantArrayType) {
                return $type;
            }
            return new PHPStan\Type\Constant\ConstantArrayType(
                $type->getKeyTypes(), $type->getValueTypes(), $type->getNextAutoIndexes(),
                $type->getOptionalKeys(), $type->isList(),
                $type->getUnsealedTypes() ?? [new PHPStan\Type\NeverType(true), new PHPStan\Type\NeverType(true)],
            );
        };
        $a = PHPStan\Type\TypeTraverser::map($resolver->resolve($a), $definite);
        $b = PHPStan\Type\TypeTraverser::map($resolver->resolve($b), $definite);
        $out[$name] = [describe(PHPStan\Type\TypeCombinator::intersect($a, $b)), describe(PHPStan\Type\TypeCombinator::intersect($b, $a))];
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
