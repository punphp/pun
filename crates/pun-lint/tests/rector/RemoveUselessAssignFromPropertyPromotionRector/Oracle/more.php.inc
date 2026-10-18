<?php

class Regular
{
    public function __construct(private int $a)
    {
        parent::__construct();
        $b = $a;
        $this->a = $a;
    }

    public function other(int $a)
    {
        $this->a = $a;
    }
}

abstract class AbstractOne
{
    abstract public function __construct(int $a);
}
