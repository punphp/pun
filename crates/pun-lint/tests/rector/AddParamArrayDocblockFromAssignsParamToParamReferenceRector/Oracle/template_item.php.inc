<?php

final class SomeClass
{
    /**
     * @template T
     * @param T $v
     */
    public function run(array &$items, $v)
    {
        $items[] = $v;
    }
}
