<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class SkipNoArrayAssign
{
    public function run($value)
    {
        $values = $value;

        return $values;
    }
}
