<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function __construct()
    {
    }

    public function __call($n, $a)
    {
    }
}

(new A())->foo(1);
