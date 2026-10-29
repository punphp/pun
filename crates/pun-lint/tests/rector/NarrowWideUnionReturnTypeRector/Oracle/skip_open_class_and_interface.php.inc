<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

interface SomeContract
{
    public function run(): string|int;
}

enum SomeEnum
{
    public function run(): string|int
    {
        return 'a';
    }
}

class OpenClass
{
    public function run(): string|int
    {
        return 'a';
    }
}
