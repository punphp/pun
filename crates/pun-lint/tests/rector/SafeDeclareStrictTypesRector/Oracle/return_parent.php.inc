<?php

namespace Pun\Oracle\SafeStrict;

class P
{
}

class A extends P
{
    public function m(): parent
    {
        return new P();
    }
}
