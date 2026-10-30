<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public int $i = 0;

    public static int $s = 0;

    public function m(int $a): void
    {
        $name = 'm';
        $this->$name(1);
    }
}
