<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class ExplicitMixedReturnDoc
{
    /**
     * Collects the values.
     *
     * @return mixed
     */
    public function run(string $value)
    {
        $values = [];
        $values[] = $value;

        return $values;
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

final class ExplicitMixedReturnDoc
{
    /**
     * Collects the values.
     *
     * @return list<string>
     */
    public function run(string $value): array
    {
        $values = [];
        $values[] = $value;

        return $values;
    }
}
