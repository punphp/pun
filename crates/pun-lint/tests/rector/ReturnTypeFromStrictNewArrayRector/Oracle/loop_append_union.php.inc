<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class LoopAppendUnion
{
    public function run(array $items)
    {
        $values = [];
        foreach ($items as $item) {
            $values[] = $item ? 1 : 'none';
        }

        return $values;
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class LoopAppendUnion
{
    /**
     * @return int[]|'none'[]
     */
    public function run(array $items): array
    {
        $values = [];
        foreach ($items as $item) {
            $values[] = $item ? 1 : 'none';
        }

        return $values;
    }
}
