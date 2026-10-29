<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

final class SkipScalarReturn
{
    public function create(): object
    {
        return 1;
    }
}
