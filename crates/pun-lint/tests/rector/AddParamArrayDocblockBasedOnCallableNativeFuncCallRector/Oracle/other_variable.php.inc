<?php

final class SomeClass
{
    public function run(array $items): void
    {
        $other = $items;
        array_map(fn (int $item) => $item, $other);
    }
}
