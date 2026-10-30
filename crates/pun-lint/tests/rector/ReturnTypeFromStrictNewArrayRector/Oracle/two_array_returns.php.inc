<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class TwoArrayReturns
{
    public function run(bool $flag)
    {
        $values = [];
        if ($flag) {
            return $values;
        }

        return [1, 2];
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class TwoArrayReturns
{
    /**
     * @return int[]
     */
    public function run(bool $flag): array
    {
        $values = [];
        if ($flag) {
            return $values;
        }

        return [1, 2];
    }
}
