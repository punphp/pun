<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\ConferenceTalk;

function create(): object
{
    return new ConferenceTalk();
}

final class SkipUntyped
{
    public function create()
    {
        return new ConferenceTalk();
    }
}
