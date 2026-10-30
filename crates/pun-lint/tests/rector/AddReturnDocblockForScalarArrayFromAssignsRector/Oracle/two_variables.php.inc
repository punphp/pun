<?php

abstract class SomeClass
{
    public function run($flag)
    {
        $ints = [];
        $ints[] = 1;
        $strings = [];
        $strings[] = 'a';
        if ($flag) {
            return $ints;
        }

        return $strings;
    }
}
