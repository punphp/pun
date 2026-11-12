<?php

namespace Rector\Tests\CodeQuality\Rector\If_\ExplicitNullableObjectInstanceRector\Pun;

final class SkipNullThenObject
{
    public function run(\stdClass $object, bool $flag)
    {
        // Rector 2.8.4 writes `$flag ? null : $object instanceof \stdClass`, which is null when `$flag` is set
        if ($flag ? null : $object) {
            return 1;
        }

        return 0;
    }
}
