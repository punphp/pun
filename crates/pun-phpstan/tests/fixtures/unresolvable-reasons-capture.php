<?php declare(strict_types=1);

// php unresolvable-reasons-capture.php /path/to/phpstan.phar unresolvable-reasons.json > refreshed.json
// Refreshes only api; the older oracle observations remain unchanged.
Phar::loadPhar($argv[1], 'reason-api.phar');
require 'phar://reason-api.phar/vendor/autoload.php';

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Reflection\ReflectionProvider;
use PHPStan\Type\ErrorType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeHelper;
use PHPStan\Type\Generic\TemplateTypeMap;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\Generic\TemplateTypeVarianceMap;
use PHPStan\Type\MixedType;
use PHPStan\Type\NeverType;
use PHPStan\Type\ObjectType;
use PHPStan\Type\Type;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\TypeUtils;
use PHPStan\Type\VerbosityLevel;

function observation(Type $type): array
{
    return [$type->describe(VerbosityLevel::precise()), method_exists($type, 'getReason') ? $type->getReason() : null];
}

$tmp = sys_get_temp_dir() . '/pun-reason-api-' . getmypid();
mkdir($tmp);
file_put_contents($tmp . '/source.php', <<<'PHP'
<?php
namespace ReasonApi;
final class Missing {}
final class Readable { public int $target = 1; }
final class PrivateProperty { private int $target = 1; }
final class WriteOnly { public int $target { set {} } }
final class WrongValue { public string $target = 'one'; }
class Extensible {}
final class Ordered { public int $first = 1; private int $second = 2; }
PHP);
file_put_contents($tmp . '/config.neon', "parameters:\n    phpVersion: 80500\n    scanFiles:\n        - $tmp/source.php\n");
$container = (new ContainerFactory($tmp))->create($tmp . '/cache', [$tmp . '/config.neon'], [$tmp . '/source.php']);
$resolver = $container->getByType(TypeStringResolver::class);
$provider = $container->getByType(ReflectionProvider::class);
$result = ['phpstan' => '2.3.1'];

foreach (['Missing', 'Readable', 'PrivateProperty', 'WriteOnly', 'WrongValue', 'Extensible', 'Ordered'] as $name) {
    $name = 'ReasonApi\\' . $name;
    $class = $provider->getClass($name);
    $object = new ObjectType($name);
    $properties = [];
    foreach (['target', 'first', 'second'] as $property) {
        if (!$class->hasInstanceProperty($property)) {
            continue;
        }
        $reflection = $class->getNativeProperty($property);
        $properties[] = [
            'name' => $property, 'declaring' => $reflection->getDeclaringClass()->getDisplayName(),
            'public' => $reflection->isPublic(), 'readable' => $reflection->isReadable(),
            'type' => $reflection->getReadableType()->describe(VerbosityLevel::precise()),
        ];
    }
    $row = [
        'class' => $name, 'final' => $class->isFinalByKeyword(),
        'missing' => $object->hasInstanceProperty('absent')->describe(), 'properties' => $properties,
    ];
    foreach (['object{target: int}', 'object{target?: int}', 'object{first: int, second: int}', 'object{second: int, first: int}'] as $shape) {
        $type = $resolver->resolve($shape);
        $row['intersections'][] = [
            'shape' => $shape,
            'objectFirst' => observation(TypeCombinator::intersect($object, $type)),
            'shapeFirst' => observation(TypeCombinator::intersect($type, $object)),
        ];
    }
    $result['properties'][] = $row;
}

$templates = [];
foreach (['K', 'L'] as $name) {
    $templates[$name] = TemplateTypeFactory::create(TemplateTypeScope::createWithFunction('shape'), $name, new MixedType(), TemplateTypeVariance::createInvariant());
}
$scope = new PHPStan\Analyser\NameScope(null, [], null, 'shape', new TemplateTypeMap($templates));
foreach ([
    ['array{K: int}', 'array{target: int}'],
    ['array{K?: int}', 'array{target: int}'],
    ['array{K: K|L}', 'array{target: string}'],
] as [$input, $other]) {
    $late = $resolver->resolve($input, $scope);
    $intersection = TypeCombinator::intersect($late, $resolver->resolve($other));
    $map = new TemplateTypeMap(['K' => $resolver->resolve("'target'"), 'L' => $resolver->resolve("'value'")]);
    $substitute = static fn (Type $type): Type => TemplateTypeHelper::resolveTemplateTypes($type, $map, new TemplateTypeVarianceMap([]), TemplateTypeVariance::createContravariant());
    $result['late'][] = [
        'input' => $input, 'other' => $other,
        'hasMethod' => $late->hasMethod('absent')->describe(),
        'iterableValue' => $late->getIterableValueType()->describe(VerbosityLevel::precise()),
        'resolved' => observation(TypeUtils::resolveLateResolvableTypes($late)),
        'intersection' => observation($intersection),
        'substituted' => observation($substitute($intersection)),
    ];
}
foreach ([
    ['array{item: int}', 'array{item: int}', false],
    ['array{item: int}', 'array{item: string}', false],
    ['Closure(int): string', 'Closure(int): string', false],
    ['Closure(int): string', 'Closure(string): string', false],
    ['array{K: int}', 'array{K: int}', true],
    ['array{K: int}', 'array{L: int}', true],
    ['array{K: int}', 'array{K?: int}', true],
] as [$left, $right, $templated]) {
    $a = $resolver->resolve($left, $templated ? $scope : null);
    $b = $resolver->resolve($right, $templated ? $scope : null);
    $result['identity'][] = [$left, $right, $templated, $a->equals($b)];
}
foreach (['error' => new ErrorType('An array key reason.'), 'never' => new NeverType(false, 'An intersection reason.')] as $name => $type) {
    $result['metadata'][$name] = [
        'traverse' => observation($type->traverse(static fn (Type $child): Type => $child)),
        'substitute' => observation(TemplateTypeHelper::resolveTemplateTypes($type, new TemplateTypeMap([]), new TemplateTypeVarianceMap([]), TemplateTypeVariance::createContravariant())),
    ];
}
foreach ([
    'error' => new ErrorType(),
    'reasoned-error' => new ErrorType('An array key reason.'),
    'implicit' => new MixedType(),
    'explicit' => new MixedType(true),
    'implicit-subtracted' => new MixedType(false, new PHPStan\Type\IntegerType()),
    'explicit-subtracted' => new MixedType(true, new PHPStan\Type\IntegerType()),
] as $name => $type) {
    foreach (['0', "'known'"] as $offset) {
        $value = $type->getOffsetValueType($resolver->resolve($offset));
        $result['mixedOffsets'][] = [
            'name' => $name, 'offset' => $offset, 'value' => observation($value),
            'explicit' => $value->isExplicitMixed(), 'isArray' => $value->isArray()->describe(),
        ];
    }
}

$fixture = json_decode(file_get_contents($argv[2]), true, flags: JSON_THROW_ON_ERROR);
$fixture['api'] = $result;
echo json_encode($fixture, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR) . "\n";

function removeTemporary(string $path): void
{
    foreach (scandir($path) as $name) {
        if ($name === '.' || $name === '..') {
            continue;
        }
        $file = $path . '/' . $name;
        is_dir($file) ? removeTemporary($file) : unlink($file);
    }
    rmdir($path);
}
removeTemporary($tmp);
