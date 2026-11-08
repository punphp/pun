<?php

final class MultilineDoc
{
    /**
     * @param array<
     *     int
     * > $other
     */
    public function run(array &$items, $other)
    {
        $items[] = 1;
    }
}
