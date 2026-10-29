<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(int $a): void
    {
    }

    public function n(): void
    {
        $this->m("1");
    }
}
