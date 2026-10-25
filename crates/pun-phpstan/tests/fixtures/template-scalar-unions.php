<?php
// PHPStan 2.3.1 cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php template-scalar-unions.php /path/to/phpstan.phar > template-scalar-unions.json
Phar::loadPhar($argv[1], 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

use PHPStan\Analyser\NameScope;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Type\Generic\TemplateType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\Type;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\UnionType;
use PHPStan\Type\VerbosityLevel;

$tmp = sys_get_temp_dir().'/pun-template-scalar-oracle';
@mkdir($tmp);
$resolver = (new ContainerFactory($tmp))->create($tmp, [], [])->getByType(TypeStringResolver::class);
$scope = new NameScope(null, []);
$input = function (string|array $spec, bool $argument = false) use (&$input, $resolver, $scope): Type {
    if (is_string($spec)) {
        return $resolver->resolve($spec, $scope);
    }
    $argument = $argument || $spec['argument'];
    $template = TemplateTypeFactory::create(
        TemplateTypeScope::createWithFunction($spec['function'] ?? 'fixture'),
        $spec['name'],
        $input($spec['bound'], $argument),
        TemplateTypeVariance::createInvariant(),
    );
    return $argument ? $template->toArgument() : $template;
};
$rows = [];
$record = function (array $inputs) use (&$rows, $input): void {
    $result = TypeCombinator::union(...array_map($input, $inputs));
    $rows[] = [
        'inputs' => $inputs,
        'descriptions' => array_map(fn ($level) => $result->describe($level), [VerbosityLevel::precise(), VerbosityLevel::value(), VerbosityLevel::typeOnly()]),
        'members' => $result instanceof UnionType && !$result instanceof TemplateType ? count($result->getTypes()) : 1,
    ];
};
foreach ([
    ["'duplicate'", ["'duplicate'", "'other'", 'string', 'int', 'null']],
    ['1', ['1', '2', 'int', 'int<0, 3>', 'string', 'null']],
    ['null', ['null', 'string', 'bool']],
    ['true', ['true', 'false', 'bool', 'null']],
    ['string', ["'duplicate'", 'string', 'null']],
    ['int', ['1', 'int', 'null']],
    ["'duplicate'|'other'", ["'duplicate'", 'string', 'null']],
    ["__benevolent<'duplicate'|'other'>", ["'duplicate'", 'string', 'null']],
] as [$bound, $others]) {
    foreach ([false, true] as $argument) {
        $direct = ['name' => 'T', 'bound' => $bound, 'argument' => $argument];
        $nested = ['name' => 'U', 'bound' => $direct, 'argument' => $argument];
        foreach ([$direct, $nested] as $template) {
            foreach ($others as $other) {
                $record([$template, $other]);
                $record([$other, $template]);
            }
        }
        // Constant-template buckets key by cache description, not TemplateType::equals().
        $record([$direct, $direct]);
        $record([$direct, [...$direct, 'argument' => !$argument]]);
        $record([$direct, [...$direct, 'name' => 'Other']]);
        $record([$direct, [...$direct, 'function' => 'other']]);
    }
}
foreach ([false, true] as $argument) {
    $template = ['name' => 'T', 'bound' => "'duplicate'", 'argument' => $argument];
    $record([$template, "'duplicate'", "'duplicate'", "'other'", '1', 'null']);
    $record([$template, 'int', 'string']);
    $record([['name' => 'T', 'bound' => '1', 'argument' => $argument], '1.0', '2.0']);
    $record([['name' => 'T', 'bound' => '2', 'argument' => $argument], '1', '0.5']);
    $record([['name' => 'N', 'bound' => ['name' => 'U', 'bound' => 'null', 'argument' => $argument], 'argument' => $argument], 'null', 'true']);
}
echo json_encode($rows, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
