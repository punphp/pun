<?php

final class SomeClass
{
    public function run(array $items): void
    {
        $fn = 'array_map';
        $fn(fn (int $item) => $item, $items);
    }
}
