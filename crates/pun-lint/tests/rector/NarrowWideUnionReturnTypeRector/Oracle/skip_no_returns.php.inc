<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

function a(): string|int
{
    throw new \RuntimeException();
}

$f = fn(): \Generator|int => yield;
