<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class NoDocForEmptyArray
{
    public function run(bool $flag)
    {
        $values = [];

        if ($flag) {
            return $values;
        }

        return [];
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class NoDocForEmptyArray
{
    /**
     * @return never[]
     */
    public function run(bool $flag): array
    {
        $values = [];

        if ($flag) {
            return $values;
        }

        return [];
    }
}
