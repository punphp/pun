<?php

final class SomeClass
{
    public function run()
    {
        $v = [];
        $v[] = fn () => 1;

        return $v;
    }
}
