<?php
// php capture-relations.php /absolute/path/to/phpstan-2.3.1.phar > expected-relations.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
require __DIR__ . '/relation-controls.php';
$tmp = sys_get_temp_dir() . '/pun-closure-relations-' . getmypid();
mkdir($tmp);
try {
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        $tmp, ['phar://' . $phar . '/conf/config.level9.neon'], [],
    );
    $resolver = $container->getByType(PHPStan\PhpDoc\TypeStringResolver::class);
    $opaque = new PHPStan\Type\ObjectType(Closure::class);
    $result = ['phar_sha256' => hash_file('sha256', $phar), 'relations' => []];
    foreach (['callable', 'callable(): mixed', 'callable(int): string', 'Closure(): mixed', 'Closure(int): string', 'pure-Closure(): int', 'pure-callable(): int'] as $name) {
        $type = $resolver->resolve($name);
        $result['relations'][$name] = [
            'accepts' => $type->accepts($opaque, true)->result->describe(),
            'supertype' => $type->isSuperTypeOf($opaque)->result->describe(),
            'intersection' => PHPStan\Type\TypeCombinator::intersect($type, $opaque)->describe(PHPStan\Type\VerbosityLevel::precise()),
            'union' => PHPStan\Type\TypeCombinator::union($type, $opaque)->describe(PHPStan\Type\VerbosityLevel::precise()),
        ];
    }
    $input = $resolver->resolve('RelationContra<callable(int): string>');
    $target = $resolver->resolve('RelationContra<Closure>');
    $result['contravariant_supertype'] = $target->isSuperTypeOf($input)->result->describe();
    $input = $resolver->resolve('RelationContra<Closure|int>');
    $target = $resolver->resolve('RelationContra<Closure(int): string>');
    $result['contravariant_union_supertype'] = $target->isSuperTypeOf($input)->result->describe();
    foreach ([
        'invariant' => ['RelationInvariant<Closure(int): string>', 'RelationInvariant<Closure>'],
        'iterable' => ['iterable<int, Closure>', 'iterable<int, callable(int): string>'],
    ] as $name => [$input, $target]) {
        $result[$name . '_supertype'] = $resolver->resolve($target)->isSuperTypeOf($resolver->resolve($input))->result->describe();
    }
    echo json_encode($result, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($tmp);
}
