<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): float|string|int
{
    return 1.5;
}

function b(): iterable|string|int
{
    return [];
}

function c(): callable|int|null
{
    return 'strlen';
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): float
{
    return 1.5;
}

function b(): iterable
{
    return [];
}

function c(): callable
{
    return 'strlen';
}
