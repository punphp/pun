<?php

final class SkipReboundLocal
{
    public function destructured(array $pair)
    {
        [$first, $second] = $pair;
        $first = [];

        return empty($first);
    }

    public function boundByForeach(array $items)
    {
        foreach ([[1]] as $items) {
        }

        return empty($items);
    }
}
