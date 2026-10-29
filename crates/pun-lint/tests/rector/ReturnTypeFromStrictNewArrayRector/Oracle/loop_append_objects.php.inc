<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class LoopAppendObjects
{
    public function run(array $items)
    {
        $values = [];
        foreach ($items as $item) {
            $values[] = new \DateTimeImmutable($item);
        }

        return $values;
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class LoopAppendObjects
{
    /**
     * @return \DateTimeImmutable[]
     */
    public function run(array $items): array
    {
        $values = [];
        foreach ($items as $item) {
            $values[] = new \DateTimeImmutable($item);
        }

        return $values;
    }
}
