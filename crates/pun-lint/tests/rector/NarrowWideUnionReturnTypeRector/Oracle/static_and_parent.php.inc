<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

class Base
{
}

final class StaticAndParent extends Base
{
    public function single(): static|int|null
    {
        return $this;
    }

    public function nullable(): static|int|null
    {
        if (rand(0, 1)) {
            return null;
        }

        return $this;
    }

    public function parent(): parent|int|null
    {
        return new Base();
    }
}
