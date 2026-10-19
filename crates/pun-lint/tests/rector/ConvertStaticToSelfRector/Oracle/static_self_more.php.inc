<?php

final class A
{
    public const FOO = 1;

    public int $hooked {
        get => 1;
    }

    public function run($name)
    {
        return static::{$name} . static::$hooked;
    }
}
