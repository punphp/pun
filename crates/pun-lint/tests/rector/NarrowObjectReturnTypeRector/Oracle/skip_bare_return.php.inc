<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\ConferenceTalk;

final class SkipBareReturn
{
    public function create(): object
    {
        if (rand(0, 1)) {
            return;
        }

        return new ConferenceTalk();
    }
}
