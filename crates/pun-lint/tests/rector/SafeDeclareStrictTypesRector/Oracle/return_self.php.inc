<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(): self
    {
        return new self();
    }
}
