<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\ConferenceTalk;

final class SkipNullableAndAlready
{
    public function nullable(): ?object
    {
        return new ConferenceTalk();
    }

    public function already(): ConferenceTalk
    {
        return new ConferenceTalk();
    }
}
