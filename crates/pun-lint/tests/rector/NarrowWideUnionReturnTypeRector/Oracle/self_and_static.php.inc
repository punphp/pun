<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

final class SelfAndStatic
{
    public function run(): self|int|null
    {
        return $this;
    }
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

final class SelfAndStatic
{
    public function run(): self
    {
        return $this;
    }
}
