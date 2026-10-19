<?php

final class PropertyPartial
{
    private int $a = 0;

    public function f($x)
    {
        if ($x) {
            return $this->a;
        }
    }
}
