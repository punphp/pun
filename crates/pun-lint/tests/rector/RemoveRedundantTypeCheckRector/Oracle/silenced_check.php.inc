<?php

namespace Rector\Tests\DeadCode\Rector\BinaryOp\RemoveRedundantTypeCheckRector\Oracle;

final class SilencedCheck
{
    public function run(?string $value)
    {
        return null === $value || @is_string($value);
    }
}
