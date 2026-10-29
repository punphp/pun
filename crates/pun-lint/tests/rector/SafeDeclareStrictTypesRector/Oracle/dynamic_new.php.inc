<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function __construct()
    {
    }
}

$c = "A";
new $c();
