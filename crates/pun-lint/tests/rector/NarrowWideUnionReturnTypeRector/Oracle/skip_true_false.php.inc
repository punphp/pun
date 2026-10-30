<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): string|false
{
    return 'a';
}

function b(): ?false
{
    return null;
}
