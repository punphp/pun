<?php

final class SomeClass
{
    /**
     * @param mixed[] $items
     */
    public function run(array &$items, $v)
    {
        $items[] = $v;
    }
}
