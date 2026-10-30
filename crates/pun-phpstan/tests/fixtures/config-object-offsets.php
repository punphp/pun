<?php
// php config-object-offsets.php /absolute/path/to/phpstan.phar /output.json
declare(strict_types=1);

$phar = realpath($argv[1]);
require "phar://$phar/vendor/autoload.php";
$tmp = sys_get_temp_dir().'/pun-config-object-offsets-'.getmypid();
mkdir($tmp);
try {
    $resolver = (new PHPStan\DependencyInjection\ContainerFactory($tmp))
        ->create($tmp, [], [])->getByType(PHPStan\PhpDoc\TypeStringResolver::class);
    $rows = [];
    foreach (['stdClass', 'DateTimeInterface', 'Generator', 'SensitiveParameterValue',
        'ConfigShapeMissingObject', 'DOMNodeList', 'FFI\\CData', 'ArrayAccess<int, string>',
        'ArrayObject<int, string>'] as $name) {
        $type = $resolver->resolve($name);
        foreach ([new PHPStan\Type\Constant\ConstantIntegerType(0),
            new PHPStan\Type\Constant\ConstantStringType('key')] as $offset) {
            $value = $type->getOffsetValueType($offset);
            $rows[] = ['type' => $name, 'offset' => $offset->describe(PHPStan\Type\VerbosityLevel::precise()),
                'value' => $value->describe(PHPStan\Type\VerbosityLevel::precise()),
                'array' => $value->isArray()->describe(),
                'explicit' => $value instanceof PHPStan\Type\MixedType ? $value->isExplicitMixed() : null];
        }
    }
    file_put_contents($argv[2], json_encode(['phar_sha256' => hash_file('sha256', $phar),
        'rows' => $rows], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR)."\n");
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) { $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname()); }
    rmdir($tmp);
}
