<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class TwoItems
{
    public function run()
    {
        $values = [1, 2];

        return $values;
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class TwoItems
{
    public function run(): array
    {
        $values = [1, 2];

        return $values;
    }
}
