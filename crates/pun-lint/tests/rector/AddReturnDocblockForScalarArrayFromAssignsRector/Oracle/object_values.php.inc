<?php

abstract class SomeClass
{
    public function run($flag)
    {
        $v = [];
        $v[] = new \stdClass();

        return $v;
    }
}
