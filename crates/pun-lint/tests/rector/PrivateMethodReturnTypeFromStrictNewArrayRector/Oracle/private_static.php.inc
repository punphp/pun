<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\PrivateMethodReturnTypeFromStrictNewArrayRector\Oracle;

class PrivateStatic
{
    private static function run(int $i)
    {
        $values = [];
        $values[] = $i;

        return $values;
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\PrivateMethodReturnTypeFromStrictNewArrayRector\Oracle;

class PrivateStatic
{
    /**
     * @return list<int>
     */
    private static function run(int $i): array
    {
        $values = [];
        $values[] = $i;

        return $values;
    }
}
