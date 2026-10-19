<?php

trait UselessTernaryTrait
{
    public function f(array $a)
    {
        return $a ? $a : [];
    }
}
