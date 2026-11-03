<?php

namespace AssertTypeCoverage;

use PHPStan\TrinaryLogic;
use function PHPStan\Testing\assertNativeType;
use function PHPStan\Testing\assertSuperType;
use function PHPStan\Testing\assertType;
use function PHPStan\Testing\assertVariableCertainty;

/**
 * @param positive-int $n
 * @param array{a: int, b?: string} $shape
 */
function f(int $n, array $shape, string $s, bool $flag): void
{
    assertType('int<1, max>', $n);
    assertType('int', $n);
    assertType($s, $n);
    assertNativeType('int', $n);
    assertNativeType('positive-int', $n);
    assertNativeType($s, $n);
    assertSuperType('int', $n);
    assertSuperType('string', $n);
    assertSuperType($s, $n);

    if ($flag) {
        $maybe = 1;
    }
    $yes = 1;
    assertVariableCertainty(TrinaryLogic::createYes(), $yes);
    assertVariableCertainty(TrinaryLogic::createMaybe(), $maybe);
    assertVariableCertainty(TrinaryLogic::createYes(), $maybe);
    assertVariableCertainty(TrinaryLogic::createNo(), $undefined);
    assertVariableCertainty(TrinaryLogic::createYes(), $shape['a']);
    assertVariableCertainty(TrinaryLogic::createYes(), $shape['b']);
    assertVariableCertainty(TrinaryLogic::createNo(), $shape['c']);
    assertVariableCertainty(TrinaryLogic::createYes(), $shape['c']);
    assertVariableCertainty('yes', $yes);
    assertVariableCertainty(Other::createYes(), $yes);
    assertVariableCertainty(TrinaryLogic::createYes(), $shape['a'] ?? 1);
}

class Other
{
    public static function createYes(): int
    {
        return 1;
    }
}
