<?php

final class ConstantOther
{
    public const A = 1;

    public function f($x)
    {
        if ($x) {
            return self::A;
        }
        return 2;
    }
}
