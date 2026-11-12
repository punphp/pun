<?php

namespace Rector\Tests\CodeQuality\Rector\If_\ExplicitNullableObjectInstanceRector\Oracle;

final class CoalesceToNull
{
    public function run(?\stdClass $object)
    {
        if ($object ?? null) {
            return 1;
        }

        return 0;
    }
}
