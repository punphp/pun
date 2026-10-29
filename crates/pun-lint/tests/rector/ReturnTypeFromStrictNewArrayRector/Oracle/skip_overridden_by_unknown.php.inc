<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class SkipOverriddenByUnknown
{
    public function run()
    {
        $values = [];
        $values = unknown_function();

        return $values;
    }
}
