<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class SkipSilentVoid
{
    public function run(bool $flag)
    {
        $values = [];
        if ($flag) {
            return $values;
        }
    }
}
