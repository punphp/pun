<?php

namespace Pun\Oracle\SafeStrict;

trait T
{
    public int $p;

    public function m(): void
    {
        $this->p = 1;
    }
}
