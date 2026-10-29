<?php declare(strict_types=1);

// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php object-subtraction-api.php /absolute/path/phpstan.phar /tmp/oracle-cache > object-subtraction-api.json
require 'phar://' . realpath($argv[1]) . '/vendor/autoload.php';

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Type as T;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel as V;

$tmp = $argv[2];
@mkdir($tmp, recursive: true);
$resolver = (new ContainerFactory($tmp))->create($tmp, [], [])->getByType(TypeStringResolver::class);
$inputs = [
    'exception' => 'Exception', 'runtime' => 'RuntimeException', 'logic' => 'LogicException',
    'std' => 'stdClass', 'mixed' => 'mixed', 'never' => 'never',
    'not-runtime' => ['object' => 'Exception', 'excluded' => 'RuntimeException'],
    'not-logic' => ['object' => 'Exception', 'excluded' => 'LogicException'],
    'throwable-not-runtime' => ['object' => 'Throwable', 'excluded' => 'RuntimeException'],
    'logic-not-exception' => ['object' => 'LogicException', 'excluded' => 'Exception'],
    'iterator-not-array' => ['object' => 'Iterator<int, string>', 'excluded' => 'ArrayIterator<int, string>'],
    'union' => ['union' => ['RuntimeException', 'LogicException']],
];
$input = function ($spec) use (&$input, $resolver): T\Type {
    if (is_string($spec)) { return $resolver->resolve($spec); }
    if (isset($spec['object'])) {
        return $input($spec['object'])->changeSubtractedType($input($spec['excluded']));
    }
    return new T\UnionType(array_map($input, $spec['union']));
};
$types = array_map($input, $inputs);
$describe = fn ($type) => array_map(fn ($v) => $type->describe($v), [V::precise(), V::value(), V::typeOnly()]);
$out = ['inputs' => $inputs];
foreach (['not-runtime', 'iterator-not-array'] as $name) {
    $type = $types[$name];
    $out['queries'][$name] = [
        'descriptions' => $describe($type), 'names' => $type->getObjectClassNames(),
        'key' => $describe($type->getIterableKeyType()), 'value' => $describe($type->getIterableValueType()),
    ];
}
foreach (['exception', 'runtime', 'logic', 'std', 'not-logic', 'throwable-not-runtime',
    'logic-not-exception', 'union', 'mixed', 'never'] as $other) {
    foreach ([['not-runtime', $other], [$other, 'not-runtime']] as [$a, $b]) {
        $out['super']["$a:$b"] = $types[$a]->isSuperTypeOf($types[$b])->result->describe();
    }
}
foreach (['logic', 'runtime', 'std', 'not-logic'] as $other) {
    foreach ([['not-runtime', $other], [$other, 'not-runtime']] as [$a, $b]) {
        $out['unions']["$a:$b"] = $describe(TypeCombinator::union($types[$a], $types[$b]));
    }
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR) . "\n";
