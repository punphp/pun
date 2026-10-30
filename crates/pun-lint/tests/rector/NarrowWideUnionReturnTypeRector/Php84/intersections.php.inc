<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Php84;

function single(): (\Countable&\Traversable)|int
{
    return new \ArrayObject();
}

function nullable(): (\Countable&\Traversable)|int|null
{
    if (rand(0, 1)) {
        return null;
    }

    return new \ArrayObject();
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Php84;

function single(): \Countable&\Traversable
{
    return new \ArrayObject();
}

function nullable(): (\Countable&\Traversable)|null
{
    if (rand(0, 1)) {
        return null;
    }

    return new \ArrayObject();
}
