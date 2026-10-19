<?php

final class NullableMany
{
    public function f($x, $y)
    {
        if ($x) {
            return 1;
        }
        if ($y) {
            return 'a';
        }
        return null;
    }
}
