<?php
declare(strict_types=1);

use PHPStan\Reflection\Native\NativeParameterReflection;
use PHPStan\Reflection\PassedByReference;
use PHPStan\Type\CallableType;
use PHPStan\Type\Constant\ConstantIntegerType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeHelper;
use PHPStan\Type\Generic\TemplateTypeMap;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\Generic\TemplateTypeVarianceMap;
use PHPStan\Type\IntegerType;
use PHPStan\Type\StringType;
use PHPStan\Type\Type;
use PHPStan\Type\UnionType;
use PHPStan\Type\VerbosityLevel;

Phar::loadPhar(realpath($argv[1]), 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';
$variance = ['invariant' => TemplateTypeVariance::createInvariant(), 'covariant' => TemplateTypeVariance::createCovariant(),
    'contravariant' => TemplateTypeVariance::createContravariant(), 'bivariant' => TemplateTypeVariance::createBivariant()];
$describe = fn (Type $t) => array_map(fn ($level) => $t->describe($level),
    [VerbosityLevel::precise(), VerbosityLevel::value(), VerbosityLevel::typeOnly()]);
$bound = fn () => new UnionType([new IntegerType(), new StringType()]);
$out = [];
foreach (['assignment', 'clone', 'traverse-identity', 'traverse-fresh-equal-bound', 'traverse-changed-bound',
    'traverse-fresh-equal-default', 'to-argument', 'argument-to-argument'] as $operation) {
    $original = TemplateTypeFactory::create(TemplateTypeScope::createWithFunction('fixture'), 'T', $bound(),
        $variance['invariant'], default: new ConstantIntegerType(1));
    if ($operation === 'argument-to-argument') { $original = $original->toArgument(); }
    $candidate = match ($operation) {
        'assignment' => $original,
        'clone' => clone $original,
        'traverse-identity' => $original->traverse(fn (Type $t) => $t),
        'traverse-fresh-equal-bound' => $original->traverse(fn (Type $t) => $t === $original->getBound() ? $bound() : $t),
        'traverse-changed-bound' => $original->traverse(fn (Type $t) => $t === $original->getBound() ? new StringType() : $t),
        'traverse-fresh-equal-default' => $original->traverse(fn (Type $t) => $t === $original->getDefault() ? new ConstantIntegerType(1) : $t),
        'to-argument', 'argument-to-argument' => $original->toArgument(),
    };
    foreach ($variance as $positionName => $position) {
        foreach ($variance as $callsiteName => $callsite) {
            $type = new CallableType([new NativeParameterReflection('x', false, $candidate, PassedByReference::createNo(), false, null)], $original, false);
            $resolved = TemplateTypeHelper::resolveTemplateTypes($type, new TemplateTypeMap(['T' => new StringType()]),
                new TemplateTypeVarianceMap(['T' => $callsite]), $position);
            $ids = [];
            $references = [];
            foreach ($type->getReferencedTemplateTypes($position) as $reference) {
                $id = spl_object_id($reference->getType());
                $ids[$id] ??= count($ids);
                $references[] = ['instance' => $ids[$id], 'name' => $reference->getType()->getName(), 'position' => $reference->getPositionVariance()->describe()];
            }
            $out[] = ['operation' => $operation, 'sameInstance' => $original === $candidate, 'equal' => $original->equals($candidate),
                'sameBound' => $original->getBound() === $candidate->getBound(), 'sameDefault' => $original->getDefault() === $candidate->getDefault(),
                'original' => $describe($original), 'candidate' => $describe($candidate), 'position' => $positionName,
                'callsite' => $callsiteName, 'references' => $references, 'before' => $describe($type),
                'after' => $describe($resolved), 'phpdoc' => (string) $resolved->toPhpDocNode()];
        }
    }
}
file_put_contents($argv[2], json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
echo count($out), " complete source transformation records\n";
