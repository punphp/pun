<?php

namespace App;

abstract class Numbers
{
    abstract public function all();

    public function some()
    {
        yield 1;
    }
}
