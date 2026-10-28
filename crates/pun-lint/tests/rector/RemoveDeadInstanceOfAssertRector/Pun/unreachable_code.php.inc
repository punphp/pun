<?php

namespace Pun\AssertUnreachable;

final class Child
{
}

final class Holder
{
    public function run(Child $c)
    {
        return;
        assert($c instanceof Child);
    }
}
