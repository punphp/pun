<?php declare(strict_types=1);

// PHPStan 2.3.1; phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/phpstan.phar /tmp/oracle-cache > expected.json
require 'phar://' . realpath($argv[1]) . '/vendor/autoload.php';

use PHPStan\Analyser\NameScope;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Type as T;
use PHPStan\Type\Accessory\HasMethodType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel as V;

$tmp = $argv[2];
@mkdir($tmp, recursive: true);
$container = (new ContainerFactory($tmp))->create($tmp, [], []);
$resolver = $container->getByType(TypeStringResolver::class);
$parent = new T\NonexistentParentClassType();
$named = new T\ObjectType('parent');
$object = new T\ObjectWithoutClassType();
$std = new T\ObjectType('stdClass');
$null = new T\NullType();
$levels = [V::precise(), V::value(), V::typeOnly()];
$describe = fn ($type) => array_map(fn ($level) => $type->describe($level), $levels);
$tri = fn ($result) => $result->result->describe();
$types = [
    'parent-sentinel' => $parent,
    'named-parent' => $named,
    'stdClass' => $std,
    'object' => $object,
    'generic-object' => new T\Generic\GenericObjectType('ArrayObject', [new T\IntegerType()]),
    'shape-empty' => new T\ObjectShapeType([], []),
    'shape-optional' => new T\ObjectShapeType(['foo' => new T\IntegerType()], ['foo']),
    'shape-required' => new T\ObjectShapeType(['foo' => new T\IntegerType()], []),
    'callable' => new T\CallableType(),
    'iterable' => new T\IterableType(new T\MixedType(), new T\MixedType()),
    'error' => new T\ErrorType(),
    'mixed-implicit' => new T\MixedType(),
    'mixed-explicit' => new T\MixedType(true),
    'strict-mixed' => new T\StrictMixedType(),
    'never-implicit' => new T\NeverType(),
    'never-explicit' => new T\NeverType(true),
    'never-nonaccepting' => new T\NonAcceptingNeverType(),
    'null' => $null,
    'union-null' => new T\UnionType([$parent, $null]),
    'benevolent-null' => new T\BenevolentUnionType([$parent, $null]),
    'intersection-method' => new T\IntersectionType([$parent, new HasMethodType('foo')]),
    'union-object' => new T\UnionType([$parent, $std]),
    'benevolent-object' => new T\BenevolentUnionType([$parent, $std]),
    'template-union' => TemplateTypeFactory::create(
        TemplateTypeScope::createWithFunction('fixture'), 'T',
        new T\UnionType([$parent, $null]), TemplateTypeVariance::createInvariant(),
    ),
];
$out = [];
foreach ($types as $name => $type) {
    $out['relations'][$name] = [
        'equals' => [$parent->equals($type), $type->equals($parent)],
        'super' => [$tri($parent->isSuperTypeOf($type)), $tri($type->isSuperTypeOf($parent))],
        'accepts' => array_map(fn ($strict) => [
            $tri($parent->accepts($type, $strict)), $tri($type->accepts($parent, $strict)),
        ], [false, true]),
        'names' => $type->getObjectClassNames(),
        'isObject' => $type->isObject()->describe(),
    ];
}
$queries = [
    'isObject', 'isString', 'isInteger', 'isFloat', 'isBoolean', 'isArray', 'isList',
    'isIterable', 'isIterableAtLeastOnce', 'isCloneable', 'isCallable', 'isNull', 'isVoid',
];
foreach ($queries as $method) {
    $out['leaf']['queries'][$method] = $parent->$method()->describe();
}
$out['leaf']['descriptions'] = $describe($parent);
$out['leaf']['phpdoc'] = (string) $parent->toPhpDocNode();
$out['leaf']['constantStrings'] = $parent->getConstantStrings();
$out['leaf']['constantArrays'] = count($parent->getConstantArrays());
$out['leaf']['hasMethod'] = $parent->hasMethod('foo')->describe();
$out['leaf']['iterableKey'] = $describe($parent->getIterableKeyType());
$out['leaf']['iterableValue'] = $describe($parent->getIterableValueType());
$calls = 0;
$traversed = $parent->traverse(function ($type) use (&$calls) { $calls++; return new T\StringType(); });
$out['leaf']['traverse'] = [$parent->equals($traversed), $calls];
$out['leaf']['generalize'] = $parent->equals($parent->generalize(T\GeneralizePrecision::lessSpecific()));
$out['leaf']['inference'] = count($parent->inferTemplateTypes($types['template-union'])->getTypes());
foreach (['parent-sentinel', 'named-parent', 'object', 'error', 'mixed-implicit', 'never-implicit'] as $name) {
    foreach (['toString', 'toNumber', 'toInteger', 'toFloat', 'toBoolean', 'toBitwiseNotType'] as $method) {
        $out['casts'][$name][] = $types[$name]->$method() instanceof T\ErrorType;
    }
}
foreach (['named-parent', 'object', 'null', 'error', 'mixed-implicit', 'never-implicit'] as $name) {
    foreach ([false, true] as $reverse) {
        $pair = [$parent, $types[$name]];
        if ($reverse) { $pair = array_reverse($pair); }
        $out['combinators'][$name][] = [
            'union' => $describe(TypeCombinator::union(...$pair)),
            'intersection' => $describe(TypeCombinator::intersect(...$pair)),
        ];
    }
}
foreach (['stdClass', 'RuntimeException', null, 'UnknownClass'] as $class) {
    foreach (['parent', 'class-string<parent>', 'array<parent, int>', 'parent<int>', 'parent[0]', 'key-of<parent>', 'value-of<parent>'] as $input) {
        $result = $resolver->resolve($input, new NameScope(null, [], $class));
        $out['resolution'][] = [
            'class' => $class, 'input' => $input, 'descriptions' => $describe($result),
            'names' => $result->getObjectClassNames(), 'sentinel' => $result instanceof T\NonexistentParentClassType,
        ];
    }
}
$template = TypeCombinator::intersect($types['template-union'], $parent);
$out['templateBound'] = [$describe($template->getBound()), $template->getBound()->isExplicitMixed()];
foreach (['null', 'callable', 'iterable', 'object{}', 'Closure', 'string', 'stdClass'] as $input) {
    $out['objectQueries'][$input] = $resolver->resolve($input)->isObject()->describe();
}
$out['objectQueries']['mixed~object'] = (new T\MixedType(false, $object))->isObject()->describe();
$out['objectQueries']['hasMethod'] = (new HasMethodType('foo'))->isObject()->describe();
$out['nativeParent'] = (T\ParserNodeTypeToPHPStanType::resolve(new PhpParser\Node\Name('parent'), null))->getObjectClassNames();
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n";
