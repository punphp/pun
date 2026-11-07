<?php

// php capture-channels.php /absolute/path/to/phpstan.phar [output.json]
// PHPStan 2.3.5, phpstan-src 95d9e8b0578ebf75439a3744a845ef25417028fa.
declare(strict_types=1);

Phar::loadPhar(realpath($argv[1]), 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';
$out = [];
foreach ([80400, 80500] as $version) {
    $tmp = sys_get_temp_dir() . '/pun-compound-channels-' . $version;
    @mkdir($tmp);
    file_put_contents($tmp . '/v.neon', "parameters:\n    phpVersion: $version\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory($tmp))->create($tmp, [$tmp . '/v.neon'], []);
    $provider = $container->getByType(PHPStan\Reflection\ReflectionProvider::class);
    $describe = fn (PHPStan\Type\Type $type) => $type->describe(PHPStan\Type\VerbosityLevel::precise());
    foreach ([['ArrayObject', 'offsetGet'], ['DateTimeImmutable', 'modify'], ['ArrayIterator', 'current'],
        ['Directory', 'read'], ['ReflectionGenerator', 'getExecutingLine'], ['Ds\\Deque', 'map']] as [$class, $method]) {
        foreach ($provider->getClass($class)->getNativeMethod($method)->getVariants() as $index => $variant) {
            $out[] = [
                'version' => $version,
                'class' => $class,
                'method' => $method,
                'variant' => $index,
                'effective' => $describe($variant->getReturnType()),
                'phpdoc' => $describe($variant->getPhpDocReturnType()),
                'native' => $describe($variant->getNativeReturnType()),
                'templates' => array_map($describe, $variant->getTemplateTypeMap()->getTypes()),
                'resolvedTemplates' => array_map($describe, $variant->getResolvedTemplateTypeMap()->getTypes()),
                'variances' => array_map(fn ($variance) => $variance->describe(), $variant->getCallSiteVarianceMap()->getVariances()),
            ];
        }
    }
}
file_put_contents($argv[2] ?? __DIR__ . '/channels.json', json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
