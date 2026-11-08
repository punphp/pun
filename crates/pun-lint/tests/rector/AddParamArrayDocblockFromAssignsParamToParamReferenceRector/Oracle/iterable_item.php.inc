<?php

final class SomeClass
{
    public function run(array &$items, iterable $v)
    {
        $items[] = $v;
    }
}
