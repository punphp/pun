<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): (\Countable&\Traversable)|int
{
    return new \ArrayObject();
}
