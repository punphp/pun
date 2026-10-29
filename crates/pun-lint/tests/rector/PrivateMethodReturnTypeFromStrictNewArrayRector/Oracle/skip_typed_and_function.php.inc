<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\PrivateMethodReturnTypeFromStrictNewArrayRector\Oracle;

function run()
{
    $values = [];

    return $values;
}

final class SkipTyped
{
    private function run(): iterable
    {
        $values = [];

        return $values;
    }
}
