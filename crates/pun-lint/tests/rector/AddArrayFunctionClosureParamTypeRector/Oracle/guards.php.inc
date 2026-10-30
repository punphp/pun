<?php

final class Filters
{
    /**
     * @param  list<int>  $items
     * @param  list<resource>  $handles
     */
    public function run(array $items, array $handles)
    {
        $keys = array_filter(array: $items, mode: ARRAY_FILTER_USE_KEY);
        $noArray = array_filter(callback: fn ($item) => $item > 1, mode: ARRAY_FILTER_USE_KEY);
        $trimmed = array_map('trim', $items);
        $open = array_filter($handles, fn ($handle) => is_resource($handle));
    }
}
