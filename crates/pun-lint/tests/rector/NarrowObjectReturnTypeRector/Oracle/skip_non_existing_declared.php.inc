<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\ConferenceTalk;

final class SkipNonExistingDeclared
{
    public function create(): MissingClass
    {
        return new ConferenceTalk();
    }
}
