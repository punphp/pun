<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class ReturnDocKept
{
    /**
     * @return int[]
     */
    public function run()
    {
        $values = [];
        $values[] = 1;

        return $values;
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class ReturnDocKept
{
    /**
     * @return int[]
     */
    public function run(): array
    {
        $values = [];
        $values[] = 1;

        return $values;
    }
}
