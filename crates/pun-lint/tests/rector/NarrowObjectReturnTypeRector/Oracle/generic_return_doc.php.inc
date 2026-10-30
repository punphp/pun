<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\Talk;
use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\ConferenceTalkExtended;

final class GenericReturnDoc
{
    /**
     * @return Talk<int>
     */
    public function create(): Talk
    {
        return new ConferenceTalkExtended();
    }
}
