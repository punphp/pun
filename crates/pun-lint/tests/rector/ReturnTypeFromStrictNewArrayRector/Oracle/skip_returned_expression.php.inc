<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class SkipReturnedExpression
{
    public function run()
    {
        $values = [];

        return [$values];
    }
}
