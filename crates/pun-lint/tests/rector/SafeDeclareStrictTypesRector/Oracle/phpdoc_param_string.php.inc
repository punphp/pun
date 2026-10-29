<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function __construct()
    {
    }

    /** @param non-empty-string $a */
    public function m(string $a): void
    {
    }
}

(new A())->m("");
