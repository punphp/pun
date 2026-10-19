<?php

final class Nested
{
    private $a;

    private $b;

    public function f()
    {
        $this->b = foo($this->a = 1);
    }
}
