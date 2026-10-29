<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

final class SkipNoReturn
{
    public function create(): object
    {
        throw new \RuntimeException();
    }
}
