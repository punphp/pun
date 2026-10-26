<?php

final class SkipReferencedLocal
{
    public function referenced()
    {
        $items = array_values([]);
        $alias = &$items;

        return empty($items);
    }

    public function capturedByReference()
    {
        $items = array_values([]);
        $add = function () use (&$items) {
            $items[] = 1;
        };
        $add();

        return empty($items);
    }
}
