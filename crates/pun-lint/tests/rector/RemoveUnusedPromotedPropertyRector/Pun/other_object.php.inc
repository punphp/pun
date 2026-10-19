<?php

namespace App;

final class Point
{
    public function __construct(private int $x)
    {
    }

    public function equals($other): bool
    {
        return $other->x === 1;
    }
}
