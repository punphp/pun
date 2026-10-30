<?php
declare(strict_types=1);

// PHPStan 2.3.1 cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// Observe actual source objects independently of the production graph serializer.
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Reflection\ReflectionProvider;
use PHPStan\Type\Constant\ConstantIntegerType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\IntegerType;
use PHPStan\Type\Type;
use PHPStan\Type\VerbosityLevel;

$root = dirname(__DIR__, 5);
Phar::loadPhar(realpath($argv[1]), 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';
$export = file_get_contents($root.'/scripts/export-phpstan-symbols.php');
$start = strpos($export, 'function source_id(');
$end = strpos($export, 'function callable_entry(');
eval('use PHPStan\Type\Type; use PHPStan\Type\VerbosityLevel; use PHPStan\Reflection\ParametersAcceptor; '.substr($export, $start, $end - $start));
$GLOBALS['sourceVersion'] = 80500;
$observe = function (array $slots, bool $printer = false): array {
    $templates = new SplObjectStorage();
    $children = new SplObjectStorage();
    $id = function (Type $type, SplObjectStorage $objects): int {
        if (!$objects->contains($type)) { $objects[$type] = count($objects); }
        return $objects[$type];
    };
    $out = [];
    foreach ($slots as $name => $type) {
        $out[] = ['slot' => $name,
            'descriptions' => array_map(fn ($level) => $type->describe($level),
                [VerbosityLevel::precise(), VerbosityLevel::value(), VerbosityLevel::typeOnly()]),
            'phpdoc' => $printer ? (new PHPStan\PhpDocParser\Printer\Printer())->print($type->toPhpDocNode()) : (string) $type->toPhpDocNode(),
            'references' => array_map(function ($reference) use ($id, $templates, $children) {
                $type = $reference->getType();
                return ['instance' => $id($type, $templates), 'name' => $type->getName(),
                    'bound' => $id($type->getBound(), $children),
                    'default' => $type->getDefault() === null ? null : $id($type->getDefault(), $children),
                    'argument' => $type->isArgument(), 'position' => $reference->getPositionVariance()->describe()];
            }, $type->getReferencedTemplateTypes(TemplateTypeVariance::createInvariant()))];
    }
    return $out;
};
$rows = [];
foreach ([true, false] as $sharedBound) {
    foreach ([true, false] as $sharedDefault) {
        $bound = new IntegerType();
        $default = new ConstantIntegerType(1);
        $make = fn (Type $bound, Type $default) => TemplateTypeFactory::create(
            TemplateTypeScope::createWithFunction('fixture'), 'T', $bound,
            TemplateTypeVariance::createInvariant(), default: $default);
        $first = $make($bound, $default);
        $second = $make($sharedBound ? $bound : new IntegerType(), $sharedDefault ? $default : new ConstantIntegerType(1));
        $slots = ['first' => $first, 'second' => $second, 'clone' => clone $first, 'argument' => $first->toArgument()];
        $rows[] = ['name' => "children-$sharedBound-$sharedDefault", 'input' => array_map(fn ($t) => t($t, true), $slots),
            'expected' => $observe($slots), 'printed' => $observe($slots, true)];
    }
}
$tmp = sys_get_temp_dir().'/pun-source-identity-reflections';
@mkdir($tmp);
file_put_contents($tmp.'/phpstan.neon', "parameters:\n\tphpVersion: 80500\n");
$provider = (new ContainerFactory($tmp))->create($tmp, [$tmp.'/phpstan.neon'], [])->getByType(ReflectionProvider::class);
foreach (['array_map', 'array_filter', 'iterator_to_array', 'get_class', 'Generator::send', 'ArrayIterator::offsetGet'] as $name) {
    $reflection = str_contains($name, '::')
        ? $provider->getClass(explode('::', $name)[0])->getNativeMethod(explode('::', $name)[1])
        : $provider->getFunction(new PhpParser\Node\Name($name), null);
    foreach ($reflection->getVariants() as $i => $variant) {
        $slots = [];
        foreach ($variant->getParameters() as $p => $parameter) {
            $slots['param:'.$p] = $parameter->getType();
            if ($parameter->getDefaultValue() !== null) { $slots['default:'.$p] = $parameter->getDefaultValue(); }
        }
        $slots['effective'] = $variant->getReturnType();
        $slots['phpdoc'] = $variant->getPhpDocReturnType();
        $slots['native'] = $variant->getNativeReturnType();
        foreach ($variant->getTemplateTypeMap()->getTypes() as $key => $type) { $slots['template:'.$key] = $type; }
        foreach ($variant->getResolvedTemplateTypeMap()->getTypes() as $key => $type) { $slots['resolved:'.$key] = $type; }
        $rows[] = ['name' => $name.'#'.$i, 'builtin' => $name, 'variant' => $i,
            'input' => array_map(fn ($t) => t($t, true), $slots), 'expected' => $observe($slots), 'printed' => $observe($slots, true)];
    }
}
foreach (['Generator::send', 'ArrayIterator::offsetGet'] as $name) {
    [$class, $method] = explode('::', $name);
    $reflection = $provider->getClass($class);
    $variant = $reflection->getNativeMethod($method)->getVariants()[0];
    $slots = [];
    foreach ($reflection->getTemplateTypeMap()->getTypes() as $key => $type) { $slots['class:'.$key] = $type; }
    foreach ($variant->getParameters() as $p => $parameter) { $slots['param:'.$p] = $parameter->getType(); }
    $slots['effective'] = $variant->getReturnType();
    $slots['phpdoc'] = $variant->getPhpDocReturnType();
    $slots['native'] = $variant->getNativeReturnType();
    $rows[] = ['name' => $name.' declaration', 'builtin' => $name, 'variant' => 0,
        'input' => array_map(fn ($t) => t($t, true), $slots), 'expected' => $observe($slots), 'printed' => $observe($slots, true)];
}
$bound = new PHPStan\Type\StaticType($provider->getClass('ArrayIterator'));
$first = TemplateTypeFactory::create(TemplateTypeScope::createWithFunction('fixture'), 'S', $bound, TemplateTypeVariance::createInvariant());
$second = TemplateTypeFactory::create(TemplateTypeScope::createWithFunction('fixture'), 'S', $bound, TemplateTypeVariance::createInvariant());
$slots = ['first' => $first, 'second' => $second, 'static' => $bound];
$rows[] = ['name' => 'shared contextual static bound',
    'input' => array_map(fn ($t) => t($t, true), $slots), 'expected' => $observe($slots), 'printed' => $observe($slots, true)];
foreach ([true, false] as $sharedBound) {
    foreach ([true, false] as $sharedDefault) {
        $bound = new IntegerType();
        $default = new ConstantIntegerType(1);
        $make = fn (Type $bound, Type $default) => TemplateTypeFactory::create(
            TemplateTypeScope::createWithFunction('fixture'), 'T', $bound,
            TemplateTypeVariance::createInvariant(), default: $default);
        $first = $make($bound, $default);
        $second = $make($sharedBound ? $bound : new IntegerType(), $sharedDefault ? $default : new ConstantIntegerType(1));
        $restored = $second->traverse(fn (Type $type) => $type === $second->getBound() ? $first->getBound() : $first->getDefault());
        $slots = ['first' => $first, 'second' => $second, 'restored' => $restored];
        $rows[] = ['name' => "restore children-$sharedBound-$sharedDefault", 'bridge' => true,
            'input' => array_map(fn ($t) => t($t, true), $slots), 'expected' => $observe($slots), 'printed' => $observe($slots, true)];
    }
}
$callable = new PHPStan\Type\CallableType([
    new PHPStan\Reflection\Native\NativeParameterReflection('x', false, $first, PHPStan\Reflection\PassedByReference::createNo(), false, null),
], $first, false);
$slots = ['declaration' => $first, 'callable' => $callable];
$rows[] = ['name' => 'nested callable source forwarding',
    'input' => array_map(fn ($t) => t($t, true), $slots), 'expected' => $observe($slots), 'printed' => $observe($slots, true)];
foreach ([true, false] as $addDefault) {
    $bound = new IntegerType();
    $default = new ConstantIntegerType(1);
    $make = fn (?Type $default) => TemplateTypeFactory::create(
        TemplateTypeScope::createWithFunction('fixture'), 'T', $bound,
        TemplateTypeVariance::createInvariant(), default: $default);
    $first = $make($addDefault ? $default : null);
    $second = $make($addDefault ? null : $default);
    $restored = $make($first->getDefault());
    $slots = ['first' => $first, 'second' => $second, 'restored' => $restored];
    $rows[] = ['name' => $addDefault ? 'factory adds source default' : 'factory removes source default',
        'bridge' => true, 'input' => array_map(fn ($t) => t($t, true), $slots),
        'expected' => $observe($slots), 'printed' => $observe($slots, true)];
}
file_put_contents($argv[2], json_encode($rows, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n");
echo count($rows), " complete source-reflection graph records\n";
