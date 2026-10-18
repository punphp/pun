<?php

namespace Rector\Tests\CodeQuality\Rector\If_\SimplifyIfNotNullReturnRector\Fixture;

final class Skip
{
    public function run()
    {
        $newNode = 'something';
        if ($newNode === null) {
            return null;
        }

        return 5;
    }
}
