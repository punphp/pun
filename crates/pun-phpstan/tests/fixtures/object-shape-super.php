<?php
// php object-shape-super.php /path/to/phpstan-2.3.5.phar, one JSON row per line: [a, b, $a->isSuperTypeOf($b)].
Phar::loadPhar($argv[1], 'p.phar');
require 'phar://p.phar/vendor/autoload.php';
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
$tmp = sys_get_temp_dir() . '/pun-shape-' . getmypid();
$r = (new ContainerFactory($tmp))->create($tmp, [], [])->getByType(TypeStringResolver::class);
$t = ['object{a: int}', 'object{b: int}', 'object{a?: int}', 'object{a: int, b?: int}', 'object{a: 1}', 'object{a: string}'];
foreach ($t as $a) foreach ($t as $b) echo json_encode([$a, $b, $r->resolve($a)->isSuperTypeOf($r->resolve($b))->describe()]), "\n";
