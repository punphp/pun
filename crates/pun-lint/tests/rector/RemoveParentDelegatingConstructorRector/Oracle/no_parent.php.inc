<?php

namespace Pun\Oracle\UselessConstructor;

final class WithoutParent
{
    public function __construct($value)
    {
        $this->value = $value;
    }
}
