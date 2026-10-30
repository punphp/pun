<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public static function m(int $a): void
    {
        $name = 'm';
        A::$name(1);
    }
}
