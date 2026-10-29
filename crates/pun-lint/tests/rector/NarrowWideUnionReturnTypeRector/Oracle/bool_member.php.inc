<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): string|bool|int
{
    if (rand(0, 1)) {
        return true;
    }

    return 'a';
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): string|bool
{
    if (rand(0, 1)) {
        return true;
    }

    return 'a';
}
