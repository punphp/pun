<?php
// php generic-comparisons.php /path/to/phpstan-2.3.1.phar > generic-comparisons.json
Phar::loadPhar($argv[1], 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Rules\RuleLevelHelper;

$tmp = sys_get_temp_dir().'/pun-generic-comparisons-oracle';
@mkdir($tmp);
$resolver = (new ContainerFactory($tmp))->create($tmp, [], [])->getByType(TypeStringResolver::class);
$reflection = new ReflectionClass(RuleLevelHelper::class);
$helper = $reflection->newInstanceWithoutConstructor();
$reflection->getProperty('checkExplicitMixed')->setValue($helper, true);
$reflection->getProperty('checkImplicitMixed')->setValue($helper, true);
$pairs = [
    ['callable(int): int', 'callable<A>(A): A'],
    ['callable(int): string', 'callable<A>(A): A'],
    ['Closure(int): int', 'Closure<A>(A): A'],
    ['Closure(int): string', 'Closure<A>(A): A'],
    ['callable(): int', 'callable<A = int>(): A'],
    ['callable(): string', 'callable<A = int>(): A'],
    ['callable(int, string): int|string', 'callable<A>(A ...): A'],
    ['callable(int, string): int', 'callable<A>(A ...): A'],
    ['callable(): mixed', 'callable<A>(A ...): A'],
    ['callable(int): int', 'callable<A of string>(A): A'],
    ['callable(): mixed', 'callable<A>(A): A'],
    ['callable(int, int): int', 'callable<A>(A): A'],
    ['Traversable<mixed, DateTime>', 'ArrayIterator<int, DateTime>'],
    ['Traversable<int, DateTime>', 'ArrayIterator<int, DateTimeImmutable>'],
    ['Traversable<int, object>', 'ArrayIterator<int, DateTime>'],
    ['Generator<int, string, int, void>', 'Generator<int, string, mixed, void>'],
    ['Generator<int, string, mixed, void>', 'Generator<int, string, int, void>'],
    ['ArrayObject<int, int>', 'ArrayObject<int, mixed>'],
    ['ArrayObject<int, mixed>', 'ArrayObject<int, int>'],
    ['ArrayObject<int, __benevolent<int|string>>', 'ArrayObject<int, int>'],
    ['ArrayObject<int, int>', 'ArrayObject<int, __benevolent<int|string>>'],
    ['ArrayObject<int, int>', 'ArrayObject<int, never>'],
    ['ArrayObject<int, covariant int>', 'ArrayObject<int, mixed>'],
    ['ArrayObject<int, contravariant int>', 'ArrayObject<int, mixed>'],
    ['ArrayObject<int, covariant mixed>', 'ArrayObject<int, int>'],
];
$rows = [];
foreach ($pairs as [$left, $right]) {
    foreach ([false, true] as $checked) {
        $a = $resolver->resolve($left);
        $b = $resolver->resolve($right);
        if ($checked) {
            $b = $reflection->getMethod('transformCommonType')->invoke($helper, $b);
        }
        $accepted = $a->accepts($b, true);
        $rows[] = [$left, $right, $checked, $a->isSuperTypeOf($b)->result->describe(), $accepted->result->describe(), $accepted->reasons];
    }
}
echo json_encode($rows, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR)."\n";
