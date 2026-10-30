<?php
// Public ExpressionResultFactory controls: the raw callback is independent of scope holders.
function resultControls($factory, $scope, $expr, $describe): string {
    $void = new PHPStan\Type\VoidType();
    $int = new PHPStan\Type\IntegerType();
    $mixed = new PHPStan\Type\MixedType();
    $string = new PHPStan\Type\StringType();
    $template = PHPStan\Type\Generic\TemplateTypeFactory::create(PHPStan\Type\Generic\TemplateTypeScope::createWithFunction('sample'), 'T', $mixed, PHPStan\Type\Generic\TemplateTypeVariance::createInvariant());
    $held = $scope->specifyExpressionType($expr, $int, $string, PHPStan\TrinaryLogic::createYes());
    $heldVoid = $scope->specifyExpressionType($expr, $void, $string, PHPStan\TrinaryLogic::createYes());
    $types = [
        'lazy-void' => [$void, false, $scope],
        'eager-void' => [$void, true, $held],
        'held-void' => [$void, false, $held],
        'held-mixed' => [$mixed, false, $held],
        'held-nonvoid' => [$string, false, $held],
        'held-void-value' => [$int, false, $heldVoid],
        'union' => [new PHPStan\Type\UnionType([$void, $int, new PHPStan\Type\NullType()]), false, $scope],
        'nested' => [new PHPStan\Type\ArrayType($int, $void), false, $scope],
        'mixed-minus-void' => [new PHPStan\Type\MixedType(false, $void), false, $held],
        'template-mixed' => [$template, false, $held],
        'mixed-minus-template' => [new PHPStan\Type\MixedType(false, $template), false, $held],
        'mixed-minus-mixed-minus-void' => [new PHPStan\Type\MixedType(false, new PHPStan\Type\MixedType(false, $void)), false, $held],
    ];
    $rows = [];
    foreach ($types as $name => [$raw, $eager, $before]) {
        $result = $factory->create(scope: $before, beforeScope: $before, expr: $expr, hasYield: false, isAlwaysTerminating: false, throwPoints: [], impurePoints: [], typeCallback: $eager ? null : static fn(bool $native) => $raw, specifyTypesCallback: static fn() => new PHPStan\Analyser\SpecifiedTypes([], []), type: $eager ? $raw : null, nativeType: $eager ? $raw : null);
        $ask = $result->atAskPosition($held);
        $rows[] = implode("\t", [$name, $describe($result->getType()), $describe((new ReflectionMethod($result, 'resolveOwnRawType'))->invoke($result, false)), $describe($result->getKeepVoidType(false)), $describe($result->getNativeType()), $describe($result->getTypeOnScope($held, false)), $describe($result->getTypeOnScope($held, true)), $describe($ask->getType()), $describe($ask->getNativeType()), $describe((new ReflectionMethod($result, 'resolveOwnRawType'))->invoke($result, true)), $describe($result->getKeepVoidType(true)), $describe($result->getTypeOnScope($heldVoid, false))]);
    }
    return implode("\n", $rows) . "\n";
}
