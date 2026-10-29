<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class SkipMagicMethod
{
    public function __call($name, $arguments)
    {
        $values = [];

        return $values;
    }
}
