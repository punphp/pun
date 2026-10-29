<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): ?string
{
    return null;
}

function b(): ?\DateTime
{
    return new \DateTime();
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): ?string
{
    return null;
}

function b(): \DateTime
{
    return new \DateTime();
}
