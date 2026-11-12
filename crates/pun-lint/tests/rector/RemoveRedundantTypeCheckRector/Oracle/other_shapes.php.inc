<?php

namespace Rector\Tests\DeadCode\Rector\BinaryOp\RemoveRedundantTypeCheckRector\Oracle;

final class OtherShapes
{
    public function leftIsNoComparison(?string $value, bool $flag)
    {
        return $flag || ! is_string($value);
    }

    public function rightIsNotNegated(?string $value)
    {
        return null === $value || is_string($value);
    }

    public function variableFunction(array $items, callable $check)
    {
        return $items && $check($items);
    }
}
