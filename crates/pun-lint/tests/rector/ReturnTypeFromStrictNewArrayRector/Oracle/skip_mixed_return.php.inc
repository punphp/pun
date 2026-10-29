<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class SkipMixedReturn
{
    public function run($value)
    {
        $values = [];
        if ($value) {
            return $values;
        }

        return $value;
    }
}
