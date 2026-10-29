<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class LoopAppend
{
    public function run(array $items)
    {
        $names = [];
        foreach ($items as $item) {
            $names[] = (string) $item;
        }

        return $names;
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class LoopAppend
{
    /**
     * @return string[]
     */
    public function run(array $items): array
    {
        $names = [];
        foreach ($items as $item) {
            $names[] = (string) $item;
        }

        return $names;
    }
}
