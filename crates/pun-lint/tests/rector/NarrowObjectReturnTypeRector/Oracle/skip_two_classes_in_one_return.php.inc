<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

final class SkipTwoClassesInOneReturn
{
    public function create(bool $flag): object
    {
        return $flag ? new \DateTime() : new \stdClass();
    }
}
