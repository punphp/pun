<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class SkipReturnedOtherVariable
{
    public function run(array $other)
    {
        $values = [];

        return $other;
    }
}
