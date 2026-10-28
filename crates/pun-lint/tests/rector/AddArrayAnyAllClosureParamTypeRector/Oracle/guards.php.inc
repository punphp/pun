<?php

final class Checks
{
    /**
     * @param  list<int>  $items
     * @param  list<resource>  $handles
     */
    public function run(array $items, array $handles)
    {
        $one = array_any($items);
        $named = array_any($items, 'is_int');
        $none = array_all($items, fn () => true);
        $open = array_any($handles, fn ($handle) => is_resource($handle));
    }
}
