<?php

class DynamicSet
{
    public function f($b)
    {
        $this->{$b} = 1;
        $this->{$b} = 2;
        $v = 1;
    }
}
