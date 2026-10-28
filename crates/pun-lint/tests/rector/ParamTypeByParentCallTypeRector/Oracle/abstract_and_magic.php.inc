<?php

namespace Probe;

/**
 * @method void magic(int $value)
 */
abstract class Base
{
    public function plain(int $value)
    {
    }

    public function __call($name, $arguments)
    {
    }
}

abstract class Child extends Base
{
    abstract public function later($value);

    public function plain($value)
    {
        echo $value;
    }

    public function magic($value)
    {
        parent::magic($value);
    }
}
