<?php

namespace Versions80;

class DynamicFetch
{
    public const FOO = 1;

    public function fetch(string $name): void
    {
        echo self::{$name};
        echo static::FOO;
    }
}
