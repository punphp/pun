<?php
// php exact-objects.php /path/to/phpstan-2.3.1.phar > exact-objects.json
// PHPStan 2.3.1 cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
Phar::loadPhar($argv[1], 'exact-objects.phar');
require 'phar://exact-objects.phar/vendor/autoload.php';

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Reflection\ReflectionProvider;
use PHPStan\Type\Generic\GenericObjectType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\IntegerType;
use PHPStan\Type\IntersectionType;
use PHPStan\Type\MixedType;
use PHPStan\Type\ObjectType;
use PHPStan\Type\StaticType;
use PHPStan\Type\ThisType;
use PHPStan\Type\Type;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\TypeTraverser;
use PHPStan\Type\VerbosityLevel;

$tmp = sys_get_temp_dir() . '/pun-exact-api-' . getmypid();
$provider = (new ContainerFactory($tmp))->create($tmp, [], [])->getByType(ReflectionProvider::class);
$object = static fn (string $name, bool $exact): ObjectType => new ObjectType(
    $name, classReflection: $exact ? $provider->getClass($name)->asFinal() : null,
);
$generic = static fn (bool $exact, Type $item): GenericObjectType => new GenericObjectType(
    'ArrayObject', [new IntegerType(), $item],
    classReflection: $exact ? $provider->getClass('ArrayObject')->asFinal() : null,
);
$types = [
    'broad' => $object('Exception', false),
    'exact' => $object('Exception', true),
    'child' => $object('RuntimeException', true),
    'broad-child' => $object('RuntimeException', false),
    'other' => $object('LogicException', true),
    'static' => new StaticType($provider->getClass('Exception')),
    'this' => new ThisType($provider->getClass('Exception')),
    'interface' => $object('Throwable', false),
    'generic-broad' => $generic(false, $object('Exception', false)),
    'generic-exact' => $generic(true, $object('Exception', false)),
    'generic-nested' => $generic(true, $object('Exception', true)),
    'cased' => $object('exception', false),
    'cased-exact' => $object('exception', true),
    'final' => $object('Closure', true),
];
function observation(Type $type): array {
    $exact = [];
    $visit = static function (Type $type, callable $traverse) use (&$exact): Type {
        if ($type instanceof ObjectType && !$type instanceof StaticType) {
            $reflection = $type->getClassReflection();
            $exact[] = [$type->getClassName(), $reflection !== null && $reflection->hasFinalByKeywordOverride()];
        }
        return $traverse($type);
    };
    TypeTraverser::map($type, $visit);
    sort($exact);
    return [
        $type->describe(VerbosityLevel::precise()),
        $exact,
    ];
}
$rows = [];
foreach ($types as $left => $a) {
    foreach ($types as $right => $b) {
        ObjectType::resetCaches();
        $rows[] = [
            'left' => $left, 'right' => $right,
            'equals' => $a->equals($b),
            'super' => $a->isSuperTypeOf($b)->result->describe(),
            'accepts' => $a->accepts($b, true)->result->describe(),
            'union' => observation(TypeCombinator::union($a, $b)),
            'intersection' => observation(TypeCombinator::intersect($a, $b)),
        ];
    }
}
// Exclusion metadata survives ordinary traversal; subtraction simplification is tested separately.
$types['excluded'] = new ObjectType('Exception', $object('RuntimeException', false), $provider->getClass('Exception')->asFinal());
$types['excluded-nested'] = new ObjectType('Exception', $object('RuntimeException', true), $provider->getClass('Exception')->asFinal());
$types['receiver-conjunction'] = new IntersectionType([$object('Exception', true), new ThisType($provider->getClass('Exception'))]);
$preserved = $phpdoc = [];
foreach ($types as $name => $type) {
    $preserved[$name] = observation(TypeTraverser::map($type, static fn (Type $t, callable $traverse): Type => $traverse($t)));
    if (!str_starts_with($name, 'excluded') && $name !== 'receiver-conjunction') {
        $phpdoc[$name] = (string) $type->toPhpDocNode();
    }
}
$template = TemplateTypeFactory::create(
    TemplateTypeScope::createWithFunction('exact'), 'T', new MixedType(), TemplateTypeVariance::createInvariant(),
);
$inferred = [];
foreach (['exact', 'generic-nested', 'excluded', 'excluded-nested', 'static', 'this', 'receiver-conjunction'] as $name) {
    $inferred[$name] = observation($template->inferTemplateTypes($types[$name])->getType('T'));
}
$queries = [];
foreach ($types as $name => $type) {
    $queries[$name] = [$type->isCallable()->describe(), $type->hasMethod('missing')->describe()];
}
$result = ['pairs' => $rows, 'preserved' => $preserved, 'phpdoc' => $phpdoc, 'inferred' => $inferred, 'queries' => $queries];
echo "{\n";
foreach ($result as $name => $value) {
    $items = [];
    foreach ($value as $key => $item) {
        $items[] = '    ' . (is_array($value) && array_is_list($value) ? '' : json_encode($key) . ': ')
            . json_encode($item, JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR);
    }
    $list = array_is_list($value);
    echo '  ' . json_encode($name) . ': ' . ($list ? '[' : '{') . "\n" . implode(",\n", $items)
        . "\n  " . ($list ? ']' : '}') . ($name === 'queries' ? '' : ',') . "\n";
}
echo "}\n";
