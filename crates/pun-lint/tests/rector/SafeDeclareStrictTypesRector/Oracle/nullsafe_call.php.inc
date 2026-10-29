<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(): void
    {
    }
}

function f(?A $a): void
{
    $a?->m();
}
