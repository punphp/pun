<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): \DateTime|object|null
{
    return new \stdClass();
}

function b(): \DateTime|string|int
{
    return new \DateTime();
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): object
{
    return new \stdClass();
}

function b(): \DateTime
{
    return new \DateTime();
}
