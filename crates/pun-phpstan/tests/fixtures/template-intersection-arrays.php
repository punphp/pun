<?php
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php template-intersection-arrays.php /path/to/phpstan.phar > template-intersection-arrays.json
Phar::loadPhar($argv[1], 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Type\Accessory\AccessoryArrayListType;
use PHPStan\Type\ArrayType;
use PHPStan\Type\Generic\TemplateType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\IntegerType;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel;

$tmp = sys_get_temp_dir().'/pun-template-intersection-arrays';
@mkdir($tmp);
$container = (new ContainerFactory($tmp))->create($tmp, [], []);
$resolver = $container->getByType(TypeStringResolver::class);
$makeTemplate = function ($bound, $argument, $name = 'T', $default = 'array{}') use ($resolver) {
    $template = TemplateTypeFactory::create(
        TemplateTypeScope::createWithFunction('fixture'),
        $name,
        $bound,
        TemplateTypeVariance::createInvariant(),
        default: $resolver->resolve($default),
    );
    return $argument ? $template->toArgument() : $template;
};
$describe = fn ($type) => array_map(
    fn ($level) => $type->describe($level),
    [VerbosityLevel::precise(), VerbosityLevel::value(), VerbosityLevel::typeOnly()],
);
$rows = [];
foreach ([
    ['list<never>', 'array{}'],
    ['list<int>', 'array{}'],
    ['list<int>', "array{'value'}"],
    ['list<int>', 'array{key: string}'],
    ['list<int>', 'array<int, string>'],
    ['list<int>', 'list<string>'],
    ['non-empty-list<int>', "array{'value'}"],
    ['non-empty-list<int>', 'array{}'],
    ['non-empty-list<int>', 'list<string>'],
    ['list<int>', 'non-empty-list<string>'],
    ['non-empty-array<string, int>', 'array{key: string}'],
    ['non-empty-array<string, int>', 'array{}'],
    ['non-empty-array<string, int>', 'array<string, string>'],
    ['list<int>', null],
    ['non-empty-list<int>', null],
    ['list<never>', null],
    ['array<string, int>', null],
    ['int', null],
    ['list<int>', 'null'],
    ['list<int>', 'int'],
    ['list<U>', 'array{}', true],
    ['list<U>', "array{'value'}", true],
] as $case) {
    [$bound, $other] = $case;
    $elementTemplate = $case[2] ?? false;
    foreach ([false, true] as $argument) {
        $boundType = $elementTemplate
            ? TypeCombinator::intersect(
                new ArrayType(new IntegerType(), $makeTemplate($resolver->resolve('int'), $argument, 'U', '1')),
                new AccessoryArrayListType(),
            )
            : $resolver->resolve($bound);
        $template = $makeTemplate($boundType, $argument);
        foreach ($other === null ? [false] : [false, true] as $reverse) {
            $pair = $other === null ? [$template] : [$template, $resolver->resolve($other)];
            $result = TypeCombinator::union(...($reverse ? array_reverse($pair) : $pair));
            $rows[] = [
                'bound' => $bound,
                'other' => $other,
                'elementTemplate' => $elementTemplate,
                'argument' => $argument,
                'reverse' => $reverse,
                'descriptions' => $describe($result),
                'queries' => array_map(fn ($answer) => $answer->describe(), [
                    $result->isArray(), $result->isList(), $result->isIterableAtLeastOnce(),
                ]),
                'template' => $result instanceof TemplateType ? [
                    'name' => $result->getName(),
                    'scope' => $result->getScope()->describe(),
                    'argument' => $result->getStrategy()->isArgument(),
                    'bound' => $describe($result->getBound()),
                    'default' => $describe($result->getDefault()),
                ] : null,
            ];
        }
    }
}
echo json_encode($rows, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
