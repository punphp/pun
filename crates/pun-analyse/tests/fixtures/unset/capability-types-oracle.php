<?php
// php capability-types-oracle.php /absolute/path/to/phpstan/vendor/autoload.php
declare(strict_types=1);
require $argv[1];

use PHPStan\Type\ArrayType;
use PHPStan\Type\BenevolentUnionType;
use PHPStan\Type\ConditionalType;
use PHPStan\Type\Constant\ConstantArrayType;
use PHPStan\Type\Constant\ConstantIntegerType;
use PHPStan\Type\Constant\ConstantStringType;
use PHPStan\Type\Generic\GenericObjectType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\IntegerType;
use PHPStan\Type\IntersectionType;
use PHPStan\Type\MixedType;
use PHPStan\Type\ObjectType;
use PHPStan\Type\OffsetAccessType;
use PHPStan\Type\StringType;
use PHPStan\Type\UnionType;
use PHPStan\Type\VerbosityLevel;

$tmp = sys_get_temp_dir().'/pun-offset-capability-types';
@mkdir($tmp);
$container = (new PHPStan\DependencyInjection\ContainerFactory(getcwd()))->create($tmp, [], []);
PHPStan\Reflection\ReflectionProviderStaticAccessor::registerInstance(
    $container->getByType(PHPStan\Reflection\ReflectionProvider::class),
);
$mixed = new MixedType();
$int = new IntegerType();
$zero = new ConstantIntegerType(0);
$one = new ConstantIntegerType(1);
$string = new StringType();
$array = new ArrayType($mixed, $mixed);
$domain = new UnionType([$string, $array, new ObjectType(ArrayAccess::class)]);
$excluded = new MixedType(false, $domain);
$template = static fn ($bound) => TemplateTypeFactory::create(
    TemplateTypeScope::createWithFunction('offsetQueries'),
    'T', $bound, TemplateTypeVariance::createInvariant(),
);
$shape = new ConstantArrayType([$zero], [$int]);
$types = [
    'mixed' => $mixed,
    'not-zero' => new MixedType(false, $zero),
    'not-array-keys' => new MixedType(false, new UnionType([$int, $string])),
    'not-string-array' => new MixedType(false, new UnionType([$string, $array])),
    'not-offsets' => $excluded,
    'template-not-offsets' => $template($excluded),
    'union-no' => new UnionType([$excluded, $int]),
    'union-maybe' => new UnionType([$excluded, $string]),
    'benevolent-no' => new BenevolentUnionType([$excluded, $int]),
    'intersection-array' => new IntersectionType([$excluded, $array]),
    'conditional-array' => new ConditionalType($int, $int, $shape, $int, false),
    'conditional-union' => new ConditionalType($mixed, $int, $shape, $string, false),
    'conditional-key' => new ConditionalType($int, $int, $one, $zero, false),
    'late-array' => new OffsetAccessType($template(new ArrayType($string, $shape)), new ConstantStringType('inner')),
    'late-key' => new OffsetAccessType($template(new ArrayType($string, $one)), new ConstantStringType('inner')),
    'late-object-mixed' => new OffsetAccessType(new GenericObjectType(ArrayAccess::class, [$int, $mixed]), $zero),
    'late-array-error' => new OffsetAccessType(new ArrayType($string, new PHPStan\Type\ErrorType()), new ConstantStringType('inner')),
    'late-array-invalid-key' => new OffsetAccessType(new ArrayType($int, $one), new ConstantStringType('inner')),
    'late-shape-error' => new OffsetAccessType(new ConstantArrayType([$zero], [new PHPStan\Type\ErrorType()]), $zero),
    'late-shape-template-value' => new OffsetAccessType(new ConstantArrayType([$zero], [$template(new PHPStan\Type\FloatType())]), $zero),
    'late-array-template-value' => new OffsetAccessType($template(new ArrayType($string, $template(new PHPStan\Type\FloatType()))), new ConstantStringType('inner')),
    'late-array-template-mixed' => new OffsetAccessType(new ArrayType($string, $template($excluded)), new ConstantStringType('inner')),
    'late-template-mixed-container' => new OffsetAccessType($template($excluded), $zero),
    'late-conditional-container' => new OffsetAccessType(new ConditionalType($int, $int, new ArrayType($string, $one), $int, false), new ConstantStringType('inner')),
    'late-open' => new OffsetAccessType(new ConstantArrayType([], [], unsealed: [$string, $int]), new ConstantStringType('inner')),
    'late-open-template' => new OffsetAccessType($template(new ConstantArrayType([], [], unsealed: [$string, $int])), new ConstantStringType('inner')),
    'late-unknown-shape' => new OffsetAccessType(new ConstantArrayType([new ConstantStringType('inner')], [$mixed]), $string),
    'late-shape-general' => new OffsetAccessType(new ConstantArrayType([new ConstantStringType('inner')], [$int]), $string),
    'late-shape-numeric' => new OffsetAccessType($shape, $string),
    'late-shape-bad-union' => new OffsetAccessType($shape, new UnionType([new ConstantStringType('bad'), new ConstantStringType('missing')])),
    'late-shape-mixed-key' => new OffsetAccessType($shape, $template($mixed)),
    'late-shape-template-key' => new OffsetAccessType($shape, $template(new ConstantStringType('bad'))),
    'late-shape-nonstring' => new OffsetAccessType(new ConstantArrayType([new ConstantStringType('inner')], [$int]), $int),
    'late-open-incompatible' => new OffsetAccessType(new ConstantArrayType([], [], unsealed: [$string, $int]), $int),
    'late-shape-covered-extra' => new OffsetAccessType(new ConstantArrayType([$zero], [$template(new PHPStan\Type\FloatType())], unsealed: [$string, $int]), $zero),
    'template-key' => $template($int),
];
$keys = ['mixed', 'not-zero', 'not-array-keys', 'not-offsets', 'template-not-offsets', 'conditional-key', 'late-key', 'template-key',
    'late-object-mixed', 'late-array-error', 'late-array-invalid-key', 'late-shape-error',
    'late-shape-template-value', 'late-array-template-value', 'late-array-template-mixed',
    'late-template-mixed-container', 'late-conditional-container', 'late-open', 'late-open-template', 'late-unknown-shape',
    'late-shape-general', 'late-shape-numeric', 'late-shape-bad-union', 'late-shape-mixed-key', 'late-shape-template-key',
    'late-shape-nonstring', 'late-open-incompatible', 'late-shape-covered-extra'];
$out = [];
foreach ($types as $name => $type) {
    $out[$name] = [
        'accessible' => strtolower($type->isOffsetAccessible()->describe()),
        'hasOffset' => strtolower($type->hasOffsetValueType($zero)->describe()),
    ];
    if (in_array($name, $keys, true)) {
        $out[$name]['arrayKey'] = $type->toArrayKey()->describe(VerbosityLevel::precise());
    }
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
