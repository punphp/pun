<?php

class A
{
    public function setA($v)
    {
        foo($v);
        return $this;
    }

    public function setB($v)
    {
        $b = $v;
        return $this;
    }
}
